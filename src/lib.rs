mod sys;

pub mod error;
pub mod types;

use error::Error;

#[derive(Debug)]
pub struct Context {
    raw: *mut sys::ca_client_context,
}

impl Context {
    pub fn new(preemptive: bool) -> Result<Self, Error> {
        let select = if preemptive {
            sys::ca_preemptive_callback_select::ca_enable_preemptive_callback
        } else {
            sys::ca_preemptive_callback_select::ca_disable_preemptive_callback
        };
        Error::try_from_raw(unsafe { sys::ca_context_create(select) })?;
        let raw = unsafe { sys::ca_current_context() };
        assert!(!raw.is_null());
        Self::detach();
        Ok(Self { raw })
    }
    fn attach(&mut self) {
        unsafe { sys::ca_attach_context(self.raw) };
    }
    fn detach() {
        unsafe { sys::ca_detach_context() };
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        self.attach();
        unsafe { sys::ca_context_destroy() };
    }
}

#[derive(Debug)]
pub struct Channel {
    raw: sys::chanId,
}
