use super::Channel;
use crate::{
    error::{self, Error},
    types::Scalar,
};
use derive_more::{Deref, DerefMut, Into};
use std::{
    any::type_name,
    fmt::{self, Debug},
    marker::PhantomData,
    ptr,
};

impl Channel {
    pub fn into_typed<T: Scalar>(self) -> Result<TypedChannel<T>, (Error, Self)> {
        let dbf = match self.field_type() {
            Ok(dbf) => dbf,
            Err(err) => return Err((err, self)),
        };
        if dbf == T::ENUM {
            Ok(TypedChannel::new_unchecked(self))
        } else {
            Err((error::BADTYPE, self))
        }
    }
}

/// Typed channel.
#[repr(transparent)]
#[derive(Deref, DerefMut, Into)]
pub struct TypedChannel<T: Scalar> {
    #[deref]
    #[deref_mut]
    pub(crate) base: Channel,
    #[into(ignore)]
    _p: PhantomData<T>,
}

impl<T: Scalar> TypedChannel<T> {
    /// Convert [`AnyChannel`] to [`Channel<T>`] without type checking.
    ///
    /// It is safe because the type of remote channel can change at any moment and checks are done reading/writing/monitoring anyway.
    ///
    /// If you want to check type before converting use [`AnyChannel::into_typed`].
    pub fn new_unchecked(base: Channel) -> Self {
        Self {
            base,
            _p: PhantomData,
        }
    }
}

pub(crate) struct ProcessData {
    id_counter: usize,
    pub(crate) put_res: Option<Result<(), Error>>,
    pub(crate) state: *mut u8,
}

impl ProcessData {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            put_res: None,
            state: ptr::null_mut(),
        }
    }
    pub fn id(&self) -> usize {
        self.id_counter
    }
    pub fn change_id(&mut self) {
        self.id_counter += 1;
    }
}

impl<T: Scalar> Debug for TypedChannel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Channel<{}>({:?})", type_name::<T>(), self.raw())
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
    async fn downcast() {
        let ctx = Context::new().unwrap();
        let mut base = Channel::new(ctx, c_str!("ca:test:ai")).unwrap();
        base.connected().await;
        base.into_typed::<f64>().unwrap();
    }
}
