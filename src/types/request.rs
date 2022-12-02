use super::{
    time_from_epics, Alarm, AlarmCondition, AlarmSeverity, EpicsEnum, EpicsString, Scalar, Type,
};
use std::time::SystemTime;

trait Request {
    type Raw;
    unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize);
}

impl<T: Type + ?Sized> Request for T {
    type Raw = <<Self as Type>::Element as Scalar>::Raw;
    unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
        <<Self as Type>::Element as Scalar>::copy_data(
            raw,
            this as *mut <Self as Type>::Element,
            count,
        );
    }
}

pub struct Time<T: Type + ?Sized> {
    alarm: Alarm,
    stamp: SystemTime,
    value: T,
}

macro_rules! make_time {
    ($ty:ty, $raw:ty) => {
        impl Request for Time<$ty> {
            type Raw = $raw;
            unsafe fn load_raw(this: *mut Self, raw: *const Self::Raw, count: usize) {
                (*this).alarm = Alarm {
                    condition: AlarmCondition::try_from_raw((*raw).status as _).unwrap(),
                    severity: AlarmSeverity::try_from_raw((*raw).severity as _).unwrap(),
                };
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
