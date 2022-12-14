use super::{EpicsEnum, EpicsString, FieldId};
use std::fmt::Debug;

/// # Safety
///
/// Should be implemented only for types supported by channel access.
pub unsafe trait Field: Copy + Send + Sized + 'static + Debug {
    type Raw: Copy + Send + Sized + 'static;
    const ENUM: FieldId;
}
pub trait Int: Field {}
pub trait Float: Field {}

unsafe impl Field for u8 {
    type Raw = u8;
    const ENUM: FieldId = FieldId::Char;
}
impl Int for u8 {}

unsafe impl Field for i16 {
    type Raw = i16;
    const ENUM: FieldId = FieldId::Short;
}
impl Int for i16 {}

unsafe impl Field for EpicsEnum {
    type Raw = u16;
    const ENUM: FieldId = FieldId::Enum;
}

unsafe impl Field for i32 {
    type Raw = i32;
    const ENUM: FieldId = FieldId::Long;
}
impl Int for i32 {}

unsafe impl Field for f32 {
    type Raw = f32;
    const ENUM: FieldId = FieldId::Float;
}
impl Float for f32 {}

unsafe impl Field for f64 {
    type Raw = f64;
    const ENUM: FieldId = FieldId::Double;
}
impl Float for f64 {}

unsafe impl Field for EpicsString {
    type Raw = sys::epicsOldString;
    const ENUM: FieldId = FieldId::String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    fn assert_layout<T: Field>() {
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
