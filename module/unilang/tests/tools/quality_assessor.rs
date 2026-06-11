//! Tests for the `QualityAssessor` library.

#[ path = "../../examples/assess_quality_cli/quality_assessor.rs" ]
mod assessor;
use assessor::*;

use std::path::Path;
use tempfile::TempDir;

#[ test ]
fn test_quality_assessor_creation()
{
  let temp_dir = TempDir::new().unwrap();
  let assessor = QualityAssessor::new( temp_dir.path() );

  assert_eq!( assessor.tests_root, temp_dir.path() );
  assert_eq!( assessor.config.target_line_coverage, 95.0 );
}

#[ test ]
fn test_file_naming_compliance()
{
  let temp_dir = TempDir::new().unwrap();
  let assessor = QualityAssessor::new( temp_dir.path() );

  assert!( assessor.check_file_naming_compliance( Path::new( "semantic_analysis.rs" ) ) );
  assert!( assessor.check_file_naming_compliance( Path::new( "argument_parsing.rs" ) ) );

  assert!( !assessor.check_file_naming_compliance( Path::new( "task_024_fix.rs" ) ) );
  assert!( !assessor.check_file_naming_compliance( Path::new( "issue_017_workaround.rs" ) ) );
}

#[ test ]
fn test_structure_compliance()
{
  let temp_dir = TempDir::new().unwrap();
  let assessor = QualityAssessor::new( temp_dir.path() );

  assert!( assessor.check_file_structure_compliance( Path::new( "unit/parser/argument_parsing.rs" ) ) );
  assert!( assessor.check_file_structure_compliance( Path::new( "integration/end_to_end.rs" ) ) );

  assert!( !assessor.check_file_structure_compliance( Path::new( "random/test_file.rs" ) ) );
  assert!( !assessor.check_file_structure_compliance( Path::new( "test_file.rs" ) ) );
}
