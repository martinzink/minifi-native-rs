// minifi-sys/src/lib.rs

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! This crate provides raw, unsafe FFI bindings for the Apache NiFi MiNiFi C API.
//!
//! The bindings are generated automatically by `bindgen` from the `minifi-c.h` header.

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Since bindgen can struggle with C macros that involve type casting,
// we manually define the boolean constants here to ensure they are
// always available to any crate that uses this one.
pub const MINIFI_TRUE: MinifiBool = 1;
pub const MINIFI_FALSE: MinifiBool = 0;