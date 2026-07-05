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

### FT-6: Help request is intercepted and converted to a successful output

- **Given:** A `Pipeline` initialized with a registry containing at least one registered command; input `"."` (or a command followed by `?`) that triggers the semantic analyzer's `HelpRequested` signal
- **When:** `pipeline.process_command(".", context)` is called
- **Then:** `result.success == true`; `result.error.is_none()`; `result.outputs.len() == 1`; `result.outputs[0].content` contains formatted help text — the `HelpRequested` semantic-analysis signal is transparently converted into a successful result rather than propagated as an error, per the pipeline's Design contract that "integrators do not need to handle this case separately"

### FT-7: Argv-based execution with default context succeeds via the simple convenience wrapper

- **Given:** A `Pipeline` with `.test message::` argument; argv `[".test", "message::world"]` as separate OS argv elements
- **When:** `pipeline.process_command_from_argv_simple(&argv)` is called (no explicit `ExecutionContext` supplied)
- **Then:** `result.success == true`; `result.outputs[0].content == "world"` — behavior is identical to `process_command_from_argv` with an explicit `ExecutionContext::default()`, confirming the convenience wrapper constructs and passes the default context correctly

### FT-8: Batch and sequence modes handle an empty command list without division-by-zero

- **Given:** A `Pipeline` with a registry containing `.test`; an empty command slice `[]`
- **When:** `pipeline.process_batch(&[], context)` is called (and independently `pipeline.process_sequence(&[], context)`)
- **Then:** `batch_result.total_commands == 0`; `batch_result.successful_commands == 0`; `batch_result.failed_commands == 0`; `batch_result.results.is_empty() == true`; `batch_result.success_rate() == 0.0` (no panic or NaN from the empty-list division guard); identical result shape for both `process_batch` and `process_sequence`
