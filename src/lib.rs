pub mod error;
pub mod types;

#[cfg(test)]
mod tests;

use crate::error::Error;
use std::{
    ffi::CStr,
    ops::Deref,
    ptr::{self, NonNull},
    time::Duration,
};

trait Ptr {
    type NonNull;
}
impl<T> Ptr for *mut T {
    type NonNull = NonNull<T>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Context {
    raw: NonNull<sys::ca_client_context>,
}
impl Context {
    pub fn new() -> Result<Self, Error> {
        Error::try_from_raw(unsafe {
            sys::ca_context_create(
                sys::ca_preemptive_callback_select::ca_enable_preemptive_callback,
            )
        })?;
        let raw = Self::current();
        Ok(Self {
            raw: NonNull::new(raw).unwrap(),
        })
    }
    fn current() -> *mut sys::ca_client_context {
        unsafe { sys::ca_current_context() }
    }
    fn attach(&self) {
        unsafe { sys::ca_attach_context(self.raw.as_ptr()) };
    }
    fn detach() {
        unsafe { sys::ca_detach_context() };
    }
    fn assert_attached(&self) {
        debug_assert_eq!(self.raw.as_ptr(), Self::current());
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.attach();
            sys::ca_context_destroy();
        }
    }
}

fn pend_io(ctx: &Context, timeout: Duration) -> Result<(), Error> {
    ctx.assert_attached();
    Error::try_from_raw(unsafe { sys::ca_pend_io(timeout.as_secs_f64()) })
}

#[derive(Debug)]
pub struct Channel<C: Deref<Target = Context> + Clone> {
    ctx: C,
    raw: <sys::chanId as Ptr>::NonNull,
}
impl<C: Deref<Target = Context> + Clone> Channel<C> {
    pub fn new(ctx: C, name: &CStr) -> Result<Self, Error> {
        let mut raw: sys::chanId = ptr::null_mut();
        ctx.attach();
        Error::try_from_raw(unsafe {
            sys::ca_create_channel(name.as_ptr(), None, ptr::null_mut(), 0, &mut raw as *mut _)
        })?;
        pend_io(&ctx, Duration::ZERO)?;
        let raw = NonNull::new(raw).unwrap();
        Ok(Self { ctx, raw })
    }
    pub fn context(&self) -> C {
        self.ctx.clone()
    }
    fn raw(&self) -> sys::chanId {
        self.raw.as_ptr()
    }
}
impl<C: Deref<Target = Context> + Clone> Drop for Channel<C> {
    fn drop(&mut self) {
        self.ctx.attach();
        unsafe { sys::ca_clear_channel(self.raw()) };
    }
}
