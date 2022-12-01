use crate::{
    types::{DbField, EpicsString, Type},
    Channel, Context,
};
use async_std::test as async_test;
use c_str_macro::c_str;
use serial_test::serial;
use std::{
    f64::consts::{E, PI},
    ffi::CStr,
    sync::Arc,
};

async fn connect_and_check<T: Type + ?Sized>(
    ctx: Arc<Context>,
    name: &CStr,
    dbf: DbField,
    count: usize,
) -> Channel<T> {
    let chan = ctx.connect(name).await.unwrap();
    assert_eq!(chan.name(), name);
    assert_eq!(chan.field_type().unwrap(), dbf);
    assert_eq!(chan.element_count().unwrap(), count);
    chan.into_typed::<T>().unwrap()
}

#[async_test]
#[serial]
async fn analog() {
    let ctx = Context::new().unwrap();
    let mut output =
        connect_and_check::<f64>(ctx.clone(), c_str!("ca:test:ao"), DbField::Double, 1).await;
    let mut input =
        connect_and_check::<f64>(ctx.clone(), c_str!("ca:test:ai"), DbField::Double, 1).await;

    output.put(&E).unwrap().await.unwrap();
    assert_eq!(input.get_copy().await.unwrap(), E);

    output.put(&PI).unwrap().await.unwrap();
    assert_eq!(input.get_copy().await.unwrap(), PI);
}

#[async_test]
#[serial]
async fn binary() {
    let ctx = Context::new().unwrap();
    let mut output =
        connect_and_check::<i16>(ctx.clone(), c_str!("ca:test:bo"), DbField::Enum, 1).await;
    let mut input =
        connect_and_check::<i16>(ctx.clone(), c_str!("ca:test:bi"), DbField::Enum, 1).await;

    output.put(&1).unwrap().await.unwrap();
    assert_eq!(input.get_copy().await.unwrap(), 1);

    output.put(&0).unwrap().await.unwrap();
    assert_eq!(input.get_copy().await.unwrap(), 0);
}

#[async_test]
#[serial]
async fn string() {
    let ctx = Context::new().unwrap();
    let mut _output = connect_and_check::<EpicsString>(
        ctx.clone(),
        c_str!("ca:test:stringout"),
        DbField::String,
        1,
    )
    .await;
    let mut _input = connect_and_check::<EpicsString>(
        ctx.clone(),
        c_str!("ca:test:stringin"),
        DbField::String,
        1,
    )
    .await;
}

#[async_test]
#[serial]
async fn array() {
    let ctx = Context::new().unwrap();
    let max_len = 64;
    let mut output =
        connect_and_check::<[i32]>(ctx.clone(), c_str!("ca:test:aao"), DbField::Long, max_len)
            .await;
    let mut input =
        connect_and_check::<[i32]>(ctx.clone(), c_str!("ca:test:aai"), DbField::Long, max_len)
            .await;
    let mut nord =
        connect_and_check::<f64>(ctx.clone(), c_str!("ca:test:aai.NORD"), DbField::Double, 1).await;

    let data = (0..42).collect::<Vec<_>>();
    output.put(&data).unwrap().await.unwrap();
    let len = nord.get_copy().await.unwrap() as usize;
    assert_eq!(len, data.len());
    assert_eq!(input.get_vec().await.unwrap()[..len], data);

    let data = (-64..0).collect::<Vec<_>>();
    output.put(&data).unwrap().await.unwrap();
    assert_eq!(input.get_vec().await.unwrap(), data);
}
