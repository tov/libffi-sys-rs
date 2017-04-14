extern crate bindgen;
extern crate make_cmd;

use std::env;
use std::path::Path;
use std::process::Command;
use std::fs;
use std::io;

use make_cmd::make;

const LIBFFI_DIR: &'static str = "libffi";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let build_dir = Path::new(&out_dir).join("libffi-build");
    let prefix = Path::new(&out_dir).join("libffi-root");
    let include = Path::new(&prefix)
        .join("lib")
        .join("libffi-3.2.1")
        .join("include");
    let libdir = Path::new(&prefix).join("lib");

    fn run_command(which: &'static str, cmd: &mut Command) {
        assert!(cmd.status().expect(which).success(), which);
    }

    // Copy LIBFFI_DIR into build_dir to avoid an unnecessary build
    if let Err(e) = fs::remove_dir_all(&build_dir) {
        assert!(e.kind() == io::ErrorKind::NotFound,
            "can't remove the build directory: {}", e);
    }
    run_command("Copying libffi into the build directory",
                Command::new("cp")
                    .arg("-ar")
                    .arg(LIBFFI_DIR)
                    .arg(&build_dir));


    // Generate configure, run configure, make, make install
    run_command("Generating configure",
                Command::new("./autogen.sh").current_dir(&build_dir));
    run_command("Configuring libffi",
                Command::new("./configure")
                    .arg("--prefix")
                    .arg(prefix)
                    .arg("--with-pic")
                    .current_dir(&build_dir));
    run_command("Building libffi",
                make()
                    .arg("install")
                    .current_dir(&build_dir));

    // Now run bindgen.

    let include_file = Path::new("include").join("include_ffi.h");
    let out_file = Path::new(&out_dir).join("generated.rs");

    let builder = bindgen::Builder::default();
    builder.header(include_file.display().to_string())
        .clang_arg(format!("-I{}", include.display()))
        .no_unstable_rust()
        .derive_default(true)
        .hide_type("max_align_t")
        .generate()
        .expect("
        **********
        Bindgen generation failed. Please file a bug \
                 report.
        **********
        ")
        .write_to_file(out_file.display().to_string())
        .expect("bindgen output");

    // Cargo linking directives
    println!("cargo:rustc-link-lib=static=ffi");
    println!("cargo:rustc-link-search={}", libdir.display());
}
