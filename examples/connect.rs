use async_std::task::sleep;
use c_str_macro::c_str;
use epics_ca::{AnyChannel, Context};
use std::{sync::Arc, time::Duration};

#[async_std::main]
async fn main() {
    let ctx = Arc::new(Context::new().unwrap());
    let mut chan = AnyChannel::new(ctx, c_str!("ca:test:ai")).unwrap();
    chan.connected().await;
    println!("Connected");
    sleep(Duration::from_secs(10)).await;
}
