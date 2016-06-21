extern crate bindgen;
extern crate curl;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use curl::easy::Easy;

const LIBFFI_ARCHIVE: &'static str = "libffi.tar.gz";
const LIBFFI_DIR: &'static str     = "libffi-3.2.1";
const LIBFFI_URL: &'static str     =
    "ftp://sourceware.org/pub/libffi/libffi-3.2.1.tar.gz";

fn main() {
    let out_dir   = env::var("OUT_DIR").unwrap();

    let archive   = Path::new(&out_dir).join(LIBFFI_ARCHIVE);
    let build_dir = Path::new(&out_dir).join(LIBFFI_DIR);
    let prefix    = Path::new(&out_dir).join("libffi-root");
    let include   = Path::new(&prefix).join("lib")
                                      .join("libffi-3.2.1")
                                      .join("include");
    let libdir    = Path::new(&prefix).join("lib");

    // Download libffi
    let mut file  = File::create(&archive).unwrap();
    let mut curl = Easy::new();
    curl.url(LIBFFI_URL).unwrap();
    curl.write_function(move |data| Ok(file.write(data).unwrap())).unwrap();
    curl.perform().unwrap();

    fn run_command(which: &'static str, cmd: &mut Command) {
        assert!(cmd.status().expect(which).success(), which);
    }

    // Untar it.
    run_command("Untarring libffi",
                Command::new("tar")
                        .arg("-zxf")
                        .arg(LIBFFI_ARCHIVE)
                        .current_dir(&out_dir));

    // Configure, make, make install
    run_command("Configuring libffi",
                Command::new("./configure")
                        .arg("--disable-shared")
                        .arg("--prefix")
                        .arg(prefix)
                        .current_dir(&build_dir));
    run_command("Building libffi",
                Command::new("make")
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

