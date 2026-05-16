# Feature: Argument System

### Scope

- **Purpose:** Define behavioral requirements for argument parsing, binding, and validation
- **Responsibility:** FR-ARG-1 through FR-ARG-8: kind support, positional/named binding, defaults, validation
- **In Scope:** Argument kind requirements, binding rules, validation behavior, error handling
- **Out of Scope:** Parser internals, type system implementation details

### Design

The argument system defines a typed taxonomy of argument kinds (`String`, `Integer`, `Float`, `Boolean`, `Path`, `File`, `Directory`, `Enum`, `Url`, `DateTime`, `Pattern`, `List`, `Map`, `JsonString`, `Object`) and a binding model that maps raw parsed tokens to typed values.

Binding proceeds in two passes. The first pass handles positional arguments by assigning tokens to argument definitions in declaration order. The second pass handles named arguments (`name::value` syntax) by matching token names against both canonical argument names and declared aliases, regardless of their position in the input. This ordering ensures positional arguments are resolved before named ones can shadow them.

After binding, each argument value is type-checked against its declared `Kind`. If an optional argument is absent, its declared default value is injected at this stage. `ValidationRule` constraints (`Min`, `Max`, `MinLength`, `MaxLength`, `Pattern`, `MinItems`) are enforced immediately after type checking, so the `VerifiedCommand` produced by the semantic analyzer carries fully valid, typed values.

The interactive argument protocol is a special case: when a mandatory argument has `interactive: true` and is not supplied, the semantic analyzer returns a distinct error rather than treating the absence as a validation failure. This allows the calling modality (REPL or interactive CLI) to intercept the signal and prompt the user before retrying.

### FR-ARG-1 (Type Support)

The framework **must** support parsing and type-checking for the following `Kind`s: `String`, `Integer`, `Float`, `Boolean`, `Path`, `File`, `Directory`, `Enum`, `Url`, `DateTime`, `Pattern`, `List`, `Map`, `JsonString`, and `Object`.

**Implementation status:** ✅ All 15 `Kind` variants implemented. Type checking enforced in `SemanticAnalyzer` during argument binding.

### FR-ARG-2 (Positional Binding)

The framework **must** correctly bind positional arguments from a `GenericInstruction` to the corresponding `ArgumentDefinition`s in the order they are defined.

**Implementation status:** ✅ Positional binding implemented in the semantic analyzer's `bind_arguments()` function. Arguments bound in definition order when no name qualifier is provided.

### FR-ARG-3 (Named Binding)

The framework **must** correctly bind named arguments (`name::value`) from a `GenericInstruction` to the corresponding `ArgumentDefinition`, regardless of order.

**Implementation status:** ✅ Named `name::value` binding implemented. Arguments bound by name regardless of order. Comprehensive test coverage in the semantic analysis test suite.

### FR-ARG-4 (Alias Binding)

The framework **must** correctly bind named arguments specified via an alias to the correct `ArgumentDefinition`.

**Implementation status:** ✅ Alias binding implemented. Named arguments checked against both primary name and all aliases via `find_argument_by_name_or_alias()`.

### FR-ARG-5 (Default Values)

If an optional argument with a default value is not provided, the framework **must** use the default value during semantic analysis.

**Implementation status:** ✅ Default value injection implemented. When optional arguments are absent, `ArgumentDefinition::default_value` is used to populate the bound argument map.

### FR-ARG-6 (Validation Rule Enforcement)

The `Semantic Analyzer` **must** enforce all `ValidationRule`s (`Min`, `Max`, `MinLength`, `MaxLength`, `Pattern`, `MinItems`) defined for an argument. If a rule is violated, a `UNILANG_VALIDATION_RULE_FAILED` error **must** be returned.

**Implementation status:** ✅ ValidationRule enforcement implemented. All six constraint types validated. Returns `UNILANG_VALIDATION_RULE_FAILED` error on violation.

### FR-ARG-7 (Automatic Multiple Parameter Collection)

When the same parameter name appears multiple times in a command invocation (e.g., `command::"value1" command::"value2" command::"value3"`), the `Semantic Analyzer` **must** automatically collect all values into a `Value::List`, regardless of the argument definition's `multiple` attribute. This ensures that no parameter values are lost due to semantic processing limitations. Single parameters **must** remain as single values to maintain backward compatibility.

**Implementation status:** ✅ Implemented with comprehensive test coverage. Resolves the critical tokenization failure identified in Task 024.

### FR-ARG-8 (Unknown Parameter Detection)

The `Semantic Analyzer` **must** reject any command invocation that contains named parameters not defined in the `CommandDefinition` (including aliases). When unknown parameters are detected, a `UNILANG_UNKNOWN_PARAMETER` error **must** be returned with helpful error messages. For single unknown parameters with Levenshtein distance <= 2 from a valid parameter name, the error message **must** include a "Did you mean...?" suggestion. The error message **must** reference command-specific help (e.g., "Use '.command ??' for help"). This validation is **mandatory** and **cannot** be bypassed — there are no flags, settings, or configurations to disable unknown parameter detection.

**Implementation status:** ✅ Implemented with `check_unknown_named_arguments()`, `find_closest_parameter_name()`, and `levenshtein_distance()`. Comprehensive test coverage across core tests and edge case tests covering all boundary conditions, alias matching, distance thresholds, and complex scenarios.

### Analysis Instances

| File | Relationship |
|------|--------------|
| [001_api_analysis.md](../analysis/001_api_analysis.md) | Analysis of argument extraction boilerplate |
| [002_usability_improvements.md](../analysis/002_usability_improvements.md) | Usability improvements for argument API |

### API Instances

| File | Relationship |
|------|--------------|
| [001_public_types.md](../api/001_public_types.md) | ArgumentDefinition and Kind public types |

### Feature Instances

| File | Relationship |
|------|--------------|
| [001_command_registry.md](001_command_registry.md) | Commands that arguments belong to |
| [003_pipeline.md](003_pipeline.md) | Pipeline that processes bound arguments |
