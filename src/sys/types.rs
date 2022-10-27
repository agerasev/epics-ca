use core::ffi::*;

pub const MAX_STRING_SIZE: usize = 40;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum epicsBoolean {
    epicsFalse = 0,
    epicsTrue = 1,
}
pub type epicsInt8 = i8;
pub type epicsUInt8 = u8;
pub type epicsInt16 = i16;
pub type epicsUInt16 = u16;
pub type epicsInt32 = i32;
pub type epicsUInt32 = u32;
pub type epicsInt64 = i64;
pub type epicsUInt64 = u64;
pub type epicsEnum16 = epicsUInt16;
pub type epicsFloat32 = f32;
pub type epicsFloat64 = f64;
pub type epicsStatus = epicsInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct epicsString {
    pub length: c_uint,
    pub pString: *mut c_char,
}
pub type epicsOldString = [c_char; MAX_STRING_SIZE];
#[repr(C)]
#[derive(Copy, Clone)]
pub union epics_any {
    pub int8: epicsInt8,
    pub uInt8: epicsUInt8,
    pub int16: epicsInt16,
    pub uInt16: epicsUInt16,
    pub enum16: epicsEnum16,
    pub int32: epicsInt32,
    pub uInt32: epicsUInt32,
    pub int64: epicsInt64,
    pub uInt64: epicsUInt64,
    pub float32: epicsFloat32,
    pub float64: epicsFloat64,
    pub string: epicsString,
}
pub type epicsAny = epics_any;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum epicsType {
    epicsInt8T = 0,
    epicsUInt8T = 1,
    epicsInt16T = 2,
    epicsUInt16T = 3,
    epicsEnum16T = 4,
    epicsInt32T = 5,
    epicsUInt32T = 6,
    epicsFloat32T = 7,
    epicsFloat64T = 8,
    epicsStringT = 9,
    epicsOldStringT = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum epicsTypeClass {
    epicsIntC = 0,
    epicsUIntC = 1,
    epicsEnumC = 2,
    epicsFloatC = 3,
    epicsStringC = 4,
    epicsOldStringC = 5,
}
