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
    Other(i32),
}

impl ErrorKind {
    pub const fn from_raw_msg_no(msg_no: i32) -> Self {
        if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_ALLOCMEM) {
            Self::Allocmem
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_TOLARGE) {
            Self::Tolarge
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_TIMEOUT) {
            Self::Timeout
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_BADTYPE) {
            Self::Badtype
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_INTERNAL) {
            Self::Internal
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_GETFAIL) {
            Self::Getfail
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_PUTFAIL) {
            Self::Putfail
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_BADCOUNT) {
            Self::Badcount
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_BADSTR) {
            Self::Badstr
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_DISCONN) {
            Self::Disconn
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_DBLCHNL) {
            Self::Dblchnl
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_EVDISALLOW) {
            Self::Evdisallow
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_BADMONID) {
            Self::Badmonid
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_BADMASK) {
            Self::Badmask
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_BADSYNCGRP) {
            Self::Badsyncgrp
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_PUTCBINPROG) {
            Self::Putcbinprog
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_NORDACCESS) {
            Self::Nordaccess
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_NOWTACCESS) {
            Self::Nowtaccess
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_ANACHRONISM) {
            Self::Anachronism
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_NOSEARCHADDR) {
            Self::Nosearchaddr
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_NOCONVERT) {
            Self::Noconvert
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_BADCHID) {
            Self::Badchid
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_BADFUNCPTR) {
            Self::Badfuncptr
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_ISATTACHED) {
            Self::Isattached
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_UNAVAILINSERV) {
            Self::Unavailinserv
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_CHANDESTROY) {
            Self::Chandestroy
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_BADPRIORITY) {
            Self::Badpriority
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_NOTTHREADED) {
            Self::Notthreaded
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_16KARRAYCLIENT) {
            Self::N16karrayclient
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_CONNSEQTMO) {
            Self::Connseqtmo
        } else if msg_no == sys::CA_EXTRACT_MSG_NO(sys::ECA_UNRESPTMO) {
            Self::Unresptmo
        } else {
            Self::Other(msg_no)
        }
    }

    pub const fn to_raw_msg_no(self) -> i32 {
        match self {
            Self::Allocmem => sys::CA_EXTRACT_MSG_NO(sys::ECA_ALLOCMEM),
            Self::Tolarge => sys::CA_EXTRACT_MSG_NO(sys::ECA_TOLARGE),
            Self::Timeout => sys::CA_EXTRACT_MSG_NO(sys::ECA_TIMEOUT),
            Self::Badtype => sys::CA_EXTRACT_MSG_NO(sys::ECA_BADTYPE),
            Self::Internal => sys::CA_EXTRACT_MSG_NO(sys::ECA_INTERNAL),
            Self::Getfail => sys::CA_EXTRACT_MSG_NO(sys::ECA_GETFAIL),
            Self::Putfail => sys::CA_EXTRACT_MSG_NO(sys::ECA_PUTFAIL),
            Self::Badcount => sys::CA_EXTRACT_MSG_NO(sys::ECA_BADCOUNT),
            Self::Badstr => sys::CA_EXTRACT_MSG_NO(sys::ECA_BADSTR),
            Self::Disconn => sys::CA_EXTRACT_MSG_NO(sys::ECA_DISCONN),
            Self::Dblchnl => sys::CA_EXTRACT_MSG_NO(sys::ECA_DBLCHNL),
            Self::Evdisallow => sys::CA_EXTRACT_MSG_NO(sys::ECA_EVDISALLOW),
            Self::Badmonid => sys::CA_EXTRACT_MSG_NO(sys::ECA_BADMONID),
            Self::Badmask => sys::CA_EXTRACT_MSG_NO(sys::ECA_BADMASK),
            Self::Badsyncgrp => sys::CA_EXTRACT_MSG_NO(sys::ECA_BADSYNCGRP),
            Self::Putcbinprog => sys::CA_EXTRACT_MSG_NO(sys::ECA_PUTCBINPROG),
            Self::Nordaccess => sys::CA_EXTRACT_MSG_NO(sys::ECA_NORDACCESS),
            Self::Nowtaccess => sys::CA_EXTRACT_MSG_NO(sys::ECA_NOWTACCESS),
            Self::Anachronism => sys::CA_EXTRACT_MSG_NO(sys::ECA_ANACHRONISM),
            Self::Nosearchaddr => sys::CA_EXTRACT_MSG_NO(sys::ECA_NOSEARCHADDR),
            Self::Noconvert => sys::CA_EXTRACT_MSG_NO(sys::ECA_NOCONVERT),
            Self::Badchid => sys::CA_EXTRACT_MSG_NO(sys::ECA_BADCHID),
            Self::Badfuncptr => sys::CA_EXTRACT_MSG_NO(sys::ECA_BADFUNCPTR),
            Self::Isattached => sys::CA_EXTRACT_MSG_NO(sys::ECA_ISATTACHED),
            Self::Unavailinserv => sys::CA_EXTRACT_MSG_NO(sys::ECA_UNAVAILINSERV),
            Self::Chandestroy => sys::CA_EXTRACT_MSG_NO(sys::ECA_CHANDESTROY),
            Self::Badpriority => sys::CA_EXTRACT_MSG_NO(sys::ECA_BADPRIORITY),
            Self::Notthreaded => sys::CA_EXTRACT_MSG_NO(sys::ECA_NOTTHREADED),
            Self::N16karrayclient => sys::CA_EXTRACT_MSG_NO(sys::ECA_16KARRAYCLIENT),
            Self::Connseqtmo => sys::CA_EXTRACT_MSG_NO(sys::ECA_CONNSEQTMO),
            Self::Unresptmo => sys::CA_EXTRACT_MSG_NO(sys::ECA_UNRESPTMO),
            Self::Other(msg_no) => msg_no,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Severe,
    Fatal,
    Unknown(i32),
}

impl ErrorSeverity {
    pub const fn from_raw_severity(severity: i32) -> Self {
        match severity {
            sys::CA_K_ERROR => Self::Error,
            sys::CA_K_WARNING => Self::Warning,
            sys::CA_K_SEVERE => Self::Severe,
            sys::CA_K_FATAL => Self::Fatal,
            sev => Self::Unknown(sev),
        }
    }

    pub const fn to_raw_severity(self) -> i32 {
        match self {
            Self::Error => sys::CA_K_ERROR,
            Self::Warning => sys::CA_K_WARNING,
            Self::Severe => sys::CA_K_SEVERE,
            Self::Fatal => sys::CA_K_FATAL,
            Self::Unknown(sev) => sev,
        }
    }
}

/// Error that can occur in EPICS client or server.
#[derive(Debug, Clone, Copy)]
pub struct Error {
    pub kind: ErrorKind,
    pub severity: ErrorSeverity,
}

impl Error {
    pub const fn try_from_raw(eca: i32) -> Option<Self> {
        if sys::CA_EXTRACT_SUCCESS(eca) != 0 {
            None
        } else {
            Some(Self::from_raw_unchecked(eca))
        }
    }

    const fn from_raw_unchecked(eca: i32) -> Self {
        Error {
            kind: ErrorKind::from_raw_msg_no(sys::CA_EXTRACT_MSG_NO(eca)),
            severity: ErrorSeverity::from_raw_severity(sys::CA_EXTRACT_SEVERITY(eca)),
        }
    }

    pub const fn into_raw(self) -> i32 {
        sys::CA_INSERT_MSG_NO(self.kind.to_raw_msg_no())
            | sys::CA_INSERT_SEVERITY(self.severity.to_raw_severity())
    }
}

/// Convert raw EPICS error to Result.
pub fn result_from_raw(eca: i32) -> Result<(), Error> {
    match Error::try_from_raw(eca) {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub const ALLOCMEM: Error = Error::from_raw_unchecked(sys::ECA_ALLOCMEM);
pub const TOLARGE: Error = Error::from_raw_unchecked(sys::ECA_TOLARGE);
pub const TIMEOUT: Error = Error::from_raw_unchecked(sys::ECA_TIMEOUT);
pub const BADTYPE: Error = Error::from_raw_unchecked(sys::ECA_BADTYPE);
pub const INTERNAL: Error = Error::from_raw_unchecked(sys::ECA_INTERNAL);
pub const GETFAIL: Error = Error::from_raw_unchecked(sys::ECA_GETFAIL);
pub const PUTFAIL: Error = Error::from_raw_unchecked(sys::ECA_PUTFAIL);
pub const BADCOUNT: Error = Error::from_raw_unchecked(sys::ECA_BADCOUNT);
pub const BADSTR: Error = Error::from_raw_unchecked(sys::ECA_BADSTR);
pub const DISCONN: Error = Error::from_raw_unchecked(sys::ECA_DISCONN);
pub const DBLCHNL: Error = Error::from_raw_unchecked(sys::ECA_DBLCHNL);
pub const EVDISALLOW: Error = Error::from_raw_unchecked(sys::ECA_EVDISALLOW);
pub const BADMONID: Error = Error::from_raw_unchecked(sys::ECA_BADMONID);
pub const BADMASK: Error = Error::from_raw_unchecked(sys::ECA_BADMASK);
pub const BADSYNCGRP: Error = Error::from_raw_unchecked(sys::ECA_BADSYNCGRP);
pub const PUTCBINPROG: Error = Error::from_raw_unchecked(sys::ECA_PUTCBINPROG);
pub const NORDACCESS: Error = Error::from_raw_unchecked(sys::ECA_NORDACCESS);
pub const NOWTACCESS: Error = Error::from_raw_unchecked(sys::ECA_NOWTACCESS);
pub const ANACHRONISM: Error = Error::from_raw_unchecked(sys::ECA_ANACHRONISM);
pub const NOSEARCHADDR: Error = Error::from_raw_unchecked(sys::ECA_NOSEARCHADDR);
pub const NOCONVERT: Error = Error::from_raw_unchecked(sys::ECA_NOCONVERT);
pub const BADCHID: Error = Error::from_raw_unchecked(sys::ECA_BADCHID);
pub const BADFUNCPTR: Error = Error::from_raw_unchecked(sys::ECA_BADFUNCPTR);
pub const ISATTACHED: Error = Error::from_raw_unchecked(sys::ECA_ISATTACHED);
pub const UNAVAILINSERV: Error = Error::from_raw_unchecked(sys::ECA_UNAVAILINSERV);
pub const CHANDESTROY: Error = Error::from_raw_unchecked(sys::ECA_CHANDESTROY);
pub const BADPRIORITY: Error = Error::from_raw_unchecked(sys::ECA_BADPRIORITY);
pub const NOTTHREADED: Error = Error::from_raw_unchecked(sys::ECA_NOTTHREADED);
pub const N16KARRAYCLIENT: Error = Error::from_raw_unchecked(sys::ECA_16KARRAYCLIENT);
pub const CONNSEQTMO: Error = Error::from_raw_unchecked(sys::ECA_CONNSEQTMO);
pub const UNRESPTMO: Error = Error::from_raw_unchecked(sys::ECA_UNRESPTMO);
