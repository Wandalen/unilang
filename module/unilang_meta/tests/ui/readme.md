# unilang_meta — Trybuild UI Test Fixtures

Each `.rs` file is a mini-program compiled by `trybuild` to verify the
`#[unilang_meta::command]` proc-macro behaviour.  `.stderr` files are
auto-generated snapshots of expected compiler error output.

## Responsibility Table

| File | Type | What is tested |
|------|------|----------------|
| 01_basic_command_compiles.rs | pass | Basic attribute parsing, zero-param function |
| 02_argument_inference_compiles.rs | pass | String/bool/Option<i64> type mapping |
| 03_wrapper_generation_compiles.rs | pass | Wrapper fn signature and missing-arg error path |
| 04_generates_full_definition.rs | pass | Full pipeline: namespace, description, arguments |
| 05_missing_name_fails.rs | compile_fail | Missing required `name` attribute |
| 05_missing_name_fails.stderr | snapshot | Expected error for T05 |
| 06_unsupported_type_fails.rs | compile_fail | `Vec<String>` param rejects with error |
| 06_unsupported_type_fails.stderr | snapshot | Expected error for T06 |
| 07_integer_and_path_types.rs | pass | i32/u64/u32/usize/isize → Kind::Integer; PathBuf → Kind::Path |
| 08_optional_type_variants.rs | pass | Option<String/bool/PathBuf>: optional=true, runtime extraction |
| 09_defaults_and_namespace.rs | pass | description defaults to name; namespace defaults to "" |
| 10_multiple_commands.rs | pass | Two macros in same file; non-colliding identifiers |
| 11_wrong_value_type_returns_err.rs | pass | Wrong Value variant → Err (not panic) for required arg |
| 12_unknown_attribute_fails.rs | compile_fail | Unknown attribute key `bogus` produces error |
| 12_unknown_attribute_fails.stderr | snapshot | Expected error for T12 |
| 13_applied_to_struct_fails.rs | compile_fail | Macro applied to struct produces error |
| 13_applied_to_struct_fails.stderr | snapshot | Expected error for T13 |
| 14_option_unsupported_inner_fails.rs | compile_fail | Option<Vec<String>> unsupported inner type |
| 14_option_unsupported_inner_fails.stderr | snapshot | Expected error for T14 |
| 15_reference_param_fails.rs | compile_fail | `&str` reference param produces error |
| 15_reference_param_fails.stderr | snapshot | Expected error for T15 |
