use cstr::cstr;
use epics_ca::{
    request::{CtrlFloat, Time},
    Channel, Context,
};
use std::f64::consts::PI;

#[async_std::main]
async fn main() {
    let ctx = Context::new().unwrap();

    let name = cstr!("ca:test:ao");
    let mut channel = Channel::new(&ctx, name).unwrap();
    channel.connected().await;
    let mut channel = channel.into_typed::<f64>().unwrap();
    println!("Connected to {:?}", name);

    {
        let value = PI;
        channel.put(value).unwrap().await.unwrap();
        println!("Put: {}", value);
    }

    {
        let value = channel.get::<f64>().await.unwrap();
        println!("Got: {}", value);
        assert_eq!(value, PI);
    }

    {
        let request = channel.get::<Time<f64>>().await.unwrap();
        println!("Got: {:?}", request);
    }

    {
        let request = channel.get::<CtrlFloat<f64>>().await.unwrap();
        println!("Got: {:?}", request);
    }
}
