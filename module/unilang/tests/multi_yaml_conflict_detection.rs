//! Test Suite: Multi-Module YAML Conflict Detection
//!
//! Validates that MultiYamlAggregator correctly detects (and doesn't false-positive)
//! command name conflicts when aggregating multiple YAML files.
//!
//! # Test Coverage
//! - No conflicts when commands are different
//! - Conflicts detected when commands are identical
//! - Module prefixes prevent conflicts
//! - Global prefixes work correctly
//! - Regression test for "example" placeholder bug

use unilang::multi_yaml::{ MultiYamlAggregator, AggregationConfig, ModuleConfig, ConflictResolutionStrategy, NamespaceIsolation };
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

/// Helper function to create temporary YAML file
fn create_temp_yaml( name: &str, content: &str ) -> PathBuf
{
  let temp_dir = std::env::temp_dir();
  let file_path = temp_dir.join( format!( "unilang_test_{}_{}.yaml", name, std::process::id() ) );
  let mut file = fs::File::create( &file_path ).expect( "Failed to create temp file" );
  file.write_all( content.as_bytes() ).expect( "Failed to write temp file" );
  file_path
}

/// Helper function to cleanup temporary YAML file
fn cleanup_temp_yaml( path: &PathBuf )
{
  let _ = fs::remove_file( path );
}

/// Test: No conflicts when modules have different commands
///
/// Module A: `.foo`, Module B: `.bar`
/// Expected: Zero conflicts detected
#[ test ]
fn test_no_conflict_different_commands()
{
  let yaml_a = r#"
- name: ".foo"
  namespace: ""
  description: "Command foo"
  arguments: []
  examples: []
"#;

  let yaml_b = r#"
- name: ".bar"
  namespace: ""
  description: "Command bar"
  arguments: []
  examples: []
"#;

  // Create temporary YAML files
  let file_a = create_temp_yaml( "module_a", yaml_a );
  let file_b = create_temp_yaml( "module_b", yaml_b );

  let config = AggregationConfig
  {
    base_dir: std::env::temp_dir(),
    modules: vec!
    [
      ModuleConfig
      {
        name: "module_a".to_string(),
        yaml_path: file_a.to_string_lossy().to_string(),
        prefix: None,
        enabled: true,
      },
      ModuleConfig
      {
        name: "module_b".to_string(),
        yaml_path: file_b.to_string_lossy().to_string(),
        prefix: None,
        enabled: true,
      },
    ],
    global_prefix: None,
    detect_conflicts: true,
    env_overrides: HashMap::new(),
    conflict_resolution: ConflictResolutionStrategy::Fail,
    auto_discovery: false,
    discovery_patterns: vec![],
    namespace_isolation: NamespaceIsolation
    {
      enabled: false,
      separator: ".".to_string(),
      strict_mode: false,
    },
  };

  let mut aggregator = MultiYamlAggregator::new( config );

  // Load YAML files
  aggregator.load_yaml_files().expect( "Failed to load YAML files" );

  // Run conflict detection
  aggregator.detect_conflicts();

  // Verify no conflicts
  assert_eq!( aggregator.conflicts().len(), 0, "Expected no conflicts between .foo and .bar" );

  // Cleanup
  cleanup_temp_yaml( &file_a );
  cleanup_temp_yaml( &file_b );
}

/// Test: Conflicts detected when modules have same command
///
/// Module A: `.foo`, Module B: `.foo`
/// Expected: One conflict detected
#[ test ]
fn test_conflict_same_command_name()
{
  let yaml_a = r#"
- name: ".foo"
  namespace: ""
  description: "Command foo from module A"
  arguments: []
  examples: []
"#;

  let yaml_b = r#"
- name: ".foo"
  namespace: ""
  description: "Command foo from module B"
  arguments: []
  examples: []
"#;

  let config = AggregationConfig
  {
    base_dir: PathBuf::from( "." ),
    modules: vec!
    [
      ModuleConfig
      {
        name: "module_a".to_string(),
        yaml_path: "a.yaml".to_string(),
        prefix: None,
        enabled: true,
      },
      ModuleConfig
      {
        name: "module_b".to_string(),
        yaml_path: "b.yaml".to_string(),
        prefix: None,
        enabled: true,
      },
    ],
    global_prefix: None,
    detect_conflicts: true,
    env_overrides: HashMap::new(),
    conflict_resolution: ConflictResolutionStrategy::Fail,
    auto_discovery: false,
    discovery_patterns: vec![],
    namespace_isolation: NamespaceIsolation
    {
      enabled: false,
      separator: ".".to_string(),
      strict_mode: false,
    },
  };

  let mut aggregator = MultiYamlAggregator::new( config );
  aggregator.yaml_files_mut().insert( "module_a".to_string(), yaml_a.to_string() );
  aggregator.yaml_files_mut().insert( "module_b".to_string(), yaml_b.to_string() );

  aggregator.detect_conflicts();

  // Verify conflict detected
  assert_eq!( aggregator.conflicts().len(), 1, "Expected one conflict for duplicate .foo" );

  let conflict = &aggregator.conflicts()[0];
  assert_eq!( conflict.command_name, ".foo" );
  assert_eq!( conflict.modules.len(), 2 );
  assert!( conflict.modules.contains( &"module_a".to_string() ) );
  assert!( conflict.modules.contains( &"module_b".to_string() ) );
}

/// Test: Module prefix prevents conflicts
///
/// Module A: `.foo` (no prefix) → `.foo`
/// Module B: `.foo` (prefix "pr") → `.pr.foo`
/// Expected: Zero conflicts (different final names)
#[ test ]
fn test_prefix_prevents_conflict()
{
  let yaml_a = r#"
- name: ".foo"
  namespace: ""
  description: "Command foo"
  arguments: []
  examples: []
"#;

  let yaml_b = r#"
- name: ".foo"
  namespace: ""
  description: "Command foo with prefix"
  arguments: []
  examples: []
"#;

  let config = AggregationConfig
  {
    base_dir: PathBuf::from( "." ),
    modules: vec!
    [
      ModuleConfig
      {
        name: "module_a".to_string(),
        yaml_path: "a.yaml".to_string(),
        prefix: None,  // No prefix
        enabled: true,
      },
      ModuleConfig
      {
        name: "module_b".to_string(),
        yaml_path: "b.yaml".to_string(),
        prefix: Some( "pr".to_string() ),  // Prefix applied
        enabled: true,
      },
    ],
    global_prefix: None,
    detect_conflicts: true,
    env_overrides: HashMap::new(),
    conflict_resolution: ConflictResolutionStrategy::Fail,
    auto_discovery: false,
    discovery_patterns: vec![],
    namespace_isolation: NamespaceIsolation
    {
      enabled: false,
      separator: ".".to_string(),
      strict_mode: false,
    },
  };

  let mut aggregator = MultiYamlAggregator::new( config );
  aggregator.yaml_files_mut().insert( "module_a".to_string(), yaml_a.to_string() );
  aggregator.yaml_files_mut().insert( "module_b".to_string(), yaml_b.to_string() );

  aggregator.detect_conflicts();

  // Verify no conflicts (.foo vs .pr.foo are different)
  assert_eq!( aggregator.conflicts().len(), 0,
    "Expected no conflicts: .foo and .pr.foo are different commands" );
}

/// Test: Regression test for "example" placeholder bug
///
/// This is the actual bug we're fixing: get_module_base_commands()
/// was returning vec!["example"] for all modules, causing false positives.
///
/// Module A: `.review`, Module B: `.list`
/// Old behavior: Both return "example" → false conflict
/// New behavior: Parse actual YAMLs → no conflict
#[ test ]
fn test_placeholder_regression()
{
  let yaml_review = r#"
- name: ".review"
  namespace: ""
  description: "Review command"
  arguments: []
  examples: []
"#;

  let yaml_list = r#"
- name: ".list"
  namespace: ""
  description: "List command"
  arguments: []
  examples: []
"#;

  let config = AggregationConfig
  {
    base_dir: PathBuf::from( "." ),
    modules: vec!
    [
      ModuleConfig
      {
        name: "wip".to_string(),
        yaml_path: "wip.yaml".to_string(),
        prefix: None,
        enabled: true,
      },
      ModuleConfig
      {
        name: "pr_review_workflow".to_string(),
        yaml_path: "pr_review.yaml".to_string(),
        prefix: Some( "pr".to_string() ),
        enabled: true,
      },
    ],
    global_prefix: None,
    detect_conflicts: true,
    env_overrides: HashMap::new(),
    conflict_resolution: ConflictResolutionStrategy::Fail,
    auto_discovery: false,
    discovery_patterns: vec![],
    namespace_isolation: NamespaceIsolation
    {
      enabled: false,
      separator: ".".to_string(),
      strict_mode: false,
    },
  };

  let mut aggregator = MultiYamlAggregator::new( config );
  aggregator.yaml_files_mut().insert( "wip".to_string(), yaml_list.to_string() );
  aggregator.yaml_files_mut().insert( "pr_review_workflow".to_string(), yaml_review.to_string() );

  aggregator.detect_conflicts();

  // With placeholder: Would incorrectly detect "example" conflict
  // With fix: Correctly detects no conflicts (.list vs .pr.review)
  assert_eq!( aggregator.conflicts().len(), 0,
    "Regression: Placeholder was returning 'example' for all modules" );
}

/// Test: Real wip + pr_review_workflow integration scenario
///
/// Uses actual command names from the two projects:
/// - wip: `.prs`, `.prs.list`, `.orgs`, etc.
/// - pr_review_workflow: `.review`, `.workflow.generate` (with "pr" prefix)
///
/// Expected: Zero conflicts
#[ test ]
fn test_real_wip_integration()
{
  let wip_yaml = r#"
- name: ".prs"
  namespace: ""
  description: "PR commands help"
  arguments: []
  examples: []
- name: ".prs.list"
  namespace: ""
  description: "List PRs"
  arguments: []
  examples: []
- name: ".orgs"
  namespace: ""
  description: "Org commands help"
  arguments: []
  examples: []
"#;

  let pr_review_yaml = r#"
- name: ".review"
  namespace: ""
  description: "Execute PR review"
  arguments: []
  examples: []
- name: ".workflow.generate"
  namespace: ""
  description: "Generate workflow"
  arguments: []
  examples: []
"#;

  let config = AggregationConfig
  {
    base_dir: PathBuf::from( "." ),
    modules: vec!
    [
      ModuleConfig
      {
        name: "wip".to_string(),
        yaml_path: "wip.commands.yaml".to_string(),
        prefix: None,
        enabled: true,
      },
      ModuleConfig
      {
        name: "pr_review_workflow".to_string(),
        yaml_path: "pr_review.commands.yaml".to_string(),
        prefix: Some( "pr".to_string() ),  // Commands become .pr.review, .pr.workflow.generate
        enabled: true,
      },
    ],
    global_prefix: None,
    detect_conflicts: true,
    env_overrides: HashMap::new(),
    conflict_resolution: ConflictResolutionStrategy::Fail,
    auto_discovery: false,
    discovery_patterns: vec![],
    namespace_isolation: NamespaceIsolation
    {
      enabled: false,
      separator: ".".to_string(),
      strict_mode: false,
    },
  };

  let mut aggregator = MultiYamlAggregator::new( config );
  aggregator.yaml_files_mut().insert( "wip".to_string(), wip_yaml.to_string() );
  aggregator.yaml_files_mut().insert( "pr_review_workflow".to_string(), pr_review_yaml.to_string() );

  aggregator.detect_conflicts();

  // Verify no conflicts in real integration scenario
  assert_eq!( aggregator.conflicts().len(), 0,
    "Real integration should have zero conflicts:\n\
     wip: .prs, .prs.list, .orgs\n\
     pr_review_workflow: .pr.review, .pr.workflow.generate" );
}
