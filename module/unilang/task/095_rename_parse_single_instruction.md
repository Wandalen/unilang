# Rename `parse_single_instruction` → `parse_repl_input` with Deprecation Shim

## Execution State

- **Status:** ✅ (Complete)
- **Executor Type:** AI
- **Actor:** Claude (exec_pln)
- **Created:** 2026-04-18
- **Priority:** 3
- **Claims:** 1

## Goal

Add `parse_repl_input` as a new public method on `Parser` that delegates to the existing `parse_single_instruction`; migrate all 302 call sites in the workspace from `.parse_single_instruction(` to `.parse_repl_input(`; then mark `parse_single_instruction` as `#[deprecated]` with a forwarding shim to `parse_repl_input`. After this task, `parse_repl_input` is the canonical name for REPL/string input parsing.

**Motivated:** The name `parse_single_instruction` is misleading — it sounds primary but is only correct for REPL/string input. DEMAND 2 from task 086 requires this rename.
**Observable:** `grep -c "pub fn parse_repl_input" module/unilang_parser/src/parser_engine/mod.rs` → 1; `grep -r "\.parse_single_instruction(" module/ --include="*.rs" | grep -v "pub fn\|#\[deprecated" | wc -l` → 0.
**Scoped:** `unilang_parser` crate (source) + all callers in `unilang` crate; no other crates.
**Testable:** `w3 .test level::3` passes with 0 failures and 0 warnings after all three phases.

## In Scope

- Adding `parse_repl_input` method to `Parser` (Phases 1)
- Migrating all 302 `.parse_single_instruction(` call sites (Phase 2)
- Adding `#[deprecated]` to `parse_single_instruction` with delegation to `parse_repl_input` (Phase 3)
- Updating `lib.rs` doc examples and `docs/cli_integration.md` (Phase 3)
- TDD test for the new method (Phase 1)

## Out of Scope

- Removing `parse_single_instruction` (deferred to semver major)
- Adding `ShellArgv`/`ReplInput` marker types (task 096)
- Any changes outside `module/unilang_parser/` and `module/unilang/`

## Work Procedure

1. Write failing test in `module/unilang_parser/tests/comprehensive_tests.rs` asserting `parse_repl_input` exists and delegates identically — RED
2. Add `parse_repl_input` method to `Parser` in `mod.rs` (one-line delegation to `parse_single_instruction`) — GREEN
3. Run `cargo nextest run -p unilang_parser parse_repl_input_exists` → PASS
4. Update `lib.rs` doc examples: change 2 occurrences of `parse_single_instruction` → `parse_repl_input` (BEFORE adding `#[deprecated]`)
5. Add `#[deprecated(since = "X.Y.Z", note = "Use parse_repl_input()...")]` to `parse_single_instruction` with body calling `parse_repl_input` (delegation inversion)
6. Run batch sed: `find module/ -name "*.rs" -exec sed -i 's/\.parse_single_instruction(/.parse_repl_input(/g' {} +`
7. Verify zero call sites: `grep -r "\.parse_single_instruction(" module/ --include="*.rs" | grep -v "pub fn\|#\[deprecated" | wc -l` → 0
8. Update `docs/cli_integration.md` — `parse_single_instruction` → `parse_repl_input` throughout (≤2 remaining)
9. Run `w3 .test level::3` → 0 failures, 0 warnings

## Test Matrix

| Scenario | Expected |
|----------|----------|
| `parse_repl_input(".cmd key::val")` called on Parser | Same result as `parse_single_instruction(".cmd key::val")` |
| `parse_single_instruction` called by external caller | Compiler deprecation warning (compile-time) |
| All 302 migrated call sites | Still compile and behave identically |
| `w3 .test level::3` after all phases | 0 failures, 0 warnings |

## Acceptance Criteria

1. `grep -c "pub fn parse_repl_input" module/unilang_parser/src/parser_engine/mod.rs` → 1
2. `grep -r "\.parse_single_instruction(" module/ --include="*.rs" | grep -v "pub fn\|#\[deprecated" | wc -l` → 0
3. `grep -c "#\[deprecated" module/unilang_parser/src/parser_engine/mod.rs` → 1
4. `grep -c "parse_single_instruction" module/unilang_parser/src/lib.rs` → 0
5. `w3 .test level::3` → 0 failures, 0 warnings

## Validation

### Checklist

- [ ] C1 — Does `parse_repl_input` exist as a public method with a doc comment?
- [ ] C2 — Are zero `.parse_single_instruction(` call sites remaining in the workspace (excluding definition and deprecated attr)?
- [ ] C3 — Is `#[deprecated]` present on `parse_single_instruction` with body calling `parse_repl_input`?
- [ ] C4 — Are lib.rs doc examples updated (0 occurrences of `parse_single_instruction`)?
- [ ] C5 — Does `w3 .test level::3` pass with 0 failures and 0 warnings?

### Measurements

- [ ] M1 — `grep -c "pub fn parse_repl_input" module/unilang_parser/src/parser_engine/mod.rs` → 1 (was: 0)
- [ ] M2 — `grep -r "\.parse_single_instruction(" module/ --include="*.rs" | grep -v "pub fn\|#\[deprecated" | wc -l` → 0 (was: 302)
- [ ] M3 — `grep -c "#\[deprecated" module/unilang_parser/src/parser_engine/mod.rs` → 1 (was: 0)

### Invariants

- [ ] I1 — `RUSTFLAGS="-D warnings" cargo check -p unilang_parser -p unilang --all-features` → exit 0

### Anti-faking Checks

- [ ] AF1 — TDD test was written before implementation: test file shows the test function name exists
- [ ] AF2 — `grep -r "\.parse_repl_input(" module/ --include="*.rs" | wc -l` → ≥ 200 (actual migration happened)

## Requirements

Apply all rulebooks discovered via `kbase .role name::dev`. Key: code_style.rulebook.md (2-space indent), test_organization.rulebook.md (tests in `tests/`), codebase_hygiene.rulebook.md (no backups).
