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
    fn load_raw(&mut self, raw: &Self::Raw, count: usize) {
        unsafe {
            <<Self as Type>::Element as Scalar>::copy_data(
                raw as *const _,
                self as *mut _ as *mut <Self as Type>::Element,
                count,
            );
        }
    }
}

macro_rules! make_alarm {
    ($raw:expr) => {
        Alarm {
            condition: AlarmCondition::try_from_raw($raw.status as _).unwrap(),
            severity: AlarmSeverity::try_from_raw($raw.severity as _).unwrap(),
        }
    };
}

macro_rules! copy_value {
    ($ty:ty, $self:expr, $raw:expr, $count:expr) => {
        unsafe {
            <<$ty as Type>::Element as Scalar>::copy_data(
                &$raw.value as *const <<$ty as Type>::Element as Scalar>::Raw,
                &mut $self.value as *mut $ty as *mut <$ty as Type>::Element,
                $count,
            );
        }
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
            fn load_raw(&mut self, raw: &Self::Raw, count: usize) {
                self.alarm = make_alarm!(raw);
                copy_value!($ty, self, raw, count);
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
    pub alarm: Alarm,
    pub stamp: SystemTime,
    pub value: T,
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
            fn load_raw(&mut self, raw: &Self::Raw, count: usize) {
                self.alarm = make_alarm!(raw);
                self.stamp = time_from_epics(raw.stamp);
                copy_value!($ty, self, raw, count);
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
    pub alarm: Alarm,
    pub units: StaticCString<8>,
    pub upper_disp_limit: T::Element,
    pub lower_disp_limit: T::Element,
    pub upper_alarm_limit: T::Element,
    pub upper_warning_limit: T::Element,
    pub lower_warning_limit: T::Element,
    pub lower_alarm_limit: T::Element,
    pub value: T,
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
            fn load_raw(&mut self, raw: &Self::Raw, count: usize) {
                self.alarm = make_alarm!(raw);
                self.units = StaticCString::from_array(raw.units).unwrap();
                self.upper_disp_limit = <$ty as Type>::Element::from_raw(raw.upper_disp_limit);
                self.lower_disp_limit = <$ty as Type>::Element::from_raw(raw.lower_disp_limit);
                self.upper_alarm_limit = <$ty as Type>::Element::from_raw(raw.upper_alarm_limit);
                self.upper_warning_limit =
                    <$ty as Type>::Element::from_raw(raw.upper_warning_limit);
                self.lower_warning_limit =
                    <$ty as Type>::Element::from_raw(raw.lower_warning_limit);
                self.lower_alarm_limit = <$ty as Type>::Element::from_raw(raw.lower_alarm_limit);
                copy_value!($ty, self, raw, count);
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
    pub alarm: Alarm,
    pub precision: i16,
    pub units: StaticCString<8>,
    pub upper_disp_limit: T::Element,
    pub lower_disp_limit: T::Element,
    pub upper_alarm_limit: T::Element,
    pub upper_warning_limit: T::Element,
    pub lower_warning_limit: T::Element,
    pub lower_alarm_limit: T::Element,
    pub value: T,
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
            fn load_raw(&mut self, raw: &Self::Raw, count: usize) {
                self.alarm = make_alarm!(raw);
                self.precision = raw.precision;
                self.units = StaticCString::from_array(raw.units).unwrap();
                self.upper_disp_limit = <$ty as Type>::Element::from_raw(raw.upper_disp_limit);
                self.lower_disp_limit = <$ty as Type>::Element::from_raw(raw.lower_disp_limit);
                self.upper_alarm_limit = <$ty as Type>::Element::from_raw(raw.upper_alarm_limit);
                self.upper_warning_limit =
                    <$ty as Type>::Element::from_raw(raw.upper_warning_limit);
                self.lower_warning_limit =
                    <$ty as Type>::Element::from_raw(raw.lower_warning_limit);
                self.lower_alarm_limit = <$ty as Type>::Element::from_raw(raw.lower_alarm_limit);
                copy_value!($ty, self, raw, count);
            }
        }
    };
}

impl_all!(make_float_gr, f32, sys::dbr_gr_float);
impl_all!(make_float_gr, f64, sys::dbr_gr_double);

pub struct EnumGr<T: Type<Element = EpicsEnum> + ?Sized> {
    pub alarm: Alarm,
    pub no_str: <EpicsEnum as Scalar>::Raw,
    pub strs: [StaticCString<26>; 16],
    pub value: T,
}

impl<T: Type<Element = EpicsEnum> + ?Sized> Request for EnumGr<T> {
    type Type = T;
}
impl<T: Type<Element = EpicsEnum> + ?Sized> AnyRequest for EnumGr<T> {
    type Raw = sys::dbr_gr_enum;
    const ENUM: DbRequest = DbRequest::Gr(<<EpicsEnum as Type>::Element as Scalar>::ENUM);
}
impl<T: Type<Element = EpicsEnum> + ?Sized> ReadRequest for EnumGr<T> {
    fn load_raw(&mut self, raw: &Self::Raw, count: usize) {
        self.alarm = make_alarm!(raw);
        self.no_str = raw.no_str as u16;
        self.strs = raw.strs.map(|s| StaticCString::from_array(s).unwrap());
        copy_value!(T, self, raw, count);
    }
}

pub struct IntCtrl<T: Type + ?Sized>
where
    T::Element: Primitive,
{
    pub alarm: Alarm,
    pub units: StaticCString<8>,
    pub upper_disp_limit: T::Element,
    pub lower_disp_limit: T::Element,
    pub upper_alarm_limit: T::Element,
    pub upper_warning_limit: T::Element,
    pub lower_warning_limit: T::Element,
    pub lower_alarm_limit: T::Element,
    pub upper_ctrl_limit: T::Element,
    pub lower_ctrl_limit: T::Element,
    pub value: T,
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
            fn load_raw(&mut self, raw: &Self::Raw, count: usize) {
                self.alarm = make_alarm!(raw);
                self.units = StaticCString::from_array(raw.units).unwrap();
                self.upper_disp_limit = <$ty as Type>::Element::from_raw(raw.upper_disp_limit);
                self.lower_disp_limit = <$ty as Type>::Element::from_raw(raw.lower_disp_limit);
                self.upper_alarm_limit = <$ty as Type>::Element::from_raw(raw.upper_alarm_limit);
                self.upper_warning_limit =
                    <$ty as Type>::Element::from_raw(raw.upper_warning_limit);
                self.lower_warning_limit =
                    <$ty as Type>::Element::from_raw(raw.lower_warning_limit);
                self.lower_alarm_limit = <$ty as Type>::Element::from_raw(raw.lower_alarm_limit);
                self.upper_ctrl_limit = <$ty as Type>::Element::from_raw(raw.upper_ctrl_limit);
                self.lower_ctrl_limit = <$ty as Type>::Element::from_raw(raw.lower_ctrl_limit);
                copy_value!($ty, self, raw, count);
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
    pub alarm: Alarm,
    pub precision: i16,
    pub units: StaticCString<8>,
    pub upper_disp_limit: T::Element,
    pub lower_disp_limit: T::Element,
    pub upper_alarm_limit: T::Element,
    pub upper_warning_limit: T::Element,
    pub lower_warning_limit: T::Element,
    pub lower_alarm_limit: T::Element,
    pub upper_ctrl_limit: T::Element,
    pub lower_ctrl_limit: T::Element,
    pub value: T,
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
            fn load_raw(&mut self, raw: &Self::Raw, count: usize) {
                self.alarm = make_alarm!(raw);
                self.precision = raw.precision;
                self.units = StaticCString::from_array(raw.units).unwrap();
                self.upper_disp_limit = <$ty as Type>::Element::from_raw(raw.upper_disp_limit);
                self.lower_disp_limit = <$ty as Type>::Element::from_raw(raw.lower_disp_limit);
                self.upper_alarm_limit = <$ty as Type>::Element::from_raw(raw.upper_alarm_limit);
                self.upper_warning_limit =
                    <$ty as Type>::Element::from_raw(raw.upper_warning_limit);
                self.lower_warning_limit =
                    <$ty as Type>::Element::from_raw(raw.lower_warning_limit);
                self.lower_alarm_limit = <$ty as Type>::Element::from_raw(raw.lower_alarm_limit);
                self.upper_ctrl_limit = <$ty as Type>::Element::from_raw(raw.upper_ctrl_limit);
                self.lower_ctrl_limit = <$ty as Type>::Element::from_raw(raw.lower_ctrl_limit);
                copy_value!($ty, self, raw, count);
            }
        }
    };
}

impl_all!(make_float_ctrl, f32, sys::dbr_ctrl_float);
impl_all!(make_float_ctrl, f64, sys::dbr_ctrl_double);

pub struct EnumCtrl<T: Type<Element = EpicsEnum> + ?Sized> {
    pub alarm: Alarm,
    pub no_str: <EpicsEnum as Scalar>::Raw,
    pub strs: [StaticCString<26>; 16],
    pub value: T,
}

impl<T: Type<Element = EpicsEnum> + ?Sized> Request for EnumCtrl<T> {
    type Type = T;
}
impl<T: Type<Element = EpicsEnum> + ?Sized> AnyRequest for EnumCtrl<T> {
    type Raw = sys::dbr_ctrl_enum;
    const ENUM: DbRequest = DbRequest::Gr(<<EpicsEnum as Type>::Element as Scalar>::ENUM);
}
impl<T: Type<Element = EpicsEnum> + ?Sized> ReadRequest for EnumCtrl<T> {
    fn load_raw(&mut self, raw: &Self::Raw, count: usize) {
        self.alarm = make_alarm!(raw);
        self.no_str = raw.no_str as u16;
        self.strs = raw.strs.map(|s| StaticCString::from_array(s).unwrap());
        copy_value!(T, self, raw, count);
    }
}
