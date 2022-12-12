use super::{Get, GetFn, Put, Subscribe, SubscribeFn, TypedChannel};
use crate::{
    error::{self, Error},
    types::{
        request::{Extended, ReadRequest, Request, ScalarRequest},
        Scalar,
    },
};
use derive_more::{Deref, DerefMut, Into};
use std::{collections::VecDeque, marker::PhantomData, ops::Deref};

impl<T: Scalar> TypedChannel<T> {
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
pub struct ScalarChannel<T: Scalar> {
    chan: TypedChannel<T>,
}

impl<T: Scalar> ScalarChannel<T> {
    pub fn new_unchecked(chan: TypedChannel<T>) -> Self {
        Self { chan }
    }

    pub fn put(&mut self, value: T) -> Result<Put<'_>, Error> {
        self.chan.put_slice(&[value])
    }

    pub fn get_request<R>(&mut self) -> Get<'_, GetScalar<R>>
    where
        R: ScalarRequest<Type = T> + ReadRequest,
    {
        self.chan.get_request_with(GetScalar { _p: PhantomData })
    }

    pub async fn get(&mut self) -> Result<T, Error> {
        self.get_request::<T>().await
    }

    pub fn subscribe_request<R>(&mut self) -> Subscribe<'_, SubscribeScalar<R>>
    where
        R: ScalarRequest<Type = T> + ReadRequest,
    {
        self.chan
            .subscribe_request_with(SubscribeScalar { last: None })
    }

    pub fn subscribe(&mut self) -> Subscribe<'_, SubscribeScalar<T>> {
        self.subscribe_request::<T>()
    }
    /*
    pub fn subscribe_buffered(
        &mut self,
    ) -> SubscribeBuffered<T, _> {
        SubscribeBuffered {
        self.chan.subscribe_with(|res: Result<&[T], Error>| {
            res.map(|data| {
                debug_assert_eq!(data.len(), 1);
                data[0]
            })
        })
    }
    */
}

pub struct GetScalar<R: ScalarRequest + ReadRequest> {
    _p: PhantomData<R>,
}

impl<R: ScalarRequest + ReadRequest> GetFn for GetScalar<R> {
    type Request = Extended<R>;
    type Output = R;
    fn apply(self, input: Result<&Self::Request, Error>) -> Result<Self::Output, Error> {
        input.map(|req| {
            debug_assert_eq!(req.len(), 1);
            req.deref().clone()
        })
    }
}

pub struct SubscribeScalar<R: ScalarRequest + ReadRequest> {
    last: Option<Result<R, Error>>,
}

impl<R: ScalarRequest + ReadRequest> SubscribeFn for SubscribeScalar<R> {
    type Request = Extended<R>;
    type Output = R;
    fn push(&mut self, input: Result<&Self::Request, Error>) {
        self.last = Some(input.map(|req| {
            debug_assert_eq!(req.len(), 1);
            req.deref().clone()
        }));
    }
    fn pop(&mut self) -> Option<Result<Self::Output, Error>> {
        self.last.take()
    }
}

/*
#[pin_project]
pub struct SubscribeBuffered<T: Scalar, S> {
    #[pin]
    stream: S,
    buffer: VecDeque<Result<T, Error>>,
}
*/

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
        let mut output = output.into_typed::<f64>().unwrap().into_scalar().unwrap();
        output.put(PI).unwrap().await.unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:ai")).unwrap();
        input.connected().await;
        let mut input = input.into_typed::<f64>().unwrap().into_scalar().unwrap();
        assert_eq!(input.get().await.unwrap(), PI);
    }

    #[async_test]
    #[serial]
    async fn subscribe_buffered() {
        let ctx = Context::new().unwrap();

        let mut output = Channel::new(ctx.clone(), c_str!("ca:test:ao")).unwrap();
        output.connected().await;
        let mut output = output.into_typed::<f64>().unwrap().into_scalar().unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:ai")).unwrap();
        input.connected().await;
        let mut input = input.into_typed::<f64>().unwrap().into_scalar().unwrap();

        output.put(0.0).unwrap().await.unwrap();
        let monitor = input.subscribe();
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
