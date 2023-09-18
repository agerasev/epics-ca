use super::{get::Callback, subscribe::Queue, Get, Put, Subscription};
use crate::{
    context::Context,
    error::{self, result_from_raw, Error},
    request::WriteRequest,
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
        Mutex,
    },
    task::{Context as Cx, Poll},
};

/// Basic channel.
///
/// Channel is an entity that has a name and could be read, written or subscribed to.
#[derive(Debug)]
pub struct Channel {
    ctx: Context,
    raw: <sys::chanId as Ptr>::NonNull,
}

unsafe impl Send for Channel where Context: Send {}

impl Channel {
    /// Create channel without waiting for connection.
    pub fn new(ctx: &Context, name: &CStr) -> Result<Self, Error> {
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
                        ctx: ctx.clone(),
                        raw: NonNull::new(raw).unwrap(),
                    })
                }
                Err(e) => {
                    drop(unsafe { Box::from_raw(puser) });
                    Err(e)
                }
            }
        })
    }
    /// Wait for channel become connected.
    pub fn connected(&mut self) -> Connect<'_> {
        Connect::new(self)
    }
    /// Context of the channel.
    pub fn context(&self) -> &Context {
        &self.ctx
    }
    /// Raw channed identifier.
    pub fn raw(&self) -> sys::chanId {
        self.raw.as_ptr()
    }
    pub(crate) fn user_data(&self) -> &UserData {
        unsafe { &*(sys::ca_puser(self.raw.as_ptr()) as *const UserData) }
    }
    /// Channel name.
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::ca_name(self.raw())) }
    }
    /// Channel field type.
    pub fn field_type(&self) -> Result<FieldId, Error> {
        let raw = unsafe { sys::ca_field_type(self.raw()) } as i32;
        if raw == sys::TYPENOTCONN {
            return Err(error::DISCONN);
        }
        FieldId::try_from_raw(raw).ok_or(error::BADTYPE)
    }
    /// Number of elements in the channel.
    pub fn element_count(&self) -> Result<usize, Error> {
        let count = unsafe { sys::ca_element_count(self.raw()) } as usize;
        if count == 0 {
            return Err(error::DISCONN);
        }
        Ok(count)
    }
    /// Name of the host which serves the channel.
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
            drop(unsafe { Box::from_raw(puser) });
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

pub(crate) struct ProcessData {
    id_counter: usize,
    pub(crate) data: *mut u8,
    pub(crate) put_res: Option<Result<(), Error>>,
}

impl ProcessData {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            data: ptr::null_mut(),
            put_res: None,
        }
    }
    pub fn id(&self) -> usize {
        self.id_counter
    }
    pub fn change_id(&mut self) {
        self.id_counter += 1;
    }
}

/// Future to wait for connection.
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
    /// Make write request by reference.
    pub fn put_ref<R: WriteRequest + ?Sized>(&mut self, req: &R) -> Result<Put<'_>, Error> {
        Put::new(self, req)
    }
    /// Make read request and call closure when it's done, successfully or not.
    pub fn get_with<F: Callback>(&mut self, func: F) -> Get<'_, F> {
        Get::new(self, func)
    }
    /// Subscribe to channel updates and call closure each time when update occured.
    pub fn subscribe_with<F: Queue>(&mut self, func: F) -> Subscription<'_, F> {
        Subscription::new(self, func)
    }
}

#[cfg(test)]
mod tests {
    use crate::{context::UniqueContext, Channel, Context};
    use async_std::{task::sleep, test as async_test};
    use cstr::cstr;
    use futures::{select, FutureExt};
    use serial_test::serial;
    use std::{ptr, time::Duration};

    #[async_test]
    #[serial]
    async fn connect() {
        let ctx = Context::new().unwrap();
        Channel::new(&ctx, cstr!("ca:test:ai"))
            .unwrap()
            .connected()
            .await;
    }

    #[async_test]
    async fn connect_nonexistent() {
        let mut chan = Channel::new(&Context::new().unwrap(), cstr!("__nonexistent__")).unwrap();
        select! {
            _ = chan.connected() => panic!(),
            _ = sleep(Duration::from_millis(100)).fuse() => (),
        }
    }

    #[async_test]
    #[serial]
    async fn user_data() {
        let ctx = Context::new().unwrap();
        let mut channel = Channel::new(&ctx, cstr!("ca:test:ai")).unwrap();
        channel.connected().await;

        // Test that user data can be accessed without context attachment.
        assert!(UniqueContext::current().is_null());
        let user_data = channel.user_data();
        ctx.with(|| {
            assert!(ptr::eq(channel.user_data(), user_data));
        });
    }
}
