use super::{Channel, TypedChannel, UserData};
use crate::{
    error::{result_from_raw, Error},
    types::{
        request::{ArrayRequest, ReadRequest, Request},
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

pub trait GetFn: Send {
    type Request: ReadRequest + ?Sized;
    type Output: Send + Sized;

    fn apply(self, input: Result<&Self::Request, Error>) -> Result<Self::Output, Error>;
}

enum GetState<F: GetFn> {
    Empty,
    Pending(F),
    Ready(Result<F::Output, Error>),
}

#[must_use]
#[pin_project(PinnedDrop)]
pub struct Get<'a, F: GetFn> {
    owner: &'a mut Channel,
    /// Must be locked by `owner.user_data().process` mutex
    state: UnsafeCell<GetState<F>>,
    started: bool,
    #[pin]
    _pp: PhantomPinned,
}

impl<'a, F: GetFn> Get<'a, F> {
    fn new(owner: &'a mut Channel, func: F) -> Self {
        Self {
            owner,
            state: UnsafeCell::new(GetState::Pending(func)),
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
                    F::Request::ENUM.raw() as _,
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
        let state = &mut *(proc.data as *mut GetState<F>);
        let func = match mem::replace(state, GetState::Empty) {
            GetState::Pending(func) => func,
            _ => unreachable!(),
        };
        *state = GetState::Ready(match result {
            Ok(()) => {
                debug_assert_eq!(
                    F::Request::ENUM,
                    DbRequest::try_from_raw(args.type_ as _).unwrap()
                );
                debug_assert_ne!(args.count, 0);
                let request = F::Request::ref_from_ptr(args.dbr as *const u8, args.count as usize);
                func.apply(Ok(request))
            }
            Err(err) => func.apply(Err(err)),
        });
        user_data.waker.wake();
    }
}

impl<'a, F: GetFn> Future for Get<'a, F> {
    type Output = Result<F::Output, Error>;
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
            GetState::Pending(func) => {
                *state = GetState::Pending(func);
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
impl<'a, F: GetFn> PinnedDrop for Get<'a, F> {
    #[allow(clippy::needless_lifetimes)]
    fn drop(self: Pin<&mut Self>) {
        let mut proc = self.owner.user_data().process.lock().unwrap();
        proc.change_id();
        proc.data = ptr::null_mut();
    }
}

impl Channel {
    pub fn get_request_with<F: GetFn>(&mut self, func: F) -> Get<'_, F> {
        Get::new(self, func)
    }
}

impl<T: Scalar> TypedChannel<T> {
    pub fn get_request_with<R, F>(&mut self, func: F) -> Get<'_, F>
    where
        R: ArrayRequest<Type = T> + ReadRequest + ?Sized,
        F: GetFn<Request = R>,
    {
        self.base.get_request_with(func)
    }

    pub fn get_with<F: GetFn<Request = [T]>>(&mut self, func: F) -> Get<'_, F> {
        self.get_request_with(func)
    }

    pub fn get_to_slice<'a, 'b>(&'a mut self, dst: &'b mut [T]) -> Get<'a, GetToSlice<'b, T>> {
        self.get_with(GetToSlice { dst })
    }

    pub fn get_vec(&mut self) -> Get<'_, GetVec<T>> {
        self.get_with(GetVec { _p: PhantomData })
    }
}

pub struct GetToSlice<'a, T: Scalar> {
    dst: &'a mut [T],
}

impl<'a, T: Scalar> GetFn for GetToSlice<'a, T> {
    type Request = [T];
    type Output = usize;
    fn apply(self, input: Result<&[T], Error>) -> Result<Self::Output, Error> {
        input.map(|src| {
            let len = usize::min(self.dst.len(), src.len());
            self.dst[..len].copy_from_slice(&src[..len]);
            src.len()
        })
    }
}

pub struct GetVec<T: Scalar> {
    _p: PhantomData<T>,
}

impl<T: Scalar> GetFn for GetVec<T> {
    type Request = [T];
    type Output = Vec<T>;
    fn apply(self, input: Result<&Self::Request, Error>) -> Result<Self::Output, Error> {
        input.map(|src| Vec::from_iter(src.iter().cloned()))
    }
}
