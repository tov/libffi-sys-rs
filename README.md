# libffi-sys-rs: Low-level Rust bindings for [libffi]

[![Travis CI build status](https://travis-ci.org/tov/libffi-sys-rs.svg?branch=master)](https://travis-ci.org/tov/libffi-sys-rs)
[![Appveyor build status](https://ci.appveyor.com/api/projects/status/7jlhe1ahf7vjkcnw/branch/master?svg=true)](https://ci.appveyor.com/project/tov/libffi-sys-rs/branch/master)
[![Crates.io](https://img.shields.io/crates/v/libffi-sys.svg?maxAge=2592000)](https://crates.io/crates/libffi-sys)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](LICENSE-APACHE)

The C libffi library provides two main facilities: assembling calls
to functions dynamically, and creating closures that can be called
as ordinary C functions. This is an undocumented wrapper, generated
by bindgen, intended as the basis for higher-level bindings, but you
can see the [libffi documentation].

If you clone this repository in order to build the library and you do
not plan to enable the `system` Cargo feature to build against your
system’s C libffi, then you should do a recursive clone, by default this
library builds C libffi from a Git submodule.

See [the `libffi` crate] for a higher-level API.

## Usage

`libffi-sys` can either build its own copy of the libffi C library [from
github][libffi github] or it can link against your
system’s C libffi. By default it builds its own because many systems
ship with an old C libffi; this requires that you have a working make,
C compiler, automake, and autoconf first. If your system libffi
is new enough (v3.2.1 as of October 2019), you can instead enable the
`system` feature flag to use that. If you want this crate to build
a C libffi for you, add

```toml
[dependencies]
libffi-sys = "1.0.0"
```

to your `Cargo.toml`. If you want to use your system C libffi, then

```toml
[dependencies.libffi-sys]
version = "1.0.0"
features = ["system"]
```

to your `Cargo.toml` instead.

This crate supports Rust version 1.32 and later.


[the `libffi` crate]: https://crates.io/crates/libffi/

[libffi]: https://sourceware.org/libffi/

[libffi github]: https://github.com/libffi/libffi

[libffi documentation]: http://www.atmark-techno.com/~yashi/libffi.html

