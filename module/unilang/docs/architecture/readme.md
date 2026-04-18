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
