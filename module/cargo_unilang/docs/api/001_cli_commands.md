# API: CLI Commands

### Scope

- **Purpose:** Document the public CLI command surface, parameter contracts, and exit code semantics for cargo_unilang
- **Responsibility:** Command list, shared parameters, exit code definitions
- **In Scope:** .new, .check, .help command signatures; verbosity parameter; exit code mapping
- **Out of Scope:** Implementation details, anti-pattern detection logic, scaffolding template internals

`cargo_unilang` exposes three commands via the `unilang` CLI framework: `.new` (scaffold a new project), `.check` (validate an existing project for anti-patterns), and `.help` (display usage information); all commands accept a `verbosity::<0-5>` parameter and return documented exit codes (0 = success, 1 = issues found, 2 = invalid parameters, 3 = path/creation error).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | [feature/001_new_command.md](../feature/001_new_command.md) | .new command behavioral requirements |
| doc  | [feature/002_check_command.md](../feature/002_check_command.md) | .check command behavioral requirements |
| doc  | [invariant/001_governing_principles.md](../invariant/001_governing_principles.md) | Scope constraints governing all commands |
