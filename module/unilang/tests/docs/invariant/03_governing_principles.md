# Invariant Spec: Governing Principles

### Scope

- **Purpose:** Verify that the five governing principles defined in `docs/invariant/003_governing_principles.md` hold at runtime and at compile time
- **Responsibility:** Test cases exercising Minimum Implicit Magic, Single Source of Truth, Fail-Fast Validation, Make Illegal States Unrepresentable, and Consistent Help Access
- **In Scope:** Minimum Implicit Magic (no hidden registrations), Fail-Fast (first stage rejects bad input), Make Illegal States Unrepresentable (type-state builder, three-layer defense), Consistent Help Access (`?`/`.cmd.help` equivalence), Single Source of Truth (no duplicate definitions)
- **Out of Scope:** NFR thresholds (invariant 002); specific FR behaviors (feature specs)

### IN-1: Fail-Fast — malformed command string rejected at Parse stage, not Interpret stage

- **Given:** A `Pipeline` and input `"@invalid!command"` (unparseable token)
- **When:** `pipeline.run("@invalid!command")` is called
- **Then:** Returns a `ParseError` variant (not a `SemanticError` or `InterpreterError`); the error is produced by the Parser without reaching the Semantic Analyzer

### IN-2: Illegal states unrepresentable — incomplete CommandDefinition does not compile

- **Given:** A call to `CommandDefinition::former()` that sets all fields except the mandatory `name` field before calling `.end()`
- **When:** The Rust code is compiled
- **Then:** Compilation fails with a type error originating from the type-state builder (missing required field state); the error occurs at compile time, not at runtime

### IN-3: Minimum Implicit Magic — no command is registered without explicit registration call

- **Given:** A fresh `CommandRegistry` or `StaticCommandMap` with no user-provided command definitions
- **When:** `registry.get(".help")` or any implicit system command is called
- **Then:** Returns `None` unless the user explicitly registered `.help`; no hidden system commands exist in the default registry

### IN-4: Consistent Help Access — `?` and `.cmd.help` produce equivalent content

- **Given:** A `Pipeline` with `.greet` registered (auto_help_enabled = true by default)
- **When:** Help is requested via `.greet ?` (parser-level operator) and `.greet.help` (auto-registered sub-command)
- **Then:** Both outputs contain the same command name and argument descriptions; formatting may differ but no information is exclusive to one route
- **Note:** `??` as a bare token is rejected by the parser ("Help operator '?' must be the last token"); the two verified working routes are `?` and `.cmd.help`

### IN-5: Single Source of Truth — duplicate command registration is rejected

- **Given:** A `CommandRegistry` that already contains `.dup`
- **When:** `command_add_runtime` is called a second time with a definition named `".dup"`
- **Then:** Returns an error with code `CommandAlreadyExists`; the registry retains the original definition unmodified
