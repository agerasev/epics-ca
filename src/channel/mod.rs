//mod array;
mod base;
mod get;
mod put;
//mod scalar;
mod subscribe;

//pub use array::*;
pub use base::*;
pub use get::*;
pub use put::*;
//pub use scalar::*;
pub use subscribe::*;

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

//#[cfg(test)]
//mod tests;
