use super::{Channel, TypedChannel, UserData};
use crate::{
    error::{result_from_raw, Error},
    types::{
        request::{ArrayRequest, ReadRequest},
        DbEvent, DbRequest, Scalar,
    },
};
use futures::Stream;
use pin_project::{pin_project, pinned_drop};
use std::{
    cell::UnsafeCell,
    marker::{PhantomData, PhantomPinned},
    pin::Pin,
    ptr,
    task::{Context, Poll},
};

struct SubscribeState<R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnMut(Result<&R, Error>) -> Option<Result<Q, Error>> + Send,
{
    func: F,
    output: Option<Result<Q, Error>>,
    _p: PhantomData<R>,
}

#[must_use]
#[pin_project(PinnedDrop)]
pub struct Subscribe<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnMut(Result<&R, Error>) -> Option<Result<Q, Error>> + Send,
{
    owner: &'a mut Channel,
    /// Must be locked by `owner.user_data().process` mutex
    state: UnsafeCell<SubscribeState<R, Q, F>>,
    mask: DbEvent,
    evid: Option<sys::evid>,
    #[pin]
    _pp: PhantomPinned,
}

impl<'a, R, Q, F> Subscribe<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnMut(Result<&R, Error>) -> Option<Result<Q, Error>> + Send,
{
    fn new(owner: &'a mut Channel, func: F) -> Self {
        Self {
            owner,
            state: UnsafeCell::new(SubscribeState {
                func,
                output: None,
                _p: PhantomData,
            }),
            mask: DbEvent::VALUE | DbEvent::ALARM,
            evid: None,
            _pp: PhantomPinned,
        }
    }

    pub fn set_event_mask(&mut self, mask: DbEvent) {
        self.mask = mask;
    }

    fn start(self: Pin<&mut Self>) -> Result<(), Error> {
        println!("Start: {:p}", self);
        assert!(self.evid.is_none());
        let this = self.project();
        let owner = this.owner;
        owner.context().with(|| {
            let mut proc = owner.user_data().process.lock().unwrap();
            proc.data = this.state.get() as *mut u8;
            let mut evid: sys::evid = ptr::null_mut();
            result_from_raw(unsafe {
                sys::ca_create_subscription(
                    R::ENUM.raw() as _,
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
        let result = result_from_raw(args.status);
        let state = &mut *(proc.data as *mut SubscribeState<R, Q, F>);
        state.output = match result {
            Ok(()) => {
                debug_assert_eq!(R::ENUM, DbRequest::try_from_raw(args.type_ as _).unwrap());
                debug_assert_ne!(args.count, 0);
                let request = R::ref_from_ptr(args.dbr as *const u8, args.count as usize);
                (state.func)(Ok(request))
            }
            Err(err) => (state.func)(Err(err)),
        };
        drop(proc);
        user_data.waker.wake();
    }
}

impl<'a, R, Q, F> Stream for Subscribe<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnMut(Result<&R, Error>) -> Option<Result<Q, Error>> + Send,
{
    type Item = Result<Q, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        println!("Poll next: {:p}", self);
        self.owner.user_data().waker.register(cx.waker());
        if self.evid.is_none() {
            self.start()?;
            return Poll::Pending;
        }
        let this = self.project();
        let proc = this.owner.user_data().process.lock().unwrap();
        let state = unsafe { &mut *this.state.get() };
        let poll = match state.output.take() {
            Some(res) => Poll::Ready(Some(res)),
            None => Poll::Pending,
        };
        drop(proc);
        poll
    }
}

#[pinned_drop]
impl<'a, R, Q, F> PinnedDrop for Subscribe<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnMut(Result<&R, Error>) -> Option<Result<Q, Error>> + Send,
{
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

impl Channel {
    pub fn subscribe_request_with<R, Q, F>(&mut self, func: F) -> Subscribe<'_, R, Q, F>
    where
        R: ReadRequest + ?Sized,
        Q: Send,
        F: FnMut(Result<&R, Error>) -> Option<Result<Q, Error>> + Send,
    {
        Subscribe::new(self, func)
    }
}

impl<T: Scalar> TypedChannel<T> {
    pub fn subscribe_request_with<R, Q, F>(&mut self, func: F) -> Subscribe<'_, R, Q, F>
    where
        R: ArrayRequest<Type = T> + ReadRequest + ?Sized,
        Q: Send,
        F: FnMut(Result<&R, Error>) -> Option<Result<Q, Error>> + Send,
    {
        Subscribe::new(self, func)
    }

    pub fn subscribe_with<Q, F>(&mut self, func: F) -> Subscribe<'_, [T], Q, F>
    where
        Q: Send,
        F: FnMut(Result<&[T], Error>) -> Option<Result<Q, Error>> + Send,
    {
        self.subscribe_request_with(func)
    }

    pub fn subscribe_vec(&mut self) -> impl Stream<Item = Result<Vec<T>, Error>> + '_ {
        self.subscribe_with(|res: Result<&[T], Error>| {
            Some(res.map(|s| Vec::from_iter(s.iter().cloned())))
        })
    }
}
