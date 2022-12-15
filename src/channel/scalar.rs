use super::{
    subscribe::{LastFn, QueueFn},
    ArrayChannel, Get, GetFn, Put, Subscription,
};
use crate::{
    error::{self, Error},
    request::{ReadRequest, TypedRequest, WriteRequest},
    types::Field,
};
use derive_more::{Deref, DerefMut, Into};
use std::ops::DerefMut;

impl<T: Field> ArrayChannel<T> {
    pub fn into_scalar(self) -> Result<ScalarChannel<T>, (Error, Self)> {
        let count = match self.element_count() {
            Ok(n) => n,
            Err(err) => return Err((err, self)),
        };
        if count == 1 {
            Ok(ScalarChannel::new_unchecked(self))
        } else {
            Err((error::BADCOUNT, self))
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Deref, DerefMut, Into)]
pub struct ScalarChannel<T: Field> {
    chan: ArrayChannel<T>,
}

impl<T: Field> ScalarChannel<T> {
    /// Create [`ScalarChannel`] from [`ArrayChannel`] without checking that `element_count == 1`.
    pub fn new_unchecked(chan: ArrayChannel<T>) -> Self {
        Self { chan }
    }

    pub fn put<R>(&mut self, req: R) -> Result<Put<'_>, Error>
    where
        R: TypedRequest<Value = T> + WriteRequest + Copy,
    {
        self.chan.deref_mut().put::<R>(&req)
    }

    pub fn get<R>(&mut self) -> Get<'_, GetFn<R, R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.chan
            .deref_mut()
            .get_with(GetFn::<R, R>::new(copied::<R>))
    }

    pub fn subscribe<R>(&mut self) -> Subscription<'_, LastFn<R, R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.chan
            .deref_mut()
            .subscribe_with(LastFn::<R, R>::new(copied_some::<R>))
    }

    pub fn subscribe_buffered<R>(&mut self) -> Subscription<'_, QueueFn<R, R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.chan
            .deref_mut()
            .subscribe_with(QueueFn::<R, R>::new(copied_some::<R>))
    }
}

fn copied<R: Copy>(input: Result<&R, Error>) -> Result<R, Error> {
    input.copied()
}

fn copied_some<R: Copy>(input: Result<&R, Error>) -> Option<Result<R, Error>> {
    Some(input.copied())
}

#[cfg(test)]
mod tests {
    use crate::{Channel, Context};
    use async_std::{task::sleep, test as async_test};
    use c_str_macro::c_str;
    use futures::{join, pin_mut, StreamExt};
    use serial_test::serial;
    use std::{f64::consts::PI, time::Duration};

    #[async_test]
    #[serial]
    async fn put_get() {
        let ctx = Context::new().unwrap();

        let mut output = Channel::new(ctx.clone(), c_str!("ca:test:ao")).unwrap();
        output.connected().await;
        let mut output = output.into_array::<f64>().unwrap().into_scalar().unwrap();
        output.put::<f64>(PI).unwrap().await.unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:ai")).unwrap();
        input.connected().await;
        let mut input = input.into_array::<f64>().unwrap().into_scalar().unwrap();
        assert_eq!(input.get::<f64>().await.unwrap(), PI);
    }

    #[async_test]
    #[serial]
    async fn subscribe_buffered() {
        let ctx = Context::new().unwrap();

        let mut output = Channel::new(ctx.clone(), c_str!("ca:test:ao")).unwrap();
        output.connected().await;
        let mut output = output.into_array::<f64>().unwrap().into_scalar().unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:ai")).unwrap();
        input.connected().await;
        let mut input = input.into_array::<f64>().unwrap().into_scalar().unwrap();

        output.put::<f64>(0.0).unwrap().await.unwrap();
        let monitor = input.subscribe_buffered::<f64>();
        pin_mut!(monitor);
        assert_eq!(monitor.next().await.unwrap().unwrap(), 0.0);

        let count = 0x10;
        let values = (0..count)
            .into_iter()
            .map(|i| (i + 1) as f64 / 16.0)
            .collect::<Vec<_>>();
        join!(
            async {
                for x in values.iter() {
                    output.put::<f64>(*x).unwrap().await.unwrap();
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
}
