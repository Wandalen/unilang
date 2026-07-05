# Fix YAML command loader validating bare `name` before namespace+name combination (issue-005)

## Execution State

- **Executor Type:** any
- **filed_by:** claude
- **actor:** null
- **started_at:** null
- **expires_at:** null
- **round:** 1
- **state:** ❓ (Unverified)
- **closes:** null
- **unit_type:** module
- **unit:** lib/yrd_core/unilang/dev/module/unilang
- **validated_by:** null
- **validation_date:** null

## Goal

`CommandRegistry::builder().load_from_yaml_str()` rejects a YAML command definition using the `namespace` + bare `name` format (e.g. `name: list` combined with `namespace: .session`) with `Invalid command name 'list': all commands must start with dot prefix` — even though combining `namespace` and `name` produces a valid dot-prefixed full name (`.session.list`) (Motivated: discovered as a side effect of validating an unrelated task (003); the crate's own test explicitly exercises two supported YAML authoring formats documented/tested as producing identical registered commands, and one of the two formats is currently broken; Observable: `cd module/unilang && cargo test --test registry -- registry::command_loader_yaml::test_ft15_yaml_format1_and_format2_produce_identical_command_name --exact --nocapture` panics with `called `Result::unwrap()` on an `Err` value: Yaml(Error(".[0]: Invalid command name 'list': all commands must start with dot prefix (e.g., '.command')", line: 2, column: 5))`; Scoped: the YAML-loading validation-ordering path only — likely `src/command_validation.rs:184` and/or `src/validation_core.rs:67` (both currently validate the raw `name` field's dot-prefix in isolation, without considering an accompanying `namespace` field that would combine into a valid full name); Testable: the FT-15 test above passes, and `cargo test -p unilang --all-features` no longer shows this specific failure).

## In Scope

- Fix the YAML command-loading validation path so that when a `namespace` field is present, the dot-prefix validation is applied to the COMBINED full name (`namespace` + `name`), not the bare `name` field in isolation
- 3-field source comment (`Fix(issue-005)`, `Root cause`, `Pitfall`) at the fixed location(s)
- Confirm no regression to the format-1 style (full dotted name directly in `name`, no separate `namespace` field) — both formats must continue to produce identical registered commands per the existing FT-15 test's own assertion

## Out of Scope

- Any change to the YAML schema itself (field names, structure)
- Any change to `command_validation.rs`/`validation_core.rs` logic for commands that don't use the `namespace` field at all
- The two clippy-lint test failures discovered in the same validation pass (`tests/build/build_runtime_separation.rs`, `tests/data/validated_command_name.rs`) — filed separately as task 006, unrelated subsystem
- Task 003's semantic-analyzer empty-path fix — unrelated subsystem, already complete

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- No mocking — test the real `CommandRegistry::builder().load_from_yaml_str()` path
- Bug reproducer already exists and is confirmed failing (`test_ft15_yaml_format1_and_format2_produce_identical_command_name`) — do not delete or weaken it

## Delivery Requirements

Execute in order. Do not skip or reorder steps.

1. **Confirm still failing** — `cd module/unilang && cargo test --test registry -- registry::command_loader_yaml::test_ft15_yaml_format1_and_format2_produce_identical_command_name --exact --nocapture`; confirm the exact panic captured in `## Outcomes` below (note: raw cargo output may be reformatted by a shell hook — if the result looks like a suspiciously terse one-line summary, prefix with `rtk proxy` to get unfiltered output)
2. **Read validation call sites** — `src/command_validation.rs:170-200`, `src/validation_core.rs:50-110`, and the YAML deserialization/builder path that calls them, to find exactly where `name` is validated before any `namespace` combination occurs
3. **Design fix** — validation must run against the combined full name when `namespace` is non-empty
4. **Implement fix** — add 3-field fix comment (`Fix(issue-005)`, `Root cause`, `Pitfall`)
5. **Confirm reproducer passes** — same command as step 1, expect ok/1 passed
6. **Green state** — `cargo test -p unilang --all-features` — this specific failure gone (task 006's failure may remain unless filed/fixed separately — do not treat that as blocking this task)
7. **Add/confirm test documentation** — 5-section doc comment on the FT-15 test per `test_organization.rulebook.md` if not already adequate
8. **Walk Validation** — walk every item in `## Validation`; attach evidence to `## Outcomes`
9. **Update task status** — set ✅ in `task/readme.md`, move file to `task/completed/`

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---|---|---|
| T01 | `name: list`, `namespace: .session` | current code | Yaml `Err` "Invalid command name 'list'" — FAILS (red state, confirmed) |
| T02 | same, after fix | — | Registers `.session.list`; matches format-1 equivalent registration |
| T03 | `name: .session.list`, no `namespace` (format 1) | after fix | Unchanged — still registers `.session.list` directly |
| T04 | `cargo test -p unilang --all-features` | after fix | This specific failure no longer present |

## Acceptance Criteria

**Bug-Fixing Quality Requirements (all 7 must be satisfied):**

1. **Rulebook compliance** — adheres to `code_design`, `codebase_hygiene`, `test_organization`, `code_style` rulebooks
2. **Test-first** — reproducer already written and confirmed failing (see `## Outcomes`)
3. **Evidence of failure** — exact panic output captured in `## Outcomes`
4. **Proper fix** — no mocking; addresses validation-ordering root cause (not a workaround); 3-field source comment present
5. **Fix validation** — reproducer confirmed failing without fix, passing with fix
6. **Knowledge preservation** — 5-section test documentation if not already present; 3-field source comment
7. **Code cleanliness** — no TODO/FIXME markers, no dead code, no `#[cfg(test)]` in `src/`

**Functional Acceptance Criteria:**

- Both YAML command-definition formats (direct dotted `name`, or `namespace` + bare `name`) produce identical registered commands
- No regression to any other existing YAML-loading test

## Validation

**Execution:** Independent validator (not executor) walks this section after SUBMIT per MAAV (`governance/maav.rulebook.md`) — dispatch independent subagents with at least one adversarial mandate; do not self-verify.

### Checklist

Desired answer for every question is YES.

- [ ] C1 — Reproducer test confirmed failing before fix, with exact evidence in `## Outcomes`?
- [ ] C2 — Does the fix validate the COMBINED name, not just the bare `name` field, when `namespace` is present?
- [ ] C3 — Does format-1 (no separate namespace field) still work unchanged?
- [ ] C4 — Is the 3-field fix comment (`Fix(issue-005)`, `Root cause`, `Pitfall`) present?
- [ ] C5 — Does `cargo test -p unilang --all-features` no longer show this specific failure?

### Anti-faking checks

- [ ] AF1 — Fix is in production code (`command_validation.rs`/`validation_core.rs`), not the test
- [ ] AF2 — Reproducer test assertions unchanged/unweakened from their current form

## Outcomes

*(Executor fills remaining fields during execution. Red-state evidence below was captured while validating an unrelated task (003); no changes were made to this bug as part of that filing.)*

**Red State Evidence (captured 2026-07-05, MRE run directly against unilang's public API):**

Command run:
```
cd module/unilang && cargo test --test registry -- registry::command_loader_yaml::test_ft15_yaml_format1_and_format2_produce_identical_command_name --exact --nocapture
```

Actual captured output:
```
thread 'registry::command_loader_yaml::test_ft15_yaml_format1_and_format2_produce_identical_command_name' panicked at module/unilang/tests/registry/command_loader_yaml.rs:616:81:
called `Result::unwrap()` on an `Err` value: Yaml(Error(".[0]: Invalid command name 'list': all commands must start with dot prefix (e.g., '.command')", line: 2, column: 5))
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 148 filtered out; finished in 0.00s
```

Reproduced deterministically across 3/3 independent runs (not flaky/order-dependent). Confirmed via `git log`/`git blame` to be unrelated to task 003's `core.rs` fix — no shared files, no shared subsystem; the affected files were last touched by unrelated `chore:`/`refactor:` commits weeks before task 003's fix commit.

**Fix Applied:** *(pending — fill in during execution)*

**Green State Confirmation:** *(pending — fill in during execution)*

**Key Learnings:** *(pending — fill in during execution)*

## History

*(append-only — newest entry last; never edit or remove past entries)*

- **2026-07-05** `FILED` — Discovered as a side effect of validating task 003 (unrelated semantic-analyzer empty-path fix) via an adversarial MAAV full-suite check. Filed as its own task, split from the clippy-lint defect (task 006) since the two share no root cause or subsystem. Not yet fixed; no production code touched as part of filing.

## Technical Context

### Root Cause (preliminary — confirm during Delivery Requirements steps 2-3)

`src/command_validation.rs:184` and `src/validation_core.rs:67` both raise `"Invalid command name '{}': all commands must start with dot prefix"` against the raw `name` field. When a YAML entry supplies `namespace: .session` alongside `name: list`, the intended combined full name is `.session.list` (valid, dot-prefixed) — but validation appears to run against `list` in isolation, before any namespace-prefixing/combination step occurs, causing a false-positive rejection of an otherwise-valid, intentionally-supported authoring format.

### Impact

The `namespace` + bare-`name` YAML authoring format (as opposed to writing the full dotted name directly) is entirely unusable for any command whose bare name lacks a dot prefix — which is the normal case, since the whole point of the `namespace` field is to avoid repeating the namespace prefix on every command's `name`.

## Cross-References

- **Reproducer:** `unilang/tests/registry/command_loader_yaml.rs:577` (function `test_ft15_yaml_format1_and_format2_produce_identical_command_name`), panic at line 616
- **Suspected validation call sites:** `unilang/src/command_validation.rs:184`, `unilang/src/validation_core.rs:67`
- **Error definition:** `unilang/src/error.rs:68`
- **Discovered during:** task 003 validation (2026-07-05); unrelated subsystem, no shared code path
- **Related sibling finding:** task 006 (clippy-lint failures, different subsystem, filed same day)
