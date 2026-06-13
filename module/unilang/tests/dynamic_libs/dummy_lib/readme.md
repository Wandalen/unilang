# dummy_lib

Minimal Rust library crate used as a dynamic library fixture for dynamic-loading tests.

## Files

| File | Responsibility |
|------|----------------|
| `Cargo.toml` | Crate manifest declaring cdylib crate type |
| `src/lib.rs` | Exported symbols loaded at runtime in tests |
