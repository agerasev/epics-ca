mod any;
mod typed;

pub use any::*;
pub use typed::*;

use crate::{context::Context, error::Error, types::Type};
use std::{ffi::CStr, sync::Arc};

impl Context {
    /// Create channel and wait for connection.
    pub async fn connect(self: Arc<Context>, name: &CStr) -> Result<AnyChannel, Error> {
        let mut chan = AnyChannel::new(self, name)?;
        chan.connected().await;
        Ok(chan)
    }

    /// Create channel, wait for connection, and try to cast it to typed one.
    pub async fn connect_typed<T: Type + ?Sized>(
        self: Arc<Context>,
        name: &CStr,
    ) -> Result<Channel<T>, Error> {
        let mut chan = AnyChannel::new(self, name)?;
        chan.connected().await;
        chan.into_typed::<T>().map_err(|(err, _)| err)
    }
}

#[cfg(test)]
mod tests;
