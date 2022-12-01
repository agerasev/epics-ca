use crate::types::DbField;
use std::ffi::CStr;

pub trait Scalar {
    fn matches_field(dbf: DbField) -> bool;
}

pub trait Type {
    fn matches(dbf: DbField, count: usize) -> bool;
}

impl Scalar for i8 {
    fn matches_field(dbf: DbField) -> bool {
        matches!(dbf, DbField::Char)
    }
}

impl Scalar for i16 {
    fn matches_field(dbf: DbField) -> bool {
        matches!(dbf, DbField::Short | DbField::Enum)
    }
}

impl Scalar for i32 {
    fn matches_field(dbf: DbField) -> bool {
        matches!(dbf, DbField::Long)
    }
}

impl Scalar for f32 {
    fn matches_field(dbf: DbField) -> bool {
        matches!(dbf, DbField::Float)
    }
}

impl Scalar for f64 {
    fn matches_field(dbf: DbField) -> bool {
        matches!(dbf, DbField::Double)
    }
}

impl Scalar for CStr {
    fn matches_field(dbf: DbField) -> bool {
        matches!(dbf, DbField::String)
    }
}

impl<T: Scalar + ?Sized> Type for T {
    fn matches(dbf: DbField, count: usize) -> bool {
        Self::matches_field(dbf) && matches!(count, 1)
    }
}

impl<T: Scalar> Type for [T] {
    fn matches(dbf: DbField, _count: usize) -> bool {
        T::matches_field(dbf)
    }
}
