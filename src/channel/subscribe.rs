use super::{base::UserData, Channel};
use crate::{
    error::{result_from_raw, Error},
    request::{ReadRequest, Request},
    types::{EventMask, RequestId},
};
use futures::Stream;
use pin_project::{pin_project, pinned_drop};
use std::{
    cell::UnsafeCell,
    collections::VecDeque,
    marker::{PhantomData, PhantomPinned},
    pin::Pin,
    ptr,
    task::{Context, Poll},
};

/// Subscription queue.
pub trait Queue: Send {
    type Request: ReadRequest + ?Sized;
    type Output: Send + Sized;

    /// Called immediately on channel updates.
    fn push(&mut self, input: Result<&Self::Request, Error>);
    /// Called when user tries to extract data from [`Subscription`] stream.
    fn pop(&mut self) -> Option<Result<Self::Output, Error>>;
}

/// Subscription to channel.
///
/// Stores queue that called each time channel is updated.
/// Provides results extracted from queue.
///
/// Depending on the type of the queue stored subscription may provide
/// either the last unread value or all received values.
#[must_use]
#[pin_project(PinnedDrop)]
pub struct Subscription<'a, F: Queue> {
    owner: &'a mut Channel,
    /// Must be locked by `owner.user_data().process` mutex
    state: UnsafeCell<F>,
    mask: EventMask,
    evid: Option<sys::evid>,
    #[pin]
    _pp: PhantomPinned,
}

impl<'a, F: Queue> Subscription<'a, F> {
    pub(crate) fn new(owner: &'a mut Channel, func: F) -> Self {
        Self {
            owner,
            state: UnsafeCell::new(func),
            mask: EventMask::VALUE | EventMask::ALARM,
            evid: None,
            _pp: PhantomPinned,
        }
    }

    /// Set kinds of channel events this subscription should be notified.
    ///
    /// Default event mask is [`EventMask::VALUE`]` | `[`EventMask::ALARM`].
    ///
    /// *You need to call this before [`start`](`Self::start`)-ing the subscription
    /// because after it started you cannot unpin it.*
    pub fn set_event_mask(&mut self, mask: EventMask) {
        self.mask = mask;
    }

    /// Initiate subscription.
    ///
    /// **You will not receive channel update until this method was called, explicitly or implicitly.**
    ///
    /// This method can be called implicitly on the first poll.
    /// It cannot be done in constructor because `Self` must be pinned at this point.
    pub fn start(self: Pin<&mut Self>) -> Result<(), Error> {
        assert!(self.evid.is_none());
        let this = self.project();
        let owner = this.owner;
        owner.context().with(|| {
            let mut proc = owner.user_data().process.lock().unwrap();
            proc.data = this.state.get() as *mut u8;
            let mut evid: sys::evid = ptr::null_mut();
            result_from_raw(unsafe {
                sys::ca_create_subscription(
                    F::Request::ID.raw() as _,
                    0,
                    owner.raw(),
                    this.mask.raw() as _,
                    Some(Self::callback),
                    proc.id() as _,
                    &mut evid as *mut sys::evid,
                )
            })
            .map(|()| {
                owner.context().flush_io();
                *this.evid = Some(evid);
            })
        })
    }

    unsafe extern "C" fn callback(args: sys::event_handler_args) {
        println!("subscribe_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        let proc = user_data.process.lock().unwrap();
        if proc.id() != args.usr as usize {
            return;
        }
        let func = &mut *(proc.data as *mut F);
        func.push(result_from_raw(args.status).and_then(|()| {
            F::Request::from_ptr(
                args.dbr as *const u8,
                RequestId::try_from_raw(args.type_ as _).unwrap(),
                args.count as usize,
            )
        }));
        drop(proc);
        user_data.waker.wake();
    }
}

impl<'a, F: Queue> Stream for Subscription<'a, F> {
    type Item = Result<F::Output, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.owner.user_data().waker.register(cx.waker());
        if self.evid.is_none() {
            self.start()?;
            return Poll::Pending;
        }
        let this = self.project();
        let proc = this.owner.user_data().process.lock().unwrap();
        let func = unsafe { &mut *this.state.get() };
        let poll = match func.pop() {
            Some(res) => Poll::Ready(Some(res)),
            None => Poll::Pending,
        };
        drop(proc);
        poll
    }
}

#[pinned_drop]
impl<'a, F: Queue> PinnedDrop for Subscription<'a, F> {
    #[allow(clippy::needless_lifetimes)]
    fn drop(self: Pin<&mut Self>) {
        let mut proc = self.owner.user_data().process.lock().unwrap();
        proc.change_id();
        proc.data = ptr::null_mut();
        if let Some(evid) = self.evid {
            self.owner.context().with(|| unsafe {
                result_from_raw(sys::ca_clear_subscription(evid)).unwrap();
            });
        }
        drop(proc);
    }
}

/// Subscription queue that stores only last received value, applying `F` to it.
///
/// `F` applied to all received values. If `F` returned `None` the new value will not overwrite previous value.
pub struct LastFn<I, O, F = fn(Result<&I, Error>) -> Option<Result<O, Error>>>
where
    I: ReadRequest + ?Sized,
    O: Send,
    F: FnMut(Result<&I, Error>) -> Option<Result<O, Error>> + Send,
{
    func: F,
    last: Option<Result<O, Error>>,
    _p: PhantomData<I>,
}

impl<I, O, F> LastFn<I, O, F>
where
    I: ReadRequest + ?Sized,
    O: Send,
    F: FnMut(Result<&I, Error>) -> Option<Result<O, Error>> + Send,
{
    pub(crate) fn new(f: F) -> Self {
        Self {
            func: f,
            last: None,
            _p: PhantomData,
        }
    }
}

impl<I, O, F> Queue for LastFn<I, O, F>
where
    I: ReadRequest + ?Sized,
    O: Send,
    F: FnMut(Result<&I, Error>) -> Option<Result<O, Error>> + Send,
{
    type Request = I;
    type Output = O;
    fn push(&mut self, input: Result<&Self::Request, Error>) {
        if let Some(output) = (self.func)(input) {
            self.last = Some(output);
        }
    }
    fn pop(&mut self) -> Option<Result<Self::Output, Error>> {
        self.last.take()
    }
}

/// Subscription queue that stores all received values, applying `F` to them.
///
/// If `F` returned `None` the value is filtered out.
pub struct QueueFn<I, O, F = fn(Result<&I, Error>) -> Option<Result<O, Error>>>
where
    I: ReadRequest + ?Sized,
    O: Send,
    F: FnMut(Result<&I, Error>) -> Option<Result<O, Error>> + Send,
{
    func: F,
    queue: VecDeque<Result<O, Error>>,
    _p: PhantomData<I>,
}

impl<I, O, F> QueueFn<I, O, F>
where
    I: ReadRequest + ?Sized,
    O: Send,
    F: FnMut(Result<&I, Error>) -> Option<Result<O, Error>> + Send,
{
    pub(crate) fn new(f: F) -> Self {
        Self {
            func: f,
            queue: VecDeque::new(),
            _p: PhantomData,
        }
    }
}

impl<I, O, F> Queue for QueueFn<I, O, F>
where
    I: ReadRequest + ?Sized,
    O: Send,
    F: FnMut(Result<&I, Error>) -> Option<Result<O, Error>> + Send,
{
    type Request = I;
    type Output = O;
    fn push(&mut self, input: Result<&Self::Request, Error>) {
        if let Some(output) = (self.func)(input) {
            self.queue.push_back(output);
        }
    }
    fn pop(&mut self) -> Option<Result<Self::Output, Error>> {
        self.queue.pop_front()
    }
}
