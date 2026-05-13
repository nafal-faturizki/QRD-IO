//! Writer for columnar data

use crate::format::Footer;
use std::io::Write;

/// QRD Writer
pub struct Writer<W: Write> {
    writer: W,
    footer: Footer,
}

impl<W: Write> Writer<W> {
    /// Create a new writer
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            footer: Footer::new(crate::format::VERSION),
        }
    }

    /// Write column data
    pub fn write_column(&mut self, _data: &[u8]) -> crate::Result<()> {
        // TODO: Implement column writing
        Ok(())
    }

    /// Finish writing and flush footer
    pub fn finish(&mut self) -> crate::Result<()> {
        // TODO: Implement footer writing
        self.writer.flush()?;
        Ok(())
    }
}
