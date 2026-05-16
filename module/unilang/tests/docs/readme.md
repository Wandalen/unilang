# Test Surface Documentation

Test surface specification files for the `unilang` crate.
Governed by `test_surface.rulebook.md`.

### Scope

- **Purpose:** House test spec files derived from authoritative `docs/` source documents
- **Responsibility:** Define Given/When/Then test cases for all documented behavioral elements
- **In Scope:** Feature specs (`feature/`), invariant specs (`invariant/`), API specs (`api/`)
- **Out of Scope:** Actual Rust test implementations (those live in `tests/<domain>/`); CLI test specs (no `docs/cli/` exists — library crate); `docs/analysis/` instances (analysis docs contain findings and recommendations, not behavioral contracts — no executable test cases derive from them)

### Subdirectories

| Directory | Responsibility | Specs |
|-----------|----------------|------:|
| `feature/` | Behavioral requirement specs for `docs/feature/` instances | 5 |
| `invariant/` | System invariant enforcement specs for `docs/invariant/` instances | 4 |
| `api/` | Public API contract specs for `docs/api/` instances | 1 |
