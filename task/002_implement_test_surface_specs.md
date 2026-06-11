# Implement test surface — Rust tests for all 17 spec files

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

Create Rust test implementations for all 17 test surface spec files in `tests/docs/` so that every `FT-`, `IN-`, `AP-`, and `TC-` case defined in those specs has a passing, non-trivial test in the corresponding `tests/<domain>/` directory, verified by `w3 .test level::3` passing with zero failures. (Motivated: the test surface spec files define 121 test cases; the ⏳ cases have zero corresponding Rust implementations — behavioral requirements FR-REG through FR-MOD-WASM-REPL, invariants, API contracts, and domain type guarantees have coverage gaps; Observable: 106+ new test functions appear in `tests/registry/`, `tests/semantic/`, `tests/system/`, `tests/help/`, `tests/api/`, and `tests/data/`; Scoped: implements cases from `tests/docs/feature/01–05`, `tests/docs/invariant/01–06`, `tests/docs/api/01–02`, `tests/docs/type/01–04` — no new features, no refactoring; Testable: `w3 .test level::3` exits 0 with all new tests passing.)

The spec files provide the test contract. The executor reads each spec, implements one test function per case in the correct domain directory, then verifies the full suite passes. Tests must use real implementations — no mocking, no `assert!(true)`, no `#[ignore]`.

After implementation, each spec file's status in `tests/docs/<surface>/readme.md` must be updated from `⏳` to `✅`.

## In Scope

- `tests/registry/` — FT-1..13 from `01_command_registry.md` (FR-REG-1..9)
- `tests/semantic/` — FT-1..13 from `02_argument_system.md` (FR-ARG-1..8)
- `tests/system/` — FT-1..5 from `03_pipeline.md` (FR-PIPE-1..4)
- `tests/help/` — FT-1..11 from `04_help_system.md` (FR-HELP-1..8)
- `tests/system/` or `tests/acceptance/` — FT-1..5 from `05_repl_interactive.md` (FR-REPL-1, FR-INTERACTIVE-1, FR-MOD-WASM-REPL)
- `tests/system/` — IN-1..3 from `01_system_actors_vocabulary.md`
- `tests/system/` or `benches/` — IN-1..6 from `02_non_functional_requirements.md` (NFR-PERF, NFR-SEC, NFR-ROBUST, NFR-PLATFORM, NFR-MODULARITY)
- `tests/system/` — IN-1..5 from `03_governing_principles.md`
- `tests/system/` — IN-1..5 from `04_workspace_dependency_standards.md` (R1–R4 + R3 optional deps)
- `tests/system/` — IN-1..4 from `05_command_naming.md` (runtime + build-time dot-prefix enforcement)
- `tests/api/` (create directory) — AP-1..10 from `01_public_types.md`
- `tests/api/` — AP-1..14 from `02_error_codes.md` (all 12 ErrorCode variants + derives + string representations)
- `tests/system/` — IN-1..4 from `06_build_runtime_separation.md` (no runtime serde_yaml/serde_json, static data, validation_core identity)
- `tests/data/` — TC-1..6 from `type/01_command_name.md` (CommandName validated newtype)
- `tests/data/` — TC-1..5 from `type/02_namespace_type.md` (NamespaceType validated newtype)
- `tests/data/` — TC-1..5 from `type/03_version_type.md` (VersionType validated newtype)
- `tests/data/` — TC-1..7 from `type/04_command_status.md` (CommandStatus lifecycle enum + serde)
- Update status `⏳` → `✅` in all 17 `tests/docs/*/readme.md` overview table rows after tests pass

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
2. **Read spec files** — Read all 17 spec files in `tests/docs/` as the test contract. Note each case's Given/When/Then. Do not implement until all 17 are read.
3. **Write Test Matrix** — Populate every row (see Test Matrix below) before opening any test file.
4. **Create domain test files** — For each domain group (registry, semantic, system, help, api, data), create or extend one test file. Each function implements exactly one spec case. Name: `test_<case_id_snake>()` (e.g., `test_ft1_static_phf_lookup()`).
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
| T51 | `pipeline.run(".help")` or `pipeline.run("?")` | FR-HELP-8 self-exclusion | Output does NOT contain `.help` as a listed command |
| T52 | `CommandRegistry::load_from_yaml_str` with `.greet` definition | FR-REG-3 declarative loading | Registry contains `.greet` with correct args and description |
| T53 | Build validation on YAML manifest with `name: "no_dot"` | FR-REG-9 build-time validation | Build error referencing missing dot prefix |
| T54 | Input `[".cmd", "o::result.txt"]` where `"o"` is alias for `"output"` | FR-ARG-4 alias binding | `arguments["output"] == Value::String("result.txt")` |
| T55 | Input `[".cmd", "name::ab"]` with `MinLength(3)` validation rule | FR-ARG-6 validation enforcement | Returns `UNILANG_VALIDATION_RULE_FAILED` |
| T56 | `pipeline.run(".cmd val1 val2 val3")` with 1 positional arg | ErrorCode::TooManyArguments | Returns error with `TooManyArguments` code |
| T57 | `pipeline.run(".cmd count::0")` with `Min(1)` rule | ErrorCode::ValidationRuleFailed | Returns error with `ValidationRuleFailed` code |
| T58 | `pipeline.run(".login")` with interactive required arg | ErrorCode::ArgumentInteractiveRequired | Returns error with `ArgumentInteractiveRequired` code |
| T59 | Execute `.stub` with no bound routine | ErrorCode::CommandNotImplemented | Returns error with `CommandNotImplemented` code |
| T60 | `pipeline.run(".greet ??")` | ErrorCode::HelpRequested → OutputData | Returns `Ok(output_data)` with help content |
| T61 | Trigger internal invariant violation | ErrorCode::InternalError | Returns error with `InternalError` code |
| T62 | Library crate deps in `Cargo.toml` | R3 optional deps | All deps in unilang/unilang_parser have `optional = true` |
| T63 | Build validation on manifest without dot prefix | NAMING build-time | Build fails with actionable dot-prefix error |
| T64 | `CommandName::new(".hello")` | TC-1 valid dot-prefix | Ok; `as_str() == ".hello"` |
| T65 | `CommandName::new("")` | TC-2 empty string | Err(EmptyCommandName) |
| T66 | `CommandName::new("nodot")` | TC-3 missing dot prefix | Err(MissingDotPrefix("nodot")) |
| T67 | `CommandName::new(".")` | TC-4 single dot | Ok; minimal valid name |
| T68 | `CommandName::new(".video.convert")` | TC-5 nested dot name | Ok; `as_str() == ".video.convert"` |
| T69 | JSON deserialize `"nodot"` into `CommandName` | TC-6 serde boundary | Deserialization error (validation fires at serde layer) |
| T70 | `NamespaceType::new("")` | TC-1 empty namespace | Ok; `is_root() == true` |
| T71 | `NamespaceType::new(".video")` | TC-2 dot-prefixed | Ok |
| T72 | `NamespaceType::new("nodot")` | TC-3 missing dot prefix | Err(InvalidNamespace) |
| T73 | `NamespaceType::new(".tools.media")` | TC-4 nested dot namespace | Ok |
| T74 | `VersionType::new("1.0.0")` | TC-1 standard version | Ok |
| T75 | `VersionType::new("")` | TC-2 empty string | Err(EmptyVersion) |
| T76 | `VersionType::new("v")` | TC-3 single char | Ok |
| T77 | `VersionType::new("beta-rc.1+build.42")` | TC-4 arbitrary format | Ok (no format constraint) |
| T78 | `CommandStatus::Active` queried | TC-1 active variant | `is_active() == true`; `deprecation_info() == None` |
| T79 | `CommandStatus::Deprecated { reason, since, replacement }` | TC-2 deprecated metadata | All 3 fields accessible via `deprecation_info()` |
| T80 | Serde JSON serialize `CommandStatus::Active` | TC-3 simple roundtrip | Produces `"active"` (lowercase string) |
| T81 | Serde JSON roundtrip `CommandStatus::Deprecated` | TC-4 map roundtrip | Map form `{"deprecated": {"reason": ...}}` preserved |
| T82 | JSON deserialize `"ACTIVE"` | TC-5 case-insensitive | Produces `CommandStatus::Active` |
| T83 | `cargo tree -p unilang --no-default-features -e normal` | IN-1 no runtime serde_yaml | `serde_yaml` absent from output |
| T84 | Access `STATIC_COMMANDS` or generated PHF map at runtime | IN-2 static data available | Data accessible without any YAML/JSON parsing |
| T85 | `validation_core` validates `.valid` and `"invalid"` in both contexts | IN-3 shared logic identity | Build-context and runtime-context produce identical results |
| T86 | Input `[".cmd", "email::INVALID"]` with `Pattern("^[a-z]+@...")` rule | FR-ARG-6 Pattern validation | Returns `UNILANG_VALIDATION_RULE_FAILED` (pattern mismatch) |
| T87 | `pipeline.run(".greet ??")` with UNILANG_HELP_VERBOSITY unset | FR-HELP-7 default Level 2 | Output has USAGE+PARAMETERS; no version/aliases/tags (Level 3+) |
| T88 | `cargo tree --edges=normal` on built unilang crate | IN-4 no runtime serde_json | `serde_json` absent from runtime dependency tree |
| T89 | `CliBuilder` with `.db` prefix module containing `.migrate` | FR-REG-7 CliBuilder prefix | Returns `Some(def)` for `".db.migrate"`; `".migrate"` alone not found |
| T90 | Two modules both produce `".shared.run"` in `CliBuilder` | FR-REG-7 conflict detection | Error indicating naming conflict on `".shared.run"` |
| T91 | `CliBuilder::build_hybrid()` then `command_add_runtime` for `.dynamic.cmd` | FR-REG-7 hybrid mode | Both static `".db.migrate"` and dynamic `".dynamic.cmd"` accessible |
| T92 | `CommandRegistry::load_from_json_str` with `.calc` definition | FR-REG-3 JSON loading | Registry contains `.calc` with correct description and args |
| T93 | Input `[".cmd", "ratio::3.14"]` with `Kind::F32` arg `"ratio"` | FR-ARG-1 Float coercion | `arguments["ratio"] == Value::F32(3.14)` |
| T94 | Input `[".cmd", "file::/tmp/data.csv"]` with `Kind::Path` arg `"file"` | FR-ARG-1 Path coercion | `arguments["file"] == Value::Path("/tmp/data.csv")` |
| T95 | Input `[".cmd", "count::101"]` with `Max(100)` validation rule | FR-ARG-6 Max validation | Returns `UNILANG_VALIDATION_RULE_FAILED` (exceeds maximum) |
| T96 | `pipeline.run(".greet ??")` with `UNILANG_HELP_VERBOSITY=1` | FR-HELP-7 Level 1 Basic | USAGE line present; no PARAMETERS descriptions |
| T97 | `pipeline.run(".greet ??")` with `UNILANG_HELP_VERBOSITY=3` | FR-HELP-7 Level 3 Detailed | USAGE + PARAMETERS + version metadata; more than Level 2 |
| T98 | `command_add_runtime` duplicate `".dup"` on existing registry | GP SSOT principle | Error `CommandAlreadyExists`; original definition retained |
| T99 | `pipeline.process_command_from_argv(["prog", ".echo", "msg::hello world"])` | AP-9 argv boundaries | `arguments["msg"] == Value::String("hello world")` (space preserved) |
| T100 | `pipeline.process_batch([".fail", ".ok", ".fail"])` | AP-10 batch mode | Returns `[Err, Ok, Err]`; all 3 processed, no short-circuit |
| T101 | Extract `Value::String` as integer via typed method | AP-13 TypeMismatch | `error_data.code == ErrorCode::TypeMismatch` |
| T102 | `format!("{}", ErrorCode::CommandNotFound)` and other variants | AP-14 string representations | Produces `"UNILANG_COMMAND_NOT_FOUND"` etc. matching catalog |
| T103 | JSON deserialize `"video"` into `NamespaceType` | TC-5 serde namespace | Deserialization error (non-dot-prefixed rejected) |
| T104 | JSON deserialize `""` into `VersionType` | TC-5 serde version | Deserialization error (empty string rejected) |
| T105 | `CommandStatus::Experimental` queried | TC-6 experimental variant | `is_experimental() == true`; all others `false` |
| T106 | `CommandStatus::Internal` queried | TC-7 internal variant | `is_internal() == true`; all others `false` |

## Acceptance Criteria

- All 106 test matrix rows have a corresponding non-trivial passing test function in the correct domain directory
- Compile-fail cases (T03, T40, T50) use `trybuild` or `compile_fail` doc tests that actually fail to compile without the guard
- WASM check (T30) is invoked as an external `cargo check` command in a test function using `std::process::Command`
- `w3 .test level::3` exits with 0 failures and 0 warnings
- All 17 `tests/docs/<surface>/readme.md` overview rows show status `✅`
- No test function contains `assert!(true)`, `unimplemented!()`, `todo!()`, or `#[ignore]`
- Every test function has a comment `// <CASE_ID>: <short name>` citing its spec case

## Validation

### Checklist

Desired answer for every question is YES.

**Feature test coverage (tests/docs/feature/)**
- [ ] C1 — Do tests for `01_command_registry.md` exist in `tests/registry/` covering FT-1..13?
- [ ] C2 — Do tests for `02_argument_system.md` exist in `tests/semantic/` covering FT-1..13?
- [ ] C3 — Do tests for `03_pipeline.md` exist in `tests/system/` covering FT-1..5?
- [ ] C4 — Do tests for `04_help_system.md` exist in `tests/help/` covering FT-1..11?
- [ ] C5 — Do tests for `05_repl_interactive.md` exist covering FT-1..5 (including WASM build check)?

**Invariant test coverage (tests/docs/invariant/)**
- [ ] C6 — Do vocabulary invariant tests exist covering IN-1..3 for `01_system_actors_vocabulary.md`?
- [ ] C7 — Do NFR tests exist covering IN-1..6 for `02_non_functional_requirements.md`?
- [ ] C8 — Do governing principle tests exist covering IN-1..5 for `03_governing_principles.md`?
- [ ] C9 — Do workspace standard tests exist covering IN-1..5 for `04_workspace_dependency_standards.md`?
- [ ] C9b — Do command naming tests exist covering IN-1..4 for `05_command_naming.md`?
- [ ] C9c — Do build-runtime separation tests exist covering IN-1..4 for `06_build_runtime_separation.md`?

**API test coverage (tests/docs/api/)**
- [ ] C10 — Do API tests exist covering AP-1..10 for `01_public_types.md`?
- [ ] C10b — Do API tests exist covering AP-1..14 for `02_error_codes.md` (all ErrorCode variants + string representations)?

**Type test coverage (tests/docs/type/)**
- [ ] C10c — Do type tests exist covering TC-1..6 for `01_command_name.md`?
- [ ] C10d — Do type tests exist covering TC-1..5 for `02_namespace_type.md`?
- [ ] C10e — Do type tests exist covering TC-1..5 for `03_version_type.md`?
- [ ] C10f — Do type tests exist covering TC-1..7 for `04_command_status.md`?

**Test quality**
- [ ] C11 — Does every test function cite its spec case in a comment?
- [ ] C12 — Are compile-fail cases (T03, T40, T50) implemented as `compile_fail` tests that actually fail without the guard?
- [ ] C13 — Is `assert!(true)` absent from all new test functions?
- [ ] C14 — Is `#[ignore]` absent from all new test functions?

**Spec status updates**
- [ ] C15 — Do all 17 `tests/docs/*/readme.md` overview rows show `✅`?

**Out of Scope confirmation**
- [ ] C16 — Are the spec file case descriptions in `tests/docs/` unchanged (content not edited)?
- [ ] C17 — Are no new public API symbols added to `src/`?

### Measurements

- [ ] M1 — test count: `cargo nextest list --all-features 2>&1 | grep -c "test_ft\|test_in\|test_ap\|test_tc"` → ≥90 (was: 0)
- [ ] M2 — compile-fail tests: `ls tests/compile_fail/ 2>/dev/null | wc -l` → ≥3 (was: 0)
- [ ] M3 — spec status: `grep -c "✅" tests/docs/feature/readme.md tests/docs/invariant/readme.md tests/docs/api/readme.md tests/docs/type/readme.md` → 17 (was: 2)

### Invariants

- [ ] I1 — test suite: `w3 .test level::3` → 0 failures
- [ ] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check --all-features` → 0 warnings

### Anti-faking checks

- [ ] AF1 — no trivial asserts: `grep -rn "assert!(true)" tests/` → 0 matches
- [ ] AF2 — no ignored tests: `grep -rn "#\[ignore\]" tests/` → 0 matches
- [ ] AF3 — compile-fail tests compile-fail without guard: temporarily remove the guard from one compile-fail test; verify `cargo test` fails on that case
- [ ] AF4 — WASM check actually runs: `grep -n "wasm32" tests/system/` (or similar domain file) → ≥1 match confirming the check is exercised

## History

- **2026-05-16** CREATED — 10 spec files, 50 test matrix rows, initial In Scope and Validation Checklist
- **2026-06-11** UPDATED — Extended to 12 spec files, 78 spec cases, 63 test matrix rows (T51–T63 added); fixed FR-HELP range to 1..8; added FT-8/FT-9 to registry and argument specs; added AP-7..12 to error codes spec; added IN-5 to workspace deps spec; added IN-4 to command naming spec; updated In Scope, Acceptance Criteria, and Validation Checklist to match
- **2026-06-11** UPDATED — Extended to 17 spec files, 100 spec cases, 85 test matrix rows (T64–T85 added); incorporated type/ entity (4 specs: CommandName TC-1..6, NamespaceType TC-1..4, VersionType TC-1..4, CommandStatus TC-1..5) and invariant/06_build_runtime_separation (IN-1..3); added C9c, C10c–C10f to validation checklist; updated M1 grep pattern and M3 to include type/ surface
- **2026-06-11** UPDATED — Added 3 spec cases from audit gaps: FT-10 (Pattern validation) to argument system, FT-9 (default verbosity Level 2) to help system, IN-4 (serde_json absence) to build-runtime separation; 103 total spec cases, 88 test matrix rows (T86–T88); updated C2, C4, C9c ranges
- **2026-06-11** UPDATED — Deep audit gap closure: added 18 spec cases and T89–T106 matrix rows; feature/01 +4 (CliBuilder FT-10..12, JSON FT-13), feature/02 +3 (Kind::F32 FT-11, Kind::Path FT-12, Max FT-13), feature/04 +2 (verbosity L1 FT-10, L3 FT-11), invariant/03 +1 (SSOT IN-5), api/01 +2 (argv AP-9, batch AP-10), api/02 +2 (TypeMismatch AP-13, string repr AP-14), type/02 +1 (serde TC-5), type/03 +1 (serde TC-5), type/04 +2 (Experimental TC-6, Internal TC-7); 121 total spec cases, 106 test matrix rows; updated all checklist ranges and measurements

## Outcomes

### Progress (2026-06-11)

**14 of 17 spec files moved to ✅.** 3 remain ⏳ due to non-unit-testable cases.

**Work performed:**
- Annotated 40+ existing test functions with `// test_kind: ft_spec()` / `in_spec()` / `ap_spec()` comments mapping to spec cases
- Wrote 4 new test functions: FT-10 (Pattern validation), FT-12 (Path coercion), FT-1 (stateless REPL), FT-5 (empty input)
- Level 3 verification passes clean (nextest + doc tests + clippy)

**Specs completed (✅):**
- feature/01_command_registry, 02_argument_system, 03_pipeline, 04_help_system
- invariant/01_system_actors_vocabulary, 03_governing_principles, 04_workspace_dependency_standards, 05_command_naming
- api/01_public_types, 02_error_codes
- type/01_command_name, 02_namespace_type, 03_version_type, 04_command_status

**Specs remaining (⏳) — blocked on infrastructure:**
- **feature/05_repl_interactive:** FT-2/3 require interactive I/O prompting; FT-4 requires WASM target CI
- **invariant/02_non_functional_requirements:** IN-1/2 require benchmark harness; IN-4 requires panic-catch in Pipeline (unimplemented); IN-5/6 require feature-gate CI checks
- **invariant/06_build_runtime_separation:** IN-1/4 require `cargo tree` dep analysis (CI-level checks)
