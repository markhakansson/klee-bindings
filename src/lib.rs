//! Raw Rust bindings to KLEE.
//!
//! This crate can be used to create Rust bindings to KLEE using bindgen.
//! It provides unsafe FFI to the local KLEE library. Works without the
//! standard library, but you will need the `cty` crate to use it.
//!
//! Please refer to the official KLEE documentation on how to use it.
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate cty;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
