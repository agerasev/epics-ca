#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    clippy::missing_safety_doc
)]

mod generated;
pub use generated::*;

#[cfg(all(test, feature = "test"))]
mod test;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct epicsThreadOSD {
    _unused: [u8; 0],
}

pub const fn CA_EXTRACT_MSG_NO(code: i32) -> i32 {
    (code & CA_M_MSG_NO) >> CA_V_MSG_NO
}
pub const fn CA_EXTRACT_SEVERITY(code: i32) -> i32 {
    (code & CA_M_SEVERITY) >> CA_V_SEVERITY
}
pub const fn CA_EXTRACT_SUCCESS(code: i32) -> i32 {
    (code & CA_M_SUCCESS) >> CA_V_SUCCESS
}
pub const fn CA_INSERT_MSG_NO(code: i32) -> i32 {
    (code << CA_V_MSG_NO) & CA_M_MSG_NO
}
pub const fn CA_INSERT_SEVERITY(code: i32) -> i32 {
    (code << CA_V_SEVERITY) & CA_M_SEVERITY
}
pub const fn CA_INSERT_SUCCESS(code: i32) -> i32 {
    (code << CA_V_SUCCESS) & CA_M_SUCCESS
}
const fn DEFMSG(SEVERITY: i32, NUMBER: i32) -> i32 {
    CA_INSERT_MSG_NO(NUMBER) | CA_INSERT_SEVERITY(SEVERITY)
}

pub const ECA_NORMAL: i32 = DEFMSG(CA_K_SUCCESS, 0);
pub const ECA_MAXIOC: i32 = DEFMSG(CA_K_ERROR, 1);
pub const ECA_UKNHOST: i32 = DEFMSG(CA_K_ERROR, 2);
pub const ECA_UKNSERV: i32 = DEFMSG(CA_K_ERROR, 3);
pub const ECA_SOCK: i32 = DEFMSG(CA_K_ERROR, 4);
pub const ECA_CONN: i32 = DEFMSG(CA_K_WARNING, 5);
pub const ECA_ALLOCMEM: i32 = DEFMSG(CA_K_WARNING, 6);
pub const ECA_UKNCHAN: i32 = DEFMSG(CA_K_WARNING, 7);
pub const ECA_UKNFIELD: i32 = DEFMSG(CA_K_WARNING, 8);
pub const ECA_TOLARGE: i32 = DEFMSG(CA_K_WARNING, 9);
pub const ECA_TIMEOUT: i32 = DEFMSG(CA_K_WARNING, 10);
pub const ECA_NOSUPPORT: i32 = DEFMSG(CA_K_WARNING, 11);
pub const ECA_STRTOBIG: i32 = DEFMSG(CA_K_WARNING, 12);
pub const ECA_DISCONNCHID: i32 = DEFMSG(CA_K_ERROR, 13);
pub const ECA_BADTYPE: i32 = DEFMSG(CA_K_ERROR, 14);
pub const ECA_CHIDNOTFND: i32 = DEFMSG(CA_K_INFO, 15);
pub const ECA_CHIDRETRY: i32 = DEFMSG(CA_K_INFO, 16);
pub const ECA_INTERNAL: i32 = DEFMSG(CA_K_FATAL, 17);
pub const ECA_DBLCLFAIL: i32 = DEFMSG(CA_K_WARNING, 18);
pub const ECA_GETFAIL: i32 = DEFMSG(CA_K_WARNING, 19);
pub const ECA_PUTFAIL: i32 = DEFMSG(CA_K_WARNING, 20);
pub const ECA_ADDFAIL: i32 = DEFMSG(CA_K_WARNING, 21);
pub const ECA_BADCOUNT: i32 = DEFMSG(CA_K_WARNING, 22);
pub const ECA_BADSTR: i32 = DEFMSG(CA_K_ERROR, 23);
pub const ECA_DISCONN: i32 = DEFMSG(CA_K_WARNING, 24);
pub const ECA_DBLCHNL: i32 = DEFMSG(CA_K_WARNING, 25);
pub const ECA_EVDISALLOW: i32 = DEFMSG(CA_K_ERROR, 26);
pub const ECA_BUILDGET: i32 = DEFMSG(CA_K_WARNING, 27);
pub const ECA_NEEDSFP: i32 = DEFMSG(CA_K_WARNING, 28);
pub const ECA_OVEVFAIL: i32 = DEFMSG(CA_K_WARNING, 29);
pub const ECA_BADMONID: i32 = DEFMSG(CA_K_ERROR, 30);
pub const ECA_NEWADDR: i32 = DEFMSG(CA_K_WARNING, 31);
pub const ECA_NEWCONN: i32 = DEFMSG(CA_K_INFO, 32);
pub const ECA_NOCACTX: i32 = DEFMSG(CA_K_WARNING, 33);
pub const ECA_DEFUNCT: i32 = DEFMSG(CA_K_FATAL, 34);
pub const ECA_EMPTYSTR: i32 = DEFMSG(CA_K_WARNING, 35);
pub const ECA_NOREPEATER: i32 = DEFMSG(CA_K_WARNING, 36);
pub const ECA_NOCHANMSG: i32 = DEFMSG(CA_K_WARNING, 37);
pub const ECA_DLCKREST: i32 = DEFMSG(CA_K_WARNING, 38);
pub const ECA_SERVBEHIND: i32 = DEFMSG(CA_K_WARNING, 39);
pub const ECA_NOCAST: i32 = DEFMSG(CA_K_WARNING, 40);
pub const ECA_BADMASK: i32 = DEFMSG(CA_K_ERROR, 41);
pub const ECA_IODONE: i32 = DEFMSG(CA_K_INFO, 42);
pub const ECA_IOINPROGRESS: i32 = DEFMSG(CA_K_INFO, 43);
pub const ECA_BADSYNCGRP: i32 = DEFMSG(CA_K_ERROR, 44);
pub const ECA_PUTCBINPROG: i32 = DEFMSG(CA_K_ERROR, 45);
pub const ECA_NORDACCESS: i32 = DEFMSG(CA_K_WARNING, 46);
pub const ECA_NOWTACCESS: i32 = DEFMSG(CA_K_WARNING, 47);
pub const ECA_ANACHRONISM: i32 = DEFMSG(CA_K_ERROR, 48);
pub const ECA_NOSEARCHADDR: i32 = DEFMSG(CA_K_WARNING, 49);
pub const ECA_NOCONVERT: i32 = DEFMSG(CA_K_WARNING, 50);
pub const ECA_BADCHID: i32 = DEFMSG(CA_K_ERROR, 51);
pub const ECA_BADFUNCPTR: i32 = DEFMSG(CA_K_ERROR, 52);
pub const ECA_ISATTACHED: i32 = DEFMSG(CA_K_WARNING, 53);
pub const ECA_UNAVAILINSERV: i32 = DEFMSG(CA_K_WARNING, 54);
pub const ECA_CHANDESTROY: i32 = DEFMSG(CA_K_WARNING, 55);
pub const ECA_BADPRIORITY: i32 = DEFMSG(CA_K_ERROR, 56);
pub const ECA_NOTTHREADED: i32 = DEFMSG(CA_K_ERROR, 57);
pub const ECA_16KARRAYCLIENT: i32 = DEFMSG(CA_K_WARNING, 58);
pub const ECA_CONNSEQTMO: i32 = DEFMSG(CA_K_WARNING, 59);
pub const ECA_UNRESPTMO: i32 = DEFMSG(CA_K_WARNING, 60);

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
