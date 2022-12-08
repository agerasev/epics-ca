use crate::types::{DbRequest, Scalar};

use super::{ReadRequest, Request, ScalarRequest, TypedRequest, WriteRequest};
use derive_more::{Deref, DerefMut};
use std::slice;

#[repr(C)]
#[derive(Debug, Deref, DerefMut)]
pub struct Extended<R: ScalarRequest> {
    #[deref]
    #[deref_mut]
    base: R,
    extent: [R::Type],
}

impl<R: ScalarRequest> Extended<R> {
    pub fn value(&self) -> &[R::Type] {
        unsafe { slice::from_raw_parts(self.base.value() as *const R::Type, self.extent.len() + 1) }
    }
    pub fn value_mut(&mut self) -> &mut [R::Type] {
        unsafe {
            slice::from_raw_parts_mut(self.base.value_mut() as *mut R::Type, self.extent.len() + 1)
        }
    }
}

impl<R: ScalarRequest> Request for Extended<R> {
    type Raw = R::Raw;
    const ENUM: DbRequest = R::ENUM;
}
impl<R: TypedRequest + ScalarRequest> TypedRequest for Extended<R> {
    type Type = R::Type;
}
impl<R: ReadRequest + ScalarRequest> ReadRequest for Extended<R> {}
impl<R: WriteRequest + ScalarRequest> WriteRequest for Extended<R> {}

impl<T: Scalar> Request for [T] {
    type Raw = T::Raw;
    const ENUM: DbRequest = DbRequest::Base(T::ENUM);
}
impl<T: Scalar> TypedRequest for [T] {
    type Type = T;
}
impl<T: Scalar> ReadRequest for [T] {}
impl<T: Scalar> WriteRequest for [T] {}
