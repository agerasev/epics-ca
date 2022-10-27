#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    clippy::missing_safety_doc
)]

use core::ffi::*;

pub const CA_K_INFO: u32 = 3;
pub const CA_K_ERROR: u32 = 2;
pub const CA_K_SUCCESS: u32 = 1;
pub const CA_K_WARNING: u32 = 0;
pub const CA_K_SEVERE: u32 = 4;
pub const CA_K_FATAL: u32 = 6;

pub const CA_M_MSG_NO: u32 = 65528;
pub const CA_M_SEVERITY: u32 = 7;
pub const CA_M_LEVEL: u32 = 3;
pub const CA_M_SUCCESS: u32 = 1;
pub const CA_M_ERROR: u32 = 2;
pub const CA_M_SEVERE: u32 = 4;

pub const CA_S_MSG_NO: u32 = 13;
pub const CA_S_SEVERITY: u32 = 3;

pub const CA_V_MSG_NO: u32 = 3;
pub const CA_V_SEVERITY: u32 = 0;
pub const CA_V_SUCCESS: u32 = 0;

pub const epicsTimeOK: u32 = 0;

pub const epicsTimeEventCurrentTime: u32 = 0;
pub const epicsTimeEventBestTime: i32 = -1;
pub const epicsTimeEventDeviceTime: i32 = -2;

pub const MAX_UNITS_SIZE: u32 = 8;
pub const MAX_ENUM_STRING_SIZE: u32 = 26;
pub const MAX_ENUM_STATES: u32 = 16;

pub const DBF_STRING: c_long = 0;
pub const DBF_INT: c_long = 1;
pub const DBF_SHORT: c_long = 1;
pub const DBF_FLOAT: c_long = 2;
pub const DBF_ENUM: c_long = 3;
pub const DBF_CHAR: c_long = 4;
pub const DBF_LONG: c_long = 5;
pub const DBF_DOUBLE: c_long = 6;
pub const DBF_NO_ACCESS: c_long = 7;

pub const DBR_STRING: c_long = 0;
pub const DBR_INT: c_long = 1;
pub const DBR_SHORT: c_long = 1;
pub const DBR_FLOAT: c_long = 2;
pub const DBR_ENUM: c_long = 3;
pub const DBR_CHAR: c_long = 4;
pub const DBR_LONG: c_long = 5;
pub const DBR_DOUBLE: c_long = 6;
pub const DBR_STS_STRING: c_long = 7;
pub const DBR_STS_SHORT: c_long = 8;
pub const DBR_STS_INT: c_long = 8;
pub const DBR_STS_FLOAT: c_long = 9;
pub const DBR_STS_ENUM: c_long = 10;
pub const DBR_STS_CHAR: c_long = 11;
pub const DBR_STS_LONG: c_long = 12;
pub const DBR_STS_DOUBLE: c_long = 13;
pub const DBR_TIME_STRING: c_long = 14;
pub const DBR_TIME_INT: c_long = 15;
pub const DBR_TIME_SHORT: c_long = 15;
pub const DBR_TIME_FLOAT: c_long = 16;
pub const DBR_TIME_ENUM: c_long = 17;
pub const DBR_TIME_CHAR: c_long = 18;
pub const DBR_TIME_LONG: c_long = 19;
pub const DBR_TIME_DOUBLE: c_long = 20;
pub const DBR_GR_STRING: c_long = 21;
pub const DBR_GR_SHORT: c_long = 22;
pub const DBR_GR_INT: c_long = 22;
pub const DBR_GR_FLOAT: c_long = 23;
pub const DBR_GR_ENUM: c_long = 24;
pub const DBR_GR_CHAR: c_long = 25;
pub const DBR_GR_LONG: c_long = 26;
pub const DBR_GR_DOUBLE: c_long = 27;
pub const DBR_CTRL_STRING: c_long = 28;
pub const DBR_CTRL_SHORT: c_long = 29;
pub const DBR_CTRL_INT: c_long = 29;
pub const DBR_CTRL_FLOAT: c_long = 30;
pub const DBR_CTRL_ENUM: c_long = 31;
pub const DBR_CTRL_CHAR: c_long = 32;
pub const DBR_CTRL_LONG: c_long = 33;
pub const DBR_CTRL_DOUBLE: c_long = 34;
pub const DBR_PUT_ACKT: c_long = 35;
pub const DBR_PUT_ACKS: c_long = 36;
pub const DBR_STSACK_STRING: c_long = 37;
pub const DBR_CLASS_NAME: c_long = 38;

pub const DBE_VALUE: u32 = 1;
pub const DBE_ARCHIVE: u32 = 2;
pub const DBE_LOG: u32 = 2;
pub const DBE_ALARM: u32 = 4;
pub const DBE_PROPERTY: u32 = 8;

pub const CA_OP_GET: u32 = 0;
pub const CA_OP_PUT: u32 = 1;
pub const CA_OP_CREATE_CHANNEL: u32 = 2;
pub const CA_OP_ADD_EVENT: u32 = 3;
pub const CA_OP_CLEAR_EVENT: u32 = 4;
pub const CA_OP_OTHER: u32 = 5;
pub const CA_OP_CONN_UP: u32 = 6;
pub const CA_OP_CONN_DOWN: u32 = 7;
pub const CA_OP_SEARCH: u32 = 2;

pub const CA_PRIORITY_MAX: u32 = 99;
pub const CA_PRIORITY_MIN: u32 = 0;
pub const CA_PRIORITY_DEFAULT: u32 = 0;
pub const CA_PRIORITY_DB_LINKS: u32 = 80;
pub const CA_PRIORITY_ARCHIVE: u32 = 20;
pub const CA_PRIORITY_OPI: u32 = 0;

pub type epicsThreadId = *mut c_void;

extern "C" {
    pub fn ca_message(ca_status: c_long) -> *const c_char;
}

pub type chid = *mut c_void;
pub type chanId = chid;
pub type chtype = c_long;
pub type evid = *mut c_void;
pub type ca_real = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct connection_handler_args {
    pub chid: chanId,
    pub op: c_long,
}
pub type caCh = Option<unsafe extern "C" fn(args: connection_handler_args)>;

type ca_access_rights = c_uint;
#[cfg(target_endian = "big")]
pub const CA_READ_ACCESS: c_uint = 1 << 31;
#[cfg(target_endian = "little")]
pub const CA_READ_ACCESS: c_uint = 1 << 0;
#[cfg(target_endian = "big")]
pub const CA_READ_ACCESS: c_uint = 1 << 30;
#[cfg(target_endian = "little")]
pub const CA_WRITE_ACCESS: c_uint = 1 << 1;
pub type caar = ca_access_rights;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct access_rights_handler_args {
    pub chid: chanId,
    pub ar: caar,
}
pub type caArh = Option<unsafe extern "C" fn(args: access_rights_handler_args)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct event_handler_args {
    pub usr: *mut c_void,
    pub chid: chanId,
    pub type_: c_long,
    pub count: c_long,
    pub dbr: *const c_void,
    pub status: c_int,
}
pub type evargs = event_handler_args;
pub type caEventCallBackFunc = Option<unsafe extern "C" fn(arg1: event_handler_args)>;
extern "C" {
    pub fn ca_test_event(arg1: event_handler_args);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct exception_handler_args {
    pub usr: *mut c_void,
    pub chid: chanId,
    pub type_: c_long,
    pub count: c_long,
    pub addr: *mut c_void,
    pub stat: c_long,
    pub op: c_long,
    pub ctx: *const c_char,
    pub pFile: *const c_char,
    pub lineNo: c_uint,
}
pub type CA_SYNC_GID = c_uint;
extern "C" {
    pub fn ca_field_type(chan: chid) -> c_short;

    pub fn ca_element_count(chan: chid) -> c_ulong;

    pub fn ca_name(chan: chid) -> *const c_char;

    pub fn ca_set_puser(chan: chid, puser: *mut c_void);

    pub fn ca_puser(chan: chid) -> *mut c_void;

    pub fn ca_read_access(chan: chid) -> c_uint;

    pub fn ca_write_access(chan: chid) -> c_uint;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum channel_state {
    cs_never_conn = 0,
    cs_prev_conn = 1,
    cs_conn = 2,
    cs_closed = 3,
}
extern "C" {
    pub fn ca_state(chan: chid) -> channel_state;

    pub fn ca_task_initialize() -> c_int;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ca_preemptive_callback_select {
    ca_disable_preemptive_callback = 0,
    ca_enable_preemptive_callback = 1,
}
extern "C" {
    pub fn ca_context_create(select: ca_preemptive_callback_select) -> c_int;

    pub fn ca_detach_context();

    pub fn ca_task_exit() -> c_int;

    pub fn ca_context_destroy();
}
pub type capri = c_uint;
extern "C" {
    pub fn ca_create_channel(
        pChanName: *const c_char,
        pConnStateCallback: caCh,
        pUserPrivate: *mut c_void,
        priority: capri,
        pChanID: *mut chid,
    ) -> c_int;

    pub fn ca_change_connection_event(chan: chid, pfunc: caCh) -> c_int;

    pub fn ca_replace_access_rights_event(chan: chid, pfunc: caArh) -> c_int;
}
pub type caExceptionHandler = Option<unsafe extern "C" fn(arg1: exception_handler_args)>;
extern "C" {
    pub fn ca_add_exception_event(pfunc: caExceptionHandler, pArg: *mut c_void) -> c_int;

    pub fn ca_clear_channel(chanId: chid) -> c_int;

    pub fn ca_array_put(
        type_: chtype,
        count: c_ulong,
        chanId: chid,
        pValue: *const c_void,
    ) -> c_int;

    pub fn ca_array_put_callback(
        type_: chtype,
        count: c_ulong,
        chanId: chid,
        pValue: *const c_void,
        pFunc: caEventCallBackFunc,
        pArg: *mut c_void,
    ) -> c_int;

    pub fn ca_array_get(type_: chtype, count: c_ulong, chanId: chid, pValue: *mut c_void) -> c_int;

    pub fn ca_array_get_callback(
        type_: chtype,
        count: c_ulong,
        chanId: chid,
        pFunc: caEventCallBackFunc,
        pArg: *mut c_void,
    ) -> c_int;

    pub fn ca_create_subscription(
        type_: chtype,
        count: c_ulong,
        chanId: chid,
        mask: c_long,
        pFunc: caEventCallBackFunc,
        pArg: *mut c_void,
        pEventID: *mut evid,
    ) -> c_int;

    pub fn ca_clear_subscription(eventID: evid) -> c_int;

    pub fn ca_evid_to_chid(id: evid) -> chid;

    pub fn ca_pend_event(timeOut: ca_real) -> c_int;

    pub fn ca_pend_io(timeOut: ca_real) -> c_int;

    pub fn ca_pend(timeout: ca_real, early: c_int) -> c_int;

    pub fn ca_test_io() -> c_int;

    pub fn ca_flush_io() -> c_int;

    pub fn ca_signal(errorCode: c_long, pCtxStr: *const c_char);

    pub fn ca_signal_with_file_and_lineno(
        errorCode: c_long,
        pCtxStr: *const c_char,
        pFileStr: *const c_char,
        lineNo: c_int,
    );

    pub fn ca_signal_formated(
        ca_status: c_long,
        pfilenm: *const c_char,
        lineno: c_int,
        pFormat: *const c_char,
        ...
    );

    pub fn ca_host_name(channel: chid) -> *const c_char;

    pub fn ca_get_host_name(pChan: chid, pBuf: *mut c_char, bufLength: c_uint) -> c_uint;
}
pub type CAFDHANDLER = Option<unsafe extern "C" fn(parg: *mut c_void, fd: c_int, opened: c_int)>;
extern "C" {
    pub fn ca_add_fd_registration(pHandler: CAFDHANDLER, pArg: *mut c_void) -> c_int;

    pub fn ca_sg_create(pgid: *mut CA_SYNC_GID) -> c_int;

    pub fn ca_sg_delete(gid: CA_SYNC_GID) -> c_int;

    pub fn ca_sg_block(gid: CA_SYNC_GID, timeout: ca_real) -> c_int;

    pub fn ca_sg_test(gid: CA_SYNC_GID) -> c_int;

    pub fn ca_sg_reset(gid: CA_SYNC_GID) -> c_int;

    pub fn ca_sg_array_get(
        gid: CA_SYNC_GID,
        type_: chtype,
        count: c_ulong,
        chan: chid,
        pValue: *mut c_void,
    ) -> c_int;

    pub fn ca_sg_array_put(
        gid: CA_SYNC_GID,
        type_: chtype,
        count: c_ulong,
        chan: chid,
        pValue: *const c_void,
    ) -> c_int;

    pub fn ca_sg_stat(gid: CA_SYNC_GID) -> c_int;

    pub fn ca_dump_dbr(type_: chtype, count: c_uint, pbuffer: *const c_void);

    pub fn ca_v42_ok(chan: chid) -> c_int;

    pub fn ca_version() -> *const c_char;

    pub fn ca_replace_printf_handler(ca_printf_func: *mut c_void) -> c_int;

    pub fn ca_get_ioc_connection_count() -> c_uint;

    pub fn ca_preemtive_callback_is_enabled() -> c_int;

    pub fn ca_self_test();

    pub fn ca_beacon_anomaly_count() -> c_uint;

    pub fn ca_search_attempts(chan: chid) -> c_uint;

    pub fn ca_beacon_period(chan: chid) -> f64;

    pub fn ca_receive_watchdog_delay(chan: chid) -> f64;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ca_client_context {
    _unused: [u8; 0],
}
extern "C" {
    pub fn ca_current_context() -> *mut ca_client_context;

    pub fn ca_attach_context(context: *mut ca_client_context) -> c_int;

    pub fn ca_client_status(level: c_uint) -> c_int;

    pub fn ca_context_status(arg1: *mut ca_client_context, level: c_uint) -> c_int;

    pub fn ca_build_and_connect(
        pChanName: *const c_char,
        arg1: chtype,
        arg2: c_ulong,
        pChanID: *mut chid,
        arg3: *mut c_void,
        pFunc: caCh,
        pArg: *mut c_void,
    ) -> c_int;

    pub fn ca_search_and_connect(
        pChanName: *const c_char,
        pChanID: *mut chid,
        pFunc: caCh,
        pArg: *mut c_void,
    ) -> c_int;

    pub fn ca_channel_status(tid: epicsThreadId) -> c_int;

    pub fn ca_clear_event(eventID: evid) -> c_int;

    pub fn ca_add_masked_array_event(
        type_: chtype,
        count: c_ulong,
        chanId: chid,
        pFunc: caEventCallBackFunc,
        pArg: *mut c_void,
        p_delta: ca_real,
        n_delta: ca_real,
        timeout: ca_real,
        pEventID: *mut evid,
        mask: c_long,
    ) -> c_int;

    pub fn ca_modify_user_name(pUserName: *const c_char) -> c_int;

    pub fn ca_modify_host_name(pHostName: *const c_char) -> c_int;
}

pub unsafe fn ca_put(type_: chtype, chanId: chid, pValue: *const c_void) -> c_int {
    ca_array_put(type_, 1, chanId, pValue)
}

pub unsafe fn ca_put_callback(
    type_: chtype,
    chanId: chid,
    pValue: *const c_void,
    pFunc: caEventCallBackFunc,
    pArg: *mut c_void,
) -> c_int {
    ca_array_put_callback(type_, 1, chanId, pValue, pFunc, pArg)
}

pub unsafe fn ca_get(type_: chtype, chanId: chid, pValue: *mut c_void) -> c_int {
    ca_array_get(type_, 1, chanId, pValue)
}

pub unsafe fn ca_get_callback(
    type_: chtype,
    chanId: chid,
    pFunc: caEventCallBackFunc,
    pArg: *mut c_void,
) -> c_int {
    ca_array_get_callback(type_, 1, chanId, pFunc, pArg)
}
