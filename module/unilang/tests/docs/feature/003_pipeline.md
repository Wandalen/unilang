# Feature Spec: Pipeline

### Scope

- **Purpose:** Verify all FR-PIPE behavioral requirements for pipeline orchestration
- **Responsibility:** Test cases covering Parse→Semantic→Interpret flow, batch processing, sequence processing, and argv-based execution
- **In Scope:** FR-PIPE-1 (pipeline orchestration), FR-PIPE-2 (batch processing, non-stopping), FR-PIPE-3 (sequence processing, fail-fast), FR-PIPE-4 (argv-based execution combining consecutive elements)
- **Out of Scope:** Individual stage internals (covered by parser/semantic/interpreter domain tests); registry setup (FR-REG)

### FT-1: Pipeline processes valid command through all three stages

- **Given:** A `Pipeline` initialized with a registry containing `.test` with optional argument `"message"` (default `"hello"`); input `".test world"` (positional binding)
- **When:** `pipeline.process_command(".test world", context)` is called
- **Then:** `result.success == true`; `result.error.is_none()`; `result.outputs.len() == 1`; `result.outputs[0].content == "world"` — the value passed through Parse→Semantic→Interpret without error

### FT-2: Batch mode processes all commands and collects all errors without stopping

- **Given:** A `Pipeline` with a registry containing `.test`; batch input `[".test hello", ".test world", "nonexistent"]` where `"nonexistent"` is not in the registry
- **When:** `pipeline.process_batch(&commands, context)` is called
- **Then:** `batch_result.total_commands == 3`; `batch_result.successful_commands == 2`; `batch_result.failed_commands == 1`; `batch_result.success_rate() ≈ 66.7%`; all three commands are processed regardless of the failure

### FT-3: Sequence mode stops at first failure and does not execute subsequent commands

- **Given:** A `Pipeline` with a registry containing `.test`; sequence `[".nonexistent", ".test"]` where `".nonexistent"` is not in the registry
- **When:** `pipeline.process_sequence(&commands, context)` is called
- **Then:** `batch_result.total_commands == 2`; `batch_result.failed_commands == 1`; `batch_result.successful_commands == 0`; `batch_result.results.len() == 1` (second command never executed); `batch_result.results[0].success == false`

### FT-4: Argv-based execution joins elements into a single command string

- **Given:** A `Pipeline` with `.test message::` argument; argv `[".test", "message::world"]` as separate OS argv elements
- **When:** `pipeline.process_command_from_argv(&argv, context)` is called
- **Then:** `result.success == true`; `result.outputs[0].content == "world"` — elements joined transparently before parsing

### FT-5: Pipeline returns CommandNotFound error for unregistered command

- **Given:** A `Pipeline` with a registry that does not contain `.nonexistent`
- **When:** `pipeline.process_command(".nonexistent", context)` is called
- **Then:** `result.success == false`; `result.error.is_some()`; error message contains `"nonexistent"`, `"not found"`, or `"CommandNotFound"`; no panic occurs
