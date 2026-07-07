# Task Management

Task tracking for the unilang crate.

## File Responsibility Table

| Entry | Responsibility |
|-------|---------------|
| `completed/` | Tasks that passed full validation |
| `decisions/` | Decision records referenced by tasks' `closes` field |

---

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | State | Executor | UnitType | Unit | Task | Purpose |
|-------|----|--------------|-------|----------|--------|----------|-------|----------|----------|------|------|---------|
| 1 | 005 | 0 | 7 | 6 | 7 | 0 | ✅ (Completed) | any | module | lib/yrd_core/unilang/dev/module/unilang | [Fix YAML loader namespace+name combination validated before combining](completed/005_fix_yaml_loader_namespace_name_combination_validated_before_combining.md) | Fix YAML command loader rejecting valid `namespace`+bare-`name` combination as if the bare name itself must be dot-prefixed |
| 2 | 006 | 0 | 4 | 9 | 9 | 0 | ✅ (Completed) | any | module | lib/yrd_core/unilang/dev/module/unilang | [Fix clippy lint violations blocking validation_v6](completed/006_fix_clippy_lint_violations_blocking_validation_v6.md) | Fix 2 clippy lint violations in unrelated test files blocking the validation_v6_clippy_passes meta-test |
| 3 | 004 | 560 | 8 | 5 | 7 | 2 | 🎯 (Verified) | any | module | lib/yrd_core/unilang/dev/module/unilang | [Implement test surface spec coverage gaps](004_implement_test_surface_spec_coverage_gaps.md) | Implement Rust tests for 64 spec cases added across 15 feature/invariant/api/type spec files since task 002 |
| 4 | 002 | 0 | 8 | 5 | 7 | 0 | ✅ (Completed) | claude-sonnet-4-6 | module | lib/yrd_core/unilang/dev/module/unilang | [Implement test surface specs](completed/002_implement_test_surface_specs.md) | Implement Rust tests for all 121 spec cases across 17 feature/invariant/api/type spec files |
| 5 | 001 | 0 | 6 | 4 | 7 | 0 | ✅ (Completed) | claude-sonnet-4-6 | module | lib/yrd_core/unilang/dev/module/unilang | [Fix phf_map! codegen absolute path expansion](completed/001_fix_phf_map_codegen_absolute_path_expansion.md) | Fix codegen so downstream crates don't need direct phf dependency |
| 6 | 003 | 2352 | 8 | 6 | 7 | 7 | ✅ (Completed) | any | module | lib/yrd_core/unilang/dev/module/unilang | [Fix semantic analyzer empty-path bypasses named-argument validation](completed/003_fix_semantic_analyzer_empty_path_bypasses_named_argument_validation.md) | Fix `analyze_internal()` silently returning help listing instead of validating named arguments when command_path_slices is empty — MAAV-validated, 18/20 checklist items direct pass, 2 resolved via scope-adjustment (see tasks 005/006) |

---

## Issues Index

| ID | Status | Task ID | Title |
|----|--------|---------|-------|

---

## Issues

*No issues recorded.*

---

<!-- task_system_metadata
type: local
total_tasks: 6
completed: 5
active: 0
backlog: 0
last_updated: 2026-07-06
-->

## Task System Metadata

- **Last Updated:** 2026-07-06
- **Total Tasks:** 6
- **Completed:** 5
- **Active:** 0
- **Backlog:** 0
