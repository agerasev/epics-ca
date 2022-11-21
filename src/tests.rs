use crate::Context;

#[test]
fn context() {
    Context::new().unwrap().attach();
}
