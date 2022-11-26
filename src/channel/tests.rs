use crate::{types::DbField, AnyChannel, Context};
use async_std::test as async_test;
use c_str_macro::c_str;
use serial_test::serial;
use std::sync::Arc;

#[async_test]
#[serial]
async fn analog() {
    let name = c_str!("ca:test:ai");
    let chan = AnyChannel::connect(Arc::new(Context::new().unwrap()), name)
        .await
        .unwrap();
    assert_eq!(chan.name(), name);
    assert_eq!(chan.field_type().unwrap(), DbField::Double);
    assert_eq!(chan.element_count().unwrap(), 1);
    //assert_eq!(chan.host_name().unwrap(), c_str!("localhost:5064"));
}

#[async_test]
#[serial]
async fn binary() {
    let name = c_str!("ca:test:bi");
    let chan = AnyChannel::connect(Arc::new(Context::new().unwrap()), name)
        .await
        .unwrap();
    assert_eq!(chan.name(), name);
    assert_eq!(chan.field_type().unwrap(), DbField::Enum);
    assert_eq!(chan.element_count().unwrap(), 1);
}

#[async_test]
#[serial]
async fn string() {
    let name = c_str!("ca:test:stringin");
    let chan = AnyChannel::connect(Arc::new(Context::new().unwrap()), name)
        .await
        .unwrap();
    assert_eq!(chan.name(), name);
    assert_eq!(chan.field_type().unwrap(), DbField::String);
    assert_eq!(chan.element_count().unwrap(), 1);
}

#[async_test]
#[serial]
async fn array() {
    let name = c_str!("ca:test:aai");
    let chan = AnyChannel::connect(Arc::new(Context::new().unwrap()), name)
        .await
        .unwrap();
    assert_eq!(chan.name(), name);
    assert_eq!(chan.field_type().unwrap(), DbField::Long);
    assert_eq!(chan.element_count().unwrap(), 64);
}
