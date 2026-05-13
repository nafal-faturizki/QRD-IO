#[cfg(test)]
mod tests {
    use qrd_core::types::{ColumnType, Schema, ColumnDef};

    #[test]
    fn test_schema_creation() {
        let schema = Schema::new()
            .with_column("id".to_string(), ColumnType::Int64, false)
            .with_column("name".to_string(), ColumnType::String, false);

        assert_eq!(schema.columns.len(), 2);
        assert_eq!(schema.columns[0].name, "id");
        assert_eq!(schema.columns[1].name, "name");
    }

    #[test]
    fn test_schema_validation() {
        let schema = Schema::new();
        let result = schema.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_column_type_sizes() {
        assert_eq!(ColumnType::Int8.byte_size(), Some(1));
        assert_eq!(ColumnType::Int32.byte_size(), Some(4));
        assert_eq!(ColumnType::Int64.byte_size(), Some(8));
        assert_eq!(ColumnType::String.byte_size(), None);
    }
}
