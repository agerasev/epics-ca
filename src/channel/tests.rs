use crate::{
    types::{DbField, EpicsEnum, EpicsString, Scalar},
    ArrayChannel, Context, ScalarChannel,
};
use async_std::test as async_test;
use c_str_macro::c_str;
use serial_test::serial;
use std::{
    f64::consts::{E, PI},
    ffi::CStr,
    sync::Arc,
};

async fn connect_and_check<T: Scalar>(
    ctx: Arc<Context>,
    name: &CStr,
    dbf: DbField,
) -> ScalarChannel<T> {
    let chan = ctx.connect(name).await.unwrap();
    assert_eq!(chan.name(), name);
    assert_eq!(chan.field_type().unwrap(), dbf);
    assert_eq!(chan.element_count().unwrap(), 1);
    chan.into_typed::<T>().unwrap().into_scalar().unwrap()
}

#[async_test]
#[serial]
async fn analog() {
    let ctx = Context::new().unwrap();
    let mut output =
        connect_and_check::<f64>(ctx.clone(), c_str!("ca:test:ao"), DbField::Double).await;
    let mut input =
        connect_and_check::<f64>(ctx.clone(), c_str!("ca:test:ai"), DbField::Double).await;

    output.put(E).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), E);

    output.put(PI).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), PI);
}

#[async_test]
#[serial]
async fn binary() {
    let ctx = Context::new().unwrap();
    let mut output =
        connect_and_check::<EpicsEnum>(ctx.clone(), c_str!("ca:test:bo"), DbField::Enum).await;
    let mut input =
        connect_and_check::<EpicsEnum>(ctx.clone(), c_str!("ca:test:bi"), DbField::Enum).await;

    output.put(EpicsEnum(1)).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), EpicsEnum(1));

    output.put(EpicsEnum(0)).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), EpicsEnum(0));
}

#[async_test]
#[serial]
async fn string() {
    let ctx = Context::new().unwrap();
    let mut output =
        connect_and_check::<EpicsString>(ctx.clone(), c_str!("ca:test:stringout"), DbField::String)
            .await;
    let mut input =
        connect_and_check::<EpicsString>(ctx.clone(), c_str!("ca:test:stringin"), DbField::String)
            .await;

    let data = EpicsString::from_cstr(c_str!("abcdefghijklmnopqrstuvwxyz")).unwrap();
    output.put(data).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), data);

    let data = EpicsString::from_cstr(c_str!("0123456789abcdefghijABCDEFGHIJ!@#$%^&*(")).unwrap();
    output.put(data).unwrap().await.unwrap();
    assert_eq!(input.get().await.unwrap(), data);
}

async fn connect_and_check_array<T: Scalar>(
    ctx: Arc<Context>,
    name: &CStr,
    dbf: DbField,
    count: usize,
) -> ArrayChannel<T> {
    let chan = ctx.connect(name).await.unwrap();
    assert_eq!(chan.name(), name);
    assert_eq!(chan.field_type().unwrap(), dbf);
    assert_eq!(chan.element_count().unwrap(), count);
    chan.into_typed::<T>().unwrap().into_array().await.unwrap()
}

#[async_test]
#[serial]
async fn array() {
    let ctx = Context::new().unwrap();
    let max_len = 64;
    let mut output =
        connect_and_check_array::<i32>(ctx.clone(), c_str!("ca:test:aao"), DbField::Long, max_len)
            .await;
    let mut input =
        connect_and_check_array::<i32>(ctx.clone(), c_str!("ca:test:aai"), DbField::Long, max_len)
            .await;

    let data = (0..42).collect::<Vec<_>>();
    output.put(&data).unwrap().await.unwrap();
    assert_eq!(input.get_vec().await.unwrap(), data);

    let data = (-64..0).collect::<Vec<_>>();
    output.put(&data).unwrap().await.unwrap();
    assert_eq!(input.get_vec().await.unwrap(), data);
}
