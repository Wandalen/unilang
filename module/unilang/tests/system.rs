//! System Domain Tests
//!
//! All tests related to cross-cutting system concerns: end-to-end workflows,
//! API compatibility, external usage patterns, and comprehensive integration.

mod system {
  mod api_compatibility;
  mod argv_api;
  mod comprehensive_workflow;
  mod end_to_end;
  mod external_usage;
  mod multi_yaml_system;
  mod vocabulary_enforcement;
  mod nfr_sensitive_data;
  mod nfr_platform;
  mod nfr_robustness;
  mod nfr_performance;
  mod nfr_modularity;
  mod invariant_03_governing_principles;
}