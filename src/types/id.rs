use bitflags::bitflags;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FieldId {
    String,
    Short,
    // Int, // Alias to Short
    Float,
    Enum,
    Char,
    Long,
    Double,
}

impl FieldId {
    pub fn try_from_raw(raw: i32) -> Option<Self> {
        match raw {
            sys::DBF_STRING => Some(FieldId::String),
            sys::DBF_SHORT => Some(FieldId::Short),
            // sys::DBF_INT => Some(DbField::Int),
            sys::DBF_FLOAT => Some(FieldId::Float),
            sys::DBF_ENUM => Some(FieldId::Enum),
            sys::DBF_CHAR => Some(FieldId::Char),
            sys::DBF_LONG => Some(FieldId::Long),
            sys::DBF_DOUBLE => Some(FieldId::Double),
            _ => None,
        }
    }

    pub fn raw(&self) -> i32 {
        match self {
            FieldId::String => sys::DBF_STRING,
            FieldId::Short => sys::DBF_SHORT,
            // DbField::Int => sys::DBF_INT,
            FieldId::Float => sys::DBF_FLOAT,
            FieldId::Enum => sys::DBF_ENUM,
            FieldId::Char => sys::DBF_CHAR,
            FieldId::Long => sys::DBF_LONG,
            FieldId::Double => sys::DBF_DOUBLE,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RequestId {
    Base(FieldId),
    Sts(FieldId),
    Time(FieldId),
    Gr(FieldId),
    Ctrl(FieldId),
    PutAckt,
    PutAcks,
    StsackString,
    ClassName,
}

impl RequestId {
    pub fn try_from_raw(raw: i32) -> Option<Self> {
        match raw {
            sys::DBR_STRING => Some(RequestId::Base(FieldId::String)),
            sys::DBR_SHORT => Some(RequestId::Base(FieldId::Short)),
            // sys::DBR_INT => Some(DbRequest::Base(DbField::Int)),
            sys::DBR_FLOAT => Some(RequestId::Base(FieldId::Float)),
            sys::DBR_ENUM => Some(RequestId::Base(FieldId::Enum)),
            sys::DBR_CHAR => Some(RequestId::Base(FieldId::Char)),
            sys::DBR_LONG => Some(RequestId::Base(FieldId::Long)),
            sys::DBR_DOUBLE => Some(RequestId::Base(FieldId::Double)),

            sys::DBR_STS_STRING => Some(RequestId::Sts(FieldId::String)),
            sys::DBR_STS_SHORT => Some(RequestId::Sts(FieldId::Short)),
            // sys::DBR_STS_INT => Some(DbRequest::Sts(DbField::Int)),
            sys::DBR_STS_FLOAT => Some(RequestId::Sts(FieldId::Float)),
            sys::DBR_STS_ENUM => Some(RequestId::Sts(FieldId::Enum)),
            sys::DBR_STS_CHAR => Some(RequestId::Sts(FieldId::Char)),
            sys::DBR_STS_LONG => Some(RequestId::Sts(FieldId::Long)),
            sys::DBR_STS_DOUBLE => Some(RequestId::Sts(FieldId::Double)),

            sys::DBR_TIME_STRING => Some(RequestId::Time(FieldId::String)),
            sys::DBR_TIME_SHORT => Some(RequestId::Time(FieldId::Short)),
            // sys::DBR_TIME_INT => Some(DbRequest::Time(DbField::Int)),
            sys::DBR_TIME_FLOAT => Some(RequestId::Time(FieldId::Float)),
            sys::DBR_TIME_ENUM => Some(RequestId::Time(FieldId::Enum)),
            sys::DBR_TIME_CHAR => Some(RequestId::Time(FieldId::Char)),
            sys::DBR_TIME_LONG => Some(RequestId::Time(FieldId::Long)),
            sys::DBR_TIME_DOUBLE => Some(RequestId::Time(FieldId::Double)),

            sys::DBR_GR_STRING => Some(RequestId::Gr(FieldId::String)),
            sys::DBR_GR_SHORT => Some(RequestId::Gr(FieldId::Short)),
            // sys::DBR_GR_INT => Some(DbRequest::Gr(DbField::Int)),
            sys::DBR_GR_FLOAT => Some(RequestId::Gr(FieldId::Float)),
            sys::DBR_GR_ENUM => Some(RequestId::Gr(FieldId::Enum)),
            sys::DBR_GR_CHAR => Some(RequestId::Gr(FieldId::Char)),
            sys::DBR_GR_LONG => Some(RequestId::Gr(FieldId::Long)),
            sys::DBR_GR_DOUBLE => Some(RequestId::Gr(FieldId::Double)),

            sys::DBR_CTRL_STRING => Some(RequestId::Ctrl(FieldId::String)),
            sys::DBR_CTRL_SHORT => Some(RequestId::Ctrl(FieldId::Short)),
            // sys::DBR_CTRL_INT => Some(DbRequest::Ctrl(DbField::Int)),
            sys::DBR_CTRL_FLOAT => Some(RequestId::Ctrl(FieldId::Float)),
            sys::DBR_CTRL_ENUM => Some(RequestId::Ctrl(FieldId::Enum)),
            sys::DBR_CTRL_CHAR => Some(RequestId::Ctrl(FieldId::Char)),
            sys::DBR_CTRL_LONG => Some(RequestId::Ctrl(FieldId::Long)),
            sys::DBR_CTRL_DOUBLE => Some(RequestId::Ctrl(FieldId::Double)),

            sys::DBR_PUT_ACKT => Some(RequestId::PutAckt),
            sys::DBR_PUT_ACKS => Some(RequestId::PutAcks),
            sys::DBR_STSACK_STRING => Some(RequestId::StsackString),
            sys::DBR_CLASS_NAME => Some(RequestId::ClassName),

            _ => None,
        }
    }

    pub fn raw(&self) -> i32 {
        match self {
            RequestId::Base(dbf) => match dbf {
                FieldId::String => sys::DBR_STRING,
                FieldId::Short => sys::DBR_SHORT,
                // DbField::Int => sys::DBR_INT,
                FieldId::Float => sys::DBR_FLOAT,
                FieldId::Enum => sys::DBR_ENUM,
                FieldId::Char => sys::DBR_CHAR,
                FieldId::Long => sys::DBR_LONG,
                FieldId::Double => sys::DBR_DOUBLE,
            },
            RequestId::Sts(dbf) => match dbf {
                FieldId::String => sys::DBR_STS_STRING,
                FieldId::Short => sys::DBR_STS_SHORT,
                // DbField::Int => sys::DBR_STS_INT,
                FieldId::Float => sys::DBR_STS_FLOAT,
                FieldId::Enum => sys::DBR_STS_ENUM,
                FieldId::Char => sys::DBR_STS_CHAR,
                FieldId::Long => sys::DBR_STS_LONG,
                FieldId::Double => sys::DBR_STS_DOUBLE,
            },
            RequestId::Time(dbf) => match dbf {
                FieldId::String => sys::DBR_TIME_STRING,
                FieldId::Short => sys::DBR_TIME_SHORT,
                // DbField::Int => sys::DBR_TIME_INT,
                FieldId::Float => sys::DBR_TIME_FLOAT,
                FieldId::Enum => sys::DBR_TIME_ENUM,
                FieldId::Char => sys::DBR_TIME_CHAR,
                FieldId::Long => sys::DBR_TIME_LONG,
                FieldId::Double => sys::DBR_TIME_DOUBLE,
            },
            RequestId::Gr(dbf) => match dbf {
                FieldId::String => sys::DBR_GR_STRING,
                FieldId::Short => sys::DBR_GR_SHORT,
                // DbField::Int => sys::DBR_GR_INT,
                FieldId::Float => sys::DBR_GR_FLOAT,
                FieldId::Enum => sys::DBR_GR_ENUM,
                FieldId::Char => sys::DBR_GR_CHAR,
                FieldId::Long => sys::DBR_GR_LONG,
                FieldId::Double => sys::DBR_GR_DOUBLE,
            },
            RequestId::Ctrl(dbf) => match dbf {
                FieldId::String => sys::DBR_CTRL_STRING,
                FieldId::Short => sys::DBR_CTRL_SHORT,
                // DbField::Int => sys::DBR_CTRL_INT,
                FieldId::Float => sys::DBR_CTRL_FLOAT,
                FieldId::Enum => sys::DBR_CTRL_ENUM,
                FieldId::Char => sys::DBR_CTRL_CHAR,
                FieldId::Long => sys::DBR_CTRL_LONG,
                FieldId::Double => sys::DBR_CTRL_DOUBLE,
            },
            RequestId::PutAckt => sys::DBR_PUT_ACKT,
            RequestId::PutAcks => sys::DBR_PUT_ACKS,
            RequestId::StsackString => sys::DBR_STSACK_STRING,
            RequestId::ClassName => sys::DBR_CLASS_NAME,
        }
    }
}

bitflags! {
    pub struct EventMask: u32 {
        const VALUE = sys::DBE_VALUE as u32;
        const ARCHIVE = sys::DBE_ARCHIVE as u32;
        const ALARM = sys::DBE_ALARM as u32;
        const PROPERTY = sys::DBE_PROPERTY as u32;
    }
}

impl EventMask {
    pub fn try_from_raw(raw: i32) -> Option<Self> {
        Self::from_bits(raw as u32)
    }
    pub fn raw(&self) -> i32 {
        self.bits() as i32
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

#[cfg(test)]
mod tests {
    use super::*;

    fn dbf_size(dbf: FieldId) -> usize {
        unsafe { *(sys::dbr_size.as_ptr().offset(dbf.raw() as isize)) as usize }
    }

    #[test]
    fn dbr_sizes() {
        assert_eq!(dbf_size(FieldId::String), sys::MAX_STRING_SIZE as usize);
        assert_eq!(dbf_size(FieldId::Short), 2);
        assert_eq!(dbf_size(FieldId::Float), 4);
        assert_eq!(dbf_size(FieldId::Enum), 2);
        assert_eq!(dbf_size(FieldId::Char), 1);
        assert_eq!(dbf_size(FieldId::Long), 4);
        assert_eq!(dbf_size(FieldId::Double), 8);
    }
}
