use super::AnyChannel;
use crate::{error::Error, traits::Downcast, types::DbField};
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

#[repr(transparent)]
pub struct Channel<T: Copy> {
    base: AnyChannel,
    _p: PhantomData<T>,
}

impl<T: Copy> Deref for Channel<T> {
    type Target = AnyChannel;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: Copy> DerefMut for Channel<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Downcast<Channel<i8>> for AnyChannel {
    fn is_instance_of(&self) -> bool {
        matches!(self.field_type(), Ok(DbField::Char)) && matches!(self.element_count(), Ok(1))
    }
    fn downcast_unchecked(self) -> Channel<i8> {
        Channel {
            base: self,
            _p: PhantomData,
        }
    }
}

impl Downcast<Channel<i16>> for AnyChannel {
    fn is_instance_of(&self) -> bool {
        matches!(self.field_type(), Ok(DbField::Short | DbField::Enum))
            && matches!(self.element_count(), Ok(1))
    }
    fn downcast_unchecked(self) -> Channel<i16> {
        Channel {
            base: self,
            _p: PhantomData,
        }
    }
}

impl Downcast<Channel<i32>> for AnyChannel {
    fn is_instance_of(&self) -> bool {
        matches!(self.field_type(), Ok(DbField::Long)) && matches!(self.element_count(), Ok(1))
    }
    fn downcast_unchecked(self) -> Channel<i32> {
        Channel {
            base: self,
            _p: PhantomData,
        }
    }
}

impl Downcast<Channel<f32>> for AnyChannel {
    fn is_instance_of(&self) -> bool {
        matches!(self.field_type(), Ok(DbField::Float)) && matches!(self.element_count(), Ok(1))
    }
    fn downcast_unchecked(self) -> Channel<f32> {
        Channel {
            base: self,
            _p: PhantomData,
        }
    }
}

impl Downcast<Channel<f64>> for AnyChannel {
    fn is_instance_of(&self) -> bool {
        matches!(self.field_type(), Ok(DbField::Double)) && matches!(self.element_count(), Ok(1))
    }
    fn downcast_unchecked(self) -> Channel<f64> {
        Channel {
            base: self,
            _p: PhantomData,
        }
    }
}

impl<T: Copy> Channel<T> {
    pub fn get(&mut self) -> Result<T, Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{AnyChannel, Channel, Context, Downcast};
    use async_std::test as async_test;
    use c_str_macro::c_str;
    use serial_test::serial;
    use std::sync::Arc;

    #[async_test]
    #[serial]
    async fn downcast() {
        let ctx = Arc::new(Context::new().unwrap());
        let any = AnyChannel::connect(ctx, c_str!("ca:test:ai"))
            .await
            .unwrap();
        let _: Channel<f64> = any.downcast().unwrap();
    }
}
