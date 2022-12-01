use super::{AnyChannel, UserData};
use crate::{
    error::{self, result_from_raw, Error},
    types::{DbField, Type},
};
use futures::future::FusedFuture;
use std::{
    future::Future,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
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
    dbf: DbField,
    count: usize,
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

impl<T: Type + ?Sized> Channel<T> {
    pub fn get(&self, data: &mut T) -> Result<Get<'_, T>, Error> {
        result_from_raw(unsafe {
            sys::ca_array_get_callback(
                self.dbf as _,
                0,
                self.raw(),
                Some(Self::get_callback),
                self.user_data().id() as _,
            )
        })
        .and_then(|()| self.context().flush_io());
        unimplemented!()
    }
    pub fn put(&self, data: &T) -> Result<Put<'_, T>, Error> {
        unimplemented!()
    }
}

impl<T: Type + Default> Channel<T> {
    pub async fn get_copy(&mut self) -> Result<T, Error> {
        let mut value = T::default();
        assert_eq!(self.get(&mut value)?.await?, 1);
        Ok(value)
    }
}
impl<T: Type + Default + Clone> Channel<[T]> {
    pub async fn get_vec(&mut self) -> Result<Vec<T>, Error> {
        let mut data = vec![T::default(); self.count];
        let len = self.get(&mut data)?.await?;
        data.truncate(len);
        Ok(data)
    }
}

pub struct Get<'a, T: Type + ?Sized> {
    owner: &'a mut Channel<T>,
}
impl<'a, T: Type + ?Sized> Unpin for Get<'a, T> {}
impl<'a, T: Type + ?Sized> Future for Get<'a, T> {
    type Output = Result<usize, Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unimplemented!()
    }
}
impl<'a, T: Type + ?Sized> FusedFuture for Get<'a, T> {
    fn is_terminated(&self) -> bool {
        unimplemented!()
    }
}
impl<'a, T: Type + ?Sized> Drop for Get<'a, T> {
    fn drop(&mut self) {
        self.owner.user_data().change_id();
    }
}

pub struct Put<'a, T: Type + ?Sized> {
    owner: &'a mut Channel<T>,
}
impl<'a, T: Type + ?Sized> Unpin for Put<'a, T> {}
impl<'a, T: Type + ?Sized> Future for Put<'a, T> {
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unimplemented!()
    }
}
impl<'a, T: Type + ?Sized> FusedFuture for Put<'a, T> {
    fn is_terminated(&self) -> bool {
        unimplemented!()
    }
}
impl<'a, T: Type + ?Sized> Drop for Put<'a, T> {
    fn drop(&mut self) {
        self.owner.user_data().change_id();
    }
}

impl<T: Type + ?Sized> Channel<T> {
    unsafe extern "C" fn get_callback(args: sys::event_handler_args) {
        println!("get_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        if user_data.id() != args.usr as usize {
            return;
        }
    }
    unsafe extern "C" fn put_callback(args: sys::event_handler_args) {
        println!("put_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        if user_data.id() != args.usr as usize {
            return;
        }
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
