//! Type system for QRD-SDK

use serde::{Deserialize, Serialize};

/// Supported column types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
    /// Null type
    Null,
    /// Boolean
    Boolean,
    /// 8-bit signed integer
    Int8,
    /// 16-bit signed integer
    Int16,
    /// 32-bit signed integer
    Int32,
    /// 64-bit signed integer
    Int64,
    /// 32-bit unsigned integer
    UInt32,
    /// 64-bit unsigned integer
    UInt64,
    /// 32-bit IEEE float
    Float32,
    /// 64-bit IEEE float
    Float64,
    /// String (UTF-8)
    String,
    /// Binary data
    Binary,
    /// Date (days since epoch)
    Date32,
    /// Timestamp (microseconds since epoch)
    Timestamp,
    /// Decimal with precision and scale
    Decimal(DecimalType),
}

/// Decimal type configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecimalType {
    /// Precision (total digits)
    pub precision: u8,
    /// Scale (digits after decimal point)
    pub scale: u8,
}

impl ColumnType {
    /// Get byte size for fixed-width types
    pub fn byte_size(&self) -> Option<usize> {
        match self {
            ColumnType::Int8 => Some(1),
            ColumnType::Int16 => Some(2),
            ColumnType::Int32 | ColumnType::Float32 => Some(4),
            ColumnType::Int64 | ColumnType::Float64 | ColumnType::Timestamp => Some(8),
            ColumnType::Date32 => Some(4),
            ColumnType::UInt32 => Some(4),
            ColumnType::UInt64 => Some(8),
            ColumnType::Boolean => Some(1),
            _ => None,
        }
    }
}

/// Schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// Column definitions
    pub columns: Vec<ColumnDef>,
}

/// Column definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDef {
    /// Column name
    pub name: String,
    /// Column type
    pub col_type: ColumnType,
    /// Is nullable
    pub nullable: bool,
}

impl Schema {
    /// Create a new schema
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    /// Add a column to the schema
    pub fn with_column(mut self, name: String, col_type: ColumnType, nullable: bool) -> Self {
        self.columns.push(ColumnDef {
            name,
            col_type,
            nullable,
        });
        self
    }

    /// Validate schema
    pub fn validate(&self) -> crate::Result<()> {
        if self.columns.is_empty() {
            return Err(crate::Error::SchemaError("Schema cannot be empty".to_string()));
        }

        for (i, col) in self.columns.iter().enumerate() {
            if col.name.is_empty() {
                return Err(crate::Error::SchemaError(
                    format!("Column {} has empty name", i),
                ));
            }
        }

        Ok(())
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
    }
}
