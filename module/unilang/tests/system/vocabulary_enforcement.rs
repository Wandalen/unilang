//! System actor vocabulary enforcement tests.
//!
//! Implements IN-1..5 specification cases from `tests/docs/invariant/01_system_actors_vocabulary.md`.
//!
//! Tests verify that the canonical actor taxonomy defined in
//! `docs/invariant/001_system_actors_vocabulary.md` is enforced in source code and
//! documentation: no deprecated synonyms exist, all actor categories are documented,
//! and the canonical `SemanticAnalyzer` name is used in the implementation.

use walkdir::WalkDir;
use unilang::semantic::SemanticAnalyzer;

/// Search for a pattern string in all `.rs` source files under the given directory.
/// Returns the total number of lines that contain the pattern.
fn count_pattern_in_rs_files( dir : &str, pattern : &str ) -> usize
{
  WalkDir::new( dir )
    .into_iter()
    .filter_map( | e | e.ok() )
    .filter( | e |
    {
      e.file_type().is_file()
        && e.path().extension().and_then( | ext | ext.to_str() ) == Some( "rs" )
    })
    .map( | e | std::fs::read_to_string( e.path() ).unwrap_or_default() )
    .flat_map( | content | content.lines().map( | l | l.to_string() ).collect::< Vec< _ > >() )
    .filter( | line | line.contains( pattern ) )
    .count()
}

/// IN-1: The deprecated synonym `"Executor"` does not appear as a type definition in source.
///
/// `Interpreter` is the canonical name for the execution actor. Any type declaration
/// named `Executor` would violate the vocabulary contract.
///
/// ## Scope
///
/// Scans `src/` for Rust type declarations (`struct Executor`, `enum Executor`,
/// `type Executor`) only — incidental word matches (comments, variable names) are
/// excluded by the specific pattern.
// test_kind: in_spec(IN-1)  [invariant/01_system_actors_vocabulary]
#[ test ]
fn test_in1_executor_synonym_absent_as_type_definition()
{
  let src_dir = format!( "{}/src", env!( "CARGO_MANIFEST_DIR" ) );

  let struct_count = count_pattern_in_rs_files( &src_dir, "struct Executor" );
  let enum_count = count_pattern_in_rs_files( &src_dir, "enum Executor" );
  let type_count = count_pattern_in_rs_files( &src_dir, "type Executor" );

  assert_eq!(
    struct_count + enum_count + type_count,
    0,
    "No type declaration named 'Executor' must exist in src/ — \
     'Interpreter' is the canonical name for the execution actor (IN-1 violation)"
  );
}

/// IN-2: Actor taxonomy document contains all three required actor categories.
///
/// The `docs/invariant/001_system_actors_vocabulary.md` file must enumerate all three
/// categories from the canonical taxonomy: Human Actors, External System Actors, and
/// Internal System Actors.
// test_kind: in_spec(IN-2)  [invariant/01_system_actors_vocabulary]
#[ test ]
fn test_in2_actor_taxonomy_contains_all_three_categories()
{
  let doc_path = format!(
    "{}/docs/invariant/001_system_actors_vocabulary.md",
    env!( "CARGO_MANIFEST_DIR" )
  );

  let content = std::fs::read_to_string( &doc_path )
    .unwrap_or_else( | e | panic!( "Cannot read actor vocabulary doc at {doc_path}: {e}" ) );

  assert!(
    content.contains( "Human Actors" ) || content.contains( "Human actors" ),
    "Vocabulary doc must contain 'Human Actors' category"
  );
  assert!(
    content.contains( "External System Actors" )
      || content.contains( "External system actors" )
      || content.contains( "System Actors" ),
    "Vocabulary doc must contain external system actors category"
  );
  assert!(
    content.contains( "Internal System Actors" )
      || content.contains( "Internal actors" )
      || content.contains( "Internal system actors" ),
    "Vocabulary doc must contain 'Internal System Actors' category"
  );
}

/// IN-3: `SemanticAnalyzer` is the canonical name used in the implementation.
///
/// This is a compile-time verification: if the semantic analysis actor were renamed to
/// `Validator`, `Checker`, or `Verifier`, this import and usage would fail to compile.
///
/// ## Design Rationale
///
/// Compile-time name enforcement is stronger than a grep test because it fails at build
/// time rather than at test runtime, and it cannot produce false positives.
// test_kind: in_spec(IN-3)  [invariant/01_system_actors_vocabulary]
#[ test ]
fn test_in3_semantic_analyzer_is_canonical_name()
{
  // Using SemanticAnalyzer proves the canonical name exists in the public API.
  // This test compiles only if the struct is exported under this exact name.
  let registry = unilang::registry::CommandRegistry::new();
  let instructions : &[ unilang_parser::GenericInstruction ] = &[];
  let _analyzer = SemanticAnalyzer::new( instructions, &registry );

  // Compilation success is the assertion — no runtime check needed
}

/// IN-4: The deprecated synonyms `"CommandStore"`, `"CommandCache"`, and `"CommandDatabase"`
/// do not appear as type definitions in source.
///
/// `CommandRegistry` (and `StaticCommandMap` for the static variant) are the canonical
/// names for the runtime command database. Any type declaration named `CommandStore`,
/// `CommandCache`, or `CommandDatabase` would violate the vocabulary contract.
///
/// ## Scope
///
/// Scans `src/` for Rust type declarations (`struct`/`enum`/`type` followed by one of the
/// three synonym names) only — incidental word matches (comments, variable names) are
/// excluded by the specific pattern.
// test_kind: in_spec(IN-4)  [invariant/01_system_actors_vocabulary]
#[ test ]
fn test_in4_command_registry_synonyms_absent_as_type_definitions()
{
  let src_dir = format!( "{}/src", env!( "CARGO_MANIFEST_DIR" ) );

  let synonyms = [ "CommandStore", "CommandCache", "CommandDatabase" ];

  let total_count : usize = synonyms
    .iter()
    .map( | synonym |
    {
      let struct_count = count_pattern_in_rs_files( &src_dir, &format!( "struct {synonym}" ) );
      let enum_count = count_pattern_in_rs_files( &src_dir, &format!( "enum {synonym}" ) );
      let type_count = count_pattern_in_rs_files( &src_dir, &format!( "type {synonym}" ) );
      struct_count + enum_count + type_count
    })
    .sum();

  assert_eq!(
    total_count,
    0,
    "No type declaration named 'CommandStore', 'CommandCache', or 'CommandDatabase' must exist \
     in src/ — 'CommandRegistry' (and 'StaticCommandMap' for the static variant) are the \
     canonical names for the runtime command database (IN-4 violation)"
  );
}

/// IN-5: The deprecated synonyms `"ArgType"`, `"DataType"`, and `"ValueType"` do not appear
/// as type definitions in source.
///
/// `Kind` is the canonical enum name for an argument's data type. Any type declaration
/// named `ArgType`, `DataType`, or `ValueType` would violate the vocabulary contract.
///
/// ## Scope
///
/// Scans `src/` for Rust type declarations (`struct`/`enum`/`type` followed by one of the
/// three synonym names) only — incidental word matches (comments, variable names) are
/// excluded by the specific pattern.
// test_kind: in_spec(IN-5)  [invariant/01_system_actors_vocabulary]
#[ test ]
fn test_in5_kind_synonyms_absent_as_type_definitions()
{
  let src_dir = format!( "{}/src", env!( "CARGO_MANIFEST_DIR" ) );

  let synonyms = [ "ArgType", "DataType", "ValueType" ];

  let total_count : usize = synonyms
    .iter()
    .map( | synonym |
    {
      let struct_count = count_pattern_in_rs_files( &src_dir, &format!( "struct {synonym}" ) );
      let enum_count = count_pattern_in_rs_files( &src_dir, &format!( "enum {synonym}" ) );
      let type_count = count_pattern_in_rs_files( &src_dir, &format!( "type {synonym}" ) );
      struct_count + enum_count + type_count
    })
    .sum();

  assert_eq!(
    total_count,
    0,
    "No type declaration named 'ArgType', 'DataType', or 'ValueType' must exist in src/ — \
     'Kind' is the canonical enum name for argument data types (IN-5 violation)"
  );
}
