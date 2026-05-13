# Contributing to QRD-SDK

Thank you for your interest in contributing to QRD-SDK! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Please be respectful and constructive in all interactions with other contributors.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/QRD-IO.git`
3. Add upstream remote: `git remote add upstream https://github.com/nafal-faturizki/QRD-IO.git`
4. Create a feature branch: `git checkout -b feature/your-feature-name`

## Development Workflow

### Prerequisites
- Rust 1.70+ (stable)
- WASM target: `rustup target add wasm32-unknown-unknown`
- Development tools: `make install-hooks`

### Building
```bash
make build          # Build in release mode
make check          # Check all features
make test           # Run tests
make lint           # Run clippy
make fmt            # Format code
```

### Code Style

We follow standard Rust conventions:
- Use `cargo fmt` for formatting (configured in `rustfmt.toml`)
- Use `cargo clippy` for linting (configured in `clippy.toml`)
- Maximum line width: 100 characters
- Spaces for indentation (4 spaces)

### Testing Requirements

- **All public APIs must have tests**: Unit tests, integration tests, or property-based tests
- **Zero-panic policy for parsers**: No `unwrap()`, `panic!()`, or `todo!()` in parsing code
- **Fuzzing**: Security-critical paths should have fuzzing tests
- **Benchmarks**: Performance-critical code should have benchmarks
- Run `make test-all` before submitting PR

### Documentation Requirements

- Public APIs must have doc comments with examples
- Complex algorithms must have explanation comments
- Security-sensitive operations must include threat model documentation
- Update CHANGELOG.md for user-facing changes

### Commit Guidelines

- Use clear, descriptive commit messages
- Reference issues when applicable: `Fixes #123` or `Relates to #456`
- Keep commits focused on single logical changes
- Format: `[area] Short description` (e.g., `[core] Implement row group writer`)

### Pull Request Process

1. Create a PR with a clear title and description
2. Link related issues using `Fixes #123`
3. Ensure all tests pass: `make test-all`
4. Ensure formatting is correct: `make fmt-check`
5. Ensure linting passes: `make lint`
6. Request review from maintainers
7. Address review feedback
8. Squash commits if requested by maintainer

## Architecture Guidelines

QRD-SDK has strict architectural constraints. **DO NOT:**

- ❌ Simplify the security model
- ❌ Change binary format contracts without versioning
- ❌ Change streaming architecture contracts
- ❌ Skip zero-panic policy in parser code
- ❌ Use unwrap() on external input
- ❌ Generate placeholder code
- ❌ Buffer entire files in memory
- ❌ Duplicate business logic across SDKs

**DO:**

- ✅ Maintain deterministic binary output
- ✅ Preserve append-only streaming architecture
- ✅ Keep bounded memory guarantees
- ✅ Follow cross-language fidelity rules
- ✅ Use footer-last file layout
- ✅ Maintain little-endian encoding
- ✅ Preserve zero-knowledge encryption model
- ✅ Ensure WASM compatibility

## Reporting Issues

### Security Issues
For security vulnerabilities, please email security@qrd.dev instead of using the public issue tracker.

### Bug Reports
Please include:
- Rust version: `rustc --version`
- QRD-SDK version
- Minimal reproducible example
- Expected vs actual behavior
- Platform (Linux, macOS, Windows, etc.)

### Feature Requests
Please clearly describe:
- Problem you're trying to solve
- Proposed solution
- Alternative approaches considered
- Use case and priority

## Review Process

- At least one maintainer review required
- Security-critical code requires security review
- Large architectural changes require design discussion first
- Maintainers may request changes, refactoring, or additional tests

## Getting Help

- Check existing issues and discussions
- Read documentation in `docs/`
- Review code examples in `examples/`
- Ask in discussions or on our community channels

## License

By contributing, you agree that your contributions will be licensed under the BSL-1.1 License.

---

Thank you for contributing to QRD-SDK! 🚀
