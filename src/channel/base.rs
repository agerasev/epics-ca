use super::{array::ProcessData, get::Callback, subscribe::Queue, Get, Subscription};
use crate::{
    context::Context,
    error::{self, result_from_raw, Error},
    types::FieldId,
    utils::Ptr,
};
use futures::{future::FusedFuture, task::AtomicWaker};
use std::{
    ffi::{c_void, CStr},
    future::Future,
    pin::Pin,
    ptr::{self, NonNull},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Context as Cx, Poll},
};

#[derive(Debug)]
pub struct Channel {
    ctx: Arc<Context>,
    raw: <sys::chanId as Ptr>::NonNull,
}

unsafe impl Send for Channel where Context: Send {}

impl Channel {
    /// Create channel without waiting for connection.
    pub fn new(ctx: Arc<Context>, name: &CStr) -> Result<Self, Error> {
        ctx.clone().with(|| {
            let mut raw: sys::chanId = ptr::null_mut();
            let puser = Box::leak(Box::new(UserData::new())) as *mut UserData;
            const DEFAULT_PRIORITY: u32 = 0;

            match result_from_raw(unsafe {
                sys::ca_create_channel(
                    name.as_ptr(),
                    Some(Self::connect_callback),
                    puser as *mut c_void,
                    DEFAULT_PRIORITY,
                    &mut raw as *mut _,
                )
            }) {
                Ok(()) => {
                    ctx.flush_io();
                    Ok(Channel {
                        ctx,
                        raw: NonNull::new(raw).unwrap(),
                    })
                }
                Err(e) => {
                    unsafe { Box::from_raw(puser) };
                    Err(e)
                }
            }
        })
    }
    /// Wait for channel become connected.
    pub fn connected(&mut self) -> Connect<'_> {
        Connect::new(self)
    }

    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }
    pub(crate) fn raw(&self) -> sys::chanId {
        self.raw.as_ptr()
    }
    pub(crate) fn user_data(&self) -> &UserData {
        unsafe { &*(sys::ca_puser(self.raw.as_ptr()) as *const UserData) }
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::ca_name(self.raw())) }
    }
    pub fn field_type(&self) -> Result<FieldId, Error> {
        let raw = unsafe { sys::ca_field_type(self.raw()) } as i32;
        if raw == sys::TYPENOTCONN {
            return Err(error::DISCONN);
        }
        FieldId::try_from_raw(raw).ok_or(error::BADTYPE)
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

impl Drop for Channel {
    fn drop(&mut self) {
        self.context().with(|| {
            let puser = self.user_data() as *const _ as *mut UserData;
            result_from_raw(unsafe { sys::ca_clear_channel(self.raw()) }).unwrap();
            unsafe { Box::from_raw(puser) };
        });
    }
}

pub(crate) struct UserData {
    pub(crate) waker: AtomicWaker,
    pub(crate) connected: AtomicBool,
    pub(crate) process: Mutex<ProcessData>,
}

impl UserData {
    fn new() -> Self {
        Self {
            connected: AtomicBool::new(false),
            waker: AtomicWaker::new(),
            process: Mutex::new(ProcessData::new()),
        }
    }
}

#[must_use]
pub struct Connect<'a> {
    channel: Option<&'a mut Channel>,
}

impl<'a> Connect<'a> {
    fn new(channel: &'a mut Channel) -> Self {
        Connect {
            channel: Some(channel),
        }
    }
}

impl<'a> Future for Connect<'a> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Cx<'_>) -> Poll<Self::Output> {
        let channel = self.channel.take().unwrap();
        channel.user_data().waker.register(cx.waker());
        if channel.user_data().connected.load(Ordering::Acquire) {
            Poll::Ready(())
        } else {
            self.channel.replace(channel);
            Poll::Pending
        }
    }
}

impl<'a> FusedFuture for Connect<'a> {
    fn is_terminated(&self) -> bool {
        self.channel.is_none()
    }
}

impl Channel {
    unsafe extern "C" fn connect_callback(args: sys::connection_handler_args) {
        println!("connect_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        user_data.connected.store(
            match args.op as _ {
                sys::CA_OP_CONN_UP => true,
                sys::CA_OP_CONN_DOWN => false,
                _ => unreachable!(),
            },
            Ordering::Release,
        );
        user_data.waker.wake();
    }
}

impl Channel {
    pub fn get_with<F: Callback>(&mut self, func: F) -> Get<'_, F> {
        Get::new(self, func)
    }
    pub fn subscribe_with<F: Queue>(&mut self, func: F) -> Subscription<'_, F> {
        Subscription::new(self, func)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Channel, Context};
    use async_std::{task::sleep, test as async_test};
    use c_str_macro::c_str;
    use futures::{select, FutureExt};
    use serial_test::serial;
    use std::{ptr, time::Duration};

    #[async_test]
    #[serial]
    async fn connect() {
        let ctx = Context::new().unwrap();
        Channel::new(ctx, c_str!("ca:test:ai"))
            .unwrap()
            .connected()
            .await;
    }

    #[async_test]
    async fn connect_nonexistent() {
        let mut chan = Channel::new(Context::new().unwrap(), c_str!("__nonexistent__")).unwrap();
        select! {
            _ = chan.connected() => panic!(),
            _ = sleep(Duration::from_millis(100)).fuse() => (),
        }
    }

    #[async_test]
    #[serial]
    async fn user_data() {
        let ctx = Context::new().unwrap();
        let mut channel = Channel::new(ctx.clone(), c_str!("ca:test:ai")).unwrap();
        channel.connected().await;

        // Test that user data can be accessed without context attachment.
        assert!(Context::current().is_null());
        let user_data = channel.user_data();
        ctx.with(|| {
            assert!(ptr::eq(channel.user_data(), user_data));
        });
    }
}
