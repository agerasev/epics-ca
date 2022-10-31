use crate::Context;

#[test]
fn context() {
    Context::new(false).unwrap().attach();
}
