//! Footer I/O - reading and writing file footers with append-only semantics
//!
//! QRD file footers are written last in the file and contain:
//! - Schema definition
//! - Row group metadata and offsets
//! - Integrity checksums
//! - Authentication tags
//!
//! Footer format (append-only contract):
//! - All footer data is appended at EOF
//! - Last 4 bytes contain footer length (U32LE)
//! - Reader seeks backwards from EOF to find footer
//! - Never backtracks during write

use crate::format::Footer;
use crate::integrity;
use std::io::{Seek, SeekFrom, Read, Write};

/// Write footer to writer at end of file (append-only)
///
/// # Contract
/// - Writer must be seekable
/// - Writes footer JSON + footer length
/// - Last 4 bytes are always footer length (U32LE)
/// - Never seeks backwards
///
/// # Returns
/// Total bytes written (footer + 4-byte length)
pub fn write_footer<W: Write + Seek>(writer: &mut W, footer: &Footer) -> crate::Result<u64> {
    // Serialize footer to JSON
    let footer_json = serde_json::to_vec(footer)?;
    let footer_len = footer_json.len() as u32;

    // Validate footer size (reasonable limit: 1MB)
    if footer_len > 1_000_000 {
        return Err(crate::Error::InvalidFormat(
            format!("Footer too large: {} bytes", footer_len),
        ));
    }

    // Write footer data
    let bytes_written = writer.write(&footer_json)? as u64;
    if bytes_written as u32 != footer_len {
        return Err(crate::Error::InvalidFormat(
            format!(
                "Failed to write complete footer: wrote {} of {} bytes",
                bytes_written, footer_len
            ),
        ));
    }

    // Write footer length (last 4 bytes, little-endian)
    writer.write_all(&footer_len.to_le_bytes())?;

    Ok(bytes_written + 4)
}

/// Read footer from reader
///
/// # Contract
/// - Reader must be seekable
/// - Reads last 4 bytes to get footer length
/// - Seeks backwards to read footer data
/// - Validates magic/version
///
/// # Returns
/// Tuple of (Footer struct, total bytes read including length)
pub fn read_footer<R: Read + Seek>(reader: &mut R) -> crate::Result<(Footer, u64)> {
    // Get file size
    let file_size = reader.seek(SeekFrom::End(0))?;

    if file_size < 4 {
        return Err(crate::Error::InvalidFormat(
            "File too small for footer (need at least 4 bytes)".to_string(),
        ));
    }

    // Read footer length (last 4 bytes)
    reader.seek(SeekFrom::End(-4))?;
    let mut len_bytes = [0u8; 4];
    reader.read_exact(&mut len_bytes)?;
    let footer_len = u32::from_le_bytes(len_bytes) as u64;

    // Validate footer length
    if footer_len == 0 {
        return Err(crate::Error::InvalidFormat(
            "Footer length cannot be zero".to_string(),
        ));
    }

    if footer_len > 1_000_000 {
        return Err(crate::Error::InvalidFormat(
            format!("Footer too large: {} bytes", footer_len),
        ));
    }

    if footer_len > file_size {
        return Err(crate::Error::InvalidFormat(
            format!(
                "Invalid footer length: {} bytes exceeds file size {} bytes",
                footer_len, file_size
            ),
        ));
    }

    // Read footer data
    reader.seek(SeekFrom::End(-(footer_len as i64 + 4)))?;
    let mut footer_data = vec![0u8; footer_len as usize];
    reader.read_exact(&mut footer_data)?;

    // Deserialize footer
    let footer: Footer = serde_json::from_slice(&footer_data)
        .map_err(|e| crate::Error::SerializationError(e))?;

    // Validate footer version
    if footer.version != 1 {
        return Err(crate::Error::InvalidFormat(
            format!("Unsupported footer version: {}", footer.version),
        ));
    }

    Ok((footer, footer_len + 4))
}

/// Footer with optional CRC32 validation
pub struct FooterWithChecksum {
    /// The footer data
    pub footer: Footer,
    /// CRC32 of footer data (optional)
    pub checksum: Option<u32>,
}

impl FooterWithChecksum {
    /// Create new footer with checksum
    pub fn new(footer: Footer, checksum: Option<u32>) -> Self {
        Self { footer, checksum }
    }

    /// Validate footer checksum if present
    pub fn validate(&self, data: &[u8]) -> bool {
        match self.checksum {
            Some(expected) => integrity::verify_crc32(data, expected),
            None => true, // No checksum means validation passes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_footer_roundtrip() {
        let mut footer = Footer::new(1);
        footer.row_group_count = 5;

        let mut buf = Vec::new();
        let written = write_footer(&mut Cursor::new(&mut buf), &footer).unwrap();

        let mut cursor = Cursor::new(&buf);
        let (loaded, footer_size) = read_footer(&mut cursor).unwrap();

        assert_eq!(loaded.version, footer.version);
        assert_eq!(loaded.row_group_count, footer.row_group_count);
        assert_eq!(footer_size, written);
    }

    #[test]
    fn test_footer_too_small() {
        let buf = vec![1, 2];
        let mut cursor = Cursor::new(&buf);
        let result = read_footer(&mut cursor);
        assert!(result.is_err());
    }

    #[test]
    fn test_footer_invalid_length() {
        let mut buf = vec![0u8; 10];
        // Set invalid length (too large)
        let len = buf.len();
        buf[len - 4..].copy_from_slice(&(1000000u32).to_le_bytes());
        let mut cursor = Cursor::new(&buf);
        let result = read_footer(&mut cursor);
        assert!(result.is_err());
    }

    #[test]
    fn test_footer_checksum() {
        let footer = Footer::new(1);
        let json = serde_json::to_vec(&footer).unwrap();
        let checksum = integrity::crc32(&json);

        let with_checksum = FooterWithChecksum::new(footer, Some(checksum));
        assert!(with_checksum.validate(&json));

        let wrong_checksum = FooterWithChecksum::new(Footer::new(1), Some(checksum ^ 0xFF));
        assert!(!wrong_checksum.validate(&json));
    }
}
