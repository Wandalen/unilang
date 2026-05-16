# Feature Spec: REPL and Interactive Mode

### Scope

- **Purpose:** Verify FR-REPL-1, FR-INTERACTIVE-1, and FR-MOD-WASM-REPL behavioral requirements
- **Responsibility:** Test cases covering stateless REPL operation, interactive argument prompting, and WebAssembly modality constraints
- **In Scope:** FR-REPL-1 (stateless REPL — same pipeline reusable across multiple calls), FR-INTERACTIVE-1 (interactive prompting when `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` is set and argument is missing), FR-MOD-WASM-REPL (WebAssembly build compatibility — no std-only APIs in WASM path)
- **Out of Scope:** Help output formatting (FR-HELP); pipeline batch/sequence modes (FR-PIPE)

### FT-1: Stateless REPL — repeated calls produce no state leakage between invocations

- **Given:** A `Pipeline` used to execute `.set value::42` followed by `.get`
- **When:** Both calls are made using the same pipeline instance without any reset
- **Then:** The second call's result is independent of the first call's argument; no side-effects from `.set` appear in `.get`'s processing context (pipeline holds no mutable per-call state)

### FT-2: Interactive prompting triggered when required arg absent and env var set

- **Given:** `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED=1` is set in the environment; command `.greet` has one required argument `"name"` with no default; input is `".greet"` (argument absent)
- **When:** `pipeline.run(".greet")` is called in a context that supports interactive I/O
- **Then:** A prompt is emitted to request the `"name"` value; execution does not proceed until the value is provided

### FT-3: No prompting when all required arguments are already provided

- **Given:** `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED=1` is set; command `.greet` has argument `"name"`; input is `".greet name::world"`
- **When:** `pipeline.run(".greet name::world")` is called
- **Then:** No prompt is emitted; the command executes immediately with `name = "world"`

### FT-4: WASM build compiles without std-only features

- **Given:** The `unilang` crate compiled with `--target wasm32-unknown-unknown --no-default-features --features enabled`
- **When:** `cargo build` or `cargo check` runs with the WASM target
- **Then:** Compilation succeeds with zero errors; no `std`-only API (threads, filesystem, process exit) is referenced in the WASM code path

### FT-5: Empty REPL input handled without panic

- **Given:** A `Pipeline` and input string `""` (empty)
- **When:** `pipeline.run("")` is called
- **Then:** Returns an error (e.g., `ParseError::EmptyInput`) or `Ok` with no-op behavior; no panic occurs
