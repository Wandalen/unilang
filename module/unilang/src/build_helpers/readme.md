# Build Helpers

Utilities used by `build.rs` to analyze and generate code at build time.

## Files

| File | Responsibility |
|------|----------------|
| `type_analyzer.rs` | Detect type mismatches in YAML argument definitions |
| `hint_generator.rs` | Emit `cargo:warning` hints for detected issues |
