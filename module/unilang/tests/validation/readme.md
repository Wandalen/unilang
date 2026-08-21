# Validation Domain

CI-level validation tests that verify crate-wide properties: clippy cleanliness, ABI compatibility, feature gate correctness, PHF re-export accessibility, and documentation example compilation.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `abi_compatibility.rs` | Cross-crate type ABI compatibility via external build |
| `clippy.rs` | Crate passes clippy with `-D warnings` |
| `core_test.rs` | Internal validation logic unit-level tests |
| `direct_import.rs` | Direct import works without feature-gate ceremony |
| `doc_examples.rs` | Documentation examples compile and produce expected output |
| `feature_gate.rs` | `enabled`/`full` feature gates enable/disable correctly |
| `fixture.rs` | Downstream-fixture manifest local-path patch support |
| `phf_indirect.rs` | PHF usable via `unilang` re-export without direct dep |
