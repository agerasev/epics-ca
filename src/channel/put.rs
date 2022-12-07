use super::{TypedChannel, UserData};
use crate::{
    error::{result_from_raw, Error},
    types::Scalar,
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

impl<T: Scalar> TypedChannel<T> {
    pub fn put_slice(&mut self, data: &[T]) -> Result<Put<'_, T>, Error> {
        self.context()
            .with(|| {
                let mut proc = self.user_data().process.lock().unwrap();
                result_from_raw(unsafe {
                    sys::ca_array_put_callback(
                        T::ENUM.raw() as _,
                        data.len() as _,
                        self.raw(),
                        data.as_ptr() as *const _,
                        Some(Self::callback),
                        proc.id() as _,
                    )
                })
                .map(|()| {
                    self.context().flush_io();
                    proc.result = None;
                })
            })
            .map(|()| Put { owner: self })
    }

    unsafe extern "C" fn callback(args: sys::event_handler_args) {
        println!("put_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        let mut proc = user_data.process.lock().unwrap();
        if proc.id() != args.usr as usize {
            return;
        }
        proc.result = Some(result_from_raw(args.status));
        user_data.waker.wake();
    }
}

pub struct Put<'a, T: Scalar> {
    owner: &'a mut TypedChannel<T>,
}

impl<'a, T: Scalar> Unpin for Put<'a, T> {}

impl<'a, T: Scalar> Future for Put<'a, T> {
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let user_data = self.owner.user_data();
        user_data.waker.register(cx.waker());
        let mut proc = user_data.process.lock().unwrap();
        match proc.result.take() {
            Some(status) => Poll::Ready(status),
            None => Poll::Pending,
        }
    }
}

impl<'a, T: Scalar> Drop for Put<'a, T> {
    fn drop(&mut self) {
        let mut proc = self.owner.user_data().process.lock().unwrap();
        proc.change_id();
        proc.result = None;
    }
}
