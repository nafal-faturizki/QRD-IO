# Quick Start Guide

## Installation

Add QRD-SDK to your `Cargo.toml`:

```toml
[dependencies]
qrd-sdk = "1.0"
```

## Basic Usage

### Create a QRD File

```rust
use qrd_sdk::{Writer, Schema, ColumnType};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define schema
    let schema = Schema::new()
        .with_column("id".to_string(), ColumnType::Int64, false)
        .with_column("name".to_string(), ColumnType::String, false)
        .with_column("score".to_string(), ColumnType::Float64, true);

    schema.validate()?;

    // Create writer
    let file = File::create("data.qrd")?;
    let mut writer = Writer::new(file);

    // Write data (placeholder - full implementation coming)
    writer.write_column(&[0, 1, 2])?;
    writer.finish()?;

    Ok(())
}
```

### Read a QRD File

```rust
use qrd_sdk::Reader;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data.qrd")?;
    let mut reader = Reader::open(file)?;

    // Read first column
    let data = reader.read_column(0)?;
    println!("Column data: {:?}", data);

    Ok(())
}
```

## Command-line Interface

```bash
# Create QRD file from JSON
qrd create data.qrd --format json < input.json

# Read column from QRD file
qrd read data.qrd --column 0

# Show file metadata
qrd info data.qrd

# Validate file integrity
qrd validate data.qrd
```

## Encryption

Encrypt data using a password:

```rust
use qrd_sdk::crypto::Crypto;

let password = b"my-secure-password";
let salt = b"random-salt-value";
let key = Crypto::derive_key(password, salt)?;

// Key is now ready for encryption/decryption
```

## Examples

Complete examples are available in `examples/`:

```bash
# Run example
cargo run --example basic_usage
cargo run --example encryption
cargo run --example streaming
```

## Next Steps

- Read [Binary Format Specification](../docs/spec/BINARY_FORMAT.md)
- Review [Cryptography Guide](../docs/security/CRYPTOGRAPHY.md)
- Check [API Documentation](../docs/api/)
- Explore [Examples](../examples/)

---

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development setup.
