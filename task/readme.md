# Task Management

Task tracking for the unilang crate.

## File Responsibility Table

| Entry | Responsibility |
|-------|---------------|
| `backlog/` | Tasks not yet prioritized — reviewed, metrics assigned, pending promotion |
| `completed/` | Tasks that passed full validation |
| `cancelled/` | Tasks stopped or abandoned with documented reason |

---

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------|-------|----------|--------|----------|--------|----------|------|---------|
| 1 | 003 | 7 | 8 | 6 | 7 | 7 | ❓ | unclaimed | [Fix semantic analyzer empty-path bypasses named-argument validation](003_fix_semantic_analyzer_empty_path_bypasses_named_argument_validation.md) | Fix `analyze_internal()` silently returning help listing instead of validating named arguments when command_path_slices is empty |
| 2 | 002 | 0 | 8 | 5 | 7 | 0 | ✅ | claude-sonnet-4-6 | [Implement test surface specs](completed/002_implement_test_surface_specs.md) | Implement Rust tests for all 121 spec cases across 17 feature/invariant/api/type spec files |
| 3 | 001 | 0 | 6 | 4 | 7 | 0 | ✅ | claude-sonnet-4-6 | [Fix phf_map! codegen absolute path expansion](completed/001_fix_phf_map_codegen_absolute_path_expansion.md) | Fix codegen so downstream crates don't need direct phf dependency |

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
total_tasks: 3
completed: 2
active: 0
backlog: 0
last_updated: 2026-07-04
-->

## Task System Metadata

- **Last Updated:** 2026-07-04
- **Total Tasks:** 3
- **Completed:** 2
- **Active:** 0
- **Backlog:** 0
