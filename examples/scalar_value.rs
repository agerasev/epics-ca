use cstr::cstr;
use epics_ca::Context;
use std::f64::consts::PI;

#[async_std::main]
async fn main() {
    let ctx = Context::new().unwrap();

    let name = cstr!("ca:test:ao");
    let mut channel = ctx.connect::<f64>(name).await.unwrap();
    println!("Connected to {:?}", name);

    {
        let value = PI;
        channel.put(value).unwrap().await.unwrap();
        println!("Put {}", value);
    }

    {
        let value = channel.get().await.unwrap();
        println!("Got {}", value);
        assert_eq!(value, PI);
    }
}
