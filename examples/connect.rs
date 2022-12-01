use c_str_macro::c_str;
use epics_ca::Context;

#[async_std::main]
async fn main() {
    let ctx = Context::new().unwrap();
    ctx.connect_typed::<f64>(c_str!("ca:test:ai"))
        .await
        .unwrap();
    println!("Connected");
}
