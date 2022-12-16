use crate::{
    types::{EpicsEnum, EpicsString},
    Context,
};
use async_std::test as async_test;
use c_str_macro::c_str;
use serial_test::serial;
use std::f64::consts::{E, PI};

#[async_test]
#[serial]
async fn analog() {
    let ctx = Context::new().unwrap();
    let mut output = ctx.connect::<f64>(c_str!("ca:test:ao")).await.unwrap();
    let mut input = ctx.connect::<f64>(c_str!("ca:test:ai")).await.unwrap();

    output.put(E).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), E);

    output.put(PI).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), PI);
}

#[async_test]
#[serial]
async fn binary() {
    let ctx = Context::new().unwrap();
    let mut output = ctx
        .connect::<EpicsEnum>(c_str!("ca:test:bo"))
        .await
        .unwrap();
    let mut input = ctx
        .connect::<EpicsEnum>(c_str!("ca:test:bi"))
        .await
        .unwrap();

    output.put(EpicsEnum(1)).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), EpicsEnum(1));

    output.put(EpicsEnum(0)).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), EpicsEnum(0));
}

#[async_test]
#[serial]
async fn string() {
    let ctx = Context::new().unwrap();
    let mut output = ctx
        .connect::<EpicsString>(c_str!("ca:test:stringout"))
        .await
        .unwrap();
    let mut input = ctx
        .connect::<EpicsString>(c_str!("ca:test:stringin"))
        .await
        .unwrap();

    let data = EpicsString::from_cstr(c_str!("abcdefghijklmnopqrstuvwxyz")).unwrap();
    output.put(data).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), data);

    let data = EpicsString::from_cstr(c_str!("0123456789abcdefghijABCDEFGHIJ!@#$%^&*(")).unwrap();
    output.put(data).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), data);
}

#[async_test]
#[serial]
async fn array() {
    let ctx = Context::new().unwrap();
    let max_len = 64;
    let mut output = ctx.connect::<[i32]>(c_str!("ca:test:aao")).await.unwrap();
    let mut input = ctx.connect::<[i32]>(c_str!("ca:test:aai")).await.unwrap();
    assert_eq!(output.element_count().unwrap(), max_len);
    assert_eq!(input.element_count().unwrap(), max_len);

    let data = (0..42).collect::<Vec<_>>();
    output.put_ref(&data).unwrap().await.unwrap();
    assert_eq!(Vec::from(input.get_boxed().await.unwrap()), data);

    let data = (-64..0).collect::<Vec<_>>();
    output.put_ref(&data).unwrap().await.unwrap();
    assert_eq!(Vec::from(input.get_boxed().await.unwrap()), data);

    output.put_ref(&[]).unwrap().await.unwrap();
    assert_eq!(Vec::from(input.get_boxed().await.unwrap()), []);
}
