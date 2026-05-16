# Analysis: Usability Improvements

### Scope

- **Purpose:** Document prioritized recommendations for improving API ergonomics and misuse prevention
- **Responsibility:** Actionable summary of usability findings from comprehensive example analysis
- **In Scope:** Boilerplate patterns, type safety gaps, builder ergonomics, error handling improvements
- **Out of Scope:** Formal feature requirements (see feature/ instances), implementation details

### Executive Summary

Based on comprehensive analysis of 40+ examples and 3,000+ lines of framework code, here are prioritized recommendations to make Unilang easier to use correctly and nearly impossible to misuse.

### Critical Issues (Fix First)

#### 1. Boilerplate Explosion (90% of code affected)

**Problem:** Every command routine repeats the same 4-line argument extraction pattern — getting an argument by name, conditionally matching the `Value` enum variant, and providing a default via `unwrap_or`. This pattern appears in 15+ examples, teaches silent type mismatch failures, and causes developer frustration.

**Solution:** Add typed extraction helpers to `VerifiedCommand` — `get_string()`, `require_string()`, `get_bool()`, `get_integer()`, `get_float()`, etc. These eliminate the boilerplate, prevent silent type mismatches, and standardize extraction across all routines. **Implemented.**

**Benefit:** Eliminates 90% of boilerplate, prevents silent type mismatches.

#### 2. Silent Type Mismatches (CRITICAL severity)

**Problem:** When the argument extraction pattern uses `unwrap_or` as a fallback, a wrong `Value` variant (e.g., the parser returning `Value::String("5")` when `Value::Integer` was expected) silently uses the default instead of failing. The type system does not enforce correctness, and bugs go undetected.

**Solution (Option A — Macro):** A `command! { ... }` macro that declares argument types and generates a typed argument struct, eliminating `Value` matching entirely at compile time.

**Solution (Option B — Type-State Builder):** A builder API with generic type parameters for argument types, exposing type-checked accessors to the routine.

**Benefit:** Catch type errors at compile time, not runtime.

#### 3. Builder Error Swallowing (HIGH severity)

**Problem:** Registration errors in `CommandRegistry::builder()` are only logged with `eprintln!`, never returned to the caller. An invalid command name is silently ignored. Callers receive a registry that may be missing commands with no indication of failure.

**Solution:** Use `build_checked()` which returns a `Result` and propagates registration errors. Alternatively, change `command_with_routine()` itself to return `Result<Self, CommandValidationError>` for fail-fast behavior. **`build_checked()` implemented.**

**Benefit:** Errors caught immediately, not discovered later.

### High Priority Improvements

#### 4. String-Based Error Codes

**Problem:** Error detection via string constant comparison is fragile — typos are not caught at compile time, pattern matching requires string equality, and IDEs cannot enumerate valid codes.

**Solution:** A typed `ErrorCode` enum where each variant corresponds to one error condition. Comparisons use enum equality instead of string equality. **Implemented.**

**Benefit:** Compile-time safety, exhaustive matching, better IDE support.

#### 5. Missing Argument Name Validation

**Problem:** Typos in argument names accessed in routines (e.g., accessing `"usrname"` when the definition declares `"username"`) compile successfully but always return `None` at runtime.

**Solution:** A proc macro that declares argument names in the command definition and generates a typed accessor struct, making typos compile errors.

**Benefit:** Typos caught at compile time.

#### 6. OutputData Construction Boilerplate

**Problem:** Every routine constructs `OutputData` manually with repetitive field initialization for content and format.

**Solution:** Convenience constructors — `OutputData::text(content)` for plain text output and `OutputData::json(value)` for serialized JSON — reduce construction to a single call.

### Medium Priority Improvements

#### 7. Builder String Conversion Spam

**Problem:** Builder methods require `.to_string()` for every string literal argument, producing repetitive noise throughout command definition code.

**Solution:** Accept `impl Into<String>` in all builder methods so string literals can be passed directly without explicit conversion.

#### 8. Namespace vs Name Confusion

**Problem:** The distinction between the `name` field (e.g., `".greet"`) and `full_name()` (e.g., `".namespace.greet"`) is easy to confuse, especially when both of the two supported YAML formats produce the same final result.

**Solution:** Clearer type separation with a `CommandPath` struct that makes namespace and local name explicit, generating the full name deterministically from typed components.

#### 9. Example Code Uses `unwrap()`

**Problem:** Examples teach bad patterns — using `.unwrap()` on command lookups, argument access, and registration results without handling the `None` or `Err` cases.

**Solution:** Update all examples to use proper error handling — `if let Some(cmd) = ...` for optional access, `?` propagation for fallible operations, no bare `unwrap()` calls.

### Low Priority (But Still Valuable)

#### 10. CommandDefinition Default Pollution

**Problem:** Every command specifies identical default values for status, version, deprecation message, http_method_hint, and idempotent flag, duplicating the same boilerplate across all command definitions.

**Solution:** A builder that pre-populates sensible defaults (stable, 1.0.0, GET, true) so only overrides need to be specified.

#### 11. Interactive Argument Pattern Hidden

**Problem:** The interactive argument required signal exists (via error code) but there are no public API helpers to detect it, extract the argument name that requires input, or communicate the REPL retry protocol clearly.

**Solution:** Formalize the pattern in public API with helper methods — `requires_interactive_input()`, `interactive_argument()`, `is_help_response()`.

#### 12. Argument Validation Helper Missing

**Problem:** Constructing validation rules requires verbose separate entries for each constraint in a `vec![]`, with no fluent composition.

**Solution:** A fluent validation API on `ArgumentDefinition::builder()` with chained `.min_length()`, `.max_length()`, `.pattern()`, and `.done()` calls.

### Root Cause Analysis

The main issues stem from:

1. **No type-safe argument extraction** — forces manual `Value` enum matching
2. **Builder lacks defaults** — requires specifying everything
3. **String-based validation** — no compile-time checking
4. **Examples as truth** — users copy bad patterns

### Success Metrics

After implementing these improvements:

- 90% reduction in boilerplate code
- Zero silent type mismatches
- All errors caught at compile time or explicit at runtime
- Examples show best practices only
- API guides users toward correct usage
- Misuse becomes difficult or impossible

### Analysis Instances

| File | Relationship |
|------|--------------|
| [001_api_analysis.md](001_api_analysis.md) | Detailed API analysis backing these recommendations |

### Feature Instances

| File | Relationship |
|------|--------------|
| [001_command_registry.md](../feature/001_command_registry.md) | Registry requirements relevant to builder issues |
| [002_argument_system.md](../feature/002_argument_system.md) | Argument system requirements relevant to type safety |
| [005_repl_interactive.md](../feature/005_repl_interactive.md) | Interactive argument handling requirements |
