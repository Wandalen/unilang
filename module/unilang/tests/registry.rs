//! Registry Domain Tests
//!
//! All tests related to registry management: static/dynamic registry,
//! command lookup, and performance metrics.

mod registry {
  mod command_loader_build_time;
  mod command_loader_error;
  #[ cfg( feature = "json_parser" ) ]
  mod command_loader_json;
  mod command_loader_yaml;
  mod debug;
  mod duplicate_detection;
  mod feature_parity;
  mod multi_yaml_conflict_detection;
  mod phf_map_functionality;
  mod phf_reexport;
  mod registration_error_handling;
  mod registry_basic;
  #[ cfg( feature = "json_parser" ) ]
  mod rust_dsl_inline_closure;
  mod static_const_constructor;
  mod static_registry;
  mod static_registry_conversion;
  mod validation_enforcement;
}