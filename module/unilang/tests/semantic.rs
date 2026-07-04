//! Semantic Domain Tests
//!
//! All tests related to semantic analysis: command validation, argument binding,
//! type checking, and multiple parameter collection.

mod semantic {
  mod argument_binding;
  mod auto_categorize_decoupling;
  mod centralized_validation;
  mod command_validation;
  mod empty_path_named_argument;
  mod format_category_name_decoupling;
  mod multiple_parameters;
  mod parameter_storage_validation;
  mod parameter_typo_suggestion;
  mod parser_semantic;
  mod unknown_parameters;
  mod unknown_parameters_edge_cases;
}