use cstr::cstr;
use epics_ca::Context;

#[async_std::main]
async fn main() {
    let ctx = Context::new().unwrap();

    let name = cstr!("ca:test:aao");
    let mut channel = ctx.connect::<[i32]>(name).await.unwrap();
    println!("Connected to {:?}", name);

    {
        let value = [0, 1, 2, 3];
        channel.put_ref(&value).unwrap().await.unwrap();
        println!("Put: {:?}", value);
    }

    {
        let value = channel.get_vec().await.unwrap();
        println!("Got: {:?}", &value);
        assert_eq!(value, [0, 1, 2, 3]);
    }
}
