# Incremental Tests

Phase-by-phase integration and unit tests tracking implementation milestones.

## Files / Directories

| File / Directory | Responsibility |
|------------------|----------------|
| `mod.rs` | Module root; shared test utilities |
| `integration_tests.rs` | Cross-phase integration test runner |
| `unit_tests.rs` | Cross-phase unit test runner |
| `phase1/` | Phase 1 foundational tests (pipeline, setup, try_build) |
| `phase2/` | Phase 2 feature tests (CLI, loaders, registry, help) |
| `phase3/` | Phase 3 refinement tests (already has readme.md) |
| `phase4/` | Phase 4 extended tests |
| `phase5/` | Phase 5 advanced tests |
