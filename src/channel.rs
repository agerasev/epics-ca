use crate::{context::Context, error::Error, traits::Ptr};
use futures::task::AtomicWaker;
use std::{
    ffi::{c_void, CStr, CString},
    future::Future,
    mem,
    pin::Pin,
    ptr::{self, NonNull},
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
    task::{Context as Cx, Poll},
};

#[derive(Debug)]
pub struct Channel {
    ctx: Arc<Context>,
    raw: <sys::chanId as Ptr>::NonNull,
}

impl Channel {
    pub fn connect(ctx: Arc<Context>, name: &CStr) -> Connect {
        Connect::new(ctx, name)
    }

    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    fn raw(&self) -> sys::chanId {
        self.raw.as_ptr()
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        self.ctx.attach();
        unsafe { sys::ca_clear_channel(self.raw()) };
    }
}

enum ConnectStage {
    Init { name: CString },
    Connecting { channel: Channel },
    Done,
}

struct ConnectShared {
    waker: AtomicWaker,
    op: AtomicI32,
}

pub struct Connect {
    ctx: Arc<Context>,
    stage: ConnectStage,
    shared: ConnectShared,
}

impl Connect {
    fn new(ctx: Arc<Context>, name: &CStr) -> Self {
        Connect {
            ctx,
            stage: ConnectStage::Init {
                name: CString::from(name),
            },
            shared: ConnectShared {
                waker: AtomicWaker::new(),
                op: AtomicI32::new(-1),
            },
        }
    }

    unsafe extern "C" fn callback(args: sys::connection_handler_args) {
        if let Some(user_data) = (sys::ca_puser(args.chid) as *const ConnectShared).as_ref() {
            user_data.op.store(args.op as i32, Ordering::Release);
            user_data.waker.wake();
        }
    }
}

impl Future for Connect {
    type Output = Result<Channel, Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Cx<'_>) -> Poll<Self::Output> {
        self.shared.waker.register(cx.waker());
        let stage = match mem::replace(&mut self.stage, ConnectStage::Done) {
            ConnectStage::Init { name } => {
                let mut raw: sys::chanId = ptr::null_mut();
                let mut guard = self.ctx.attach();
                Error::try_from_raw(unsafe {
                    sys::ca_create_channel(
                        name.as_ptr(),
                        Some(Connect::callback),
                        &self.shared as *const _ as *mut c_void,
                        0,
                        &mut raw as *mut _,
                    )
                })?;
                guard.flush_io()?;
                ConnectStage::Connecting {
                    channel: Channel {
                        ctx: self.ctx.clone(),
                        raw: NonNull::new(raw).unwrap(),
                    },
                }
            }
            ConnectStage::Connecting { channel } => match self.shared.op.load(Ordering::Acquire) {
                -1 => ConnectStage::Connecting { channel }, // spurious wakeup
                other => {
                    return {
                        unsafe { sys::ca_set_puser(channel.raw(), ptr::null_mut()) };
                        match other {
                            sys::CA_OP_CONN_UP => Poll::Ready(Ok(channel)),
                            sys::CA_OP_CONN_DOWN => Poll::Ready(Err(Error::try_from_raw(
                                sys::ECA_DISCONNCHID,
                            )
                            .unwrap_err())),
                            _ => unreachable!(),
                        }
                    };
                }
            },
            ConnectStage::Done => panic!("Connect is already done"),
        };
        let _ = mem::replace(&mut self.stage, stage);
        Poll::Pending
    }
}

impl Drop for Connect {
    fn drop(&mut self) {
        if let ConnectStage::Connecting { channel } = &self.stage {
            unsafe { sys::ca_set_puser(channel.raw(), ptr::null_mut()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Channel, Context};
    use async_std::test as async_test;
    use c_str_macro::c_str;
    use serial_test::serial;
    use std::sync::Arc;

    #[async_test]
    #[serial]
    async fn connect() {
        let ctx = Arc::new(Context::new().unwrap());
        Channel::connect(ctx, c_str!("ca:test:ai")).await.unwrap();
    }
}
