use crate::error::Error;
use std::{marker::PhantomData, ptr::NonNull};

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
        Self::detach();
        Ok(Self {
            raw: NonNull::new(raw).unwrap(),
        })
    }
    pub(crate) fn current() -> *mut sys::ca_client_context {
        unsafe { sys::ca_current_context() }
    }
    fn attach_unbounded(&self) {
        unsafe { sys::ca_attach_context(self.raw.as_ptr()) };
    }
    fn detach() {
        unsafe { sys::ca_detach_context() };
    }
    /// Panics if some context (including itself) already attached to this thread.
    pub fn attach(&self) -> AttachGuard<'_> {
        AttachGuard::new(self)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.attach_unbounded();
            sys::ca_context_destroy();
        }
    }
}

pub struct AttachGuard<'a> {
    owner: PhantomData<&'a Context>,
    /// To make guard non-Send.
    _unused: [*mut u8; 0],
}

impl<'a> AttachGuard<'a> {
    fn new(owner: &'a Context) -> Self {
        assert!(Context::current().is_null());
        owner.attach_unbounded();
        Self {
            owner: PhantomData,
            _unused: [],
        }
    }

    pub(crate) fn flush_io(&mut self) -> Result<(), Error> {
        Error::try_from_raw(unsafe { sys::ca_flush_io() })
    }
}

impl<'a> Drop for AttachGuard<'a> {
    fn drop(&mut self) {
        Context::detach();
    }
}

#[cfg(test)]
mod tests {
    use super::Context;

    #[test]
    fn attach() {
        Context::new().unwrap().attach();
    }

    #[test]
    #[should_panic]
    fn attach_twice() {
        let ctx = Context::new().unwrap();
        let guard = ctx.attach();
        ctx.attach(); // panic here
        drop(guard);
    }
}
