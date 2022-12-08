use super::TypedChannel;
use crate::{
    error::{self, Error},
    types::{
        request::{ReadRequest, ScalarRequest, TypedRequest},
        Scalar,
    },
};
use derive_more::{Deref, DerefMut, Into};

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

    pub async fn get_request<R>(&mut self) -> Result<R, Error>
    where
        R: ReadRequest + TypedRequest<Type = T> + ScalarRequest,
    {
        self.get_request_with(|request: &R| {
            debug_assert_eq!(request.len(), 1);
            request.clone()
        })
        .await
    }

    pub async fn get(&mut self) -> Result<T, Error> {
        self.get_request::<T>().await
    }

    pub async fn put(&mut self, value: T) -> Result<(), Error> {
        self.put_slice(&[value])?.await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Channel, Context};
    use async_std::test as async_test;
    use c_str_macro::c_str;
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
}
