use super::AnyChannel;
use crate::{
    error::{result_from_raw, Error},
    traits::Scalar,
    types::DbField,
};
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

pub struct Channel<T: ?Sized> {
    any: AnyChannel,
    dbf: DbField,
    count: usize,
    _p: PhantomData<T>,
}

impl<T: ?Sized> Channel<T> {
    pub(crate) fn from_any_unchecked(any: AnyChannel, dbf: DbField, count: usize) -> Self {
        Self {
            any,
            dbf,
            count,
            _p: PhantomData,
        }
    }
    pub fn into_any(self) -> AnyChannel {
        self.any
    }
}

impl<T: ?Sized> Deref for Channel<T> {
    type Target = AnyChannel;
    fn deref(&self) -> &Self::Target {
        &self.any
    }
}
impl<T: ?Sized> DerefMut for Channel<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.any
    }
}

impl<T: Scalar> Channel<T> {
    pub async fn get(&mut self) -> Result<T, Error> {
        let mut value = MaybeUninit::<T>::uninit();
        result_from_raw(unsafe {
            sys::ca_get_callback(
                self.dbf as _,
                self.raw(),
                Some(Self::callback),
                value.as_mut_ptr() as *mut _,
            )
        })
        .map(|()| unsafe { value.assume_init() })
    }
    pub async fn put(&mut self, value: T) -> Result<(), Error> {
        unimplemented!()
    }

    unsafe extern "C" fn callback(args: sys::event_handler_args) {}
}
impl<T: Scalar> Channel<[T]> {
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
    use crate::{AnyChannel, Context};
    use async_std::test as async_test;
    use c_str_macro::c_str;
    use serial_test::serial;
    use std::sync::Arc;

    #[async_test]
    #[serial]
    async fn downcast() {
        let ctx = Arc::new(Context::new().unwrap());
        let mut any = AnyChannel::new(ctx, c_str!("ca:test:ai")).unwrap();
        any.connected().await;
        any.into_typed::<f64>().unwrap();
    }
}
