# API: Error Codes

### Scope

- **Purpose:** Specify the public error code taxonomy as a stable API contract for all integrators
- **Responsibility:** Canonical error code names, their string representations, semantic meaning, and stability guarantees
- **In Scope:** All public `ErrorCode` enum variants, their string forms, when each is produced, and how to detect them
- **Out of Scope:** Internal error wrapping mechanics, logging configuration, panic recovery strategies

### Abstract

The `unilang` framework communicates all recoverable errors through a typed `ErrorCode` enum. Every `ErrorData` value produced by the pipeline carries a variant from this enum, providing compile-time safety when matching errors in integrator code. The enum replaces the former string-based error codes and eliminates typo-prone string comparison at error-handling sites.

### Operations

Integrators interact with error codes in two ways:

**Matching on pipeline results:** When `Pipeline::process_command` returns an `Err`, the `ErrorData` value exposes its `ErrorCode` variant via `error_data.code`. Integrators match this variant to distinguish recoverable conditions (such as `ArgumentInteractiveRequired`) from hard failures (such as `CommandNotFound`).

**Detecting special conditions:** Two error codes represent non-failure conditions that require special handling. `HelpRequested` signals that the user invoked the `??` help token rather than executing a command — the pipeline converts this to a successful output before returning, so integrators using `Pipeline` normally never see it directly. `ArgumentInteractiveRequired` signals that a mandatory interactive argument was not supplied; REPL implementations intercept this to prompt the user before retrying execution.

**Error code reference:**

| `ErrorCode` Variant | String Representation | Produced when |
|---------------------|----------------------|---------------|
| `CommandNotFound` | `UNILANG_COMMAND_NOT_FOUND` | Input command path does not match any registered command or alias |
| `ArgumentMissing` | `UNILANG_ARGUMENT_MISSING` | A required argument (non-optional, no default) was not provided |
| `ArgumentTypeMismatch` | `UNILANG_ARGUMENT_TYPE_MISMATCH` | Argument value cannot be coerced to the declared `Kind` |
| `TooManyArguments` | `UNILANG_TOO_MANY_ARGUMENTS` | More positional arguments were supplied than the command declares |
| `UnknownParameter` | `UNILANG_UNKNOWN_PARAMETER` | A named parameter (`name::value`) is not defined in the command (includes typo suggestions when Levenshtein distance ≤ 2) |
| `ValidationRuleFailed` | `UNILANG_VALIDATION_RULE_FAILED` | An argument value violates a declared `ValidationRule` (Min, Max, MinLength, MaxLength, Pattern, MinItems) |
| `ArgumentInteractiveRequired` | `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` | A mandatory argument has `interactive: true` and was not supplied; the calling modality should prompt the user |
| `CommandAlreadyExists` | `UNILANG_COMMAND_ALREADY_EXISTS` | A duplicate command name was registered (including auto-generated `.command.help` entries) |
| `CommandNotImplemented` | `UNILANG_COMMAND_NOT_IMPLEMENTED` | A command is registered in the registry but has no bound routine |
| `TypeMismatch` | `UNILANG_TYPE_MISMATCH` | A type conversion or internal type mismatch error not covered by `ArgumentTypeMismatch` |
| `HelpRequested` | `HELP_REQUESTED` | The unquoted `??` help token triggered help display; the `Pipeline` converts this to a successful `OutputData` before returning to integrators |
| `InternalError` | `UNILANG_INTERNAL_ERROR` | An unexpected system error with no user-actionable recovery path |

### Error Handling

All errors propagate as `unilang::Error`, which wraps `ErrorData`. The `ErrorData` type exposes:
- `code: ErrorCode` — the typed variant for programmatic matching
- `message: String` — a human-readable description with context (argument name, command name, suggestions)

The recommended pattern for handling interactive arguments in a REPL loop is to match `ErrorCode::ArgumentInteractiveRequired` on error, prompt the user for the missing argument's value, then re-invoke the pipeline with the argument explicitly supplied.

The `HelpRequested` code is an internal pipeline signal. Integrators using `Pipeline::process_command` receive successful `OutputData` containing formatted help text when help is requested — no special error handling is required at the integrator level.

### Compatibility Guarantees

- Existing `ErrorCode` variants and their `UNILANG_*` string representations are stable after v1.0 and will not be renamed or removed
- New variants may be added in minor releases; integrators should use non-exhaustive matching (`_` arm) to remain forward-compatible
- The `InternalError` string representation (`UNILANG_INTERNAL_ERROR`) is stable; its message field is not guaranteed stable
- The `ErrorCode` enum implements `Display`, `Debug`, `Clone`, `PartialEq`, and `Eq`

### APIs

| File | Relationship |
|------|--------------|
| [001_public_types.md](001_public_types.md) | Defines the ErrorCode type referenced in this catalog |

### Features

| File | Relationship |
|------|--------------|
| [001_command_registry.md](../feature/001_command_registry.md) | FR-REG-* produce `CommandAlreadyExists`, `CommandNotFound` |
| [005_repl_interactive.md](../feature/005_repl_interactive.md) | FR-INTERACTIVE-1 produces `ArgumentInteractiveRequired` |

### Invariants

| File | Relationship |
|------|--------------|
| [002_non_functional_requirements.md](../invariant/002_non_functional_requirements.md) | NFR-ROBUST-1 mandates structured `ErrorData` for all user-facing errors |

### Sources

| File | Relationship |
|------|--------------|
| `src/error.rs` | ErrorCode enum and ErrorData definitions |

### Tests

| File | Relationship |
|------|--------------|
| `tests/api/error_codes.rs` | AP-1..14 error code taxonomy, string forms, derives, type-mismatch, catalog match |
| `tests/data/error_handling.rs` | Error code construction and matching |
