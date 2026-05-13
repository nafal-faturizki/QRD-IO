//! Reader for columnar data

use std::io::Read;

/// QRD Reader
pub struct Reader<R: Read> {
    reader: R,
}

impl<R: Read> Reader<R> {
    /// Open a reader
    pub fn open(reader: R) -> crate::Result<Self> {
        Ok(Self { reader })
    }

    /// Read column data
    pub fn read_column(&mut self, _index: usize) -> crate::Result<Vec<u8>> {
        // TODO: Implement column reading
        Ok(Vec::new())
    }
}
