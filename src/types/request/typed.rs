use super::{AnyRequest, ReadRequest, WriteRequest};
use crate::types::{
    Alarm, DbRequest, EpicsEnum, EpicsString, EpicsTimeStamp, Primitive, Scalar, StaticCString,
    Type,
};
use std::{marker::PhantomData, mem::MaybeUninit};

pub const MAX_UNITS_SIZE: usize = sys::MAX_UNITS_SIZE as usize;
pub const MAX_ENUM_STRING_SIZE: usize = sys::MAX_ENUM_STRING_SIZE as usize;
pub const MAX_ENUM_STATES: usize = sys::MAX_ENUM_STATES as usize;

#[derive(Clone, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
#[repr(C)]
pub struct Units(pub StaticCString<MAX_UNITS_SIZE>);

pub trait Request: AnyRequest {
    type Type: Type + ?Sized;
}

impl<T: Type + ?Sized> Request for T {
    type Type = T;
}
impl<T: Type + ?Sized> AnyRequest for T {
    type Raw = <<Self as Type>::Element as Scalar>::Raw;
    const ENUM: DbRequest = DbRequest::Base(<<Self as Type>::Element as Scalar>::ENUM);
}
impl<T: Type + ?Sized> WriteRequest for T {}
impl<T: Type + ?Sized> ReadRequest for T {}

macro_rules! impl_all {
    ($struct:ident, $enum:path, $scal:ty, $raw:ty) => {
        impl_one!($struct, $enum, $scal, $raw);
        impl_one!($struct, $enum, [$scal], $raw);
    };
}

macro_rules! impl_one {
    ($struct:ident, $enum:path, $ty:ty, $raw:ty) => {
        impl Request for $struct<$ty> {
            type Type = $ty;
        }
        impl AnyRequest for $struct<$ty> {
            type Raw = $raw;
            const ENUM: DbRequest = $enum(<<$ty as Type>::Element as Scalar>::ENUM);
        }
        impl ReadRequest for $struct<$ty> {}
    };
}

#[repr(C)]
pub struct Sts<T: Type + ?Sized> {
    pub alarm: Alarm,
    _value: MaybeUninit<T::Element>,
    _unsized: PhantomData<T>,
}

impl_all!(Sts, DbRequest::Sts, u8, sys::dbr_sts_char);
impl_all!(Sts, DbRequest::Sts, i16, sys::dbr_sts_short);
impl_all!(Sts, DbRequest::Sts, EpicsEnum, sys::dbr_sts_enum);
impl_all!(Sts, DbRequest::Sts, i32, sys::dbr_sts_long);
impl_all!(Sts, DbRequest::Sts, f32, sys::dbr_sts_float);
impl_all!(Sts, DbRequest::Sts, f64, sys::dbr_sts_double);
impl_all!(Sts, DbRequest::Sts, EpicsString, sys::dbr_sts_string);

#[repr(C)]
pub struct Time<T: Type + ?Sized> {
    pub alarm: Alarm,
    pub stamp: EpicsTimeStamp,
    _value: MaybeUninit<T::Element>,
}

impl_all!(Time, DbRequest::Time, u8, sys::dbr_time_char);
impl_all!(Time, DbRequest::Time, i16, sys::dbr_time_short);
impl_all!(Time, DbRequest::Time, EpicsEnum, sys::dbr_time_enum);
impl_all!(Time, DbRequest::Time, i32, sys::dbr_time_long);
impl_all!(Time, DbRequest::Time, f32, sys::dbr_time_float);
impl_all!(Time, DbRequest::Time, f64, sys::dbr_time_double);
impl_all!(Time, DbRequest::Time, EpicsString, sys::dbr_time_string);

#[repr(C)]
pub struct GrInt<T: Type + ?Sized>
where
    T::Element: Primitive,
{
    pub alarm: Alarm,
    pub units: Units,
    pub upper_disp_limit: T::Element,
    pub lower_disp_limit: T::Element,
    pub upper_alarm_limit: T::Element,
    pub upper_warning_limit: T::Element,
    pub lower_warning_limit: T::Element,
    pub lower_alarm_limit: T::Element,
    _value: MaybeUninit<T::Element>,
}

impl_all!(GrInt, DbRequest::Gr, u8, sys::dbr_gr_char);
impl_all!(GrInt, DbRequest::Gr, i16, sys::dbr_gr_short);
impl_all!(GrInt, DbRequest::Gr, i32, sys::dbr_gr_long);

#[repr(C)]
pub struct GrFloat<T: Type + ?Sized>
where
    T::Element: Primitive,
{
    pub alarm: Alarm,
    pub precision: i16,
    _padding: u16,
    pub units: Units,
    pub upper_disp_limit: T::Element,
    pub lower_disp_limit: T::Element,
    pub upper_alarm_limit: T::Element,
    pub upper_warning_limit: T::Element,
    pub lower_warning_limit: T::Element,
    pub lower_alarm_limit: T::Element,
    _value: MaybeUninit<T::Element>,
}

impl_all!(GrFloat, DbRequest::Gr, f32, sys::dbr_gr_float);
impl_all!(GrFloat, DbRequest::Gr, f64, sys::dbr_gr_double);

#[repr(C)]
pub struct GrEnum<T: Type<Element = EpicsEnum> + ?Sized> {
    pub alarm: Alarm,
    pub no_str: u16,
    pub strs: [StaticCString<MAX_ENUM_STRING_SIZE>; MAX_ENUM_STATES],
    _value: MaybeUninit<T::Element>,
}

impl_all!(GrEnum, DbRequest::Ctrl, EpicsEnum, sys::dbr_ctrl_enum);

#[repr(C)]
pub struct CtrlInt<T: Type + ?Sized>
where
    T::Element: Primitive,
{
    pub alarm: Alarm,
    pub units: Units,
    pub upper_disp_limit: T::Element,
    pub lower_disp_limit: T::Element,
    pub upper_alarm_limit: T::Element,
    pub upper_warning_limit: T::Element,
    pub lower_warning_limit: T::Element,
    pub lower_alarm_limit: T::Element,
    pub upper_ctrl_limit: T::Element,
    pub lower_ctrl_limit: T::Element,
    _value: MaybeUninit<T::Element>,
}

impl_all!(CtrlInt, DbRequest::Ctrl, u8, sys::dbr_ctrl_char);
impl_all!(CtrlInt, DbRequest::Ctrl, i16, sys::dbr_ctrl_short);
impl_all!(CtrlInt, DbRequest::Ctrl, i32, sys::dbr_ctrl_long);

#[repr(C)]
pub struct CtrlFloat<T: Type + ?Sized>
where
    T::Element: Primitive,
{
    pub alarm: Alarm,
    pub precision: i16,
    _padding: u16,
    pub units: Units,
    pub upper_disp_limit: T::Element,
    pub lower_disp_limit: T::Element,
    pub upper_alarm_limit: T::Element,
    pub upper_warning_limit: T::Element,
    pub lower_warning_limit: T::Element,
    pub lower_alarm_limit: T::Element,
    pub upper_ctrl_limit: T::Element,
    pub lower_ctrl_limit: T::Element,
    _value: MaybeUninit<T::Element>,
}

impl_all!(CtrlFloat, DbRequest::Ctrl, f32, sys::dbr_ctrl_float);
impl_all!(CtrlFloat, DbRequest::Ctrl, f64, sys::dbr_ctrl_double);

#[repr(C)]
pub struct CtrlEnum<T: Type<Element = EpicsEnum> + ?Sized> {
    pub alarm: Alarm,
    pub no_str: u16,
    pub strs: [StaticCString<MAX_ENUM_STRING_SIZE>; MAX_ENUM_STATES],
    _value: MaybeUninit<T::Element>,
}

impl_all!(CtrlEnum, DbRequest::Ctrl, EpicsEnum, sys::dbr_ctrl_enum);
