# Implement build-runtime separation tests (invariant/06)

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)
- **Validated By:** MAAV (spec-compliance agent + adversarial agent)
- **Validation Date:** 2026-06-11
- **Closes:** null

## Goal

Implement Rust test cases for the 4 IN-cases defined in `tests/docs/invariant/06_build_runtime_separation.md` so that the spec status changes from ⏳ to ✅, verified by `w3 .test level::3` passing with zero failures. (Motivated: invariant/006 was created during the normalization session but has 0% test coverage — the build-runtime separation boundary is untested and could silently regress; Observable: 4 new test functions appear in `tests/build/` that exercise cargo tree inspection, static data access, and validation_core identity; Scoped: implements IN-1..IN-4 from one spec file — no new features, no refactoring, no other spec files; Testable: `w3 .test level::3` exits 0 with all 4 tests passing and `tests/docs/invariant/readme.md` shows ✅ for `06_build_runtime_separation.md`.)

## Null Hypothesis

"The existing test suite already enforces the build-runtime separation invariant, making these tests redundant."

**Refuted:** No existing test verifies that `serde_yaml` or `serde_json` are absent from the runtime dependency tree. No test confirms `validation_core.rs` produces identical results in both build and runtime contexts. The invariant could silently break (e.g., by adding a non-optional serde_yaml dep) without any test failing.

## In Scope

- `tests/build/build_runtime_separation.rs` — New test file with 4 test functions:
  - `test_in1_runtime_deps_exclude_serde_yaml` — runs `cargo tree -p unilang --no-default-features --features enabled --edges=normal` and asserts stdout does NOT contain `serde_yaml_ng` (absent from runtime dep tree when only `enabled` feature is active)
  - `test_in2_static_data_accessible_without_parsing` — accesses `StaticCommandDefinition` fields at runtime under `#[cfg(feature = "static_registry")]`, confirming they are compile-time constants requiring no parsing
  - `test_in3_validation_core_identity` — calls `validate_command_name_core` from the runtime module path and compares results against known inputs (valid `.cmd`, invalid `nodot`) to confirm the shared logic is accessible
  - `test_in4_runtime_deps_exclude_serde_json` — runs `cargo tree -p unilang --no-default-features --features enabled --edges=normal` and asserts stdout does NOT contain `serde_json` (absent from runtime dep tree when only `enabled` feature is active)
- Update `tests/docs/invariant/readme.md` status for `06_build_runtime_separation.md` from ⏳ to ✅
- Register new test file in `tests/build/readme.md` Responsibility Table

## Out of Scope

- Modifying invariant/006 doc instance content (already consistent)
- Modifying `src/validation_core.rs` or `build/main.rs` (tests exercise existing behavior)
- Other spec files (feature/01, feature/05, invariant/02, invariant/03 gaps are tracked by workspace task 002)
- PHF implementation details

## Requirements

- All work must strictly adhere to all applicable rulebooks
- Tests must use real implementations — no mocking, no `assert!(true)`, no `#[ignore]`
- Each test function must have a doc comment citing the spec case it implements
- IN-1 and IN-4 (cargo tree tests) must use `--no-default-features --features enabled` to isolate the `enabled` feature without default features (which include `approach_yaml_multi_build` pulling in `yaml_parser`). Inspect stdout content for package name absence — do NOT rely on exit code (cargo tree exits 0 even when a `-i` target is absent from the tree)
- IN-2 (static data) must be gated with `#[cfg(feature = "static_registry")]` since `StaticCommandDefinition` requires that feature

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read spec** — Read `tests/docs/invariant/06_build_runtime_separation.md` for the 4 Given/When/Then cases
2. **Read source** — Read `src/validation_core.rs`, `src/static_data.rs`, and `build/main.rs` to understand the include!() pattern and static data access
3. **Create test file** — Write `tests/build/build_runtime_separation.rs` with 4 test functions following the IN-1..IN-4 spec
4. **Register in entry point** — Add `mod build_runtime_separation;` inside the `mod build { }` block in `tests/build.rs`
5. **Register in readme** — Add row to `tests/build/readme.md` Responsibility Table
6. **Green state** — `w3 .test level::3` must pass with zero failures
7. **Update spec status** — Change `06_build_runtime_separation.md` row in `tests/docs/invariant/readme.md` from ⏳ to ✅

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | `cargo tree -p unilang --no-default-features --features enabled --edges=normal` | Runtime dep tree with `enabled` only | stdout does NOT contain `serde_yaml_ng` |
| T02 | Access `StaticCommandDefinition` fields at runtime | `src/static_data.rs` types + `#[cfg(feature = "static_registry")]` | Fields accessible as constants; no parsing call needed |
| T03 | `validate_command_name_core(".valid")` and `validate_command_name_core("invalid")` | `validation_core.rs` shared logic | Ok for ".valid", Err for "invalid"; same results as build-time would produce |
| T04 | `cargo tree -p unilang --no-default-features --features enabled --edges=normal` | Runtime dep tree with `enabled` only | stdout does NOT contain `serde_json` |

## Acceptance Criteria

- 4 test functions exist in `tests/build/build_runtime_separation.rs`, each citing its IN-N case
- Cargo tree tests (IN-1, IN-4) use `--no-default-features --features enabled` and inspect stdout content (not exit code)
- `w3 .test level::3` exits with 0 failures
- `tests/docs/invariant/readme.md` shows ✅ for `06_build_runtime_separation.md`
- No test contains `assert!(true)`, `unimplemented!()`, `todo!()`, or `#[ignore]`

## Validation

### Checklist

Desired answer for every question is YES.

- [x] C1 — Do 4 test functions exist in `tests/build/build_runtime_separation.rs`?
- [x] C2 — Does each test cite its IN-N spec case in a doc comment?
- [x] C3 — Do cargo tree tests use `--no-default-features --features enabled` and inspect stdout (not exit code)?
- [x] C4 — Is `tests/docs/invariant/readme.md` status ✅ for `06_build_runtime_separation.md`?
- [x] C5 — Is `tests/build/readme.md` updated with the new file?
- [x] C6 — Does `w3 .test level::3` pass with zero failures?

### Invariants

- [x] I1 — `w3 .test level::3` → 0 failures
- [x] I2 — No new `optional = true` deps added to `Cargo.toml`

### Anti-faking checks

- [x] AF1 — `grep -rn "assert!(true)" tests/build/build_runtime_separation.rs` → 0 matches
- [x] AF2 — Cargo tree tests actually invoke `cargo tree` (not hard-coded results)

## Related Documentation

- `/home/user1/pro/lib/wip_core/unilang/dev/module/unilang/docs/invariant/006_build_runtime_separation.md` — Invariant definition (created during normalization)
- `/home/user1/pro/lib/wip_core/unilang/dev/module/unilang/tests/docs/invariant/06_build_runtime_separation.md` — Test surface spec (4 TCs)
- `/home/user1/pro/lib/wip_core/unilang/dev/task/002_implement_test_surface_specs.md` — Workspace-level task covering all 17 specs (Related: 002)

## History

- **[2026-06-11]** `CREATED` — Implement 4 test cases for new invariant/006 (build-runtime separation) doc instance created during normalization session.
- **[2026-06-11]** `VERIFY-1` — MAAV gate: Scope Coherence PASS, MOST Goal PASS, Value/YAGNI PASS, Implementation Readiness FAIL (5 findings). Fixed: (1) cargo tree stdout inspection instead of exit code, (2) `--no-default-features --features enabled` instead of `--features enabled`, (3) `#[cfg(feature = "static_registry")]` for T02, (4) entry point inside `mod build {}`, (5) updated test matrix and acceptance criteria.
- **[2026-06-11]** `VERIFY-2` — MAAV re-verification: Implementation Readiness PASS. All 5 findings confirmed fixed. Task promoted to 🎯 Verified.
- **[2026-06-11]** `COMPLETED` — 4 tests implemented in `tests/build/build_runtime_separation.rs`, registered in `tests/build.rs` mod entry point and `tests/build/readme.md`. `w3 .test level::3` passes (nextest ✅ doc tests ✅ clippy ✅). Spec status updated to ✅ in `tests/docs/invariant/readme.md`. MAAV: spec-compliance agent + adversarial agent both PASS.

## Verification Record

- **Date:** 2026-06-11
- **Scope Coherence:** PASS — In Scope lists 3 concrete artifacts; Out of Scope excludes 4 domains; observable outcome unambiguous
- **MOST Goal Quality:** PASS — Motivated (0% coverage regression risk), Observable (4 named tests + readme status), Scoped (one spec file), Testable (w3 .test level::3 binary criterion)
- **Value/YAGNI:** PASS — No existing test checks cargo tree for parser absence; Null Hypothesis genuinely refuted; no speculative work
- **Implementation Readiness:** PASS (after fix) — stdout content inspection (not exit code), `--no-default-features --features enabled --edges=normal` isolates correctly, `#[cfg(feature = "static_registry")]` gate for T02, entry point inside `mod build {}`
