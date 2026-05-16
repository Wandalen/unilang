# System Tests

End-to-end workflows, API compatibility, and external usage pattern tests.

## Files

| File | Responsibility |
|------|----------------|
| `end_to_end.rs` | Full parse → semantic → execute pipeline |
| `comprehensive_workflow.rs` | Complex multi-command workflow scenarios |
| `api_compatibility.rs` | Public API surface stability |
| `external_usage.rs` | Usage patterns as seen by downstream crates |
| `multi_yaml_system.rs` | Multi-YAML aggregation end-to-end |
| `argv_api.rs` | argv-based API request parsing and dispatch |
