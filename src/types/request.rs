use super::{
    time_from_epics, Alarm, AlarmCondition, AlarmSeverity, DbRequest, EpicsEnum, EpicsString,
    Primitive, Scalar, StaticCString, Type,
};
use std::time::SystemTime;

trait Request {
    type Raw;
    const ENUM: DbRequest;
    unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize);
}

impl<T: Type + ?Sized> Request for T {
    type Raw = <<Self as Type>::Element as Scalar>::Raw;
    const ENUM: DbRequest = DbRequest::Base(<<Self as Type>::Element as Scalar>::ENUM);
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

pub struct Sts<T: Type + ?Sized> {
    alarm: Alarm,
    value: T,
}

macro_rules! make_sts {
    ($ty:ty, $raw:ty) => {
        impl Request for Sts<$ty> {
            type Raw = $raw;
            const ENUM: DbRequest = DbRequest::Sts(<<$ty as Type>::Element as Scalar>::ENUM);
            unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
                (*this).alarm = make_alarm!(raw);
                <$ty>::copy_data(
                    &(*raw).value as *const _,
                    &mut (*this).value as *mut _,
                    count,
                );
            }
        }
    };
}

make_sts!(u8, sys::dbr_sts_char);
make_sts!(i16, sys::dbr_sts_short);
make_sts!(EpicsEnum, sys::dbr_sts_enum);
make_sts!(i32, sys::dbr_sts_long);
make_sts!(f32, sys::dbr_sts_float);
make_sts!(f64, sys::dbr_sts_double);
make_sts!(EpicsString, sys::dbr_sts_string);

pub struct Time<T: Type + ?Sized> {
    alarm: Alarm,
    stamp: SystemTime,
    value: T,
}

macro_rules! make_time {
    ($ty:ty, $raw:ty) => {
        impl Request for Time<$ty> {
            type Raw = $raw;
            const ENUM: DbRequest = DbRequest::Time(<<$ty as Type>::Element as Scalar>::ENUM);
            unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
                (*this).alarm = make_alarm!(raw);
                (*this).stamp = time_from_epics((*raw).stamp);
                <$ty>::copy_data(
                    &(*raw).value as *const _,
                    &mut (*this).value as *mut _,
                    count,
                );
            }
        }
    };
}

make_time!(u8, sys::dbr_time_char);
make_time!(i16, sys::dbr_time_short);
make_time!(EpicsEnum, sys::dbr_time_enum);
make_time!(i32, sys::dbr_time_long);
make_time!(f32, sys::dbr_time_float);
make_time!(f64, sys::dbr_time_double);
make_time!(EpicsString, sys::dbr_time_string);

pub struct Gr<T: Type + ?Sized>
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

macro_rules! make_gr {
    ($ty:ty, $raw:ty) => {
        impl Request for Gr<$ty> {
            type Raw = $raw;
            const ENUM: DbRequest = DbRequest::Gr(<<$ty as Type>::Element as Scalar>::ENUM);
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
                <$ty>::copy_data(
                    &(*raw).value as *const _,
                    &mut (*this).value as *mut _,
                    count,
                );
            }
        }
    };
}

make_gr!(u8, sys::dbr_gr_char);
make_gr!(i16, sys::dbr_gr_short);
make_gr!(i32, sys::dbr_gr_long);
make_gr!(f32, sys::dbr_gr_float);
make_gr!(f64, sys::dbr_gr_double);

pub struct EnumGr<T: Type<Element = EpicsEnum> + ?Sized> {
    alarm: Alarm,
    no_str: <EpicsEnum as Scalar>::Raw,
    strs: [StaticCString<26>; 16],
    value: T,
}

impl<T: Type<Element = EpicsEnum> + ?Sized> Request for EnumGr<T> {
    type Raw = sys::dbr_gr_enum;
    const ENUM: DbRequest = DbRequest::Gr(<<EpicsEnum as Type>::Element as Scalar>::ENUM);
    unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
        (*this).alarm = make_alarm!(raw);
        (*this).no_str = (*raw).no_str as u16;
        (*this).strs = (*raw).strs.map(|s| StaticCString::from_array(s).unwrap());
        EpicsEnum::copy_data(
            &(*raw).value as *const _,
            &mut (*this).value as *mut T as *mut _,
            count,
        );
    }
}
