# Feature Spec: REPL and Interactive Mode

### Scope

- **Purpose:** Verify FR-REPL-1, FR-INTERACTIVE-1, and FR-MOD-WASM-REPL behavioral requirements
- **Responsibility:** Test cases covering stateless REPL operation, interactive argument prompting, and WebAssembly modality constraints
- **In Scope:** FR-REPL-1 (stateless REPL — same pipeline reusable across multiple calls), FR-INTERACTIVE-1 (interactive signal emitted when `interactive: true` argument is absent — `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` error code returned, not an env-var check), FR-MOD-WASM-REPL (WebAssembly build compatibility — no std-only APIs in WASM path)
- **Out of Scope:** Help output formatting (FR-HELP); pipeline batch/sequence modes (FR-PIPE)

### FT-1: Stateless REPL — repeated calls produce no state leakage between invocations

- **Given:** A `Pipeline` with a `.test` command that returns its optional `message` argument (default `"hello"`); three consecutive calls made on the same pipeline instance: `".test first"`, `".test second"`, `".test"` (no argument)
- **When:** All three calls are made without resetting the pipeline
- **Then:** Each result reflects only the argument supplied in that call; `result3.outputs[0].content == "hello"` (the default, not the previous `"second"`); no per-call state accumulates between invocations

### FT-2: Interactive signal emitted when required interactive arg is absent

- **Given:** A command `.greet` with a required argument `"name"` (`interactive: true, optional: false`, no default); invocation `".greet"` (argument absent)
- **When:** `pipeline.process_command(".greet")` is called
- **Then:** `result.success == false`; `result.requires_interactive_input() == true`; `result.interactive_argument() == Some("name")` — the REPL layer can intercept this signal to prompt the user for the missing value

### FT-3: No interactive signal when required interactive arg is provided

- **Given:** Same `.greet` command with `"name"` argument (`interactive: true`); invocation `".greet name::alice"` (argument present)
- **When:** `pipeline.process_command(".greet name::alice")` is called
- **Then:** `result.success == true`; `result.requires_interactive_input() == false`; output contains `"Hello, alice!"`; no interactive prompt is triggered

### FT-4: WASM build compiles without std-only features

- **Given:** The `unilang` crate compiled with `--target wasm32-unknown-unknown --no-default-features --features enabled`
- **When:** `cargo build` or `cargo check` runs with the WASM target
- **Then:** Compilation succeeds with zero errors; no `std`-only API (threads, filesystem, process exit) is referenced in the WASM code path

### FT-5: Empty REPL input handled without panic

- **Given:** A `Pipeline` and input string `""` (empty)
- **When:** `pipeline.process_command("", context)` is called
- **Then:** Returns a failed result with `result.error.is_some()`, or succeeds with no-op behavior (e.g., help listing as fallback); no panic occurs under any path
