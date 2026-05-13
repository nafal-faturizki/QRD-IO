# QRD-SDK AUTONOMOUS DEVELOPMENT ROADMAP
# STRICT EXECUTION PROTOCOL
# SOURCE OF TRUTH: README.md

==================================================
MANDATORY STARTUP PROTOCOL
==================================================

EVERY TIME BEFORE STARTING ANY TASK:

1. READ README.md AGAIN.
2. TREAT README.md AS THE CANONICAL SPECIFICATION.
3. DO NOT INVENT ARCHITECTURE OUTSIDE README.md.
4. DO NOT SIMPLIFY SECURITY MODEL.
5. DO NOT CHANGE BINARY FORMAT CONTRACTS.
6. DO NOT CHANGE STREAMING CONTRACTS.
7. DO NOT CHANGE MEMORY GUARANTEES.
8. DO NOT CHANGE ENCRYPTION ORDER.
9. DO NOT SKIP TESTS.
10. DO NOT GENERATE PLACEHOLDER CODE.

AFTER READING README.md:
- summarize current task goals internally
- verify dependencies of current task
- continue only if prerequisites are complete

==================================================
GLOBAL ENGINEERING RULES
==================================================

CORE PRINCIPLES:
[ ] deterministic binary output
[ ] append-only streaming architecture
[ ] bounded-memory guarantees
[ ] zero-panic parser policy
[ ] cross-language fidelity
[ ] footer-last file layout
[ ] little-endian canonical encoding
[ ] zero-knowledge encrypted storage
[ ] WASM compatibility
[ ] production-grade Rust only

PROHIBITED:
[ ] no unwrap() on external input
[ ] no todo!()
[ ] no panic!() in parser paths
[ ] no fake crypto
[ ] no placeholder implementations
[ ] no full-file buffering
[ ] no architecture drift
[ ] no duplicated business logic across SDKs

==================================================
PHASE 0 — INITIAL REPOSITORY SETUP
==================================================

GOAL:
Create complete repository structure before implementing features.

CHECKLIST:

[ ] Create root Cargo workspace
[ ] Create Rust workspace configuration
[ ] Create repository folder structure
[ ] Create root documentation structure
[ ] Create CI/CD skeleton
[ ] Create formatting/linting rules
[ ] Create testing foundation
[ ] Create benchmark foundation
[ ] Create fuzzing foundation
[ ] Create SDK placeholder structure
[ ] Create tooling structure

==================================================
ROOT REPOSITORY STRUCTURE
==================================================

QRD-SDK/
│
├── core/
├── sdk/
├── tests/
├── docs/
├── tools/
├── examples/
├── benches/
├── specs/
├── scripts/
├── .github/
├── Cargo.toml
├── rust-toolchain.toml
├── clippy.toml
├── rustfmt.toml
├── Makefile
├── README.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── SECURITY.md
└── LICENSE

==================================================
FOLDER RULES
==================================================

==================================================
/core
==================================================

RULES:
- Rust source of truth
- all binary logic implemented here
- no SDK-specific logic
- deterministic implementation only

CHECKLIST:
[ ] qrd-core
[ ] qrd-ffi
[ ] qrd-wasm

--------------------------------------------------
/core/qrd-core
--------------------------------------------------

RULES:
- canonical implementation
- no unsafe unless documented
- parser hardening mandatory
- all algorithms tested
- memory bounds enforced

REQUIRED MODULES:

[ ] schema/
[ ] writer/
[ ] reader/
[ ] encoding/
[ ] compression/
[ ] encryption/
[ ] ecc/
[ ] integrity/
[ ] columnar/
[ ] error/
[ ] footer/
[ ] header/
[ ] io/
[ ] utils/

--------------------------------------------------
/core/qrd-ffi
--------------------------------------------------

RULES:
- stable C ABI only
- no business logic duplication
- thin wrapper over qrd-core

CHECKLIST:
[ ] FFI bindings
[ ] opaque pointer management
[ ] C headers
[ ] memory safety validation

--------------------------------------------------
/core/qrd-wasm
--------------------------------------------------

RULES:
- browser-compatible
- no platform assumptions
- wasm-bindgen only
- minimal allocations

CHECKLIST:
[ ] WASM bindings
[ ] browser APIs
[ ] JS glue generation
[ ] browser memory tests

==================================================
/sdk
==================================================

RULES:
- thin wrappers only
- NEVER reimplement binary logic
- all SDKs depend on Rust core

CHECKLIST:
[ ] python/
[ ] typescript/
[ ] go/
[ ] java/

--------------------------------------------------
/sdk/python
--------------------------------------------------

RULES:
- PyO3 binding only
- idiomatic Python API
- no duplicate encoding logic

CHECKLIST:
[ ] FileReader
[ ] FileWriter
[ ] schema bindings
[ ] encryption bindings

--------------------------------------------------
/sdk/typescript
--------------------------------------------------

RULES:
- WASM-backed
- browser-first
- edge-compatible

CHECKLIST:
[ ] ESM package
[ ] WASM loader
[ ] browser examples
[ ] typed APIs

--------------------------------------------------
/sdk/go
--------------------------------------------------

RULES:
- CGO wrapper only
- no duplicated algorithms

CHECKLIST:
[ ] reader bindings
[ ] writer bindings
[ ] memory safety validation

--------------------------------------------------
/sdk/java
--------------------------------------------------

RULES:
- JNI wrapper only
- deterministic interop

CHECKLIST:
[ ] JNI bridge
[ ] Maven packaging
[ ] Java API docs

==================================================
/tests
==================================================

RULES:
- mandatory for every module
- every bug gets regression test
- no feature merged without tests

CHECKLIST:
[ ] unit/
[ ] property/
[ ] integration/
[ ] golden/
[ ] fuzz/
[ ] regression/
[ ] compliance/

==================================================
/docs
==================================================

RULES:
- architecture-first documentation
- every public API documented
- cryptography documented formally

CHECKLIST:
[ ] FORMAT_SPEC.md
[ ] ARCHITECTURE.md
[ ] SECURITY_AUDIT.md
[ ] THREAT_MODEL.md
[ ] CRYPTOGRAPHY.md
[ ] STREAMING_MODEL.md
[ ] MEMORY_MODEL.md
[ ] PERFORMANCE.md
[ ] COMPATIBILITY.md

==================================================
/tools
==================================================

RULES:
- production-grade utilities only
- no experimental tools here

CHECKLIST:
[ ] qrd-inspect
[ ] qrd-verify
[ ] qrd-convert
[ ] qrd-keygen

==================================================
/specs
==================================================

RULES:
- future proposals only
- no unstable features in core

CHECKLIST:
[ ] extension proposals
[ ] roadmap drafts
[ ] RFC documents

==================================================
PHASE 1 — CORE FOUNDATION
==================================================

CHECKLIST:

[ ] Create error taxonomy
[ ] Create binary IO helpers
[ ] Create schema system
[ ] Create serialization helpers
[ ] Create endian-safe primitives
[ ] Create schema fingerprinting
[ ] Create CRC32 utilities

EXIT REQUIREMENTS:
[ ] all modules compile
[ ] unit tests pass
[ ] clippy clean
[ ] docs generated

==================================================
PHASE 2 — FILE FORMAT
==================================================

CHECKLIST:

[ ] Implement file header
[ ] Implement footer
[ ] Implement row-group metadata
[ ] Implement statistics metadata
[ ] Implement binary parser
[ ] Implement binary serializer
[ ] Implement parser validation
[ ] Implement corruption detection

EXIT REQUIREMENTS:
[ ] malformed files rejected
[ ] no parser panic
[ ] CRC validated
[ ] deterministic serialization verified

==================================================
PHASE 3 — ENCODING ENGINE
==================================================

CHECKLIST:

[ ] PLAIN
[ ] RLE
[ ] BIT_PACKED
[ ] DELTA_BINARY
[ ] DELTA_BYTE_ARRAY
[ ] BYTE_STREAM_SPLIT
[ ] DICT_RLE

FOR EVERY ENCODING:
[ ] encoder
[ ] decoder
[ ] unit tests
[ ] property tests
[ ] fuzz target
[ ] benchmark

==================================================
PHASE 4 — COMPRESSION
==================================================

CHECKLIST:

[ ] NONE
[ ] LZ4
[ ] ZSTD
[ ] adaptive codec selection
[ ] entropy estimation

==================================================
PHASE 5 — ENCRYPTION
==================================================

CHECKLIST:

[ ] HKDF-SHA256
[ ] AES-256-GCM
[ ] nonce management
[ ] auth tags
[ ] encrypted statistics
[ ] key metadata

MANDATORY:
[ ] compression before encryption
[ ] unique nonce per chunk
[ ] NIST vector validation

==================================================
PHASE 6 — STREAMING WRITER
==================================================

CHECKLIST:

[ ] row buffering
[ ] columnar transpose
[ ] bounded memory
[ ] row-group flush
[ ] append-only writes
[ ] footer-last finalize

PIPELINE:
ENCODE
→ COMPRESS
→ ENCRYPT
→ CRC32
→ ECC
→ WRITE

==================================================
PHASE 7 — FILE READER
==================================================

CHECKLIST:

[ ] footer-first parsing
[ ] schema inspection
[ ] partial column reads
[ ] row-group projection
[ ] selective decryption
[ ] integrity verification

==================================================
PHASE 8 — ECC
==================================================

CHECKLIST:

[ ] Reed-Solomon encoding
[ ] parity chunk generation
[ ] corruption recovery
[ ] degraded media testing

==================================================
PHASE 9 — CROSS-LANGUAGE
==================================================

CHECKLIST:

[ ] Python SDK
[ ] TypeScript SDK
[ ] Go SDK
[ ] Java SDK
[ ] cross-language golden vectors
[ ] deterministic binary validation

==================================================
PHASE 10 — HARDENING
==================================================

CHECKLIST:

[ ] fuzz parser
[ ] fuzz decryptor
[ ] fuzz encoders
[ ] memory regression tests
[ ] adversarial file tests
[ ] malformed footer tests

==================================================
PHASE 11 — PERFORMANCE
==================================================

CHECKLIST:

[ ] Criterion benchmarks
[ ] throughput tracking
[ ] memory tracking
[ ] latency tracking
[ ] regression detection

==================================================
PHASE 12 — TOOLING
==================================================

CHECKLIST:

[ ] qrd-inspect
[ ] qrd-verify
[ ] qrd-convert
[ ] qrd-keygen

==================================================
FINAL RELEASE CHECKLIST
==================================================

[ ] all tests pass
[ ] fuzzing stable
[ ] benchmarks documented
[ ] docs complete
[ ] cross-language vectors verified
[ ] no clippy warnings
[ ] no formatting issues
[ ] no parser panics
[ ] deterministic outputs verified
[ ] WASM build verified
[ ] CI green
[ ] release artifacts generated

==================================================
AUTONOMOUS EXECUTION RULES
==================================================

AFTER EVERY TASK:

1. READ README.md AGAIN
2. VERIFY CURRENT PHASE
3. RUN TESTS
4. RUN CLIPPY
5. RUN FORMATTER
6. VERIFY ARCHITECTURE CONSISTENCY
7. VERIFY MEMORY CONTRACT
8. VERIFY SECURITY CONTRACT
9. VERIFY DETERMINISTIC OUTPUT
10. COMMIT ONLY LOGICAL CHANGES

NEVER SKIP A CHECKLIST ITEM.
NEVER ADVANCE PHASES PREMATURELY.

BEGIN FROM:
PHASE 0 — INITIAL REPOSITORY SETUP
