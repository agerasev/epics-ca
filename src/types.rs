use std::{
    cmp::Ordering,
    ffi::{c_char, CStr},
    mem::{align_of, size_of},
    ops::Deref,
    ptr::copy_nonoverlapping,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DbField {
    String,
    Short,
    // Int, // Alias to Short
    Float,
    Enum,
    Char,
    Long,
    Double,
}

impl DbField {
    pub fn try_from_raw(raw: i32) -> Option<Self> {
        match raw {
            sys::DBF_STRING => Some(DbField::String),
            sys::DBF_SHORT => Some(DbField::Short),
            // sys::DBF_INT => Some(DbField::Int),
            sys::DBF_FLOAT => Some(DbField::Float),
            sys::DBF_ENUM => Some(DbField::Enum),
            sys::DBF_CHAR => Some(DbField::Char),
            sys::DBF_LONG => Some(DbField::Long),
            sys::DBF_DOUBLE => Some(DbField::Double),
            _ => None,
        }
    }

    pub fn raw(&self) -> i32 {
        match self {
            DbField::String => sys::DBF_STRING,
            DbField::Short => sys::DBF_SHORT,
            // DbField::Int => sys::DBF_INT,
            DbField::Float => sys::DBF_FLOAT,
            DbField::Enum => sys::DBF_ENUM,
            DbField::Char => sys::DBF_CHAR,
            DbField::Long => sys::DBF_LONG,
            DbField::Double => sys::DBF_DOUBLE,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DbRequest {
    Base(DbField),
    Sts(DbField),
    Time(DbField),
    Gr(DbField),
    Ctrl(DbField),
    PutAck(bool),
    StsackString,
    ClassName,
}

impl DbRequest {
    pub fn raw(&self) -> i32 {
        match self {
            DbRequest::Base(dbf) => match dbf {
                DbField::String => sys::DBR_STRING,
                DbField::Short => sys::DBR_SHORT,
                // DbField::Int => sys::DBR_INT,
                DbField::Float => sys::DBR_FLOAT,
                DbField::Enum => sys::DBR_ENUM,
                DbField::Char => sys::DBR_CHAR,
                DbField::Long => sys::DBR_LONG,
                DbField::Double => sys::DBR_DOUBLE,
            },
            DbRequest::Sts(dbf) => match dbf {
                DbField::String => sys::DBR_STS_STRING,
                DbField::Short => sys::DBR_STS_SHORT,
                // DbField::Int => sys::DBR_STS_INT,
                DbField::Float => sys::DBR_STS_FLOAT,
                DbField::Enum => sys::DBR_STS_ENUM,
                DbField::Char => sys::DBR_STS_CHAR,
                DbField::Long => sys::DBR_STS_LONG,
                DbField::Double => sys::DBR_STS_DOUBLE,
            },
            DbRequest::Time(dbf) => match dbf {
                DbField::String => sys::DBR_TIME_STRING,
                DbField::Short => sys::DBR_TIME_SHORT,
                // DbField::Int => sys::DBR_TIME_INT,
                DbField::Float => sys::DBR_TIME_FLOAT,
                DbField::Enum => sys::DBR_TIME_ENUM,
                DbField::Char => sys::DBR_TIME_CHAR,
                DbField::Long => sys::DBR_TIME_LONG,
                DbField::Double => sys::DBR_TIME_DOUBLE,
            },
            DbRequest::Gr(dbf) => match dbf {
                DbField::String => sys::DBR_GR_STRING,
                DbField::Short => sys::DBR_GR_SHORT,
                // DbField::Int => sys::DBR_GR_INT,
                DbField::Float => sys::DBR_GR_FLOAT,
                DbField::Enum => sys::DBR_GR_ENUM,
                DbField::Char => sys::DBR_GR_CHAR,
                DbField::Long => sys::DBR_GR_LONG,
                DbField::Double => sys::DBR_GR_DOUBLE,
            },
            DbRequest::Ctrl(dbf) => match dbf {
                DbField::String => sys::DBR_CTRL_STRING,
                DbField::Short => sys::DBR_CTRL_SHORT,
                // DbField::Int => sys::DBR_CTRL_INT,
                DbField::Float => sys::DBR_CTRL_FLOAT,
                DbField::Enum => sys::DBR_CTRL_ENUM,
                DbField::Char => sys::DBR_CTRL_CHAR,
                DbField::Long => sys::DBR_CTRL_LONG,
                DbField::Double => sys::DBR_CTRL_DOUBLE,
            },
            DbRequest::PutAck(ts) => match ts {
                false => sys::DBR_PUT_ACKT,
                true => sys::DBR_PUT_ACKS,
            },
            DbRequest::StsackString => sys::DBR_STSACK_STRING,
            DbRequest::ClassName => sys::DBR_CLASS_NAME,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dbe {
    Value,
    Archive,
    Log,
    Alarm,
    Property,
}

impl Dbe {
    pub fn raw(&self) -> i32 {
        match self {
            Dbe::Value => sys::DBE_VALUE,
            Dbe::Archive => sys::DBE_ARCHIVE,
            Dbe::Log => sys::DBE_LOG,
            Dbe::Alarm => sys::DBE_ALARM,
            Dbe::Property => sys::DBE_PROPERTY,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct AccessRights {
    read_access: bool,
    write_access: bool,
}

impl AccessRights {
    pub fn raw(self) -> sys::ca_access_rights {
        let mut raw = 0;
        if self.read_access {
            raw |= sys::CA_READ_ACCESS;
        }
        if self.write_access {
            raw |= sys::CA_WRITE_ACCESS;
        }
        raw
    }
}

#[derive(Clone, Debug, Eq)]
#[repr(transparent)]
pub struct StaticCString<const N: usize> {
    data: [c_char; N],
}
impl<const N: usize> Default for StaticCString<N> {
    fn default() -> Self {
        Self { data: [0; N] }
    }
}
impl<const N: usize> PartialEq for StaticCString<N> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}
impl<const N: usize> PartialOrd for StaticCString<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}
impl<const N: usize> Ord for StaticCString<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.deref().cmp(other.deref())
    }
}
impl<const N: usize> StaticCString<N> {
    pub fn len(&self) -> Option<usize> {
        self.data
            .iter()
            .copied()
            .enumerate()
            .find(|(_, c)| *c == 0)
            .map(|(i, _)| i)
    }
    pub fn is_empty(&self) -> bool {
        self.data[0] == 0
    }
    pub fn from_array(data: [c_char; N]) -> Option<Self> {
        if data.iter().copied().any(|c| c == 0) {
            Some(Self { data })
        } else {
            None
        }
    }
    pub fn from_cstr(cstr: &CStr) -> Option<Self> {
        let bytes = cstr.to_bytes();
        if bytes.len() < N {
            let mut this = Self::default();
            unsafe {
                copy_nonoverlapping(
                    bytes.as_ptr() as *const i8,
                    this.data.as_mut_ptr(),
                    bytes.len() + 1,
                )
            };
            Some(this)
        } else {
            None
        }
    }
}
impl<const N: usize> Deref for StaticCString<N> {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        debug_assert!(
            self.data.iter().copied().any(|c| c == 0),
            "String is not nul-terminated"
        );
        unsafe { CStr::from_ptr(self.data.as_ptr()) }
    }
}
pub type EpicsString = StaticCString<{ sys::MAX_STRING_SIZE as usize }>;

trait Scalar: Sized {
    fn matches(dbf: DbField) -> bool;
}

impl Scalar for i8 {
    fn matches(dbf: DbField) -> bool {
        matches!(dbf, DbField::Char)
    }
}
impl Scalar for i16 {
    fn matches(dbf: DbField) -> bool {
        matches!(dbf, DbField::Short | DbField::Enum)
    }
}
impl Scalar for i32 {
    fn matches(dbf: DbField) -> bool {
        matches!(dbf, DbField::Long)
    }
}
impl Scalar for f32 {
    fn matches(dbf: DbField) -> bool {
        matches!(dbf, DbField::Float)
    }
}
impl Scalar for f64 {
    fn matches(dbf: DbField) -> bool {
        matches!(dbf, DbField::Double)
    }
}
impl Scalar for EpicsString {
    fn matches(dbf: DbField) -> bool {
        matches!(dbf, DbField::String)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn dbf_size(dbf: DbField) -> usize {
        unsafe { *(sys::dbr_size.as_ptr().offset(dbf.raw() as isize)) as usize }
    }

    #[test]
    fn dbr_sizes() {
        assert_eq!(dbf_size(DbField::String), size_of::<EpicsString>());
        assert_eq!(dbf_size(DbField::Short), size_of::<i16>());
        assert_eq!(dbf_size(DbField::Float), size_of::<f32>());
        assert_eq!(dbf_size(DbField::Enum), size_of::<i16>());
        assert_eq!(dbf_size(DbField::Char), size_of::<i8>());
        assert_eq!(dbf_size(DbField::Long), size_of::<i32>());
        assert_eq!(dbf_size(DbField::Double), size_of::<f64>());
    }
}
