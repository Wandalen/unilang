# Task Procedures

Operational procedures for the task lifecycle. Tasks use 3-digit Unified IDs and emoji lifecycle stages.

## Procedure: Create Task

**Trigger:** A new unit of work is identified.

1. Choose the next available 3-digit ID (scan `completed/`, `unverified/`, `cancelled/` for the highest used ID, increment by 1).
2. Create `unverified/NNN_<slug>.md` where `<slug>` is a lowercase_snake_case summary.
3. Populate the task file with: ID, title, executor, state `❓Unverified`, purpose, acceptance criteria, and any known subtasks.
4. Add a row to the Tasks Index in `readme.md`.

## Procedure: Verify Task

**Trigger:** A task in `unverified/` has been reviewed and is ready for execution.

1. Open the task file in `unverified/`.
2. Confirm acceptance criteria are measurable and the executor is named.
3. Update the state field to `🎯Verified`.
4. Move the file to the root `task/` directory (or leave in `unverified/` if the executor acts immediately).
5. Update the state column in `readme.md` Tasks Index.

## Procedure: Close Task

**Trigger:** All acceptance criteria for a task are met.

1. Perform final verification that every acceptance criterion is satisfied.
2. Update the state field to `✅Completed`.
3. Move the file to `completed/NNN_<slug>.md`.
4. Update the state column in `readme.md` Tasks Index to `✅ (Completed)`.

## Procedure: Cancel Task

**Trigger:** A task is no longer relevant or has been superseded.

1. Update the state field to `❌Cancelled` with a brief rationale.
2. Move the file to `cancelled/NNN_<slug>.md`.
3. Update the state column in `readme.md` Tasks Index to `❌ (Cancelled)`.

## Procedure: Report Bug

**Trigger:** A defect is discovered during development or testing.

1. Check `bug/` for an existing report covering the same symptom (deduplication).
2. If new, create `bug/NNN_<slug>.md` with: symptom, reproduction steps, affected component, and severity.
3. Create or link a corresponding task in `unverified/` with state `❓Unverified` and a reference to the bug ID.
4. Add an entry to the Issues Index in `readme.md`.
5. When the fix lands, update the bug file state to `Fixed` and close the linked task.
