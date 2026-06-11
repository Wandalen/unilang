//! Data Domain Tests
//!
//! All tests related to data models: serialization, validation,
//! error handling, and type systems.

mod data {
  mod api_consistency;
  mod category_field_backward_compat;
  mod category_field_codegen;
  mod category_field_conversion;
  mod category_field_edge_cases;
  mod category_field_unit;
  mod command_definition;
  mod config_extraction;
  mod data_model_features;
  mod error_handling;
  mod get_string_normalized;
  mod loader;
  mod static_data;
  mod static_data_auto_help;
  mod static_data_category;
  mod types;
  mod validated_command_name;
  mod validated_namespace;
  mod validated_version_status;
}