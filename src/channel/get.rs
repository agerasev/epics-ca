use super::{base::UserData, Channel};
use crate::{
    error::{result_from_raw, Error},
    request::{ReadRequest, Request},
    types::RequestId,
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

/// Callback that called when request result is ready.
pub trait Callback: Send {
    type Request: ReadRequest + ?Sized;
    type Output: Send;

    /// Performs some operation on request result.
    fn apply(self, input: Result<&Self::Request, Error>) -> Result<Self::Output, Error>;
}

pub(crate) enum GetState<F: Callback> {
    Empty,
    Pending(F),
    Ready(Result<F::Output, Error>),
}

/// Future that performs reading from channel.
#[must_use]
#[pin_project(PinnedDrop)]
pub struct Get<'a, F: Callback> {
    owner: &'a mut Channel,
    /// Must be locked by `owner.user_data().process` mutex
    state: UnsafeCell<GetState<F>>,
    started: bool,
    #[pin]
    _pp: PhantomPinned,
}

impl<'a, F: Callback> Get<'a, F> {
    pub(crate) fn new(owner: &'a mut Channel, func: F) -> Self {
        Self {
            owner,
            state: UnsafeCell::new(GetState::Pending(func)),
            started: false,
            _pp: PhantomPinned,
        }
    }

    /// Initiate reading.
    ///
    /// This method can be called implicitly on the first poll.
    /// It cannot be done in constructor because `Self` must be pinned at this point.
    pub fn start(self: Pin<&mut Self>) -> Result<(), Error> {
        assert!(!self.started);
        let this = self.project();
        let owner = this.owner;
        owner.context().with(|| {
            let mut proc = owner.user_data().process.lock().unwrap();
            proc.data = this.state.get() as *mut u8;
            result_from_raw(unsafe {
                sys::ca_array_get_callback(
                    F::Request::ID.raw() as _,
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
        let state = &mut *(proc.data as *mut GetState<F>);
        let func = match mem::replace(state, GetState::Empty) {
            GetState::Pending(func) => func,
            _ => unreachable!(),
        };
        *state = GetState::Ready(func.apply(result_from_raw(args.status).and_then(|()| {
            F::Request::from_ptr(
                args.dbr as *const u8,
                RequestId::try_from_raw(args.type_ as _).unwrap(),
                args.count as usize,
            )
        })));
        user_data.waker.wake();
    }
}

impl<'a, F: Callback> Future for Get<'a, F> {
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
impl<'a, F: Callback> PinnedDrop for Get<'a, F> {
    #[allow(clippy::needless_lifetimes)]
    fn drop(self: Pin<&mut Self>) {
        let mut proc = self.owner.user_data().process.lock().unwrap();
        proc.change_id();
        proc.data = ptr::null_mut();
    }
}

/// Adapter that allows to use arbitrary function or closure as [`Callback`].
pub struct GetFn<R, O, F = fn(Result<&R, Error>) -> Result<O, Error>>
where
    R: ReadRequest + ?Sized,
    O: Send,
    F: FnOnce(Result<&R, Error>) -> Result<O, Error> + Send,
{
    func: F,
    _p: PhantomData<(*const R, O)>,
}

impl<R, O, F> GetFn<R, O, F>
where
    R: ReadRequest + ?Sized,
    O: Send,
    F: FnOnce(Result<&R, Error>) -> Result<O, Error> + Send,
{
    pub(crate) fn new(f: F) -> Self {
        Self {
            func: f,
            _p: PhantomData,
        }
    }
}

unsafe impl<R, O, F> Send for GetFn<R, O, F>
where
    R: ReadRequest + ?Sized,
    O: Send,
    F: FnOnce(Result<&R, Error>) -> Result<O, Error> + Send,
{
}

impl<R, O, F> Callback for GetFn<R, O, F>
where
    R: ReadRequest + ?Sized,
    O: Send,
    F: FnOnce(Result<&R, Error>) -> Result<O, Error> + Send,
{
    type Request = R;
    type Output = O;
    fn apply(self, input: Result<&Self::Request, Error>) -> Result<Self::Output, Error> {
        (self.func)(input)
    }
}
