//! File header (32 bytes) parsing and writing

use std::io::{Read, Write};

/// File header (32 bytes)
#[derive(Debug, Clone)]
pub struct Header {
    /// Magic number: 0x51 0x52 0x44 0x01 ("QRD\x01")
    pub magic: [u8; 4],
    /// Format major version (breaking changes)
    pub format_major: u16,
    /// Format minor version (non-breaking)
    pub format_minor: u16,
    /// Truncated SHA-256 of schema (8 bytes)
    pub schema_id: [u8; 8],
    /// Flags: encrypted, stats_encrypted, ecc, schema_signed
    pub flags: u16,
    /// Reserved for future use (must be 0)
    pub reserved: u16,
    /// Writer version string (12 bytes, UTF-8, null-padded)
    pub writer_version: [u8; 12],
}

impl Header {
    /// Create a new header with default values
    pub fn new() -> Self {
        Self {
            magic: *b"QRD\x01",
            format_major: 1,
            format_minor: 0,
            schema_id: [0u8; 8],
            flags: 0,
            reserved: 0,
            writer_version: [0u8; 12],
        }
    }

    /// Serialize header to bytes (32 bytes total)
    pub fn to_bytes(&self) -> crate::Result<[u8; 32]> {
        let mut bytes = [0u8; 32];
        let mut cursor = std::io::Cursor::new(&mut bytes[..]);

        cursor.write_all(&self.magic)?;
        cursor.write_all(&self.format_major.to_le_bytes())?;
        cursor.write_all(&self.format_minor.to_le_bytes())?;
        cursor.write_all(&self.schema_id)?;
        cursor.write_all(&self.flags.to_le_bytes())?;
        cursor.write_all(&self.reserved.to_le_bytes())?;
        cursor.write_all(&self.writer_version)?;

        Ok(bytes)
    }

    /// Deserialize header from bytes
    pub fn from_bytes(bytes: &[u8; 32]) -> crate::Result<Self> {
        if bytes.len() < 32 {
            return Err(crate::Error::InvalidFormat(
                "Header must be 32 bytes".to_string(),
            ));
        }

        // Check magic
        if &bytes[0..4] != b"QRD\x01" {
            return Err(crate::Error::InvalidFormat(
                "Invalid magic number".to_string(),
            ));
        }

        let mut cursor = std::io::Cursor::new(&bytes[..]);

        let mut magic = [0u8; 4];
        cursor.read_exact(&mut magic)?;

        let mut format_major_bytes = [0u8; 2];
        cursor.read_exact(&mut format_major_bytes)?;
        let format_major = u16::from_le_bytes(format_major_bytes);

        let mut format_minor_bytes = [0u8; 2];
        cursor.read_exact(&mut format_minor_bytes)?;
        let format_minor = u16::from_le_bytes(format_minor_bytes);

        let mut schema_id = [0u8; 8];
        cursor.read_exact(&mut schema_id)?;

        let mut flags_bytes = [0u8; 2];
        cursor.read_exact(&mut flags_bytes)?;
        let flags = u16::from_le_bytes(flags_bytes);

        let mut reserved_bytes = [0u8; 2];
        cursor.read_exact(&mut reserved_bytes)?;
        let reserved = u16::from_le_bytes(reserved_bytes);

        if reserved != 0 {
            return Err(crate::Error::InvalidFormat(
                "Reserved field must be 0".to_string(),
            ));
        }

        let mut writer_version = [0u8; 12];
        cursor.read_exact(&mut writer_version)?;

        Ok(Header {
            magic,
            format_major,
            format_minor,
            schema_id,
            flags,
            reserved,
            writer_version,
        })
    }

    /// Check if encrypted
    pub fn is_encrypted(&self) -> bool {
        self.flags & 0x01 != 0
    }

    /// Set encrypted flag
    pub fn set_encrypted(&mut self, encrypted: bool) {
        if encrypted {
            self.flags |= 0x01;
        } else {
            self.flags &= !0x01;
        }
    }

    /// Check if ECC enabled
    pub fn has_ecc(&self) -> bool {
        self.flags & 0x04 != 0
    }

    /// Set ECC flag
    pub fn set_ecc(&mut self, enabled: bool) {
        if enabled {
            self.flags |= 0x04;
        } else {
            self.flags &= !0x04;
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_roundtrip() {
        let mut header = Header::new();
        header.set_encrypted(true);
        header.set_ecc(true);

        let bytes = header.to_bytes().unwrap();
        let decoded = Header::from_bytes(&bytes).unwrap();

        assert_eq!(decoded.magic, header.magic);
        assert_eq!(decoded.format_major, header.format_major);
        assert_eq!(decoded.flags, header.flags);
        assert!(decoded.is_encrypted());
        assert!(decoded.has_ecc());
    }

    #[test]
    fn test_header_invalid_magic() {
        let mut bytes = [0u8; 32];
        bytes[0] = 0xFF;
        let result = Header::from_bytes(&bytes);
        assert!(result.is_err());
    }
}
