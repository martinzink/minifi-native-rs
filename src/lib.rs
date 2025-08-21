// minifi/src/lib.rs

//! A safe, idiomatic Rust wrapper for the Apache NiFi MiNiFi C API.
//!
//! This crate provides a high-level API for creating custom MiNiFi processors
//! in Rust without writing `unsafe` code.

// Declare the internal modules.
mod processor;
mod wrapper;
mod relationship_wrapper;
mod property_wrapper;
mod primitives;

// Re-export the public-facing types that a processor developer will need.
pub use processor::{ProcessContext, Processor, ProcessorBridge};
pub use wrapper::{Descriptor, FlowFile, Logger, Session, SessionFactory};
pub use relationship_wrapper::Relationship;
pub use property_wrapper::Property;

// Also re-export the raw C types from the -sys crate under a `sys` module,
// which is a common convention. This allows users to access the raw types
// if they have an advanced use case that requires it.
pub use minificpp_sys as sys;
