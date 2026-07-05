# Feature Test Surface

Test spec files for `docs/feature/` doc instances.
Case prefix: `FT-`. Minimum 4 cases per spec.

### Scope

- **Purpose:** Enumerate test cases covering every FR identifier in `docs/feature/`
- **Responsibility:** One spec file per feature doc instance; each case maps to at least one FR
- **In Scope:** FR-REG-1..9, FR-ARG-1..8, FR-PIPE-1..4, FR-HELP-1..8, FR-REPL-1, FR-INTERACTIVE-1, FR-MOD-WASM-REPL
- **Out of Scope:** NFR testing (covered in `invariant/`); API contract testing (covered in `api/`)

### Overview Table

| Name | Purpose | Status |
|------|---------|--------|
| `01_command_registry.md` | `feature` spec for command registry (FR-REG-1..9) | ⏳ |
| `02_argument_system.md` | `feature` spec for argument system (FR-ARG-1..8) | ⏳ |
| `03_pipeline.md` | `feature` spec for pipeline orchestration (FR-PIPE-1..4) | ⏳ |
| `04_help_system.md` | `feature` spec for help system (FR-HELP-1..8) | ⏳ |
| `05_repl_interactive.md` | `feature` spec for REPL and interactive mode | ⏳ |
