use core::ffi::*;

pub const CA_K_INFO: c_int = 3;
pub const CA_K_ERROR: c_int = 2;
pub const CA_K_SUCCESS: c_int = 1;
pub const CA_K_WARNING: c_int = 0;
pub const CA_K_SEVERE: c_int = 4;
pub const CA_K_FATAL: c_int = CA_K_ERROR | CA_K_SEVERE;

pub const CA_M_MSG_NO: c_int = 65528;
pub const CA_M_SEVERITY: c_int = 7;
pub const CA_M_LEVEL: c_int = 3;
pub const CA_M_SUCCESS: c_int = 1;
pub const CA_M_ERROR: c_int = 2;
pub const CA_M_SEVERE: c_int = 4;

pub const CA_S_MSG_NO: c_int = 13;
pub const CA_S_SEVERITY: c_int = 3;

pub const CA_V_MSG_NO: c_int = 3;
pub const CA_V_SEVERITY: c_int = 0;
pub const CA_V_SUCCESS: c_int = 0;

pub const fn CA_EXTRACT_MSG_NO(code: c_int) -> c_int {
    (code & CA_M_MSG_NO) >> CA_V_MSG_NO
}
pub const fn CA_EXTRACT_SEVERITY(code: c_int) -> c_int {
    (code & CA_M_SEVERITY) >> CA_V_SEVERITY
}
pub const fn CA_EXTRACT_SUCCESS(code: c_int) -> c_int {
    (code & CA_M_SUCCESS) >> CA_V_SUCCESS
}

pub const fn CA_INSERT_MSG_NO(code: c_int) -> c_int {
    (code << CA_V_MSG_NO) & CA_M_MSG_NO
}
pub const fn CA_INSERT_SEVERITY(code: c_int) -> c_int {
    (code << CA_V_SEVERITY) & CA_M_SEVERITY
}
pub const fn CA_INSERT_SUCCESS(code: c_int) -> c_int {
    (code << CA_V_SUCCESS) & CA_M_SUCCESS
}

const fn DEFMSG(SEVERITY: c_int, NUMBER: c_int) -> c_int {
    CA_INSERT_MSG_NO(NUMBER) | CA_INSERT_SEVERITY(SEVERITY)
}

pub const ECA_NORMAL: c_int = DEFMSG(CA_K_SUCCESS, 0);
pub const ECA_MAXIOC: c_int = DEFMSG(CA_K_ERROR, 1);
pub const ECA_UKNHOST: c_int = DEFMSG(CA_K_ERROR, 2);
pub const ECA_UKNSERV: c_int = DEFMSG(CA_K_ERROR, 3);
pub const ECA_SOCK: c_int = DEFMSG(CA_K_ERROR, 4);
pub const ECA_CONN: c_int = DEFMSG(CA_K_WARNING, 5);
pub const ECA_ALLOCMEM: c_int = DEFMSG(CA_K_WARNING, 6);
pub const ECA_UKNCHAN: c_int = DEFMSG(CA_K_WARNING, 7);
pub const ECA_UKNFIELD: c_int = DEFMSG(CA_K_WARNING, 8);
pub const ECA_TOLARGE: c_int = DEFMSG(CA_K_WARNING, 9);
pub const ECA_TIMEOUT: c_int = DEFMSG(CA_K_WARNING, 10);
pub const ECA_NOSUPPORT: c_int = DEFMSG(CA_K_WARNING, 11);
pub const ECA_STRTOBIG: c_int = DEFMSG(CA_K_WARNING, 12);
pub const ECA_DISCONNCHID: c_int = DEFMSG(CA_K_ERROR, 13);
pub const ECA_BADTYPE: c_int = DEFMSG(CA_K_ERROR, 14);
pub const ECA_CHIDNOTFND: c_int = DEFMSG(CA_K_INFO, 15);
pub const ECA_CHIDRETRY: c_int = DEFMSG(CA_K_INFO, 16);
pub const ECA_INTERNAL: c_int = DEFMSG(CA_K_FATAL, 17);
pub const ECA_DBLCLFAIL: c_int = DEFMSG(CA_K_WARNING, 18);
pub const ECA_GETFAIL: c_int = DEFMSG(CA_K_WARNING, 19);
pub const ECA_PUTFAIL: c_int = DEFMSG(CA_K_WARNING, 20);
pub const ECA_ADDFAIL: c_int = DEFMSG(CA_K_WARNING, 21);
pub const ECA_BADCOUNT: c_int = DEFMSG(CA_K_WARNING, 22);
pub const ECA_BADSTR: c_int = DEFMSG(CA_K_ERROR, 23);
pub const ECA_DISCONN: c_int = DEFMSG(CA_K_WARNING, 24);
pub const ECA_DBLCHNL: c_int = DEFMSG(CA_K_WARNING, 25);
pub const ECA_EVDISALLOW: c_int = DEFMSG(CA_K_ERROR, 26);
pub const ECA_BUILDGET: c_int = DEFMSG(CA_K_WARNING, 27);
pub const ECA_NEEDSFP: c_int = DEFMSG(CA_K_WARNING, 28);
pub const ECA_OVEVFAIL: c_int = DEFMSG(CA_K_WARNING, 29);
pub const ECA_BADMONID: c_int = DEFMSG(CA_K_ERROR, 30);
pub const ECA_NEWADDR: c_int = DEFMSG(CA_K_WARNING, 31);
pub const ECA_NEWCONN: c_int = DEFMSG(CA_K_INFO, 32);
pub const ECA_NOCACTX: c_int = DEFMSG(CA_K_WARNING, 33);
pub const ECA_DEFUNCT: c_int = DEFMSG(CA_K_FATAL, 34);
pub const ECA_EMPTYSTR: c_int = DEFMSG(CA_K_WARNING, 35);
pub const ECA_NOREPEATER: c_int = DEFMSG(CA_K_WARNING, 36);
pub const ECA_NOCHANMSG: c_int = DEFMSG(CA_K_WARNING, 37);
pub const ECA_DLCKREST: c_int = DEFMSG(CA_K_WARNING, 38);
pub const ECA_SERVBEHIND: c_int = DEFMSG(CA_K_WARNING, 39);
pub const ECA_NOCAST: c_int = DEFMSG(CA_K_WARNING, 40);
pub const ECA_BADMASK: c_int = DEFMSG(CA_K_ERROR, 41);
pub const ECA_IODONE: c_int = DEFMSG(CA_K_INFO, 42);
pub const ECA_IOINPROGRESS: c_int = DEFMSG(CA_K_INFO, 43);
pub const ECA_BADSYNCGRP: c_int = DEFMSG(CA_K_ERROR, 44);
pub const ECA_PUTCBINPROG: c_int = DEFMSG(CA_K_ERROR, 45);
pub const ECA_NORDACCESS: c_int = DEFMSG(CA_K_WARNING, 46);
pub const ECA_NOWTACCESS: c_int = DEFMSG(CA_K_WARNING, 47);
pub const ECA_ANACHRONISM: c_int = DEFMSG(CA_K_ERROR, 48);
pub const ECA_NOSEARCHADDR: c_int = DEFMSG(CA_K_WARNING, 49);
pub const ECA_NOCONVERT: c_int = DEFMSG(CA_K_WARNING, 50);
pub const ECA_BADCHID: c_int = DEFMSG(CA_K_ERROR, 51);
pub const ECA_BADFUNCPTR: c_int = DEFMSG(CA_K_ERROR, 52);
pub const ECA_ISATTACHED: c_int = DEFMSG(CA_K_WARNING, 53);
pub const ECA_UNAVAILINSERV: c_int = DEFMSG(CA_K_WARNING, 54);
pub const ECA_CHANDESTROY: c_int = DEFMSG(CA_K_WARNING, 55);
pub const ECA_BADPRIORITY: c_int = DEFMSG(CA_K_ERROR, 56);
pub const ECA_NOTTHREADED: c_int = DEFMSG(CA_K_ERROR, 57);
pub const ECA_16KARRAYCLIENT: c_int = DEFMSG(CA_K_WARNING, 58);
pub const ECA_CONNSEQTMO: c_int = DEFMSG(CA_K_WARNING, 59);
pub const ECA_UNRESPTMO: c_int = DEFMSG(CA_K_WARNING, 60);

extern "C" {
    pub fn ca_message(ca_status: c_long) -> *const c_char;
}
