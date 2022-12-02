use super::{DbField, EpicsString};
use std::{
    mem::{align_of, size_of},
    ptr::copy_nonoverlapping,
};

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EpicsEnum(pub u16);

trait Scalar: Sized {
    const FIELD: DbField;

    fn matches(dbf: DbField) -> bool {
        dbf == Self::FIELD
    }
}

impl Scalar for i8 {
    const FIELD: DbField = DbField::Char;
}
impl Scalar for i16 {
    const FIELD: DbField = DbField::Short;
}
impl Scalar for EpicsEnum {
    const FIELD: DbField = DbField::Enum;
}
impl Scalar for i32 {
    const FIELD: DbField = DbField::Long;
}
impl Scalar for f32 {
    const FIELD: DbField = DbField::Float;
}
impl Scalar for f64 {
    const FIELD: DbField = DbField::Double;
}
impl Scalar for EpicsString {
    const FIELD: DbField = DbField::String;
}

pub trait Type {
    type Element: Type + Sized;

    fn match_field(dbf: DbField) -> bool;
    fn match_count(count: usize) -> bool;

    fn element_count(&self) -> usize;

    fn as_ptr(&self) -> *const u8;
    fn as_mut_ptr(&mut self) -> *mut u8;

    /// # Safety
    ///
    /// `src` and `dst` must be valid pointers to memory of size `count * size_of::<T>()` and must not overlap.
    /// Also `dst` must be aligned as `T::Element`.
    unsafe fn copy_data(dst: *mut u8, src: *const u8, count: usize);
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

    unsafe fn copy_data(dst: *mut u8, src: *const u8, count: usize) {
        debug_assert_eq!(count, 1);
        debug_assert_eq!(dst.align_offset(align_of::<T>()), 0);
        copy_nonoverlapping(src, dst, size_of::<T>());
    }
}

impl<T: Type> Type for [T] {
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

    unsafe fn copy_data(dst: *mut u8, src: *const u8, count: usize) {
        debug_assert_eq!(dst.align_offset(align_of::<T>()), 0);
        copy_nonoverlapping(src, dst, count * size_of::<T>());
    }
}
