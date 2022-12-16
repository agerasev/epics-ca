use crate::{
    error::{self, Error},
    types::{EpicsString, RequestId},
};

/// Abstract request to channel.
///
/// # Safety
///
/// Should be implemented only for requests supported by channel access.
///
/// `Self` and `Self::Raw` must be safely transmutable to each other.
#[allow(clippy::len_without_is_empty)]
pub unsafe trait Request: Send + 'static {
    /// Raw request structure.
    type Raw: Copy + Send + Sized + 'static;
    /// Request identifier.
    const ID: RequestId;

    /// Length of the value in the request.
    fn len(&self) -> usize;
    /// Create reference (possibly wide) to the request from raw pointer and count of elements.
    ///
    /// # Safety
    ///
    /// Pointer must be valid and point to raw request structure.
    unsafe fn from_ptr<'a>(ptr: *const u8, dbr: RequestId, count: usize)
        -> Result<&'a Self, Error>;
    /// Clone request and put it in newly allocated box.
    fn clone_boxed(&self) -> Box<Self>;
}

macro_rules! impl_request_methods {
    () => {
        fn len(&self) -> usize {
            1
        }
        unsafe fn from_ptr<'a>(
            ptr: *const u8,
            dbr: RequestId,
            count: usize,
        ) -> Result<&'a Self, Error> {
            if dbr != Self::ID {
                Err(error::BADTYPE)
            } else if count != 1 {
                Err(error::BADCOUNT)
            } else {
                Ok(&*(ptr as *const Self))
            }
        }
        fn clone_boxed(&self) -> Box<Self> {
            Box::new(*self)
        }
    };
}

/// Request that writes some data to channel.
pub trait WriteRequest: Request {}
/// Request that reads some data from channel.
pub trait ReadRequest: Request {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutAckt(pub u16);

unsafe impl Request for PutAckt {
    type Raw = sys::dbr_put_ackt_t;
    const ID: RequestId = RequestId::PutAckt;
    impl_request_methods!();
}
impl WriteRequest for PutAckt {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutAcks(pub u16);

unsafe impl Request for PutAcks {
    type Raw = sys::dbr_put_acks_t;
    const ID: RequestId = RequestId::PutAcks;
    impl_request_methods!();
}
impl WriteRequest for PutAcks {}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClassName(pub EpicsString);

unsafe impl Request for ClassName {
    type Raw = sys::dbr_class_name_t;
    const ID: RequestId = RequestId::ClassName;
    impl_request_methods!();
}
impl ReadRequest for ClassName {}
