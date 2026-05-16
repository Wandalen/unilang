# Invariant Doc Entity

System properties and constraints that must always hold, regardless of code path or configuration.

### Scope

- **Purpose:** Document system invariants: NFRs, vocabulary contracts, and governing principles
- **Responsibility:** Answers: what constraints must always be true, what are the measurable thresholds
- **In Scope:** Performance NFRs with measurable thresholds, vocabulary definitions, framework governing principles
- **Out of Scope:** Feature requirements, public API specs, project goals, conformance checklists

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [System Actors Vocabulary](001_system_actors_vocabulary.md) | Canonical actor and term definitions that must remain stable | ✅ |
| 002 | [Non-Functional Requirements](002_non_functional_requirements.md) | Performance, security, and modularity thresholds | ✅ |
| 003 | [Governing Principles](003_governing_principles.md) | Framework principles that must always guide design decisions | ✅ |
| 004 | [Workspace Dependency Standards](004_workspace_dependency_standards.md) | Dep version format, workspace centralization, and optional-dep pattern compliance | ✅ |
| 005 | [Command Naming](005_command_naming.md) | Dot-prefix requirement and explicit naming conventions for all registered commands | ✅ |
