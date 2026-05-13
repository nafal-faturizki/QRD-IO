// QRD-Core Library
//
// Privacy-native columnar binary container format with encryption as a first-class property.
// All modules maintain:
// - Zero-panic parser policy
// - Bounded memory guarantees
// - Deterministic binary output
// - Cross-language fidelity

#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unsafe_code,
    unused_qualifications
)]
#![forbid(unsafe_code)]

//! # QRD-Core
//!
//! A privacy-native columnar binary container format with end-to-end encryption.
//!
//! ## Quick Start
//!
//! ```ignore
//! use qrd_core::{Writer, Reader};
//!
//! // Write encrypted columnar data
//! let mut writer = Writer::new(output);
//! writer.write_column(column_data)?;
//! writer.finish()?;
//!
//! // Read encrypted columnar data
//! let mut reader = Reader::open(input)?;
//! let data = reader.read_column(0)?;
//! ```

pub mod error;
pub mod schema;
pub mod encryption;
pub mod encoding;
pub mod compression;
pub mod format;
pub mod header;
pub mod footer;
pub mod integrity;
pub mod columnar;
pub mod io;
pub mod writer;
pub mod reader;

pub use error::{Result, Error};
pub use writer::Writer;
pub use reader::Reader;
pub use schema::{Schema, ColumnType};

/// QRD-Core library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
