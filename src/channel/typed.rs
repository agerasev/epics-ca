use super::AnyChannel;
use crate::{
    error::{self, Error},
    types::{DbField, Type},
};
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr,
};

impl AnyChannel {
    fn match_type<T: Type + ?Sized>(&self) -> Result<(DbField, usize), Error> {
        let dbf = self.field_type()?;
        let count = self.element_count()?;
        if !T::match_field(dbf) {
            Err(error::BADTYPE)
        } else if !T::match_count(count) {
            Err(error::BADCOUNT)
        } else {
            Ok((dbf, count))
        }
    }
    pub fn into_typed<T: Type + ?Sized>(self) -> Result<Channel<T>, (Error, Self)> {
        match self.match_type::<T>() {
            Ok((dbf, count)) => Ok(Channel::from_any_unchecked(self, dbf, count)),
            Err(err) => Err((err, self)),
        }
    }
}

/// Typed channel.
pub struct Channel<T: ?Sized> {
    any: AnyChannel,
    pub(crate) dbf: DbField,
    pub(crate) count: usize,
    _p: PhantomData<T>,
}

impl<T: ?Sized> Channel<T> {
    /// Convert [`AnyChannel`] to [`Channel<T>`] without type checking.
    ///
    /// It is safe because the type of remote channel can change at any moment and checks are done reading/writing/monitoring anyway.
    ///
    /// If you want to check type before converting use [`AnyChannel::into_typed`].
    pub fn from_any_unchecked(any: AnyChannel, dbf: DbField, count: usize) -> Self {
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

pub(crate) struct ProcessData {
    id_counter: usize,
    pub(crate) result: Option<Result<(), Error>>,
    pub(crate) state: *mut u8,
}

impl ProcessData {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            result: None,
            state: ptr::null_mut(),
        }
    }
    pub fn id(&self) -> usize {
        self.id_counter
    }
    pub fn change_id(&mut self) {
        self.id_counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{AnyChannel, Context};
    use async_std::test as async_test;
    use c_str_macro::c_str;
    use serial_test::serial;
    use std::f64::consts::PI;

    #[async_test]
    #[serial]
    async fn downcast() {
        let ctx = Context::new().unwrap();
        let mut any = AnyChannel::new(ctx, c_str!("ca:test:ai")).unwrap();
        any.connected().await;
        any.into_typed::<f64>().unwrap();
    }

    #[async_test]
    #[serial]
    async fn put_get() {
        let ctx = Context::new().unwrap();

        let mut output = AnyChannel::new(ctx.clone(), c_str!("ca:test:ao")).unwrap();
        output.connected().await;
        let mut output = output.into_typed::<f64>().unwrap();
        output.put(&PI).unwrap().await.unwrap();

        let mut input = AnyChannel::new(ctx, c_str!("ca:test:ai")).unwrap();
        input.connected().await;
        let mut input = input.into_typed::<f64>().unwrap();
        assert_eq!(input.get().await.unwrap(), PI);
    }
}
