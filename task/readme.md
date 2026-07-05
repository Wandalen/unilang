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
| 1 | 004 | 560 | 8 | 5 | 7 | 2 | 🎯 (Verified) | any | module | lib/yrd_core/unilang/dev/module/unilang | [Implement test surface spec coverage gaps](004_implement_test_surface_spec_coverage_gaps.md) | Implement Rust tests for 64 spec cases added across 15 feature/invariant/api/type spec files since task 002 |
| 2 | 003 | 2352 | 8 | 6 | 7 | 7 | ❓ (Unverified) | any | module | lib/yrd_core/unilang/dev/module/unilang | [Fix semantic analyzer empty-path bypasses named-argument validation](003_fix_semantic_analyzer_empty_path_bypasses_named_argument_validation.md) | Fix `analyze_internal()` silently returning help listing instead of validating named arguments when command_path_slices is empty |
| 3 | 002 | 0 | 8 | 5 | 7 | 0 | ✅ (Completed) | claude-sonnet-4-6 | module | lib/yrd_core/unilang/dev/module/unilang | [Implement test surface specs](completed/002_implement_test_surface_specs.md) | Implement Rust tests for all 121 spec cases across 17 feature/invariant/api/type spec files |
| 4 | 001 | 0 | 6 | 4 | 7 | 0 | ✅ (Completed) | claude-sonnet-4-6 | module | lib/yrd_core/unilang/dev/module/unilang | [Fix phf_map! codegen absolute path expansion](completed/001_fix_phf_map_codegen_absolute_path_expansion.md) | Fix codegen so downstream crates don't need direct phf dependency |

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
total_tasks: 4
completed: 2
active: 0
backlog: 0
last_updated: 2026-07-05
-->

## Task System Metadata

- **Last Updated:** 2026-07-05
- **Total Tasks:** 4
- **Completed:** 2
- **Active:** 0
- **Backlog:** 0
