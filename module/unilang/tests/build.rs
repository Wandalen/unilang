//! Build Domain Tests
//!
//! All tests related to build-time code generation: YAML extraction,
//! PHF generation, and compile-time static registry construction.

mod build {
  mod helpers_hint_generator;
  mod helpers_type_analyzer;
  mod validation;
  mod phf_codegen_no_leaked_dep;
  mod dependency_standards;
  mod build_runtime_separation;
}
