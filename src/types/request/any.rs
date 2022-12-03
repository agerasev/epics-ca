use crate::types::{DbRequest, EpicsString};

pub trait AnyRequest {
    type Raw;
    const ENUM: DbRequest;
}

/// # Safety
///
/// `Self` and `Self::Raw` must be safely transmutable to each other.
pub unsafe trait WriteRequest: AnyRequest {}

pub trait ReadRequest: AnyRequest {
    /// # Safety
    ///
    /// See [`copy_data`](`crate::types::Scalar::copy_data`).
    unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize);
}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutAckt(pub u16);

impl AnyRequest for PutAckt {
    type Raw = sys::dbr_put_ackt_t;
    const ENUM: DbRequest = DbRequest::PutAckt;
}
unsafe impl WriteRequest for PutAckt {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutAcks(pub u16);

impl AnyRequest for PutAcks {
    type Raw = sys::dbr_put_acks_t;
    const ENUM: DbRequest = DbRequest::PutAcks;
}
unsafe impl WriteRequest for PutAcks {}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct StsackString(pub EpicsString);

impl AnyRequest for StsackString {
    type Raw = sys::dbr_stsack_string_t;
    const ENUM: DbRequest = DbRequest::PutAcks;
}
impl ReadRequest for StsackString {
    unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, _: usize) {
        *this = StsackString(EpicsString::from_array(*raw).unwrap());
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct ClassName(pub EpicsString);

impl AnyRequest for ClassName {
    type Raw = sys::dbr_class_name_t;
    const ENUM: DbRequest = DbRequest::ClassName;
}
impl ReadRequest for ClassName {
    unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, _: usize) {
        *this = ClassName(EpicsString::from_array(*raw).unwrap());
    }
}
