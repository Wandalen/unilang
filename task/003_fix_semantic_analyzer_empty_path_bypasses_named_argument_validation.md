# Fix `SemanticAnalyzer::analyze_internal` silently bypassing named-argument validation when `command_path_slices` is empty (issue-003)

## Execution State

- **Executor Type:** any
- **Actor:** unclaimed
- **Claimed At:** —
- **Status:** ❓ (Unverified)
- **Validated By:** —
- **Validation Date:** —

## Goal

`analyze_internal()` in `unilang/src/semantic/core.rs:319-321` unconditionally returns `self.generate_help_listing()` whenever `instruction.command_path_slices.is_empty()` is true — before `bind_arguments` (and therefore `check_unknown_named_arguments`) ever runs (Motivated: a downstream consumer filed a 2-stage bug report claiming both the parser and the semantic analyzer silently accept unknown named parameters; the parser-side defect is already fixed in this repo, but the semantic-analyzer side remains live and is the sole remaining cause of the observable defect — any CLI or embedder built on unilang that relies on `check_unknown_named_arguments` for input validation will silently accept typo'd/invalid named parameters whenever the resolved command path is empty, instead of rejecting them; Observable: a `bug_reproducer(issue-003)` test in `unilang/tests/semantic/empty_path_named_argument.rs` (already written and confirmed FAILING against current code, see `## Outcomes` below) parses `. some_unknown_param::xyz` and asserts semantic analysis rejects the unknown parameter — it currently returns `Err(HelpRequested)` with a full command listing instead; Scoped: one conditional branch in `analyze_internal()` (`unilang/src/semantic/core.rs:319-321`), no parser changes, no changes to intentional bare-dot help behavior (`. ` with zero arguments must still show help); Testable: `cargo test -p unilang --test semantic` passes with zero failures and zero warnings after the fix, including the new reproducer test).

## In Scope

- Fix `analyze_internal()` (`unilang/src/semantic/core.rs:319-321`) so that the empty-`command_path_slices` early return only fires when there are also no named arguments attached — i.e., true bare-dot help (`.` alone, or `.` with no arguments) must still short-circuit to `generate_help_listing()`, but `. some_unknown_param::xyz` must instead run argument validation (`bind_arguments` / `check_unknown_named_arguments`) and surface an `UnknownParameter`-style error
- 3-field source comment (`Fix(issue-003)`, `Root cause`, `Pitfall`) on the fixed lines in `core.rs`
- 5-section test documentation (`Root Cause`, `Why Not Caught`, `Fix Applied`, `Prevention`, `Pitfall`) added to `unilang/tests/semantic/empty_path_named_argument.rs` (currently has bug-context doc comments but not yet the full 5-section post-fix block, since the fix has not landed yet)
- Confirm no regression to the genuine bare-dot help path (empty path AND empty named/positional arguments must still return help listing)

## Out of Scope

- The parser-side fix — already complete and verified in this repo (`unilang_parser/src/parser_engine/mod.rs:526-563`, tagged `Fix(issue-cmd-path)`); do not modify parser code as part of this task
- Any changes to `generate_help_listing()` itself, or to the `HelpGenerator` / help-content formatting
- Any changes to the `??` / help-operator detection logic (`core.rs:332-351`) — orthogonal to this defect
- Modifying or investigating the downstream consumer repo's test (`assistant::commands help_unknown_named_parameter_rejected`) — that is a separate repo's regression test that is expected to start passing once this fix lands here, but fixing it directly is out of scope for this task
- Any change to `command_path_slices` population logic itself — the parser's decision about what belongs in the path vs. what belongs in named_arguments is correct and untouched

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Minimum rulebook references: `code_design.rulebook.md`, `codebase_hygiene.rulebook.md`, `test_organization.rulebook.md`, `code_style.rulebook.md`
- Custom codestyle per `code_style.rulebook.md` — 2-space indents, no `cargo fmt`
- Tests in `unilang/tests/` — no `#[cfg(test)]` in `src/`
- No mocking — test the real `SemanticAnalyzer::analyze()` return value against a real `CommandRegistry`
- Bug reproducer test already written and confirmed failing (see `## Outcomes`) — this satisfies the "written and confirmed failing BEFORE any production code change" requirement; do not delete or weaken it when implementing the fix

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note code style, test organization, and fix documentation constraints
2. **Read current code** — `unilang/src/semantic/core.rs:309-361` (`analyze_internal`), `unilang/src/semantic/argument_binding.rs:22-55` (`bind_arguments`), `unilang/src/semantic/validation.rs:171-` (`check_unknown_named_arguments`) to internalize the full call chain that the empty-path early return currently skips
3. **Confirm reproducer still fails** — run `cargo test -p unilang --test semantic empty_command_path_with_unknown_named_argument_should_error -- --nocapture`; confirm it fails with the exact `HelpRequested` / help-listing output documented in `## Outcomes` below (this proves the bug is still present before starting the fix)
4. **Design the corrected condition** — the early return at `core.rs:319-321` must become conditional on there being no arguments at all attached to the instruction (not just an empty command path); check both `instruction.named_arguments.is_empty()` and (for symmetry/completeness) `instruction.positional_arguments.is_empty()` per the generalized invariant in `## Technical Context`
5. **Implement fix** — apply the corrected condition to `analyze_internal()`; add 3-field fix comment (`Fix(issue-003)`, `Root cause`, `Pitfall`) at `core.rs:319-321`
6. **Confirm test passes** — run `cargo test -p unilang --test semantic empty_command_path_with_unknown_named_argument_should_error`; test MUST pass
7. **Confirm bare-dot help still works** — add/run a control assertion (or confirm an existing test covers) that a literal `.` with zero arguments still returns the help listing unchanged
8. **Green state** — `cargo test -p unilang --all-features` passes with zero failures and zero warnings
9. **Add test documentation** — add the remaining 4 sections (`Why Not Caught`, `Fix Applied`, `Prevention`, `Pitfall` — `Root Cause` already present) to `unilang/tests/semantic/empty_path_named_argument.rs` per `test_organization.rulebook.md`
10. **Walk Validation** — walk every item in `## Validation`; attach evidence to `## Outcomes`
11. **Update task status** — set ✅ in `task/readme.md`, move file to `task/completed/`

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | `. some_unknown_param::xyz` on unmodified code | registry with `.test` registered, no `some_unknown_param` argument on any command | Analysis returns `Err(HelpRequested)` with full command listing — test FAILS (red state, already proven, see Outcomes) |
| T02 | `. some_unknown_param::xyz` after fix | same registry | Analysis returns `Err` mentioning "Unknown parameter" / `some_unknown_param` — NOT a help listing |
| T03 | `.` (bare dot, zero arguments) after fix | any registry | Analysis returns `Err(HelpRequested)` with full command listing — unchanged, genuine help behavior preserved |
| T04 | `. dry::true` where `dry` IS a valid global/root argument (if such a construct exists) after fix | registry supporting root-level arguments | Analysis proceeds to validate `dry` normally, does not short-circuit to help |
| T05 | `cargo test -p unilang --all-features` | after fix | 0 failures, 0 warnings |

## Acceptance Criteria

**Bug-Fixing Quality Requirements (all 7 must be satisfied):**

1. **Rulebook compliance** — task work references and adheres to `code_design`, `codebase_hygiene`, `test_organization`, and `code_style` rulebooks; no exceptions
2. **Test-first** — `bug_reproducer(issue-003)` test is written and confirmed failing BEFORE any production code change; evidence of failure is captured in `## Outcomes` (already satisfied — see below)
3. **Evidence of failure** — `## Outcomes` contains the exact `cargo test` output (panic message with actual vs. expected) proving the reproducer test failed on unmodified code
4. **Proper fix** — no mocking; fix addresses root cause (unconditional early return not checking for attached arguments); 3-field source comment (`Fix(issue-003)`, `Root cause`, `Pitfall`) present in `core.rs`
5. **Fix validation** — reproducer test confirmed failing without fix, confirmed passing with fix; `cargo test -p unilang --all-features` passes with zero failures after fix
6. **Knowledge preservation** — test file has 5-section documentation (`Root Cause`, `Why Not Caught`, `Fix Applied`, `Prevention`, `Pitfall`) per STATC quality standard (Specific/Technical/Actionable/Traceable/Concise); source has 3-field fix comment
7. **Code cleanliness** — no TODO/FIXME markers, no commented-out implementations, no code duplication; no `#[cfg(test)]` in `src/`

**Functional Acceptance Criteria:**

- For every instruction where `command_path_slices.is_empty()` is true, if `named_arguments` is non-empty (or `positional_arguments` is non-empty), argument validation must still run before falling back to help-listing behavior
- Genuine bare-dot help (`.` with zero arguments of any kind) continues to return the help listing unchanged
- All existing `cargo test -p unilang --all-features` tests pass after the fix
- No change to parser behavior, command registration, or help-content formatting

## Validation

**Execution:** Independent validator (not executor) walks this section after SUBMIT (⏳ → 🔍) per MAAV (`governance/maav.rulebook.md`) — dispatch independent subagents with at least one adversarial mandate; do not self-verify.

### Checklist

Desired answer for every question is YES.

**Bug Reproducer**
- [ ] C1 — Does `unilang/tests/semantic/empty_path_named_argument.rs` exist?
- [ ] C2 — Does the test file contain a `bug_reproducer(issue-003)` marker? (already present as of this filing, on the test function's doc comment)
- [ ] C3 — Is there evidence in `## Outcomes` that the test FAILED on unmodified code (exact panic/assertion output)?

**Fix Correctness**
- [ ] C4 — Does `analyze_internal()` reject `. some_unknown_param::xyz` with an unknown-parameter error rather than a help listing?
- [ ] C5 — Does a bare `.` with zero arguments still return the help listing unchanged (no regression to intentional help behavior)?
- [ ] C6 — Is the 3-field fix comment (`Fix(issue-003)`, `Root cause`, `Pitfall`) present at the changed lines in `core.rs`?

**Test Documentation**
- [ ] C7 — Does the test file contain a doc comment with all 5 sections: `Root Cause`, `Why Not Caught`, `Fix Applied`, `Prevention`, `Pitfall`?
- [ ] C8 — Does the documentation meet STATC quality (Specific/Technical/Actionable/Traceable/Concise — not generic "fixed bug" or "be careful")?

**Out of Scope Confirmation**
- [ ] C9 — Is `unilang_parser/src/parser_engine/mod.rs` NOT modified (parser-side fix already complete)?
- [ ] C10 — Is `generate_help_listing()` / `HelpGenerator` NOT modified (help-content formatting unchanged)?

### Measurements

- [ ] M1 — reproducer test result before fix: `cargo test -p unilang --test semantic empty_command_path_with_unknown_named_argument_should_error 2>&1 | grep -E '^test result'` → FAILED (captured in Outcomes, already done)
- [ ] M2 — reproducer test result after fix: same command → ok, 1 passed
- [ ] M3 — full suite: `cargo test -p unilang --all-features 2>&1 | grep -E '^test result'` → all tests passed
- [ ] M4 — warnings: `RUSTFLAGS="-D warnings" cargo check -p unilang --all-features 2>&1 | grep -c 'warning'` → 0

### Invariants

- [ ] I1 — test suite: `cargo test -p unilang --all-features` → 0 failures
- [ ] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check -p unilang --all-features` → 0 warnings
- [ ] I3 — no cfg(test) in src: `grep -r '#\[cfg(test)\]' unilang/src/` → no matches

### Anti-faking checks

- [ ] AF1 — reproducer test is substantive: test body asserts on the actual `SemanticAnalyzer::analyze()` return value and its error content (NOT `assert!(true)` or a vacuous check)
- [ ] AF2 — test genuinely validates content: assertion checks for `"Unknown parameter"` or the literal unknown parameter name in the error, not merely `result.is_err()` (which the buggy code already satisfies via `HelpRequested`)
- [ ] AF3 — fix is in `core.rs`, not the test: the corrected condition lives in `analyze_internal()`, not suppressed/worked-around in the test file

## Outcomes

*(Executor fills remaining fields during execution. Reproducer red-state evidence below was captured during this filing task, prior to any fix.)*

**Red State Evidence (captured 2026-07-04, MRE run directly against unilang's public API, no downstream repo involved):**

Test file: `unilang/tests/semantic/empty_path_named_argument.rs`, registered in `unilang/tests/semantic.rs`.

Command run:
```
cd module/unilang && cargo test --test semantic empty_command_path_with_unknown_named_argument_should_error -- --nocapture
```

Actual captured output (re-confirmed after adding the `bug_reproducer(issue-003)` doc-comment tag, which shifted the panic line by +2 with no change in behavior):
```
thread 'semantic::empty_path_named_argument::test_empty_command_path_with_unknown_named_argument_should_error' panicked at module/unilang/tests/semantic/empty_path_named_argument.rs:89:3:
Error should identify 'some_unknown_param' as an unknown/unvalidated parameter (validation should have run before any help-listing fallback), got: Execution(ErrorData { code: HelpRequested, message: "Available commands:\n\n  .test                Test command used to prove registry is non-empty\nUse '<command> help' to get detailed help for a specific command.\nExample: . .list help\n", source: None })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass `--test semantic`

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 93 filtered out; finished in 0.00s
```

This confirms: the instruction parsed from `. some_unknown_param::xyz` has `command_path_slices` empty and `named_arguments` containing `some_unknown_param` (both precondition assertions in the test passed silently, proving the already-fixed parser behaves exactly as expected — see `## Technical Context`); `analyze()` DID return `Err` (the `result.is_err()` assertion earlier in the test also passed silently) — but it is the generic `HelpRequested` / full command-listing error, not an unknown-parameter validation error. `bind_arguments` / `check_unknown_named_arguments` never ran.

Full regression run confirming no side effects from adding this reproducer:
```
cd module/unilang && cargo test --test semantic
test result: FAILED. 93 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```
All 93 pre-existing semantic tests pass unchanged; only the new reproducer fails, exactly as expected for a red-state MRE.

**Fix Applied:** *(pending — fill in during execution)*

**Green State Confirmation:** *(pending — fill in during execution)*

**Key Learnings:** *(pending — fill in during execution)*

## Technical Context

### Root Cause

Originally reported by a downstream consumer as a 2-stage defect spanning both `unilang_parser` and `unilang`. Verified against current code in this repo (2026-07-04):

**Stage 1 (parser) — ALREADY FIXED, not part of this task.** `parse_command_path()` in `unilang_parser/src/parser_engine/mod.rs` now performs an explicit lookahead (added under `Fix(issue-cmd-path)`, comment block starting at line 526) before consuming an identifier into `command_path_slices`: if the identifier is immediately followed by a `::` / ` :: ` operator token, the parser recognizes it as the NAME half of a `name::value` named-argument pattern and `break`s out of the command-path loop without consuming it (line 563), instead of incorrectly appending it to the path. This means input like `. some_unknown_param::xyz` now correctly produces `command_path_slices: []` and `named_arguments: {"some_unknown_param": "xyz"}` — verified directly in the MRE test's precondition assertions, which pass.

**Stage 2 (semantic analyzer) — STILL LIVE, this task's target.** `analyze_internal()` in `unilang/src/semantic/core.rs:319-321`:
```rust
if instruction.command_path_slices.is_empty()
{
  return self.generate_help_listing();
}
```
This check only inspects `command_path_slices`. It does not check whether `instruction.named_arguments` (or `instruction.positional_arguments`) is non-empty. As a direct consequence of Stage 1's fix, an instruction can now legitimately have an empty `command_path_slices` while still carrying a non-empty `named_arguments` map — and this unconditional early return fires regardless, returning straight to `generate_help_listing()` at line 321. `Self::bind_arguments(instruction, &command_def)` — the call that would eventually invoke `check_unknown_named_arguments` (`unilang/src/semantic/argument_binding.rs:45`, itself defined at `unilang/src/semantic/validation.rs:171`) — lives at `core.rs:353`, entirely unreachable once the line 319 condition is true. No validation of the attached named argument ever occurs; the analyzer instead silently returns a generic command listing as if the user had typed a bare `.`.

**Generalized/detection-invariant statement:** for every instruction where `command_path_slices.is_empty()` is true, if `named_arguments` is non-empty, argument validation must still run before falling back to help-listing behavior. (The same reasoning likely extends to non-empty `positional_arguments`, though no reported case currently exercises that path — included in Test Matrix T04 for completeness during fix design.)

### Impact

Any CLI or embedder built on unilang that relies on `check_unknown_named_arguments` for input validation will silently accept typo'd or entirely invalid named parameters whenever the resolved command path is empty — instead of rejecting them with a helpful "Unknown parameter" error (with Levenshtein-based suggestions, per the existing behavior in `unilang/tests/semantic/unknown_parameters.rs`). The user instead receives an unrelated full command listing, with no indication that their input was malformed.

### Known Downstream Impact

This defect is the direct cause of a currently-failing regression test in a consumer crate: `assistant::commands help_unknown_named_parameter_rejected` (separate repo, not part of this workspace). That test is expected to start passing once this task's fix lands here — no further action on the downstream repo is in scope for this task.

### Reproduction Steps (Automated, already committed as a test)

```bash
cd module/unilang
cargo test --test semantic empty_command_path_with_unknown_named_argument_should_error -- --nocapture
```

See `## Outcomes` above for the exact captured failure output.

## Cross-References

- **Live defect:** `unilang/src/semantic/core.rs:319-321` (function `analyze_internal`)
- **Unreachable validation call:** `unilang/src/semantic/core.rs:353` (`Self::bind_arguments`) → `unilang/src/semantic/argument_binding.rs:45` (`Self::check_unknown_named_arguments`) → defined at `unilang/src/semantic/validation.rs:171`
- **Already-fixed parser side (reference only, do not modify):** `unilang_parser/src/parser_engine/mod.rs:526-563` (`Fix(issue-cmd-path)`)
- **Reproducer test:** `unilang/tests/semantic/empty_path_named_argument.rs`, registered in `unilang/tests/semantic.rs`
- **Related existing coverage (no empty-path case, confirmed no overlap):** `unilang/tests/semantic/unknown_parameters.rs`, `unilang/tests/semantic/unknown_parameters_edge_cases.rs`
- **Downstream consumer regression (context only, separate repo):** `assistant::commands help_unknown_named_parameter_rejected`
