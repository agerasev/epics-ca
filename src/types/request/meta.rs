use crate::types::{
    Alarm, EpicsEnum, EpicsString, EpicsTimeStamp, Field, Float, Int, RequestId, StaticCString,
    Value,
};
use std::mem::MaybeUninit;

pub const MAX_UNITS_SIZE: usize = sys::MAX_UNITS_SIZE as usize;
pub const MAX_ENUM_STRING_SIZE: usize = sys::MAX_ENUM_STRING_SIZE as usize;
pub const MAX_ENUM_STATES: usize = sys::MAX_ENUM_STATES as usize;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Default, PartialEq, PartialOrd, Ord)]
pub struct Units(pub StaticCString<MAX_UNITS_SIZE>);

pub trait Meta: Send + 'static {
    type Value: Value;
    type Raw: Copy + Send + Sized + 'static;
    const ENUM: RequestId;

    fn value(&self) -> &Self::Value;
    fn value_mut(&mut self) -> &mut Self::Value;
}

macro_rules! impl_methods {
    () => {
        fn value(&self) -> &V {
            &self.value
        }
        fn value_mut(&mut self) -> &mut V {
            &mut self.value
        }
    };
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Sts<V: Value> {
    pub alarm: Alarm,
    _value_padding: <V::Field as Field>::__StsPad,
    pub value: V,
}
impl<V: Value> Meta for Sts<V> {
    type Value = V;
    type Raw = <V::Field as Field>::StsRaw;
    const ENUM: RequestId = RequestId::Sts(V::Field::ENUM);
    impl_methods!();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Time<V: Value> {
    pub alarm: Alarm,
    pub stamp: EpicsTimeStamp,
    _value_padding: <V::Field as Field>::__TimePad,
    pub value: V,
}
impl<V: Value> Meta for Time<V> {
    type Value = V;
    type Raw = <V::Field as Field>::TimeRaw;
    const ENUM: RequestId = RequestId::Time(V::Field::ENUM);
    impl_methods!();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrInt<V: Value>
where
    V::Field: Int,
{
    pub alarm: Alarm,
    pub units: Units,
    pub upper_disp_limit: V::Field,
    pub lower_disp_limit: V::Field,
    pub upper_alarm_limit: V::Field,
    pub upper_warning_limit: V::Field,
    pub lower_warning_limit: V::Field,
    pub lower_alarm_limit: V::Field,
    _value_padding: <V::Field as Field>::__GrPad,
    pub value: V,
}
impl<V: Value> Meta for GrInt<V>
where
    V::Field: Int,
{
    type Value = V;
    type Raw = <V::Field as Field>::GrRaw;
    const ENUM: RequestId = RequestId::Gr(V::Field::ENUM);
    impl_methods!();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrFloat<V: Value>
where
    V::Field: Float,
{
    pub alarm: Alarm,
    pub precision: i16,
    _units_padding: [MaybeUninit<u8>; 2],
    pub units: Units,
    pub upper_disp_limit: V::Field,
    pub lower_disp_limit: V::Field,
    pub upper_alarm_limit: V::Field,
    pub upper_warning_limit: V::Field,
    pub lower_warning_limit: V::Field,
    pub lower_alarm_limit: V::Field,
    _value_padding: <V::Field as Field>::__GrPad,
    pub value: V,
}
impl<V: Value> Meta for GrFloat<V>
where
    V::Field: Float,
{
    type Value = V;
    type Raw = <V::Field as Field>::GrRaw;
    const ENUM: RequestId = RequestId::Gr(V::Field::ENUM);
    impl_methods!();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrEnum<V: Value<Field = EpicsEnum>> {
    pub alarm: Alarm,
    pub no_str: u16,
    pub strs: [StaticCString<MAX_ENUM_STRING_SIZE>; MAX_ENUM_STATES],
    _value_padding: <V::Field as Field>::__GrPad,
    pub value: V,
}
impl<V: Value<Field = EpicsEnum>> Meta for GrEnum<V> {
    type Value = V;
    type Raw = <V::Field as Field>::GrRaw;
    const ENUM: RequestId = RequestId::Sts(V::Field::ENUM);
    impl_methods!();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GrString<V: Value<Field = EpicsString>> {
    pub alarm: Alarm,
    _value_padding: <V::Field as Field>::__GrPad,
    pub value: V,
}
impl<V: Value<Field = EpicsString>> Meta for GrString<V> {
    type Value = V;
    type Raw = <V::Field as Field>::GrRaw;
    const ENUM: RequestId = RequestId::Gr(V::Field::ENUM);
    impl_methods!();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlInt<V: Value>
where
    V::Field: Int,
{
    pub alarm: Alarm,
    pub units: Units,
    pub upper_disp_limit: V::Field,
    pub lower_disp_limit: V::Field,
    pub upper_alarm_limit: V::Field,
    pub upper_warning_limit: V::Field,
    pub lower_warning_limit: V::Field,
    pub lower_alarm_limit: V::Field,
    pub upper_ctrl_limit: V::Field,
    pub lower_ctrl_limit: V::Field,
    _value_padding: <V::Field as Field>::__CtrlPad,
    pub value: V,
}
impl<V: Value> Meta for CtrlInt<V>
where
    V::Field: Int,
{
    type Value = V;
    type Raw = <V::Field as Field>::CtrlRaw;
    const ENUM: RequestId = RequestId::Ctrl(V::Field::ENUM);
    impl_methods!();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlFloat<V: Value>
where
    V::Field: Float,
{
    pub alarm: Alarm,
    pub precision: i16,
    _units_padding: MaybeUninit<u16>,
    pub units: Units,
    pub upper_disp_limit: V::Field,
    pub lower_disp_limit: V::Field,
    pub upper_alarm_limit: V::Field,
    pub upper_warning_limit: V::Field,
    pub lower_warning_limit: V::Field,
    pub lower_alarm_limit: V::Field,
    pub upper_ctrl_limit: V::Field,
    pub lower_ctrl_limit: V::Field,
    _value_padding: <V::Field as Field>::__CtrlPad,
    pub value: V,
}
impl<V: Value> Meta for CtrlFloat<V>
where
    V::Field: Float,
{
    type Value = V;
    type Raw = <V::Field as Field>::CtrlRaw;
    const ENUM: RequestId = RequestId::Ctrl(V::Field::ENUM);
    impl_methods!();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlEnum<V: Value<Field = EpicsEnum>> {
    pub alarm: Alarm,
    pub no_str: u16,
    pub strs: [StaticCString<MAX_ENUM_STRING_SIZE>; MAX_ENUM_STATES],
    _value_padding: <V::Field as Field>::__CtrlPad,
    pub value: V,
}
impl<V: Value<Field = EpicsEnum>> Meta for CtrlEnum<V> {
    type Value = V;
    type Raw = <V::Field as Field>::CtrlRaw;
    const ENUM: RequestId = RequestId::Sts(V::Field::ENUM);
    impl_methods!();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlString<V: Value<Field = EpicsString>> {
    pub alarm: Alarm,
    _value_padding: <V::Field as Field>::__CtrlPad,
    pub value: V,
}
impl<V: Value<Field = EpicsString>> Meta for CtrlString<V> {
    type Value = V;
    type Raw = <V::Field as Field>::CtrlRaw;
    const ENUM: RequestId = RequestId::Ctrl(V::Field::ENUM);
    impl_methods!();
}
