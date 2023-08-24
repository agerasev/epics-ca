//! Channel Access communicates with PVs via [requests](`request::Request`).
//! PV receive [read request](`request::ReadRequest`) and respond with [write request](`request::WriteRequest`).
//!
//! There are [typed requests](`request::TypedRequest`) that contain typed items.
//! Such requsts are [DST](https://doc.rust-lang.org/reference/dynamically-sized-types.html) for array PVs as they can contain arbitrary number of items.
//!

mod base;
mod typed;

pub use base::*;
pub use typed::*;

#[cfg(test)]
mod tests;
