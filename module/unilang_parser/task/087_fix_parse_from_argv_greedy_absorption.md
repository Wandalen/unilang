# Fix `parse_from_argv` greedy multi-word absorption

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** 🎯 (Available)
- **Progress:** 0/8 — not started

## Goal

Fix `parse_from_argv` so that bare positional argv tokens after a named parameter (`key::value`) are NOT absorbed into that parameter's value. Currently, any subsequent token without `::` or leading `.` is greedily concatenated with a space separator (lines 1186-1191), silently mangling arguments. After this fix: `["repo::Wandalen/willbe", "willbe/assistant"]` parses `repo` as `"Wandalen/willbe"` only; `"willbe/assistant"` becomes a separate positional argument (or triggers an "excess positional" error if no parameter accepts it). Verified by `w3 .test l::3` passing with zero failures. (Motivated: Users hit silent data corruption — the mangled URL passes through to `git clone`, fails with a cryptic remote error, and registers a broken entry in `.wip/wip.yaml`; Observable: `parse_from_argv(&[".add", "repo::Wandalen/willbe", "willbe/assistant"])` returns repo=`"Wandalen/willbe"` not repo=`"Wandalen/willbe willbe/assistant"`; Scoped: `parser_engine/mod.rs` lines 1148-1193 only; Testable: `w3 .test l::3` passes, new tests assert correct token boundary.)

## In Scope

- `src/parser_engine/mod.rs` lines 1148-1193 — modify the greedy absorption `while` loop stop conditions
- `tests/` — new test file `parse_from_argv_boundary_test.rs` covering all Test Matrix rows
- `tests/readme.md` — add row for new test file

## Out of Scope

- `parse_single_instruction` changes (string-based parser, different concern)
- `cli_parser.rs` validation (covered by TSK-086)
- Multi-word values for `message::` type parameters (intentional design — must continue working)
- Downstream wip handler changes (wip should not need to change if parser is correct)
- The argv misuse warning system (separate detection layer)

## References

- Root cause investigation: `wip .add repo::Wandalen/willbe willbe/assistant` produces `repo::"Wandalen/willbe willbe/assistant"` — space injected at `mod.rs:1186-1191`
- Counterexample: `wip .add repo::Wandalen/willbe path::mypath` works because `path::` contains `::`, triggering the break at line 1156
- TSK-086 Out of Scope explicitly excluded `parse_from_argv` — this bug was not known at TSK-086 creation time
- Rulebooks: code_design.rulebook.md, codebase_hygiene.rulebook.md, test_organization.rulebook.md, code_style.rulebook.md

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- No mocking — use real parser instances
- Custom code style (2-space indent); never run `cargo fmt`
- Bug reproducer test must be created BEFORE the fix (test-first TDD)
- Evidence of test failure must be captured before implementing fix
- 3-field source comment required at fix site (`Fix(issue-087)`, `Root cause`, `Pitfall`)
- 5-section test documentation required (Root Cause, Why Not Caught, Fix Applied, Prevention, Pitfall)
- No legacy code, no duplication, no mocking
- Validation: `w3 .test l::3` from `module/unilang_parser/` — zero failures, zero warnings

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note constraints on error handling and code style.
2. **Write Test Matrix** — populate all rows before opening any test file.
3. **Write failing bug reproducer test** — implement `bug_reproducer(issue-087)` test asserting `parse_from_argv(&[".add", "repo::Wandalen/willbe", "willbe/assistant"])` produces `repo = "Wandalen/willbe"` and `willbe/assistant` as a separate positional. Confirm it fails (repo = `"Wandalen/willbe willbe/assistant"`). Capture failure output as evidence.
4. **Write remaining failing tests** — implement all Test Matrix rows. Confirm T01-T04 fail, T05-T06 pass (regression guards).
5. **Implement fix** — modify `while` loop at `mod.rs:1186-1191` to stop absorbing when the next token could be a standalone positional argument. Add 3-field source comment. Key constraint: `message::hello world` (two argv tokens intended as one value) must still work — the stop condition needs to distinguish between "continuation of a multi-word value" and "separate positional argument".
6. **Green state** — `w3 .test l::3` passes; same or higher test count, zero regressions, zero warnings.
7. **Document** — add 5-section bug documentation to test file. Update `tests/readme.md`.
8. **Update task status** — mark ✅, move to completed/.

## Test Matrix

| # | Input Scenario | Expected Behavior |
|---|---------------|-------------------|
| T01 | `parse_from_argv(&[".add", "repo::Wandalen/willbe", "willbe/assistant"])` | `repo` = `"Wandalen/willbe"`, `willbe/assistant` = separate positional |
| T02 | `parse_from_argv(&[".add", "repo::Wandalen/willbe", "path::mydir"])` | `repo` = `"Wandalen/willbe"`, `path` = `"mydir"` (regression — already works) |
| T03 | `parse_from_argv(&[".cmd", "message::hello", "world"])` | `message` = `"hello world"` (multi-word value — must continue working) |
| T04 | `parse_from_argv(&[".add", "repo::Wandalen/willbe", "extra1", "extra2"])` | `repo` = `"Wandalen/willbe"`, two separate positionals |
| T05 | `parse_from_argv(&[".add", "repo::git@github.com:user/repo.git"])` | `repo` = `"git@github.com:user/repo.git"` (single token, no absorption — regression) |
| T06 | `parse_from_argv(&[".status"])` | No parameters, no absorption (regression guard) |

## Acceptance Criteria

- `parse_from_argv(&[".add", "repo::Wandalen/willbe", "willbe/assistant"])` returns repo=`"Wandalen/willbe"` only — NOT `"Wandalen/willbe willbe/assistant"`
- `parse_from_argv(&[".cmd", "message::hello", "world"])` still returns message=`"hello world"` (multi-word absorption preserved for intended use case)
- Bug reproducer test `bug_reproducer(issue-087)` exists and passes after fix
- 3-field source comment at fix site in `parser_engine/mod.rs`
- 5-section test documentation in test file
- `w3 .test l::3` passes with zero failures and zero warnings
- No tests removed or disabled to make suite pass

## Validation

### Checklist

- [ ] Does `parse_from_argv` stop absorbing bare positional tokens after named params?
- [ ] Does multi-word value absorption (`message::hello world`) still work?
- [ ] Does `bug_reproducer(issue-087)` marker exist in test code?
- [ ] Does source code contain 3-field `Fix(issue-087)` comment?
- [ ] Does test file contain all 5 documentation sections?

### Measurements

| Metric | Expected | Command |
|--------|----------|---------|
| Bug reproducer test count | ≥1 | `grep -r "bug_reproducer(issue-087)" tests/` |
| Fix comment count | ≥1 | `grep -r "Fix(issue-087)" src/` |
| Test failures | 0 | `w3 .test l::3` |
| Test count | ≥ previous count | `w3 .test l::3` summary |

### Invariants

- `parse_from_argv` produces identical results to `parse_single_instruction` for all single-token named params
- Multi-word value absorption works when explicitly intended (quote-based or message-type params)

### Anti-faking checks

- `grep -rn "bug_reproducer(issue-087)" tests/` must return at least 1 result
- `grep -rn "Fix(issue-087)" src/parser_engine/mod.rs` must return at least 1 result
- Test count must be ≥ pre-fix count (no tests deleted)
- T03 must still pass — multi-word absorption not broken
