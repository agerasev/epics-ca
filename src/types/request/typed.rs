use super::{Meta, ReadRequest, Request, WriteRequest};
use crate::{
    error::{self, Error},
    types::{DbRequest, Scalar},
};
use std::{
    ops::{Deref, DerefMut},
    ptr, slice,
};

pub trait TypedRequest: Request {
    type Field: Scalar;
    type Scalar: Copy + Send + Sized + 'static;

    fn values(&self) -> &[Self::Field];
    fn values_mut(&mut self) -> &mut [Self::Field];

    /// # Safety
    ///
    /// Request length must be equal to 1.
    unsafe fn to_scalar_unchecked(&self) -> Self::Scalar;

    fn to_scalar(&self) -> Result<Self::Scalar, Error> {
        if self.len() == 1 {
            Ok(unsafe { self.to_scalar_unchecked() })
        } else {
            Err(error::BADCOUNT)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union MetaValue<T: Scalar, M: Meta<T>> {
    this: M,
    raw: M::Raw,
}

impl<T: Scalar, M: Meta<T>> Deref for MetaValue<T, M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.this }
    }
}
impl<T: Scalar, M: Meta<T>> DerefMut for MetaValue<T, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.this }
    }
}

impl<T: Scalar, M: Meta<T>> MetaValue<T, M> {
    pub fn value(&self) -> &T {
        unsafe { &*(((self as *const Self).offset(1) as *const T).offset(-1)) }
    }
    pub fn value_mut(&mut self) -> &mut T {
        unsafe { &mut *(((self as *mut Self).offset(1) as *mut T).offset(-1)) }
    }
}

#[repr(C)]
pub struct Extended<T: Scalar, M: Meta<T>> {
    scalar: MetaValue<T, M>,
    extent: [T],
}

impl<T: Scalar, M: Meta<T>> Deref for Extended<T, M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.scalar
    }
}
impl<T: Scalar, M: Meta<T>> DerefMut for Extended<T, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scalar
    }
}

unsafe impl<T: Scalar, M: Meta<T>> Request for Extended<T, M> {
    type Raw = M::Raw;
    const ENUM: DbRequest = M::ENUM;

    fn len(&self) -> usize {
        self.extent.len() + 1
    }
    unsafe fn ref_from_ptr<'a>(ptr: *const u8, count: usize) -> &'a Self {
        &*(ptr::slice_from_raw_parts(ptr, count - 1) as *const Self)
    }
}
impl<T: Scalar, M: Meta<T>> TypedRequest for Extended<T, M> {
    type Field = T;
    type Scalar = MetaValue<T, M>;

    fn values(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.scalar.value() as *const T, self.len()) }
    }
    fn values_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.scalar.value_mut() as *mut T, self.len()) }
    }

    unsafe fn to_scalar_unchecked(&self) -> Self::Scalar {
        self.scalar
    }
}
impl<T: Scalar, M: Meta<T>> ReadRequest for Extended<T, M> {}

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
    type Field = T;
    type Scalar = T;

    fn values(&self) -> &[T] {
        self
    }
    fn values_mut(&mut self) -> &mut [T] {
        self
    }

    unsafe fn to_scalar_unchecked(&self) -> Self::Scalar {
        *self.get_unchecked(0)
    }
}
impl<T: Scalar> ReadRequest for [T] {}
impl<T: Scalar> WriteRequest for [T] {}
