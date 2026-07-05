# API Domain

Public API contract tests covering all 17 public types and compatibility guarantees defined in `docs/api/001_public_types.md`.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `public_types.rs` | AP-1..19 spec cases — builder API, round-trip, type coverage, private fields, static/dynamic conversions, env vars, typed extraction |
| `error_codes.rs` | AP-1..6 spec cases — error code variants, conditions, and derive contracts |
