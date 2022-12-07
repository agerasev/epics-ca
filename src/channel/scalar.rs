use super::TypedChannel;
use crate::{
    error::{self, Error},
    types::Scalar,
};

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
struct ScalarChannel<T: Scalar> {
    chan: TypedChannel<T>,
}

impl<T: Scalar> ScalarChannel<T> {
    pub fn new_unchecked(chan: TypedChannel<T>) -> Self {}
}
