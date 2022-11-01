use anyhow::Error;
use bindgen::{self, builder};
use std::env;

fn main() -> Result<(), Error> {
    let epics_base = env::var("EPICS_BASE").unwrap();
    println!("{}", epics_base);

    let main_header = format!("{}/include/cadef.h", epics_base);
    builder()
        .generate_comments(false)
        .derive_copy(true)
        .derive_debug(true)
        .layout_tests(false)
        .prepend_enum_name(false)
        .use_core()
        .size_t_is_usize(true)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
        .merge_extern_blocks(true)
        .ctypes_prefix("libc")
        .clang_args([
            format!("-I{}/include", epics_base),
            format!("-I{}/include/compiler/clang", epics_base),
            format!("-I{}/include/os/Linux", epics_base),
            "-DCA_DONT_INCLUDE_STDARGH".to_string(),
        ])
        .header(&main_header)
        .allowlist_recursively(false)
        .allowlist_file(format!("{}/include/epicsTypes.h", epics_base))
        .allowlist_type("epicsTimeStamp")
        .allowlist_type("epicsThreadId")
        .allowlist_file(format!("{}/include/db_access.h", epics_base))
        .allowlist_file(format!("{}/include/caerr.h", epics_base))
        .allowlist_file(format!("{}/include/caeventmask.h", epics_base))
        .allowlist_file(&main_header)
        .blocklist_type("ca_access_rights")
        .raw_line("use crate::{ca_access_rights, epicsThreadOSD};")
        .generate()?
        .write_to_file("../sys/src/generated.rs")?;

    Ok(())
}
