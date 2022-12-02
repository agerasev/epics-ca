use super::{AnyChannel, UserData};
use crate::{
    error::{self, result_from_raw, Error},
    types::{DbField, Scalar, Type},
};
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
    pub fn get(&mut self, data: &mut T) -> Result<Get<'_, T>, Error> {
        self.context()
            .with(|| {
                let mut proc = self.user_data().process.lock().unwrap();
                proc.data = data.as_mut_ptr() as _;
                if cfg!(debug_assertions) {
                    proc.count = data.element_count();
                }
                result_from_raw(unsafe {
                    sys::ca_array_get_callback(
                        self.dbf as _,
                        data.element_count() as _,
                        self.raw(),
                        Some(Self::get_callback),
                        proc.id() as _,
                    )
                })
                .map(|()| {
                    self.context().flush_io();
                    proc.status = None;
                })
            })
            .map(|()| Get { owner: self })
    }
    pub fn put(&mut self, data: &T) -> Result<Put<'_, T>, Error> {
        self.context()
            .with(|| {
                let mut proc = self.user_data().process.lock().unwrap();
                result_from_raw(unsafe {
                    sys::ca_array_put_callback(
                        self.dbf as _,
                        data.element_count() as _,
                        self.raw(),
                        data.as_ptr() as _,
                        Some(Self::put_callback),
                        proc.id() as _,
                    )
                })
                .map(|()| {
                    self.context().flush_io();
                    proc.status = None;
                })
            })
            .map(|()| Put { owner: self })
    }
}

impl<T: Type + Default> Channel<T> {
    pub async fn get_copy(&mut self) -> Result<T, Error> {
        let mut value = T::default();
        assert_eq!(self.get(&mut value)?.await?, 1);
        Ok(value)
    }
}
impl<T: Scalar + Default + Clone> Channel<[T]> {
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
        let user_data = self.owner.user_data();
        user_data.waker.register(cx.waker());
        let mut proc = user_data.process.lock().unwrap();
        match proc.status.take() {
            Some(status) => Poll::Ready(status.map(|()| proc.count)),
            None => Poll::Pending,
        }
    }
}
impl<'a, T: Type + ?Sized> Drop for Get<'a, T> {
    fn drop(&mut self) {
        self.owner.user_data().process.lock().unwrap().change_id();
    }
}

pub struct Put<'a, T: Type + ?Sized> {
    owner: &'a mut Channel<T>,
}
impl<'a, T: Type + ?Sized> Unpin for Put<'a, T> {}
impl<'a, T: Type + ?Sized> Future for Put<'a, T> {
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let user_data = self.owner.user_data();
        user_data.waker.register(cx.waker());
        let mut proc = user_data.process.lock().unwrap();
        match proc.status.take() {
            Some(status) => Poll::Ready(status),
            None => Poll::Pending,
        }
    }
}
impl<'a, T: Type + ?Sized> Drop for Put<'a, T> {
    fn drop(&mut self) {
        self.owner.user_data().process.lock().unwrap().change_id();
    }
}

impl<T: Type + ?Sized> Channel<T> {
    unsafe extern "C" fn get_callback(args: sys::event_handler_args) {
        println!("get_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        let mut proc = user_data.process.lock().unwrap();
        if proc.id() != args.usr as usize {
            return;
        }
        let status = result_from_raw(args.status);
        if status.is_ok() {
            let count = args.count as usize;
            debug_assert!(count <= proc.count);
            debug_assert!(T::match_field(
                DbField::try_from_raw(args.type_ as _).unwrap()
            ));
            T::Element::copy_data(args.dbr as _, proc.data as *mut T::Element, count);
            proc.count = count;
        }
        proc.status = Some(status);
        user_data.waker.wake();
    }
    unsafe extern "C" fn put_callback(args: sys::event_handler_args) {
        println!("put_callback: {:?}", args);
        let user_data = &*(sys::ca_puser(args.chid) as *const UserData);
        let mut proc = user_data.process.lock().unwrap();
        if proc.id() != args.usr as usize {
            return;
        }
        proc.status = Some(result_from_raw(args.status));
        user_data.waker.wake();
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
        assert_eq!(input.get_copy().await.unwrap(), PI);
    }
}
