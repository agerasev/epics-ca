use super::{impl_sized_request_methods, ReadRequest, Request, WriteRequest};
use crate::types::{
    Alarm, DbRequest, EpicsEnum, EpicsString, EpicsTimeStamp, Float, Int, Scalar, StaticCString,
};
use std::mem::MaybeUninit;

pub const MAX_UNITS_SIZE: usize = sys::MAX_UNITS_SIZE as usize;
pub const MAX_ENUM_STRING_SIZE: usize = sys::MAX_ENUM_STRING_SIZE as usize;
pub const MAX_ENUM_STATES: usize = sys::MAX_ENUM_STATES as usize;

#[repr(transparent)]
#[derive(Clone, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct Units(pub StaticCString<MAX_UNITS_SIZE>);

pub trait TypedRequest: Request {
    type Type: Scalar;
}
pub trait ScalarRequest: TypedRequest + Sized + Clone + Send {
    fn value(&self) -> &Self::Type {
        unsafe { &*(((self as *const Self).offset(1) as *const Self::Type).offset(-1)) }
    }
    fn value_mut(&mut self) -> &mut Self::Type {
        unsafe { &mut *(((self as *mut Self).offset(1) as *mut Self::Type).offset(-1)) }
    }
}

impl<T: Scalar> TypedRequest for T {
    type Type = T;
}
impl<T: Scalar> Request for T {
    type Raw = T::Raw;
    const ENUM: DbRequest = DbRequest::Base(T::ENUM);
    impl_sized_request_methods!();
}
impl<T: Scalar> ScalarRequest for T {}
impl<T: Scalar> WriteRequest for T {}
impl<T: Scalar> ReadRequest for T {}

macro_rules! impl_typed_request {
    ($struct:ty, $enum:path, $ty:ty, $raw:ty) => {
        impl TypedRequest for $struct {
            type Type = $ty;
        }
        impl Request for $struct {
            type Raw = $raw;
            const ENUM: DbRequest = $enum(<$ty as Scalar>::ENUM);
            impl_sized_request_methods!();
        }
        impl ScalarRequest for $struct {}
        impl ReadRequest for $struct {}
    };
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Sts<T: Scalar> {
    pub alarm: Alarm,
    _value: MaybeUninit<T>,
}

impl_typed_request!(Sts<u8>, DbRequest::Sts, u8, sys::dbr_sts_char);
impl_typed_request!(Sts<i16>, DbRequest::Sts, i16, sys::dbr_sts_short);
impl_typed_request!(Sts<EpicsEnum>, DbRequest::Sts, EpicsEnum, sys::dbr_sts_enum);
impl_typed_request!(Sts<i32>, DbRequest::Sts, i32, sys::dbr_sts_long);
impl_typed_request!(Sts<f32>, DbRequest::Sts, f32, sys::dbr_sts_float);
impl_typed_request!(Sts<f64>, DbRequest::Sts, f64, sys::dbr_sts_double);
impl_typed_request!(
    Sts<EpicsString>,
    DbRequest::Sts,
    EpicsString,
    sys::dbr_sts_string
);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Time<T: Scalar> {
    pub alarm: Alarm,
    pub stamp: EpicsTimeStamp,
    _value: MaybeUninit<T>,
}

impl_typed_request!(Time<u8>, DbRequest::Time, u8, sys::dbr_time_char);
impl_typed_request!(Time<i16>, DbRequest::Time, i16, sys::dbr_time_short);
impl_typed_request!(
    Time<EpicsEnum>,
    DbRequest::Time,
    EpicsEnum,
    sys::dbr_time_enum
);
impl_typed_request!(Time<i32>, DbRequest::Time, i32, sys::dbr_time_long);
impl_typed_request!(Time<f32>, DbRequest::Time, f32, sys::dbr_time_float);
impl_typed_request!(Time<f64>, DbRequest::Time, f64, sys::dbr_time_double);
impl_typed_request!(
    Time<EpicsString>,
    DbRequest::Time,
    EpicsString,
    sys::dbr_time_string
);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct GrInt<T: Scalar + Int> {
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

impl_typed_request!(GrInt<u8>, DbRequest::Gr, u8, sys::dbr_gr_char);
impl_typed_request!(GrInt<i16>, DbRequest::Gr, i16, sys::dbr_gr_short);
impl_typed_request!(GrInt<i32>, DbRequest::Gr, i32, sys::dbr_gr_long);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct GrFloat<T: Scalar + Float> {
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

impl_typed_request!(GrFloat<f32>, DbRequest::Gr, f32, sys::dbr_gr_float);
impl_typed_request!(GrFloat<f64>, DbRequest::Gr, f64, sys::dbr_gr_double);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct GrEnum {
    pub alarm: Alarm,
    pub no_str: u16,
    pub strs: [StaticCString<MAX_ENUM_STRING_SIZE>; MAX_ENUM_STATES],
    _value: MaybeUninit<EpicsEnum>,
}

impl_typed_request!(GrEnum, DbRequest::Ctrl, EpicsEnum, sys::dbr_ctrl_enum);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CtrlInt<T: Scalar + Int> {
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

impl_typed_request!(CtrlInt<u8>, DbRequest::Ctrl, u8, sys::dbr_ctrl_char);
impl_typed_request!(CtrlInt<i16>, DbRequest::Ctrl, i16, sys::dbr_ctrl_short);
impl_typed_request!(CtrlInt<i32>, DbRequest::Ctrl, i32, sys::dbr_ctrl_long);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CtrlFloat<T: Scalar + Float> {
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

impl_typed_request!(CtrlFloat<f32>, DbRequest::Ctrl, f32, sys::dbr_ctrl_float);
impl_typed_request!(CtrlFloat<f64>, DbRequest::Ctrl, f64, sys::dbr_ctrl_double);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CtrlEnum {
    pub alarm: Alarm,
    pub no_str: u16,
    pub strs: [StaticCString<MAX_ENUM_STRING_SIZE>; MAX_ENUM_STATES],
    _value: MaybeUninit<EpicsEnum>,
}

impl_typed_request!(CtrlEnum, DbRequest::Ctrl, EpicsEnum, sys::dbr_ctrl_enum);
