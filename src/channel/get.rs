use super::{Channel, TypedChannel, UserData};
use crate::{
    error::{result_from_raw, Error},
    types::{
        request::{ArrayRequest, ReadRequest},
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

enum GetState<'b, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnOnce(Result<&R, Error>) -> Result<Q, Error> + Send,
{
    Empty,
    Pending(F, PhantomData<&'b R>),
    Ready(Result<Q, Error>),
}

#[must_use]
#[pin_project(PinnedDrop)]
pub struct Get<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnOnce(Result<&R, Error>) -> Result<Q, Error> + Send,
{
    owner: &'a mut Channel,
    /// Must be locked by `owner.user_data().process` mutex
    state: UnsafeCell<GetState<'a, R, Q, F>>,
    started: bool,
    #[pin]
    _pp: PhantomPinned,
}

impl<'a, R, Q, F> Get<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnOnce(Result<&R, Error>) -> Result<Q, Error> + Send,
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
            proc.data = this.state.get() as *mut u8;
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
                *this.started = true
            })
        })
    }

    unsafe extern "C" fn callback(args: sys::event_handler_args) {
        println!("get_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        let proc = user_data.process.lock().unwrap();
        if proc.id() != args.usr as usize {
            return;
        }
        let result = result_from_raw(args.status);
        let state = &mut *(proc.data as *mut GetState<'a, R, Q, F>);
        let func = match mem::replace(state, GetState::Empty) {
            GetState::Pending(func, _) => func,
            _ => unreachable!(),
        };
        *state = GetState::Ready(match result {
            Ok(()) => {
                debug_assert_eq!(R::ENUM, DbRequest::try_from_raw(args.type_ as _).unwrap());
                debug_assert_ne!(args.count, 0);
                let request = R::ref_from_ptr(args.dbr as *const u8, args.count as usize);
                func(Ok(request))
            }
            Err(err) => func(Err(err)),
        });
        user_data.waker.wake();
    }
}

impl<'a, R, Q, F> Future for Get<'a, R, Q, F>
where
    R: ReadRequest + ?Sized,
    Q: Send,
    F: FnOnce(Result<&R, Error>) -> Result<Q, Error> + Send,
{
    type Output = Result<Q, Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.owner.user_data().waker.register(cx.waker());
        if !self.started {
            self.start()?;
            return Poll::Pending;
        }
        let this = self.project();
        let proc = this.owner.user_data().process.lock().unwrap();
        let state = unsafe { &mut *this.state.get() };
        let poll = match mem::replace(state, GetState::Empty) {
            GetState::Empty => unreachable!(),
            GetState::Pending(func, _) => {
                *state = GetState::Pending(func, PhantomData);
                Poll::Pending
            }
            GetState::Ready(res) => match res {
                Ok(ret) => Poll::Ready(Ok(ret)),
                Err(err) => Poll::Ready(Err(err)),
            },
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
    F: FnOnce(Result<&R, Error>) -> Result<Q, Error> + Send,
{
    #[allow(clippy::needless_lifetimes)]
    fn drop(self: Pin<&mut Self>) {
        let mut proc = self.owner.user_data().process.lock().unwrap();
        proc.change_id();
        proc.data = ptr::null_mut();
    }
}

impl Channel {
    pub fn get_request_with<R, Q, F>(&mut self, func: F) -> Get<'_, R, Q, F>
    where
        R: ReadRequest + ?Sized,
        Q: Send,
        F: FnOnce(Result<&R, Error>) -> Result<Q, Error> + Send,
    {
        Get::new(self, func)
    }
}

impl<T: Scalar> TypedChannel<T> {
    pub fn get_request_with<R, Q, F>(&mut self, func: F) -> Get<'_, R, Q, F>
    where
        R: ArrayRequest<Type = T> + ReadRequest + ?Sized,
        Q: Send,
        F: FnOnce(Result<&R, Error>) -> Result<Q, Error> + Send,
    {
        self.base.get_request_with(func)
    }

    pub fn get_with<Q, F>(&mut self, func: F) -> Get<'_, [T], Q, F>
    where
        Q: Send,
        F: FnOnce(Result<&[T], Error>) -> Result<Q, Error> + Send,
    {
        self.get_request_with(func)
    }

    pub async fn get_to_slice(&mut self, dst: &mut [T]) -> Result<usize, Error> {
        self.get_with(|res: Result<&[T], Error>| {
            res.map(|src| {
                let len = usize::min(dst.len(), src.len());
                dst[..len].copy_from_slice(&src[..len]);
                len
            })
        })
        .await
    }

    pub async fn get_vec(&mut self) -> Result<Vec<T>, Error> {
        self.get_with(|res: Result<&[T], Error>| res.map(|src| Vec::from_iter(src.iter().cloned())))
            .await
    }
}
