use super::{
    get::Callback,
    subscribe::{LastFn, Queue, QueueFn},
    typed::TypedChannel,
    Get, GetFn, Put, Subscription,
};
use crate::{
    error::Error,
    request::Request,
    types::{Field, Value},
};
use derive_more::{Deref, DerefMut, From, Into};
use std::{
    any::type_name,
    fmt::{self, Debug},
};

impl<V: Value + ?Sized> TypedChannel<V> {
    pub fn into_value(self) -> ValueChannel<V> {
        ValueChannel::from(self)
    }
}

/// Channel used to read and write only value rather than other requests.
#[repr(transparent)]
#[derive(From, Into, Deref, DerefMut)]
pub struct ValueChannel<V: Value + ?Sized> {
    pub(crate) typed: TypedChannel<V>,
}

impl<V: Value + ?Sized> Debug for ValueChannel<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ValueChannel<{}>({:?})", type_name::<V>(), self.raw())
    }
}

impl<V: Value + ?Sized> ValueChannel<V> {
    /// Write value by reference to the channel.
    pub fn put_ref(&mut self, data: &V) -> Result<Put<'_>, Error> {
        self.typed.put_ref::<V>(data)
    }

    /// Request value from the channel and call callback when it's done.
    pub fn get_with<F>(&mut self, func: F) -> Get<'_, F>
    where
        F: Callback<Request = V>,
    {
        self.typed.get_with(func)
    }

    /// Subscribe to value updates and call closure each time when update occured.
    pub fn subscribe_with<F: Queue<Request = V>>(&mut self, func: F) -> Subscription<'_, F> {
        self.typed.subscribe_with(func)
    }
}

impl<T: Field> ValueChannel<[T]> {
    /// Request array value and store it in [`Vec`].
    pub fn get_vec(&mut self) -> Get<'_, GetFn<[T], Vec<T>>> {
        self.get_with(GetFn::<[T], Vec<T>>::new(clone_vec::<T>))
    }

    /// Write value to slice and return received value length (which may be greater than `dst` length).
    pub fn get_to_slice<'a, 'b>(&'a mut self, dst: &'b mut [T]) -> Get<'a, GetToSlice<'b, T>> {
        self.get_with(GetToSlice { dst })
    }

    /// Subscribe to array value updates and obtain [`Vec`] stream.
    pub fn subscribe_vec(&mut self) -> Subscription<'_, LastFn<[T], Vec<T>>> {
        self.subscribe_with(LastFn::<[T], Vec<T>>::new(clone_vec_some::<T>))
    }
}

impl<T: Field> ValueChannel<T> {
    /// Write scalar value.
    pub fn put(&mut self, val: T) -> Result<Put<'_>, Error> {
        self.typed.put::<T>(val)
    }

    /// Get scalar value.
    pub fn get(&mut self) -> Get<'_, GetFn<T, T>> {
        self.typed.get::<T>()
    }

    /// Subscribe to updates of scalar value.
    ///
    /// See [`TypedChannel::subscribe`].
    pub fn subscribe(&mut self) -> Subscription<'_, LastFn<T, T>> {
        self.typed.subscribe::<T>()
    }

    /// Subscribe to updates of scalar value and store all updates.
    ///
    /// See [`TypedChannel::subscribe_buffered`].
    pub fn subscribe_buffered(&mut self) -> Subscription<'_, QueueFn<T, T>> {
        self.typed.subscribe_buffered::<T>()
    }
}

fn clone_vec<T: Field>(input: Result<&[T], Error>) -> Result<Vec<T>, Error> {
    input.map(|data| Vec::from(data.clone_boxed()))
}

fn clone_vec_some<T: Field>(input: Result<&[T], Error>) -> Option<Result<Vec<T>, Error>> {
    Some(input.map(|data| Vec::from(data.clone_boxed())))
}

pub struct GetToSlice<'a, T: Field> {
    dst: &'a mut [T],
}

impl<'a, T: Field> Callback for GetToSlice<'a, T> {
    type Request = [T];
    type Output = usize;
    fn apply(self, input: Result<&Self::Request, Error>) -> Result<Self::Output, Error> {
        input.map(|src| {
            let len = usize::min(self.dst.len(), src.len());
            self.dst[..len].copy_from_slice(&src[..len]);
            src.len()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Context;
    use async_std::{task::sleep, test as async_test};
    use cstr::cstr;
    use futures::{join, pin_mut, StreamExt};
    use serial_test::serial;
    use std::{f64::consts::PI, time::Duration};

    #[async_test]
    #[serial]
    async fn put_get_scalar() {
        let ctx = Context::new().unwrap();

        let mut output = ctx.connect::<f64>(cstr!("ca:test:ao")).await.unwrap();
        output.put(PI).unwrap().await.unwrap();

        let mut input = ctx.connect::<f64>(cstr!("ca:test:ai")).await.unwrap();
        assert_eq!(input.get().await.unwrap(), PI);
    }

    #[async_test]
    #[serial]
    async fn subscribe_buffered() {
        let ctx = Context::new().unwrap();

        let mut output = ctx.connect::<f64>(cstr!("ca:test:ao")).await.unwrap();
        let mut input = ctx.connect::<f64>(cstr!("ca:test:ai")).await.unwrap();

        output.put(0.0).unwrap().await.unwrap();
        let monitor = input.subscribe_buffered();
        pin_mut!(monitor);
        assert_eq!(monitor.next().await.unwrap().unwrap(), 0.0);

        let count = 0x10;
        let values = (0..count)
            .map(|i| (i + 1) as f64 / 16.0)
            .collect::<Vec<_>>();
        join!(
            async {
                for x in values.iter() {
                    output.put(*x).unwrap().await.unwrap();
                }
            },
            async {
                for x in values.iter() {
                    assert_eq!(monitor.next().await.unwrap().unwrap(), *x);
                    sleep(Duration::from_millis(10)).await;
                }
            }
        );
    }

    #[async_test]
    #[serial]
    async fn put_get_array() {
        let ctx = Context::new().unwrap();

        let mut output = ctx.connect::<[i32]>(cstr!("ca:test:aao")).await.unwrap();
        let mut input = ctx.connect::<[i32]>(cstr!("ca:test:aai")).await.unwrap();

        let data = (0..8).collect::<Vec<i32>>();
        output.put_ref(&data).unwrap().await.unwrap();
        assert_eq!(input.get_vec().await.unwrap(), data);
    }

    #[async_test]
    #[serial]
    async fn subscribe_array() {
        let ctx = Context::new().unwrap();

        let mut output = ctx.connect::<[i32]>(cstr!("ca:test:aao")).await.unwrap();
        let mut input = ctx.connect::<[i32]>(cstr!("ca:test:aai")).await.unwrap();

        output.put_ref(&[-1]).unwrap().await.unwrap();
        let monitor = input.subscribe_vec();
        pin_mut!(monitor);
        assert_eq!(monitor.next().await.unwrap().unwrap(), [-1]);

        let count = 0x10;
        for i in 0..count {
            let data = (0..(i + 1)).collect::<Vec<_>>();
            output.put_ref(&data).unwrap().await.unwrap();
            assert_eq!(monitor.next().await.unwrap().unwrap(), data);
        }
    }
}
