//! Low-level Rust bindings for [libffi](https://sourceware.org/libffi/)
//!
//! The C libffi library provides two main facilities: assembling calls
//! to functions dynamically, and creating closures that can be called
//! as ordinary C functions. This is an undocumented wrapper, generated
//! by bindgen, intended as the basis for higher-level bindings, but you
//! can see the [C libffi
//! documentation](http://www.atmark-techno.com/~yashi/libffi.html).
//!
//! See [the libffi crate](https://crates.io/crates/libffi/) for a
//! higher-level API.
//!
//! # Usage
//!
//! Make sure you have a working make, C compiler, automake, and autoconf
//! first. It’s [on crates.io](https://crates.io/crates/libffi-sys), so you
//! can add
//!
//! ```toml
//! [dependencies]
//! libffi-sys = "0.5.0"
//! ```
//!
//! to your `Cargo.toml` and
//!
//! ```rust
//! extern crate libffi_sys;
//! ```
//!
//! to your crate root.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
