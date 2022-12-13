use super::*;
use crate::types::{EpicsEnum, EpicsString};
use std::mem::{align_of, size_of};

fn assert_layout_sized<R: Request>() {
    assert_eq!(size_of::<R>(), size_of::<R::Raw>());
    assert_eq!(align_of::<R>(), align_of::<R::Raw>());
}

fn assert_layout_typed<R: ScalarRequest>() {
    assert_eq!(size_of::<R>(), size_of::<<R::Array as Request>::Raw>());
    assert_eq!(align_of::<R>(), align_of::<<R::Array as Request>::Raw>());
}

#[test]
fn value() {
    assert_layout_typed::<u8>();
    assert_layout_typed::<i16>();
    assert_layout_typed::<EpicsEnum>();
    assert_layout_typed::<i32>();
    assert_layout_typed::<f32>();
    assert_layout_typed::<f64>();
    assert_layout_typed::<EpicsString>();
}

#[test]
fn sts() {
    assert_layout_typed::<Scalar<u8, Sts>>();
    assert_layout_typed::<Scalar<i16, Sts>>();
    assert_layout_typed::<Scalar<EpicsEnum, Sts>>();
    assert_layout_typed::<Scalar<i32, Sts>>();
    assert_layout_typed::<Scalar<f32, Sts>>();
    assert_layout_typed::<Scalar<f64, Sts>>();
    assert_layout_typed::<Scalar<EpicsString, Sts>>();
}

#[test]
fn time() {
    assert_layout_typed::<Scalar<u8, Time>>();
    assert_layout_typed::<Scalar<i16, Time>>();
    assert_layout_typed::<Scalar<EpicsEnum, Time>>();
    assert_layout_typed::<Scalar<i32, Time>>();
    assert_layout_typed::<Scalar<f32, Time>>();
    assert_layout_typed::<Scalar<f64, Time>>();
    assert_layout_typed::<Scalar<EpicsString, Time>>();
}

#[test]
fn gr() {
    assert_layout_typed::<Scalar<u8, GrInt<u8>>>();
    assert_layout_typed::<Scalar<i16, GrInt<i16>>>();
    assert_layout_typed::<Scalar<EpicsEnum, GrEnum>>();
    assert_layout_typed::<Scalar<i32, GrInt<i32>>>();
    assert_layout_typed::<Scalar<f32, GrFloat<f32>>>();
    assert_layout_typed::<Scalar<f64, GrFloat<f64>>>();
}

#[test]
fn ctrl() {
    assert_layout_typed::<Scalar<u8, CtrlInt<u8>>>();
    assert_layout_typed::<Scalar<i16, CtrlInt<i16>>>();
    assert_layout_typed::<Scalar<EpicsEnum, CtrlEnum>>();
    assert_layout_typed::<Scalar<i32, CtrlInt<i32>>>();
    assert_layout_typed::<Scalar<f32, CtrlFloat<f32>>>();
    assert_layout_typed::<Scalar<f64, CtrlFloat<f64>>>();
}

#[test]
fn put_ackt() {
    assert_layout_sized::<PutAckt>();
}

#[test]
fn put_acks() {
    assert_layout_sized::<PutAcks>();
}

#[test]
fn stsack_string() {
    assert_layout_sized::<StsackString>();
}

#[test]
fn class_name() {
    assert_layout_sized::<ClassName>();
}
