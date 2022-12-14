use super::{Channel, TypedChannel, UserData};
use crate::{
    error::{result_from_raw, Error},
    types::{
        request::{ReadRequest, Request, TypedRequest},
        EventMask, Field, RequestId,
    },
};
use futures::{Stream, StreamExt};
use pin_project::{pin_project, pinned_drop};
use std::{
    cell::UnsafeCell,
    marker::PhantomPinned,
    mem,
    pin::Pin,
    ptr,
    task::{Context, Poll},
};

pub trait SubscribeFn: Send {
    type Request: ReadRequest + ?Sized;
    type Output: Send + Sized;

    fn push(&mut self, input: Result<&Self::Request, Error>);
    fn pop(&mut self) -> Option<Result<Self::Output, Error>>;
}

#[must_use]
#[pin_project(PinnedDrop)]
pub struct Subscribe<'a, F: SubscribeFn> {
    owner: &'a mut Channel,
    /// Must be locked by `owner.user_data().process` mutex
    state: UnsafeCell<F>,
    mask: EventMask,
    evid: Option<sys::evid>,
    #[pin]
    _pp: PhantomPinned,
}

impl<'a, F: SubscribeFn> Subscribe<'a, F> {
    fn new(owner: &'a mut Channel, func: F) -> Self {
        Self {
            owner,
            state: UnsafeCell::new(func),
            mask: EventMask::VALUE | EventMask::ALARM,
            evid: None,
            _pp: PhantomPinned,
        }
    }

    pub fn set_event_mask(&mut self, mask: EventMask) {
        self.mask = mask;
    }

    fn start(self: Pin<&mut Self>) -> Result<(), Error> {
        assert!(self.evid.is_none());
        let this = self.project();
        let owner = this.owner;
        owner.context().with(|| {
            let mut proc = owner.user_data().process.lock().unwrap();
            proc.data = this.state.get() as *mut u8;
            let mut evid: sys::evid = ptr::null_mut();
            result_from_raw(unsafe {
                sys::ca_create_subscription(
                    F::Request::ENUM.raw() as _,
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
            debug_assert_eq!(
                F::Request::ENUM,
                RequestId::try_from_raw(args.type_ as _).unwrap()
            );
            F::Request::from_ptr(args.dbr as *const u8, args.count as usize)
        }));
        drop(proc);
        user_data.waker.wake();
    }
}

impl<'a, F: SubscribeFn> Stream for Subscribe<'a, F> {
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
impl<'a, F: SubscribeFn> PinnedDrop for Subscribe<'a, F> {
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
    pub fn subscribe_request_with<F: SubscribeFn>(&mut self, func: F) -> Subscribe<'_, F> {
        Subscribe::new(self, func)
    }
}

impl<T: Field> TypedChannel<T> {
    pub fn subscribe_request_with<F: SubscribeFn>(&mut self, func: F) -> Subscribe<'_, F> {
        Subscribe::new(self, func)
    }

    pub fn subscribe_with<F: SubscribeFn<Request = [T]>>(&mut self, func: F) -> Subscribe<'_, F> {
        self.subscribe_request_with(func)
    }

    pub fn subscribe_request_vec<R>(&mut self) -> Subscribe<'_, SubscribeLastVec<R>>
    where
        R: TypedRequest<Field = T> + ReadRequest + ?Sized,
    {
        self.subscribe_request_with(SubscribeLastVec {
            buffer: Vec::new(),
            result: None,
        })
    }

    pub fn subscribe_vec(&mut self) -> impl Stream<Item = Result<Vec<T>, Error>> + '_ {
        self.subscribe_request_vec::<[T]>().map(|r| r.map(|x| x.1))
    }
}

pub struct SubscribeLastVec<R: TypedRequest + ReadRequest + ?Sized> {
    buffer: Vec<R::Field>,
    result: Option<Result<R::Meta, Error>>,
}

impl<R: TypedRequest + ReadRequest + ?Sized> SubscribeFn for SubscribeLastVec<R> {
    type Request = R;
    type Output = (R::Meta, Vec<R::Field>);
    fn push(&mut self, input: Result<&Self::Request, Error>) {
        self.result = Some(input.map(|req| {
            self.buffer.clear();
            self.buffer.extend_from_slice(req.values());
            *req.meta()
        }));
    }
    fn pop(&mut self) -> Option<Result<Self::Output, Error>> {
        self.result
            .take()
            .map(|res| res.map(|meta| (meta, mem::take(&mut self.buffer))))
    }
}
