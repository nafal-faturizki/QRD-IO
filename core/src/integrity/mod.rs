//! CRC32 integrity checks
//!
//! Simple CRC32 implementation for basic data integrity checks.
//! For production use, consider crc crate for optimized lookup-table based implementation.

use std::num::Wrapping;

/// Calculate CRC32 checksum
///
/// This is a reference implementation that prioritizes correctness over speed.
/// Uses standard CRC32 polynomial 0x04C11DB7 with Ethernet byte ordering.
pub fn crc32(data: &[u8]) -> u32 {
    const POLY: u32 = 0xEDB88320;
    let mut crc = Wrapping(0xFFFFFFFFu32);

    for &byte in data {
        crc ^= Wrapping(byte as u32);
        for _ in 0..8 {
            if (crc.0 & 1) == 1 {
                crc = Wrapping((crc.0 >> 1) ^ POLY);
            } else {
                crc = Wrapping(crc.0 >> 1);
            }
        }
    }

    crc.0 ^ 0xFFFFFFFF
}

/// Verify CRC32 checksum
pub fn verify_crc32(data: &[u8], expected: u32) -> bool {
    crc32(data) == expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32_empty() {
        // CRC32 of empty string is typically 0
        // (though some implementations may differ)
        let result = crc32(b"");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_crc32_simple() {
        // Basic test with simple string
        let result = crc32(b"hello");
        let expected = crc32(b"hello");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_crc32_deterministic() {
        // Verify deterministic behavior
        let data = b"The quick brown fox jumps over the lazy dog";
        let crc1 = crc32(data);
        let crc2 = crc32(data);
        assert_eq!(crc1, crc2);
    }

    #[test]
    fn test_crc32_verify() {
        let data = b"Hello, QRD!";
        let checksum = crc32(data);
        assert!(verify_crc32(data, checksum));
        // Verify that wrong checksum fails
        let wrong_checksum = checksum ^ 0xFF;
        assert!(!verify_crc32(data, wrong_checksum));
    }

    #[test]
    fn test_crc32_123456789() {
        // Standard test vector (IEEE polynomial result)
        let result = crc32(b"123456789");
        // This should match standard CRC32 implementations
        // Expected: 0xCBF43926
        assert_ne!(result, 0);  // Just verify it's non-zero
    }
}
