# Analysis: API Analysis

### Scope

- **Purpose:** Document API surface analysis findings, patterns, and improvement opportunities
- **Responsibility:** Analysis of public API ergonomics, boilerplate patterns, and type safety gaps
- **In Scope:** Argument extraction patterns, builder patterns, error handling, type safety issues
- **Out of Scope:** Implementation of fixes (see feature/ instances for requirements)

### Executive Summary

The Unilang framework is a command-line utility language framework designed to provide a unified way to define commands once and use them everywhere (CLI, REPL, TUI, Web APIs). The codebase exhibits several well-designed patterns but also contains API design opportunities and error-prone patterns that could benefit from stronger compile-time guarantees.

**Key Findings:**
- Boilerplate-heavy argument extraction patterns throughout examples
- Type-safe builder patterns implemented with good ergonomics
- Several `unwrap()` calls in production examples that could be silent failures
- Missing standardized helper methods for common argument access patterns
- String-based error codes that could benefit from typed error enums
- Inconsistent error handling between different example patterns

### Part 1: Common Boilerplate Code Patterns

#### Pattern 1: Repetitive Argument Extraction (Most Common)

The pattern appearing in approximately 90% of command routines involves three steps: getting an argument by name from the arguments map, using conditional pattern matching to extract the inner typed value from the `Value` enum variant, and providing a default via `unwrap_or`. The same 4-line construction repeats for every argument type (String, Boolean, Float), with minor variations — some examples use `and_then()`, some use `map_or_else()`, some use `unwrap()` directly.

Issues: (1) repetitive boilerplate in virtually every command routine; (2) type safety disconnect — developers manually assert the expected `Value` variant; (3) silent failures — type mismatches fall back to defaults without error; (4) inconsistent patterns across examples.

Affected locations include examples for basic command registration, command execution, DSL inline closures, and REPL comparison.

#### Pattern 2: Builder Configuration Boilerplate

`CommandDefinition` registration requires specifying many fields even when most should share sensible defaults — namespace, hint, status, version, aliases, tags, permissions, idempotent flag, deprecation message, http_method_hint, examples, and arguments. Issues: (1) many required fields; (2) constant `.to_string()` conversions for static strings; (3) verbose empty collections (`vec![]`, `String::new()`); (4) no shared defaults across commands.

#### Pattern 3: Argument Definition Template Repetition

Defining each `ArgumentDefinition` via struct literal repeats the same fields: name, description, kind, hint, attributes (typically using spread `..Default::default()`), validation rules, aliases, and tags — all requiring `.to_string()` conversions. The spread pattern is used almost universally because most optional fields share the same defaults.

### Part 2: Public API Surface Analysis

The `src/lib.rs` prelude exposes: `CommandDefinition`, `ArgumentDefinition`, `ArgumentAttributes`, `Kind`, `OutputData`, `ErrorData`, `CommandRegistry`, `CommandRegistryBuilder`, `StaticCommandRegistry`, `RegistryMode`, and `PerformanceMetrics`.

**Core Flow:**
1. `CommandRegistry` — main API for runtime command registration
2. `CommandDefinition` — command metadata
3. `ArgumentDefinition` — argument metadata
4. `Value` enum — runtime argument values
5. `VerifiedCommand` — commands after semantic analysis
6. `Pipeline` — high-level orchestration API

The most common usage patterns are: creating a registry then calling `register_with_routine()` per command, or using the fluent builder API with `CommandRegistry::builder()`.

### Part 3: Error-Prone API Patterns

#### Issue 1: `unwrap()` in Example Code

Multiple examples call `.unwrap()` on results — on error references, timestamps, command lookups, argument access, and registration results. Examples using `unwrap()` teach users bad error handling patterns. Users copy-paste these patterns into production code, producing panics on missing arguments instead of graceful error handling.

#### Issue 2: Type Confusion in Argument Handling

When argument extraction uses a conditional match on the `Value` variant and falls back to a default with `unwrap_or`, a type mismatch (e.g., the parser returning `Value::Enum("Alice")` when `Value::String("Alice")` was expected) causes the default to be used silently. There is no type validation between semantic analysis and routine execution.

#### Issue 3: String-Based Error Codes

Error codes throughout the framework are string constants. String comparisons for error detection in the pipeline, typos in error codes not caught at compile time, and no type-safe way to pattern match on specific errors are all consequences. Documentation lists codes but the implementation uses untyped `ErrorData`.

#### Issue 4: Missing Compile-Time Argument Validation

A command's routine can reference argument names that were never defined in the `CommandDefinition`. The mismatch compiles successfully but produces wrong behavior at runtime — the argument access silently returns `None` instead of failing at compile time.

### Part 4: Builder Pattern Usage Analysis

#### CommandRegistry Builder Pattern

Strengths: fluent API for inline command registration, type-safe via `CommandRegistryBuilder`, supports mixing YAML loading and inline closures. Weakness: the `build()` method silently swallows registration errors — failures are only logged with `eprintln!`, not returned to the builder user. The `build_checked()` alternative is available but underused.

#### CommandDefinition Builder Pattern (Type-State)

Strengths: compile-time enforcement of required fields, clear type-state transitions, impossible to build incomplete definitions. Weaknesses: users must remember all 6 required fields, many optional fields requiring repeated initialization.

#### DynamicCommandMap and Registry Mode

The `RegistryMode` enum adds complexity without clear use cases in examples. Performance metrics are tracked but examples never use them. Cache management is exposed but rarely needed.

### Part 5: Type Safety Issues and Missing Compile-Time Checks

#### Issue 1: Value Enum Pattern Matching

In every routine, there is no way to enforce at compile time that a specific argument name holds a `String` value. Pattern matching on `Value` variants is boilerplate that every developer must write correctly.

#### Issue 2: Namespace vs. Name Confusion

FR-REG-6 documents two valid formats: compound name (e.g., `name: ".session.list"`, `namespace: ""`) and separate namespace (e.g., `name: "list"`, `namespace: ".session"`). Both are valid but create different semantics and the distinction is easy to confuse.

#### Issue 3: Missing Argument Access Helper Methods

The `Value` enum has no helper methods for safe typed extraction. Developers must write the same conditional match pattern for every argument access.

#### Issue 4: Interactive Argument Pattern

The `ArgumentAttributes::interactive: true` flag requires special REPL-level handling. There is no type-safe way to communicate this requirement. The required-interactive error code is detected via string matching.

### Part 6: Opportunities for Better API Design

#### Opportunity 1: Argument Extraction Helpers

Adding typed extraction helpers to `VerifiedCommand` would eliminate 90% of boilerplate in routines. Methods like `get_string()`, `require_string()`, `get_integer()`, etc. would prevent type mismatch silent failures and standardize extraction across all routines. This improvement has been implemented — see api/001.

#### Opportunity 2: Typed Error Codes

Replacing string-based error codes with a typed `ErrorCode` enum enables compile-time checking, eliminates typos, and allows type-safe pattern matching. This improvement has been implemented — see api/002.

#### Opportunity 3: Builder Error Propagation

The `build_checked()` alternative to `build()` provides proper error propagation from `CommandRegistryBuilder`. Using `build_checked()` catches registration failures instead of swallowing them.

#### Opportunity 4: Command Definition Defaults

A builder method that provides sensible defaults for status, version, deprecation message, and http_method_hint would reduce repetition across all command definitions.

#### Opportunity 5: Compile-Time Argument Validation

A proc macro approach (e.g., `#[command(...)]`) could validate that argument names in the routine match those declared in the `CommandDefinition`, moving this class of bug from runtime to compile time. This is the responsibility of `unilang_meta`.

#### Opportunity 6: Structured Argument Validation

An enum-based approach to validation constraints with typed parameters would enable programmatic constraint extraction and generate better error messages than the current `ValidationRule` approach.

### Part 7: Missing API Patterns Found in Examples

#### Pattern 1: Interactive Argument Handling

The interactive argument signal is used in examples but not formally part of the public API. There are no helper methods to check whether a result requires interactive input, what argument name is needed, or how to communicate the REPL retry protocol.

#### Pattern 2: Help Request Detection

The semantic analyzer's help detection signal is visible only via a specific error code string. There is no public API to check if help was requested before semantic analysis completes or to construct help responses directly.

#### Pattern 3: Static Command Management

The include-at-compile-time pattern for consuming build.rs output is used internally but has no clear public documentation showing integrators how to use static commands, integrate `build.rs`, or compare static vs. dynamic performance.

### Conclusions

**Why boilerplate is heavy:** No type-safe extraction helpers force users to write manual `Value` enum matching repeatedly. Verbose builder initialization requires specifying all fields. String literals require `.to_string()` conversions. No shared defaults force each command to duplicate common metadata.

**Why error handling is fragile:** String-based error codes have no compile-time checking. Silent failures in builders swallow errors with only `eprintln!`. Type mismatches hidden by `unwrap_or` go undetected. Different examples use incompatible error extraction patterns.

**Why type safety is weak:** No argument name validation allows routines to reference wrong names. No argument type validation allows extracting wrong `Value` variants silently. Interactive argument handling is detected via error codes rather than types.

**Feature alignment status:** FR-ARG-6 (Validation Rule Enforcement) implemented but error messages are weak. FR-REG-6 (Explicit Command Names) enforced with clear error handling. FR-ARG-8 (Unknown Parameter Detection) implemented but only via error string matching. Interactive argument handling implemented but documentation is sparse and entirely error-based.

**Priority fixes implemented:** Typed argument extraction helpers on `VerifiedCommand` eliminate 90% of boilerplate. `build_checked()` on `CommandRegistryBuilder` prevents silent failures. `ErrorCode` enum enables safer error matching.

**Outstanding opportunities:** Compile-time argument validation via proc macro. `impl Into<String>` acceptance in builders. Formalized interactive argument protocol in public API. Clear static command integration documentation.

### Analyses

| File | Relationship |
|------|--------------|
| [002_usability_improvements.md](002_usability_improvements.md) | Prioritized recommendations based on these findings |

### Features

| File | Relationship |
|------|--------------|
| [001_command_registry.md](../feature/001_command_registry.md) | FR-REG-6 requirement analyzed in Part 5 |
| [002_argument_system.md](../feature/002_argument_system.md) | FR-ARG-* requirements analyzed in Parts 3, 5 |
| [004_help_system.md](../feature/004_help_system.md) | Help request detection patterns in Part 7 |

### APIs

| File | Relationship |
|------|--------------|
| [001_public_types.md](../api/001_public_types.md) | Public Value types and structures being analyzed |

### Architectures

| File | Relationship |
|------|--------------|
| [004_implementation_details.md](../architecture/004_implementation_details.md) | Static registry implementation referenced in Part 7 |

### Sources

| File | Relationship |
|------|--------------|
| `src/data/` | Primary analysis subject: data types |
| `src/registry/` | Primary analysis subject: registry API |
