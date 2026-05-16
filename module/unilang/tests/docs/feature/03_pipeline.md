# Feature Spec: Pipeline

### Scope

- **Purpose:** Verify all FR-PIPE behavioral requirements for pipeline orchestration
- **Responsibility:** Test cases covering Parse→Semantic→Interpret flow, batch processing, sequence processing, and argv-based execution
- **In Scope:** FR-PIPE-1 (pipeline orchestration), FR-PIPE-2 (batch processing, non-stopping), FR-PIPE-3 (sequence processing, fail-fast), FR-PIPE-4 (argv-based execution combining consecutive elements)
- **Out of Scope:** Individual stage internals (covered by parser/semantic/interpreter domain tests); registry setup (FR-REG)

### FT-1: Pipeline processes valid command through all three stages

- **Given:** A `Pipeline` initialized with a registry containing `.greet` with one argument `"name"` and input `".greet name::world"`
- **When:** `pipeline.run(".greet name::world")` is called
- **Then:** Returns `Ok(VerifiedCommand { name: ".greet", arguments: { "name": Value::String("world") } })`; no stage error

### FT-2: Batch mode processes all commands and collects all errors without stopping

- **Given:** A `Pipeline` with registry containing `.ok` and `.fail`, and a batch input `[".fail", ".ok", ".fail"]` where `.fail` always returns an error
- **When:** Batch execution is triggered for all three commands
- **Then:** Returns a result set with 2 errors (for `.fail` invocations) and 1 success (for `.ok`); all three commands are processed regardless of failures

### FT-3: Sequence mode stops at first failure and returns that error

- **Given:** A `Pipeline` with a sequence of commands `[".fail", ".ok"]` where `.fail` always returns an error
- **When:** Sequence execution is triggered
- **Then:** Returns the error from `.fail`; `.ok` is never executed (observable via side-effect counter or mock)

### FT-4: Argv-based execution joins consecutive argv elements for named arg

- **Given:** A `Pipeline` and argv array `["prog", ".cmd", "--url", "https://example.com"]` where `--url` is the named argument prefix
- **When:** `pipeline.run_argv(&args)` is called
- **Then:** The `url` argument receives `Value::String("https://example.com")`; the split across argv elements is transparent to the argument system

### FT-5: Pipeline returns CommandNotFound error for unregistered command

- **Given:** A `Pipeline` with a registry that does not contain `.missing`
- **When:** `pipeline.run(".missing")` is called
- **Then:** Returns `Err(UnilangError::CommandNotFound { name: ".missing", suggestions: [...] })` without panic
