use super::{ScalarChannel, TypedChannel};
use crate::{error::Error, types::Scalar};
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
    /*
    pub async fn get_with<F: FnOnce(&[T]) -> R + Send, R>(&mut self, func: F) -> Result<R, Error> {
        let ts = None;
        loop {
            self.value
                .get_with(|data| {
                    assert_eq!(data.len(), 1);
                    data[0]
                })
                .await;
        }
    }
    */
    pub async fn put(&mut self, data: &[T]) -> Result<(), Error> {
        self.value.put_slice(data)?.await
    }
}
