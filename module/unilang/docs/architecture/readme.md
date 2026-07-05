# Architecture Doc Entity

Architectural decisions, mandates, ADRs, and implementation rationale for the `unilang` crate.

### Scope

- **Purpose:** Document design decisions and their rationale, system diagrams, and implementation guides
- **Responsibility:** Answers: why was it designed this way, what are the architectural constraints
- **In Scope:** Design decisions with rationale, system diagrams, ADRs, migration guides, implementation internals
- **Out of Scope:** Behavioral requirements, public API contracts, analysis reports

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Mandates](001_mandates.md) | Architectural mandates, diagrams, and crate responsibilities | ✅ |
| 002 | [Benchmark Separation](002_benchmark_separation.md) | Rationale for separate benchmark crate architecture | ✅ |
| 003 | [Vision & Scope](003_vision_scope.md) | Framework vision, scope boundaries, and design goals | ✅ |
| 004 | [Implementation Details](004_implementation_details.md) | Internal PHF optimization, build system, and codegen internals | ✅ |
| 005 | [Help Decoupling](005_help_decoupling.md) | Help system decoupling migration rationale and status | ✅ |
| 006 | [REPL Implementation](006_repl_implementation.md) | REPL feature implementation guide, flags, and usage patterns | ✅ |
| 007 | [Migration Guide](007_migration_guide.md) | Runtime-to-build-time registration migration walkthrough | ✅ |

### Type Declaration

- **Decision Criteria**: Use when documenting a design decision, mandate, or implementation rationale with narrative context (why it was designed this way); no standard type or recognized extension covers narrative architectural rationale — `invariant/` documents properties that must always hold, not the reasoning behind a design choice, and no standard type covers migration/decoupling rationale or system diagrams.
- **Contrast with invariant/**: `invariant/` documents system properties that must always hold (testable, timeless constraints); `architecture/` documents why a design was chosen, including diagrams, migration rationale, and status — it does not assert an always-true property. `architecture/` also does NOT use a formal ADR format (no `## Decision`/`## Consequences` structure); it uses free-form narrative sections (e.g., Overview, Rationale, Migration Phases, Diagrams) tailored to each decision.
- **Required Sections**: Scope, at least one narrative content section (e.g., Overview, Rationale, Migration Overview, Architectural Mandates & Design Principles — content varies per decision)
- **Overview Table Columns**: `ID`, `Name`, `Purpose`, `Status`
- **Quality Checklist**:
  - [ ] Does the instance explain the rationale behind the decision, not just describe the resulting structure?
  - [ ] Are diagrams or migration phases (if present) accurate to the current implementation state?
  - [ ] Is this genuinely a design decision/mandate/rationale, not a behavioral requirement (belongs in `feature/`) or an always-true property (belongs in `invariant/`)?
