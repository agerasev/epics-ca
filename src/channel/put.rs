use super::{Channel, TypedChannel, UserData};
use crate::{
    error::{result_from_raw, Error},
    types::{request::WriteRequest, Field},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

impl Channel {
    pub fn put_request<R: WriteRequest + ?Sized>(&mut self, request: &R) -> Result<Put<'_>, Error> {
        self.context()
            .with(|| {
                let mut proc = self.user_data().process.lock().unwrap();
                result_from_raw(unsafe {
                    sys::ca_array_put_callback(
                        R::ENUM.raw() as _,
                        request.len() as _,
                        self.raw(),
                        request as *const R as *const _,
                        Some(Self::callback),
                        proc.id() as _,
                    )
                })
                .map(|()| {
                    self.context().flush_io();
                    proc.put_res = None;
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
        proc.put_res = Some(result_from_raw(args.status));
        user_data.waker.wake();
    }
}

pub struct Put<'a> {
    owner: &'a mut Channel,
}

impl<'a> Unpin for Put<'a> {}

impl<'a> Future for Put<'a> {
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let user_data = self.owner.user_data();
        user_data.waker.register(cx.waker());
        let mut proc = user_data.process.lock().unwrap();
        match proc.put_res.take() {
            Some(status) => Poll::Ready(status),
            None => Poll::Pending,
        }
    }
}

impl<'a> Drop for Put<'a> {
    fn drop(&mut self) {
        let mut proc = self.owner.user_data().process.lock().unwrap();
        proc.change_id();
        proc.put_res = None;
    }
}

impl<T: Field> TypedChannel<T> {
    pub fn put_slice(&mut self, data: &[T]) -> Result<Put<'_>, Error> {
        self.put_request(data)
    }
}
