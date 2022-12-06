use super::*;
use crate::types::{EpicsEnum, EpicsString};
use std::mem::{align_of, size_of};

fn assert_layout<T: AnyRequest>() {
    assert_eq!(size_of::<T>(), size_of::<T::Raw>());
    assert_eq!(align_of::<T>(), align_of::<T::Raw>());
}

#[test]
fn value() {
    assert_layout::<u8>();
    assert_layout::<i16>();
    assert_layout::<EpicsEnum>();
    assert_layout::<i32>();
    assert_layout::<f32>();
    assert_layout::<f64>();
    assert_layout::<EpicsString>();
}

#[test]
fn sts() {
    assert_layout::<Sts<u8>>();
    assert_layout::<Sts<i16>>();
    assert_layout::<Sts<EpicsEnum>>();
    assert_layout::<Sts<i32>>();
    assert_layout::<Sts<f32>>();
    assert_layout::<Sts<f64>>();
    assert_layout::<Sts<EpicsString>>();
}

#[test]
fn time() {
    assert_layout::<Time<u8>>();
    assert_layout::<Time<i16>>();
    assert_layout::<Time<EpicsEnum>>();
    assert_layout::<Time<i32>>();
    assert_layout::<Time<f32>>();
    assert_layout::<Time<f64>>();
    assert_layout::<Time<EpicsString>>();
}

#[test]
fn gr() {
    assert_layout::<GrInt<u8>>();
    assert_layout::<GrInt<i16>>();
    assert_layout::<GrEnum<EpicsEnum>>();
    assert_layout::<GrInt<i32>>();
    assert_layout::<GrFloat<f32>>();
    assert_layout::<GrFloat<f64>>();
}

#[test]
fn ctrl() {
    assert_layout::<CtrlInt<u8>>();
    assert_layout::<CtrlInt<i16>>();
    assert_layout::<CtrlEnum<EpicsEnum>>();
    assert_layout::<CtrlInt<i32>>();
    assert_layout::<CtrlFloat<f32>>();
    assert_layout::<CtrlFloat<f64>>();
}

#[test]
fn put_ackt() {
    assert_layout::<PutAckt>();
}

#[test]
fn put_acks() {
    assert_layout::<PutAcks>();
}

#[test]
fn stsack_string() {
    assert_layout::<StsackString>();
}

#[test]
fn class_name() {
    assert_layout::<ClassName>();
}
