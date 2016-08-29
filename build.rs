extern crate bindgen;
extern crate make_cmd;

use std::env;
use std::path::Path;
use std::process::Command;

use make_cmd::make;

const LIBFFI_DIR: &'static str     = "libffi";

fn main() {
    let out_dir   = env::var("OUT_DIR").unwrap();

    let build_dir = Path::new(LIBFFI_DIR);
    let prefix    = Path::new(&out_dir).join("libffi-root");
    let include   = Path::new(&prefix).join("lib")
                                      .join("libffi-3.2.1")
                                      .join("include");
    let libdir    = Path::new(&prefix).join("lib");

    fn run_command(which: &'static str, cmd: &mut Command) {
        assert!(cmd.status().expect(which).success(), which);
    }

    // Generate configure, run configure, make, make install
    run_command("Generating configure",
                Command::new("./autogen.sh")
                        .current_dir(&build_dir));
    run_command("Configuring libffi",
                Command::new("./configure")
                        .arg("--disable-docs")
                        .arg("--prefix")
                        .arg(prefix)
                        .current_dir(&build_dir));
    run_command("Building libffi",
                make()
                        .arg("install")
                        .current_dir(&build_dir));

    // Now run bindgen.

    let include_file = Path::new("include").join("include_ffi.h");
    let out_file = Path::new("src").join("generated.rs");

    let mut builder = bindgen::Builder::default();
    builder.header(include_file.display().to_string());
    builder.clang_arg(format!("-I{}", include.display()));

    let bindings = builder.generate().expect("
        **********
        Bindgen generation failed. Please file a bug report.
        **********
    ");
    bindings.write_to_file(out_file.display().to_string())
            .expect("bindgen output");

    // Cargo linking directives
    println!("cargo:rustc-link-lib=ffi");
    println!("cargo:rustc-link-search={}", libdir.display());
}

