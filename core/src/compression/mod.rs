//! Compression support

/// Compression codec
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    /// No compression
    None,
    /// LZ4 compression
    #[cfg(feature = "compression")]
    Lz4,
    /// Zstandard compression
    #[cfg(feature = "compression")]
    Zstd,
}

impl Codec {
    /// Compress data
    pub fn compress(&self, data: &[u8]) -> crate::Result<Vec<u8>> {
        match self {
            Codec::None => Ok(data.to_vec()),
            #[cfg(feature = "compression")]
            Codec::Lz4 => {
                lz4_flex::compress_prepend_size(data)
                    .map_err(|e| crate::Error::CompressionError(format!("LZ4 error: {:?}", e)))
            }
            #[cfg(feature = "compression")]
            Codec::Zstd => {
                zstd::encode_all(data, 10)
                    .map_err(|e| crate::Error::CompressionError(format!("Zstd error: {}", e)))
            }
        }
    }

    /// Decompress data
    pub fn decompress(&self, data: &[u8]) -> crate::Result<Vec<u8>> {
        match self {
            Codec::None => Ok(data.to_vec()),
            #[cfg(feature = "compression")]
            Codec::Lz4 => {
                lz4_flex::decompress_size_prepended(data)
                    .map_err(|e| crate::Error::CompressionError(format!("LZ4 error: {:?}", e)))
            }
            #[cfg(feature = "compression")]
            Codec::Zstd => {
                zstd::decode_all(data)
                    .map_err(|e| crate::Error::CompressionError(format!("Zstd error: {}", e)))
            }
        }
    }
}
