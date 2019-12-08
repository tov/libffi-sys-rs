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

fn run_command(which: &'static str, cmd: &mut Command) {
    assert!(cmd.status().expect(which).success(), which);
}

fn build_and_link() -> IncludePaths {
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&out_dir).join("libffi-build");
    let prefix = Path::new(&out_dir).join("libffi-root");
    let include = Path::new(&prefix).join("include");
    let libdir = Path::new(&prefix).join("lib");
    let libdir64 = Path::new(&prefix).join("lib64");

    // Copy LIBFFI_DIR into build_dir to avoid an unnecessary build
    if let Err(e) = fs::remove_dir_all(&build_dir) {
        assert!(e.kind() == io::ErrorKind::NotFound,
            "can't remove the build directory: {}", e);
    }
    run_command("Copying libffi into the build directory",
                Command::new("cp")
                    .arg("-R")
                    .arg(LIBFFI_DIR)
                    .arg(&build_dir));

    // Generate configure, run configure, make, make install
    autogen(&build_dir);

    configure_libffi(prefix, &build_dir);

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

fn autogen(build_dir: &Path) {
    assert!(build_dir.join("autogen.sh").exists(), "
        **********
        build.rs could not find autogen.sh when attempting to build C
        libffi. Either init and update the libffi submodule or pass the
        \"system\" feature to Cargo to use your system’s libffi.
        **********
        ");

    let mut command = Command::new("sh");

    command
        .arg("autogen.sh")
        .current_dir(&build_dir);

    if cfg!(windows) {
        // When building in MSYS2, not clearing the environment variables first
        // results in `configure` being generated incorrectly. By clearing the
        // variables first, then restoring PATH, we can ensure the correct file
        // is generated.
        command
            .env_clear()
            .env("PATH", env::var("PATH").unwrap());
    }

    run_command("Generating configure", &mut command);
}

fn configure_libffi(prefix: PathBuf, build_dir: &Path) {
    let mut command = Command::new("sh");

    command.arg("configure")
        .arg("--with-pic")
        .arg("--disable-docs")
        .current_dir(&build_dir);

    if cfg!(windows) {
        // When using MSYS2, OUT_DIR will be a Windows like path such as
        // C:\foo\bar. Unfortunately, the various scripts used for building
        // libffi do not like such a path, so we have to turn this into a Unix
        // like path such as /c/foo/bar.
        //
        // This code assumes the path only uses : for the drive letter, and only
        // uses \ as a component separator. It will likely break for file paths
        // that include a :.
        let mut msys_prefix = prefix
            .to_str()
            .unwrap()
            .replace(":\\", "/")
            .replace("\\", "/");

        msys_prefix.insert(0, '/');

        command
            .arg("--prefix")
            .arg(msys_prefix);
    } else {
        command
            .arg("--prefix")
            .arg(prefix);
    }

    run_command("Configuring libffi", &mut command);
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
