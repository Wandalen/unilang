# Feature Spec: Help System

### Scope

- **Purpose:** Verify all FR-HELP behavioral requirements for help generation and access
- **Responsibility:** Test cases covering command list generation, detailed help, `?` operator, `.cmd.help` commands, `??` parameter, automatic help API, and verbosity levels
- **In Scope:** FR-HELP-1 (command list), FR-HELP-2 (detailed command help), FR-HELP-3 (`?` operator), FR-HELP-4 (`.cmd.help` commands), FR-HELP-5 (`??` parameter), FR-HELP-6 (automatic help API), FR-HELP-7 (verbosity levels 0–4 via `UNILANG_HELP_VERBOSITY`)
- **Out of Scope:** Registry initialization (FR-REG); argument parsing (FR-ARG); REPL state (FR-REPL)

### FT-1: Command list returns all registered command names

- **Given:** A registry with three commands: `.foo`, `.bar`, `.baz`
- **When:** The help API is called to generate the command list (e.g., `help_list(&registry)`)
- **Then:** The output contains `.foo`, `.bar`, and `.baz`; no other names appear; no panic on empty registry

### FT-2: Detailed help output includes argument names and descriptions

- **Given:** A registry containing `.greet` with one argument `"name"` described as "The recipient's name"
- **When:** `help_command(&registry, ".greet")` is called
- **Then:** Output contains both `"name"` and `"The recipient's name"` in the help text

### FT-3: `?` operator alone produces command list output

- **Given:** A `Pipeline` with a registry containing `.foo`
- **When:** `pipeline.run("?")` is called
- **Then:** Output contains `.foo` in a list format; returns without error

### FT-4: `??` parameter causes help output and no command execution

- **Given:** A `Pipeline` with `.greet` registered and input `".greet ??"`
- **When:** `pipeline.run(".greet ??")` is called
- **Then:** Help text for `.greet` is included in output; the command's handler function is not called

### FT-5: UNILANG_HELP_VERBOSITY=0 produces minimal output; =4 produces full output

- **Given:** A command `.greet` with one argument `"name"` that has a description, a type, and a default value
- **When:** `help_command` is called first with `UNILANG_HELP_VERBOSITY=0` set, then with `UNILANG_HELP_VERBOSITY=4`
- **Then:** At level 0 the output contains only command name; at level 4 the output additionally contains argument type and default value; the two outputs are observably different strings

### FT-6: `.greet.help` command is automatically available and returns help

- **Given:** A registry where `.greet` is registered and automatic help API is enabled
- **When:** `pipeline.run(".greet.help")` is called
- **Then:** Returns help text for `.greet` equivalent to calling the help API directly; no error

### FT-7: `?` operator with unknown command name returns not-found message

- **Given:** A `Pipeline` with a registry that does not contain `.unknown`
- **When:** `pipeline.run("? .unknown")` is called
- **Then:** Output includes a not-found indication for `.unknown` (with optional Levenshtein suggestions); no panic
