use std::env;

fn main() {
    println!("cargo:rustc-link-lib=dylib=ca");
    if let Ok(epics_base) = env::var("EPICS_BASE") {
        println!("cargo:rustc-link-search={}/lib/linux-x86_64", epics_base);
    }
}
