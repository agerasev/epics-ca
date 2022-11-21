use crate::{
    context::{pend_io, Context},
    error::Error,
    traits::Ptr,
};
use futures::task::AtomicWaker;
use std::{
    ffi::CStr,
    future::Future,
    pin::Pin,
    ptr::{self, NonNull},
    sync::Arc,
    task::{Context as Cx, Poll},
    time::Duration,
};

#[derive(Debug)]
pub struct Channel {
    ctx: Arc<Context>,
    raw: <sys::chanId as Ptr>::NonNull,
}

impl Channel {
    pub fn connect(ctx: Arc<Context>, name: &CStr) -> Result<Connect, Error> {
        let mut raw: sys::chanId = ptr::null_mut();
        ctx.attach();
        Error::try_from_raw(unsafe {
            sys::ca_create_channel(name.as_ptr(), None, ptr::null_mut(), 0, &mut raw as *mut _)
        })?;
        pend_io(&ctx, Duration::ZERO)?;
        let raw = NonNull::new(raw).unwrap();
        Ok(Connect {
            owner: Self { ctx, raw },
            waker: AtomicWaker::new(),
        })
    }

    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    fn raw(&self) -> sys::chanId {
        self.raw.as_ptr()
    }
}

pub struct Connect {
    owner: Channel,
    waker: AtomicWaker,
}

impl Connect {}

impl Future for Connect {
    type Output = Result<Channel, Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Cx<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        self.ctx.attach();
        unsafe { sys::ca_clear_channel(self.raw()) };
    }
}

impl Channel {}
