use super::{ReadRequest, Request, WriteRequest};
use crate::{
    error::{self, Error},
    types::*,
};
use std::{
    alloc::{alloc, Layout},
    mem::{size_of, MaybeUninit},
    ptr,
};

pub const MAX_UNITS_SIZE: usize = sys::MAX_UNITS_SIZE as usize;
pub const MAX_ENUM_STRING_SIZE: usize = sys::MAX_ENUM_STRING_SIZE as usize;
pub const MAX_ENUM_STATES: usize = sys::MAX_ENUM_STATES as usize;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Units(pub StaticCString<MAX_UNITS_SIZE>);

/// Request that stores value of specific type (along with optional metadata).
pub trait TypedRequest: Request {
    type Value: Value + ?Sized;

    fn value(&self) -> &Self::Value;
    fn value_mut(&mut self) -> &mut Self::Value;
}

macro_rules! impl_request_methods {
    () => {
        fn len(&self) -> usize {
            self.value().len()
        }
        unsafe fn from_ptr<'a>(
            ptr: *const u8,
            dbr: RequestId,
            count: usize,
        ) -> Result<&'a Self, Error> {
            if dbr != Self::ID {
                Err(error::BADTYPE)
            } else if !V::check_len(count) {
                Err(error::BADCOUNT)
            } else {
                Ok(&*(V::cast_ptr(ptr, count) as *const Self))
            }
        }
        fn clone_boxed(&self) -> Box<Self> {
            unsafe {
                let ptr = alloc(Layout::for_value(self));
                let size = size_of::<Self::Raw>()
                    + size_of::<V::Item>() * (if self.len() == 0 { 0 } else { self.len() - 1 });
                ptr::copy_nonoverlapping(self as *const _ as *const u8, ptr, size);
                let this_ptr = V::cast_ptr(ptr, self.len()) as *mut Self;
                Box::from_raw(this_ptr)
            }
        }
    };
}
macro_rules! impl_typed_request {
    () => {
        type Value = V;

        fn value(&self) -> &Self::Value {
            &self.value
        }
        fn value_mut(&mut self) -> &mut Self::Value {
            &mut self.value
        }
    };
}

unsafe impl<V: Value + ?Sized> Request for V {
    type Raw = <V::Item as Field>::Raw;
    const ID: RequestId = RequestId::Base(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value + ?Sized> TypedRequest for V {
    type Value = V;
    fn value(&self) -> &V {
        self
    }
    fn value_mut(&mut self) -> &mut V {
        self
    }
}
impl<V: Value + ?Sized> ReadRequest for V {}
impl<V: Value + ?Sized> WriteRequest for V {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Sts<V: Value + ?Sized> {
    pub alarm: Alarm,
    _value_padding: <V::Item as Field>::__StsPad,
    pub value: V,
}
unsafe impl<V: Value + ?Sized> Request for Sts<V> {
    type Raw = <V::Item as Field>::StsRaw;
    const ID: RequestId = RequestId::Sts(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value + ?Sized> TypedRequest for Sts<V> {
    impl_typed_request!();
}
impl<V: Value + ?Sized> ReadRequest for Sts<V> {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct StsackString<V: Value<Item = EpicsString> + ?Sized> {
    pub alarm: Alarm,
    pub ackt: u16,
    pub acks: u16,
    pub value: V,
}
unsafe impl<V: Value<Item = EpicsString> + ?Sized> Request for StsackString<V> {
    type Raw = sys::dbr_stsack_string;
    const ID: RequestId = RequestId::StsackString;
    impl_request_methods!();
}
impl<V: Value<Item = EpicsString> + ?Sized> TypedRequest for StsackString<V> {
    impl_typed_request!();
}
impl<V: Value<Item = EpicsString> + ?Sized> ReadRequest for StsackString<V> {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Time<V: Value + ?Sized> {
    pub alarm: Alarm,
    pub stamp: EpicsTimeStamp,
    _value_padding: <V::Item as Field>::__TimePad,
    pub value: V,
}
unsafe impl<V: Value + ?Sized> Request for Time<V> {
    type Raw = <V::Item as Field>::TimeRaw;
    const ID: RequestId = RequestId::Time(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value + ?Sized> TypedRequest for Time<V> {
    impl_typed_request!();
}
impl<V: Value + ?Sized> ReadRequest for Time<V> {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrInt<V: Value + ?Sized>
where
    V::Item: Int,
{
    pub alarm: Alarm,
    pub units: Units,
    pub upper_disp_limit: V::Item,
    pub lower_disp_limit: V::Item,
    pub upper_alarm_limit: V::Item,
    pub upper_warning_limit: V::Item,
    pub lower_warning_limit: V::Item,
    pub lower_alarm_limit: V::Item,
    _value_padding: <V::Item as Field>::__GrPad,
    pub value: V,
}
unsafe impl<V: Value + ?Sized> Request for GrInt<V>
where
    V::Item: Int,
{
    type Raw = <V::Item as Field>::GrRaw;
    const ID: RequestId = RequestId::Gr(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value + ?Sized> TypedRequest for GrInt<V>
where
    V::Item: Int,
{
    impl_typed_request!();
}
impl<V: Value + ?Sized> ReadRequest for GrInt<V> where V::Item: Int {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrFloat<V: Value + ?Sized>
where
    V::Item: Float,
{
    pub alarm: Alarm,
    pub precision: i16,
    _units_padding: [MaybeUninit<u8>; 2],
    pub units: Units,
    pub upper_disp_limit: V::Item,
    pub lower_disp_limit: V::Item,
    pub upper_alarm_limit: V::Item,
    pub upper_warning_limit: V::Item,
    pub lower_warning_limit: V::Item,
    pub lower_alarm_limit: V::Item,
    _value_padding: <V::Item as Field>::__GrPad,
    pub value: V,
}
unsafe impl<V: Value + ?Sized> Request for GrFloat<V>
where
    V::Item: Float,
{
    type Raw = <V::Item as Field>::GrRaw;
    const ID: RequestId = RequestId::Gr(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value + ?Sized> TypedRequest for GrFloat<V>
where
    V::Item: Float,
{
    impl_typed_request!();
}
impl<V: Value + ?Sized> ReadRequest for GrFloat<V> where V::Item: Float {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrEnum<V: Value<Item = EpicsEnum> + ?Sized> {
    pub alarm: Alarm,
    pub no_str: u16,
    pub strs: [StaticCString<MAX_ENUM_STRING_SIZE>; MAX_ENUM_STATES],
    _value_padding: <V::Item as Field>::__GrPad,
    pub value: V,
}
unsafe impl<V: Value<Item = EpicsEnum> + ?Sized> Request for GrEnum<V> {
    type Raw = <V::Item as Field>::GrRaw;
    const ID: RequestId = RequestId::Sts(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value<Item = EpicsEnum> + ?Sized> TypedRequest for GrEnum<V> {
    impl_typed_request!();
}
impl<V: Value<Item = EpicsEnum> + ?Sized> ReadRequest for GrEnum<V> {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrString<V: Value<Item = EpicsString> + ?Sized> {
    pub alarm: Alarm,
    _value_padding: <V::Item as Field>::__GrPad,
    pub value: V,
}
unsafe impl<V: Value<Item = EpicsString> + ?Sized> Request for GrString<V> {
    type Raw = <V::Item as Field>::GrRaw;
    const ID: RequestId = RequestId::Gr(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value<Item = EpicsString> + ?Sized> TypedRequest for GrString<V> {
    impl_typed_request!();
}
impl<V: Value<Item = EpicsString> + ?Sized> ReadRequest for GrString<V> {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlInt<V: Value + ?Sized>
where
    V::Item: Int,
{
    pub alarm: Alarm,
    pub units: Units,
    pub upper_disp_limit: V::Item,
    pub lower_disp_limit: V::Item,
    pub upper_alarm_limit: V::Item,
    pub upper_warning_limit: V::Item,
    pub lower_warning_limit: V::Item,
    pub lower_alarm_limit: V::Item,
    pub upper_ctrl_limit: V::Item,
    pub lower_ctrl_limit: V::Item,
    _value_padding: <V::Item as Field>::__CtrlPad,
    pub value: V,
}
unsafe impl<V: Value + ?Sized> Request for CtrlInt<V>
where
    V::Item: Int,
{
    type Raw = <V::Item as Field>::CtrlRaw;
    const ID: RequestId = RequestId::Ctrl(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value + ?Sized> TypedRequest for CtrlInt<V>
where
    V::Item: Int,
{
    impl_typed_request!();
}
impl<V: Value + ?Sized> ReadRequest for CtrlInt<V> where V::Item: Int {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlFloat<V: Value + ?Sized>
where
    V::Item: Float,
{
    pub alarm: Alarm,
    pub precision: i16,
    _units_padding: MaybeUninit<u16>,
    pub units: Units,
    pub upper_disp_limit: V::Item,
    pub lower_disp_limit: V::Item,
    pub upper_alarm_limit: V::Item,
    pub upper_warning_limit: V::Item,
    pub lower_warning_limit: V::Item,
    pub lower_alarm_limit: V::Item,
    pub upper_ctrl_limit: V::Item,
    pub lower_ctrl_limit: V::Item,
    _value_padding: <V::Item as Field>::__CtrlPad,
    pub value: V,
}
unsafe impl<V: Value + ?Sized> Request for CtrlFloat<V>
where
    V::Item: Float,
{
    type Raw = <V::Item as Field>::CtrlRaw;
    const ID: RequestId = RequestId::Ctrl(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value + ?Sized> TypedRequest for CtrlFloat<V>
where
    V::Item: Float,
{
    impl_typed_request!();
}
impl<V: Value + ?Sized> ReadRequest for CtrlFloat<V> where V::Item: Float {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlEnum<V: Value<Item = EpicsEnum> + ?Sized> {
    pub alarm: Alarm,
    pub no_str: u16,
    pub strs: [StaticCString<MAX_ENUM_STRING_SIZE>; MAX_ENUM_STATES],
    _value_padding: <V::Item as Field>::__CtrlPad,
    pub value: V,
}
unsafe impl<V: Value<Item = EpicsEnum> + ?Sized> Request for CtrlEnum<V> {
    type Raw = <V::Item as Field>::CtrlRaw;
    const ID: RequestId = RequestId::Sts(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value<Item = EpicsEnum> + ?Sized> TypedRequest for CtrlEnum<V> {
    impl_typed_request!();
}
impl<V: Value<Item = EpicsEnum> + ?Sized> ReadRequest for CtrlEnum<V> {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlString<V: Value<Item = EpicsString> + ?Sized> {
    pub alarm: Alarm,
    _value_padding: <V::Item as Field>::__CtrlPad,
    pub value: V,
}
unsafe impl<V: Value<Item = EpicsString> + ?Sized> Request for CtrlString<V> {
    type Raw = <V::Item as Field>::CtrlRaw;
    const ID: RequestId = RequestId::Ctrl(<V::Item as Field>::ID);
    impl_request_methods!();
}
impl<V: Value<Item = EpicsString> + ?Sized> TypedRequest for CtrlString<V> {
    impl_typed_request!();
}
impl<V: Value<Item = EpicsString> + ?Sized> ReadRequest for CtrlString<V> {}
