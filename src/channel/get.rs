use super::{TypedChannel, UserData};
use crate::{
    error::{result_from_raw, Error},
    types::{DbField, Scalar},
};
use pin_project::{pin_project, pinned_drop};
use std::{
    cell::UnsafeCell,
    future::Future,
    marker::PhantomData,
    mem,
    pin::Pin,
    ptr, slice,
    task::{Context, Poll},
};

enum GetState<T: Scalar, F: FnOnce(&[T]) -> R + Send, R> {
    Empty,
    Pending(F, PhantomData<T>),
    Ready(R),
}

#[must_use]
#[pin_project(PinnedDrop)]
pub struct Get<'a, T: Scalar, F: FnOnce(&[T]) -> R + Send, R> {
    owner: &'a mut TypedChannel<T>,
    /// Must be locked by `owner.user_data().process` mutex
    #[pin]
    state: UnsafeCell<GetState<T, F, R>>,
    started: bool,
}

impl<'a, T: Scalar, F: FnOnce(&[T]) -> R + Send, R> Get<'a, T, F, R> {
    fn new(owner: &'a mut TypedChannel<T>, func: F) -> Self {
        Self {
            owner,
            state: UnsafeCell::new(GetState::Pending(func, PhantomData)),
            started: false,
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
                    T::ENUM.raw() as _,
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
        let state = &mut *(proc.state as *mut GetState<T, F, R>);
        let func = match mem::replace(state, GetState::Empty) {
            GetState::Pending(func, _) => func,
            _ => unreachable!(),
        };
        if result.is_ok() {
            debug_assert_eq!(T::ENUM, DbField::try_from_raw(args.type_ as _).unwrap());
            *state = GetState::Ready(func(slice::from_raw_parts(
                args.dbr as *const T,
                args.count as usize,
            )));
        }
        proc.result = Some(result);
        user_data.waker.wake();
    }
}

impl<'a, T: Scalar, F: FnOnce(&[T]) -> R + Send, R> Future for Get<'a, T, F, R> {
    type Output = Result<R, Error>;
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
impl<'a, T: Scalar, F: FnOnce(&[T]) -> R + Send, R> PinnedDrop for Get<'a, T, F, R> {
    #[allow(clippy::needless_lifetimes)]
    fn drop(self: Pin<&mut Self>) {
        let mut proc = self.owner.user_data().process.lock().unwrap();
        proc.change_id();
        proc.state = ptr::null_mut();
        proc.result = None;
    }
}

impl<T: Scalar> TypedChannel<T> {
    pub async fn get_with<F: FnOnce(&[T]) -> R + Send, R>(&mut self, func: F) -> Result<R, Error> {
        Get::new(self, func).await
    }

    pub async fn get_to_slice(&mut self, dst: &mut [T]) -> Result<usize, Error> {
        self.get_with(|src| {
            let len = usize::min(dst.len(), src.len());
            dst[..len].copy_from_slice(&src[..len]);
            len
        })
        .await
    }

    pub async fn get_vec(&mut self) -> Result<Vec<T>, Error> {
        self.get_with(|s| Vec::from_iter(s.iter().cloned())).await
    }
}
