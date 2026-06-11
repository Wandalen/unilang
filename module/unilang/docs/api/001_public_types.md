# API: Public Types

### Scope

- **Purpose:** Document the public data structures, environment variables, and stable operations exposed to integrators
- **Responsibility:** Public types, operations, error handling contract, environment variables, compatibility guarantees
- **In Scope:** Public structs, enums, environment variables, API operations, compatibility guarantees
- **Out of Scope:** Internal implementation details, vision/scope, migration history, architectural decisions

### Abstract

The `unilang` public API surface provides integrators with `CommandDefinition`, `ArgumentDefinition`, `CommandRegistry`, `Pipeline`, `Value`, `Kind`, `StaticCommandDefinition`, and related types for building multi-modal command-line utilities. The API is organized into type definitions, registration operations, execution operations, and argument extraction operations.

### Public Type Surface

The public API exposes the following data structures to integrators:

- **CommandDefinition**: Command metadata — name, description, arguments, aliases, status, examples, and auto-help flag.
- **ArgumentDefinition**: Argument metadata — name, kind, validation rules, optional/required attributes.
- **ArgumentAttributes**: Behavioral flags for arguments — optional, multiple, sensitive, interactive.
- **Kind**: Data type enum for arguments — String, Integer, Float, Boolean, Path, File, Directory, Enum, Url, DateTime, Pattern, List, Map, JsonString, Object.
- **ValidationRule**: Constraint for argument validation — Min, Max, MinLength, MaxLength, Pattern, MinItems.
- **OutputData**: Standardized result for successful command execution — content, format, and optional execution timing (auto-populated by the Interpreter).
- **ErrorData**: Standardized error structure — typed `ErrorCode` enum and human-readable message.
- **Value**: Runtime argument value union type matching the `Kind` enum variants.
- **StaticCommandMap**: Opaque wrapper for compile-time optimized command maps; hides PHF implementation from downstream crates.
- **StaticCommandDefinition**: Const-compatible command definition for static storage.
- **VerifiedCommand**: Validated command output from the semantic analyzer — contains bound, typed argument values.
- **Pipeline**: High-level orchestration object for the Parse → SemanticAnalysis → Interpret flow.
- **CommandRegistry**: Runtime registry for command definitions and their associated routines.

### Operations

**Command Construction**: `CommandDefinition::former()` provides a type-state builder requiring at minimum a name and description. The `end()` method provides sensible defaults; `build()` requires all fields to be set explicitly. `CommandName`, `NamespaceType`, `VersionType`, and `CommandStatus` newtypes enforce valid states at construction time.

**Command Registration**: `CommandRegistry` accepts commands via `command_add_runtime()` (single command with full validation) or `register_with_auto_help()` (with automatic `.command.help` companion). `CommandRegistryBuilder` provides a fluent builder API where `build_checked()` propagates registration errors explicitly (preferred over `build()` which silently ignores errors for backward compatibility).

**Pipeline Execution**: `Pipeline::process_command()` orchestrates the full Parse → SemanticAnalysis → Interpret flow. `process_batch()` executes multiple commands independently, collecting all results regardless of failures. `process_sequence()` stops on the first failure. `process_command_from_argv()` and `process_command_from_argv_simple()` accept OS argument arrays directly, preserving argument boundaries without re-quoting.

**Argument Extraction**: `VerifiedCommand` provides typed extraction methods — `get_string()`, `require_string()`, `get_string_normalized()`, `require_string_normalized()`, `get_integer()`, `require_integer()`, `get_float()`, `require_float()`, `get_boolean()`, `require_boolean()`, `get_path()`, `require_path()`, `get_list()`, `require_list()`, `has_argument()`, and `get_value()` — eliminating manual `Value` enum matching in command routines.

**Static Registry**: `StaticCommandMap` provides O(1) lookups via `get()`, `contains_key()`, `keys()`, `entries()`, `values()`, `len()`, and `is_empty()`. Created by the build script and integrated via `StaticCommandRegistry::from_commands()`. Supports indexing syntax with `Index<&str>` (panics if key not found).

**Configuration Utilities** (requires `json_parser` feature): Typed extraction functions support configuration parsing from `ConfigMap<S>` — a `HashMap<String, (JsonValue, S)>` typed alias. Available for u8, u16, u32, u64, i32, i64, f64, bool, String, and string arrays.

**Help Generation**: `HelpGenerator::with_verbosity()` creates a generator at a specific verbosity level. `set_verbosity()` updates the level dynamically. `verbosity()` queries the current level. Levels 0–4 range from Minimal to Comprehensive; default is Level 2 (Standard).

### Error Handling

All API errors are returned as `unilang::Error` wrapping an `ErrorData` struct with a typed `ErrorCode` enum and a human-readable message. See [002_error_codes.md](002_error_codes.md) for the complete error code reference and stability guarantees.

### Environment Variables

| Variable | Purpose | Example |
| :--- | :--- | :--- |
| `UNILANG_VERBOSITY` | Sets logging verbosity for CLI binaries (0=quiet, 1=normal, 2=debug) | `2` |
| `UNILANG_HELP_VERBOSITY` | Controls help output detail level (0=Minimal, 1=Basic, 2=Standard/DEFAULT, 3=Detailed, 4=Comprehensive) | `2` |
| `UNILANG_HELP_HIDE_VERSION` | When set, suppresses the version line in command help output | `1` |

### Compatibility Guarantees

- Semver: Breaking changes increment major version
- `StaticCommandDefinition` fields are additive — new fields have defaults (no breaking change)
- Environment variable names are stable after v1.0

### Analyses

| File | Relationship |
|------|--------------|
| [001_api_analysis.md](../analysis/001_api_analysis.md) | Analysis of public type usage patterns and boilerplate |

### Features

| File | Relationship |
|------|--------------|
| [001_command_registry.md](../feature/001_command_registry.md) | CommandRegistry and CommandDefinition types |
| [002_argument_system.md](../feature/002_argument_system.md) | ArgumentDefinition and Kind types |
| [003_pipeline.md](../feature/003_pipeline.md) | Pipeline operations |
| [004_help_system.md](../feature/004_help_system.md) | HelpGenerator and help verbosity |

### Invariants

| File | Relationship |
|------|--------------|
| [002_non_functional_requirements.md](../invariant/002_non_functional_requirements.md) | NFRs that govern API performance |

### Architectures

| File | Relationship |
|------|--------------|
| [004_implementation_details.md](../architecture/004_implementation_details.md) | Internal implementation of StaticCommandMap |

### APIs

| File | Relationship |
|------|--------------|
| [002_error_codes.md](002_error_codes.md) | Error codes stable API contract |

### Types

| File | Relationship |
|------|--------------|
| [001_command_name.md](../type/001_command_name.md) | CommandName validated newtype for command identifiers |
| [002_namespace_type.md](../type/002_namespace_type.md) | NamespaceType validated newtype for namespace identifiers |
| [003_version_type.md](../type/003_version_type.md) | VersionType validated newtype for version strings |
| [004_command_status.md](../type/004_command_status.md) | CommandStatus lifecycle enum for command metadata |

### Sources

| File | Relationship |
|------|--------------|
| `src/data/` | Core data types: CommandDefinition, argument types, validated types |
| `src/registry/` | Registry types: CommandRegistry, StaticCommandMap |
| `src/pipeline/` | Pipeline types: PipelineResult |

### Tests

| File | Relationship |
|------|--------------|
| `tests/data/` | Data model and type validation tests |
| `tests/api/` | Public API contract tests |
