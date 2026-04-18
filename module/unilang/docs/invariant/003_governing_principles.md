# Invariant: Governing Principles

### Scope

- **Purpose:** Define framework principles that must always guide every design decision and implementation choice
- **Responsibility:** Framework governing principles, development principles, design philosophy
- **In Scope:** Must-always-hold framework principles, explicit naming rules, fail-fast mandate
- **Out of Scope:** Feature requirements, specific API contracts, performance thresholds

### Invariant Statement

All principles defined in this document MUST be respected in every code change, API design, and architectural decision. No feature, convenience, or performance optimization justifies violating these principles.

### Enforcement Mechanism

- Code review checklist references these principles
- PR template includes principle compliance check
- Architecture review for any proposal that might violate a principle

### Violation Consequences

Principle violation leads to: predictability loss, API inconsistency, silent failures, and maintenance burden accumulation.

Framework-level governing principles and core development principles that guide all architectural decisions.

### Framework Governing Principles

The unilang framework is built on fundamental principles that guide all architectural decisions and implementation details.

### Minimum Implicit Magic

The framework **must** minimize implicit behavior and transformations to maximize predictability:

- **Explicit Operations**: All operations should be explicit rather than implicit
- **Predictable Behavior**: What you specify is exactly what you get — no hidden transformations
- **Clear APIs**: Function behavior should be obvious from signatures and documentation
- **No Surprising Side Effects**: Commands and functions should behave exactly as documented

### Single Source of Truth

Each piece of information **must** have exactly one authoritative source:

- **Command Definitions**: Commands registered exactly as specified, used exactly as registered
- **Configuration**: One canonical location for each configuration setting
- **Documentation**: Single authoritative source for each concept or procedure

### Fail-Fast Validation

The framework **must** detect and report errors as early as possible:

- **Registration Time**: Invalid command definitions rejected immediately during registration
- **Parse Time**: Syntax errors detected during parsing phase
- **Semantic Analysis**: Type and validation errors caught before execution
- **Clear Error Messages**: All errors include actionable guidance for resolution

### Explicit Dependencies

All dependencies and relationships **must** be made explicit:

- **Command Dependencies**: Clear specification of required arguments and constraints
- **Type Dependencies**: Explicit type requirements and conversions
- **System Dependencies**: Clear documentation of external requirements

### Consistent Help Access

The framework **must** provide standardized, predictable help access for all commands:

- **Universal Help Commands**: Every command `.command` automatically generates a `.command.help` counterpart
- **Uniform Help Parameter**: The `??` parameter provides consistent help access across all commands
- **Help Convention APIs**: Developer-friendly APIs make following help conventions effortless
- **Discoverability**: Users can always find help through predictable patterns

These principles serve as the foundation for all design decisions and implementation choices throughout the framework.

### Make Illegal States Unrepresentable

The framework **must** make invalid domain states impossible to construct, not just rejected after construction. Triggered by the wplan bug (Task 085), where `multiple:true` with non-List storage caused silent data loss:

- **Parse, Don't Validate**: Accept only data that's already valid by type construction
- **Prefer Compile-Time Errors**: Catch bugs during `cargo build`, not during execution
- **Prefer Runtime Errors Over Silent Failures**: If compile-time prevention isn't possible, fail loudly
- **No Partial Initialization**: Every constructed value must be fully valid

**Three-layer defense:**
| Layer | When | Mechanism |
|-------|------|-----------|
| Build-Time | `cargo build` | `build/codegen.rs` validates YAML/JSON manifests |
| Registration-Time | Runtime API calls | `validate_command_for_registration()` |
| Execution-Time | Command execution | Interpreter checks for handler presence |

### Core Principles of Development

### Repository as SSOT

The project's Git repository **must** be the absolute single source of truth for all project-related information. This includes specifications, documentation, source code, configuration files, and architectural diagrams.

### Documentation-First Development

All changes to the system's functionality or architecture **must** be documented in the relevant specification files *before* implementation begins.

### Review-Driven Change Control

All modifications to the repository, without exception, **must** go through a formal Pull Request review.

### Radical Transparency and Auditability

The development process **must** be fully transparent and auditable. All significant decisions and discussions **must** be captured in writing within the relevant Pull Request or a linked issue tracker. The repository's history should provide a clear, chronological narrative of the project's evolution.

### File Naming Conventions

All file names within the project repository **must** use lowercase `snake_case`.

### Explicit Command Naming Principle

The framework **must** adhere to the principle of explicit command naming with minimal implicit transformations:

- **Commands as Registered**: Command names **must** be used exactly as registered, without automatic prefix addition or name transformation
- **Dot Prefix Requirement**: All commands **must** be registered with explicit dot prefix (e.g., `.chat`, `.session.list`)
- **Validation Enforcement**: The framework **must** reject command registrations that do not start with a dot prefix
- **No Implicit Behavior**: The system **must not** automatically add dots, modify namespaces, or transform command names during registration or execution
- **Principle of Least Surprise**: Command behavior should be predictable — what you register is exactly what gets executed

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [invariant/001_system_actors_vocabulary.md](001_system_actors_vocabulary.md) | Vocabulary these principles govern |
| doc | [invariant/002_non_functional_requirements.md](002_non_functional_requirements.md) | NFRs that embody these principles |
| doc | [architecture/001_mandates.md](../architecture/001_mandates.md) | Architectural mandates derived from these principles |
