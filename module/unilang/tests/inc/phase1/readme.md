# Phase 1 Tests

Foundational integration tests establishing the core pipeline and build harness.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | Module root for phase 1 tests |
| `foundational_setup.rs` | Verifies core crate setup and basic imports |
| `full_pipeline_test.rs` | End-to-end pipeline: register, parse, execute |
| `try_build.rs` | Validates build-time code generation runs cleanly |
