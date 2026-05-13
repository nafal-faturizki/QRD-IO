//! Stream I/O abstraction for append-only operations

use std::io::{Write, Seek, SeekFrom};

/// Append-only writer for QRD files
pub struct AppendWriter<W: Write + Seek> {
    writer: W,
    position: u64,
}

impl<W: Write + Seek> AppendWriter<W> {
    /// Create a new append writer
    pub fn new(mut writer: W) -> crate::Result<Self> {
        let position = writer.seek(SeekFrom::End(0))?;
        Ok(Self { writer, position })
    }

    /// Write data and return number of bytes written
    pub fn write_data(&mut self, data: &[u8]) -> crate::Result<u64> {
        let n = self.writer.write(data)? as u64;
        self.position += n;
        Ok(n)
    }

    /// Get current position
    pub fn position(&self) -> u64 {
        self.position
    }

    /// Flush writer
    pub fn flush(&mut self) -> crate::Result<()> {
        self.writer.flush()?;
        Ok(())
    }

    /// Get mutable reference to underlying writer
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Consume and return underlying writer
    pub fn into_inner(self) -> W {
        self.writer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_append_writer() {
        let buf = Vec::new();
        let cursor = Cursor::new(buf);
        let mut writer = AppendWriter::new(cursor).unwrap();

        let n = writer.write_data(b"Hello").unwrap();
        assert_eq!(n, 5);
        assert_eq!(writer.position(), 5);

        let n = writer.write_data(b", ").unwrap();
        assert_eq!(n, 2);
        assert_eq!(writer.position(), 7);

        let n = writer.write_data(b"World!").unwrap();
        assert_eq!(n, 6);
        assert_eq!(writer.position(), 13);

        let inner = writer.into_inner();
        assert_eq!(inner.get_ref(), b"Hello, World!");
    }
}
