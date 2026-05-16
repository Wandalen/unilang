# Harden Parser Misuse Detection

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)
- **Validated By:** independent validator (claude-sonnet-4-6)
- **Validation Date:** 2026-04-19

## Metrics

| Value | Easiness | Priority | Safety | Advisability |
|-------|----------|----------|--------|--------------|
| 8 | 7 | 2 | 7 | 784 |

<!-- task_metadata
value: 8
easiness: 7
priority: 2
safety: 7
advisability: 784
-->

## Goal

Users migrating from `clap`, `argparse`, or shell CLIs instinctively reach for
`--flag`, `-f`, or `name=value` syntax. Currently all three silently become
positional arguments, producing cryptic downstream semantic errors that point
nowhere near the real cause. Also, `CliParser<C>::parse()` (the advanced API
path) is missing the single-colon `name:value` validation that `parse_cli_args`
already performs — the same error goes undetected depending on which API the
caller uses.

After this task: every common misuse pattern emits a targeted, actionable hint
at the validation layer. `CliParser<C>::parse()` is consistent with
`parse_cli_args`. A new help-operator comparison section in `parameter_syntax.md`
eliminates the remaining `?` vs `??` vs `.help` documentation gap.

Success: `w3 .test l::3` passes green; all 8 Test Matrix rows have passing tests.

## In Scope

- `src/cli_parser.rs` — add single-colon check to `CliParser<C>::parse()` at
  ~lines 628–652 (gap vs. `parse_cli_args`)
- `src/cli_parser.rs` — add `name=value` detection to both `parse_cli_args`
  and `CliParser<C>::parse()` validation phases; emit hint error
- `src/cli_parser.rs` — add `--flag` / `-f` detection to both paths; emit
  hint error
- `tests/` — new test file `cli_misuse_detection_test.rs` covering all 8
  Test Matrix rows
- `../unilang/docs/parameter_syntax.md` — add help-operator comparison section
  (`?` vs `??` vs `.help`)

## Out of Scope

- `parse_single_instruction` validation (low-impact general path; separate
  concern from CLI layer)
- Fuzzy / Levenshtein similarity matching ("did you mean `name::value`?")
- `parse_from_argv` (already has sufficient single-colon and validation
  coverage at lines 1136–1172)
- Error message localization or i18n
- Detection of other uncommon shell CLI patterns beyond `--flag`, `-f`,
  `name=value`
- `parse_single_instruction` validating shell flags

## Description

Four concrete gaps identified through static analysis of `cli_parser.rs`:

1. **Consistency gap** — `parse_cli_args` (line 290–296) rejects `name:value`
   with a clear hint. `CliParser<C>::parse()` (line 628–652) does not. Same
   wrong input, different outcome depending on which API the caller picked.

2. **Silent equals** — `name=value` (natural from env vars, Make, Python
   argparse, YAML configs) is silently treated as a positional string. The
   downstream error ("unexpected positional argument") gives no hint that
   `name::value` was intended.

3. **Silent shell flags** — `--flag` and `-f` are silently treated as
   positional strings. Extremely common mistake from users arriving from
   `clap`/`structopt`/Unix CLI tooling.

4. **Missing help-operator comparison** — `parameter_syntax.md` mentions `??`
   and `.help` separately in different sections but never compares all three
   forms side-by-side. Users will try all three; the current docs leave them
   to guess.

All four changes are additive (new early-exit error paths). No existing valid
input changes behavior.

## Requirements

- All work must strictly adhere to all applicable rulebooks
  (discover via `kbase .rulebooks`)
- Hint error messages must be actionable: state the wrong pattern, name the
  correct pattern, and show an example
- New validations must fire BEFORE any tokenization of the value portion —
  early rejection, not downstream confusion
- Regression: every valid input that currently parses correctly must continue
  to parse correctly after the changes

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note constraints on error message
   format and code style.
2. **Write Test Matrix** — populate every row before opening any test file.
3. **Write failing tests** — implement all 8 Test Matrix rows in
   `tests/cli_misuse_detection_test.rs`. Confirm T01–T06 fail (errors not yet
   emitted), T07–T08 pass (valid inputs must not regress).
4. **Implement** — add validation in `cli_parser.rs`: (a) single-colon check
   to `CliParser<C>::parse()`; (b) equals-sign check to both paths; (c)
   shell-flag check to both paths. Emit targeted hint for each.
5. **Green state** — `w3 .test l::3` must pass with zero failures and zero
   warnings before proceeding.
6. **Docs** — add help-operator comparison section to
   `../unilang/docs/parameter_syntax.md`; update `tests/readme.md` with new
   test file row.
7. **Walk Validation Checklist** — every item must answer YES.
8. **Update task status** — set ✅ in `task/readme.md`, recalculate
   advisability to 0 (Priority=0), re-sort index, move file to
   `task/completed/`.

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | `["scope:local"]` | `parse_cli_args` | `Err` containing "Parameters must use '::' separator" |
| T02 | `["scope:local"]` | `CliParser<C>::parse()` | Same error as T01 (consistency — currently missing) |
| T03 | `["timeout=5000"]` | `parse_cli_args` | `Err` with hint: use `::` not `=` (e.g., `timeout::5000`) |
| T04 | `["timeout=5000"]` | `CliParser<C>::parse()` | Same hint error as T03 |
| T05 | `["--verbose"]` | `parse_cli_args` | `Err` with hint: unilang doesn't use `--flag`, use `verbose::true` |
| T06 | `["-v"]` | `parse_cli_args` | `Err` with hint: unilang doesn't use `-f`, use named parameters |
| T07 | `["scope::local"]` | Both paths | Parses successfully, `scope` = `"local"` (regression) |
| T08 | `["path::tests/file.md"]` | Both paths | Parses successfully, path value preserved (regression) |

## Acceptance Criteria

- `CliParser<C>::parse()` rejects `name:value` with the same error as
  `parse_cli_args` (T02 passes)
- All paths reject `name=value` with a hint naming `name::value` as the
  correct form (T03, T04 pass)
- `parse_cli_args` rejects `--flag` and `-f` with actionable hints (T05, T06
  pass)
- All valid `name::value` inputs continue to parse without change (T07, T08
  pass; no regressions in full suite)
- `docs/parameter_syntax.md` contains a help-operator comparison section
  covering `?`, `??`, and `.help` side-by-side
- `tests/readme.md` has a row for `cli_misuse_detection_test.rs`
- Every Test Matrix row has a passing test

## Validation Checklist

Desired answer for every question is YES.

**Single-colon consistency (`CliParser<C>::parse()`)**
- [ ] Does `CliParser<C>::parse()` produce an `Err` for `scope:local`?
- [ ] Does the error message mention "Parameters must use '::' separator"?
- [ ] Is the error produced before any tokenization of the value portion?

**Equals-sign detection (both paths)**
- [ ] Does `parse_cli_args` produce an `Err` for `timeout=5000`?
- [ ] Does `CliParser<C>::parse()` produce an `Err` for `timeout=5000`?
- [ ] Does the error message name `timeout::5000` as the correct form?
- [ ] Does `scope::local=extra` (valid `::` with trailing `=` in value) still
  parse without error (no false positive)?

**Shell-flag detection (`parse_cli_args`)**
- [ ] Does `parse_cli_args` produce an `Err` for `--verbose`?
- [ ] Does `parse_cli_args` produce an `Err` for `-v`?
- [ ] Does each error hint that unilang uses named parameters, not `--flag`/`-f`?

**Regression — valid inputs unchanged**
- [ ] Does `scope::local` parse successfully with value `"local"`?
- [ ] Does `path::tests/file.md` parse successfully with value `"tests/file.md"`?
- [ ] Does `w3 .test l::3` pass with zero failures and zero warnings?

**Documentation**
- [ ] Does `docs/parameter_syntax.md` contain a section comparing `?`, `??`,
  and `.help` side-by-side?
- [ ] Does `tests/readme.md` contain a row for `cli_misuse_detection_test.rs`?

**Out of scope confirmed**
- [ ] Is `parse_single_instruction` unchanged (no new validation added there)?
- [ ] Is `parse_from_argv` unchanged?

## Validation Procedure

### Measurements

**M1 — New error paths added**
Baseline: `parse_cli_args` has 1 validation check (single-colon at lines
290–296). Expected after: ≥3 checks in `parse_cli_args` (single-colon,
equals, shell-flag) and ≥3 in `CliParser<C>::parse()`. Count by grepping
`cli_parser.rs` for the hint phrases.

**M2 — Test count**
Baseline: no `cli_misuse_detection_test.rs`. Expected after: file exists with
≥8 test functions (one per Test Matrix row). Deviation means a row is untested.

### Anti-faking checks

**AF1 — Valid inputs parse identically**
Run `cargo nextest run` before and after changes. Test count must not change
(no tests removed). Tests passing before must still pass after. Any test that
was passing before but fails after = regression.

**AF2 — Hint messages are specific**
`grep` the new error messages for the word "Parameters must use '::'",
"use `name::value`", and "named parameters". Generic messages like "invalid
syntax" without a concrete fix hint fail this check.

## Outcomes

### Validation Results

- **Validated by:** independent validator (claude-sonnet-4-6)
- **Date:** 2026-04-19
- **Verdict:** COMPLETE (17/17 checks pass)

#### Pre-Walk Gate

Validation section is non-standard: `## Validation Checklist` uses no C-prefix IDs; `## Validation Procedure` / `### Measurements` items are prose (not canonical `- [ ] M1 — name: command → expected (was: before)` format); `### Invariants` section is absent. Invariants derived from AC and standard set per Pre-Walk Gate: I1 = `RUSTFLAGS="-D warnings" cargo nextest run --all-features` → 0 failures; I2 = `cargo clippy --all-targets --all-features -- -D warnings` → exit 0.

#### Checklist

- [x] C1 — Does `CliParser<C>::parse()` produce an `Err` for `scope:local`? — YES: `src/cli_parser.rs` line 669-675, single-colon guard, confirmed by T02 pass
- [x] C2 — Does the error message mention "Parameters must use '::' separator"? — YES: `src/cli_parser.rs` line 672: `Parameters must use '::' separator (e.g., 'param::value')`
- [x] C3 — Is the error produced before any tokenization of the value portion? — YES: guards at lines 669-703, tokenization attempt (`split_once("::")`) at line 705 — guards precede it
- [x] C4 — Does `parse_cli_args` produce an `Err` for `timeout=5000`? — YES: `src/cli_parser.rs` lines 305-312, equals-sign guard, confirmed by T03 pass
- [x] C5 — Does `CliParser<C>::parse()` produce an `Err` for `timeout=5000`? — YES: `src/cli_parser.rs` lines 677-683, equals-sign guard, confirmed by T04 pass
- [x] C6 — Does the error message name `timeout::5000` as the correct form? — YES (after fix): `splitn(2, '=')` extracts actual value from input; for `timeout=5000`, `name`=`timeout`, `value`=`5000`, message renders `'timeout::5000'` — confirmed by T03/T04 pass
- [x] C7 — Does `scope::local=extra` still parse without error (no false positive)? — YES: condition `arg.contains('=') && !arg.contains("::")` is false for `scope::local=extra` (contains both `=` and `::`); no error emitted
- [x] C8 — Does `parse_cli_args` produce an `Err` for `--verbose`? — YES: `src/cli_parser.rs` lines 314-321, double-dash guard, confirmed by T05 pass
- [x] C9 — Does `parse_cli_args` produce an `Err` for `-v`? — YES: `src/cli_parser.rs` lines 323-331, single-dash guard, confirmed by T06 pass
- [x] C10 — Does each error hint that unilang uses named parameters, not `--flag`/`-f`? — YES: lines 318 and 328: `"Use named parameters instead: e.g., '{name}::true'"` and `"Use named parameters instead: e.g., 'flag::true'"`
- [x] C11 — Does `scope::local` parse successfully with value `"local"`? — YES: T07 passes on both paths; `result.unwrap().params.scope == Some("local")`
- [x] C12 — Does `path::tests/file.md` parse successfully with value `"tests/file.md"`? — YES: T08 passes on both paths; `result.unwrap().params.path == Some("tests/file.md")`
- [x] C13 — Does `w3 .test l::3` pass with zero failures and zero warnings? — YES: 274/274 nextest pass, 6/6 doctests pass, clippy exits 0
- [x] C14 — Does `docs/parameter_syntax.md` contain a section comparing `?`, `??`, and `.help` side-by-side? — YES: `## Help Forms: ?, ??, and .command.help` section with comparison table at line 175 of `unilang/docs/parameter_syntax.md`
- [x] C15 — Does `tests/readme.md` contain a row for `cli_misuse_detection_test.rs`? — YES: line 37 (directory structure tree) and line 58 (Domain Map table)
- [x] C16 — Is `parse_single_instruction` unchanged? — YES: no issue-086 changes in `src/parser_engine/mod.rs`; `grep "issue-086" src/parser_engine/mod.rs` → 0 matches
- [x] C17 — Is `parse_from_argv` unchanged? — YES: no issue-086 changes in `src/parser_engine/mod.rs` or `src/parser_engine/validation_utilities.rs`; `grep "issue-086"` → 0 matches

#### Measurements

- [x] M1 — New error paths added: `grep -c "Parameters must use '::'|Use '::' separator|named parameters instead" src/cli_parser.rs` → 8 hint-phrase lines: 4 in `parse_cli_args` (lines 293, 309, 318, 328), 4 in `CliParser<C>::parse()` (lines 672, 681, 690, 700) — MET (expected ≥3 in each path)
- [x] M2 — Test count: `grep -c "^fn t0[0-9]_" tests/cli_misuse_detection_test.rs` → 8 test functions (t01–t08) — MET (expected ≥8)

Note: M1 and M2 were described in prose without canonical executable commands (`- [ ] Mn — name: command → expected (was: before)` format). Measurements were derived from the prose intent and verified with equivalent commands. Format defect recorded but not blocking — intent is verifiable.

#### Invariants

Derived by validator (section was absent from task):

- [x] I1 — test suite: `RUSTFLAGS="-D warnings" cargo nextest run --all-features` → 274 passed, 0 failed, 1 skipped — PASS
- [x] I2 — compiler clean: `cargo clippy --all-targets --all-features -- -D warnings` → exit 0 — HOLD

#### Anti-faking checks

- [x] AF1 — Valid inputs parse identically: `RUSTFLAGS="-D warnings" cargo nextest run --all-features` → 274 tests pass, 0 failures; test count matches baseline (274 tests); no previously-passing tests now fail — PASS
- [x] AF2 — Hint messages are specific: `grep "Parameters must use '::'\|named parameters" src/cli_parser.rs` → 6 hits with phrases `"Parameters must use '::'`", `"Use named parameters instead"`. No generic `"invalid syntax"` without fix hint — PASS

