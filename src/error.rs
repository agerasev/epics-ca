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
        (Self::Allocmem, sys::ECA_ALLOCMEM),
        (Self::Tolarge, sys::ECA_TOLARGE),
        (Self::Timeout, sys::ECA_TIMEOUT),
        (Self::Badtype, sys::ECA_BADTYPE),
        (Self::Internal, sys::ECA_INTERNAL),
        (Self::Getfail, sys::ECA_GETFAIL),
        (Self::Putfail, sys::ECA_PUTFAIL),
        (Self::Badcount, sys::ECA_BADCOUNT),
        (Self::Badstr, sys::ECA_BADSTR),
        (Self::Disconn, sys::ECA_DISCONN),
        (Self::Dblchnl, sys::ECA_DBLCHNL),
        (Self::Evdisallow, sys::ECA_EVDISALLOW),
        (Self::Badmonid, sys::ECA_BADMONID),
        (Self::Badmask, sys::ECA_BADMASK),
        (Self::Badsyncgrp, sys::ECA_BADSYNCGRP),
        (Self::Putcbinprog, sys::ECA_PUTCBINPROG),
        (Self::Nordaccess, sys::ECA_NORDACCESS),
        (Self::Nowtaccess, sys::ECA_NOWTACCESS),
        (Self::Anachronism, sys::ECA_ANACHRONISM),
        (Self::Nosearchaddr, sys::ECA_NOSEARCHADDR),
        (Self::Noconvert, sys::ECA_NOCONVERT),
        (Self::Badchid, sys::ECA_BADCHID),
        (Self::Badfuncptr, sys::ECA_BADFUNCPTR),
        (Self::Isattached, sys::ECA_ISATTACHED),
        (Self::Unavailinserv, sys::ECA_UNAVAILINSERV),
        (Self::Chandestroy, sys::ECA_CHANDESTROY),
        (Self::Badpriority, sys::ECA_BADPRIORITY),
        (Self::Notthreaded, sys::ECA_NOTTHREADED),
        (Self::N16karrayclient, sys::ECA_16KARRAYCLIENT),
        (Self::Connseqtmo, sys::ECA_CONNSEQTMO),
        (Self::Unresptmo, sys::ECA_UNRESPTMO),
    ];

    pub fn from_raw_msg_no(msg_no: c_int) -> Self {
        for (this, eca) in Self::ECA_MAP {
            if msg_no == sys::CA_EXTRACT_MSG_NO(eca) {
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
                    return sys::CA_EXTRACT_MSG_NO(eca);
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
            sys::CA_K_ERROR => Self::Error,
            sys::CA_K_WARNING => Self::Warning,
            sys::CA_K_SEVERE => Self::Severe,
            sys::CA_K_FATAL => Self::Fatal,
            _ => panic!("Unsupported severity: {}", severity),
        }
    }

    pub fn to_raw_severity(self) -> c_int {
        match self {
            Self::Error => sys::CA_K_ERROR,
            Self::Warning => sys::CA_K_WARNING,
            Self::Severe => sys::CA_K_SEVERE,
            Self::Fatal => sys::CA_K_FATAL,
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
        if sys::CA_EXTRACT_SUCCESS(eca) != 0 {
            Ok(())
        } else {
            Err(Error {
                kind: ErrorKind::from_raw_msg_no(sys::CA_EXTRACT_MSG_NO(eca)),
                severity: ErrorSeverity::from_raw_severity(sys::CA_EXTRACT_SEVERITY(eca)),
            })
        }
    }

    pub fn into_raw(self) -> c_int {
        sys::CA_INSERT_MSG_NO(self.kind.to_raw_msg_no())
            | sys::CA_INSERT_SEVERITY(self.severity.to_raw_severity())
    }
}
