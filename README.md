# libffi-sys-rs: Low-level Rust bindings for [libffi](https://sourceware.org/libffi/)

[![Build Status](https://travis-ci.org/tov/libffi-sys-rs.svg?branch=master)](https://travis-ci.org/tov/libffi-sys-rs)
[![Crates.io](https://img.shields.io/crates/v/libffi-sys.svg?maxAge=2592000)](https://crates.io/crates/libffi-sys)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](LICENSE-APACHE)

The C libffi library provides two main facilities: assembling calls
to functions dynamically, and creating closures that can be called
as ordinary C functions. This is an undocumented wrapper, generated
by bindgen, intended as the basis for higher-level bindings, but you
can see the [C libffi
documentation](http://www.atmark-techno.com/~yashi/libffi.html).

See [the libffi crate](https://crates.io/crates/libffi/) for a
higher-level API.

## Usage

Make sure you have a working make, C compiler, automake, and autoconf
first. It’s [on crates.io](https://crates.io/crates/libffi-sys), so you
can add

```toml
[dependencies]
libffi-sys = "0.4.6"
```

to your `Cargo.toml` and

```rust
extern crate libffi_sys;
```

to your crate root.
