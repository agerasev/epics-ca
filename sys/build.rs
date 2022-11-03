use std::env;

fn os(target: &str) -> &str {
    target.split('-').nth(2).unwrap()
}

fn epics_target(target: &str) -> String {
    let arch = target.split('-').next().unwrap();
    let os = os(target);
    format!("{}-{}", os, arch)
}

fn main() {
    println!("cargo:rustc-link-lib=dylib=ca");
    if let Ok(epics_base) = env::var("EPICS_BASE") {
        let target = env::var("TARGET").unwrap();
        println!(
            "cargo:rustc-link-search={}/lib/{}",
            epics_base,
            epics_target(&target),
        );
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=EPICS_BASE");

    #[cfg(feature = "test")]
    build_tests();
}

#[cfg(feature = "test")]
fn build_tests() {
    let epics_base = env::var("EPICS_BASE").unwrap();
    let mut build = cc::Build::new();
    let compiler = {
        let tool = build.get_compiler();
        if tool.is_like_gnu() {
            "gcc"
        } else if tool.is_like_clang() {
            "clang"
        } else {
            unimplemented!()
        }
    };
    let os = {
        let target = env::var("TARGET").unwrap();
        let lower = os(&target);
        format!("{}{}", lower[..1].to_uppercase(), &lower[1..])
    };
    build
        .file(format!(
            "{}/src/test.c",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        ))
        .includes([
            format!("{}/include", &epics_base),
            format!("{}/include/compiler/{}", &epics_base, &compiler),
            format!("{}/include/os/{}", &epics_base, &os),
        ])
        .compile("ctest");

    println!("cargo:rustc-link-lib=ctest");
}
