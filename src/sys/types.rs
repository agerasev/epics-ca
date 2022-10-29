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

pub const MAX_UNITS_SIZE: usize = 8;
pub const MAX_ENUM_STRING_SIZE: usize = 26;
pub const MAX_ENUM_STATES: usize = 16;

pub const DBF_STRING: c_ulong = 0;
pub const DBF_INT: c_ulong = 1;
pub const DBF_SHORT: c_ulong = 1;
pub const DBF_FLOAT: c_ulong = 2;
pub const DBF_ENUM: c_ulong = 3;
pub const DBF_CHAR: c_ulong = 4;
pub const DBF_LONG: c_ulong = 5;
pub const DBF_DOUBLE: c_ulong = 6;
pub const DBF_NO_ACCESS: c_ulong = 7;

pub const DBR_STRING: c_ulong = DBF_STRING;
pub const DBR_INT: c_ulong = DBF_INT;
pub const DBR_SHORT: c_ulong = DBF_SHORT;
pub const DBR_FLOAT: c_ulong = DBF_FLOAT;
pub const DBR_ENUM: c_ulong = DBF_ENUM;
pub const DBR_CHAR: c_ulong = DBF_CHAR;
pub const DBR_LONG: c_ulong = DBF_LONG;
pub const DBR_DOUBLE: c_ulong = DBF_DOUBLE;
pub const DBR_STS_STRING: c_ulong = 7;
pub const DBR_STS_SHORT: c_ulong = 8;
pub const DBR_STS_INT: c_ulong = 8;
pub const DBR_STS_FLOAT: c_ulong = 9;
pub const DBR_STS_ENUM: c_ulong = 10;
pub const DBR_STS_CHAR: c_ulong = 11;
pub const DBR_STS_LONG: c_ulong = 12;
pub const DBR_STS_DOUBLE: c_ulong = 13;
pub const DBR_TIME_STRING: c_ulong = 14;
pub const DBR_TIME_INT: c_ulong = 15;
pub const DBR_TIME_SHORT: c_ulong = 15;
pub const DBR_TIME_FLOAT: c_ulong = 16;
pub const DBR_TIME_ENUM: c_ulong = 17;
pub const DBR_TIME_CHAR: c_ulong = 18;
pub const DBR_TIME_LONG: c_ulong = 19;
pub const DBR_TIME_DOUBLE: c_ulong = 20;
pub const DBR_GR_STRING: c_ulong = 21;
pub const DBR_GR_SHORT: c_ulong = 22;
pub const DBR_GR_INT: c_ulong = 22;
pub const DBR_GR_FLOAT: c_ulong = 23;
pub const DBR_GR_ENUM: c_ulong = 24;
pub const DBR_GR_CHAR: c_ulong = 25;
pub const DBR_GR_LONG: c_ulong = 26;
pub const DBR_GR_DOUBLE: c_ulong = 27;
pub const DBR_CTRL_STRING: c_ulong = 28;
pub const DBR_CTRL_SHORT: c_ulong = 29;
pub const DBR_CTRL_INT: c_ulong = 29;
pub const DBR_CTRL_FLOAT: c_ulong = 30;
pub const DBR_CTRL_ENUM: c_ulong = 31;
pub const DBR_CTRL_CHAR: c_ulong = 32;
pub const DBR_CTRL_LONG: c_ulong = 33;
pub const DBR_CTRL_DOUBLE: c_ulong = 34;
pub const DBR_PUT_ACKT: c_ulong = 35;
pub const DBR_PUT_ACKS: c_ulong = 36;
pub const DBR_STSACK_STRING: c_ulong = 37;
pub const DBR_CLASS_NAME: c_ulong = 38;

extern "C" {
    pub static dbr_size: [c_ushort; 0];
    pub static dbr_value_size: [c_ushort; 0];
}

unsafe fn get_dbr_size(dbr_type: c_ulong) -> usize {
    dbr_size.as_ptr().offset(dbr_type as isize).read() as usize
}

unsafe fn get_dbr_value_size(dbr_type: c_ulong) -> usize {
    dbr_value_size.as_ptr().offset(dbr_type as isize).read() as usize
}

pub unsafe fn dbr_size_n(dbr_type: c_ulong, count: usize) -> usize {
    if count == 0 {
        get_dbr_size(dbr_type) as usize - get_dbr_value_size(dbr_type) as usize
    } else {
        get_dbr_size(dbr_type) as usize + (count - 1) * get_dbr_value_size(dbr_type) as usize
    }
}

extern "C" {
    pub static mut dbf_text: [*const c_char; 9];
    pub static dbf_text_dim: c_short;
    pub static mut dbf_text_invalid: *const c_char;

    pub static mut dbr_text: [*const c_char; 39];
    pub static dbr_text_dim: c_short;
    pub static mut dbr_text_invalid: *const c_char;
}

pub unsafe fn dbf_type_to_text(type_: c_ulong) -> *const c_char {
    if type_ < dbf_text_dim as c_ulong - 2 {
        dbf_text[type_ as usize + 1]
    } else {
        dbf_text_invalid
    }
}
pub unsafe fn dbr_type_to_text(type_: c_ulong) -> *const c_char {
    if type_ < dbr_text_dim as c_ulong - 2 {
        dbr_text[type_ as usize + 1]
    } else {
        dbr_text_invalid
    }
}
