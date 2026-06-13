# Invariant Spec: Non-Functional Requirements

### Scope

- **Purpose:** Verify measurable NFR thresholds defined in `docs/invariant/002_non_functional_requirements.md` hold under realistic conditions
- **Responsibility:** Test cases exercising NFR-PERF-1..3 (performance thresholds), NFR-SEC-1 (no sensitive data logged), NFR-ROBUST-1 (structured errors, panic catch), NFR-PLATFORM-1 (WASM compat), NFR-MODULARITY-1..2 (feature gating)
- **In Scope:** NFR-PERF-1 (zero startup overhead for 1M+ commands), NFR-PERF-2 (p99 lookup < 100ns), NFR-PERF-3 (≥5M lookups/sec), NFR-SEC-1 (sensitive data excluded), NFR-ROBUST-1 (no raw panics), NFR-PLATFORM-1 (WASM build), NFR-MODULARITY-1 (feature gates), NFR-MODULARITY-2 (`enabled` feature no-op)
- **Out of Scope:** Behavioral correctness (feature specs); actor vocabulary (invariant 001); API contract (api 001)

### IN-1: Static registry startup cost is zero — no runtime initialization for 1M+ commands

- **Given:** A `StaticCommandMap` containing 1,000,000 entries compiled via PHF at build time
- **When:** The binary starts and the registry is accessed for the first time
- **Then:** No heap allocation or initialization loop occurs at startup; the registry is immediately available as a static constant

### IN-2: PHF registry lookup completes in ≤100ns p99 under repeated access

- **Given:** A `StaticCommandMap` with at least 1,000 entries and a benchmark exercising `registry.get(name)` in a tight loop
- **When:** 1,000,000 lookups are performed and p99 latency is measured
- **Then:** The p99 latency is ≤100ns; the throughput is ≥5,000,000 lookups per second

### IN-3: Sensitive argument value is absent from error output

- **Given:** A command `.login` with a `password` argument marked as sensitive; input `".login password::s3cr3t"` that triggers an error
- **When:** The error is formatted and returned as `ErrorData`
- **Then:** The string `"s3cr3t"` does not appear anywhere in the formatted error, in `OutputData`, or in any log output

### IN-4: Panicking command handler is caught and returned as structured error

- **Given:** A command `.panic_cmd` whose handler closure calls `panic!("intentional")`
- **When:** `pipeline.run(".panic_cmd")` is called
- **Then:** Returns `Err` with `error_data.code == ErrorCode::InternalError` rather than unwinding the caller stack; the process does not abort (panics are caught via `std::panic::catch_unwind` in `SemanticAnalyzer::analyze` and mapped to `InternalError`)

### IN-5: Zero-feature build compiles without errors

- **Given:** The `unilang` crate configured with `--no-default-features`
- **When:** `cargo check --no-default-features` is run
- **Then:** Exits with code 0 and zero compiler errors; the crate compiles as a no-op stub

### IN-6: Modularity — `enabled` and `full` features activate distinct capability sets

- **Given:** The `unilang` crate compiled once with `--features enabled` and once with `--features full`
- **When:** The available public API surface is compared between the two builds
- **Then:** The `full` build exposes additional functionality not present in the `enabled` build; the `enabled` build is a strict subset of `full`
