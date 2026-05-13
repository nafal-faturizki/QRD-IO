# Binary Format Specification

## File Structure

QRD files follow a footer-last architecture: data is written sequentially, then the footer is appended at the end with metadata about all preceding data.

```
┌─────────────────────────────────────────┐
│         Row Group 1                     │
│  ┌──────────────────────────────────┐   │
│  │ Column 1 Data (encrypted)        │   │
│  ├──────────────────────────────────┤   │
│  │ Column 2 Data (encrypted)        │   │
│  ├──────────────────────────────────┤   │
│  │ Column N Data (encrypted)        │   │
│  └──────────────────────────────────┘   │
├─────────────────────────────────────────┤
│         Row Group 2                     │
│         (same structure)                 │
├─────────────────────────────────────────┤
│         Row Group N                     │
├─────────────────────────────────────────┤
│  FOOTER (Written Last!)                 │
│  - Magic: "QRD\x01"                     │
│  - Version: u32                         │
│  - Row Groups Metadata                  │
│  - Schema Definition                    │
│  - Authentication Tags                  │
│  - Footer Length: u64                   │
└─────────────────────────────────────────┘
```

## Magic Number and Version

- **Magic Number**: `0x51 0x52 0x44 0x01` (ASCII "QRD\x01")
- **Version**: `0x00000001` (Version 1, little-endian)

## Encoding

All multi-byte integers use **little-endian byte order**.

### Data Types

| Type | Size | Encoding |
|------|------|----------|
| int8 | 1 byte | Two's complement |
| int16 | 2 bytes | Two's complement, little-endian |
| int32 | 4 bytes | Two's complement, little-endian |
| int64 | 8 bytes | Two's complement, little-endian |
| uint32 | 4 bytes | Unsigned, little-endian |
| uint64 | 8 bytes | Unsigned, little-endian |
| float32 | 4 bytes | IEEE 754, little-endian |
| float64 | 8 bytes | IEEE 754, little-endian |
| bool | 1 byte | 0x00 = false, 0x01 = true |
| string | variable | Length-prefixed UTF-8 |
| binary | variable | Length-prefixed bytes |

## Encryption

Each column is encrypted independently with **ChaCha20-Poly1305 AEAD**:

1. **Key Derivation**: HKDF-SHA256 from user-provided key
2. **Nonce**: 12 random bytes per column
3. **Plaintext**: Compressed (optional) column data
4. **AAD**: Column metadata (index, size, etc.)
5. **Ciphertext**: Encrypted column data
6. **Tag**: 16-byte authentication tag

## Row Groups

Each row group contains:
- Metadata: column count, row count
- Column data (encrypted or plaintext)
- Column metadata: offset, size, nonce, tag

## Footer

The footer is written after all data and contains:
- File metadata (version, row group count)
- Schema definition
- Row group metadata
- Authentication tag (optional)
- Footer length (u64, last 8 bytes)

This allows readers to:
1. Seek to end of file
2. Read last 8 bytes (footer length)
3. Seek backward by that length
4. Read footer and all metadata

---

See [CRYPTOGRAPHY.md](CRYPTOGRAPHY.md) for encryption details.
