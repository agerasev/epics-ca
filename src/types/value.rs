use super::{DbField, EpicsEnum, EpicsString};
use std::ptr;

pub trait Scalar: Copy + Send + Sized {
    type Raw: Sized;

    const ENUM: DbField;

    fn from_raw(raw: <Self as Scalar>::Raw) -> Self {
        unsafe { ptr::read(&raw as *const _ as *const Self) }
    }
}

impl Scalar for u8 {
    type Raw = u8;
    const ENUM: DbField = DbField::Char;
}

impl Scalar for i16 {
    type Raw = i16;
    const ENUM: DbField = DbField::Short;
}

impl Scalar for EpicsEnum {
    type Raw = u16;
    const ENUM: DbField = DbField::Enum;
}

impl Scalar for i32 {
    type Raw = i32;
    const ENUM: DbField = DbField::Long;
}

impl Scalar for f32 {
    type Raw = f32;
    const ENUM: DbField = DbField::Float;
}

impl Scalar for f64 {
    type Raw = f64;
    const ENUM: DbField = DbField::Double;
}

impl Scalar for EpicsString {
    type Raw = sys::epicsOldString;
    const ENUM: DbField = DbField::String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    fn assert_layout<T: Scalar>() {
        assert_eq!(size_of::<T>(), size_of::<T::Raw>());
        assert_eq!(align_of::<T>(), align_of::<T::Raw>());
    }

    #[test]
    fn layout() {
        assert_layout::<u8>();
        assert_layout::<i16>();
        assert_layout::<EpicsEnum>();
        assert_layout::<i32>();
        assert_layout::<f32>();
        assert_layout::<f64>();
        assert_layout::<EpicsString>();
    }
}
