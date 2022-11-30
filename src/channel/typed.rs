use super::AnyChannel;
use crate::{error::Error, types::DbField};
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub struct Channel<T: ?Sized> {
    base: AnyChannel,
    dbf: DbField,
    count: usize,
    _p: PhantomData<T>,
}

impl<T: ?Sized> Channel<T> {
    pub(crate) fn from_any_unchecked(base: AnyChannel, dbf: DbField, count: usize) -> Self {
        Self {
            base,
            dbf,
            count,
            _p: PhantomData,
        }
    }
    pub fn into_any(self) -> AnyChannel {
        self.base
    }
}

impl<T: ?Sized> Deref for Channel<T> {
    type Target = AnyChannel;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T: ?Sized> DerefMut for Channel<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Copy> Channel<T> {
    pub async fn get(&mut self) -> Result<T, Error> {
        //unsafe { sys::ca_get_callback(self.raw()) }
        unimplemented!()
    }
    pub async fn put(&mut self, value: T) -> Result<(), Error> {
        unimplemented!()
    }
}
impl<T: Copy> Channel<[T]> {
    pub async fn get_in_place(&mut self, buf: &mut [T]) -> Result<usize, Error> {
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
    pub async fn get_in_place(&mut self, buf: &mut [u8]) -> Result<(), Error> {
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
