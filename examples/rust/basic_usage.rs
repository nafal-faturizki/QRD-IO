// Basic QRD usage example

use qrd_sdk::{Schema, ColumnType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a schema with multiple columns
    let schema = Schema::new()
        .with_column("id".to_string(), ColumnType::Int64, false)
        .with_column("name".to_string(), ColumnType::String, false)
        .with_column("age".to_string(), ColumnType::Int32, false)
        .with_column("score".to_string(), ColumnType::Float64, true);

    // Validate the schema
    schema.validate()?;

    println!("Schema created with {} columns:", schema.columns.len());
    for col in &schema.columns {
        println!("  - {}: {:?} (nullable: {})", col.name, col.col_type, col.nullable);
    }

    Ok(())
}
