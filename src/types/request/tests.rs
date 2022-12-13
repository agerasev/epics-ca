use super::*;
use crate::types::{EpicsEnum, EpicsString};
use std::mem::{align_of, size_of};

fn assert_layout_sized<R: Request>() {
    assert_eq!(size_of::<R>(), size_of::<R::Raw>());
    assert_eq!(align_of::<R>(), align_of::<R::Raw>());
}

fn assert_layout_typed<R: TypedRequest + ?Sized>() {
    assert_eq!(size_of::<R::Scalar>(), size_of::<R::Raw>());
    assert_eq!(align_of::<R::Scalar>(), align_of::<R::Raw>());
}

#[test]
fn value() {
    assert_layout_typed::<[u8]>();
    assert_layout_typed::<[i16]>();
    assert_layout_typed::<[EpicsEnum]>();
    assert_layout_typed::<[i32]>();
    assert_layout_typed::<[f32]>();
    assert_layout_typed::<[f64]>();
    assert_layout_typed::<[EpicsString]>();
}

#[test]
fn sts() {
    assert_layout_typed::<Array<u8, Sts<u8>>>();
    assert_layout_typed::<Array<i16, Sts<i16>>>();
    assert_layout_typed::<Array<EpicsEnum, Sts<EpicsEnum>>>();
    assert_layout_typed::<Array<i32, Sts<i32>>>();
    assert_layout_typed::<Array<f32, Sts<f32>>>();
    assert_layout_typed::<Array<f64, Sts<f64>>>();
    assert_layout_typed::<Array<EpicsString, Sts<EpicsString>>>();
}

#[test]
fn time() {
    assert_layout_typed::<Array<u8, Time<u8>>>();
    assert_layout_typed::<Array<i16, Time<i16>>>();
    assert_layout_typed::<Array<EpicsEnum, Time<EpicsEnum>>>();
    assert_layout_typed::<Array<i32, Time<i32>>>();
    assert_layout_typed::<Array<f32, Time<f32>>>();
    assert_layout_typed::<Array<f64, Time<f64>>>();
    assert_layout_typed::<Array<EpicsString, Time<EpicsString>>>();
}

#[test]
fn gr() {
    assert_layout_typed::<Array<u8, GrInt<u8>>>();
    assert_layout_typed::<Array<i16, GrInt<i16>>>();
    assert_layout_typed::<Array<EpicsEnum, GrEnum>>();
    assert_layout_typed::<Array<i32, GrInt<i32>>>();
    assert_layout_typed::<Array<f32, GrFloat<f32>>>();
    assert_layout_typed::<Array<f64, GrFloat<f64>>>();
}

#[test]
fn ctrl() {
    assert_layout_typed::<Array<u8, CtrlInt<u8>>>();
    assert_layout_typed::<Array<i16, CtrlInt<i16>>>();
    assert_layout_typed::<Array<EpicsEnum, CtrlEnum>>();
    assert_layout_typed::<Array<i32, CtrlInt<i32>>>();
    assert_layout_typed::<Array<f32, CtrlFloat<f32>>>();
    assert_layout_typed::<Array<f64, CtrlFloat<f64>>>();
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
