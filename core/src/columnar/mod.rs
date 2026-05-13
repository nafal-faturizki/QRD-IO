//! Columnar data layout - row-to-column transposition

/// Columnar layout builder for row-oriented data
pub struct ColumnarLayout {
    /// Column data (one vector per column)
    columns: Vec<Vec<u8>>,
    /// Number of rows
    row_count: usize,
}

impl ColumnarLayout {
    /// Create a new columnar layout with given number of columns
    pub fn new(num_columns: usize) -> Self {
        Self {
            columns: vec![Vec::new(); num_columns],
            row_count: 0,
        }
    }

    /// Add a row of data
    ///
    /// Each element in row_data corresponds to a column
    pub fn add_row(&mut self, row_data: &[Vec<u8>]) -> crate::Result<()> {
        if row_data.len() != self.columns.len() {
            return Err(crate::Error::Other(
                format!("Row has {} columns, expected {}", 
                    row_data.len(), self.columns.len())
            ));
        }

        for (col_idx, col_data) in row_data.iter().enumerate() {
            self.columns[col_idx].extend_from_slice(col_data);
        }

        self.row_count += 1;
        Ok(())
    }

    /// Get all column data
    pub fn columns(&self) -> &[Vec<u8>] {
        &self.columns
    }

    /// Get mutable reference to columns
    pub fn columns_mut(&mut self) -> &mut [Vec<u8>] {
        &mut self.columns
    }

    /// Get number of rows
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    /// Get number of columns
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Clear all data
    pub fn clear(&mut self) {
        for col in &mut self.columns {
            col.clear();
        }
        self.row_count = 0;
    }

    /// Extract column data by index
    pub fn extract_column(&self, index: usize) -> crate::Result<Vec<u8>> {
        if index >= self.columns.len() {
            return Err(crate::Error::InvalidColumn {
                index,
                reason: "Column index out of bounds".to_string(),
            });
        }
        Ok(self.columns[index].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_columnar_layout() {
        let mut layout = ColumnarLayout::new(3);

        // Add first row: [1, 2, 3]
        layout.add_row(&[vec![1], vec![2], vec![3]]).unwrap();
        
        // Add second row: [4, 5, 6]
        layout.add_row(&[vec![4], vec![5], vec![6]]).unwrap();

        assert_eq!(layout.row_count(), 2);
        assert_eq!(layout.column_count(), 3);
        assert_eq!(layout.columns[0], vec![1, 4]);
        assert_eq!(layout.columns[1], vec![2, 5]);
        assert_eq!(layout.columns[2], vec![3, 6]);
    }

    #[test]
    fn test_extract_column() {
        let mut layout = ColumnarLayout::new(2);
        layout.add_row(&[vec![1, 2], vec![3, 4]]).unwrap();

        let col0 = layout.extract_column(0).unwrap();
        assert_eq!(col0, vec![1, 2]);

        let col1 = layout.extract_column(1).unwrap();
        assert_eq!(col1, vec![3, 4]);
    }
}
