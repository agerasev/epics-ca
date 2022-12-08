use crate::types::{DbRequest, EpicsString};

/// # Safety
///
/// `Self` and `Self::Raw` must be safely transmutable to each other.
pub trait Request {
    type Raw;
    const ENUM: DbRequest;
}

pub trait WriteRequest: Request {}
pub trait ReadRequest: Request {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutAckt(pub u16);

impl Request for PutAckt {
    type Raw = sys::dbr_put_ackt_t;
    const ENUM: DbRequest = DbRequest::PutAckt;
}
impl WriteRequest for PutAckt {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutAcks(pub u16);

impl Request for PutAcks {
    type Raw = sys::dbr_put_acks_t;
    const ENUM: DbRequest = DbRequest::PutAcks;
}
impl WriteRequest for PutAcks {}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct StsackString(pub EpicsString);

impl Request for StsackString {
    type Raw = sys::dbr_stsack_string_t;
    const ENUM: DbRequest = DbRequest::PutAcks;
}
impl ReadRequest for StsackString {}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct ClassName(pub EpicsString);

impl Request for ClassName {
    type Raw = sys::dbr_class_name_t;
    const ENUM: DbRequest = DbRequest::ClassName;
}
impl ReadRequest for ClassName {}
