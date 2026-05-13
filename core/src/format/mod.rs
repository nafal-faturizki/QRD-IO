//! Binary format specification and structures

use serde::{Deserialize, Serialize};

/// Magic number for QRD files
pub const MAGIC: &[u8; 4] = b"QRD\x01";

/// File version
pub const VERSION: u32 = 1;

/// Row group metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowGroup {
    /// Number of rows in this group
    pub row_count: u64,
    /// Columns in this group
    pub columns: Vec<ColumnMetadata>,
}

/// Column metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnMetadata {
    /// Column index
    pub index: usize,
    /// Column name
    pub name: String,
    /// Uncompressed size
    pub uncompressed_size: u64,
    /// Compressed size
    pub compressed_size: u64,
    /// Is encrypted
    pub is_encrypted: bool,
    /// Nonce for encryption (if encrypted)
    pub nonce: Option<[u8; 12]>,
    /// Authentication tag (if encrypted)
    pub auth_tag: Option<[u8; 16]>,
}

/// File footer (written last)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Footer {
    /// File version
    pub version: u32,
    /// Number of row groups
    pub row_group_count: u64,
    /// Row groups metadata
    pub row_groups: Vec<RowGroup>,
    /// Schema
    pub schema: Vec<u8>, // Encoded schema
    /// Footer authentication tag
    pub auth_tag: Option<[u8; 16]>,
}

impl Footer {
    /// Create a new footer
    pub fn new(version: u32) -> Self {
        Self {
            version,
            row_group_count: 0,
            row_groups: Vec::new(),
            schema: Vec::new(),
            auth_tag: None,
        }
    }
}
