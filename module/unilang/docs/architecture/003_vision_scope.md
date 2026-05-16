# Architecture: Vision & Scope

### Scope

- **Purpose:** Define the framework's core vision, system scope, and design goals
- **Responsibility:** What the framework aims to achieve, what is and isn't in scope
- **In Scope:** Core vision, scope boundaries, design goals, supported modalities
- **Out of Scope:** Specific FR requirements, API contracts, implementation details

### Core Vision: Define Once, Use Everywhere

The `unilang` framework **must** provide a unified way to define command-line utility interfaces once, automatically enabling consistent interaction across multiple modalities such as CLI, TUI, GUI, and Web APIs. The core goals are:

- **Consistency:** A single, declarative way to define commands and their arguments, regardless of how they are presented or invoked.
- **Discoverability:** Easy ways for users and systems to find available commands and understand their usage through an automated help system.
- **Flexibility:** Support for various methods of command definition (compile-time, run-time, declarative, procedural).
- **Extensibility:** Provide structures that enable an integrator to build an extensible system.
- **Efficiency:** Support for efficient parsing and zero-overhead command dispatch for statically defined commands.
- **Interoperability:** A standardized representation for commands, enabling integration with other tools or web services.
- **Robustness:** Clear, user-friendly error handling and a rich argument validation system.
- **Security:** Provide a framework for defining and enforcing secure command execution.

### In Scope: The Multi-Crate Framework

The Unilang specification governs a suite of related crates that work together to provide the full framework functionality. The primary crates **must** be:

- **`unilang`**: The core framework crate that orchestrates parsing, semantic analysis, execution, and modality management. It provides the primary public API for integrators.
- **`unilang_parser`**: A dedicated, low-level crate responsible for the lexical and syntactic analysis of the `unilang` command language.
- **`unilang_meta`**: A companion crate providing procedural macros (e.g., `#[command]`) to simplify compile-time command definition.

### Out of Scope

The `unilang` framework is responsible for the command interface and execution pipeline, not the business logic itself. The following are explicitly out of scope for the framework:

- **Business Logic Implementation:** The framework will invoke command `Routines`, but the implementation of the business logic within those routines is the responsibility of the `Integrator`.
- **Transactional Guarantees:** The framework does not provide transactional guarantees for sequences of commands. A failure in one command in a sequence does not automatically roll back the effects of previously executed commands.
- **Inter-Command State Management:** The framework provides an `ExecutionContext` for passing data to commands, but it does not manage complex state between command invocations. State management is the responsibility of the `Integrator`.
- **User Interface (UI) Rendering:** The framework provides the data and structure for different modalities (CLI, TUI, GUI) but does not render the UI itself. UI rendering is the responsibility of modality-specific crates or the `Integrator`'s application.

### CLI Modality: Language Syntax & Processing

The `unilang_parser` crate **must** be the reference implementation for this section. The parser **must** adhere to the following rules in order:

- **Rule 1 (Tokenization):** Whitespace separates tokens. Quoted strings (`'...'` or `"..."`) are treated as a single token.
- **Rule 2 (Command Path):** The command path is the first token. It **must** be a dot-separated identifier (e.g., `.system.echo`). A leading dot is optional.
- **Rule 3 (Arguments):** All subsequent tokens are arguments.
  - **Named Arguments:** **Must** use the `name::value` syntax.
  - **Positional Arguments:** Any token that is not a named argument is a positional argument.
- **Rule 4 (Help Operator):** The `?` operator, if present, **must** be the final token and triggers the help system.
- **Rule 5 (Double Question Mark Parameter):** The `??` parameter, if present as any argument, **must** trigger help display for the command, identical to calling `.command.help`. This provides a consistent alternative to the `?` operator.
- **Rule 6 (Special Case — Discovery):** A standalone dot (`.`) **must** be interpreted as a request to list all available commands.

### Feature Instances

| File | Relationship |
|------|--------------|
| [001_command_registry.md](../feature/001_command_registry.md) | Core registration requirement from this vision |

### Invariant Instances

| File | Relationship |
|------|--------------|
| [001_system_actors_vocabulary.md](../invariant/001_system_actors_vocabulary.md) | Actors defined for this scope |

### Architecture Instances

| File | Relationship |
|------|--------------|
| [001_mandates.md](001_mandates.md) | Mandates implementing this vision |
| [006_repl_implementation.md](006_repl_implementation.md) | REPL modality implementation of this vision |
