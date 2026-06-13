# Invariant: Command Naming

### Scope

- **Purpose:** Enforce the dot-prefix naming contract for all command identifiers throughout the system
- **Responsibility:** Dot-prefix requirement, namespace format rules, naming character constraints
- **In Scope:** Fully-qualified command name format, namespace construction rules, validation boundary
- **Out of Scope:** Business naming choices, command categorization, help text conventions

### Invariant Statement

Every command name in the system MUST start with a dot prefix (e.g., `.command`, `.namespace.command`). This invariant applies at every registration boundary — runtime API, build-time YAML/JSON manifests, and static registry generation. No command may be registered, looked up, or executed without a leading dot. The framework MUST reject any attempt to register a command that does not satisfy this constraint.

### Enforcement Mechanism

- Runtime registration: `CommandRegistry::register_with_routine` validates the dot prefix via `validate_command_for_registration()` and returns an error for any command name not starting with `.`
- Build-time generation: `build.rs` applies dot-prefix normalization rules and rejects invalid manifests with actionable error messages before compilation completes
- Namespace construction: the `compute_full_name()` function always produces a dot-prefixed result regardless of the two supported YAML formats (compound name or separate namespace field)
- Static registry: `From<StaticCommandDefinition> for CommandDefinition` conversion is guaranteed not to panic only when build-time validation has already confirmed naming compliance

### Violation Consequences

A command registered without a dot prefix: (1) fails registration with a clear error, preventing silent registration of unreachable commands; (2) cannot be looked up at runtime since the parser always produces dot-prefixed command paths; (3) breaks the "define once, use everywhere" guarantee because CLI input always uses dot syntax.

### Features

| File | Relationship |
|------|--------------|
| [001_command_registry.md](../feature/001_command_registry.md) | FR-REG-6 specifies the naming rules this invariant formalizes |

### Invariants

| File | Relationship |
|------|--------------|
| [003_governing_principles.md](003_governing_principles.md) | Explicit Command Naming Principle that this invariant enforces |

### Types

| File | Relationship |
|------|--------------|
| [001_command_name.md](../type/001_command_name.md) | CommandName newtype enforces this invariant |
| [002_namespace_type.md](../type/002_namespace_type.md) | NamespaceType newtype enforces namespace portion of this invariant |

### Sources

| File | Relationship |
|------|--------------|
| `src/data/validated_types.rs` | CommandName dot-prefix validation |
| `src/command_validation.rs` | Command name validation at registration |

### Tests

| File | Relationship |
|------|--------------|
| `tests/data/validated_command_name.rs` | CommandName construction and validation |
| `tests/data/validated_namespace.rs` | Namespace format validation |
