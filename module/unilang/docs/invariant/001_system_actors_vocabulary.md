# Invariant: System Actors Vocabulary

### Scope

- **Purpose:** Define the canonical actor taxonomy and ubiquitous language that must remain stable across all codebase evolution
- **Responsibility:** Actor definitions, term definitions, vocabulary contracts
- **In Scope:** Human actors, software actors, ubiquitous language terms and their invariant meanings
- **Out of Scope:** Feature requirements, implementation patterns, design decisions

### System Actors

An Actor is any entity that plays a distinct role and participates in an interaction within the system's architecture.

#### Human Actors

- **`Integrator (Developer)`**: The primary human actor who uses the `unilang` framework crates (`unilang`, `unilang_parser`, `unilang_meta`) to build a `utility1` application. Their responsibilities include defining commands, implementing routines, and configuring the framework.
- **`End User`**: A human actor who interacts with the compiled `utility1` application through one of its exposed `Modalities` (e.g., by typing commands into a CLI).

#### External System Actors

- **`Operating System`**: A system actor that provides the execution environment for `utility1`, including the CLI shell, file system, and environment variables.
- **`External Service`**: Any external system (e.g., a database, a web API) that a command `Routine` might interact with. The `unilang` framework does not interact with these services directly, but it facilitates the execution of routines that do.

#### Internal System Actors

- **`Build Script (build.rs)`**: A critical internal actor responsible for compile-time operations. Its primary role is to process static command definitions (from code or manifests) and generate optimized static command maps (using Perfect Hash Functions internally) wrapped in `StaticCommandMap`, enabling the zero-overhead static command registry while hiding implementation details from downstream crates.
- **`Command Registry`**: An internal actor that serves as the runtime database for all command definitions. It manages both the static (`StaticCommandMap` wrapper) and dynamic (HashMap) command sets and provides the lookup service used by the `Semantic Analyzer`.
- **`Parser (unilang_parser)`**: An internal actor that performs lexical and syntactic analysis on a raw input string, converting it into a structured `GenericInstruction` without any knowledge of command definitions.
- **`Semantic Analyzer`**: An internal actor that validates a `GenericInstruction` against the `Command Registry` to produce a `VerifiedCommand` that is guaranteed to be executable.
- **`Interpreter`**: An internal actor that takes a `VerifiedCommand` and invokes its corresponding `Routine`, managing the execution context and handling results.

### Ubiquitous Language

One canonical term per concept. Use these terms consistently everywhere — directory names, headings, body text, cross-references. No synonyms, no paraphrasing.

- **`unilang`**: This specification and the core framework crate.
- **`utility1`**: A generic placeholder for the primary application that implements `unilang`.
- **`Command Registry`**: The runtime data structure that holds all known `CommandDefinition`s and their associated `Routine`s. It supports both static (compile-time) and dynamic (run-time) registration.
- **`CommandDefinition`**: The canonical metadata for a command, defining its name, arguments, aliases, and behavior. Uses private fields with validated newtypes (`CommandName`, `NamespaceType`, `VersionType`, `CommandStatus`) and type-state builder pattern for type-safe construction (Phase 2).
- **`ArgumentDefinition`**: The canonical metadata for a command's argument, defining its name, `Kind`, and validation rules.
- **`Routine`**: The executable code (a Rust closure or function) associated with a command.
- **`Modality`**: A specific way of interacting with `utility1` (e.g., CLI, REPL, Web API).
- **`GenericInstruction`**: The structured, syntax-aware output of the `unilang_parser`, representing a parsed but unvalidated command invocation.
- **`VerifiedCommand`**: The output of the `Semantic Analyzer`; a command that has been validated against the `Command Registry` and is guaranteed to be executable.
- **`Pipeline`**: A high-level API object that orchestrates the full processing flow from string input to execution result.
- **`Kind`**: The data type of an argument (e.g., `Integer`, `String`, `List`, `Map`).

### Invariant Statement

All actors and terms defined in this document MUST have exactly one meaning throughout the entire codebase, documentation, and public API. No synonym, paraphrase, or alternative naming is permitted. This vocabulary is stable: terms defined here cannot be removed or redefined without a formal migration.

### Enforcement Mechanism

- Code review: PR reviewers check that new code uses only canonical terms
- Documentation review: New docs audited against this vocabulary
- Ubiquitous language enforcement in principles.rulebook.md

### Violation Consequences

Term collision causes: (1) ambiguous API contracts, (2) incorrect test assumptions, (3) documentation drift from implementation.

### Invariants

| File | Relationship |
|------|--------------|
| [003_governing_principles.md](003_governing_principles.md) | Principles that govern how terms are used |

### Architectures

| File | Relationship |
|------|--------------|
| [003_vision_scope.md](../architecture/003_vision_scope.md) | System context that defines these actors |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs` | Module structure reflecting actor boundaries |
