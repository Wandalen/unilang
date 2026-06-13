# Aggregator Module

Split from `multi_yaml/aggregator.rs`. Merges multiple YAML command definition files.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | Module entry point and public re-exports |
| `core.rs` | `CommandAggregator` struct and aggregation orchestration |
| `codegen.rs` | Code generation for static PHF command maps from aggregated data |
| `conflict.rs` | Conflict detection and resolution for duplicate command names |
| `aggregation_fns.rs` | Convenience free functions for registry construction |
