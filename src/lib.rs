mod sys;

use core::ffi::*;

use sys::ca_preemptive_callback_select;

#[derive(Debug, Clone, Copy)]
pub struct Eca {
    raw: c_int,
}
impl Eca {
    fn from_raw(raw: c_int) -> Self {
        Self { raw }
    }
    fn raw(&self) -> c_int {
        self.raw
    }

    pub fn msg_no(&self) -> c_int {
        sys::err::CA_EXTRACT_MSG_NO(self.raw)
    }
    pub fn severity(&self) -> c_int {
        sys::err::CA_EXTRACT_SEVERITY(self.raw)
    }
    pub fn success(&self) -> c_int {
        sys::err::CA_EXTRACT_SEVERITY(self.raw)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dbf {
    String,
    Short,
    Int,
    Float,
    Enum,
    Char,
    Long,
    Double,
}

impl Dbf {
    fn raw(&self) -> c_ulong {
        match self {
            Dbf::String => sys::DBF_STRING,
            Dbf::Short => sys::DBF_SHORT,
            Dbf::Int => sys::DBF_INT,
            Dbf::Float => sys::DBF_FLOAT,
            Dbf::Enum => sys::DBF_ENUM,
            Dbf::Char => sys::DBF_CHAR,
            Dbf::Long => sys::DBF_LONG,
            Dbf::Double => sys::DBF_DOUBLE,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dbr {
    Base(Dbf),
    Sts(Dbf),
    Time(Dbf),
    Gr(Dbf),
    Ctrl(Dbf),
    PutAck(bool),
    StsackString,
    ClassName,
}

impl Dbr {
    fn raw(&self) -> c_ulong {
        match self {
            Dbr::Base(dbf) => match dbf {
                Dbf::String => sys::DBR_STRING,
                Dbf::Int => sys::DBR_INT,
                Dbf::Short => sys::DBR_SHORT,
                Dbf::Float => sys::DBR_FLOAT,
                Dbf::Enum => sys::DBR_ENUM,
                Dbf::Char => sys::DBR_CHAR,
                Dbf::Long => sys::DBR_LONG,
                Dbf::Double => sys::DBR_DOUBLE,
            },
            Dbr::Sts(dbf) => match dbf {
                Dbf::String => sys::DBR_STS_STRING,
                Dbf::Short => sys::DBR_STS_SHORT,
                Dbf::Int => sys::DBR_STS_INT,
                Dbf::Float => sys::DBR_STS_FLOAT,
                Dbf::Enum => sys::DBR_STS_ENUM,
                Dbf::Char => sys::DBR_STS_CHAR,
                Dbf::Long => sys::DBR_STS_LONG,
                Dbf::Double => sys::DBR_STS_DOUBLE,
            },
            Dbr::Time(dbf) => match dbf {
                Dbf::String => sys::DBR_TIME_STRING,
                Dbf::Int => sys::DBR_TIME_INT,
                Dbf::Short => sys::DBR_TIME_SHORT,
                Dbf::Float => sys::DBR_TIME_FLOAT,
                Dbf::Enum => sys::DBR_TIME_ENUM,
                Dbf::Char => sys::DBR_TIME_CHAR,
                Dbf::Long => sys::DBR_TIME_LONG,
                Dbf::Double => sys::DBR_TIME_DOUBLE,
            },
            Dbr::Gr(dbf) => match dbf {
                Dbf::String => sys::DBR_GR_STRING,
                Dbf::Short => sys::DBR_GR_SHORT,
                Dbf::Int => sys::DBR_GR_INT,
                Dbf::Float => sys::DBR_GR_FLOAT,
                Dbf::Enum => sys::DBR_GR_ENUM,
                Dbf::Char => sys::DBR_GR_CHAR,
                Dbf::Long => sys::DBR_GR_LONG,
                Dbf::Double => sys::DBR_GR_DOUBLE,
            },
            Dbr::Ctrl(dbf) => match dbf {
                Dbf::String => sys::DBR_CTRL_STRING,
                Dbf::Short => sys::DBR_CTRL_SHORT,
                Dbf::Int => sys::DBR_CTRL_INT,
                Dbf::Float => sys::DBR_CTRL_FLOAT,
                Dbf::Enum => sys::DBR_CTRL_ENUM,
                Dbf::Char => sys::DBR_CTRL_CHAR,
                Dbf::Long => sys::DBR_CTRL_LONG,
                Dbf::Double => sys::DBR_CTRL_DOUBLE,
            },
            Dbr::PutAck(ts) => match ts {
                false => sys::DBR_PUT_ACKT,
                true => sys::DBR_PUT_ACKS,
            },
            Dbr::StsackString => sys::DBR_STSACK_STRING,
            Dbr::ClassName => sys::DBR_CLASS_NAME,
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
    fn raw(&self) -> c_ulong {
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
    fn raw(self) -> sys::ca_access_rights {
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

#[derive(Debug)]
pub struct Context {
    raw: *mut sys::ca_client_context,
}

impl Context {
    fn new(preemptive_callback: bool) -> Self {
        let select = if preemptive_callback {
            ca_preemptive_callback_select::ca_enable_preemptive_callback
        } else {
            ca_preemptive_callback_select::ca_disable_preemptive_callback
        };
        let eca = Eca::from_raw(unsafe { sys::ca_context_create(select) });
        let raw = unsafe { sys::ca_current_context() };
        assert!(!raw.is_null());
        Self { raw }
    }
    fn attach(&mut self) {
        unsafe { sys::ca_attach_context(self.raw) };
    }
    fn detach() {
        unsafe { sys::ca_detach_context() };
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::ca_context_destroy() };
    }
}

#[derive(Debug)]
pub struct Channel {
    raw: sys::chanId,
}
