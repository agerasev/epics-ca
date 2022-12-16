use cstr::cstr;
use epics_ca::Context;

#[async_std::main]
async fn main() {
    let ctx = Context::new().unwrap();
    ctx.connect::<f64>(cstr!("ca:test:ai")).await.unwrap();
    println!("Connected");
}
