//! Client library for EPICS Channel Access protocol.
//!
//! More info about:
//!
//! + [Channels](channel)
//! + [Requests](request)
//!

/// Channels
pub mod channel;
/// Context
pub mod context;
/// Error types
pub mod error;
/// Different types of requests
pub mod request;
/// Native EPICS types
pub mod types;
mod utils;

pub use channel::{Channel, TypedChannel, ValueChannel};
pub use context::Context;
pub use error::Error;
