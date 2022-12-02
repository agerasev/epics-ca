use std::mem::transmute;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AlarmSeverity {
    None,
    Minor,
    Major,
    Invalid,
}

impl AlarmSeverity {
    pub fn try_from_raw(raw: i32) -> Option<Self> {
        if !(0..(sys::epicsAlarmSeverity::ALARM_NSEV as i32)).contains(&raw) {
            return None;
        }
        let sev = unsafe { transmute::<_, sys::epicsAlarmSeverity>(raw) };
        Some(match sev {
            sys::epicsAlarmSeverity::epicsSevNone => AlarmSeverity::None,
            sys::epicsAlarmSeverity::epicsSevMinor => AlarmSeverity::Minor,
            sys::epicsAlarmSeverity::epicsSevMajor => AlarmSeverity::Major,
            sys::epicsAlarmSeverity::epicsSevInvalid => AlarmSeverity::Invalid,
            _ => unreachable!(),
        })
    }

    pub fn raw(&self) -> sys::epicsAlarmSeverity {
        match self {
            AlarmSeverity::None => sys::epicsAlarmSeverity::epicsSevNone,
            AlarmSeverity::Minor => sys::epicsAlarmSeverity::epicsSevMinor,
            AlarmSeverity::Major => sys::epicsAlarmSeverity::epicsSevMajor,
            AlarmSeverity::Invalid => sys::epicsAlarmSeverity::epicsSevInvalid,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AlarmCondition {
    None,
    Read,
    Write,
    HiHi,
    High,
    LoLo,
    Low,
    State,
    Cos,
    Comm,
    Timeout,
    HwLimit,
    Calc,
    Scan,
    Link,
    Soft,
    BadSub,
    UDF,
    Disable,
    Simm,
    ReadAccess,
    WriteAccess,
}

impl AlarmCondition {
    pub fn try_from_raw(raw: i32) -> Option<Self> {
        if !(0..(sys::epicsAlarmCondition::ALARM_NSTATUS as i32)).contains(&raw) {
            return None;
        }
        let sev = unsafe { transmute::<_, sys::epicsAlarmCondition>(raw) };
        Some(match sev {
            sys::epicsAlarmCondition::epicsAlarmNone => AlarmCondition::None,
            sys::epicsAlarmCondition::epicsAlarmRead => AlarmCondition::Read,
            sys::epicsAlarmCondition::epicsAlarmWrite => AlarmCondition::Write,
            sys::epicsAlarmCondition::epicsAlarmHiHi => AlarmCondition::HiHi,
            sys::epicsAlarmCondition::epicsAlarmHigh => AlarmCondition::High,
            sys::epicsAlarmCondition::epicsAlarmLoLo => AlarmCondition::LoLo,
            sys::epicsAlarmCondition::epicsAlarmLow => AlarmCondition::Low,
            sys::epicsAlarmCondition::epicsAlarmState => AlarmCondition::State,
            sys::epicsAlarmCondition::epicsAlarmCos => AlarmCondition::Cos,
            sys::epicsAlarmCondition::epicsAlarmComm => AlarmCondition::Comm,
            sys::epicsAlarmCondition::epicsAlarmTimeout => AlarmCondition::Timeout,
            sys::epicsAlarmCondition::epicsAlarmHwLimit => AlarmCondition::HwLimit,
            sys::epicsAlarmCondition::epicsAlarmCalc => AlarmCondition::Calc,
            sys::epicsAlarmCondition::epicsAlarmScan => AlarmCondition::Scan,
            sys::epicsAlarmCondition::epicsAlarmLink => AlarmCondition::Link,
            sys::epicsAlarmCondition::epicsAlarmSoft => AlarmCondition::Soft,
            sys::epicsAlarmCondition::epicsAlarmBadSub => AlarmCondition::BadSub,
            sys::epicsAlarmCondition::epicsAlarmUDF => AlarmCondition::UDF,
            sys::epicsAlarmCondition::epicsAlarmDisable => AlarmCondition::Disable,
            sys::epicsAlarmCondition::epicsAlarmSimm => AlarmCondition::Simm,
            sys::epicsAlarmCondition::epicsAlarmReadAccess => AlarmCondition::ReadAccess,
            sys::epicsAlarmCondition::epicsAlarmWriteAccess => AlarmCondition::WriteAccess,
            _ => unreachable!(),
        })
    }

    pub fn raw(&self) -> sys::epicsAlarmCondition {
        match self {
            AlarmCondition::None => sys::epicsAlarmCondition::epicsAlarmNone,
            AlarmCondition::Read => sys::epicsAlarmCondition::epicsAlarmRead,
            AlarmCondition::Write => sys::epicsAlarmCondition::epicsAlarmWrite,
            AlarmCondition::HiHi => sys::epicsAlarmCondition::epicsAlarmHiHi,
            AlarmCondition::High => sys::epicsAlarmCondition::epicsAlarmHigh,
            AlarmCondition::LoLo => sys::epicsAlarmCondition::epicsAlarmLoLo,
            AlarmCondition::Low => sys::epicsAlarmCondition::epicsAlarmLow,
            AlarmCondition::State => sys::epicsAlarmCondition::epicsAlarmState,
            AlarmCondition::Cos => sys::epicsAlarmCondition::epicsAlarmCos,
            AlarmCondition::Comm => sys::epicsAlarmCondition::epicsAlarmComm,
            AlarmCondition::Timeout => sys::epicsAlarmCondition::epicsAlarmTimeout,
            AlarmCondition::HwLimit => sys::epicsAlarmCondition::epicsAlarmHwLimit,
            AlarmCondition::Calc => sys::epicsAlarmCondition::epicsAlarmCalc,
            AlarmCondition::Scan => sys::epicsAlarmCondition::epicsAlarmScan,
            AlarmCondition::Link => sys::epicsAlarmCondition::epicsAlarmLink,
            AlarmCondition::Soft => sys::epicsAlarmCondition::epicsAlarmSoft,
            AlarmCondition::BadSub => sys::epicsAlarmCondition::epicsAlarmBadSub,
            AlarmCondition::UDF => sys::epicsAlarmCondition::epicsAlarmUDF,
            AlarmCondition::Disable => sys::epicsAlarmCondition::epicsAlarmDisable,
            AlarmCondition::Simm => sys::epicsAlarmCondition::epicsAlarmSimm,
            AlarmCondition::ReadAccess => sys::epicsAlarmCondition::epicsAlarmReadAccess,
            AlarmCondition::WriteAccess => sys::epicsAlarmCondition::epicsAlarmWriteAccess,
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Alarm {
    pub severity: AlarmSeverity,
    pub condition: AlarmCondition,
}
