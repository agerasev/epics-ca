pub use crate::generated::cadef::*;

pub type ca_access_rights = u32;
#[cfg(target_endian = "big")]
pub const CA_READ_ACCESS: u32 = 1 << 31;
#[cfg(target_endian = "little")]
pub const CA_READ_ACCESS: u32 = 1 << 0;
#[cfg(target_endian = "big")]
pub const CA_READ_ACCESS: u32 = 1 << 30;
#[cfg(target_endian = "little")]
pub const CA_WRITE_ACCESS: u32 = 1 << 1;

pub unsafe fn ca_put(type_: chtype, chanId: chid, pValue: *const libc::c_void) -> libc::c_int {
    ca_array_put(type_, 1, chanId, pValue)
}

pub unsafe fn ca_put_callback(
    type_: chtype,
    chanId: chid,
    pValue: *const libc::c_void,
    pFunc: caEventCallBackFunc,
    pArg: *mut libc::c_void,
) -> libc::c_int {
    ca_array_put_callback(type_, 1, chanId, pValue, pFunc, pArg)
}

pub unsafe fn ca_get(type_: chtype, chanId: chid, pValue: *mut libc::c_void) -> libc::c_int {
    ca_array_get(type_, 1, chanId, pValue)
}

pub unsafe fn ca_get_callback(
    type_: chtype,
    chanId: chid,
    pFunc: caEventCallBackFunc,
    pArg: *mut libc::c_void,
) -> libc::c_int {
    ca_array_get_callback(type_, 1, chanId, pFunc, pArg)
}
