# Fix YAML command loader validating bare `name` before namespace+name combination (issue-005)

## Execution State

- **Executor Type:** any
- **filed_by:** claude
- **actor:** null
- **started_at:** null
- **expires_at:** null
- **round:** 1
- **state:** ✅ (Completed)
- **closes:** null
- **unit_type:** module
- **unit:** lib/yrd_core/unilang/dev/module/unilang
- **validated_by:** MAAV (3 independent subagents, Round 1 Full, general-purpose, one adversarial mandate) — see `## Validation` and `## Outcomes` for full record
- **validation_date:** 2026-07-06

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

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`); fix comment format governed by `l2_imp_universal.rulebook.md § Comments : Fix Comment Format Standard` (corrected 2026-07-06 — "code_style" is not the canonical name nor an official alias; found via independent MAAV validator, non-blocking)
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

- [x] C1 — Reproducer test confirmed failing before fix, with exact evidence in `## Outcomes`? YES — Red State Evidence section; deterministic across 3/3 runs.
- [x] C2 — Does the fix validate the COMBINED name, not just the bare `name` field, when `namespace` is present? YES — confirmed by all 3 MAAV dimensions: bare name + non-empty namespace now succeeds (namespace supplies the dot prefix); bare name + empty/absent namespace still correctly rejected (MAAV Dimension 3 independently wrote and ran ad-hoc coverage confirming this).
- [x] C3 — Does format-1 (no separate namespace field) still work unchanged? YES — MAAV Dimension 1 confirmed via full `command_loader_yaml.rs` run (6/6 tests pass, including `test_load_from_yaml_str_simple_command` and `test_load_from_yaml_str_multiple_commands`, both dotted-name-with-explicit-namespace shapes that hit the unchanged passthrough branch).
- [x] C4 — Is the 3-field fix comment (`Fix(issue-005)`, `Root cause`, `Pitfall`) present? YES — `serde_impl.rs:313-322`, confirmed verbatim by MAAV Dimension 2 against the real governing rule (`l2_imp_universal.rulebook.md § Comments : Fix Comment Format Standard`), field-for-field match.
- [x] C5 — Does `cargo test -p unilang --all-features` no longer show this specific failure? YES — confirmed independently by all 3 MAAV dimensions; full suite (`--no-fail-fast`) shows 0 failures across all 18 test binaries (`registry`: 149/0 including FT-15).

### Anti-faking checks

- [x] AF1 — Fix is in production code (`command_validation.rs`/`validation_core.rs`), not the test? YES (location differs from the preliminary guess, confirmed correct during investigation) — the actual fix is in `src/data/command_definition/serde_impl.rs` (production code, the true root-cause site — the task's own preliminary guess at `command_validation.rs`/`validation_core.rs` was investigated and ruled out by error-text mismatch before the fix was designed). MAAV Dimension 3 confirmed via `git diff --stat` that the only `src/` change is this file; the 2 `tests/` files touched belong to unrelated task 006.
- [x] AF2 — Reproducer test assertions unchanged/unweakened from their current form? YES — MAAV Dimension 3 confirmed via `git diff`/`git log -p` that `tests/registry/command_loader_yaml.rs` has zero modifications; all 7 original FT-15 assertions intact.

**Validation Round 1 result: 7/7 items PASS directly, in a single Full Round (Round 1) — CONVERGED per `governance/maav.rulebook.md § MAAV : Round Type Selection`. Validated via 3 independent MAAV subagents (general-purpose, one with an explicit adversarial mandate). Round Header + Result Table surfaced to user before this file was closed. One non-blocking finding applied as a correction above (imprecise `code_style` rulebook citation — not canonical, not an official alias).**

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

**Fix Applied (2026-07-06):**

Root cause was NOT at the preliminary-suspected locations (`command_validation.rs:184` / `validation_core.rs:67` — both validate the already-combined `full_name()`, confirmed by their error text using `'.chat'` in the example, which doesn't match the observed `'.command'` wording). Traced via the actual `Error::MissingDotPrefix` construction site (`src/data/command_name.rs:86`, the sole caller) back to `src/data/command_definition/serde_impl.rs` — the custom `serde::Deserialize` impl for `CommandDefinition`. The `name` map-visitor local was declared `Option<CommandName>` (line 100), so `map.next_value()?` (line 133) deserialized the raw YAML `name` scalar directly into a validated `CommandName` — running dot-prefix validation the instant the field was parsed, before the `namespace` field's value was available for combination, and with no path to combine them even if it had been.

Fix, in `src/data/command_definition/serde_impl.rs`:
1. Changed the `name` local from `Option<CommandName>` to `Option<String>` — deserialization now captures the raw string, deferring validation.
2. After the full map is consumed (both `name` and `namespace` known), added normalization logic before constructing the `CommandName`:
   - Bare name (no dot) + non-empty namespace → prepend a dot to the name (namespace supplies the missing prefix) — the actual fix, enabling `name: list` / `namespace: .session`.
   - Bare name + empty namespace → unchanged (still rejected — out of scope per task's own In Scope wording: only the namespace-present path changes).
   - Dotted, multi-segment name (e.g. `.session.list`) + empty namespace → derive the namespace by splitting off the last segment, so Format 1's compact form and Format 2's `namespace`+bare-`name` form normalize to the identical (namespace, local-name) representation (required for FT-15's `command1.namespace() == command2.namespace()` assertion).
   - All other shapes (e.g. dotted single-segment name with an already-explicit namespace, as in the pre-existing `test_load_from_yaml_str_simple_command` test) — unchanged passthrough, identical to prior behavior.
3. Added 3-field `Fix(issue-005)` comment at the change site.

**Green State Confirmation (2026-07-06):**

- `cargo test --test registry -- registry::command_loader_yaml::test_ft15_yaml_format1_and_format2_produce_identical_command_name --exact --nocapture` → `test ... ok` (1 passed; 0 failed).
- `cargo test -p unilang --all-features --no-fail-fast` (via `rtk proxy` to rule out output-filtering artifacts) → **0 failures across all 18 test binaries** (`build`: 39/0, `registry`: 149/0 including FT-15, `validation`: 90/0 including the clippy meta-test from task 006). Confirmed via `git diff --stat` that only 3 files are modified in the entire working tree (this fix, plus task 006's 2 unrelated fixes) — no other file touched.
- Note: an earlier run of the `build` test target showed `helpers_type_analyzer::detects_integer_as_string` FAILED (unrelated build-time argument-type-hint heuristic, no relation to `CommandDefinition` deserialization); a clean re-run with no code changes in between passed (39/0), confirming this was pre-existing test flakiness, not a regression introduced by this fix — `git diff --stat` further confirms this fix never touches that test's file or its underlying analyzer module.

**Key Learnings:**

A validated newtype's `Deserialize` impl (`CommandName`, requiring a dot prefix at construction) is safe for a struct's own single-field deserialization, but becomes a validation-ordering hazard inside a *parent* struct's custom `Deserialize` when the parent needs cross-field context (here, `namespace`) to determine whether the child field's raw form is even valid yet. The fix pattern: defer the newtype's construction until after the full map is parsed, normalize the raw fields against each other first, then construct. This is a different subsystem and failure shape from task 006's clippy fixes, confirming the two were correctly filed as separate, unrelated tasks.

## History

*(append-only — newest entry last; never edit or remove past entries)*

- **2026-07-05** `FILED` — Discovered as a side effect of validating task 003 (unrelated semantic-analyzer empty-path fix) via an adversarial MAAV full-suite check. Filed as its own task, split from the clippy-lint defect (task 006) since the two share no root cause or subsystem. Not yet fixed; no production code touched as part of filing.
- **2026-07-06** `COMPLETED` — Preliminary root-cause guess (`command_validation.rs`/`validation_core.rs`) ruled out by error-text mismatch; actual root cause traced to `src/data/command_definition/serde_impl.rs`'s custom `Deserialize` impl, which validated the bare `name` field as a `CommandName` before `namespace` was available to combine with it. Fixed by deferring `CommandName` construction until after the full map is parsed, with 3-field fix comment. FT-15 passes, full suite green (0 failures across 18 test binaries). Validated via 3 independent MAAV subagents (Round 1, Full Round, CONVERGED) — 7/7 checklist items PASS; one non-blocking finding (imprecise rulebook citation) corrected in `## Requirements` above.

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
