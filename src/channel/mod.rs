pub mod array;
pub mod base;
pub mod get;
pub mod put;
pub mod scalar;
pub mod subscribe;

pub use array::ArrayChannel;
pub use base::{Channel, Connect};
pub use get::{Get, GetFn};
pub use put::Put;
pub use scalar::ScalarChannel;
pub use subscribe::Subscription;

use crate::{context::Context, error::Error, types::Field};
use std::{ffi::CStr, sync::Arc};

impl Context {
    /// Create channel and wait for connection.
    pub async fn connect(self: Arc<Context>, name: &CStr) -> Result<Channel, Error> {
        let mut chan = Channel::new(self, name)?;
        chan.connected().await;
        Ok(chan)
    }

    /// Create channel, wait for connection, and try to cast it to typed one.
    pub async fn connect_typed<T: Field>(
        self: Arc<Context>,
        name: &CStr,
    ) -> Result<ArrayChannel<T>, Error> {
        let mut chan = Channel::new(self, name)?;
        chan.connected().await;
        chan.into_array::<T>().map_err(|(err, _)| err)
    }
}

#[cfg(test)]
mod tests;
