use super::{EpicsEnum, EpicsString, FieldId};
use std::{fmt::Debug, mem::MaybeUninit, ptr};

/// Field of the channel.
///
/// # Safety
///
/// Should be implemented only for types supported by channel access.
pub unsafe trait Field: Copy + Send + Sized + 'static + Debug {
    /// Raw field structure.
    type Raw: Copy + Send + Sized + 'static;
    /// Field type identifier.
    const ID: FieldId;

    type StsRaw: Copy + Send + Sized + 'static;
    type TimeRaw: Copy + Send + Sized + 'static;
    type GrRaw: Copy + Send + Sized + 'static;
    type CtrlRaw: Copy + Send + Sized + 'static;

    type __StsPad: Copy + Send + Sized + 'static + Debug;
    type __TimePad: Copy + Send + Sized + 'static + Debug;
    type __GrPad: Copy + Send + Sized + 'static + Debug;
    type __CtrlPad: Copy + Send + Sized + 'static + Debug;
}
/// Integral field.
pub trait Int: Field {}
// Floating-point field.
pub trait Float: Field {}

unsafe impl Field for u8 {
    type Raw = u8;
    const ID: FieldId = FieldId::Char;

    type StsRaw = sys::dbr_sts_char;
    type TimeRaw = sys::dbr_time_char;
    type GrRaw = sys::dbr_gr_char;
    type CtrlRaw = sys::dbr_ctrl_char;

    type __StsPad = [MaybeUninit<u8>; 1];
    type __TimePad = [MaybeUninit<u8>; 3];
    type __GrPad = [MaybeUninit<u8>; 1];
    type __CtrlPad = [MaybeUninit<u8>; 1];
}
impl Int for u8 {}

unsafe impl Field for i16 {
    type Raw = i16;
    const ID: FieldId = FieldId::Short;

    type StsRaw = sys::dbr_sts_short;
    type TimeRaw = sys::dbr_time_short;
    type GrRaw = sys::dbr_gr_short;
    type CtrlRaw = sys::dbr_ctrl_short;

    type __StsPad = ();
    type __TimePad = [MaybeUninit<u8>; 2];
    type __GrPad = ();
    type __CtrlPad = ();
}
impl Int for i16 {}

unsafe impl Field for EpicsEnum {
    type Raw = u16;
    const ID: FieldId = FieldId::Enum;

    type StsRaw = sys::dbr_sts_enum;
    type TimeRaw = sys::dbr_time_enum;
    type GrRaw = sys::dbr_gr_enum;
    type CtrlRaw = sys::dbr_ctrl_enum;

    type __StsPad = ();
    type __TimePad = [MaybeUninit<u8>; 2];
    type __GrPad = ();
    type __CtrlPad = ();
}

unsafe impl Field for i32 {
    type Raw = i32;
    const ID: FieldId = FieldId::Long;

    type StsRaw = sys::dbr_sts_long;
    type TimeRaw = sys::dbr_time_long;
    type GrRaw = sys::dbr_gr_long;
    type CtrlRaw = sys::dbr_ctrl_long;

    type __StsPad = ();
    type __TimePad = ();
    type __GrPad = ();
    type __CtrlPad = ();
}
impl Int for i32 {}

unsafe impl Field for f32 {
    type Raw = f32;
    const ID: FieldId = FieldId::Float;

    type StsRaw = sys::dbr_sts_float;
    type TimeRaw = sys::dbr_time_float;
    type GrRaw = sys::dbr_gr_float;
    type CtrlRaw = sys::dbr_ctrl_float;

    type __StsPad = ();
    type __TimePad = ();
    type __GrPad = ();
    type __CtrlPad = ();
}
impl Float for f32 {}

unsafe impl Field for f64 {
    type Raw = f64;
    const ID: FieldId = FieldId::Double;

    type StsRaw = sys::dbr_sts_double;
    type TimeRaw = sys::dbr_time_double;
    type GrRaw = sys::dbr_gr_double;
    type CtrlRaw = sys::dbr_ctrl_double;

    type __StsPad = [MaybeUninit<u8>; 4];
    type __TimePad = [MaybeUninit<u8>; 4];
    type __GrPad = ();
    type __CtrlPad = ();
}
impl Float for f64 {}

unsafe impl Field for EpicsString {
    type Raw = sys::epicsOldString;
    const ID: FieldId = FieldId::String;

    type StsRaw = sys::dbr_sts_string;
    type TimeRaw = sys::dbr_time_string;
    type GrRaw = sys::dbr_sts_string;
    type CtrlRaw = sys::dbr_sts_string;

    type __StsPad = ();
    type __TimePad = ();
    type __GrPad = ();
    type __CtrlPad = ();
}

/// Value of the channel.
///
/// Consists of single or many items.
///
/// # Safety
///
/// Should be implemented only for types that represented in memory as [Self::Item].
#[allow(clippy::len_without_is_empty)]
pub unsafe trait Value: Send + 'static {
    /// Type of the item.
    type Item: Field;

    /// Length of the value.
    fn len(&self) -> usize;
    /// Check that provided length allowed for `Self`.
    #[must_use]
    fn check_len(count: usize) -> bool;
    /// Create pointer (possibly wide) to value from raw pointer.
    fn cast_ptr(ptr: *const u8, len: usize) -> *const Self;
}

unsafe impl<T: Field> Value for T {
    type Item = T;

    fn len(&self) -> usize {
        1
    }
    fn check_len(count: usize) -> bool {
        count == 1
    }
    fn cast_ptr(ptr: *const u8, _: usize) -> *const Self {
        ptr as *const Self
    }
}

unsafe impl<T: Field> Value for [T] {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }
    fn check_len(_: usize) -> bool {
        true
    }
    fn cast_ptr(ptr: *const u8, len: usize) -> *const Self {
        ptr::slice_from_raw_parts(ptr as *const T, len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    fn assert_layout<T: Field>() {
        assert_eq!(size_of::<T>(), size_of::<T::Raw>());
        assert_eq!(align_of::<T>(), align_of::<T::Raw>());
    }

    #[test]
    fn layout() {
        assert_layout::<u8>();
        assert_layout::<i16>();
        assert_layout::<EpicsEnum>();
        assert_layout::<i32>();
        assert_layout::<f32>();
        assert_layout::<f64>();
        assert_layout::<EpicsString>();
    }
}
