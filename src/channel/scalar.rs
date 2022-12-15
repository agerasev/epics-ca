use super::{get::Callback, subscribe::Queue, ArrayChannel, Get, Put, Subscription};
use crate::{
    error::{self, Error},
    request::{ReadRequest, TypedRequest, WriteRequest},
    types::Field,
};
use derive_more::{Deref, DerefMut, Into};
use std::{collections::VecDeque, marker::PhantomData, ops::DerefMut};

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

    pub fn put<R>(&mut self, value: T) -> Result<Put<'_>, Error>
    where
        R: TypedRequest<Value = T> + WriteRequest + Copy,
    {
        self.chan.put(&[value])
    }

    pub fn get<R>(&mut self) -> Get<'_, GetScalar<R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.chan
            .deref_mut()
            .get_with(GetScalar { _p: PhantomData })
    }

    pub fn subscribe<R>(&mut self) -> Subscription<'_, SubscribeScalar<R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.chan
            .deref_mut()
            .subscribe_with(SubscribeScalar { last: None })
    }

    pub fn subscribe_buffered<R>(&mut self) -> Subscription<'_, SubscribeBuffered<R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.chan.deref_mut().subscribe_with(SubscribeBuffered {
            queue: VecDeque::new(),
        })
    }
}

pub struct GetScalar<R: TypedRequest + ReadRequest + Copy> {
    _p: PhantomData<R>,
}

impl<R: TypedRequest + ReadRequest + Copy> Callback for GetScalar<R> {
    type Request = R;
    type Output = R;
    fn apply(self, input: Result<&Self::Request, Error>) -> Result<Self::Output, Error> {
        input.cloned()
    }
}

pub struct SubscribeScalar<R: TypedRequest + ReadRequest + Copy> {
    last: Option<Result<R, Error>>,
}

impl<R: TypedRequest + ReadRequest + Copy> Queue for SubscribeScalar<R> {
    type Request = R;
    type Output = R;
    fn push(&mut self, input: Result<&Self::Request, Error>) {
        self.last = Some(input.cloned());
    }
    fn pop(&mut self) -> Option<Result<Self::Output, Error>> {
        self.last.take()
    }
}

pub struct SubscribeBuffered<R: TypedRequest + ReadRequest + Copy> {
    queue: VecDeque<Result<R, Error>>,
}

impl<R: TypedRequest + ReadRequest + Copy> Queue for SubscribeBuffered<R> {
    type Request = R;
    type Output = R;
    fn push(&mut self, input: Result<&Self::Request, Error>) {
        self.queue.push_back(input.map(|req| *req));
    }
    fn pop(&mut self) -> Option<Result<Self::Output, Error>> {
        self.queue.pop_front()
    }
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
