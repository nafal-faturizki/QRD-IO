//! Encoding and serialization utilities

/// Encoding errors
pub struct Encoder;

impl Encoder {
    /// Encode value to bytes using JSON
    pub fn encode<T: serde::Serialize>(value: &T) -> crate::Result<Vec<u8>> {
        serde_json::to_vec(value)
            .map_err(|e| crate::Error::SerializationError(e))
    }

    /// Decode value from bytes using JSON
    pub fn decode<T: for<'de> serde::Deserialize<'de>>(
        data: &[u8],
    ) -> crate::Result<T> {
        serde_json::from_slice(data)
            .map_err(|e| crate::Error::SerializationError(e))
    }
}

/// Decoding utilities
pub struct Decoder;

impl Decoder {
    /// Decode with validation
    pub fn decode_safe<T: for<'de> serde::Deserialize<'de>>(
        data: &[u8],
    ) -> crate::Result<T> {
        if data.is_empty() {
            return Err(crate::Error::InvalidFormat(
                "Empty data".to_string(),
            ));
        }
        Encoder::decode(data)
    }
}
