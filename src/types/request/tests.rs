use super::*;
use crate::types::{EpicsEnum, EpicsString};
use std::mem::{align_of, size_of};

fn assert_layout<R: Request>() {
    assert_eq!(size_of::<R>(), size_of::<R::Raw>());
    assert_eq!(align_of::<R>(), align_of::<R::Raw>());
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
    assert_layout::<Scalar<u8, Sts>>();
    assert_layout::<Scalar<i16, Sts>>();
    assert_layout::<Scalar<EpicsEnum, Sts>>();
    assert_layout::<Scalar<i32, Sts>>();
    assert_layout::<Scalar<f32, Sts>>();
    assert_layout::<Scalar<f64, Sts>>();
    assert_layout::<Scalar<EpicsString, Sts>>();
}

#[test]
fn time() {
    assert_layout::<Scalar<u8, Time>>();
    assert_layout::<Scalar<i16, Time>>();
    assert_layout::<Scalar<EpicsEnum, Time>>();
    assert_layout::<Scalar<i32, Time>>();
    assert_layout::<Scalar<f32, Time>>();
    assert_layout::<Scalar<f64, Time>>();
    assert_layout::<Scalar<EpicsString, Time>>();
}

#[test]
fn gr() {
    assert_layout::<Scalar<u8, GrInt<u8>>>();
    assert_layout::<Scalar<i16, GrInt<i16>>>();
    assert_layout::<Scalar<EpicsEnum, GrEnum>>();
    assert_layout::<Scalar<i32, GrInt<i32>>>();
    assert_layout::<Scalar<f32, GrFloat<f32>>>();
    assert_layout::<Scalar<f64, GrFloat<f64>>>();
}

#[test]
fn ctrl() {
    assert_layout::<Scalar<u8, CtrlInt<u8>>>();
    assert_layout::<Scalar<i16, CtrlInt<i16>>>();
    assert_layout::<Scalar<EpicsEnum, CtrlEnum>>();
    assert_layout::<Scalar<i32, CtrlInt<i32>>>();
    assert_layout::<Scalar<f32, CtrlFloat<f32>>>();
    assert_layout::<Scalar<f64, CtrlFloat<f64>>>();
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
