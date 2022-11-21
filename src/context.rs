use crate::error::Error;
use std::{ptr::NonNull, time::Duration};

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
    pub(crate) fn current() -> *mut sys::ca_client_context {
        unsafe { sys::ca_current_context() }
    }
    pub(crate) fn attach(&self) {
        unsafe { sys::ca_attach_context(self.raw.as_ptr()) };
    }
    pub(crate) fn is_attached(&self) -> bool {
        Self::current() == self.raw.as_ptr()
    }
    pub(crate) fn detach() {
        debug_assert!(!Self::current().is_null());
        unsafe { sys::ca_detach_context() };
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

/// Flush I/O to the server.
pub(crate) fn flush_io(ctx: &Context) -> Result<(), Error> {
    debug_assert!(ctx.is_attached());
    Error::try_from_raw(unsafe { sys::ca_flush_io() })
}

/// Flush I/O and wait for completion or timeout.
pub(crate) fn pend_io(ctx: &Context, timeout: Duration) -> Result<(), Error> {
    debug_assert!(ctx.is_attached());
    Error::try_from_raw(unsafe { sys::ca_pend_io(timeout.as_secs_f64()) })
}
