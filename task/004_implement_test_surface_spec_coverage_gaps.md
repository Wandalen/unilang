# Implement Rust tests for 64 test-surface spec cases added since task 002 (coverage gap)

## Execution State

- **Executor Type:** any
- **filed_by:** claude-sonnet-5
- **actor:** null
- **started_at:** null
- **expires_at:** null
- **round:** 1
- **state:** 🎯 (Verified)
- **closes:** null
- **unit_type:** module
- **unit:** lib/yrd_core/unilang/dev/module/unilang
- **validated_by:** null
- **validation_date:** null

## Goal

Implement Rust test functions for the 64 test-surface spec cases added across 15 spec files during the current documentation gap-filling session, so that every currently-uncovered `AP-`/`TC-`/`FT-`/`IN-` case has a passing, non-trivial test citing its case ID, verified by `w3 .test level::3` passing with zero failures. (Motivated: task 002 closed full test-surface coverage on 2026-06-13 for 121 spec cases across 17 spec files; this session's documentation pass grew 15 of those files with 64 additional cases — `api/01_public_types.md` AP-11..19, `api/02_error_codes.md` AP-15..16, `type/01_command_name.md` TC-7..11, `type/04_command_status.md` TC-8..14, `feature/01_command_registry.md` FT-14..24, `feature/02_argument_system.md` FT-14..26, `feature/03_pipeline.md` FT-6..8, `feature/04_help_system.md` FT-14..15, `feature/05_repl_interactive.md` FT-6..7, `invariant/01_system_actors_vocabulary.md` IN-4..5, `invariant/02_non_functional_requirements.md` IN-7, `invariant/03_governing_principles.md` IN-6..7, `invariant/04_workspace_dependency_standards.md` IN-6..8, `invariant/05_command_naming.md` IN-3b, `invariant/06_build_runtime_separation.md` IN-5 — confirmed via exhaustive grep sweep against every `test_kind: *_spec(...)` annotation in `tests/` returning zero matches for all 64 new IDs; Observable: 63 new test functions appear across `tests/api/`, `tests/data/`, `tests/registry/`, `tests/semantic/`, `tests/pipeline/`, `tests/help/`, `tests/system/`, `tests/build/`, plus 1 existing WASM-bridge test group in `examples/wasm-repl/tests/wasm.rs` gains its missing `FT-7` citation (functionally already covered, citation-only gap); Scoped: implements exactly the 64 cases listed in In Scope — no changes to `tests/docs/` spec content, no new public API, no refactoring of already-passing tests beyond required citations; Testable: `w3 .test level::3` exits 0 with all new tests passing, and all 15 affected `tests/docs/{api,feature,invariant,type}/readme.md` Overview Table rows show `✅` instead of the `⏳` this session set them to.)

## Null Hypothesis

"The existing test suite already exercises these 64 behaviors under different names, making new tests redundant."

**Refuted:** An exhaustive grep sweep (`grep -rnE "ap_spec\(AP-1[5-9]\)|tc_spec\(TC-(8|9|10|11)\)|...|in_spec\(IN-3b\)|in_spec\(IN-[78]\)" tests/*/*.rs tests/*.rs`) against every domain test file returned zero matches for all 64 new case IDs. `git status --porcelain -- tests/` shows 13 `.rs` files as modified, but inspection confirms these are all trivial single-line doc-comment fixes (spec filename references updated from the old 3-digit prefix to the current 2-digit prefix, e.g. `002_...md` → `02_...md`) with no test logic or assertions changed — consistent with `doc_tsk`'s documentation-only mandate for this session. The one partial exception — `feature/05_repl_interactive.md` FT-7 — has three existing `#[wasm_bindgen_test]` functions in `examples/wasm-repl/tests/wasm.rs` whose assertions already satisfy the spec, but they carry no `test_kind` citation and run under a separate WASM test runner outside the standard `cargo nextest` suite; this task closes that citation gap rather than duplicating test logic.

**Verification Gate finding (IN-3b/T63):** independent adversarial review found `tests/validation/core.rs::test_validate_namespace_core_invalid_missing_dot()` already asserts `validate_namespace_core("session").is_err()` with a dot-prefix error message — functionally close to IN-3b's Given/When/Then. It carries no `test_kind`/`in_spec` citation and does not assert the "`compute_full_name()` never reached" half of IN-3b, so it is not a full duplicate. T63's Delivery Requirements must extend and cite this existing test rather than writing a new one from scratch.

## In Scope

- **`tests/api/public_types.rs`** — AP-11..19 from `tests/docs/api/01_public_types.md` (9 cases): `StaticArgumentAttributes`/`StaticKind`/`StaticValidationRule`/`StaticArgumentDefinition` → non-static conversions, `StaticCommandMap` get/contains_key/len/is_empty, `UNILANG_VERBOSITY` env var, `UNILANG_HELP_HIDE_VERSION`, `VerifiedCommand` missing-argument extraction, `ConfigMap` typed extraction
- **`tests/api/error_codes.rs`** — AP-15..16 from `tests/docs/api/02_error_codes.md` (2 cases): `ErrorCode` `Debug` derive, non-exhaustive/wildcard forward-compat matching
- **`tests/data/validated_command_name.rs`** — TC-7..11 from `tests/docs/type/01_command_name.md` (5 cases): `Display`, `Serialize`, valid-deserialize, `into_inner`, `PartialEq`/`Eq`
- **`tests/data/validated_version_status.rs`** — TC-8..14 from `tests/docs/type/04_command_status.md` (7 cases): `Default`, `from_str_lossy`, `Display` (simple + Deprecated), `"stable"` alias, unrecognized-string default, simple `"deprecated"` string
- **`tests/registry/`** (extend existing files per one-second test — candidates: `static_registry.rs`, `command_loader_yaml.rs`, `command_loader_json.rs`, `validation_enforcement.rs`, `duplicate_detection.rs`) — FT-14..24 from `tests/docs/feature/01_command_registry.md` (11 cases): name-transform guarantee, YAML Format 1/2 equivalence, build-time duplicate-name rejection, `multiple:true`+non-List rejection, build-time empty-version rejection, `StaticCommandRegistry` auto-help companion, static→`CommandRegistry` conversion registers global `.help`, `register()` vs `register_with_routine()` error-type divergence, `build()` swallows vs `build_checked()` propagates, help-companion metadata (alias/hidden/priority/no-recursive-help), `status("deprecated")` structured metadata
- **`tests/semantic/`** (extend existing files per one-second test — candidates: `argument_binding.rs`, `centralized_validation.rs`, `parameter_storage_validation.rs`) — FT-14..26 from `tests/docs/feature/02_argument_system.md` (13 cases): `Kind::Enum`, `Kind::File`/`Directory`, `Kind::Url`/`DateTime`, `Kind::Pattern`, `Kind::List`/`Map` delimiters, `Kind::JsonString`/`Object` (requires `json_parser` feature), `ValidationRule::Min`, `MaxLength`, `MinItems`, sensitive-attribute redaction, interactive-attribute distinct error, `VerifiedCommand` typed extraction methods, `get_string_normalized` trimming
- **`tests/pipeline/pipeline_core.rs`** — FT-6..8 from `tests/docs/feature/03_pipeline.md` (3 cases): `HelpRequested` interception → successful output, `process_command_from_argv_simple`, empty-list batch/sequence boundary
- **`tests/help/`** (extend existing files per one-second test — candidate: `verbosity.rs`) — FT-14..15 from `tests/docs/feature/04_help_system.md` (2 cases): `HelpVerbosity::from_level` capping above 4, `UNILANG_HELP_HIDE_VERSION`
- **`tests/pipeline/pipeline_core.rs`** — FT-6 from `tests/docs/feature/05_repl_interactive.md` (1 case): interactive-argument retry round-trip (resubmission after `FT-2`'s signal)
- **`examples/wasm-repl/tests/wasm.rs`** — FT-7 from `tests/docs/feature/05_repl_interactive.md` (1 case, citation-only): add `// test_kind: ft_spec(FT-7)  [feature/05_repl_interactive]` to `test_command_execution`, `test_invalid_command`, and `test_help_command` (their existing assertions already satisfy the spec's Given/When/Then)
- **`tests/system/vocabulary_enforcement.rs`** — IN-4..5 from `tests/docs/invariant/01_system_actors_vocabulary.md` (2 cases): `"Command Registry"` synonym-absence check (`CommandStore`/`CommandCache`/`CommandDatabase`), `"Kind"` synonym-absence check (`ArgType`/`DataType`/`ValueType`)
- **`tests/system/nfr_platform.rs`** — IN-7 from `tests/docs/invariant/02_non_functional_requirements.md` (1 case): `cargo check --target wasm32-unknown-unknown --no-default-features --features enabled` subprocess check
- **`tests/system/invariant_03_governing_principles.rs`** — IN-6..7 from `tests/docs/invariant/03_governing_principles.md` (2 cases): Explicit Dependencies (missing required argument rejected with actionable error), Explicit Command Naming (no dot prefix rejected, no silent auto-correction)
- **`tests/build/dependency_standards.rs`** — IN-6..8 from `tests/docs/invariant/04_workspace_dependency_standards.md` (3 cases): `--no-default-features` build has zero external deps, workspace manifest has no `features` lists, `enabled` feature uses `dep:name` syntax
- **`tests/validation/core.rs`** — IN-3b from `tests/docs/invariant/05_command_naming.md` (1 case, EXTEND not create): `test_validate_namespace_core_invalid_missing_dot()` already asserts the dot-prefix rejection half; extend it to also assert `compute_full_name()` is never reached for this input, and add the `test_kind` citation — do not write a new function
- **`tests/build/build_runtime_separation.rs`** — IN-5 from `tests/docs/invariant/06_build_runtime_separation.md` (1 case): build-time codegen produces valid static data from a real YAML manifest through the actual `build/main.rs` → `build/codegen.rs` pipeline (not a hand-constructed `const`)
- Update all 15 affected `tests/docs/{api,feature,invariant,type}/readme.md` Overview Table rows from `⏳` back to `✅` once their tests pass

## Out of Scope

- `type/02_namespace_type.md` and `type/03_version_type.md` — no new cases added this session; already fully covered by task 002; both remain `✅` and untouched
- Editing `tests/docs/` spec file content — already consistent (documentation phase of this session already completed this); this task implements against the existing spec text only
- The 3-way `compute_full_name()` code duplication across `src/validation_core.rs`, `src/multi_yaml/aggregator/conflict.rs`, and `src/multi_yaml/aggregator/core.rs` — a code-consolidation concern, not a test-coverage gap; not part of this task
- `task/003_fix_semantic_analyzer_empty_path_bypasses_named_argument_validation.md` — a distinct, already-open bug-fix task; not touched or merged into this task
- Modifying any already-passing test function beyond adding the single required `FT-7` citation in `examples/wasm-repl/tests/wasm.rs`
- New public API additions or feature implementations — tests only exercise existing API surface
- Setting up a `wasm-pack test` / `wasm-bindgen-test-runner` CI step — `FT-7`'s existing coverage remains reachable only via the separate WASM test runner, consistent with how `FT-4`/`IN-7`'s WASM compile checks are already handled (subprocess `cargo check`, not a full WASM test execution)

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Minimum rulebook references: `code_design.rulebook.md`, `codebase_hygiene.rulebook.md`, `test_organization.rulebook.md`, `code_style.rulebook.md`
- Custom codestyle per `code_style.rulebook.md` — 2-space indents, no `cargo fmt`
- Tests must use real implementations — no mocking, no `assert!(true)`, no `#[ignore]`
- Each test function must have a doc comment citing its spec case: `// test_kind: <prefix>_spec(<CASE-ID>)  [<spec-file>]`, matching the existing annotation convention in `tests/api/error_codes.rs` and other domain files
- Apply the one-second test against existing files in each target directory before creating any new file — extend an existing file whose responsibility matches, per `codebase_hygiene.rulebook.md`
- FT-19 (`Kind::JsonString`/`Object`) must be gated with `#[cfg(feature = "json_parser")]`
- No test may silently pass due to missing tokens, missing registry entries, or unconfigured environments

## Delivery Requirements

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note test file placement, naming (`snake_case`), and doc comment format constraints
2. **Read spec files** — re-read all 15 spec files listed in In Scope as the authoritative test contract; the Test Matrix below is a summary, not a substitute for the full Given/When/Then text
3. **Read existing domain files** — for each target directory, read existing files to identify the correct one-second-test extension point before adding new functions; do not create a new file if an existing one covers the same responsibility
4. **Implement new test functions** — one function per case, named `test_<case_id_snake>()` (e.g., `test_ap11_static_argument_attributes_conversion()`), each with the `test_kind` citation comment
5. **Add FT-7 citations** — add `// test_kind: ft_spec(FT-7)  [feature/05_repl_interactive]` to the three existing wasm-repl test functions identified in In Scope; do not modify their assertions
6. **Green state** — `w3 .test level::3` must pass with zero failures and zero warnings before proceeding
7. **Update spec status** — in each of the 15 affected `tests/docs/*/readme.md` files, change `⏳` back to `✅` for the row whose tests now pass
8. **Walk Validation Checklist** — check every item; every answer must be YES
9. **Update task status** — set ✅ in `task/readme.md`, move file to `task/completed/`

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | `StaticArgumentAttributes::new().with_optional(true).with_multiple(true).with_default("fallback").with_sensitive(true).with_interactive(true)` then `ArgumentAttributes::from(&static_attrs)` | AP-11 conversion | All 5 fields match: `optional`, `multiple`, `default == Some("fallback")`, `sensitive`, `interactive` all `true`/matching |
| T02 | `StaticKind::List(&StaticKind::Integer, Some(','))` and `StaticKind::Enum(&["red","green","blue"])` via `Kind::from(&static_kind)` | AP-12 conversion | List converts preserving delimiter; Enum converts to `Vec<String>` of the 3 values |
| T03 | 6 `StaticValidationRule` variants (`Min`, `Max`, `MinLength`, `MaxLength`, `Pattern`, `MinItems`) via `ValidationRule::from(&static_rule)` | AP-13 conversion | Each converts to matching `ValidationRule` variant with identical parameter |
| T04 | `StaticArgumentDefinition::new("count", StaticKind::Integer, "A count value").with_attributes(...)` via `ArgumentDefinition::from(&static_arg)` | AP-14 conversion | `name == "count"`, `kind == Kind::Integer`, `description` matches, `attributes.optional == true` |
| T05 | `StaticCommandMap::from_phf_internal` with one entry `.greet`; `get`/`contains_key`/`len`/`is_empty` called | AP-15 lookups | `get(".greet")` → `Some`; `contains_key(".greet")` → true, `(".missing")` → false; `len() == 1`; `is_empty() == false` |
| T06 | `UNILANG_VERBOSITY=2` set before CLI binary startup | AP-16 env var | Debug-level (2) logging applied; distinct from `UNILANG_HELP_VERBOSITY` |
| T07 | `UNILANG_HELP_HIDE_VERSION=1` with a command that has a `version` field | AP-17 env var | Help output omits version line; unset restores it |
| T08 | `VerifiedCommand` with no `"count"` bound (optional, omitted) | AP-18 missing-arg extraction | `get_integer` → `None`; `has_argument` → `false`; `get_value` → `None` |
| T09 | `ConfigMap<&str>` (feature `json_parser`) with `"port": 8080`, `"enabled": true` | AP-19 typed extraction | `u32` extraction for `"port"` → `Ok(8080u32)`; `bool` extraction for `"enabled"` → `Ok(true)` |
| T10 | `format!("{:?}", error_code)` for an `ErrorCode` variant | AP-15 (api/02) Debug derive | Produces a valid `Debug` string identifying the variant |
| T11 | `match error_code { ErrorCode::CommandNotFound => ..., _ => ... }` against a future/unknown variant pattern | AP-16 (api/02) forward-compat | Wildcard arm compiles and matches; no exhaustiveness break on variant addition |
| T12 | `CommandName` from `".build"` formatted via `format!("{}", name)` | TC-7 Display | Produces `".build"` identical to `as_str()` |
| T13 | `CommandName` from `".video.convert"` via `serde_json::to_string(&name)` | TC-8 Serialize | Returns `Ok("\".video.convert\"")` — plain string, not a map |
| T14 | JSON `"\".hello\""` via `serde_json::from_str::<CommandName>` | TC-9 valid deserialize | Returns `Ok(name)` with `as_str() == ".hello"` |
| T15 | `CommandName` from `".build"` via `into_inner()` | TC-10 into_inner | Returns owned `String == ".build"`; value consumed |
| T16 | Two `CommandName` values both from `".build"` compared with `==` | TC-11 PartialEq/Eq | Returns `true` |
| T17 | `CommandStatus::default()` | TC-8 Default | Returns `CommandStatus::Active` |
| T18 | `from_str_lossy("experimental")`, `("internal")`, `("stable")`, `("unknown")` | TC-9 from_str_lossy | Returns `Experimental`, `Internal`, `Active`, `Active` respectively |
| T19 | `format!("{}", status)` for `Active`, `Experimental`, `Internal` | TC-10 Display simple | Produces `"active"`, `"experimental"`, `"internal"` |
| T20 | `format!("{}", status)` for `Deprecated{reason:"use .new", since:Some("2.0"), replacement:Some(".new")}` | TC-11 Display Deprecated | Produces `"deprecated (since 2.0): use .new → .new"` |
| T21 | JSON `"\"stable\""` via `serde_json::from_str::<CommandStatus>` | TC-12 stable alias | Produces `CommandStatus::Active` |
| T22 | JSON `"\"bogus\""` via `serde_json::from_str::<CommandStatus>` | TC-13 unrecognized default | Produces `CommandStatus::Active`, no error |
| T23 | JSON `"\"deprecated\""` (plain string, no map) via `serde_json::from_str::<CommandStatus>` | TC-14 simple deprecated | Produces `Deprecated` with `reason==""`, `since==None`, `replacement==None` |
| T24 | `CommandDefinition` named `.chat`, empty namespace, registered via `register_with_routine` | FT-14 name-transform guarantee | Found under exactly `.chat`; no transformed/alternate spelling created |
| T25 | Same command via YAML Format 1 (`name:".session.list"`) and Format 2 (`name:"list", namespace:".session"`) loaded separately | FT-15 format equivalence | Both registries expose `.session.list`; definitions equivalent |
| T26 | YAML manifest with two entries both resolving to `.dup.command` | FT-16 build-time duplicate rejection | Build error showing both occurrences; not silently deduped |
| T27 | `CommandDefinition` with `attributes.multiple==true` and `kind==Kind::String` (not List) via `CommandRegistry::register` | FT-17 multiple+non-List rejection | Returns `Err(Error::Registration(_))`; command not registered |
| T28 | YAML manifest entry with valid dot-prefixed name but `version: ""` | FT-18 build-time empty-version rejection | Build error stating version cannot be empty |
| T29 | `StaticCommandRegistry` with `.report` registered, `auto_help_enabled` true; query `.report.help` | FT-19 auto-help companion | Returns `Some(def)` for generated help command |
| T30 | Fresh `StaticCommandRegistry` converted via `From<StaticCommandRegistry>`; query `.help` | FT-20 global help registration | Returns `Some(def)` even though no command explicitly registered `.help` |
| T31 | `.dup` already registered; `register()` and `register_with_routine()` both called again with `.dup` | FT-21 error-type divergence | `register()` → `Err(Error::Registration(_))`; `register_with_routine()` → `Err(Error::Execution(ErrorData))` with `CommandAlreadyExists` |
| T32 | `CommandRegistryBuilder` with two `command_with_routine` calls using the same name; `.build()` vs `.build_checked()` on equivalent copies | FT-22 build() vs build_checked() | `.build()` silently keeps only first, no error; `.build_checked()` returns `Err` referencing the duplicate |
| T33 | `.example` registered with `auto_help_enabled` true; inspect generated `.example.help` companion | FT-23 help companion metadata | alias `.example.h`, `hidden_from_list()==true`, `priority()==999`, `auto_help_enabled()==false` |
| T34 | `CommandDefinition::former().name(".old").description("Old command").status("deprecated")` with message set | FT-24 deprecation metadata | `status()` returns `Deprecated{reason,..}` matching message; `is_deprecated()==true`, `is_active()==false` |
| T35 | `.cmd` with `Kind::Enum(["low","medium","high"])`; input `level::extreme` then `level::medium` | FT-14 (feature/02) Enum | `extreme` → `UNILANG_ARGUMENT_TYPE_MISMATCH`; `medium` → `Value::Enum("medium")` no error |
| T36 | `Kind::File` on existing file / existing directory / nonexistent path | FT-15 File/Directory | File binds `Value::File`; directory case → type-mismatch (expected file, got dir); nonexistent → type-mismatch (not found); symmetric for `Kind::Directory` |
| T37 | `Kind::Url` with `https://api.example.com/v1`; `Kind::DateTime` with `2024-01-15T10:30:00+00:00` | FT-16 Url/DateTime | Both parse to typed `Value::Url`/`Value::DateTime`; malformed input → type-mismatch, not panic |
| T38 | `Kind::Pattern` with `^[a-z]+$`; malformed `[unclosed` | FT-17 Pattern | Valid regex compiles into `Value::Pattern` matching source; invalid → type-mismatch, not panic |
| T39 | `Kind::List(String,None)` with `a,b,c`; `Kind::List(String,Some(';'))` with `a;b;c`; `Kind::Map(...)` with `k1=v1,k2=v2` | FT-18 List/Map delimiters | Default `,` and custom `;` delimiters both produce correct 3-element list; Map produces correct 2-entry map |
| T40 | `Kind::JsonString`/`Kind::Object` with `{"a":1}`; malformed `{not json}` (feature `json_parser`) | FT-19 JsonString/Object | Valid JSON binds correctly for both kinds; malformed → type-mismatch, not panic |
| T41 | `Kind::Integer` with `ValidationRule::Min(0.0)`; input `age::-1` | FT-20 Min validation | `UNILANG_VALIDATION_RULE_FAILED` — below minimum |
| T42 | `String` with `ValidationRule::MaxLength(4)`; input `code::abcdef` | FT-21 MaxLength validation | `UNILANG_VALIDATION_RULE_FAILED` — exceeds max length |
| T43 | `Kind::List` with `ValidationRule::MinItems(2)`; input `tags::solo` | FT-22 MinItems validation | `UNILANG_VALIDATION_RULE_FAILED` — fewer than 2 items |
| T44 | `String` argument `"password"` marked `sensitive=true` with `MinLength(8)`; input `password::abc` (fails validation) | FT-23 sensitive redaction | Error message contains `"[REDACTED]"` (or equivalent); does NOT contain literal `"abc"` |
| T45 | Required argument `"token"` marked `interactive=true`; no value provided | FT-24 interactive distinct error | `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`, distinct from generic `UNILANG_ARGUMENT_MISSING` |
| T46 | `VerifiedCommand` with `"name"` (String), `"count"` (Integer), no `"missing"` | FT-25 typed extraction | `get_string`/`require_string` correct for `"name"`; wrong-type and missing-key cases return `None`/`Err(ArgumentTypeMismatch)` per spec; `has_argument`/`get_value` behave correctly for all 3 |
| T47 | `Value::String("  Alice  ")` bound to `"name"`; whitespace-only value | FT-26 normalized extraction | `get_string_normalized`/`require_string_normalized` return trimmed `"Alice"`; whitespace-only → `Some("")`/`Ok("")` not `None`/error |
| T48 | `Pipeline` with registered command; input triggering `HelpRequested` signal (e.g. `"."`) | FT-6 (feature/03) HelpRequested interception | `result.success==true`; `result.error.is_none()`; output contains formatted help text |
| T49 | `Pipeline` with `.test message::` arg; argv `[".test","message::world"]` via `process_command_from_argv_simple` | FT-7 (feature/03) simple argv wrapper | `result.success==true`; `result.outputs[0].content=="world"`; behavior identical to explicit-context variant |
| T50 | `pipeline.process_batch(&[], ctx)` and `pipeline.process_sequence(&[], ctx)` on empty slice | FT-8 (feature/03) empty-list boundary | `total_commands==0`, all counts 0, `results.is_empty()==true`, `success_rate()==0.0`; no panic/NaN; identical shape for both |
| T51 | `HelpVerbosity::from_level(4)`, `(5)`, `(100)` | FT-14 (feature/04) from_level capping | All three return `HelpVerbosity::Comprehensive`; no panic |
| T52 | `.greet` with version set; `UNILANG_HELP_HIDE_VERSION=1` vs unset | FT-15 (feature/04) hide-version | Version string absent when set; present when unset |
| T53 | `.greet` with required interactive `"name"`; first call `.greet` returns `requires_interactive_input()==true`; resubmit `.greet name::alice` | FT-6 (feature/05) interactive retry round-trip | Second call: `result.success==true`, `requires_interactive_input()==false`, output contains `"Hello, alice!"` |
| T54 | `UniLangWasmRepl::new()`; `execute_command(".demo.echo text::hello")`; `execute_command(".invalid.command")`; `get_help()` | FT-7 (feature/05) WASM REPL glue — citation only | Existing `test_command_execution`/`test_invalid_command`/`test_help_command` assertions already satisfy spec; add `test_kind` citation, no new assertions needed |
| T55 | Codebase searched for `"CommandStore"`, `"CommandCache"`, `"CommandDatabase"` as type names | IN-4 (invariant/01) Command Registry synonym | Zero occurrences; `CommandRegistry`/`StaticCommandMap` used exclusively |
| T56 | Codebase searched for `"ArgType"`, `"DataType"`, `"ValueType"` as type names | IN-5 (invariant/01) Kind synonym | Zero occurrences; `Kind` used exclusively |
| T57 | `cargo check --target wasm32-unknown-unknown --no-default-features --features enabled` | IN-7 (invariant/02) NFR-PLATFORM-1 | Exits 0, zero compiler errors |
| T58 | Command `.needs_arg` with required (`optional:false`) argument, invoked without it | IN-6 (invariant/03) Explicit Dependencies | `Err(ErrorCode::ArgumentMissing)` naming the missing argument |
| T59 | `CommandName::new("build")` (no leading dot) | IN-7 (invariant/03) Explicit Command Naming | Returns `Err`, not a silently auto-corrected `.build` |
| T60 | `cargo build -p unilang --no-default-features`; inspect `cargo tree --edges=normal` | IN-6 (invariant/04) zero external deps | Zero external dependency crates compiled/linked; only `unilang` present |
| T61 | Workspace `Cargo.toml` `[workspace.dependencies]` section inspected | IN-7 (invariant/04) no features lists | No entry declares a `features = [...]` list |
| T62 | `unilang` crate `Cargo.toml` `[features]` `enabled` activation list inspected | IN-8 (invariant/04) dep:name syntax | Every activated dependency uses `dep:name` syntax; no bare crate name |
| T63 | YAML entry `namespace: "math"` (no dot), `name: "add"` processed by `validate_namespace_core()` ahead of `compute_full_name()`; EXTEND existing `test_validate_namespace_core_invalid_missing_dot()` in `tests/validation/core.rs`, do not create a new function | IN-3b (invariant/05) namespace rejected pre-construction | Returns `Err` referencing missing dot prefix (already asserted); add assertion that `compute_full_name()` never reached for this entry, plus the `test_kind` citation |
| T64 | Real YAML manifest processed through actual `build/main.rs` → `build/codegen.rs` pipeline (`static_registry` feature); generated `OUT_DIR/static_commands.rs` included and read | IN-5 (invariant/06) real codegen validity | Generated static data structurally matches source manifest; valid and accessible without runtime parsing |

## Acceptance Criteria

- All 64 Test Matrix rows have a corresponding non-trivial passing test function (or, for T54, a spec-case citation added to already-passing assertions; or, for T63, an extension plus citation added to an existing passing test) in the correct domain file
- Every test function has a doc comment `// test_kind: <prefix>_spec(<CASE-ID>)  [<spec-file>]` citing its spec case, matching the existing annotation convention
- `w3 .test level::3` exits with 0 failures and 0 warnings
- All 15 affected `tests/docs/{api,feature,invariant,type}/readme.md` Overview Table rows show `✅`
- No test function contains `assert!(true)`, `unimplemented!()`, `todo!()`, or `#[ignore]`
- No modification to any already-passing test function's assertions beyond the single required `FT-7` citation addition

## Validation

**Execution:** Independent validator (not executor) walks this section per MAAV (`governance/maav.rulebook.md`) — dispatch independent subagents with at least one adversarial mandate; do not self-verify.

### Checklist

Desired answer for every question is YES.

**API test coverage**
- [ ] C1 — Do tests for AP-11..19 exist in `tests/api/public_types.rs`?
- [ ] C2 — Do tests for AP-15..16 exist in `tests/api/error_codes.rs`?

**Type test coverage**
- [ ] C3 — Do tests for TC-7..11 exist in `tests/data/validated_command_name.rs`?
- [ ] C4 — Do tests for TC-8..14 exist in `tests/data/validated_version_status.rs`?

**Feature test coverage**
- [ ] C5 — Do tests for FT-14..24 exist in `tests/registry/`?
- [ ] C6 — Do tests for FT-14..26 exist in `tests/semantic/`?
- [ ] C7 — Do tests for FT-6..8 (pipeline) exist in `tests/pipeline/pipeline_core.rs`?
- [ ] C8 — Do tests for FT-14..15 (help) exist in `tests/help/`?
- [ ] C9 — Does a test for FT-6 (repl retry round-trip) exist in `tests/pipeline/pipeline_core.rs`?
- [ ] C10 — Do the 3 identified `examples/wasm-repl/tests/wasm.rs` functions carry the `FT-7` citation?

**Invariant test coverage**
- [ ] C11 — Do tests for IN-4..5 exist in `tests/system/vocabulary_enforcement.rs`?
- [ ] C12 — Does a test for IN-7 (NFR-PLATFORM-1) exist in `tests/system/nfr_platform.rs`?
- [ ] C13 — Do tests for IN-6..7 exist in `tests/system/invariant_03_governing_principles.rs`?
- [ ] C14 — Do tests for IN-6..8 exist in `tests/build/dependency_standards.rs`?
- [ ] C15 — Is `test_validate_namespace_core_invalid_missing_dot()` in `tests/validation/core.rs` extended with the `compute_full_name()`-unreached assertion and the IN-3b citation (not duplicated as a new function)?
- [ ] C16 — Does a test for IN-5 (invariant/06) exist in `tests/build/build_runtime_separation.rs`?

**Test quality**
- [ ] C17 — Does every new test function cite its spec case in a `test_kind` comment?
- [ ] C18 — Is `assert!(true)` absent from all new/modified test functions?
- [ ] C19 — Is `#[ignore]` absent from all new test functions?

**Spec status updates**
- [ ] C20 — Do all 15 affected `tests/docs/*/readme.md` overview rows show `✅`?

**Out of Scope confirmation**
- [ ] C21 — Are `type/02_namespace_type.md` and `type/03_version_type.md` rows unchanged (still `✅`, untouched)?
- [ ] C22 — Is `tests/docs/` spec content unchanged (only status markers flipped)?
- [ ] C23 — Are no new public API symbols added to `src/`?
- [ ] C24 — Are `src/validation_core.rs`, `src/multi_yaml/aggregator/conflict.rs`, and `src/multi_yaml/aggregator/core.rs` unchanged (compute_full_name() duplication untouched)?
- [ ] C25 — Is `task/003_fix_semantic_analyzer_empty_path_bypasses_named_argument_validation.md` unchanged?
- [ ] C26 — Was no `wasm-pack test` / `wasm-bindgen-test-runner` CI config added (e.g. no new workflow file, no new `[package.metadata]` runner section)?
- [ ] C27 — Beyond the FT-7 citation in `examples/wasm-repl/tests/wasm.rs` and the IN-3b extension in `tests/validation/core.rs`, is every other pre-existing (already-passing, pre-session) test function's body byte-for-byte unchanged (diff shows only new functions added and the two named exceptions modified)?

### Measurements

- [ ] M1 — new test function count: `grep -rc "fn test_ap1[1-9]\|fn test_tc\(7\|8\|9\|10\|11\|12\|13\|14\)\|fn test_ft1[4-9]\|fn test_ft2[0-6]\|fn test_ft6\|fn test_ft7\|fn test_ft8\|fn test_in[4-8]" tests/*/*.rs 2>/dev/null | awk -F: '{s+=$2} END{print s}'` → ≥62 (62 new functions; T54 is a citation-only addition to existing wasm-repl functions, and T63 is an extension of the existing `test_validate_namespace_core_invalid_missing_dot()`, neither counted as new)
- [ ] M2 — spec status: `grep -c "✅" tests/docs/api/readme.md tests/docs/type/readme.md tests/docs/feature/readme.md tests/docs/invariant/readme.md` → 21 total (2 api + 4 type + 5 feature + 6 invariant, with type/02 and type/03 already counted among the 4)
- [ ] M3 — FT-7 citation: `grep -c "test_kind: ft_spec(FT-7)" examples/wasm-repl/tests/wasm.rs` → 3

### Invariants

- [ ] I1 — test suite: `cargo nextest run --all-features` → 0 failures
- [ ] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check --all-features` → 0 warnings
- [ ] I3 — doc tests: `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` → 0 failures

### Anti-faking checks

- [ ] AF1 — no trivial asserts: `grep -rn "assert!(true)" tests/` → 0 matches
- [ ] AF2 — no ignored tests: `grep -rn "#\[ignore\]" tests/` → 0 matches (new/modified files only)
- [ ] AF3 — FT-7 citation is genuine: the 3 cited functions' existing assertions actually check `execute_command`/`get_help` return values (not a relabeling of an unrelated test)
- [ ] AF4 — sensitive redaction (T44) genuinely checks absence of raw value: assertion inspects error message content for the literal input value, not just error presence
- [ ] AF5 — no stubbed tests: `grep -rn "unimplemented!()\|todo!()" tests/` → 0 matches
- [ ] AF6 — IN-3b extension is genuine: `test_validate_namespace_core_invalid_missing_dot()` in `tests/validation/core.rs` contains a real assertion that `compute_full_name()` is never reached (not just a comment claiming so)

## Verification Record

**Date:** 2026-07-05
**Method:** MAAV — 4 independent `general-purpose` subagents dispatched in parallel (Scope Coherence, MOST Goal Quality, Value/YAGNI [adversarial], Implementation Readiness), each reading the task file cold with no prior session context. Author did not self-verify.

**Agent Round 1**
Round: 1/5 · Type: Full · CONVERGED · Agents: 4 · 4/4

| # | Agent | Mandate | Prev | Now | Issues | Key Findings |
|---|-------|---------|------|-----|--------|--------------|
| A1 | Scope Coherence | In Scope / Out of Scope non-empty, coherent, no contradictions; cross-checked sampled file references against the repo | — | ✅ | 2 non-blocking | All 15 sampled spec files and case-ID ranges confirmed real on disk; no In/Out overlap; minor M1 regex looseness noted |
| A2 | MOST Goal Quality | Goal is Motivated/Observable/Scoped/Testable; Null Hypothesis genuinely attempts refutation | — | ✅ | 1 non-blocking | Independently re-ran the citation-absence grep sweep — zero matches confirmed, gap is real; Null Hypothesis's git-status phrasing was slightly overstated (13 `.rs` files show as modified, but all are trivial doc-path fixes, not test-logic changes) |
| A3 | Value/YAGNI (adversarial mandate: disprove necessity) | Actively searched for existing coverage under different names; checked for duplication against tasks 002/003; verified Advisability arithmetic | — | ✅ | 2 non-blocking | Found `tests/validation/core.rs::test_validate_namespace_core_invalid_missing_dot()` already covers half of IN-3b/T63 uncited — task must extend, not duplicate; confirmed task 003 is unrelated scope; confirmed 8×5×7×2=560 |
| A4 | Implementation Readiness | Delivery Requirements executable in TDD order; Test Matrix rows spot-checked against real spec text; target files/dirs verified to exist | — | ✅ | 2 non-blocking | 8 spot-checked Test Matrix rows (T01,T15,T24,T35,T44,T48,T54,T64) matched real spec text; all named target directories and "extend" candidate files confirmed to exist; `kbase` confirmed on PATH |
| **Total** | | | — | ✅ | 7 non-blocking | — |

All 4 dimensions PASSed in this single Full Round with zero Blocking Findings — CONVERGED, no further rounds required.

**Fixes applied post-gate** (cheap, evidence-backed, non-blocking findings folded in before finalizing rather than deferred):
1. Null Hypothesis git-status claim corrected to accurately describe the 13 modified `.rs` files as trivial doc-path citation fixes, not "untouched."
2. T63/IN-3b changed from "create new test" to "extend `test_validate_namespace_core_invalid_missing_dot()` in `tests/validation/core.rs` and add citation" per A3's finding — In Scope, Test Matrix row, and Validation Checklist C15 all updated; M1's expected count corrected from ≥63 to ≥62 accordingly.

## Related Documentation

- `/home/user1/pro/lib/yrd_core/unilang/dev/task/completed/002_implement_test_surface_specs.md` — predecessor task that closed the original 121-case/17-file baseline on 2026-06-13 (`Related: 002`)
- `/home/user1/pro/lib/yrd_core/unilang/dev/module/unilang/task/completed/105_implement_build_runtime_separation_tests.md` — prior narrower precedent for `invariant/06` (IN-1..4), same spec file this task's IN-5 extends
- `/home/user1/pro/lib/yrd_core/unilang/dev/module/unilang/tests/docs/api/01_public_types.md`, `02_error_codes.md`
- `/home/user1/pro/lib/yrd_core/unilang/dev/module/unilang/tests/docs/type/01_command_name.md`, `04_command_status.md`
- `/home/user1/pro/lib/yrd_core/unilang/dev/module/unilang/tests/docs/feature/01_command_registry.md`, `02_argument_system.md`, `03_pipeline.md`, `04_help_system.md`, `05_repl_interactive.md`
- `/home/user1/pro/lib/yrd_core/unilang/dev/module/unilang/tests/docs/invariant/01_system_actors_vocabulary.md`, `02_non_functional_requirements.md`, `03_governing_principles.md`, `04_workspace_dependency_standards.md`, `05_command_naming.md`, `06_build_runtime_separation.md`
- `/home/user1/pro/lib/yrd_core/unilang/dev/module/unilang/docs/api/001_public_types.md` — updated this session with 4 new public types (`StaticArgumentDefinition`, `StaticArgumentAttributes`, `StaticKind`, `StaticValidationRule`) that AP-11..14 exercise
- `/home/user1/pro/lib/yrd_core/unilang/dev/module/unilang/docs/entity/readme.md` — module doc-entity index, rebuilt this session

## History

- **[2026-07-05]** `FILED` — Task filed by claude-sonnet-5. Goal: implement Rust tests for the 64 test-surface spec cases added across 15 spec files during this session's documentation gap-filling pass, closing the coverage gap opened since task 002's 2026-06-13 closure.
- **[2026-07-05]** `UPDATED` — MAAV Verification Gate run: 4 independent parallel subagents (Scope Coherence, MOST Goal Quality, Value/YAGNI adversarial, Implementation Readiness), Round 1 Full, CONVERGED, 0 Blocking Findings. Applied 2 fixes from non-blocking findings: corrected Null Hypothesis git-status phrasing; changed T63/IN-3b from "create new test" to "extend `test_validate_namespace_core_invalid_missing_dot()` in `tests/validation/core.rs`" (M1 count corrected 63→62 accordingly). State set to 🎯 (Verified).
