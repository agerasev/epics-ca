use super::{Channel, UserData};
use crate::{
    error::{result_from_raw, Error},
    types::Type,
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

impl<T: Type + ?Sized> Channel<T> {
    pub fn put(&mut self, data: &T) -> Result<Put<'_, T>, Error> {
        self.context()
            .with(|| {
                let mut proc = self.user_data().process.lock().unwrap();
                result_from_raw(unsafe {
                    sys::ca_array_put_callback(
                        self.dbf as _,
                        data.element_count() as _,
                        self.raw(),
                        data.as_ptr() as _,
                        Some(Self::put_callback),
                        proc.id() as _,
                    )
                })
                .map(|()| {
                    self.context().flush_io();
                    proc.status = None;
                })
            })
            .map(|()| Put { owner: self })
    }
}

pub struct Put<'a, T: Type + ?Sized> {
    owner: &'a mut Channel<T>,
}

impl<'a, T: Type + ?Sized> Unpin for Put<'a, T> {}

impl<'a, T: Type + ?Sized> Future for Put<'a, T> {
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let user_data = self.owner.user_data();
        user_data.waker.register(cx.waker());
        let mut proc = user_data.process.lock().unwrap();
        match proc.status.take() {
            Some(status) => Poll::Ready(status),
            None => Poll::Pending,
        }
    }
}

impl<'a, T: Type + ?Sized> Drop for Put<'a, T> {
    fn drop(&mut self) {
        self.owner.user_data().process.lock().unwrap().change_id();
    }
}

impl<T: Type + ?Sized> Channel<T> {
    unsafe extern "C" fn put_callback(args: sys::event_handler_args) {
        println!("put_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        let mut proc = user_data.process.lock().unwrap();
        if proc.id() != args.usr as usize {
            return;
        }
        proc.status = Some(result_from_raw(args.status));
        user_data.waker.wake();
    }
}
