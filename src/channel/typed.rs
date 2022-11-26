use super::AnyChannel;
use crate::{error::Error, types::DbField};
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

#[repr(transparent)]
pub struct Channel<T: ?Sized> {
    base: AnyChannel,
    _p: PhantomData<T>,
}

impl<T> Channel<T> {
    pub(crate) fn from_any_unchecked(base: AnyChannel) -> Self {
        Self {
            base,
            _p: PhantomData,
        }
    }
    pub(crate) fn from_any_ref_unchecked(base: &AnyChannel) -> &Self {
        unsafe { &*(base as *const _ as *const Self) }
    }
    pub(crate) fn from_any_mut_unchecked(base: &mut AnyChannel) -> &mut Self {
        unsafe { &mut *(base as *mut _ as *mut Self) }
    }

    pub fn into_any(self) -> AnyChannel {
        self.base
    }
}

impl<T> Deref for Channel<T> {
    type Target = AnyChannel;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T> DerefMut for Channel<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub unsafe trait Type {
    fn matches(dbf: DbField, count: usize) -> bool;
}

unsafe impl Type for i8 {
    fn matches(dbf: DbField, count: usize) -> bool {
        matches!(dbf, DbField::Char) && matches!(count, 1)
    }
}
unsafe impl Type for i16 {
    fn matches(dbf: DbField, count: usize) -> bool {
        matches!(dbf, DbField::Short | DbField::Enum) && matches!(count, 1)
    }
}
unsafe impl Type for i32 {
    fn matches(dbf: DbField, count: usize) -> bool {
        matches!(dbf, DbField::Long) && matches!(count, 1)
    }
}
unsafe impl Type for f32 {
    fn matches(dbf: DbField, count: usize) -> bool {
        matches!(dbf, DbField::Float) && matches!(count, 1)
    }
}
unsafe impl Type for f64 {
    fn matches(dbf: DbField, count: usize) -> bool {
        matches!(dbf, DbField::Double) && matches!(count, 1)
    }
}
unsafe impl Type for [i8] {
    fn matches(dbf: DbField, _: usize) -> bool {
        matches!(dbf, DbField::Char)
    }
}
unsafe impl Type for [i16] {
    fn matches(dbf: DbField, _: usize) -> bool {
        matches!(dbf, DbField::Short | DbField::Enum)
    }
}
unsafe impl Type for [i32] {
    fn matches(dbf: DbField, _: usize) -> bool {
        matches!(dbf, DbField::Long)
    }
}
unsafe impl Type for [f32] {
    fn matches(dbf: DbField, _: usize) -> bool {
        matches!(dbf, DbField::Float)
    }
}
unsafe impl Type for [f64] {
    fn matches(dbf: DbField, _: usize) -> bool {
        matches!(dbf, DbField::Double)
    }
}
unsafe impl Type for CStr {
    fn matches(dbf: DbField, count: usize) -> bool {
        matches!(dbf, DbField::String) && matches!(count, 1)
    }
}

impl<T: Copy> Channel<T> {
    pub async fn get(&mut self) -> Result<T, Error> {
        unimplemented!()
    }
    pub async fn put(&mut self, value: T) -> Result<(), Error> {
        unimplemented!()
    }
}
impl<T: Copy> Channel<[T]> {
    pub async fn get_in_place(&mut self, buffer: &mut [T]) -> Result<usize, Error> {
        unimplemented!()
    }
    pub async fn get_vec(&mut self) -> Result<Vec<T>, Error> {
        unimplemented!()
    }
    pub async fn put(&mut self, data: &[T]) -> Result<usize, Error> {
        unimplemented!()
    }
}
impl Channel<CStr> {
    pub async fn get_in_place(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        unimplemented!()
    }
    pub async fn get_string(&mut self) -> Result<CString, Error> {
        unimplemented!()
    }
    pub async fn put(&mut self, cstr: &CStr) -> Result<(), Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{AnyChannel, Channel, Context};
    use async_std::test as async_test;
    use c_str_macro::c_str;
    use serial_test::serial;
    use std::sync::Arc;

    #[async_test]
    #[serial]
    async fn downcast() {
        let ctx = Arc::new(Context::new().unwrap());
        let any = AnyChannel::connect(ctx, c_str!("ca:test:ai"))
            .await
            .unwrap();
        let _: Channel<f64> = any.into_typed().unwrap();
    }
}
