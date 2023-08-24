use cstr::cstr;
use epics_ca::{
    request::{CtrlInt, Time},
    Channel, Context,
};

#[async_std::main]
async fn main() {
    let ctx = Context::new().unwrap();

    let name = cstr!("ca:test:aao");
    let mut channel = Channel::new(&ctx, name).unwrap();
    channel.connected().await;
    let mut channel = channel.into_typed::<[i32]>().unwrap();
    println!("Connected to {:?}", name);

    {
        let value = [0, 1, 2, 3];
        channel.put_ref::<[i32]>(&value).unwrap().await.unwrap();
        println!("Put {:?}", value);
    }

    {
        let value = channel.get_boxed::<[i32]>().await.unwrap().into_vec();
        println!("Got value {:?}", value);
        assert_eq!(value, [0, 1, 2, 3]);
    }

    {
        let request = channel.get_boxed::<Time<[i32]>>().await.unwrap();
        println!("Got time {:?}", request);
    }

    {
        let request = channel.get_boxed::<CtrlInt<[i32]>>().await.unwrap();
        println!("Got ctrl {:?}", request);
    }
}
