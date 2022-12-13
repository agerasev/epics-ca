use crate::types::{
    Alarm, EpicsEnum, EpicsString, EpicsTimeStamp, Field, Float, Int, RequestId, StaticCString,
};
use std::mem::MaybeUninit;

pub const MAX_UNITS_SIZE: usize = sys::MAX_UNITS_SIZE as usize;
pub const MAX_ENUM_STRING_SIZE: usize = sys::MAX_ENUM_STRING_SIZE as usize;
pub const MAX_ENUM_STATES: usize = sys::MAX_ENUM_STATES as usize;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct Units(pub StaticCString<MAX_UNITS_SIZE>);

pub trait Meta<T: Field>: Copy + Send + Sized + 'static {
    type Raw: Copy + Send + Sized + 'static;
    const ENUM: RequestId;
}

impl<T: Field> Meta<T> for T {
    type Raw = T::Raw;
    const ENUM: RequestId = RequestId::Base(T::ENUM);
}

macro_rules! impl_meta {
    ($struct:ty, $enum:path, $ty:ty, $raw:ty) => {
        impl Meta<$ty> for $struct {
            type Raw = $raw;
            const ENUM: RequestId = $enum(<$ty as Field>::ENUM);
        }
    };
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Sts<T: Field> {
    pub alarm: Alarm,
    _value: MaybeUninit<T>,
}

impl_meta!(Sts<u8>, RequestId::Sts, u8, sys::dbr_sts_char);
impl_meta!(Sts<i16>, RequestId::Sts, i16, sys::dbr_sts_short);
impl_meta!(Sts<EpicsEnum>, RequestId::Sts, EpicsEnum, sys::dbr_sts_enum);
impl_meta!(Sts<i32>, RequestId::Sts, i32, sys::dbr_sts_long);
impl_meta!(Sts<f32>, RequestId::Sts, f32, sys::dbr_sts_float);
impl_meta!(Sts<f64>, RequestId::Sts, f64, sys::dbr_sts_double);
impl_meta!(
    Sts<EpicsString>,
    RequestId::Sts,
    EpicsString,
    sys::dbr_sts_string
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Time<T: Field> {
    pub alarm: Alarm,
    pub stamp: EpicsTimeStamp,
    _value: MaybeUninit<T>,
}

impl_meta!(Time<u8>, RequestId::Time, u8, sys::dbr_time_char);
impl_meta!(Time<i16>, RequestId::Time, i16, sys::dbr_time_short);
impl_meta!(
    Time<EpicsEnum>,
    RequestId::Time,
    EpicsEnum,
    sys::dbr_time_enum
);
impl_meta!(Time<i32>, RequestId::Time, i32, sys::dbr_time_long);
impl_meta!(Time<f32>, RequestId::Time, f32, sys::dbr_time_float);
impl_meta!(Time<f64>, RequestId::Time, f64, sys::dbr_time_double);
impl_meta!(
    Time<EpicsString>,
    RequestId::Time,
    EpicsString,
    sys::dbr_time_string
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrInt<T: Field + Int> {
    pub alarm: Alarm,
    pub units: Units,
    pub upper_disp_limit: T,
    pub lower_disp_limit: T,
    pub upper_alarm_limit: T,
    pub upper_warning_limit: T,
    pub lower_warning_limit: T,
    pub lower_alarm_limit: T,
    _value: MaybeUninit<T>,
}

impl_meta!(GrInt<u8>, RequestId::Gr, u8, sys::dbr_gr_char);
impl_meta!(GrInt<i16>, RequestId::Gr, i16, sys::dbr_gr_short);
impl_meta!(GrInt<i32>, RequestId::Gr, i32, sys::dbr_gr_long);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrFloat<T: Field + Float> {
    pub alarm: Alarm,
    pub precision: i16,
    _padding: u16,
    pub units: Units,
    pub upper_disp_limit: T,
    pub lower_disp_limit: T,
    pub upper_alarm_limit: T,
    pub upper_warning_limit: T,
    pub lower_warning_limit: T,
    pub lower_alarm_limit: T,
    _value: MaybeUninit<T>,
}

impl_meta!(GrFloat<f32>, RequestId::Gr, f32, sys::dbr_gr_float);
impl_meta!(GrFloat<f64>, RequestId::Gr, f64, sys::dbr_gr_double);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrEnum {
    pub alarm: Alarm,
    pub no_str: u16,
    pub strs: [StaticCString<MAX_ENUM_STRING_SIZE>; MAX_ENUM_STATES],
    _value: MaybeUninit<EpicsEnum>,
}

impl_meta!(GrEnum, RequestId::Ctrl, EpicsEnum, sys::dbr_ctrl_enum);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlInt<T: Field + Int> {
    pub alarm: Alarm,
    pub units: Units,
    pub upper_disp_limit: T,
    pub lower_disp_limit: T,
    pub upper_alarm_limit: T,
    pub upper_warning_limit: T,
    pub lower_warning_limit: T,
    pub lower_alarm_limit: T,
    pub upper_ctrl_limit: T,
    pub lower_ctrl_limit: T,
    _value: MaybeUninit<T>,
}

impl_meta!(CtrlInt<u8>, RequestId::Ctrl, u8, sys::dbr_ctrl_char);
impl_meta!(CtrlInt<i16>, RequestId::Ctrl, i16, sys::dbr_ctrl_short);
impl_meta!(CtrlInt<i32>, RequestId::Ctrl, i32, sys::dbr_ctrl_long);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlFloat<T: Field + Float> {
    pub alarm: Alarm,
    pub precision: i16,
    _padding: u16,
    pub units: Units,
    pub upper_disp_limit: T,
    pub lower_disp_limit: T,
    pub upper_alarm_limit: T,
    pub upper_warning_limit: T,
    pub lower_warning_limit: T,
    pub lower_alarm_limit: T,
    pub upper_ctrl_limit: T,
    pub lower_ctrl_limit: T,
    _value: MaybeUninit<T>,
}

impl_meta!(CtrlFloat<f32>, RequestId::Ctrl, f32, sys::dbr_ctrl_float);
impl_meta!(CtrlFloat<f64>, RequestId::Ctrl, f64, sys::dbr_ctrl_double);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlEnum {
    pub alarm: Alarm,
    pub no_str: u16,
    pub strs: [StaticCString<MAX_ENUM_STRING_SIZE>; MAX_ENUM_STATES],
    _value: MaybeUninit<EpicsEnum>,
}

impl_meta!(CtrlEnum, RequestId::Ctrl, EpicsEnum, sys::dbr_ctrl_enum);
