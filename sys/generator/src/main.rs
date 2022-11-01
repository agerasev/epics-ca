use anyhow::Error;
use bindgen::{self, builder, Bindings, Builder};
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

trait BuilderExt {
    fn header_allowlist<T: Into<String>>(self, header: T) -> Self;
    fn allowlist_type_opaque<T: Into<String>>(self, arg: T) -> Self;
}
trait BindingsExt {
    fn write_to_module<P: AsRef<Path>, T: Into<String>>(
        self,
        dir: P,
        mod_name: T,
    ) -> io::Result<()>;
}

impl BuilderExt for Builder {
    fn header_allowlist<T: Into<String>>(self, header: T) -> Self {
        let header = header.into();
        print!("\"{}\" -> ", header);
        self.header(header.clone()).allowlist_file(header)
    }
    fn allowlist_type_opaque<T: Into<String>>(self, arg: T) -> Self {
        let arg = arg.into();
        self.allowlist_type(arg.clone()).opaque_type(arg)
    }
}
impl BindingsExt for Bindings {
    fn write_to_module<P: AsRef<Path>, T: Into<String>>(
        self,
        dir: P,
        mod_name: T,
    ) -> io::Result<()> {
        let mod_name = mod_name.into();
        let mod_path = dir.as_ref().join(format!("{}.rs", &mod_name));
        self.write_to_file(&mod_path)?;
        writeln!(
            &OpenOptions::new()
                .append(true)
                .open(dir.as_ref().join("mod.rs"))?,
            "pub mod {};",
            mod_name
        )?;
        println!("{:?}", &mod_path);
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let epics_base = "../../epics-base";
    let out_dir = Path::new("../src/generated");

    fs::remove_dir_all(out_dir).or_else(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            Ok(())
        } else {
            Err(err)
        }
    })?;
    fs::create_dir(out_dir)?;
    fs::File::create(out_dir.join("mod.rs"))?;

    let builder = builder()
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
        .allowlist_recursively(false)
        .clang_args([
            format!("-I{}/include", epics_base),
            format!("-I{}/include/compiler/clang", epics_base),
            format!("-I{}/include/os/Linux", epics_base),
            "-DCA_DONT_INCLUDE_STDARGH".to_string(),
        ]);

    builder
        .clone()
        .header_allowlist(format!("{}/include/epicsTypes.h", epics_base))
        .generate()?
        .write_to_module(out_dir, "types")?;

    builder
        .clone()
        .header(format!("{}/include/epicsTime.h", epics_base))
        .raw_line("use crate::types::*;")
        .allowlist_type("epicsTimeStamp")
        .generate()?
        .write_to_module(out_dir, "time")?;

    builder
        .clone()
        .header_allowlist(format!("{}/include/db_access.h", epics_base))
        .raw_line("use crate::{time::*, types::*};")
        .generate()?
        .write_to_module(out_dir, "db_access")?;

    builder
        .clone()
        .header_allowlist(format!("{}/include/caerr.h", epics_base))
        .generate()?
        .write_to_module(out_dir, "caerr")?;

    builder
        .clone()
        .header_allowlist(format!("{}/include/caeventmask.h", epics_base))
        .generate()?
        .write_to_module(out_dir, "caeventmask")?;

    builder
        .clone()
        .header_allowlist(format!("{}/include/cadef.h", epics_base))
        .blocklist_type("ca_access_rights")
        .raw_line("use crate::{cadef::ca_access_rights, thread::*};")
        .generate()?
        .write_to_module(out_dir, "cadef")?;

    drop(builder);
    Ok(())
}
