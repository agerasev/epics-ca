use super::{Callback, Channel, UserData};
use crate::{
    error::{result_from_raw, Error},
    types::{DbField, Scalar, Type},
};
use std::{
    cell::UnsafeCell,
    future::Future,
    marker::PhantomData,
    mem,
    pin::Pin,
    task::{Context, Poll},
};

impl<T: Type + ?Sized> Channel<T> {
    pub async fn get_with<F: FnOnce(&T) -> R, R>(&mut self, func: F) -> Result<R, Error> {
        Get::new(self, func).await
    }
}
/*
impl<T: Type + Default> Channel<T> {
    pub async fn get_copy(&mut self) -> Result<T, Error> {
        let mut value = T::default();
        assert_eq!(self.get(&mut value)?.await?, 1);
        Ok(value)
    }
}
impl<T: Scalar + Default + Clone> Channel<[T]> {
    pub async fn get_vec(&mut self) -> Result<Vec<T>, Error> {
        let mut data = vec![T::default(); self.count];
        let len = self.get(&mut data)?.await?;
        data.truncate(len);
        Ok(data)
    }
}
*/

enum GetState<T: Type + ?Sized, F: FnOnce(&T) -> R, R> {
    Waiting(F),
    Complete(R, PhantomData<T>),
}

#[must_use]
pub struct Get<'a, T: Type + ?Sized, F: FnOnce(&T) -> R, R> {
    owner: &'a mut Channel<T>,
    /// Must be locked by `owner.user_data().process` mutex
    callback: UnsafeCell<GetState<T, F, R>>,
    started: bool,
}

impl<'a, T: Type + ?Sized, F: FnOnce(&T) -> R, R> Get<'a, T, F, R> {
    fn new(owner: &'a mut Channel<T>, func: F) -> Self {
        Self {
            owner,
            callback: UnsafeCell::new(GetState::Waiting(func)),
            started: false,
        }
    }

    fn start(self: Pin<&mut Self>) -> Result<(), Error> {
        assert!(!self.started);
        let mut owner = self.owner;
        owner.context().with(|| {
            let mut proc = owner.user_data().process.lock().unwrap();
            proc.callback = Some(self.callback.get_mut() as *mut dyn Callback);
            result_from_raw(unsafe {
                sys::ca_array_get_callback(
                    owner.dbf as _,
                    owner.count as _,
                    owner.raw(),
                    Some(Self::get_callback),
                    proc.id() as _,
                )
            })
            .map(|()| {
                owner.context().flush_io();
                proc.status = None;
                self.started = true
            })
        })
    }

    unsafe extern "C" fn get_callback(args: sys::event_handler_args) {
        println!("get_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        let mut proc = user_data.process.lock().unwrap();
        if proc.id() != args.usr as usize {
            return;
        }
        let status = result_from_raw(args.status);
        let callback = Pin::new_unchecked(&mut *(proc.callback.take().unwrap()));
        if status.is_ok() {
            debug_assert!(T::match_field(
                DbField::try_from_raw(args.type_ as _).unwrap()
            ));
            callback.process(args.dbr as *const u8, args.count as usize);
        }
        proc.status = Some(status);
        user_data.waker.wake();
    }
}

impl<'a, T: Type + ?Sized, F: FnOnce(&T) -> R, R> Future for Get<'a, T, F, R> {
    type Output = Result<R, Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let user_data = self.owner.user_data();
        user_data.waker.register(cx.waker());
        let mut proc = user_data.process.lock().unwrap();
        match proc.status.take() {
            Some(status) => Poll::Ready(status.map(|()| proc.count)),
            None => Poll::Pending,
        }
    }
}

impl<'a, T: Type + ?Sized, F: FnOnce(&T) -> R, R> Drop for Get<'a, T, F, R> {
    fn drop(&mut self) {
        self.owner.user_data().process.lock().unwrap().change_id();
    }
}

impl<T: Type + ?Sized, F: FnOnce(&T) -> R, R> Callback for GetState<T, F, R> {
    fn process(mut self: Pin<&mut Self>, data: *const u8, count: usize) {
        *self = GetState::Complete(
            match unsafe { self.get_unchecked_mut() } {
                GetState::Waiting(f) => f(unsafe { T::from_ptr(data, count) }),
                _ => unreachable!(),
            },
            PhantomData,
        );
    }
}
