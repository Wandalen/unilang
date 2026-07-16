# 008: Document or re-gate the undocumented `#[ignore]` network-dependency test

## Execution State

- **Executor Type:** any
- **filed_by:** user1@w002/home/user1/pro/lib/yrd_core/unilang/dev/
- **actor:** null
- **started_at:** null
- **expires_at:** null
- **round:** 1
- **state:** 📝 (Draft)
- **closes:** null
- **unit_type:** module
- **unit:** lib/yrd_core/unilang/dev/module/cargo_unilang
- **validated_by:** null
- **validation_date:** null
- **blocked_by:** null

## Goal

`module/cargo_unilang/tests/outdated_version_template_bug.rs:126` has a live `#[ignore]` on
`generated_project_dependencies_resolve()`, with only an inline `// Requires network access to
crates.io, disabled by default` comment — not the mandatory 5-field permission header
(`DISABLED:`/`REASON:`/`RE-ENABLE:`/`APPROVED:`/`TRACKING:`) required by
`l2_imp_organization.rulebook.md § Disabled Test Management : Step 2 - Document Permission`.

Discovered on 2026-07-16 while running `/tst_fix` (autonomous TDD loop) — the full `w3 .test l::3`
baseline passed clean (exit 0, zero real warnings, 4/4 crates) on the first run, so this doesn't
block the TDD loop's own Termination Condition; it surfaced from the Rust-tier disabled-test audit
commands (`l3_imp_organization.rulebook.md § Disabled Test Management : Permission Audit Commands`).

Not fixed inline for two reasons:
1. Backfilling the 5-field header would require the original disable date and approver, which
   aren't available without git history (this project's convention is to not use git, even
   read-only, for this kind of check) — fabricating those fields would violate the "never invent
   information" rule.
2. The architecturally-correct alternative — converting `#[ignore]` to an opt-in Cargo feature
   gate per `l3_imp_organization.rulebook.md § Test Infrastructure : Opt-In Feature Gating`
   ("must use opt-in features... `--all-features` enables every declared feature") — would make
   this test fire live network calls to crates.io during any `--all-features` run, including in
   sandboxed/offline CI. That's a real regression risk, not a strict improvement, and it's a
   design tradeoff worth a human decision rather than an autonomous change.

Needs a decision among remediation options before fixing:
- (a) backfill the 5-field header with `DISABLED: predates permission system` / `APPROVED: n/a
  (pre-existing)` and an explicit `RE-ENABLE: N/A — permanent, requires live network access`
  condition, keeping `#[ignore]` as the mechanism
- (b) convert to an opt-in feature (e.g. `network_tests`) per the Opt-In Feature Gating rule,
  accepting that `--all-features` runs (including this project's own `w3 .test l::3`) would then
  attempt live network access unless the test command is also changed to exclude that feature
  explicitly
- (c) leave as-is and add a narrower project-local rulebook exception documenting why this
  specific test class is exempt from the standard permission workflow (network-dependent,
  permanent-by-design tests aren't really "temporarily disabled pending a blocker" in the sense
  the workflow governs)

## History

- **[2026-07-16]** `FILED` — Task filed by user1@w002/home/user1/pro/lib/yrd_core/unilang/dev/
  during a `/tst_fix` run. Goal: resolve the undocumented `#[ignore]` permission-header gap on
  `generated_project_dependencies_resolve()` in `cargo_unilang`.
