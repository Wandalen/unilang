# Feature: Command Registry

### Scope

- **Purpose:** Define behavioral requirements for command registration and lookup
- **Responsibility:** FR-REG-1 through FR-REG-9: registration, naming, validation, parity
- **In Scope:** Static/runtime registration requirements, naming rules, validation behavior, feature parity
- **Out of Scope:** Implementation details, data structure internals, performance benchmarks

### Design

The command registry uses a two-tier architecture. The first tier is a static map generated at compile time from YAML or JSON manifests and embedded in the binary as an optimized, zero-allocation lookup structure. The second tier is a dynamic map populated at runtime through the registration API. Lookups check the static tier first and fall back to the dynamic tier, making static commands both faster and invisible to dynamic-registration code.

All command identifiers use a dot-prefixed addressing scheme (e.g., `.system.echo`). Commands may be organized into namespaces via dot-separated segments. The registry enforces this naming contract at every registration boundary — runtime API, manifest parsing, and static registry generation — rejecting non-compliant names immediately with a clear error.

The registration lifecycle consists of: definition (building a `CommandDefinition` with metadata and argument declarations), binding (associating a `Routine` closure with the definition), and enrollment (submitting both to the registry with full validation). Auto-help enrollment generates a corresponding `.command.help` command for every registered command, making help consistently discoverable.

Alias resolution maps alternative names to the canonical command at lookup time, so routines always receive a `VerifiedCommand` with the canonical name regardless of which alias was invoked.

### FR-REG-1 (Static Registration)

The framework **must** provide a mechanism, via a `build.rs` script, to register commands at compile-time from a manifest file (e.g., `unilang.commands.yaml`).

**Implementation status:** ✅ Implemented via `build.rs` parsing `unilang.commands.yaml` and generating `StaticCommandRegistry` with PHF maps. Two YAML formats supported. Phase 4 complete (M4.2, M4.3). All tests pass.

### FR-REG-2 (Dynamic Registration)

The framework **must** expose a public API (`CommandRegistry::command_add_runtime`) for registering new commands and their routines at runtime.

- **Performance Guidance:** Runtime registration has 10-50x slower performance than compile-time registration (FR-REG-1). Production CLIs **should** prefer compile-time registration.
- **Appropriate Use Cases:** REPL applications, plugin systems, and prototyping workflows **should** use runtime registration for necessary flexibility.
- **Design Decision:** This API is not deprecated and will not be removed. The performance trade-off is intentional to support interactive and plugin-based use cases.

**Implementation status:** ✅ Implemented as `CommandRegistry::command_add_runtime()`. Supports dynamic registration with full validation including dot-prefix enforcement, duplicate detection, and parameter validation.

### FR-REG-3 (Declarative Loading)

The framework **must** provide functions (`load_from_yaml_str`, `load_from_json_str`) to load `CommandDefinition`s from structured text at runtime.

**Implementation status:** ✅ Implemented as `CommandRegistry::load_from_yaml_str()` and `load_from_json_str()`. Runtime loading from structured text into `CommandDefinition`s.

### FR-REG-4 (Namespace Support)

The framework **must** support hierarchical command organization through dot-separated namespaces (e.g., `.math.add`).

**Implementation status:** ✅ Dot-separated hierarchical namespaces supported throughout. Build-time `compute_full_name()` handles namespace construction. Semantic analyzer resolves fully-qualified names at runtime.

### FR-REG-5 (Alias Resolution)

The framework **must** support command aliases. When an alias is invoked, the framework **must** execute the corresponding canonical command.

**Implementation status:** ✅ Aliases implemented in `CommandDefinition::aliases: Vec<String>`. Resolved during named argument binding via alias lookup. `From<StaticCommandRegistry> for CommandRegistry` preserves aliases through conversion.

### FR-REG-6 (Explicit Command Names)

The framework **must** enforce explicit command naming with the following rules:

- All fully-qualified command names **must** start with a dot prefix (e.g., `.chat`, `.session.list`)
- Runtime API (`CommandRegistry::command_add_runtime`) **must** reject command registrations lacking a dot prefix with a clear error
- Runtime API **must not** automatically add, remove, or transform command names — commands are registered and executed exactly as specified
- Build-time YAML manifests **may** use two valid formats that both produce dot-prefixed command names:
  - **Format 1 (Compound Names — Recommended for Examples):** `name: ".session.list"`, `namespace: ""` → produces `.session.list`
  - **Format 2 (Separate Namespace — Valid for Production):** `name: "list"`, `namespace: ".session"` → produces `.session.list` (note: namespace field MUST include dot prefix)
- The `build.rs` script applies the following transformations to YAML manifests:
  - If `namespace` is empty and `name` starts with `.`: uses `name` as-is
  - If `namespace` is empty and `name` lacks `.`: adds dot prefix to produce `.{name}`
  - If `namespace` is not empty: concatenates to produce `{namespace}.{name}` (requires namespace to have dot prefix)
- Documentation and examples **should** use Format 1 to show users the exact command syntax they will type

**Implementation status:** ✅ Implemented with runtime validation and build.rs transformations. Two YAML formats documented and tested. All tests passing including test data files using both formats.

### FR-REG-7 (CLI Module Aggregation)

The framework **must** provide a `CliBuilder` API for aggregating multiple CLI modules into a unified command interface. The API **must** support:

- **Module Registration:** `static_module_with_prefix(name, prefix, commands)` to register command modules with namespace prefixes
- **Conflict Detection:** Automatic detection of duplicate command names or conflicting prefixes when enabled
- **Namespace Isolation:** Each module's commands are isolated within its prefix namespace (e.g., `.db.` prefix for database module)
- **Build Modes:** `build_static()` for compile-time registry (`StaticCommandMap` wrapper), `build_hybrid()` for mixed static/dynamic
- **Prefix Application:** Automatic prefix prepending to all commands in a module (e.g., prefix `.db` + command `.migrate` → `.db.migrate`)

This enables organizations to consolidate multiple CLI tools while maintaining clear separation of concerns and preventing naming conflicts.

**Implementation status:** ✅ Implemented via `CliBuilder` in the multi-YAML aggregation module. Comprehensive test coverage covering module registration, prefix application, conflict detection, namespace isolation, and build modes.

### FR-REG-8 (Static Registry Feature Parity)

The `StaticCommandRegistry` **must** have complete feature parity with `CommandRegistry`. Specifically:

- **Validation:** `StaticCommandRegistry::register()` **must** validate command definitions using the same rules as `CommandRegistry::register()`
- **Auto-Help Generation:** Static commands **must** automatically generate `.command.help` counterparts, identical to `CommandRegistry::register_with_auto_help()`
- **Global Help Registration:** When registering static commands, the framework **must** also register the global `.help` command if not already present
- **Alias Resolution:** `StaticCommandRegistry::command()` **must** resolve aliases to canonical command names, not just exact matches
- **Complete Field Set:** `StaticCommandDefinition` **must** include all fields from `CommandDefinition`, including `short_desc`, `hidden_from_list`, `priority`, and `group` (with defaults for backward compatibility)
- **Conversion Bridge:** The framework **must** provide `From<StaticCommandRegistry> for CommandRegistry` conversion to enable `Pipeline::new(static_registry.into())` usage pattern
- **Purpose:** This parity ensures the "define once, use everywhere" vision applies equally to static and dynamic command definitions, allowing seamless migration between approaches

**Implementation status:** ✅ Implemented: `From<StaticCommandRegistry> for CommandRegistry` conversion bridge, validation in `CommandDefinition` builder, auto-help generated during conversion, aliases preserved through conversion.

### FR-REG-9 (Build-Time Validation)

The `build.rs` script **must** validate all command definitions at compile time with actionable error messages:

- **Command Name Validation:** All command names **must** be validated using the same rules as runtime (dot prefix, valid characters)
- **Version Validation:** Version strings **must** be validated as non-empty
- **Duplicate Detection:** Duplicate command names **must** be detected and rejected with clear error showing both occurrences
- **Parameter Storage Validation:** Parameters with `multiple:true` **must** use List storage type to prevent silent data loss
- **Field Extraction:** All command fields including `validation_rules` **must** be extracted from YAML/JSON manifests
- **Error Handling:** Invalid manifests **must** cause `cargo build` to fail with clear error messages including file path and line number
- **No Silent Failures:** The script **must not** use `unwrap()` on user-provided data; all errors must be actionable
- **Shared Validation:** Validation logic **should** be shared between build.rs and runtime to avoid duplication
- **Compile-Time Guarantee:** If build.rs validates successfully, the `From<StaticCommandDefinition> for CommandDefinition` conversion **must not** panic at runtime

**Implementation status:** ✅ Implemented with `validate_command()`, `validate_version()`, `compute_full_name()`, duplicate detection, parameter storage validation, shared validation logic, and build-time validation with actionable error messages.

### Analysis Instances

| File | Relationship |
|------|--------------|
| [001_api_analysis.md](../analysis/001_api_analysis.md) | Analysis of this registry's boilerplate patterns |
| [002_usability_improvements.md](../analysis/002_usability_improvements.md) | Usability recommendations for this registry API |

### Architecture Instances

| File | Relationship |
|------|--------------|
| [003_vision_scope.md](../architecture/003_vision_scope.md) | Vision that drives registration requirements |
| [004_implementation_details.md](../architecture/004_implementation_details.md) | PHF static registry internals |

### API Instances

| File | Relationship |
|------|--------------|
| [001_public_types.md](../api/001_public_types.md) | Public types for registry and commands |
| [002_error_codes.md](../api/002_error_codes.md) | Error codes from registry operations |

### Feature Instances

| File | Relationship |
|------|--------------|
| [002_argument_system.md](002_argument_system.md) | Arguments belonging to registered commands |
| [003_pipeline.md](003_pipeline.md) | Pipeline that queries this registry |
| [004_help_system.md](004_help_system.md) | Help system referencing registered commands |

### Invariant Instances

| File | Relationship |
|------|--------------|
| [002_non_functional_requirements.md](../invariant/002_non_functional_requirements.md) | Performance NFRs for registry operations |
| [003_governing_principles.md](../invariant/003_governing_principles.md) | Fail-fast validation principle |
| [005_command_naming.md](../invariant/005_command_naming.md) | Dot-prefix naming invariant for all registered commands |
