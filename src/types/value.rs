use super::{DbField, EpicsEnum, EpicsString};
use std::{mem::align_of, ptr, slice::from_raw_parts};

pub trait Scalar: Type + Sized + Clone {
    type Raw: Sized;

    const ENUM: DbField;

    fn from_raw(raw: <Self as Scalar>::Raw) -> Self {
        unsafe { ptr::read(&raw as *const _ as *const Self) }
    }
}

pub trait Primitive: Scalar {}

impl Scalar for u8 {
    type Raw = u8;
    const ENUM: DbField = DbField::Char;
}
impl Primitive for u8 {}

impl Scalar for i16 {
    type Raw = i16;
    const ENUM: DbField = DbField::Short;
}
impl Primitive for i16 {}

impl Scalar for EpicsEnum {
    type Raw = u16;
    const ENUM: DbField = DbField::Enum;
}
impl Primitive for EpicsEnum {}

impl Scalar for i32 {
    type Raw = i32;
    const ENUM: DbField = DbField::Long;
}
impl Primitive for i32 {}

impl Scalar for f32 {
    type Raw = f32;
    const ENUM: DbField = DbField::Float;
}
impl Primitive for f32 {}

impl Scalar for f64 {
    type Raw = f64;
    const ENUM: DbField = DbField::Double;
}
impl Primitive for f64 {}

impl Scalar for EpicsString {
    type Raw = sys::epicsOldString;
    const ENUM: DbField = DbField::String;
}

pub trait Type: Send {
    type Element: Type + Scalar;

    fn match_field(dbf: DbField) -> bool;
    fn match_count(count: usize) -> bool;

    fn element_count(&self) -> usize;

    /// # Safety
    ///
    /// `data` must be valid and `'a` must be appropriate.
    unsafe fn from_raw<'a>(data: *const <Self::Element as Scalar>::Raw, count: usize) -> &'a Self;

    fn copy_from(&mut self, src: &Self);
}

impl<T: Scalar> Type for T {
    type Element = T;

    fn match_field(dbf: DbField) -> bool {
        Self::ENUM == dbf
    }
    fn match_count(count: usize) -> bool {
        count == 1
    }

    fn element_count(&self) -> usize {
        1
    }

    unsafe fn from_raw<'a>(data: *const <Self::Element as Scalar>::Raw, count: usize) -> &'a Self {
        debug_assert!(data.align_offset(align_of::<T::Raw>()) == 0);
        assert_eq!(count, 1);
        &*(data as *const T)
    }

    fn copy_from(&mut self, src: &Self) {
        *self = src.clone();
    }
}

impl<T: Scalar> Type for [T] {
    type Element = T;

    fn match_field(dbf: DbField) -> bool {
        T::match_field(dbf)
    }
    fn match_count(_count: usize) -> bool {
        true
    }

    fn element_count(&self) -> usize {
        self.len()
    }

    unsafe fn from_raw<'a>(data: *const <Self::Element as Scalar>::Raw, count: usize) -> &'a Self {
        debug_assert!(data.align_offset(align_of::<T::Raw>()) == 0);
        from_raw_parts(data as *const T, count)
    }

    fn copy_from(&mut self, src: &Self) {
        self[..src.len()].clone_from_slice(src);
    }
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
