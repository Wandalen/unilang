//! Validation Domain Tests
//!
//! CI-level validation tests that verify crate-wide properties: clippy cleanliness,
//! ABI compatibility, feature gate correctness, PHF re-export accessibility,
//! and documentation example compilation.

mod validation {
  mod abi_compatibility;
  mod clippy;
  mod core;
  mod direct_import;
  mod doc_examples;
  mod feature_gate;
  mod phf_indirect;
}
