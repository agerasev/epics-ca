mod sys {
    pub use epics_sys::{cadef::*, caeventmask::*, db_access::*};
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DbField {
    String,
    Short,
    Int,
    Float,
    Enum,
    Char,
    Long,
    Double,
}

impl DbField {
    pub fn raw(&self) -> i32 {
        match self {
            DbField::String => sys::DBF_STRING,
            DbField::Short => sys::DBF_SHORT,
            DbField::Int => sys::DBF_INT,
            DbField::Float => sys::DBF_FLOAT,
            DbField::Enum => sys::DBF_ENUM,
            DbField::Char => sys::DBF_CHAR,
            DbField::Long => sys::DBF_LONG,
            DbField::Double => sys::DBF_DOUBLE,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DbRequest {
    Base(DbField),
    Sts(DbField),
    Time(DbField),
    Gr(DbField),
    Ctrl(DbField),
    PutAck(bool),
    StsackString,
    ClassName,
}

impl DbRequest {
    pub fn raw(&self) -> i32 {
        match self {
            DbRequest::Base(dbf) => match dbf {
                DbField::String => sys::DBR_STRING,
                DbField::Int => sys::DBR_INT,
                DbField::Short => sys::DBR_SHORT,
                DbField::Float => sys::DBR_FLOAT,
                DbField::Enum => sys::DBR_ENUM,
                DbField::Char => sys::DBR_CHAR,
                DbField::Long => sys::DBR_LONG,
                DbField::Double => sys::DBR_DOUBLE,
            },
            DbRequest::Sts(dbf) => match dbf {
                DbField::String => sys::DBR_STS_STRING,
                DbField::Short => sys::DBR_STS_SHORT,
                DbField::Int => sys::DBR_STS_INT,
                DbField::Float => sys::DBR_STS_FLOAT,
                DbField::Enum => sys::DBR_STS_ENUM,
                DbField::Char => sys::DBR_STS_CHAR,
                DbField::Long => sys::DBR_STS_LONG,
                DbField::Double => sys::DBR_STS_DOUBLE,
            },
            DbRequest::Time(dbf) => match dbf {
                DbField::String => sys::DBR_TIME_STRING,
                DbField::Int => sys::DBR_TIME_INT,
                DbField::Short => sys::DBR_TIME_SHORT,
                DbField::Float => sys::DBR_TIME_FLOAT,
                DbField::Enum => sys::DBR_TIME_ENUM,
                DbField::Char => sys::DBR_TIME_CHAR,
                DbField::Long => sys::DBR_TIME_LONG,
                DbField::Double => sys::DBR_TIME_DOUBLE,
            },
            DbRequest::Gr(dbf) => match dbf {
                DbField::String => sys::DBR_GR_STRING,
                DbField::Short => sys::DBR_GR_SHORT,
                DbField::Int => sys::DBR_GR_INT,
                DbField::Float => sys::DBR_GR_FLOAT,
                DbField::Enum => sys::DBR_GR_ENUM,
                DbField::Char => sys::DBR_GR_CHAR,
                DbField::Long => sys::DBR_GR_LONG,
                DbField::Double => sys::DBR_GR_DOUBLE,
            },
            DbRequest::Ctrl(dbf) => match dbf {
                DbField::String => sys::DBR_CTRL_STRING,
                DbField::Short => sys::DBR_CTRL_SHORT,
                DbField::Int => sys::DBR_CTRL_INT,
                DbField::Float => sys::DBR_CTRL_FLOAT,
                DbField::Enum => sys::DBR_CTRL_ENUM,
                DbField::Char => sys::DBR_CTRL_CHAR,
                DbField::Long => sys::DBR_CTRL_LONG,
                DbField::Double => sys::DBR_CTRL_DOUBLE,
            },
            DbRequest::PutAck(ts) => match ts {
                false => sys::DBR_PUT_ACKT,
                true => sys::DBR_PUT_ACKS,
            },
            DbRequest::StsackString => sys::DBR_STSACK_STRING,
            DbRequest::ClassName => sys::DBR_CLASS_NAME,
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
    pub fn raw(&self) -> i32 {
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
    pub fn raw(self) -> sys::ca_access_rights {
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
