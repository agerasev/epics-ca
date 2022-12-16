use crate::error::{result_from_raw, Error};
use std::{ops::Deref, ptr::NonNull, sync::Arc};

/// Unique context.
///
/// Manages raw EPICS CA context.
#[derive(Debug)]
pub struct UniqueContext {
    raw: NonNull<sys::ca_client_context>,
}

unsafe impl Send for UniqueContext {}

impl UniqueContext {
    /// Create a new unique context.
    pub fn new() -> Result<Self, Error> {
        let prev = Self::current();
        if !prev.is_null() {
            Self::detach();
        }
        let ret = result_from_raw(unsafe {
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

    /// Perform some operation inside of the context.
    ///
    /// This calls can be safely nested (either from same context or different ones).
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

    /// Flush IO queue.
    ///
    /// **Must be called after almost any EPICS CA function to ensure it has an effect.**
    pub(crate) fn flush_io(&self) {
        self.with(|| result_from_raw(unsafe { sys::ca_flush_io() }))
            .unwrap()
    }
}

impl Drop for UniqueContext {
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

/// Shared context.
#[derive(Clone, Debug)]
pub struct Context {
    arc: Arc<UniqueContext>,
}

unsafe impl Send for Context {}

impl Deref for Context {
    type Target = UniqueContext;
    fn deref(&self) -> &Self::Target {
        &self.arc
    }
}

impl Context {
    /// Creates a new [`UniqueContext`] and shares it.
    pub fn new() -> Result<Self, Error> {
        UniqueContext::new().map(|uniq| Self {
            arc: Arc::new(uniq),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::UniqueContext as Context;

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
