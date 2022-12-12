use super::TypedChannel;
use crate::{
    error::{self, Error},
    types::{
        request::{Extended, ReadRequest, Request, ScalarRequest},
        Scalar,
    },
};
use derive_more::{Deref, DerefMut, Into};
use futures::Stream;
use pin_project::pin_project;
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

    pub async fn put(&mut self, value: T) -> Result<(), Error> {
        self.chan.put_slice(&[value])?.await
    }

    pub async fn get_request<R>(&mut self) -> Result<R, Error>
    where
        R: ScalarRequest<Type = T> + ReadRequest,
    {
        self.chan
            .get_request_with(|res: Result<&Extended<R>, Error>| {
                res.map(|req| {
                    debug_assert_eq!(req.len(), 1);
                    req.deref().clone()
                })
            })
            .await
    }

    pub async fn get(&mut self) -> Result<T, Error> {
        self.get_request::<T>().await
    }

    pub fn subscribe_request<R>(&mut self) -> impl Stream<Item = Result<R, Error>> + '_
    where
        R: ScalarRequest<Type = T> + ReadRequest,
    {
        self.chan
            .subscribe_request_with(|res: Result<&Extended<R>, Error>| {
                Some(res.map(|req| {
                    debug_assert_eq!(req.len(), 1);
                    req.deref().clone()
                }))
            })
    }

    pub fn subscribe(&mut self) -> impl Stream<Item = Result<T, Error>> + '_ {
        self.subscribe_request::<T>()
    }
    /*
    pub fn subscribe_buffered(&mut self) -> impl Stream<Item = Result<T, Error>> + '_ {
        self.chan.subscribe_with(|res: Result<&[T], Error>| {
            res.map(|data| {
                debug_assert_eq!(data.len(), 1);
                data[0]
            })
        })
    }
    */
}

#[pin_project]
pub struct SubscribeBuffered<'a, T: Scalar, S: Stream<Item = Result<T, Error>> + 'a> {
    #[pin]
    stream: S,
    buffer: VecDeque<Result<T, Error>>,
    _p: PhantomData<&'a u8>,
}

#[cfg(test)]
mod tests {
    use crate::{Channel, Context};
    use async_std::test as async_test;
    use c_str_macro::c_str;
    use futures::{join, pin_mut, StreamExt};
    use serial_test::serial;
    use std::f64::consts::PI;

    #[async_test]
    #[serial]
    async fn put_get() {
        let ctx = Context::new().unwrap();

        let mut output = Channel::new(ctx.clone(), c_str!("ca:test:ao")).unwrap();
        output.connected().await;
        let mut output = output.into_typed::<f64>().unwrap().into_scalar().unwrap();
        output.put(PI).await.unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:ai")).unwrap();
        input.connected().await;
        let mut input = input.into_typed::<f64>().unwrap().into_scalar().unwrap();
        assert_eq!(input.get().await.unwrap(), PI);
    }

    #[async_test]
    #[serial]
    async fn subscribe() {
        let ctx = Context::new().unwrap();

        let mut output = Channel::new(ctx.clone(), c_str!("ca:test:ao")).unwrap();
        output.connected().await;
        let mut output = output.into_typed::<f64>().unwrap().into_scalar().unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:ai")).unwrap();
        input.connected().await;
        let mut input = input.into_typed::<f64>().unwrap().into_scalar().unwrap();

        output.put(0.0).await.unwrap();
        let monitor = input.subscribe();
        pin_mut!(monitor);
        assert_eq!(monitor.next().await.unwrap().unwrap(), 0.0);

        let count = 0x10;
        join!(
            async {
                for i in 0..count {
                    output.put((i + 1) as f64 / 16.0).await.unwrap();
                }
            },
            async {
                for i in 0..count {
                    assert_eq!(
                        monitor.next().await.unwrap().unwrap(),
                        (i + 1) as f64 / 16.0
                    );
                }
            }
        );
    }
}
