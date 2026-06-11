# Feature Doc Entity

Behavioral feature requirements (FR-*) that define what the unilang framework must do.

### Scope

- **Purpose:** Document functional requirements with FR-* identifiers that the system must satisfy
- **Responsibility:** Answers: what behaviors must the system exhibit, what are the acceptance criteria
- **In Scope:** Functional requirements, FR-* identifiers, behavior specifications, acceptance criteria
- **Out of Scope:** NFRs, design philosophy, public API contracts, implementation guides

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Command Registry](001_command_registry.md) | FR-REG-1 through FR-REG-9: registration requirements | ✅ |
| 002 | [Argument System](002_argument_system.md) | FR-ARG-1 through FR-ARG-8: argument parsing and types | ✅ |
| 003 | [Pipeline](003_pipeline.md) | FR-PIPE-1 through FR-PIPE-4: execution pipeline | ✅ |
| 004 | [Help System](004_help_system.md) | FR-HELP-1 through FR-HELP-8: help generation, auto-help, ?? parameter, self-exclusion | ✅ |
| 005 | [REPL Interactive](005_repl_interactive.md) | FR-REPL-1, FR-INTERACTIVE-1, FR-MOD-WASM-REPL: modality support | ✅ |
