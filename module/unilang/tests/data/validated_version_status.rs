//! Tests for the `VersionType` and `CommandStatus` validated newtypes.
//!
//! Covers construction, validation, traits, accessors, deprecation metadata, and serde.

#![ allow( clippy::expect_fun_call ) ]

use unilang::data::{ VersionType, CommandStatus };

//
// VersionType Tests
//

/// TC-1 / TC-3 / TC-4: Non-empty version strings are accepted (including single char
/// and arbitrary non-empty formats).
// test_kind: tc_spec(TC-1, TC-3, TC-4)
#[ test ]
fn version_valid_construction()
{
  let versions = vec!
  [
    "1",
    "1.0.0",
    "2.1",
    "0.1.0-alpha",
    "1.2.3+build.456",
    "beta-rc.1+build.42",
    "v1.0",
  ];

  for ver_str in versions
  {
    let result = VersionType::new( ver_str );
    assert!(
      result.is_ok(),
      "VersionType::new({:?}) should succeed",
      ver_str
    );

    let ver = result.unwrap();
    assert_eq!(
      ver.as_str(),
      ver_str,
      "Version should preserve original value"
    );
  }
}

/// TC-2: Empty string is rejected.
// test_kind: tc_spec(TC-2)
#[ test ]
fn version_rejects_empty_string()
{
  let result = VersionType::new( "" );

  assert!(
    result.is_err(),
    "VersionType::new(\"\") should fail - version cannot be empty"
  );

  let err = result.unwrap_err();
  let err_msg = err.to_string();

  assert!(
    err_msg.contains( "empty" ),
    "Error message should mention 'empty': {}",
    err_msg
  );
}

#[ test ]
fn version_display_trait()
{
  let ver = VersionType::new( "1.0.0" ).unwrap();
  assert_eq!( format!( "{}", ver ), "1.0.0" );
}

#[ test ]
fn version_accessors()
{
  let ver_str = "1.2.3";
  let ver = VersionType::new( ver_str ).unwrap();

  assert_eq!( ver.as_str(), ver_str );

  let inner = ver.into_inner();
  assert_eq!( inner, ver_str );
}

#[ test ]
fn version_clone_and_equality()
{
  let ver1 = VersionType::new( "1.0.0" ).unwrap();
  let ver2 = ver1.clone();

  assert_eq!( ver1, ver2, "Cloned version should equal original" );

  let ver3 = VersionType::new( "2.0.0" ).unwrap();
  assert_ne!( ver1, ver3, "Different versions should not be equal" );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn version_serde_json_serialize()
{
  let ver = VersionType::new( "1.0.0" ).unwrap();
  let json = serde_json::to_string( &ver ).expect( "serialization should succeed" );

  assert_eq!( json, "\"1.0.0\"" );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn version_serde_json_deserialize_valid()
{
  let json = "\"1.0.0\"";
  let ver : VersionType = serde_json::from_str( json )
    .expect( "deserialization should succeed" );
  assert_eq!( ver.as_str(), "1.0.0" );
}

/// TC-5: Serde deserialization rejects empty version string.
// test_kind: tc_spec(TC-5)
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn version_serde_json_deserialize_rejects_empty()
{
  let json_empty = "\"\"";
  let result : Result< VersionType, _ > = serde_json::from_str( json_empty );
  assert!(
    result.is_err(),
    "Deserialization should fail for empty version"
  );
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn version_serde_yaml_ng_deserialize_valid()
{
  let yaml = "1.0.0";
  let ver : VersionType = serde_yaml_ng::from_str( yaml )
    .expect( "YAML deserialization should succeed" );
  assert_eq!( ver.as_str(), "1.0.0" );
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn version_serde_yaml_ng_deserialize_rejects_empty()
{
  let yaml_empty = "\"\"";
  let result : Result< VersionType, _ > = serde_yaml_ng::from_str( yaml_empty );
  assert!(
    result.is_err(),
    "YAML deserialization should fail for empty version"
  );
}

//
// CommandStatus Tests
//

/// TC-1: Active variant is default and queryable.
// test_kind: tc_spec(TC-1)
#[ test ]
fn command_status_active()
{
  let active = CommandStatus::Active;

  assert!( active.is_active(), "Active status should report is_active()" );
  assert!( !active.is_deprecated(), "Active status should not be deprecated" );
  assert!( !active.is_experimental(), "Active status should not be experimental" );
  assert!( !active.is_internal(), "Active status should not be internal" );

  assert_eq!( format!( "{}", active ), "active" );
}

/// TC-6: Experimental variant is queryable.
// test_kind: tc_spec(TC-6)
#[ test ]
fn command_status_experimental()
{
  let experimental = CommandStatus::Experimental;

  assert!( !experimental.is_active(), "Experimental status should not be active" );
  assert!( experimental.is_experimental(), "Experimental status should report is_experimental()" );
  assert!( !experimental.is_deprecated(), "Experimental status should not be deprecated" );
  assert!( !experimental.is_internal(), "Experimental status should not be internal" );

  assert_eq!( format!( "{}", experimental ), "experimental" );
}

/// TC-7: Internal variant is queryable.
// test_kind: tc_spec(TC-7)
#[ test ]
fn command_status_internal()
{
  let internal = CommandStatus::Internal;

  assert!( !internal.is_active(), "Internal status should not be active" );
  assert!( internal.is_internal(), "Internal status should report is_internal()" );
  assert!( !internal.is_deprecated(), "Internal status should not be deprecated" );
  assert!( !internal.is_experimental(), "Internal status should not be experimental" );

  assert_eq!( format!( "{}", internal ), "internal" );
}

/// TC-2: Deprecated variant carries metadata.
// test_kind: tc_spec(TC-2)
#[ test ]
fn command_status_deprecated_full()
{
  let deprecated = CommandStatus::Deprecated
  {
    reason : "Use .new_command instead".to_string(),
    since : Some( "2.0.0".to_string() ),
    replacement : Some( ".new_command".to_string() ),
  };

  assert!( !deprecated.is_active(), "Deprecated status should not be active" );
  assert!( deprecated.is_deprecated(), "Deprecated status should report is_deprecated()" );
  assert!( !deprecated.is_experimental(), "Deprecated status should not be experimental" );
  assert!( !deprecated.is_internal(), "Deprecated status should not be internal" );

  let ( reason, since, replacement ) = deprecated.deprecation_info().unwrap();
  assert_eq!( reason, "Use .new_command instead" );
  assert_eq!( since.as_ref().unwrap(), "2.0.0" );
  assert_eq!( replacement.as_ref().unwrap(), ".new_command" );

  let display = format!( "{}", deprecated );
  assert!( display.contains( "deprecated" ) );
  assert!( display.contains( "2.0.0" ) );
  assert!( display.contains( "Use .new_command instead" ) );
  assert!( display.contains( ".new_command" ) );
}

#[ test ]
fn command_status_deprecated_minimal()
{
  let deprecated = CommandStatus::Deprecated
  {
    reason : String::new(),
    since : None,
    replacement : None,
  };

  assert!( deprecated.is_deprecated() );

  let ( reason, since, replacement ) = deprecated.deprecation_info().unwrap();
  assert_eq!( reason, "" );
  assert!( since.is_none() );
  assert!( replacement.is_none() );
}

#[ test ]
fn command_status_default()
{
  let default = CommandStatus::default();
  assert!( default.is_active(), "Default status should be Active" );
}

#[ test ]
fn command_status_clone_and_equality()
{
  let active1 = CommandStatus::Active;
  let active2 = active1.clone();
  assert_eq!( active1, active2 );

  let experimental = CommandStatus::Experimental;
  assert_ne!( active1, experimental );

  let deprecated1 = CommandStatus::Deprecated
  {
    reason : "Old API".to_string(),
    since : Some( "1.0.0".to_string() ),
    replacement : Some( ".new".to_string() ),
  };
  let deprecated2 = deprecated1.clone();
  assert_eq!( deprecated1, deprecated2 );
}

/// TC-3: Simple variant serde roundtrip (lowercase string).
// test_kind: tc_spec(TC-3)
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_active()
{
  let active = CommandStatus::Active;
  let json = serde_json::to_string( &active ).expect( "serialization should succeed" );
  assert_eq!( json, "\"active\"" );

  let deserialized : CommandStatus = serde_json::from_str( &json )
    .expect( "deserialization should succeed" );
  assert_eq!( active, deserialized );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_experimental()
{
  let experimental = CommandStatus::Experimental;
  let json = serde_json::to_string( &experimental ).expect( "serialization should succeed" );
  assert_eq!( json, "\"experimental\"" );

  let deserialized : CommandStatus = serde_json::from_str( &json )
    .expect( "deserialization should succeed" );
  assert_eq!( experimental, deserialized );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_internal()
{
  let internal = CommandStatus::Internal;
  let json = serde_json::to_string( &internal ).expect( "serialization should succeed" );
  assert_eq!( json, "\"internal\"" );

  let deserialized : CommandStatus = serde_json::from_str( &json )
    .expect( "deserialization should succeed" );
  assert_eq!( internal, deserialized );
}

/// TC-4: Deprecated variant serde roundtrip (map form).
// test_kind: tc_spec(TC-4)
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_deprecated()
{
  let deprecated = CommandStatus::Deprecated
  {
    reason : "Use .new instead".to_string(),
    since : Some( "2.0.0".to_string() ),
    replacement : Some( ".new".to_string() ),
  };

  let json = serde_json::to_string( &deprecated ).expect( "serialization should succeed" );

  assert!( json.contains( "\"status\"" ) );
  assert!( json.contains( "\"deprecated\"" ) );
  assert!( json.contains( "\"reason\"" ) );
  assert!( json.contains( "Use .new instead" ) );

  let deserialized : CommandStatus = serde_json::from_str( &json )
    .expect( "deserialization should succeed" );

  assert!( deserialized.is_deprecated() );
  let ( reason, since, replacement ) = deserialized.deprecation_info().unwrap();
  assert_eq!( reason, "Use .new instead" );
  assert_eq!( since.as_ref().unwrap(), "2.0.0" );
  assert_eq!( replacement.as_ref().unwrap(), ".new" );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_backward_compatible()
{
  let test_cases = vec!
  [
    ( "\"stable\"", CommandStatus::Active ),
    ( "\"active\"", CommandStatus::Active ),
    ( "\"experimental\"", CommandStatus::Experimental ),
    ( "\"internal\"", CommandStatus::Internal ),
    ( "\"deprecated\"", CommandStatus::Deprecated { reason : String::new(), since : None, replacement : None } ),
  ];

  for ( json, expected ) in test_cases
  {
    let deserialized : CommandStatus = serde_json::from_str( json )
      .expect( &format!( "deserialization of {} should succeed", json ) );
    assert_eq!( deserialized, expected, "Failed for JSON: {}", json );
  }
}

/// TC-5: Case-insensitive deserialization.
// test_kind: tc_spec(TC-5)
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_case_insensitive()
{
  let test_cases = vec!
  [
    ( "\"ACTIVE\"", CommandStatus::Active ),
    ( "\"Active\"", CommandStatus::Active ),
    ( "\"EXPERIMENTAL\"", CommandStatus::Experimental ),
    ( "\"Internal\"", CommandStatus::Internal ),
    ( "\"DEPRECATED\"", CommandStatus::Deprecated { reason : String::new(), since : None, replacement : None } ),
  ];

  for ( json, expected ) in test_cases
  {
    let deserialized : CommandStatus = serde_json::from_str( json )
      .expect( &format!( "case-insensitive deserialization of {} should succeed", json ) );
    assert_eq!( deserialized, expected, "Case-insensitive failed for JSON: {}", json );
  }
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn command_status_serde_yaml_ng_simple()
{
  let test_cases = vec!
  [
    ( "active", CommandStatus::Active ),
    ( "stable", CommandStatus::Active ),
    ( "experimental", CommandStatus::Experimental ),
    ( "internal", CommandStatus::Internal ),
    ( "deprecated", CommandStatus::Deprecated { reason : String::new(), since : None, replacement : None } ),
  ];

  for ( yaml, expected ) in test_cases
  {
    let deserialized : CommandStatus = serde_yaml_ng::from_str( yaml )
      .expect( &format!( "YAML deserialization of {} should succeed", yaml ) );
    assert_eq!( deserialized, expected, "Failed for YAML: {}", yaml );
  }
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn command_status_serde_yaml_ng_deprecated_object()
{
  let yaml = r"
status: deprecated
reason: Use .new instead
since: 2.0.0
replacement: .new
";

  let deserialized : CommandStatus = serde_yaml_ng::from_str( yaml )
    .expect( "YAML deserialization should succeed" );

  assert!( deserialized.is_deprecated() );
  let ( reason, since, replacement ) = deserialized.deprecation_info().unwrap();
  assert_eq!( reason, "Use .new instead" );
  assert_eq!( since.as_ref().unwrap(), "2.0.0" );
  assert_eq!( replacement.as_ref().unwrap(), ".new" );
}
