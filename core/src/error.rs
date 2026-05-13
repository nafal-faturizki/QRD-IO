//! Error types for QRD-Core

use thiserror::Error;

/// Result type for QRD operations
pub type Result<T> = std::result::Result<T, Error>;

/// QRD Error type
#[derive(Error, Debug)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid format
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Invalid column
    #[error("Invalid column {index}: {reason}")]
    InvalidColumn { index: usize, reason: String },

    /// Decryption error
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// Invalid key
    #[error("Invalid key: {0}")]
    InvalidKey(String),

    /// Schema error
    #[error("Schema error: {0}")]
    SchemaError(String),

    /// Compression error
    #[error("Compression error: {0}")]
    CompressionError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Bincode error
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}
