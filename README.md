# libffi-sys-rs: Low-level Rust bindings for [libffi](https://sourceware.org/libffi/)

[![Travis CI build status](https://travis-ci.org/tov/libffi-sys-rs.svg?branch=master)](https://travis-ci.org/tov/libffi-sys-rs)
[![Appveyor build status](https://ci.appveyor.com/api/projects/status/7jlhe1ahf7vjkcnw/branch/master?svg=true)](https://ci.appveyor.com/project/tov/libffi-sys-rs/branch/master)
[![Crates.io](https://img.shields.io/crates/v/libffi-sys.svg?maxAge=2592000)](https://crates.io/crates/libffi-sys)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](LICENSE-APACHE)


The C libffi library provides two main facilities: assembling calls
to functions dynamically, and creating closures that can be called
as ordinary C functions. This is an undocumented wrapper, generated
by bindgen, intended as the basis for higher-level bindings, but you
can see the [C libffi
documentation](http://www.atmark-techno.com/~yashi/libffi.html).

If you clone this repository in order to build the library and you do
not plan to enable the `system` Cargo feature to build against your
system’s C libffi, then you should do a recursive clone, by default this
library builds C libffi from a Git submodule.

See [the libffi crate](https://crates.io/crates/libffi/) for a
higher-level API.


## Usage

`libffi-sys` can either build its own copy of the libffi C library [from
github](https://github.com/libffi/libffi) or it can link against your
system’s C libffi. By default it builds its own because many systems
ship with an old C libffi; this requires that you have a working make,
C compiler, automake, autoconf, and texinfo first. If your system libffi
is up-to-date (v3.2.1 as of October 2019), you can instead enable the
`system` feature flag to use that. If you want this crate to build
a C libffi for you, add

```toml
[dependencies]
libffi-sys = "0.7.0"
```

to your `Cargo.toml`. If you want to use your system C libffi, then

```toml
[dependencies.libffi-sys]
version = "0.7.0"
features = ["system"]
```

to your `Cargo.toml` instead.

This crate supports Rust version 1.32 and later.

## Help

The message below means that `build.rs` is trying to build the C libffi in the
`libffi` submodule, but the submodule isn’t checked out. Either initialize and
update the submodule or enable the `system` feature flag.

```
error: failed to run custom build command for `libffi-sys v0.7.1-alpha.0 (/Users/tov/projects/libffi-sys-rs)`

Caused by:
  process didn't exit successfully: `/Users/tov/projects/libffi-sys-rs/target/debug/build/libffi-sys-0fd86a57f553b770/build-script-build` (exit code: 101)
--- stderr
sh: autogen.sh: No such file or directory
thread 'main' panicked at 'Generating configure', build.rs:41:5
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```
