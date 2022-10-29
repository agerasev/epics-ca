use core::ffi::*;

use crate::sys;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ErrorKind {
    Allocmem,
    Tolarge,
    Timeout,
    Badtype,
    Internal,
    Getfail,
    Putfail,
    Badcount,
    Badstr,
    Disconn,
    Dblchnl,
    Evdisallow,
    Badmonid,
    Badmask,
    Badsyncgrp,
    Putcbinprog,
    Nordaccess,
    Nowtaccess,
    Anachronism,
    Nosearchaddr,
    Noconvert,
    Badchid,
    Badfuncptr,
    Isattached,
    Unavailinserv,
    Chandestroy,
    Badpriority,
    Notthreaded,
    N16karrayclient,
    Connseqtmo,
    Unresptmo,
    Other(c_int),
}

impl ErrorKind {
    const ECA_MAP: [(Self, c_int); 31] = [
        (Self::Allocmem, sys::err::ECA_ALLOCMEM),
        (Self::Tolarge, sys::err::ECA_TOLARGE),
        (Self::Timeout, sys::err::ECA_TIMEOUT),
        (Self::Badtype, sys::err::ECA_BADTYPE),
        (Self::Internal, sys::err::ECA_INTERNAL),
        (Self::Getfail, sys::err::ECA_GETFAIL),
        (Self::Putfail, sys::err::ECA_PUTFAIL),
        (Self::Badcount, sys::err::ECA_BADCOUNT),
        (Self::Badstr, sys::err::ECA_BADSTR),
        (Self::Disconn, sys::err::ECA_DISCONN),
        (Self::Dblchnl, sys::err::ECA_DBLCHNL),
        (Self::Evdisallow, sys::err::ECA_EVDISALLOW),
        (Self::Badmonid, sys::err::ECA_BADMONID),
        (Self::Badmask, sys::err::ECA_BADMASK),
        (Self::Badsyncgrp, sys::err::ECA_BADSYNCGRP),
        (Self::Putcbinprog, sys::err::ECA_PUTCBINPROG),
        (Self::Nordaccess, sys::err::ECA_NORDACCESS),
        (Self::Nowtaccess, sys::err::ECA_NOWTACCESS),
        (Self::Anachronism, sys::err::ECA_ANACHRONISM),
        (Self::Nosearchaddr, sys::err::ECA_NOSEARCHADDR),
        (Self::Noconvert, sys::err::ECA_NOCONVERT),
        (Self::Badchid, sys::err::ECA_BADCHID),
        (Self::Badfuncptr, sys::err::ECA_BADFUNCPTR),
        (Self::Isattached, sys::err::ECA_ISATTACHED),
        (Self::Unavailinserv, sys::err::ECA_UNAVAILINSERV),
        (Self::Chandestroy, sys::err::ECA_CHANDESTROY),
        (Self::Badpriority, sys::err::ECA_BADPRIORITY),
        (Self::Notthreaded, sys::err::ECA_NOTTHREADED),
        (Self::N16karrayclient, sys::err::ECA_16KARRAYCLIENT),
        (Self::Connseqtmo, sys::err::ECA_CONNSEQTMO),
        (Self::Unresptmo, sys::err::ECA_UNRESPTMO),
    ];

    pub fn from_raw_msg_no(msg_no: c_int) -> Self {
        for (this, eca) in Self::ECA_MAP {
            if msg_no == sys::err::CA_EXTRACT_MSG_NO(eca) {
                return this;
            }
        }
        Self::Other(msg_no)
    }

    pub fn to_raw_msg_no(self) -> c_int {
        if let Self::Other(msg_no) = self {
            return msg_no;
        } else {
            for (this, eca) in Self::ECA_MAP {
                if self == this {
                    return sys::err::CA_EXTRACT_MSG_NO(eca);
                }
            }
        }
        unreachable!();
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Severe,
    Fatal,
}

impl ErrorSeverity {
    pub fn from_raw_severity(severity: c_int) -> Self {
        match severity {
            sys::err::CA_K_ERROR => Self::Error,
            sys::err::CA_K_WARNING => Self::Warning,
            sys::err::CA_K_SEVERE => Self::Severe,
            sys::err::CA_K_FATAL => Self::Fatal,
            _ => panic!("Unsupported severity: {}", severity),
        }
    }

    pub fn to_raw_severity(self) -> c_int {
        match self {
            Self::Error => sys::err::CA_K_ERROR,
            Self::Warning => sys::err::CA_K_WARNING,
            Self::Severe => sys::err::CA_K_SEVERE,
            Self::Fatal => sys::err::CA_K_FATAL,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Error {
    pub kind: ErrorKind,
    pub severity: ErrorSeverity,
}

impl Error {
    pub fn try_from_raw(eca: c_int) -> Result<(), Self> {
        if sys::err::CA_EXTRACT_SUCCESS(eca) != 0 {
            Ok(())
        } else {
            Err(Error {
                kind: ErrorKind::from_raw_msg_no(sys::err::CA_EXTRACT_MSG_NO(eca)),
                severity: ErrorSeverity::from_raw_severity(sys::err::CA_EXTRACT_SEVERITY(eca)),
            })
        }
    }

    pub fn into_raw(self) -> c_int {
        sys::err::CA_INSERT_MSG_NO(self.kind.to_raw_msg_no())
            | sys::err::CA_INSERT_SEVERITY(self.severity.to_raw_severity())
    }
}
