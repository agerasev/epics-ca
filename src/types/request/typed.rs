use super::{AnyRequest, ReadRequest, WriteRequest};
use crate::types::{
    time_from_epics, Alarm, AlarmCondition, AlarmSeverity, DbRequest, EpicsEnum, EpicsString,
    Primitive, Scalar, StaticCString, Type,
};
use std::time::SystemTime;

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
unsafe impl<T: Type + ?Sized> WriteRequest for T {}
impl<T: Type + ?Sized> ReadRequest for T {
    unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
        <<Self as Type>::Element as Scalar>::copy_data(
            raw,
            this as *mut <Self as Type>::Element,
            count,
        );
    }
}

macro_rules! make_alarm {
    ($raw:expr) => {
        Alarm {
            condition: AlarmCondition::try_from_raw((*$raw).status as _).unwrap(),
            severity: AlarmSeverity::try_from_raw((*$raw).severity as _).unwrap(),
        }
    };
}

macro_rules! copy_value {
    ($ty:ty, $this:expr, $raw:expr, $count:expr) => {
        <<$ty as Type>::Element as Scalar>::copy_data(
            &(*$raw).value as *const <<$ty as Type>::Element as Scalar>::Raw,
            &mut (*$this).value as *mut $ty as *mut <$ty as Type>::Element,
            $count,
        );
    };
}

macro_rules! impl_all {
    ($macro:path, $scal:ty, $raw:ty) => {
        $macro!($scal, $raw);
        $macro!([$scal], $raw);
    };
}

pub struct Sts<T: Type + ?Sized> {
    alarm: Alarm,
    value: T,
}

macro_rules! make_sts {
    ($ty:ty, $raw:ty) => {
        impl Request for Sts<$ty> {
            type Type = $ty;
        }
        impl AnyRequest for Sts<$ty> {
            type Raw = $raw;
            const ENUM: DbRequest = DbRequest::Sts(<<$ty as Type>::Element as Scalar>::ENUM);
        }
        impl ReadRequest for Sts<$ty> {
            unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
                (*this).alarm = make_alarm!(raw);
                copy_value!($ty, this, raw, count);
            }
        }
    };
}

impl_all!(make_sts, u8, sys::dbr_sts_char);
impl_all!(make_sts, i16, sys::dbr_sts_short);
impl_all!(make_sts, EpicsEnum, sys::dbr_sts_enum);
impl_all!(make_sts, i32, sys::dbr_sts_long);
impl_all!(make_sts, f32, sys::dbr_sts_float);
impl_all!(make_sts, f64, sys::dbr_sts_double);
impl_all!(make_sts, EpicsString, sys::dbr_sts_string);

pub struct Time<T: Type + ?Sized> {
    alarm: Alarm,
    stamp: SystemTime,
    value: T,
}

macro_rules! make_time {
    ($ty:ty, $raw:ty) => {
        impl Request for Time<$ty> {
            type Type = $ty;
        }
        impl AnyRequest for Time<$ty> {
            type Raw = $raw;
            const ENUM: DbRequest = DbRequest::Time(<<$ty as Type>::Element as Scalar>::ENUM);
        }
        impl ReadRequest for Time<$ty> {
            unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
                (*this).alarm = make_alarm!(raw);
                (*this).stamp = time_from_epics((*raw).stamp);
                copy_value!($ty, this, raw, count);
            }
        }
    };
}

impl_all!(make_time, u8, sys::dbr_time_char);
impl_all!(make_time, i16, sys::dbr_time_short);
impl_all!(make_time, EpicsEnum, sys::dbr_time_enum);
impl_all!(make_time, i32, sys::dbr_time_long);
impl_all!(make_time, f32, sys::dbr_time_float);
impl_all!(make_time, f64, sys::dbr_time_double);
impl_all!(make_time, EpicsString, sys::dbr_time_string);

pub struct IntGr<T: Type + ?Sized>
where
    T::Element: Primitive,
{
    alarm: Alarm,
    units: StaticCString<8>,
    upper_disp_limit: T::Element,
    lower_disp_limit: T::Element,
    upper_alarm_limit: T::Element,
    upper_warning_limit: T::Element,
    lower_warning_limit: T::Element,
    lower_alarm_limit: T::Element,
    value: T,
}

macro_rules! make_int_gr {
    ($ty:ty, $raw:ty) => {
        impl Request for IntGr<$ty> {
            type Type = $ty;
        }
        impl AnyRequest for IntGr<$ty> {
            type Raw = $raw;
            const ENUM: DbRequest = DbRequest::Gr(<<$ty as Type>::Element as Scalar>::ENUM);
        }
        impl ReadRequest for IntGr<$ty> {
            unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
                (*this).alarm = make_alarm!(raw);
                (*this).units = StaticCString::from_array((*raw).units).unwrap();
                (*this).upper_disp_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_disp_limit);
                (*this).lower_disp_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_disp_limit);
                (*this).upper_alarm_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_alarm_limit);
                (*this).upper_warning_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_warning_limit);
                (*this).lower_warning_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_warning_limit);
                (*this).lower_alarm_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_alarm_limit);
                copy_value!($ty, this, raw, count);
            }
        }
    };
}

impl_all!(make_int_gr, u8, sys::dbr_gr_char);
impl_all!(make_int_gr, i16, sys::dbr_gr_short);
impl_all!(make_int_gr, i32, sys::dbr_gr_long);

pub struct FloatGr<T: Type + ?Sized>
where
    T::Element: Primitive,
{
    alarm: Alarm,
    precision: i16,
    units: StaticCString<8>,
    upper_disp_limit: T::Element,
    lower_disp_limit: T::Element,
    upper_alarm_limit: T::Element,
    upper_warning_limit: T::Element,
    lower_warning_limit: T::Element,
    lower_alarm_limit: T::Element,
    value: T,
}

macro_rules! make_float_gr {
    ($ty:ty, $raw:ty) => {
        impl Request for FloatGr<$ty> {
            type Type = $ty;
        }
        impl AnyRequest for FloatGr<$ty> {
            type Raw = $raw;
            const ENUM: DbRequest = DbRequest::Gr(<<$ty as Type>::Element as Scalar>::ENUM);
        }
        impl ReadRequest for FloatGr<$ty> {
            unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
                (*this).alarm = make_alarm!(raw);
                (*this).precision = (*raw).precision;
                (*this).units = StaticCString::from_array((*raw).units).unwrap();
                (*this).upper_disp_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_disp_limit);
                (*this).lower_disp_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_disp_limit);
                (*this).upper_alarm_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_alarm_limit);
                (*this).upper_warning_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_warning_limit);
                (*this).lower_warning_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_warning_limit);
                (*this).lower_alarm_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_alarm_limit);
                copy_value!($ty, this, raw, count);
            }
        }
    };
}

impl_all!(make_float_gr, f32, sys::dbr_gr_float);
impl_all!(make_float_gr, f64, sys::dbr_gr_double);

pub struct EnumGr<T: Type<Element = EpicsEnum> + ?Sized> {
    alarm: Alarm,
    no_str: <EpicsEnum as Scalar>::Raw,
    strs: [StaticCString<26>; 16],
    value: T,
}

impl<T: Type<Element = EpicsEnum> + ?Sized> Request for EnumGr<T> {
    type Type = T;
}
impl<T: Type<Element = EpicsEnum> + ?Sized> AnyRequest for EnumGr<T> {
    type Raw = sys::dbr_gr_enum;
    const ENUM: DbRequest = DbRequest::Gr(<<EpicsEnum as Type>::Element as Scalar>::ENUM);
}
impl<T: Type<Element = EpicsEnum> + ?Sized> ReadRequest for EnumGr<T> {
    unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
        (*this).alarm = make_alarm!(raw);
        (*this).no_str = (*raw).no_str as u16;
        (*this).strs = (*raw).strs.map(|s| StaticCString::from_array(s).unwrap());
        copy_value!(T, this, raw, count);
    }
}

pub struct IntCtrl<T: Type + ?Sized>
where
    T::Element: Primitive,
{
    alarm: Alarm,
    units: StaticCString<8>,
    upper_disp_limit: T::Element,
    lower_disp_limit: T::Element,
    upper_alarm_limit: T::Element,
    upper_warning_limit: T::Element,
    lower_warning_limit: T::Element,
    lower_alarm_limit: T::Element,
    upper_ctrl_limit: T::Element,
    lower_ctrl_limit: T::Element,
    value: T,
}

macro_rules! make_int_ctrl {
    ($ty:ty, $raw:ty) => {
        impl Request for IntCtrl<$ty> {
            type Type = $ty;
        }
        impl AnyRequest for IntCtrl<$ty> {
            type Raw = $raw;
            const ENUM: DbRequest = DbRequest::Gr(<<$ty as Type>::Element as Scalar>::ENUM);
        }
        impl ReadRequest for IntCtrl<$ty> {
            unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
                (*this).alarm = make_alarm!(raw);
                (*this).units = StaticCString::from_array((*raw).units).unwrap();
                (*this).upper_disp_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_disp_limit);
                (*this).lower_disp_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_disp_limit);
                (*this).upper_alarm_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_alarm_limit);
                (*this).upper_warning_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_warning_limit);
                (*this).lower_warning_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_warning_limit);
                (*this).lower_alarm_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_alarm_limit);
                (*this).upper_ctrl_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_ctrl_limit);
                (*this).lower_ctrl_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_ctrl_limit);
                copy_value!($ty, this, raw, count);
            }
        }
    };
}

impl_all!(make_int_ctrl, u8, sys::dbr_ctrl_char);
impl_all!(make_int_ctrl, i16, sys::dbr_ctrl_short);
impl_all!(make_int_ctrl, i32, sys::dbr_ctrl_long);

pub struct FloatCtrl<T: Type + ?Sized>
where
    T::Element: Primitive,
{
    alarm: Alarm,
    precision: i16,
    units: StaticCString<8>,
    upper_disp_limit: T::Element,
    lower_disp_limit: T::Element,
    upper_alarm_limit: T::Element,
    upper_warning_limit: T::Element,
    lower_warning_limit: T::Element,
    lower_alarm_limit: T::Element,
    upper_ctrl_limit: T::Element,
    lower_ctrl_limit: T::Element,
    value: T,
}

macro_rules! make_float_ctrl {
    ($ty:ty, $raw:ty) => {
        impl Request for FloatCtrl<$ty> {
            type Type = $ty;
        }
        impl AnyRequest for FloatCtrl<$ty> {
            type Raw = $raw;
            const ENUM: DbRequest = DbRequest::Gr(<<$ty as Type>::Element as Scalar>::ENUM);
        }
        impl ReadRequest for FloatCtrl<$ty> {
            unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
                (*this).alarm = make_alarm!(raw);
                (*this).precision = (*raw).precision;
                (*this).units = StaticCString::from_array((*raw).units).unwrap();
                (*this).upper_disp_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_disp_limit);
                (*this).lower_disp_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_disp_limit);
                (*this).upper_alarm_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_alarm_limit);
                (*this).upper_warning_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_warning_limit);
                (*this).lower_warning_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_warning_limit);
                (*this).lower_alarm_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_alarm_limit);
                (*this).upper_ctrl_limit =
                    <$ty as Type>::Element::from_raw((*raw).upper_ctrl_limit);
                (*this).lower_ctrl_limit =
                    <$ty as Type>::Element::from_raw((*raw).lower_ctrl_limit);
                copy_value!($ty, this, raw, count);
            }
        }
    };
}

impl_all!(make_float_ctrl, f32, sys::dbr_ctrl_float);
impl_all!(make_float_ctrl, f64, sys::dbr_ctrl_double);

pub struct EnumCtrl<T: Type<Element = EpicsEnum> + ?Sized> {
    alarm: Alarm,
    no_str: <EpicsEnum as Scalar>::Raw,
    strs: [StaticCString<26>; 16],
    value: T,
}

impl<T: Type<Element = EpicsEnum> + ?Sized> Request for EnumCtrl<T> {
    type Type = T;
}
impl<T: Type<Element = EpicsEnum> + ?Sized> AnyRequest for EnumCtrl<T> {
    type Raw = sys::dbr_ctrl_enum;
    const ENUM: DbRequest = DbRequest::Gr(<<EpicsEnum as Type>::Element as Scalar>::ENUM);
}
impl<T: Type<Element = EpicsEnum> + ?Sized> ReadRequest for EnumCtrl<T> {
    unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
        (*this).alarm = make_alarm!(raw);
        (*this).no_str = (*raw).no_str as u16;
        (*this).strs = (*raw).strs.map(|s| StaticCString::from_array(s).unwrap());
        copy_value!(T, this, raw, count);
    }
}
