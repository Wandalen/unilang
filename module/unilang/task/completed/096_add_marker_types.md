# Add `ShellArgv`/`ReplInput` Marker Types and `parse_cli`/`parse_repl` Methods

## Execution State

- **Status:** ✅ (Completed)
- **Executor Type:** AI
- **Actor:** null
- **Claimed At:** null
- **Priority:** 0
- **Validated By:** N/A
- **Validation Date:** 2026-04-18

## Goal

Introduce `ShellArgv(Vec<String>)` and `ReplInput(String)` as zero-cost newtype wrappers in `unilang_parser`, export them from both `unilang_parser` and `unilang`, and add `Parser::parse_cli(&ShellArgv)` and `Parser::parse_repl(&ReplInput)` as type-safe entry points. After this task, passing a `Vec<String>` directly to `parse_repl` is a compile-time error.

- **Motivated:** DEMAND 5 from task 086 requires compile-time enforcement of argv-vs-string separation.
- **Observable:** `grep -c "pub struct ShellArgv\|pub struct ReplInput" module/unilang_parser/src/argv_types.rs` → 2; types in `cargo doc`.
- **Scoped:** New file `argv_types.rs` in `unilang_parser/src/`; 2 new methods in `mod.rs`; re-exports in both crates.
- **Testable:** `cargo nextest run -p unilang_parser -- argv_types` → PASS (≥2 tests).

## In Scope

- New file `module/unilang_parser/src/argv_types.rs` with `ShellArgv` and `ReplInput` newtypes
- `Parser::parse_cli(&ShellArgv)` and `Parser::parse_repl(&ReplInput)` methods
- Re-exports from `unilang_parser` prelude and `unilang` crate
- TDD test file `module/unilang_parser/tests/argv_types.rs` (written BEFORE implementation)
- `From` implementations for ergonomic construction

## Out of Scope

- Migrating callers to use `ShellArgv`/`ReplInput` (optional, callers can adopt gradually)
- Removing the raw `parse_from_argv(&[String])` API
- Higher-level `process_command_from_argv_typed` pipeline wrapper

## Work Procedure

1. Create `module/unilang_parser/tests/argv_types.rs` with 2+ failing tests using `ShellArgv`, `ReplInput`, `parse_cli`, `parse_repl` — RED
2. Run `cargo nextest run -p unilang_parser -- argv_types` → FAIL (types don't exist) — confirm RED
3. Create `module/unilang_parser/src/argv_types.rs` with `ShellArgv(Vec<String>)` and `ReplInput(String)`, `From` impls, `from_env()`/`from_vec()`/`as_slice()` on ShellArgv, `new()`/`as_str()` on ReplInput
4. Add `pub mod argv_types;` and re-exports to `module/unilang_parser/src/lib.rs` prelude
5. Add `parse_cli(&ShellArgv)` and `parse_repl(&ReplInput)` to `Parser` in `mod.rs`
6. Add re-export to `module/unilang/src/lib.rs`
7. Run `cargo nextest run -p unilang_parser -- argv_types` → PASS — GREEN
8. Run `w3 .test level::3` → 0 failures, 0 warnings

## Test Matrix

| Scenario | Expected |
|----------|----------|
| `ShellArgv::from_vec(vec!["prog", ".cmd", "key::val"])` passed to `parse_cli` | `Ok(GenericInstruction)` |
| `ReplInput::new(".cmd key::val")` passed to `parse_repl` | `Ok(GenericInstruction)` |
| `ShellArgv(vec![])` passed to `parse_cli` | Error (no command path) |
| Passing `Vec<String>` to `parse_repl` | Compile-time error (type mismatch) |

## Acceptance Criteria

1. `grep -c "pub struct ShellArgv\|pub struct ReplInput" module/unilang_parser/src/argv_types.rs` → 2
2. `grep -c "pub fn parse_cli\|pub fn parse_repl" module/unilang_parser/src/parser_engine/mod.rs` → 2
3. `grep -c "ShellArgv\|ReplInput" module/unilang/src/lib.rs` → ≥ 1
4. `cargo nextest run -p unilang_parser -- argv_types` → PASS (0 FAILED)
5. `grep "type ShellArgv\|type ReplInput" module/unilang_parser/src/argv_types.rs | wc -l` → 0 (must be newtypes, not aliases)
6. `w3 .test level::3` → 0 failures, 0 warnings

## Validation

### Checklist

- [ ] C1 — Were the failing tests written BEFORE the implementation (TDD red confirmed)?
- [ ] C2 — Do `ShellArgv` and `ReplInput` exist as newtype structs (not type aliases)?
- [ ] C3 — Are all public items in `argv_types.rs` documented?
- [ ] C4 — Do `parse_cli` and `parse_repl` delegate to the existing impl methods?
- [ ] C5 — Are both types re-exported from `unilang` crate?
- [ ] C6 — Do at least 2 new tests in `tests/argv_types.rs` pass?
- [ ] C7 — Does `w3 .test level::3` pass with 0 failures and 0 warnings?

### Measurements

- [ ] M1 — `grep -c "pub struct ShellArgv\|pub struct ReplInput" module/unilang_parser/src/argv_types.rs` → 2 (was: file absent)
- [ ] M2 — `grep -c "pub fn parse_cli\|pub fn parse_repl" module/unilang_parser/src/parser_engine/mod.rs` → 2 (was: 0)
- [ ] M3 — `grep -c "ShellArgv\|ReplInput" module/unilang/src/lib.rs` → ≥ 1 (was: 0)

### Invariants

- [ ] I1 — `RUSTFLAGS="-D warnings" cargo check -p unilang_parser -p unilang --all-features` → exit 0

### Anti-faking Checks

- [ ] AF1 — test file `tests/argv_types.rs` was created before `src/argv_types.rs`
- [ ] AF2 — `grep -c "type ShellArgv\|type ReplInput" module/unilang_parser/src/argv_types.rs` → 0 (no aliases)
- [ ] AF3 — `grep -c "assert" module/unilang_parser/tests/argv_types.rs` → ≥ 2

## Requirements

Apply all rulebooks discovered via `kbase .role name::dev`. Key: newtypes not type aliases, all public items documented, tests in `tests/` directory only.

## Outcomes

Delivered as specified. `ShellArgv(Vec<String>)` and `ReplInput(String)` newtypes implemented in `module/unilang_parser/src/argv_types.rs`. `Parser::parse_cli(&ShellArgv)` and `Parser::parse_repl(&ReplInput)` type-safe entry points added to `mod.rs`. Both types re-exported from `unilang_parser` prelude and from `unilang`. Test file `tests/argv_types.rs` covers all Test Matrix rows (319 lines). `w3 .test level::3` passes with 0 failures and 0 warnings. Passing a bare `Vec<String>` to `parse_repl` is now a compile-time error as required.
