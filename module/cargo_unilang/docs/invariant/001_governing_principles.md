# Invariant: Governing Principles

### Scope

- **Purpose:** Define the permanent scope boundaries and design constraints that govern all cargo_unilang behavior
- **Responsibility:** Detection-only mandate, meta-compliance requirement, out-of-scope prohibitions
- **In Scope:** Detection-only constraint, unilang-as-framework requirement, scaffolding/detection scope definition
- **Out of Scope:** Individual command specifications, API signatures, exit code definitions

`cargo_unilang` must be detection-only (no auto-fix), meta-compliant (it must itself use `unilang` as its CLI framework), and scoped exclusively to scaffolding and anti-pattern detection for `unilang`-based projects — general-purpose Rust scaffolding, auto-correction of detected issues, and IDE integration are permanently out of scope.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | [feature/001_new_command.md](../feature/001_new_command.md) | .new command governed by the scaffolding scope |
| doc  | [feature/002_check_command.md](../feature/002_check_command.md) | .check command governed by the detection-only constraint |
| doc  | [api/001_cli_commands.md](../api/001_cli_commands.md) | CLI API that must use unilang as its framework |
