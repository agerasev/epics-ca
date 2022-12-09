use crate::types::{DbRequest, Scalar};

use super::{ReadRequest, Request, ScalarRequest, TypedRequest, WriteRequest};
use derive_more::{Deref, DerefMut};
use std::{ptr, slice};

pub trait ArrayRequest: TypedRequest {
    fn values(&self) -> &[Self::Type];
    fn values_mut(&mut self) -> &mut [Self::Type];
}

#[repr(C)]
#[derive(Debug, Deref, DerefMut)]
pub struct Extended<R: ScalarRequest> {
    #[deref]
    #[deref_mut]
    base: R,
    extent: [R::Type],
}
unsafe impl<R: ScalarRequest> Request for Extended<R> {
    type Raw = R::Raw;
    const ENUM: DbRequest = R::ENUM;

    fn len(&self) -> usize {
        self.extent.len() + 1
    }
    unsafe fn ref_from_ptr<'a>(ptr: *const u8, count: usize) -> &'a Self {
        &*(ptr::slice_from_raw_parts(ptr, count - 1) as *const Self)
    }
}
impl<R: TypedRequest + ScalarRequest> TypedRequest for Extended<R> {
    type Type = R::Type;
}
impl<R: TypedRequest + ScalarRequest> ArrayRequest for Extended<R> {
    fn values(&self) -> &[R::Type] {
        unsafe { slice::from_raw_parts(self.base.value() as *const R::Type, self.len()) }
    }
    fn values_mut(&mut self) -> &mut [R::Type] {
        unsafe { slice::from_raw_parts_mut(self.base.value_mut() as *mut R::Type, self.len()) }
    }
}
impl<R: ReadRequest + ScalarRequest> ReadRequest for Extended<R> {}
impl<R: WriteRequest + ScalarRequest> WriteRequest for Extended<R> {}

unsafe impl<T: Scalar> Request for [T] {
    type Raw = T::Raw;
    const ENUM: DbRequest = DbRequest::Base(T::ENUM);

    fn len(&self) -> usize {
        self.len()
    }
    unsafe fn ref_from_ptr<'a>(ptr: *const u8, count: usize) -> &'a Self {
        &*(ptr::slice_from_raw_parts(ptr, count) as *const Self)
    }
}
impl<T: Scalar> TypedRequest for [T] {
    type Type = T;
}
impl<T: Scalar> ReadRequest for [T] {}
impl<T: Scalar> WriteRequest for [T] {}
impl<T: Scalar> ArrayRequest for [T] {
    fn values(&self) -> &[T] {
        self
    }
    fn values_mut(&mut self) -> &mut [T] {
        self
    }
}
