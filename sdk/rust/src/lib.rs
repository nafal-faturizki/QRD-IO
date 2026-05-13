//! QRD-SDK Rust Bindings
//!
//! High-level API for QRD format operations.

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

pub use qrd_core::{self, Reader, Writer, Schema, ColumnType, Error, Result};

/// QRD Rust SDK version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
