# Feature: `.new` Project Scaffolding Command

### Scope

- **Purpose:** Define requirements for the project scaffolding command that generates a correct unilang project structure
- **Responsibility:** Scaffolded file list, accepted parameters, generation constraints (no build.rs)
- **In Scope:** .new command behavior, generated file set, template options, parameter specifications
- **Out of Scope:** .check command, CLI framework requirements, governing scope constraints

`cargo_unilang` must provide a `.new project::<name>` command that creates a correctly structured `unilang` CLI project containing `Cargo.toml`, `src/main.rs`, and `commands.yaml` — with no `build.rs` generated, since `unilang` provides build logic automatically — and that accepts `template::minimal|full`, `author`, `license`, and `verbosity` parameters.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | [api/001_cli_commands.md](../api/001_cli_commands.md) | CLI command signatures and exit codes for .new |
| doc  | [invariant/001_governing_principles.md](../invariant/001_governing_principles.md) | Scope constraints governing the .new command |
