use super::{impl_scalar_request_methods, Meta, ReadRequest, Request, WriteRequest};
use crate::{
    error::Error,
    types::{Field, RequestId},
};
use std::{
    ops::{Deref, DerefMut},
    ptr, slice,
};

pub trait TypedRequest: Request {
    type Field: Field;
    type Meta: Meta<Self::Field>;

    fn values(&self) -> &[Self::Field];
    fn values_mut(&mut self) -> &mut [Self::Field];

    fn meta(&self) -> &Self::Meta;
    fn meta_mut(&mut self) -> &mut Self::Meta;
}
pub trait ScalarRequest: TypedRequest + Sized + Copy {
    fn value(&self) -> &Self::Field;
    fn value_mut(&mut self) -> &mut Self::Field;
}

// Scalar

#[repr(C)]
#[derive(Clone, Copy)]
pub union Scalar<T: Field, M: Meta<T>> {
    this: M,
    raw: M::Raw,
}

impl<T: Field, M: Meta<T>> Deref for Scalar<T, M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.this }
    }
}
impl<T: Field, M: Meta<T>> DerefMut for Scalar<T, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.this }
    }
}

unsafe impl<T: Field, M: Meta<T>> Request for Scalar<T, M> {
    type Raw = M::Raw;
    const ENUM: RequestId = M::ENUM;
    impl_scalar_request_methods!();
}
impl<T: Field, M: Meta<T>> TypedRequest for Scalar<T, M> {
    type Field = T;
    type Meta = M;

    fn values(&self) -> &[Self::Field] {
        unsafe { &*(self.value() as *const _ as *const [Self::Field; 1]) }
    }
    fn values_mut(&mut self) -> &mut [Self::Field] {
        unsafe { &mut *(self.value_mut() as *mut _ as *mut [Self::Field; 1]) }
    }

    fn meta(&self) -> &Self::Meta {
        self
    }
    fn meta_mut(&mut self) -> &mut Self::Meta {
        self
    }
}
impl<T: Field, M: Meta<T>> ScalarRequest for Scalar<T, M> {
    fn value(&self) -> &T {
        unsafe { &*(((self as *const Self).offset(1) as *const T).offset(-1)) }
    }
    fn value_mut(&mut self) -> &mut T {
        unsafe { &mut *(((self as *mut Self).offset(1) as *mut T).offset(-1)) }
    }
}
impl<T: Field, M: Meta<T>> ReadRequest for Scalar<T, M> {}

unsafe impl<T: Field> Request for T {
    type Raw = T::Raw;
    const ENUM: RequestId = RequestId::Base(T::ENUM);
    impl_scalar_request_methods!();
}
impl<T: Field> TypedRequest for T {
    type Field = T;
    type Meta = ();

    fn values(&self) -> &[Self::Field] {
        unsafe { &*(self as *const _ as *const [Self::Field; 1]) }
    }
    fn values_mut(&mut self) -> &mut [Self::Field] {
        unsafe { &mut *(self as *mut _ as *mut [Self::Field; 1]) }
    }

    fn meta(&self) -> &Self::Meta {
        unsafe { &*(self as *const _ as *const ()) }
    }
    fn meta_mut(&mut self) -> &mut Self::Meta {
        unsafe { &mut *(self as *mut _ as *mut ()) }
    }
}
impl<T: Field> ScalarRequest for T {
    fn value(&self) -> &T {
        self
    }
    fn value_mut(&mut self) -> &mut T {
        self
    }
}
impl<T: Field> ReadRequest for T {}
impl<T: Field> WriteRequest for T {}

#[repr(C)]
pub struct Array<T: Field, M: Meta<T>> {
    scalar: Scalar<T, M>,
    extent: [T],
}

impl<T: Field, M: Meta<T>> Deref for Array<T, M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.scalar
    }
}
impl<T: Field, M: Meta<T>> DerefMut for Array<T, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scalar
    }
}

unsafe impl<T: Field, M: Meta<T>> Request for Array<T, M> {
    type Raw = M::Raw;
    const ENUM: RequestId = M::ENUM;

    fn len(&self) -> usize {
        self.extent.len()
    }
    unsafe fn from_ptr<'a>(ptr: *const u8, count: usize) -> Result<&'a Self, Error> {
        Ok(&*(ptr::slice_from_raw_parts(ptr, count) as *const Self))
    }
}
impl<T: Field, M: Meta<T>> TypedRequest for Array<T, M> {
    type Field = T;
    type Meta = M;

    fn values(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.scalar.value() as *const T, self.len()) }
    }
    fn values_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.scalar.value_mut() as *mut T, self.len()) }
    }

    fn meta(&self) -> &Self::Meta {
        self
    }
    fn meta_mut(&mut self) -> &mut Self::Meta {
        self
    }
}
impl<T: Field, M: Meta<T>> ReadRequest for Array<T, M> {}

unsafe impl<T: Field> Request for [T] {
    type Raw = T::Raw;
    const ENUM: RequestId = RequestId::Base(T::ENUM);

    fn len(&self) -> usize {
        self.len()
    }
    unsafe fn from_ptr<'a>(ptr: *const u8, count: usize) -> Result<&'a Self, Error> {
        Ok(&*(ptr::slice_from_raw_parts(ptr, count) as *const Self))
    }
}
impl<T: Field> TypedRequest for [T] {
    type Field = T;
    type Meta = ();

    fn values(&self) -> &[T] {
        self
    }
    fn values_mut(&mut self) -> &mut [T] {
        self
    }

    fn meta(&self) -> &Self::Meta {
        unsafe { &*(self as *const _ as *const ()) }
    }
    fn meta_mut(&mut self) -> &mut Self::Meta {
        unsafe { &mut *(self as *mut _ as *mut ()) }
    }
}
impl<T: Field> ReadRequest for [T] {}
impl<T: Field> WriteRequest for [T] {}
