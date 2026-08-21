# Invariant Spec: Governing Principles

### Scope

- **Purpose:** Verify that the seven governing principles defined in `docs/invariant/003_governing_principles.md` hold at runtime and at compile time
- **Responsibility:** Test cases exercising Minimum Implicit Magic, Single Source of Truth, Fail-Fast Validation, Make Illegal States Unrepresentable, Consistent Help Access, Explicit Dependencies, and Explicit Command Naming
- **In Scope:** Minimum Implicit Magic (no hidden registrations), Fail-Fast (first stage rejects bad input), Make Illegal States Unrepresentable (type-state builder, three-layer defense), Consistent Help Access (`??`/`.cmd.help` identity), Single Source of Truth (no duplicate definitions), Explicit Dependencies (required argument rejection), Explicit Command Naming (dot-prefix enforcement)
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

### IN-4: Consistent Help Access — `??` and `.cmd.help` render the identical page

- **Given:** A `Pipeline` with `.greet` registered (auto_help_enabled = true by default)
- **When:** Help is requested via `.greet ??` (semantic-level help token) and `.greet.help` (auto-registered sub-command)
- **Then:** Both routes succeed, both outputs contain the command name and argument descriptions, and the two pages are byte-identical — both routes render through the same `unilang_help`-backed path
- **Note:** There is no parser-level help operator; `?` is an ordinary value token, and a quoted `"??"` stays a literal value

### IN-5: Single Source of Truth — duplicate command registration is rejected

- **Given:** A `CommandRegistry` that already contains `.dup`
- **When:** `register_with_routine` is called a second time with a definition named `".dup"`
- **Then:** Returns an error with code `CommandAlreadyExists`; the registry retains the original definition unmodified

### IN-6: Explicit Dependencies — missing required argument is rejected with actionable error

- **Given:** A command `.needs_arg` with an `ArgumentDefinition` whose `attributes.optional` is `false` (a required argument, making the dependency explicit)
- **When:** The command is invoked without providing that argument
- **Then:** Semantic analysis returns an error with code `ErrorCode::ArgumentMissing`; the error message names the missing argument and instructs the caller to provide it

### IN-7: Explicit Command Naming — registration without dot prefix is rejected

- **Given:** A command name string `"build"` (no leading dot) passed to `CommandName::new`
- **When:** The name is validated at construction
- **Then:** Returns `Err`, not a silently auto-corrected `".build"`; the framework never adds an implicit dot prefix or otherwise transforms the name on the caller's behalf
