use super::{base::UserData, Channel};
use crate::{
    error::{result_from_raw, Error},
    request::WriteRequest,
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// Future that waits for write request is done, successfully or not.
///
/// *Waiting for this future to complete is optional.
/// The write can be done successfully even if it dropped before completion.*
pub struct Put<'a> {
    owner: &'a mut Channel,
}

impl<'a> Unpin for Put<'a> {}

impl<'a> Put<'a> {
    pub fn new<R: WriteRequest + ?Sized>(
        owner: &'a mut Channel,
        request: &R,
    ) -> Result<Self, Error> {
        owner
            .context()
            .with(|| {
                let mut proc = owner.user_data().process.lock().unwrap();
                result_from_raw(unsafe {
                    sys::ca_array_put_callback(
                        R::ID.raw() as _,
                        request.len() as _,
                        owner.raw(),
                        request as *const R as *const _,
                        Some(Self::callback),
                        proc.id() as _,
                    )
                })
                .map(|()| {
                    owner.context().flush_io();
                    proc.put_res = None;
                })
            })
            .map(|()| Self { owner })
    }

    unsafe extern "C" fn callback(args: sys::event_handler_args) {
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        let mut proc = user_data.process.lock().unwrap();
        if proc.id() != args.usr as usize {
            return;
        }
        proc.put_res = Some(result_from_raw(args.status));
        user_data.waker.wake();
    }
}

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
