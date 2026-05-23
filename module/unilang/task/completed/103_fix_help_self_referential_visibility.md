# Fix .help Visible in Its Own Help Listing (BUG-102)

## Execution State

- **Status:** ✅ (Completed)
- **Executor Type:** AI
- **Actor:** claude-sonnet-4-6
- **Claimed At:** 2026-05-23
- **Priority:** 0
- **Closes:** BUG-102
- **Validated By:** w3 .test level::3 (container, 153/153 nextest pass)
- **Validation Date:** 2026-05-23

## Goal

Fix the `register_mandatory_global_help_command()` function so `.help` is hidden from its own help listing.

- **Motivated:** `dream::smoke::test_command_count_matches_spec` fails — gets 33 visible commands but expects 32; `.help` lists itself under "Help:" category, creating self-referential noise in every unilang-based CLI.
- **Observable:** `bug_reproducer(BUG-102)` test in `tests/help/enforcement.rs` passes; `dynamic.rs:583` reads `.with_hidden_from_list( true )`; dream smoke test passes with `command_count == 32`.
- **Scoped:** Single one-line change in `src/registry/dynamic.rs:583` inside `register_mandatory_global_help_command()`.
- **Testable:** `w3 .test level::3` passes with 0 failures, 0 warnings.

## In Scope

- Fix `src/registry/dynamic.rs:583`: `.with_hidden_from_list( false )` → `.with_hidden_from_list( true )`
- Update fix comment identifier: `Fix(issue-help-self-referential)` → `Fix(BUG-102)`
- Add `bug_reproducer(BUG-102)` test in `tests/help/enforcement.rs` with 5-section documentation
- Close BUG-102 in `task/bug/` (state Fixed, move to `task/bug/closed/`)

## Out of Scope

- Changing visibility of any other hidden/meta commands
- Changing public API of `HelpGenerator` or `CommandRegistry`
- Fixing related help formatting or content issues
- Refactoring `register_mandatory_global_help_command()`

## Work Procedure

1. Fix `dynamic.rs:583`: `.with_hidden_from_list( false )` → `.with_hidden_from_list( true )`
2. Update fix comment identifier to `Fix(BUG-102)`
3. Write `bug_reproducer(BUG-102)` MRE test in `tests/help/enforcement.rs`
4. Confirm test passes (container run: 153/153 pass)
5. Close BUG-102: update State to Fixed, add `fixed` history event, move to `task/bug/closed/`
6. Update `task/bug/readme.md`: move to Closed Bugs table
7. Update `task/readme.md`: add row 103

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|----------------|-------------------|-------------------|
| T01 | `CommandRegistry::new()` → `list_commands_filtered(None)` | unfixed: `hidden_from_list: false` | output contains `.help` (bug present) |
| T02 | `CommandRegistry::new()` → `list_commands_filtered(None)` | fixed: `hidden_from_list: true` | output does NOT contain `.help` as a listed command |
| T03 | dream `.help` → parse visible commands | fixed | 32 visible (not 33) |

## Acceptance Criteria

1. `grep 'hidden_from_list.*true' src/registry/dynamic.rs` → 1 match at line 588 area
2. `grep 'Fix(BUG-102)' src/registry/dynamic.rs` → 1 match
3. `grep 'bug_reproducer(BUG-102)' tests/help/enforcement.rs` → 1 match
4. `w3 .test level::3` → 0 failures, 0 warnings
5. `grep 'Fixed' task/bug/closed/102_help_self_referential_visibility.md` → 1 match (state changed)
6. No `.help` listed in `list_commands_filtered(None)` output from fresh `CommandRegistry`

## Validation

### Checklist

- [x] C1 — `dynamic.rs` has `.with_hidden_from_list( true )` at the mandatory help registration
- [x] C2 — Fix comment reads `Fix(BUG-102)` not `Fix(issue-*)`
- [x] C3 — `bug_reproducer(BUG-102)` test exists in `tests/help/enforcement.rs`
- [x] C4 — Test asserts `.help` absent from `list_commands_filtered(None)` output
- [x] C5 — Container nextest run: 153 pass, 0 fail
- [x] C6 — BUG-102 state is Fixed; file at `task/bug/closed/`
- [x] C7 — No regression in `test_mandatory_global_help_command` (`.help` still registered)

### Measurements

- [x] M1 — `grep -c 'hidden_from_list.*true' src/registry/dynamic.rs` → ≥ 1
- [x] M2 — Container nextest: 153/153 pass

### Invariants

- [x] I1 — `.help` command still registers successfully (mandatory global command invariant)
- [x] I2 — `list_commands_filtered(None)` output does NOT include `.help`
- [x] I3 — Hidden commands are not removed — they remain in registry, just filtered from listings

### Anti-faking Checks

- [x] AF1 — `test_mandatory_global_help_command` in `enforcement.rs` still passes (`.help` registered)
- [x] AF2 — Bug reproducer test actually fails without the fix (flag set to true, not just any value)
- [x] AF3 — Dream smoke test count is 32, not 33

## Requirements

Apply all rulebooks discovered via `kbase .role name::dev`. Key references:
- `code_design.rulebook.md § Bug-Fixing Procedural Script`
- `test_organization_universal.rulebook.md § Bug Reproducer Documentation Requirements`
- `code_hygiene.rulebook.md § Fix Documentation Quality Standard`
- `code_style.rulebook.md § Fix Comment Format Standard`

## Outcomes

- `src/registry/dynamic.rs:583` now reads `.with_hidden_from_list( true )` — `.help` hidden from its own listing
- Fix comment updated to `Fix(BUG-102)` per code_style.rulebook.md
- `bug_reproducer(BUG-102)` test added to `tests/help/enforcement.rs` with 5-section documentation
- Dream smoke test `test_command_count_matches_spec` passes: 32 visible commands (was 33)
- All 153 nextest tests pass in container (exit 0)
- BUG-102 closed (state: Fixed, moved to `task/bug/closed/`)
- Root cause: `register_mandatory_global_help_command()` set `hidden_from_list: false`; the interpreter's special `.help` handler calls `list_commands_filtered()` which unconditionally checks that flag
- Key pitfall: `command_add_runtime(.help)` always fails with `CommandAlreadyExists` — the only place to set `hidden_from_list` for `.help` is the mandatory registration in `dynamic.rs`
