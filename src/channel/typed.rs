use super::{
    get::Callback,
    subscribe::{LastFn, Queue, QueueFn},
    Channel, Get, GetFn, Put, Subscription,
};
use crate::{
    error::Error,
    request::{ReadRequest, Request, TypedRequest, WriteRequest},
    types::{Field, Value},
};
use derive_more::{Deref, DerefMut, Into};
use std::{
    any::type_name,
    fmt::{self, Debug},
    marker::PhantomData,
};

impl Channel {
    fn check_type<V: Value + ?Sized>(&self) -> Result<(), Error> {
        let dbf = self.field_type()?;
        let count = self.element_count()?;
        V::check_type(dbf, count)
    }

    pub fn into_typed<V: Value + ?Sized>(self) -> Result<TypedChannel<V>, (Error, Self)> {
        match self.check_type::<V>() {
            Ok(()) => Ok(TypedChannel::new_unchecked(self)),
            Err(err) => Err((err, self)),
        }
    }
}

/// Typed channel.
#[repr(transparent)]
#[derive(Deref, DerefMut, Into)]
pub struct TypedChannel<V: Value + ?Sized> {
    #[deref]
    #[deref_mut]
    pub(crate) base: Channel,
    #[into(ignore)]
    _p: PhantomData<V>,
}

impl<V: Value + ?Sized> TypedChannel<V> {
    /// Convert [`Channel`] to [`TypedChannel<V>`] without type checking.
    ///
    /// It is safe because the type of remote channel can change at any moment and checks are done during reading/writing/monitoring anyway.
    ///
    /// If you want to check type before converting use [`Channel::into_typed`].
    pub fn new_unchecked(base: Channel) -> Self {
        Self {
            base,
            _p: PhantomData,
        }
    }
}

impl<V: Value + ?Sized> Debug for TypedChannel<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TypedChannel<{}>({:?})", type_name::<V>(), self.raw())
    }
}

impl<V: Value + ?Sized> TypedChannel<V> {
    pub fn put_ref<R>(&mut self, req: &R) -> Result<Put<'_>, Error>
    where
        R: TypedRequest<Value = V> + WriteRequest + ?Sized,
    {
        self.base.put::<R>(req)
    }

    pub fn get_with<R, F>(&mut self, func: F) -> Get<'_, F>
    where
        R: TypedRequest<Value = V> + ReadRequest + ?Sized,
        F: Callback<Request = R>,
    {
        self.base.get_with(func)
    }

    pub fn subscribe_with<F: Queue>(&mut self, func: F) -> Subscription<'_, F>
    where
        F::Request: TypedRequest<Value = V> + ReadRequest,
    {
        Subscription::new(self, func)
    }
}

impl<T: Field> TypedChannel<[T]> {
    pub fn get_boxed<R>(&mut self) -> Get<'_, GetFn<R, Box<R>>>
    where
        R: TypedRequest<Value = [T]> + ReadRequest + ?Sized,
    {
        self.get_with(GetFn::<R, Box<R>>::new(clone_boxed::<R>))
    }

    pub fn subscribe_boxed<R>(&mut self) -> Subscription<'_, LastFn<R, Box<R>>>
    where
        R: TypedRequest<Value = [T]> + ReadRequest + ?Sized,
    {
        self.subscribe_with(LastFn::<R, Box<R>>::new(clone_boxed_some::<R>))
    }
}

impl<T: Field> TypedChannel<T> {
    pub fn put<R>(&mut self, req: R) -> Result<Put<'_>, Error>
    where
        R: TypedRequest<Value = T> + WriteRequest,
    {
        self.put_ref::<R>(&req)
    }

    pub fn get<R>(&mut self) -> Get<'_, GetFn<R, R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.get_with(GetFn::<R, R>::new(copy::<R>))
    }

    pub fn subscribe<R>(&mut self) -> Subscription<'_, LastFn<R, R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.subscribe_with(LastFn::<R, R>::new(copy_some::<R>))
    }

    pub fn subscribe_buffered<R>(&mut self) -> Subscription<'_, QueueFn<R, R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.subscribe_with(QueueFn::<R, R>::new(copy_some::<R>))
    }
}

fn clone_boxed<R: Request + ?Sized>(input: Result<&R, Error>) -> Result<Box<R>, Error> {
    input.map(|req| req.clone_boxed())
}

fn clone_boxed_some<R: Request + ?Sized>(
    input: Result<&R, Error>,
) -> Option<Result<Box<R>, Error>> {
    Some(input.map(|req| req.clone_boxed()))
}

fn copy<R: Copy>(input: Result<&R, Error>) -> Result<R, Error> {
    input.copied()
}

fn copy_some<R: Copy>(input: Result<&R, Error>) -> Option<Result<R, Error>> {
    Some(input.copied())
}

#[cfg(test)]
mod tests {
    use crate::{Channel, Context};
    use async_std::test as async_test;
    use c_str_macro::c_str;
    use serial_test::serial;

    #[async_test]
    #[serial]
    async fn downcast() {
        let ctx = Context::new().unwrap();
        let mut base = Channel::new(&ctx, c_str!("ca:test:ai")).unwrap();
        base.connected().await;
        let base = base.into_typed::<u8>().unwrap_err().1;
        base.into_typed::<f64>().unwrap();
    }
}
