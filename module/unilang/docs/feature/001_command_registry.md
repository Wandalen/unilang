# Feature: Command Registry

### Scope

- **Purpose:** Define behavioral requirements for command registration and lookup
- **Responsibility:** FR-REG-1 through FR-REG-9: registration, naming, validation, parity
- **In Scope:** Static/runtime registration requirements, naming rules, validation behavior, feature parity
- **Out of Scope:** Implementation details, data structure internals, performance benchmarks

Functional requirements governing command registration, naming, and registry behavior.

### FR-REG-1 (Static Registration)

The framework **must** provide a mechanism, via a `build.rs` script, to register commands at compile-time from a manifest file (e.g., `unilang.commands.yaml`).

**Implementation status:** ✅ Implemented via `build.rs` parsing `unilang.commands.yaml` and generating `StaticCommandRegistry` with PHF maps. Two YAML formats supported. Phase 4 complete (M4.2, M4.3). All tests pass.

### FR-REG-2 (Dynamic Registration)

The framework **must** expose a public API (`CommandRegistry::command_add_runtime`) for registering new commands and their routines at runtime.

- **Performance Guidance:** Runtime registration has 10-50x slower performance than compile-time registration (FR-REG-1). Production CLIs **should** prefer compile-time registration.
- **Appropriate Use Cases:** REPL applications, plugin systems, and prototyping workflows **should** use runtime registration for necessary flexibility.
- **Design Decision:** This API is not deprecated and will not be removed. The performance trade-off is intentional to support interactive and plugin-based use cases.

**Implementation status:** ✅ Implemented as `CommandRegistry::command_add_runtime()` in `src/registry.rs`. Supports dynamic registration with full validation including dot-prefix enforcement, duplicate detection, and parameter validation.

### FR-REG-3 (Declarative Loading)

The framework **must** provide functions (`load_from_yaml_str`, `load_from_json_str`) to load `CommandDefinition`s from structured text at runtime.

**Implementation status:** ✅ Implemented as `CommandRegistry::load_from_yaml_str()` and `load_from_json_str()` in `src/registry.rs`. Runtime loading from structured text into `CommandDefinition`s.

### FR-REG-4 (Namespace Support)

The framework **must** support hierarchical command organization through dot-separated namespaces (e.g., `.math.add`).

**Implementation status:** ✅ Dot-separated hierarchical namespaces supported throughout. Build-time `compute_full_name()` in `build.rs` handles namespace construction. Semantic analyzer resolves fully-qualified names at runtime.

### FR-REG-5 (Alias Resolution)

The framework **must** support command aliases. When an alias is invoked, the framework **must** execute the corresponding canonical command.

**Implementation status:** ✅ Aliases implemented in `CommandDefinition::aliases: Vec<String>`. Resolved in `src/semantic.rs` during named argument binding via alias lookup. `From<StaticCommandRegistry> for CommandRegistry` preserves aliases through conversion.

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

**Implementation status:** ✅ Implemented with runtime validation in `src/command_validation.rs` and build.rs transformations in `build.rs`. Two YAML formats documented and tested. All tests passing including test data files using both formats.

### FR-REG-7 (CLI Module Aggregation)

The framework **must** provide a `CliBuilder` API for aggregating multiple CLI modules into a unified command interface. The API **must** support:

- **Module Registration:** `static_module_with_prefix(name, prefix, commands)` to register command modules with namespace prefixes
- **Conflict Detection:** Automatic detection of duplicate command names or conflicting prefixes when enabled
- **Namespace Isolation:** Each module's commands are isolated within its prefix namespace (e.g., `.db.` prefix for database module)
- **Build Modes:** `build_static()` for compile-time registry (`StaticCommandMap` wrapper), `build_hybrid()` for mixed static/dynamic
- **Prefix Application:** Automatic prefix prepending to all commands in a module (e.g., prefix `.db` + command `.migrate` → `.db.migrate`)

This enables organizations to consolidate multiple CLI tools while maintaining clear separation of concerns and preventing naming conflicts.

**Implementation status:** ✅ Implemented in `src/multi_yaml/aggregator.rs` and `src/multi_yaml/cli_builder.rs`. Comprehensive test coverage in `tests/cli/cli_builder_api.rs` (25+ tests) covering module registration, prefix application, conflict detection, namespace isolation, and build modes. Examples: `examples/22_minimal_cli_aggregation.rs`.

### FR-REG-8 (Static Registry Feature Parity)

The `StaticCommandRegistry` **must** have complete feature parity with `CommandRegistry`. Specifically:

- **Validation:** `StaticCommandRegistry::register()` **must** validate command definitions using the same rules as `CommandRegistry::register()`
- **Auto-Help Generation:** Static commands **must** automatically generate `.command.help` counterparts, identical to `CommandRegistry::register_with_auto_help()`
- **Global Help Registration:** When registering static commands, the framework **must** also register the global `.help` command if not already present
- **Alias Resolution:** `StaticCommandRegistry::command()` **must** resolve aliases to canonical command names, not just exact matches
- **Complete Field Set:** `StaticCommandDefinition` **must** include all fields from `CommandDefinition`, including `short_desc`, `hidden_from_list`, `priority`, and `group` (with defaults for backward compatibility)
- **Conversion Bridge:** The framework **must** provide `From<StaticCommandRegistry> for CommandRegistry` conversion to enable `Pipeline::new(static_registry.into())` usage pattern
- **Purpose:** This parity ensures the "define once, use everywhere" vision applies equally to static and dynamic command definitions, allowing seamless migration between approaches

**Implementation status:** ✅ Implemented: `From<StaticCommandRegistry> for CommandRegistry` conversion bridge in `src/registry.rs`, validation in `CommandDefinition` builder, auto-help generated during conversion, aliases preserved through conversion. Tests: `tests/feature_parity_test.rs` (9 tests), `tests/static_registry_conversion_test.rs` (4 tests).

### FR-REG-9 (Build-Time Validation)

The `build.rs` script **must** validate all command definitions at compile time with actionable error messages:

- **Command Name Validation:** All command names **must** be validated using the same rules as runtime (dot prefix, valid characters)
- **Version Validation:** Version strings **must** be validated as non-empty
- **Duplicate Detection:** Duplicate command names **must** be detected and rejected with clear error showing both occurrences
- **Parameter Storage Validation:** Parameters with `multiple:true` **must** use List storage type to prevent silent data loss
- **Field Extraction:** All command fields including `validation_rules` **must** be extracted from YAML/JSON manifests
- **Error Handling:** Invalid manifests **must** cause `cargo build` to fail with clear error messages including file path and line number
- **No Silent Failures:** The script **must not** use `unwrap()` on user-provided data; all errors must be actionable
- **Shared Validation:** Validation logic **should** be shared between build.rs and runtime to avoid duplication (via `include!()` or module extraction)
- **Compile-Time Guarantee:** If build.rs validates successfully, the `From<StaticCommandDefinition> for CommandDefinition` conversion **must not** panic at runtime

**Implementation status:** ✅ Implemented: `build_validation` module in `build.rs` with `validate_command()`, `validate_version()`, `compute_full_name()`, duplicate detection via HashMap tracking, parameter storage validation, shared validation logic in `src/validation_core.rs` (6 functions), build-time validation with actionable error boxes.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [invariant/002_non_functional_requirements.md](../invariant/002_non_functional_requirements.md) | Performance NFRs for registry operations |
| doc | [invariant/003_governing_principles.md](../invariant/003_governing_principles.md) | Fail-fast validation principle |
| doc | [architecture/004_implementation_details.md](../architecture/004_implementation_details.md) | PHF static registry internals |
| doc | [api/001_public_types.md](../api/001_public_types.md) | Public types for registry and commands |
