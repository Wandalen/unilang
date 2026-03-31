# Multi-YAML Aggregation

Aggregates multiple YAML command definition files and generates static Rust code.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | Module re-exports |
| `aggregator/` | Merge multiple YAML sources, detect conflicts, emit codegen |
| `builder.rs` | Builder API for constructing aggregation configurations |
