# Security Policy

## Reporting Security Vulnerabilities

**DO NOT** open public GitHub issues for security vulnerabilities. Instead, please report security issues to:

📧 **security@qrd.dev**

Include:
1. Description of vulnerability
2. Affected version(s)
3. Steps to reproduce (if applicable)
4. Impact assessment
5. Suggested fix (if you have one)

We will:
- Acknowledge receipt within 48 hours
- Provide status updates every 7 days
- Work toward a fix and coordinate disclosure
- Credit you in the security advisory (if desired)

## Security Model

QRD-SDK is designed with privacy and security as first-class properties:

### Cryptographic Guarantees

- **Encryption**: ChaCha20-Poly1305 AEAD cipher (RFC 7539, FIPS-140-3 aligned)
- **Key Derivation**: HKDF with SHA-256 (RFC 5869)
- **Authentication**: HMAC-SHA-256 for integrity verification
- **Randomness**: `getrandom` for cryptographically secure random number generation

### Zero-Knowledge Design

- Servers cannot read encrypted data without the decryption key
- File structure is unencrypted but contains no plaintext content
- Metadata encrypted separately from data columns
- Footer contains keyed authentication tags for integrity

### Threat Model

#### Assumptions
- Decryption keys are kept secure by the client
- Keys are transmitted through secure channels (TLS, out-of-band, etc.)
- Hardware/platform provides reasonable entropy for key derivation
- Threat actor has access to stored files but not keys

#### Protections Against
- ✅ Passive adversary with file access (confidentiality)
- ✅ Data tampering detection (integrity)
- ✅ Replay attacks (per-column authentication)
- ✅ Offline brute-force attacks (proper key derivation parameters)
- ✅ Side-channel attacks from unverified operations

#### Does NOT Protect Against
- ❌ Compromised client/device (key extraction)
- ❌ Malware with code execution privileges
- ❌ Timing attacks from local adversaries
- ❌ Quantum computers (for cryptanalysis)
- ❌ Non-cryptographic attacks (social engineering, etc.)

## Code Security Practices

### Parser Safety

All parsers in QRD-SDK follow a **zero-panic policy**:

- ❌ NO `unwrap()` on external input
- ❌ NO `panic!()` in parsing code paths
- ❌ NO `todo!()` or `unimplemented!()` in production code
- ✅ All errors handled via `Result` types
- ✅ Fuzzing tests for all parser entry points
- ✅ Bounded memory guarantees for streaming

### Memory Safety

Rust provides memory safety guarantees:
- ✅ Automatic bounds checking
- ✅ No buffer overflows
- ✅ No use-after-free
- ✅ No data races (single-threaded or properly synchronized)
- ✅ Zeroinized sensitive data via `zeroize` crate

### Cryptographic Safety

- ✅ Use of well-audited cryptographic libraries
- ✅ No custom cryptographic implementations
- ✅ Proper use of authenticated encryption (AEAD)
- ✅ Key material zeroinized after use
- ✅ Secure random number generation via `getrandom`

## Dependency Audit

We maintain:
- Minimal external dependencies
- Regular audit of dependencies with `cargo audit`
- Preference for widely-used, well-maintained crates
- Monitoring of security advisories

### Key Dependencies

| Crate | Purpose | Audit Status |
|-------|---------|--------------|
| `chacha20poly1305` | AEAD encryption | ✅ Verified |
| `sha2`, `hmac` | Cryptographic hashing | ✅ Verified |
| `zeroize` | Secure memory clearing | ✅ Verified |
| `serde`, `bincode` | Serialization | ✅ Verified |

## Testing & Verification

- **Unit tests**: All public APIs and security properties
- **Integration tests**: End-to-end encryption/decryption flows
- **Fuzzing**: Parser robustness with `cargo-fuzz`
- **Property-based tests**: Encryption determinism and fidelity
- **Benchmarks**: No performance regressions in security operations

## Vulnerability Disclosure Timeline

We follow responsible disclosure:

1. **Initial Report**: Vulnerability reported to security@qrd.dev
2. **Triage** (48 hours): Initial assessment and impact evaluation
3. **Fix Development** (1-7 days): Patch development and testing
4. **Testing** (3-5 days): Internal testing and quality assurance
5. **Release** (14-30 days): Coordinated release with security advisory
6. **Public Disclosure**: Advisory posted after patch release

## Security Advisories

Past security advisories are published in [docs/security/ADVISORIES.md](docs/security/ADVISORIES.md).

## Compliance

QRD-SDK follows:
- **FIPS-140-3**: Aligned cryptographic algorithm usage
- **RFC 5869**: HKDF key derivation
- **RFC 7539**: ChaCha20 and Poly1305 specification
- **OWASP**: Secure coding practices
- **CWE**: Common Weakness Enumeration awareness

## Security Review Process

Security-critical code (parsing, encryption, key handling) requires:
1. Code review by security-aware maintainer
2. Additional scrutiny for architecture changes
3. Threat modeling for new features
4. Fuzzing test coverage for parsers
5. Cryptographic validation by qualified reviewer

## Contact & Disclosure

- **Security Issues**: security@qrd.dev
- **Public Disclosure**: GitHub Security Advisory
- **Community**: GitHub Discussions
- **Contributors**: See [CONTRIBUTING.md](CONTRIBUTING.md)

---

**Last Updated**: 2026-05-13
**Version**: 1.0.0
