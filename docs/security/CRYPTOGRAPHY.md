# Cryptography & Encryption

## Overview

QRD implements **zero-knowledge encryption as a format property**, not as an optional layer. Every QRD file can be encrypted such that servers storing the file cannot read its contents without the decryption key.

## Algorithms

All cryptographic primitives are FIPS-140-3 aligned:

### Authenticated Encryption

- **Cipher**: ChaCha20-Poly1305 (RFC 7539)
- **Mode**: AEAD (Authenticated Encryption with Additional Data)
- **Key Size**: 256 bits (32 bytes)
- **Nonce Size**: 96 bits (12 bytes)
- **Tag Size**: 128 bits (16 bytes)

### Key Derivation

- **Algorithm**: HKDF (HMAC-based Key Derivation Function)
- **Hash**: SHA-256 (RFC 5869)
- **Salt**: User-provided or random
- **Info**: Encoded as b"qrd-key"

### Random Number Generation

- **Source**: `getrandom` crate (OS-provided entropy)
- **Linux**: `/dev/urandom`
- **macOS**: `getentropy()`
- **Windows**: `CryptGenRandom()`
- **WebAssembly**: `crypto.getRandomValues()`

## Threat Model

### Security Goals

1. **Confidentiality**: Attacker with file access cannot read plaintext without key
2. **Integrity**: Tampered ciphertext is detected via authentication tags
3. **Authenticity**: Recipients verify data comes from legitimate source
4. **Forward Secrecy**: Compromise of one file doesn't affect others (independent keys)

### Assumptions

✅ **We assume:**
- Decryption keys are kept secure by the client
- Keys transmitted through secure channels (TLS, out-of-band)
- OS provides adequate entropy
- Recipient has the decryption key

❌ **We do NOT assume:**
- Protection against key extraction via side-channel attacks
- Protection against malware running on the device
- Protection against quantum computers
- Protection against timing attacks on the same device

### Attack Resistance

| Attack | Resistance |
|--------|-----------|
| Passive eavesdropping (file at rest) | ✅ Full |
| Tampering detection | ✅ Full |
| Offline brute-force (weak keys) | ✅ Full (HKDF with SHA256) |
| Replay attacks (same file twice) | ✅ Full (per-column nonces) |
| Timing attacks from remote | ✅ Full |
| Malware with code execution | ❌ No |
| Quantum computers | ❌ No (classical crypto) |
| Side-channel from local process | ❌ No |

## Implementation Details

### Key Derivation

```
DerivedKey = HKDF-SHA256(
    IKM = UserPassword,
    Salt = RandomSalt,
    Info = b"qrd-key",
    L = 32
)
```

### Column Encryption

```
Nonce = random(12 bytes)
AAD = Column metadata (index, size, type, etc.)
Ciphertext || Tag = ChaCha20Poly1305.encrypt(
    Key = DerivedKey,
    Nonce = Nonce,
    Plaintext = ColumnData,
    AAD = AAD
)
```

### Verification

```
PlaintextOrError = ChaCha20Poly1305.decrypt(
    Key = DerivedKey,
    Nonce = StoredNonce,
    Ciphertext = StoredCiphertext,
    AAD = StoredAAD,
    Tag = StoredTag
)
```

## Security Practices

### Key Material Handling

- ✅ Keys are zeroinized after use (via `zeroize` crate)
- ✅ No keys in logs or error messages
- ✅ No keys in debug output
- ✅ Keys not serialized to disk unencrypted

### Parser Security

- ✅ No `unwrap()` on external input
- ✅ No panics in cryptographic code
- ✅ All decryption failures return `Error` type
- ✅ Fuzzing tests for encryption/decryption paths

### Dependency Auditing

All cryptographic dependencies are regularly audited:

```bash
cargo audit
```

### Compliance

- FIPS-140-3 Algorithm Validation
- RFC 5869 (HKDF)
- RFC 7539 (ChaCha20-Poly1305)
- OWASP Secure Coding Practices

## Limitations & Recommendations

### What QRD Encryption DOESN'T Protect

- ❌ Metadata about data structure (column names, types)
- ❌ Row counts or approximate data size
- ❌ Access patterns (which columns are read)
- ❌ Plaintext keys if leaked from memory

### Recommendations for Users

1. **Store keys securely**: Use password managers, key vaults, hardware tokens
2. **Rotate keys**: Periodically rederive from new passwords
3. **Transport securely**: Use TLS, VPN, or out-of-band channels for keys
4. **Audit access**: Monitor who can access encrypted files
5. **Test recovery**: Verify you can decrypt files with stored keys

## References

- [ChaCha20 and Poly1305 (RFC 7539)](https://tools.ietf.org/html/rfc7539)
- [HKDF (RFC 5869)](https://tools.ietf.org/html/rfc5869)
- [FIPS-140-3](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.140-3.pdf)
- [getrandom Crate](https://docs.rs/getrandom/)
- [ChaCha20Poly1305 Crate](https://docs.rs/chacha20poly1305/)

---

**Last Updated**: 2026-05-13
