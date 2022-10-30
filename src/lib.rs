mod sys;

pub mod error;
pub mod types;

use crate::{error::Error, types::DbRequest};
use std::{
    ffi::{self, CStr},
    marker::PhantomData,
    ptr::{self, NonNull},
    rc::Rc,
    time::Duration,
};

#[derive(Debug)]
pub struct Context {
    raw: NonNull<sys::ca_client_context>,
}
impl Context {
    pub fn new(preemptive: bool) -> Result<Self, Error> {
        let select = if preemptive {
            sys::ca_preemptive_callback_select::ca_enable_preemptive_callback
        } else {
            sys::ca_preemptive_callback_select::ca_disable_preemptive_callback
        };
        Error::try_from_raw(unsafe { sys::ca_context_create(select) })?;
        let raw = Self::current();
        unsafe { Self::detach() };
        Ok(Self {
            raw: NonNull::new(raw).unwrap(),
        })
    }
    fn current() -> *mut sys::ca_client_context {
        unsafe { sys::ca_current_context() }
    }
    unsafe fn attach_unchecked(&self) {
        sys::ca_attach_context(self.raw.as_ptr());
    }
    unsafe fn detach() {
        sys::ca_detach_context();
    }
    pub fn attach(&self) -> AttachGuard<'_> {
        AttachGuard::new(self)
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.attach_unchecked();
            sys::ca_context_destroy();
        }
    }
}
pub struct AttachGuard<'a> {
    owner: PhantomData<&'a Context>,
}
impl<'a> AttachGuard<'a> {
    fn new(owner: &'a Context) -> Self {
        assert!(Context::current().is_null());
        unsafe { owner.attach_unchecked() };
        Self { owner: PhantomData }
    }
}
impl<'a> Drop for AttachGuard<'a> {
    fn drop(&mut self) {
        unsafe { Context::detach() };
    }
}

fn pend_io(ctx: &Context, timeout: Duration) -> Result<(), Error> {
    let _guard = ctx.attach();
    Error::try_from_raw(unsafe { sys::ca_pend_io(timeout.as_secs_f64()) })
}

#[derive(Debug)]
pub struct Channel {
    ctx: Rc<Context>,
    raw: NonNull<ffi::c_void>,
}
impl Channel {
    pub fn new(ctx: Rc<Context>, name: &CStr) -> Result<Self, Error> {
        let mut raw: sys::chanId = ptr::null_mut();
        {
            let _guard = ctx.attach();
            Error::try_from_raw(unsafe {
                sys::ca_create_channel(name.as_ptr(), None, ptr::null_mut(), 0, &mut raw as *mut _)
            })?;
        }
        pend_io(&ctx, Duration::ZERO)?;
        let raw = NonNull::new(raw).unwrap();
        Ok(Self { ctx, raw })
    }
    pub fn context(&self) -> &Rc<Context> {
        &self.ctx
    }
    fn raw(&self) -> sys::chanId {
        self.raw.as_ptr()
    }
}
impl Drop for Channel {
    fn drop(&mut self) {
        let _guard = self.ctx.attach();
        unsafe { sys::ca_clear_channel(self.raw()) };
    }
}
