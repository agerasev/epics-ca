use super::{Channel, TypedChannel, UserData};
use crate::{
    error::{result_from_raw, Error},
    types::{
        request::{ReadRequest, TypedRequest},
        DbRequest, Scalar,
    },
};
use pin_project::{pin_project, pinned_drop};
use std::{
    cell::UnsafeCell,
    future::Future,
    marker::{PhantomData, PhantomPinned},
    mem,
    pin::Pin,
    ptr,
    task::{Context, Poll},
};

enum GetState<R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnOnce(&R) -> Q + Send,
{
    Empty,
    Pending(F, PhantomData<R>),
    Ready(Q),
}

#[must_use]
#[pin_project(PinnedDrop)]
pub struct Get<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnOnce(&R) -> Q + Send,
{
    owner: &'a mut Channel,
    /// Must be locked by `owner.user_data().process` mutex
    state: UnsafeCell<GetState<R, Q, F>>,
    started: bool,
    #[pin]
    _pp: PhantomPinned,
}

impl<'a, R, Q, F> Get<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnOnce(&R) -> Q + Send,
{
    fn new(owner: &'a mut Channel, func: F) -> Self {
        Self {
            owner,
            state: UnsafeCell::new(GetState::Pending(func, PhantomData)),
            started: false,
            _pp: PhantomPinned,
        }
    }

    fn start(self: Pin<&mut Self>) -> Result<(), Error> {
        assert!(!self.started);
        let this = self.project();
        let owner = this.owner;
        owner.context().with(|| {
            let mut proc = owner.user_data().process.lock().unwrap();
            proc.state = this.state.get() as *mut u8;
            result_from_raw(unsafe {
                sys::ca_array_get_callback(
                    R::ENUM.raw() as _,
                    0,
                    owner.raw(),
                    Some(Self::callback),
                    proc.id() as _,
                )
            })
            .map(|()| {
                owner.context().flush_io();
                proc.result = None;
                *this.started = true
            })
        })
    }

    unsafe extern "C" fn callback(args: sys::event_handler_args) {
        println!("get_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        let mut proc = user_data.process.lock().unwrap();
        if proc.id() != args.usr as usize {
            return;
        }
        let result = result_from_raw(args.status);
        let state = &mut *(proc.state as *mut GetState<R, Q, F>);
        let func = match mem::replace(state, GetState::Empty) {
            GetState::Pending(func, _) => func,
            _ => unreachable!(),
        };
        if result.is_ok() {
            debug_assert_eq!(R::ENUM, DbRequest::try_from_raw(args.type_ as _).unwrap());
            debug_assert_ne!(args.count, 0);
            let request = R::ref_from_ptr(args.dbr as *const u8, args.count as usize);
            *state = GetState::Ready(func(request));
        }
        proc.result = Some(result);
        user_data.waker.wake();
    }
}

impl<'a, R, Q, F> Future for Get<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnOnce(&R) -> Q + Send,
{
    type Output = Result<Q, Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.owner.user_data().waker.register(cx.waker());
        if !self.started {
            self.start()?;
            return Poll::Pending;
        }
        let this = self.project();
        let mut proc = this.owner.user_data().process.lock().unwrap();
        let state = unsafe { &mut *this.state.get() };
        let poll = match proc.result.take() {
            Some(Ok(())) => match mem::replace(state, GetState::Empty) {
                GetState::Ready(ret) => Poll::Ready(Ok(ret)),
                _ => unreachable!(),
            },
            Some(Err(err)) => Poll::Ready(Err(err)),
            None => Poll::Pending,
        };
        drop(proc);
        poll
    }
}

#[pinned_drop]
impl<'a, R, Q, F> PinnedDrop for Get<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnOnce(&R) -> Q + Send,
{
    #[allow(clippy::needless_lifetimes)]
    fn drop(self: Pin<&mut Self>) {
        let mut proc = self.owner.user_data().process.lock().unwrap();
        proc.change_id();
        proc.state = ptr::null_mut();
        proc.result = None;
    }
}

impl Channel {
    pub fn get_request_with<R, Q, F>(&mut self, func: F) -> Get<'_, R, Q, F>
    where
        R: ReadRequest + ?Sized,
        Q: Send,
        F: FnOnce(&R) -> Q + Send,
    {
        Get::new(self, func)
    }
}

impl<T: Scalar> TypedChannel<T> {
    pub fn get_request_with<R, Q, F>(&mut self, func: F) -> Get<'_, R, Q, F>
    where
        R: ReadRequest + TypedRequest<Type = T> + ?Sized,
        Q: Send,
        F: FnOnce(&R) -> Q + Send,
    {
        self.base.get_request_with(func)
    }

    pub fn get_with<Q, F>(&mut self, func: F) -> Get<'_, [T], Q, F>
    where
        Q: Send,
        F: FnOnce(&[T]) -> Q + Send,
    {
        self.get_request_with(func)
    }

    pub async fn get_to_slice(&mut self, dst: &mut [T]) -> Result<usize, Error> {
        self.get_with(|src: &[T]| {
            let len = usize::min(dst.len(), src.len());
            dst[..len].copy_from_slice(&src[..len]);
            len
        })
        .await
    }

    pub async fn get_vec(&mut self) -> Result<Vec<T>, Error> {
        self.get_with(|s: &[T]| Vec::from_iter(s.iter().cloned()))
            .await
    }
}
