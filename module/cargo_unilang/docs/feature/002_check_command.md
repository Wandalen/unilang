# Feature: `.check` Health Check Command

### Scope

- **Purpose:** Define requirements for the anti-pattern detection command that validates existing unilang projects
- **Responsibility:** Detected anti-pattern list, reporting format, exit code contract, detection-only constraint
- **In Scope:** .check command behavior, anti-pattern categories (custom build.rs, duplicate deps, deprecated API), exit codes
- **Out of Scope:** .new command, auto-fix behavior (prohibited by invariant), IDE integration

`cargo_unilang` must provide a `.check [path::<dir>]` command that detects common anti-patterns in existing `unilang` projects — custom `build.rs` that duplicates unilang's build logic, duplicate dependencies already provided transitively by `unilang`, and deprecated API usage — reporting each issue with location and suggested fix, and exiting with code `1` when any issues are found.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | [api/001_cli_commands.md](../api/001_cli_commands.md) | CLI command signatures and exit codes for .check |
| doc  | [invariant/001_governing_principles.md](../invariant/001_governing_principles.md) | Detection-only constraint that governs .check behavior |
