use std::env;

fn epics_target(target: &str) -> String {
    let parts = target.split('-').collect::<Vec<_>>();
    let arch = parts[0];
    let os = parts[2];
    format!("{}-{}", os, arch)
}

fn main() {
    println!("cargo:rustc-link-lib=dylib=ca");
    if let Ok(epics_base) = env::var("EPICS_BASE") {
        let target = env::var("TARGET").unwrap();
        println!(
            "cargo:rustc-link-search={}/lib/{}",
            epics_base,
            epics_target(&target)
        );
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=EPICS_BASE");
}
