use crate::{
    context::Context,
    error::{self, result_from_raw, Error},
    traits::Ptr,
    types::DbField,
};
use futures::task::AtomicWaker;
use std::{
    ffi::{c_void, CStr},
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
pub struct AnyChannel {
    ctx: Arc<Context>,
    raw: <sys::chanId as Ptr>::NonNull,
}

unsafe impl Send for AnyChannel where Context: Send {}

impl AnyChannel {
    pub fn connect(ctx: Arc<Context>, name: &CStr) -> Connect {
        Connect::new(ctx, name)
    }

    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    fn raw(&self) -> sys::chanId {
        self.raw.as_ptr()
    }

    pub(crate) fn user_data(&self) -> *mut c_void {
        unsafe { sys::ca_puser(self.raw.as_ptr()) }
    }

    pub(crate) fn set_user_data(&mut self, ptr: *mut c_void) {
        unsafe { sys::ca_set_puser(self.raw.as_ptr(), ptr) };
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::ca_name(self.raw())) }
    }

    pub fn field_type(&self) -> Result<DbField, Error> {
        let raw = unsafe { sys::ca_field_type(self.raw()) } as i32;
        if raw == sys::TYPENOTCONN {
            return Err(error::DISCONN);
        }
        DbField::try_from_raw(raw).ok_or(error::BADTYPE)
    }

    pub fn element_count(&self) -> Result<usize, Error> {
        let count = unsafe { sys::ca_element_count(self.raw()) } as usize;
        if count == 0 {
            return Err(error::DISCONN);
        }
        Ok(count)
    }

    pub fn host_name(&self) -> Result<&CStr, Error> {
        const DISCONN_HOST: &CStr =
            unsafe { CStr::from_bytes_with_nul_unchecked(b"<disconnected>\0") };

        let str = unsafe { CStr::from_ptr(sys::ca_host_name(self.raw())) };
        if str != DISCONN_HOST {
            Ok(str)
        } else {
            Err(error::DISCONN)
        }
    }
}

impl Drop for AnyChannel {
    fn drop(&mut self) {
        self.ctx
            .with(|| unsafe { sys::ca_clear_channel(self.raw()) });
    }
}

enum ConnectStage<'a> {
    Init { name: &'a CStr },
    Connecting { channel: AnyChannel },
    Done,
}

struct ConnectShared {
    waker: AtomicWaker,
    op: AtomicI32,
}

pub struct Connect<'a> {
    ctx: Arc<Context>,
    stage: ConnectStage<'a>,
    shared: ConnectShared,
}

impl<'a> Connect<'a> {
    fn new(ctx: Arc<Context>, name: &'a CStr) -> Self {
        Connect {
            ctx,
            stage: ConnectStage::Init { name },
            shared: ConnectShared {
                waker: AtomicWaker::new(),
                op: AtomicI32::new(-1),
            },
        }
    }
}

impl<'a> Future for Connect<'a> {
    type Output = Result<AnyChannel, Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Cx<'_>) -> Poll<Self::Output> {
        self.shared.waker.register(cx.waker());
        let stage = match mem::replace(&mut self.stage, ConnectStage::Done) {
            ConnectStage::Init { name } => {
                let mut raw: sys::chanId = ptr::null_mut();
                self.ctx.with(|| {
                    result_from_raw(unsafe {
                        sys::ca_create_channel(
                            name.as_ptr(),
                            Some(Connect::callback),
                            &self.shared as *const _ as *mut c_void,
                            0,
                            &mut raw as *mut _,
                        )
                    })
                    .and_then(|()| self.ctx.flush_io())
                })?;
                ConnectStage::Connecting {
                    channel: AnyChannel {
                        ctx: self.ctx.clone(),
                        raw: NonNull::new(raw).unwrap(),
                    },
                }
            }
            ConnectStage::Connecting { mut channel } => {
                match self.shared.op.load(Ordering::Acquire) {
                    -1 => ConnectStage::Connecting { channel }, // spurious wakeup
                    other => {
                        return {
                            channel.set_user_data(ptr::null_mut());
                            match other {
                                sys::CA_OP_CONN_UP => Poll::Ready(Ok(channel)),
                                sys::CA_OP_CONN_DOWN => Poll::Ready(Err(error::DISCONN)),
                                _ => unreachable!(),
                            }
                        };
                    }
                }
            }
            ConnectStage::Done => panic!("Connect is already done"),
        };
        let _ = mem::replace(&mut self.stage, stage);
        Poll::Pending
    }
}

impl<'a> Drop for Connect<'a> {
    fn drop(&mut self) {
        if let ConnectStage::Connecting { channel } = &mut self.stage {
            channel.set_user_data(ptr::null_mut());
        }
    }
}

impl<'a> Connect<'a> {
    unsafe extern "C" fn callback(args: sys::connection_handler_args) {
        if let Some(user_data) = (sys::ca_puser(args.chid) as *const ConnectShared).as_ref() {
            user_data.op.store(args.op as i32, Ordering::Release);
            user_data.waker.wake();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::DbField, AnyChannel, Context};
    use async_std::test as async_test;
    use c_str_macro::c_str;
    use serial_test::serial;
    use std::{ptr, sync::Arc};

    #[async_test]
    #[serial]
    async fn connect() {
        let ctx = Arc::new(Context::new().unwrap());
        AnyChannel::connect(ctx, c_str!("ca:test:ai"))
            .await
            .unwrap();
    }

    #[async_test]
    #[serial]
    async fn properties() {
        let name = c_str!("ca:test:ai");
        let chan = AnyChannel::connect(Arc::new(Context::new().unwrap()), name)
            .await
            .unwrap();
        assert_eq!(chan.name(), name);
        assert_eq!(chan.field_type().unwrap(), DbField::Double);
        assert_eq!(chan.element_count().unwrap(), 1);
    }

    #[async_test]
    #[serial]
    async fn user_data() {
        let ctx = Arc::new(Context::new().unwrap());
        let mut channel = AnyChannel::connect(ctx.clone(), c_str!("ca:test:ai"))
            .await
            .unwrap();

        // Test that user data can be accessed without context attachment.
        assert!(Context::current().is_null());
        assert!(channel.user_data().is_null());
        channel.set_user_data(0xdeadbeef as *mut _);
        assert_eq!(channel.user_data() as usize, 0xdeadbeef);
        ctx.with(|| {
            assert_eq!(channel.user_data() as usize, 0xdeadbeef);
        });
        channel.set_user_data(ptr::null_mut());
        assert!(channel.user_data().is_null());
    }
}
