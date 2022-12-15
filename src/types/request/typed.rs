use super::{Meta, ReadRequest, Request, WriteRequest};
use crate::{
    error::{self, Error},
    types::{EpicsEnum, EpicsString, Field, Float, Int, RequestId, Value},
};

pub trait TypedRequest<V: Value>: Request {
    fn value(&self) -> &V;
    fn value_mut(&mut self) -> &mut V;
}

macro_rules! impl_request_methods {
    () => {
        fn len(&self) -> usize {
            self.value().len()
        }
        unsafe fn from_ptr<'a>(ptr: *const u8, count: usize) -> Result<&'a Self, crate::Error> {
            match M::Value::cast_ptr(ptr, count) {
                Some(ptr) => Ok(&*(ptr as *const Self)),
                None => Err(error::BADCOUNT),
            }
        }
    };
}

unsafe impl<V: Value> Request for V {
    type Raw = <V::Field as Field>::Raw;
    const ENUM: RequestId = RequestId::Base(<V::Field as Field>::ENUM);
    impl_request_methods!();
}
impl<V: Value> TypedRequest<V> for V {
    fn value(&self) -> &V {
        self
    }
    fn value_mut(&mut self) -> &mut V {
        self
    }
}
impl<V: Value> ReadRequest for V {}
impl<V: Value> WriteRequest for V {}

unsafe impl<M: Meta> Request for M {
    type Raw = M::Raw;
    const ENUM: RequestId = M::ENUM;
    impl_request_methods!();
}
impl<V: Value, M: Meta<Value = V>> TypedRequest<V> for M {
    fn value(&self) -> &V {
        self.value()
    }
    fn value_mut(&mut self) -> &mut V {
        self.value_mut()
    }
}
impl<M: Meta> ReadRequest for M {}
