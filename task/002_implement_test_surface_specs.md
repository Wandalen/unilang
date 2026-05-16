# Implement test surface — Rust tests for all 10 spec files

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** 🎯 (Available)
- **Validated By:** null
- **Validation Date:** null
- **Closes:** null

## Infrastructure Completed (pre-work, 2026-05-16)

Before implementing spec cases, the test infrastructure was reorganized:
- All 35+ misplaced top-level `tests/*.rs` files moved to proper domain directories
- All domain entry-point files (`tests/<domain>.rs`) updated with full `mod` declarations
- `tests/validation/` directory created with 7 CI-level tests; `tests/validation.rs` entry point added
- `tests/build/` directory created with 4 build-time tests; entry point updated
- `tests/api/` directory created (readme.md only — ready for AP-1..8 implementations)
- `tests/readme.md` and all subdirectory `readme.md` files updated to reflect structure
- FR-mapping comments added to key existing test functions (registry, semantic, help domains)
- `.config/nextest.toml` created: validation tests run with `test-threads=1` (each spawns external cargo)
- 1072 tests pass clean after reorganization

**Blocked help tests** (require API alignment before inclusion — see `tests/help/readme.md`):
- `conventions.rs`: uses private `CommandDefinition` fields + non-existent Pipeline methods
- `enforcement.rs`: uses private `CommandDefinition` fields
- `operator.rs`: `error_data.code` type mismatch (`ErrorCode` vs `&str`) + private fields

These three files are in `tests/help/` but excluded from `tests/help.rs` until fixed. Fixing them is **in scope for this task** alongside implementing the FR-HELP spec cases.

## Goal

Create Rust test implementations for all 10 test surface spec files in `tests/docs/` so that every `FT-`, `IN-`, and `AP-` case defined in those specs has a passing, non-trivial test in the corresponding `tests/<domain>/` directory, verified by `w3 .test level::3` passing with zero failures. (Motivated: the test surface spec files were created in the doc_tsk session and define 40+ test cases with zero corresponding Rust implementations — behavioral requirements FR-REG through FR-MOD-WASM-REPL, all invariants, and all public API contracts are completely uncovered; Observable: 40+ new test functions appear in `tests/registry/`, `tests/semantic/`, `tests/parser/`, `tests/system/`, `tests/acceptance/`, and `tests/api/`; Scoped: implements cases from `tests/docs/feature/01–05`, `tests/docs/invariant/01–04`, `tests/docs/api/01` — no new features, no refactoring; Testable: `w3 .test level::3` exits 0 with all new tests passing.)

The spec files provide the test contract. The executor reads each spec, implements one test function per case in the correct domain directory, then verifies the full suite passes. Tests must use real implementations — no mocking, no `assert!(true)`, no `#[ignore]`.

After implementation, each spec file's status in `tests/docs/<surface>/readme.md` must be updated from `⏳` to `✅`.

## In Scope

- `tests/registry/` — FT-1..7 from `01_command_registry.md` (FR-REG-1..9)
- `tests/semantic/` — FT-1..7 from `02_argument_system.md` (FR-ARG-1..8)
- `tests/system/` — FT-1..5 from `03_pipeline.md` (FR-PIPE-1..4)
- `tests/help/` — FT-1..7 from `04_help_system.md` (FR-HELP-1..7)
- `tests/system/` or `tests/acceptance/` — FT-1..5 from `05_repl_interactive.md` (FR-REPL-1, FR-INTERACTIVE-1, FR-MOD-WASM-REPL)
- `tests/system/` — IN-1..3 from `01_system_actors_vocabulary.md`
- `tests/system/` or `benches/` — IN-1..6 from `02_non_functional_requirements.md` (NFR-PERF, NFR-SEC, NFR-ROBUST, NFR-PLATFORM, NFR-MODULARITY)
- `tests/system/` — IN-1..4 from `03_governing_principles.md`
- `tests/system/` — IN-1..4 from `04_workspace_dependency_standards.md`
- `tests/api/` (create directory) — AP-1..8 from `01_public_types.md`
- Update status `⏳` → `✅` in all 10 `tests/docs/*/readme.md` overview table rows after tests pass

## Out of Scope

- The spec files themselves (already created in the doc_tsk session — do not modify case content)
- New public API additions or feature implementations (tests only exercise existing API)
- Performance benchmark harness setup beyond what `cargo bench` supports natively
- CLI binary tests (`cargo_unilang` crate)
- Documentation edits other than the `⏳` → `✅` status updates

## Requirements

-   All work must strictly adhere to all applicable rulebooks
    (discover via `kbase .rulebooks`)
-   Tests must use real implementations — no mocking, no `assert!(true)`, no `#[ignore]`
-   Each test function must have a doc comment citing the spec case it implements (e.g., `// FT-1: Static PHF registry lookup`)
-   Tests producing compile-time errors (AP-1, AP-5, IN-2) must use `trybuild` or `compile_fail` doc tests, not runtime assertions
-   No test may silently pass due to missing tokens, missing registry entries, or unconfigured environments
-   WASM target test (FT-4 from `05_repl_interactive.md`) must be a build-only check using `cargo check --target wasm32-unknown-unknown`

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note constraints on test file placement (`tests/<domain>/`), naming (`snake_case`), and doc comment format.
2. **Read spec files** — Read all 10 spec files in `tests/docs/` as the test contract. Note each case's Given/When/Then. Do not implement until all 10 are read.
3. **Write Test Matrix** — Populate every row (see Test Matrix below) before opening any test file.
4. **Create domain test files** — For each domain group (registry, semantic, system, help, api), create or extend one test file. Each function implements exactly one spec case. Name: `test_<case_id_snake>()` (e.g., `test_ft1_static_phf_lookup()`).
5. **Implement compile-fail cases** — For AP-1, AP-5, IN-2: create `trybuild` test or `compile_fail` doc test confirming the type error occurs.
6. **Implement WASM check** — For FT-4 (REPL WASM): add a `Makefile` target or CI step verifying `cargo check --target wasm32-unknown-unknown --no-default-features` exits 0, or convert to an integration test that runs this command via `std::process::Command`.
7. **Green state** — `w3 .test level::3` must pass with zero failures and zero warnings before proceeding.
8. **Update spec status** — In each `tests/docs/<surface>/readme.md`, change `⏳` to `✅` for every spec whose tests pass.
9. **Walk Validation Checklist** — check every item. Every answer must be YES.
10. **Update task status** — Set ✅ in `task/readme.md`, recalculate advisability to 0, move file to `task/completed/`.

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | `registry.get(".foo")` on static PHF map containing `.foo` | `StaticCommandRegistry` | Returns `Some(def)` with `def.name() == ".foo"` |
| T02 | `command_add_runtime` adds `.bar`; then `registry.get(".bar")` | `CommandRegistry` with static base | Returns `Some(def)` for `.bar`; `.foo` still accessible |
| T03 | `CommandDefinition` built with name `"noDotPrefix"` | Builder validation | Compile error or runtime error mentioning dot-prefix requirement |
| T04 | `registry.get(".f")` where `".f"` is alias for `".foo"` | Registry with alias | Returns `Some(def)` with `def.name() == ".foo"` |
| T05 | Two YAML sources both define `".dup"` | Declarative loader merge | Returns error indicating conflict on `".dup"` |
| T06 | `registry.get(".foo")` and `registry.get(".foo.bar")` | Registry with nested namespaces | Each returns its own distinct definition |
| T07 | `get(name)` on both `StaticCommandRegistry` and `CommandRegistry` | Same `StaticCommandMap` source | Both return equivalent definitions (name, arg count, description) |
| T08 | Input `[".cmd", "url::https://example.com"]` with `String` arg `"url"` | `SemanticAnalyzer` | `arguments["url"] == Value::String("https://example.com")` |
| T09 | Input `[".cmd", "ouput::foo"]` (typo) with arg `"output"` | `SemanticAnalyzer` | Error of kind `UnknownArgument` with suggestions containing `"output"` |
| T10 | Input `[".cmd"]` with `Bool` arg `"verbose"` default `false` | `SemanticAnalyzer` | `arguments["verbose"] == Value::Bool(false)`; no error |
| T11 | Input with `tag::alpha`, `tag::beta`, `tag::gamma` on multiple arg `"tag"` | `SemanticAnalyzer` | `arguments["tag"] == Value::Array([...])` with 3 elements |
| T12 | Input `[".cmd", "hello"]` with positional `String` arg at position 0 | `SemanticAnalyzer` | Positional arg receives `Value::String("hello")` |
| T13 | Input `[".cmd", "count::42"]` with `Kind::I64` arg `"count"` | `SemanticAnalyzer` | `arguments["count"] == Value::I64(42)` |
| T14 | Input `[".cmd"]` with required `String` arg `"name"` (no default) | `SemanticAnalyzer` | Returns error indicating `"name"` is required; no panic |
| T15 | `pipeline.run(".greet name::world")` on registered `.greet` | `Pipeline` full run | Returns `Ok(VerifiedCommand { name: ".greet", arguments: {"name": String("world")} })` |
| T16 | Batch run of `[".fail", ".ok", ".fail"]` | `Pipeline` batch mode | 2 errors + 1 success; all 3 processed |
| T17 | Sequence run of `[".fail", ".ok"]` | `Pipeline` sequence mode | `.fail` error returned; `.ok` never executed |
| T18 | Argv `["prog", ".cmd", "--url", "https://x"]` | `Pipeline` argv mode | `url` argument receives `Value::String("https://x")` |
| T19 | `pipeline.run(".missing")` — not in registry | `Pipeline` | Returns `Err(CommandNotFound { name: ".missing" })` without panic |
| T20 | `help_list(&registry)` with 3 commands registered | Help API | Output contains all 3 command names |
| T21 | `help_command(&registry, ".greet")` with arg description | Help API | Output contains arg name and description |
| T22 | `pipeline.run("?")` | Pipeline with `?` operator | Output lists registered commands; no error |
| T23 | `pipeline.run(".greet ??")` | Pipeline with `??` param | Help text included; handler not called |
| T24 | `UNILANG_HELP_VERBOSITY=0` vs `=4` on same command | Help verbosity | Level 0 output ⊂ level 4 output (strictly less detail) |
| T25 | `pipeline.run(".greet.help")` | Auto help API | Returns help text for `.greet` |
| T26 | `pipeline.run("? .unknown")` — unknown command | `?` operator | Not-found indication; no panic |
| T27 | Same pipeline used for `.set` then `.get` | Stateless REPL | `.get` result independent of `.set` side effects |
| T28 | Required arg absent + `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED=1` | Interactive mode | Prompt emitted; execution waits |
| T29 | All required args provided + `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED=1` | Interactive mode | No prompt; immediate execution |
| T30 | `cargo check --target wasm32-unknown-unknown --no-default-features` | WASM build | Exits 0; zero errors |
| T31 | `pipeline.run("")` — empty input | Pipeline | Returns error or no-op; no panic |
| T32 | Search for `"executor"` as actor name in codebase | Vocabulary invariant | Zero occurrences |
| T33 | `docs/invariant/001` actor categories enumerated | Vocabulary invariant | All 3 categories (Human, System, Internal) present |
| T34 | `SemanticAnalyzer` / `semantic_analyzer` naming check | Vocabulary invariant | Canonical name found; no `validator` / `checker` synonym |
| T35 | PHF registry with 1000+ entries: throughput benchmark | NFR-PERF-2/3 | ≥5M lookups/sec; p99 ≤100ns |
| T36 | `.login password::s3cr3t` triggers error; error text inspected | NFR-SEC-1 | `"s3cr3t"` absent from all error output |
| T37 | `.panic_cmd` handler calls `panic!`; pipeline run called | NFR-ROBUST-1 | Returns `Err(HandlerPanic)` without stack unwind |
| T38 | `cargo check --no-default-features` | NFR-MODULARITY / R4 | Exits 0; zero warnings |
| T39 | `pipeline.run("@invalid!command")` | Fail-Fast principle | Returns `ParseError` not `SemanticError` |
| T40 | `CommandDefinition::former().description("x").end()` (no name) | Type-state builder | Compile-time error |
| T41 | Fresh registry: `registry.get(".help")` | Minimum Implicit Magic | Returns `None` (no hidden commands) |
| T42 | `? .greet`, `.greet ??`, `.greet.help` output comparison | Consistent Help Access | All three contain same command name and arg info |
| T43 | All workspace external deps version strings | R1 format | All match `^X.Y` pattern |
| T44 | All wTools dep version strings | R1 format | All match `=X.Y.Z` pattern |
| T45 | Individual crate Cargo.toml `[dependencies]` sections | R2/R3 | No standalone version literals; all `{ workspace = true }` |
| T46 | `CommandDefinition::former()...end()` all fields set | AP-1 builder | Valid definition returned |
| T47 | `pipeline.run(".echo msg::hello")` with `.echo` registered | AP-2 round-trip | `VerifiedCommand.arguments["msg"] == String("hello")` |
| T48 | 15 `Kind` variants each constructed from valid input | AP-3 type coverage | All 15 succeed; no panic |
| T49 | `registry.get(".query")` on registry with `.query` | AP-4 lookup | Returns `Some(def)` with matching description |
| T50 | `definition.name` direct field access | AP-5 private fields | Compile-time private-field error |

## Acceptance Criteria

- All 50 test matrix rows have a corresponding non-trivial passing test function in the correct domain directory
- Compile-fail cases (T03, T40, T50) use `trybuild` or `compile_fail` doc tests that actually fail to compile without the guard
- WASM check (T30) is invoked as an external `cargo check` command in a test function using `std::process::Command`
- `w3 .test level::3` exits with 0 failures and 0 warnings
- All 10 `tests/docs/<surface>/readme.md` overview rows show status `✅`
- No test function contains `assert!(true)`, `unimplemented!()`, `todo!()`, or `#[ignore]`
- Every test function has a comment `// <CASE_ID>: <short name>` citing its spec case

## Validation

### Checklist

Desired answer for every question is YES.

**Feature test coverage (tests/docs/feature/)**
- [ ] C1 — Do tests for `01_command_registry.md` exist in `tests/registry/` covering FT-1..7?
- [ ] C2 — Do tests for `02_argument_system.md` exist in `tests/semantic/` covering FT-1..7?
- [ ] C3 — Do tests for `03_pipeline.md` exist in `tests/system/` covering FT-1..5?
- [ ] C4 — Do tests for `04_help_system.md` exist in `tests/help/` covering FT-1..7?
- [ ] C5 — Do tests for `05_repl_interactive.md` exist covering FT-1..5 (including WASM build check)?

**Invariant test coverage (tests/docs/invariant/)**
- [ ] C6 — Do vocabulary invariant tests exist covering IN-1..3 for `01_system_actors_vocabulary.md`?
- [ ] C7 — Do NFR tests exist covering IN-1..6 for `02_non_functional_requirements.md`?
- [ ] C8 — Do governing principle tests exist covering IN-1..4 for `03_governing_principles.md`?
- [ ] C9 — Do workspace standard tests exist covering IN-1..4 for `04_workspace_dependency_standards.md`?

**API test coverage (tests/docs/api/)**
- [ ] C10 — Do API tests exist covering AP-1..8 for `01_public_types.md`?

**Test quality**
- [ ] C11 — Does every test function cite its spec case in a comment?
- [ ] C12 — Are compile-fail cases (T03, T40, T50) implemented as `compile_fail` tests that actually fail without the guard?
- [ ] C13 — Is `assert!(true)` absent from all new test functions?
- [ ] C14 — Is `#[ignore]` absent from all new test functions?

**Spec status updates**
- [ ] C15 — Do all 10 `tests/docs/*/readme.md` overview rows show `✅`?

**Out of Scope confirmation**
- [ ] C16 — Are the spec file case descriptions in `tests/docs/` unchanged (content not edited)?
- [ ] C17 — Are no new public API symbols added to `src/`?

### Measurements

- [ ] M1 — test count: `cargo nextest list --all-features 2>&1 | grep -c "test_ft\|test_in\|test_ap"` → ≥50 (was: 0)
- [ ] M2 — compile-fail tests: `ls tests/compile_fail/ 2>/dev/null | wc -l` → ≥3 (was: 0)
- [ ] M3 — spec status: `grep -c "✅" tests/docs/feature/readme.md tests/docs/invariant/readme.md tests/docs/api/readme.md` → 10 (was: 0)

### Invariants

- [ ] I1 — test suite: `w3 .test level::3` → 0 failures
- [ ] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check --all-features` → 0 warnings

### Anti-faking checks

- [ ] AF1 — no trivial asserts: `grep -rn "assert!(true)" tests/` → 0 matches
- [ ] AF2 — no ignored tests: `grep -rn "#\[ignore\]" tests/` → 0 matches
- [ ] AF3 — compile-fail tests compile-fail without guard: temporarily remove the guard from one compile-fail test; verify `cargo test` fails on that case
- [ ] AF4 — WASM check actually runs: `grep -n "wasm32" tests/system/` (or similar domain file) → ≥1 match confirming the check is exercised

## Outcomes

[To be populated upon task completion.]
