//! QRD Reader - streaming columnar data reading

use std::io::Read;

/// QRD Reader for streaming columnar data
pub struct Reader<R: Read> {
    reader: R,
    // TODO: Add footer caching
}

impl<R: Read> Reader<R> {
    /// Open a reader
    pub fn open(reader: R) -> crate::Result<Self> {
        Ok(Self { reader })
    }

    /// Read column data
    pub fn read_column(&mut self, _index: usize) -> crate::Result<Vec<u8>> {
        // TODO: Implement column reading from row groups
        Ok(Vec::new())
    }
}
