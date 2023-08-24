//! There are three kinds of channels:
//!
//! + [`Channel`] - raw channel without knowledge of what is inside.
//! + [`TypedChannel`] - channel that knows type of its items and whether it is scalar or array. Created with [`Channel::into_typed`].
//! + [`ValueChannel`] - convenience wrapper around [`TypedChannel`].
//!   Recommended to use when you need PV values only, not metadata. Created by [`Context::connect`] or [`TypedChannel::into_value`].
//!

pub mod base;
pub mod get;
pub mod put;
pub mod subscribe;
pub mod typed;
pub mod value;

pub use base::{Channel, Connect};
pub use get::{Get, GetFn};
pub use put::Put;
pub use subscribe::Subscription;
pub use typed::TypedChannel;
pub use value::ValueChannel;

use crate::{context::Context, error::Error, types::Value};
use std::ffi::CStr;

impl Context {
    /// Create channel, wait for connection, and try to cast it to typed one.
    pub async fn connect<V: Value + ?Sized>(&self, name: &CStr) -> Result<ValueChannel<V>, Error> {
        let mut chan = Channel::new(self, name)?;
        chan.connected().await;
        let typed = chan.into_typed::<V>().map_err(|(err, _)| err)?;
        Ok(typed.into_value())
    }
}

#[cfg(test)]
mod tests;
