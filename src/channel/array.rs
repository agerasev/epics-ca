use super::{Put, ScalarChannel, TypedChannel};
use crate::{
    error::Error,
    types::{
        request::{ArrayRequest, Extended, ReadRequest, ScalarRequest, Time},
        Scalar,
    },
};
use std::ffi::CString;

impl<T: Scalar> TypedChannel<T> {
    pub async fn into_array(self) -> Result<ArrayChannel<T>, Error> {
        ArrayChannel::new(self).await
    }
}

#[derive(Debug)]
pub struct ArrayChannel<T: Scalar> {
    value: TypedChannel<T>,
    nord: ScalarChannel<f64>,
}

impl<T: Scalar> ArrayChannel<T> {
    pub async fn new(chan: TypedChannel<T>) -> Result<Self, Error> {
        let name = CString::from_vec_with_nul(
            chan.name()
                .to_bytes()
                .iter()
                .chain(b".NORD\0".iter())
                .copied()
                .collect(),
        )
        .unwrap();

        let nord = chan
            .context()
            .clone()
            .connect_typed::<f64>(&name)
            .await?
            .into_scalar()
            .map_err(|(e, _)| e)?;

        Ok(Self { value: chan, nord })
    }

    pub fn put(&mut self, data: &[T]) -> Result<Put<'_>, Error> {
        self.value.put_slice(data)
    }
}

impl<T: Scalar> ArrayChannel<T>
where
    Time<T>: ScalarRequest<Type = T> + ReadRequest,
{
    pub async fn get_with<Q: Send, F: FnOnce(&[T]) -> Q + Send>(
        &mut self,
        func: F,
    ) -> Result<Q, Error> {
        let mut func_cell = Some(func);
        loop {
            let nord = self.nord.get_request::<Time<f64>>().await?;
            let result = self
                .value
                .get_request_with(|result: Result<&Extended<Time<T>>, Error>| {
                    result.map(|request| {
                        println!(
                            "nord: {}, timestamp: {:?}",
                            nord.value(),
                            nord.stamp.to_system()
                        );
                        if request.stamp == nord.stamp {
                            let len = *nord.value() as usize;
                            Some(func_cell.take().unwrap()(&request.values()[..len]))
                        } else {
                            None
                        }
                    })
                })
                .await?;
            if let Some(ret) = result {
                break Ok(ret);
            }
        }
    }

    pub async fn get_to_slice(&mut self, dst: &mut [T]) -> Result<usize, Error> {
        self.get_with(|src: &[T]| {
            let len = usize::min(dst.len(), src.len());
            dst[..len].copy_from_slice(&src[..len]);
            len
        })
        .await
    }

    pub async fn get_vec(&mut self) -> Result<Vec<T>, Error> {
        self.get_with(|s: &[T]| Vec::from_iter(s.iter().cloned()))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Channel, Context};
    use async_std::test as async_test;
    use c_str_macro::c_str;
    use serial_test::serial;

    #[async_test]
    #[serial]
    async fn put_get() {
        let ctx = Context::new().unwrap();

        let mut output = Channel::new(ctx.clone(), c_str!("ca:test:aao")).unwrap();
        output.connected().await;
        let mut output = output
            .into_typed::<i32>()
            .unwrap()
            .into_array()
            .await
            .unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:aai")).unwrap();
        input.connected().await;
        let mut input = input
            .into_typed::<i32>()
            .unwrap()
            .into_array()
            .await
            .unwrap();

        let data = (0..16).into_iter().collect::<Vec<i32>>();
        output.put(&data).unwrap().await.unwrap();
        assert_eq!(input.get_vec().await.unwrap(), data);
    }
}
