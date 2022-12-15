use crate::{
    types::{EpicsString, RequestId},
    Error,
};

/// # Safety
///
/// Should be implemented only for requests supported by channel access.
///
/// `Self` and `Self::Raw` must be safely transmutable to each other.
#[allow(clippy::len_without_is_empty)]
pub unsafe trait Request: Send + 'static {
    type Raw: Copy + Send + Sized + 'static;
    const ENUM: RequestId;

    fn len(&self) -> usize;
    /// # Safety
    ///
    /// Pointer must be valid and point to raw request structure.
    unsafe fn from_ptr<'a>(ptr: *const u8, count: usize) -> Result<&'a Self, Error>;
}

macro_rules! impl_request_methods {
    () => {
        fn len(&self) -> usize {
            1
        }
        unsafe fn from_ptr<'a>(ptr: *const u8, count: usize) -> Result<&'a Self, crate::Error> {
            if count == 1 {
                Ok(&*(ptr as *const Self))
            } else {
                Err(crate::error::BADCOUNT)
            }
        }
    };
}

pub trait WriteRequest: Request {}
pub trait ReadRequest: Request {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutAckt(pub u16);

unsafe impl Request for PutAckt {
    type Raw = sys::dbr_put_ackt_t;
    const ENUM: RequestId = RequestId::PutAckt;
    impl_request_methods!();
}
impl WriteRequest for PutAckt {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutAcks(pub u16);

unsafe impl Request for PutAcks {
    type Raw = sys::dbr_put_acks_t;
    const ENUM: RequestId = RequestId::PutAcks;
    impl_request_methods!();
}
impl WriteRequest for PutAcks {}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct StsackString(pub EpicsString);

unsafe impl Request for StsackString {
    type Raw = sys::dbr_stsack_string_t;
    const ENUM: RequestId = RequestId::PutAcks;
    impl_request_methods!();
}
impl ReadRequest for StsackString {}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct ClassName(pub EpicsString);

unsafe impl Request for ClassName {
    type Raw = sys::dbr_class_name_t;
    const ENUM: RequestId = RequestId::ClassName;
    impl_request_methods!();
}
impl ReadRequest for ClassName {}
