use crate::error::Error;
use std::ptr::NonNull;

#[derive(Debug, Eq, PartialEq)]
pub struct Context {
    raw: NonNull<sys::ca_client_context>,
}

unsafe impl Send for Context {}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let prev = Self::current();
        if !prev.is_null() {
            Self::detach();
        }
        let ret = Error::try_from_raw(unsafe {
            sys::ca_context_create(
                sys::ca_preemptive_callback_select::ca_enable_preemptive_callback,
            )
        })
        .map(|()| {
            let raw = Self::current();
            Self::detach();
            Self {
                raw: NonNull::new(raw).unwrap(),
            }
        });
        if let Some(prev) = NonNull::new(prev) {
            Self::attach(prev);
        }
        ret
    }
    pub(crate) fn current() -> *mut sys::ca_client_context {
        unsafe { sys::ca_current_context() }
    }
    fn attach(raw: NonNull<sys::ca_client_context>) {
        unsafe { sys::ca_attach_context(raw.as_ptr()) };
    }
    fn detach() {
        unsafe { sys::ca_detach_context() };
    }
    pub fn with<F: FnOnce() -> R, R>(&self, f: F) -> R {
        let prev = Self::current();
        if prev != self.raw.as_ptr() {
            if !prev.is_null() {
                Self::detach();
            }
            Self::attach(self.raw);
        }
        let ret = f();
        if prev != self.raw.as_ptr() {
            Self::detach();
            if let Some(prev) = NonNull::new(prev) {
                Self::attach(prev);
            }
        }
        ret
    }

    pub(crate) fn flush_io(&self) -> Result<(), Error> {
        self.with(|| Error::try_from_raw(unsafe { sys::ca_flush_io() }))
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let prev = Self::current();
        if !prev.is_null() {
            Self::detach();
        }
        Self::attach(self.raw);
        unsafe { sys::ca_context_destroy() };
        if let Some(prev) = NonNull::new(prev) {
            Self::attach(prev);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Context;

    #[test]
    fn new() {
        Context::new().unwrap();
    }

    #[test]
    fn attach() {
        assert!(Context::current().is_null());
        let ctx = Context::new().unwrap();
        assert!(Context::current().is_null());
        ctx.with(|| {
            assert_eq!(Context::current(), ctx.raw.as_ptr());
        });
        assert!(Context::current().is_null());
    }

    #[test]
    fn reattach_same() {
        assert!(Context::current().is_null());
        let ctx = Context::new().unwrap();
        assert!(Context::current().is_null());
        ctx.with(|| {
            assert_eq!(Context::current(), ctx.raw.as_ptr());
            ctx.with(|| {
                assert_eq!(Context::current(), ctx.raw.as_ptr());
            });
            assert_eq!(Context::current(), ctx.raw.as_ptr());
        });
        assert!(Context::current().is_null());
    }

    #[test]
    fn reattach_different() {
        assert!(Context::current().is_null());
        let ctx = Context::new().unwrap();
        assert!(Context::current().is_null());
        ctx.with(|| {
            assert_eq!(Context::current(), ctx.raw.as_ptr());
            let other_ctx = Context::new().unwrap();
            assert_eq!(Context::current(), ctx.raw.as_ptr());
            other_ctx.with(|| {
                assert_eq!(Context::current(), other_ctx.raw.as_ptr());
                ctx.with(|| {
                    assert_eq!(Context::current(), ctx.raw.as_ptr());
                });
                assert_eq!(Context::current(), other_ctx.raw.as_ptr());
            });
            assert_eq!(Context::current(), ctx.raw.as_ptr());
            drop(other_ctx);
            assert_eq!(Context::current(), ctx.raw.as_ptr());
        });
        assert!(Context::current().is_null());
    }
}
