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
| `vocabulary_enforcement.rs` | IN-1..3 spec cases — actor taxonomy and canonical term enforcement |
| `nfr_sensitive_data.rs` | IN-3 spec case — sensitive argument value excluded from error output |
| `nfr_platform.rs` | FT-4 spec case — WASM build compiles without std-only features |
| `nfr_robustness.rs` | IN-4 and IN-5 spec cases — panic safety and zero-feature build |
| `nfr_performance.rs` | IN-1 and IN-2 spec cases — static registry startup cost and lookup throughput |
| `nfr_modularity.rs` | IN-6 spec case — enabled feature is strict subset of full |
| `invariant_03_governing_principles.rs` | IN-1..5 spec cases — governing principles enforcement |
