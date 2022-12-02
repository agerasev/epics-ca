use super::{DbField, EpicsEnum, EpicsString};
use std::{mem::align_of, ptr::copy_nonoverlapping};

pub trait Scalar: Type + Sized {
    type Raw: Sized;

    const FIELD: DbField;

    fn matches(dbf: DbField) -> bool {
        dbf == Self::FIELD
    }

    /// # Safety
    ///
    /// `src` and `dst` :
    /// + must be valid pointers to memory of size `count * size_of::<T>()`.
    /// + must not overlap.
    /// + must be aligned as `T::Element`.
    unsafe fn copy_data(src: *const Self::Raw, dst: *mut Self, count: usize) {
        debug_assert!(dst.align_offset(align_of::<Self>()) == 0);
        debug_assert!(src.align_offset(align_of::<Self::Raw>()) == 0);
        copy_nonoverlapping(src, dst as *mut Self::Raw, count);
    }
}

pub trait Primitive: Scalar {}

impl Scalar for u8 {
    type Raw = u8;
    const FIELD: DbField = DbField::Char;
}
impl Primitive for u8 {}

impl Scalar for i16 {
    type Raw = i16;
    const FIELD: DbField = DbField::Short;
}
impl Primitive for i16 {}

impl Scalar for EpicsEnum {
    type Raw = u16;
    const FIELD: DbField = DbField::Enum;
}
impl Primitive for EpicsEnum {}

impl Scalar for i32 {
    type Raw = i32;
    const FIELD: DbField = DbField::Long;
}
impl Primitive for i32 {}

impl Scalar for f32 {
    type Raw = f32;
    const FIELD: DbField = DbField::Float;
}
impl Primitive for f32 {}

impl Scalar for f64 {
    type Raw = f64;
    const FIELD: DbField = DbField::Double;
}
impl Primitive for f64 {}

impl Scalar for EpicsString {
    type Raw = sys::epicsOldString;
    const FIELD: DbField = DbField::String;
}

pub trait Type {
    type Element: Type + Scalar;

    fn match_field(dbf: DbField) -> bool;
    fn match_count(count: usize) -> bool;

    fn element_count(&self) -> usize;

    fn as_ptr(&self) -> *const u8;
    fn as_mut_ptr(&mut self) -> *mut u8;
}

impl<T: Scalar> Type for T {
    type Element = T;

    fn match_field(dbf: DbField) -> bool {
        Self::matches(dbf)
    }
    fn match_count(count: usize) -> bool {
        count == 1
    }

    fn element_count(&self) -> usize {
        1
    }

    fn as_ptr(&self) -> *const u8 {
        self as *const _ as *const u8
    }
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self as *mut _ as *mut u8
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

    fn as_ptr(&self) -> *const u8 {
        self.as_ptr() as *const u8
    }
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr() as *mut u8
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
