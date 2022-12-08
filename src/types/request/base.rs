use crate::types::{DbRequest, EpicsString};

/// # Safety
///
/// Should be implemented only for requests supported by channel access.
///
/// `Self` and `Self::Raw` must be safely transmutable to each other.
#[allow(clippy::len_without_is_empty)]
pub unsafe trait Request {
    type Raw;
    const ENUM: DbRequest;

    fn len(&self) -> usize;
    /// # Safety
    ///
    /// Pointer must be valid and point to raw request structure.
    unsafe fn ref_from_ptr<'a>(ptr: *const u8, count: usize) -> &'a Self;
}

macro_rules! impl_sized_request_methods {
    () => {
        fn len(&self) -> usize {
            1
        }
        unsafe fn ref_from_ptr<'a>(ptr: *const u8, count: usize) -> &'a Self {
            debug_assert_eq!(count, 1);
            &*(ptr as *const Self)
        }
    };
}
pub(crate) use impl_sized_request_methods;

pub trait WriteRequest: Request {}
pub trait ReadRequest: Request {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutAckt(pub u16);

unsafe impl Request for PutAckt {
    type Raw = sys::dbr_put_ackt_t;
    const ENUM: DbRequest = DbRequest::PutAckt;
    impl_sized_request_methods!();
}
impl WriteRequest for PutAckt {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutAcks(pub u16);

unsafe impl Request for PutAcks {
    type Raw = sys::dbr_put_acks_t;
    const ENUM: DbRequest = DbRequest::PutAcks;
    impl_sized_request_methods!();
}
impl WriteRequest for PutAcks {}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct StsackString(pub EpicsString);

unsafe impl Request for StsackString {
    type Raw = sys::dbr_stsack_string_t;
    const ENUM: DbRequest = DbRequest::PutAcks;
    impl_sized_request_methods!();
}
impl ReadRequest for StsackString {}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct ClassName(pub EpicsString);

unsafe impl Request for ClassName {
    type Raw = sys::dbr_class_name_t;
    const ENUM: DbRequest = DbRequest::ClassName;
    impl_sized_request_methods!();
}
impl ReadRequest for ClassName {}
