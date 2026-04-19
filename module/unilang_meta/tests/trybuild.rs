//! Trybuild UI test harness for `unilang_meta` proc-macro.
//!
//! ## Test Matrix
//!
//! | ID  | File | Type | What is tested |
//! |-----|------|------|----------------|
//! | T01 | ui/01_basic_command_compiles.rs         | pass         | Basic attribute parsing, no params |
//! | T02 | ui/02_argument_inference_compiles.rs    | pass         | String/bool/Option<i64> type mapping |
//! | T03 | ui/03_wrapper_generation_compiles.rs    | pass         | Wrapper fn signature, argument marshalling |
//! | T04 | ui/04_generates_full_definition.rs      | pass         | End-to-end: register fn returns correct CommandDefinition |
//! | T05 | ui/05_missing_name_fails.rs             | compile_fail | Missing `name` attribute produces error |
//! | T06 | ui/06_unsupported_type_fails.rs         | compile_fail | `Vec<String>` param produces error |
//! | T07 | ui/07_integer_and_path_types.rs         | pass         | i32/u64/u32/usize/isize → Kind::Integer; PathBuf → Kind::Path |
//! | T08 | ui/08_optional_type_variants.rs         | pass         | Option<String/bool/PathBuf>: optional=true, runtime extraction |
//! | T09 | ui/09_defaults_and_namespace.rs         | pass         | description defaults to name; namespace defaults to "" |
//! | T10 | ui/10_multiple_commands.rs              | pass         | Two macros in same file; non-colliding identifiers |
//! | T11 | ui/11_wrong_value_type_returns_err.rs   | pass         | Wrong Value variant → Err (not panic) for required arg |
//! | T12 | ui/12_unknown_attribute_fails.rs        | compile_fail | Unknown attribute key produces error |
//! | T13 | ui/13_applied_to_struct_fails.rs        | compile_fail | Macro on struct produces error |
//! | T14 | ui/14_option_unsupported_inner_fails.rs | compile_fail | Option<Vec<String>> unsupported inner type |
//! | T15 | ui/15_reference_param_fails.rs          | compile_fail | `&str` reference param produces error |
//! | T16 | ui/16_hint_value_stored.rs              | pass         | hint attr value accessible via def.hint() |
//! | T17 | ui/17_optional_wrong_type_is_none.rs    | pass         | Optional + wrong Value type → None, Ok (not Err) |
//! | T18 | ui/18_integer_bool_pathbuf_runtime.rs   | pass         | Runtime extraction: i32/usize/bool/PathBuf via wrapper |
//! | T19 | ui/19_f64_param_fails.rs                | compile_fail | f64 param produces error |
//! | T20 | ui/20_i8_param_fails.rs                 | compile_fail | i8 param produces error |
//! | T21 | ui/21_tuple_param_fails.rs              | compile_fail | Tuple param produces error |
//! | T22 | ui/22_register_fn_idempotent.rs         | pass         | Register fn returns same pointer on repeated calls |

#[ test ]
fn ui_tests()
{
  let t = trybuild::TestCases::new();
  t.pass( "tests/ui/01_basic_command_compiles.rs" );
  t.pass( "tests/ui/02_argument_inference_compiles.rs" );
  t.pass( "tests/ui/03_wrapper_generation_compiles.rs" );
  t.pass( "tests/ui/04_generates_full_definition.rs" );
  t.compile_fail( "tests/ui/05_missing_name_fails.rs" );
  t.compile_fail( "tests/ui/06_unsupported_type_fails.rs" );
  t.pass( "tests/ui/07_integer_and_path_types.rs" );
  t.pass( "tests/ui/08_optional_type_variants.rs" );
  t.pass( "tests/ui/09_defaults_and_namespace.rs" );
  t.pass( "tests/ui/10_multiple_commands.rs" );
  t.pass( "tests/ui/11_wrong_value_type_returns_err.rs" );
  t.compile_fail( "tests/ui/12_unknown_attribute_fails.rs" );
  t.compile_fail( "tests/ui/13_applied_to_struct_fails.rs" );
  t.compile_fail( "tests/ui/14_option_unsupported_inner_fails.rs" );
  t.compile_fail( "tests/ui/15_reference_param_fails.rs" );
  t.pass( "tests/ui/16_hint_value_stored.rs" );
  t.pass( "tests/ui/17_optional_wrong_type_is_none.rs" );
  t.pass( "tests/ui/18_integer_bool_pathbuf_runtime.rs" );
  t.compile_fail( "tests/ui/19_f64_param_fails.rs" );
  t.compile_fail( "tests/ui/20_i8_param_fails.rs" );
  t.compile_fail( "tests/ui/21_tuple_param_fails.rs" );
  t.pass( "tests/ui/22_register_fn_idempotent.rs" );
}
