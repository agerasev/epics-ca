use super::{ArrayChannel, Get, GetFn, Put, Subscribe, SubscribeFn};
use crate::{
    error::{self, Error},
    types::{
        request::{ReadRequest, TypedRequest},
        Field,
    },
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
    pub fn new_unchecked(chan: ArrayChannel<T>) -> Self {
        Self { chan }
    }

    pub fn put(&mut self, value: T) -> Result<Put<'_>, Error> {
        self.chan.put(&[value])
    }

    pub fn get_request<R>(&mut self) -> Get<'_, GetScalar<R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.chan
            .deref_mut()
            .get_request_with(GetScalar { _p: PhantomData })
    }

    pub async fn get(&mut self) -> Result<T, Error> {
        self.get_request::<T>().await
    }

    pub fn subscribe_request<R>(&mut self) -> Subscribe<'_, SubscribeScalar<R>>
    where
        R: TypedRequest<Value = T> + ReadRequest + Copy,
    {
        self.chan
            .deref_mut()
            .subscribe_request_with(SubscribeScalar { last: None })
    }

    pub fn subscribe(&mut self) -> Subscribe<'_, SubscribeScalar<T>> {
        self.subscribe_request::<T>()
    }

    pub fn subscribe_buffered(&mut self) -> Subscribe<'_, SubscribeBuffered<T>> {
        self.chan.subscribe_with(SubscribeBuffered {
            queue: VecDeque::new(),
        })
    }
}

pub struct GetScalar<R: TypedRequest + ReadRequest + Copy> {
    _p: PhantomData<R>,
}

impl<R: TypedRequest + ReadRequest + Copy> GetFn for GetScalar<R> {
    type Request = R;
    type Output = R;
    fn apply(self, input: Result<&Self::Request, Error>) -> Result<Self::Output, Error> {
        input.cloned()
    }
}

pub struct SubscribeScalar<R: TypedRequest + ReadRequest + Copy> {
    last: Option<Result<R, Error>>,
}

impl<R: TypedRequest + ReadRequest + Copy> SubscribeFn for SubscribeScalar<R> {
    type Request = R;
    type Output = R;
    fn push(&mut self, input: Result<&Self::Request, Error>) {
        self.last = Some(input.cloned());
    }
    fn pop(&mut self) -> Option<Result<Self::Output, Error>> {
        self.last.take()
    }
}

pub struct SubscribeBuffered<T: Field> {
    queue: VecDeque<Result<T, Error>>,
}

impl<T: Field> SubscribeFn for SubscribeBuffered<T> {
    type Request = [T];
    type Output = T;
    fn push(&mut self, input: Result<&Self::Request, Error>) {
        self.queue.push_back(input.map(|data| {
            debug_assert_eq!(data.len(), 1);
            data[0]
        }));
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
        output.put(PI).unwrap().await.unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:ai")).unwrap();
        input.connected().await;
        let mut input = input.into_array::<f64>().unwrap().into_scalar().unwrap();
        assert_eq!(input.get().await.unwrap(), PI);
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

        output.put(0.0).unwrap().await.unwrap();
        let monitor = input.subscribe_buffered();
        pin_mut!(monitor);
        assert_eq!(monitor.next().await.unwrap().unwrap(), 0.0);

        let count = 0x10;
        join!(
            async {
                for i in 0..count {
                    output.put((i + 1) as f64 / 16.0).unwrap().await.unwrap();
                }
            },
            async {
                for i in 0..count {
                    assert_eq!(
                        monitor.next().await.unwrap().unwrap(),
                        (i + 1) as f64 / 16.0
                    );
                    sleep(Duration::from_millis(10)).await;
                }
            }
        );
    }
}
