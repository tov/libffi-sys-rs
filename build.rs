extern crate bindgen;
extern crate make_cmd;
extern crate pkg_config;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::io;

use make_cmd::make;

const LIBFFI_DIR: &'static str = "libffi";

fn main() {
    let include_paths = if cfg!(feature = "system") {
        probe_and_link()
    } else {
        build_and_link()
    };
    generate_bindings(include_paths);
}

struct IncludePaths(Vec<PathBuf>);

fn probe_and_link() -> IncludePaths {
    let libffi = pkg_config::probe_library("libffi").expect("
        **********
        pkg-config could not find libffi. This could be because you
        don't have pkg-config, because you don't have libffi, or because
        they don't know about each other. If you can run `pkg-config
        libffi --cflags` and get a reasonable result, please file a bug
        report.
        **********
    ");

    IncludePaths(libffi.include_paths)
}

fn build_and_link() -> IncludePaths {
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&out_dir).join("libffi-build");
    let prefix = Path::new(&out_dir).join("libffi-root");
    let include = Path::new(&prefix)
        .join("lib")
        .join("libffi-3.2.1")
        .join("include");
    let libdir = Path::new(&prefix).join("lib");
    let libdir64 = Path::new(&prefix).join("lib64");

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
                    .arg("-a")
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

    // Cargo linking directives
    println!("cargo:rustc-link-lib=static=ffi");
    println!("cargo:rustc-link-search={}", libdir.display());
    println!("cargo:rustc-link-search={}", libdir64.display());

    IncludePaths(vec![include])
}

fn generate_bindings(include_paths: IncludePaths) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let include_file = Path::new("include").join("include_ffi.h");
    let out_file = Path::new(&out_dir).join("generated.rs");

    let mut builder = bindgen::Builder::default();
    for path in &include_paths.0 {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }
    builder.header(include_file.display().to_string())
        .derive_default(true)
        .blacklist_type("max_align_t")
        .generate()
        .expect("
        **********
        Bindgen generation failed. 
        Note that Bindgen requires clang to be installed. See the Bindgen documentation for more information:
        https://rust-lang-nursery.github.io/rust-bindgen/requirements.html

        If you believe this should have worked, please file a bug report.
        **********
        ")
        .write_to_file(out_file.display().to_string())
        .expect("bindgen output");
}
