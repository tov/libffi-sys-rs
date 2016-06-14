extern crate bindgen;
extern crate pkg_config;

use std::path::Path;

fn main() {
    let include_path = Path::new("include").join("include_ffi.h");
    let out_path = Path::new("src").join("generated.rs");

    let libffi = pkg_config::probe_library("libffi").expect("libffi");

    let mut builder = bindgen::Builder::default();
    builder.header(include_path.display().to_string());
    for path in &libffi.include_paths {
        builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder.generate().expect("bindgen generation");
    bindings.write_to_file(out_path.display().to_string())
            .expect("bindgen output");

    for lib in &libffi.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    for path in &libffi.link_paths {
        println!("cargo:rustc-link-search={}", path.display());
    }
}

