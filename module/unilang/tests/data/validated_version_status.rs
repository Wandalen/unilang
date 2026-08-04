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
// test_kind: tc_spec(TC-1, TC-3, TC-4)  [type/03_version_type]
#[ test ]
fn test_tc1_tc3_tc4_version_non_empty_valid()
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
// test_kind: tc_spec(TC-2)  [type/03_version_type]
#[ test ]
fn test_tc2_version_empty_rejected()
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
// test_kind: tc_spec(TC-5)  [type/03_version_type]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc5_version_serde_rejects_empty()
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
// test_kind: tc_spec(TC-1)  [type/04_command_status]
#[ test ]
fn test_tc1_command_status_active()
{
  let active = CommandStatus::Active;

  assert!( active.is_active(), "Active status should report is_active()" );
  assert!( !active.is_deprecated(), "Active status should not be deprecated" );
  assert!( !active.is_experimental(), "Active status should not be experimental" );
  assert!( !active.is_internal(), "Active status should not be internal" );

  assert_eq!( format!( "{}", active ), "active" );
}

/// TC-6: Experimental variant is queryable.
// test_kind: tc_spec(TC-6)  [type/04_command_status]
#[ test ]
fn test_tc6_command_status_experimental()
{
  let experimental = CommandStatus::Experimental;

  assert!( !experimental.is_active(), "Experimental status should not be active" );
  assert!( experimental.is_experimental(), "Experimental status should report is_experimental()" );
  assert!( !experimental.is_deprecated(), "Experimental status should not be deprecated" );
  assert!( !experimental.is_internal(), "Experimental status should not be internal" );

  assert_eq!( format!( "{}", experimental ), "experimental" );
}

/// TC-7: Internal variant is queryable.
// test_kind: tc_spec(TC-7)  [type/04_command_status]
#[ test ]
fn test_tc7_command_status_internal()
{
  let internal = CommandStatus::Internal;

  assert!( !internal.is_active(), "Internal status should not be active" );
  assert!( internal.is_internal(), "Internal status should report is_internal()" );
  assert!( !internal.is_deprecated(), "Internal status should not be deprecated" );
  assert!( !internal.is_experimental(), "Internal status should not be experimental" );

  assert_eq!( format!( "{}", internal ), "internal" );
}

/// TC-2: Deprecated variant carries metadata.
// test_kind: tc_spec(TC-2)  [type/04_command_status]
#[ test ]
fn test_tc2_command_status_deprecated_carries_metadata()
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
// test_kind: tc_spec(TC-3)  [type/04_command_status]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc3_command_status_serde_json_active()
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
// test_kind: tc_spec(TC-4)  [type/04_command_status]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc4_command_status_serde_json_deprecated()
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
// test_kind: tc_spec(TC-5)  [type/04_command_status]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc5_command_status_serde_case_insensitive()
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

/// TC-8: Default trait produces Active.
// test_kind: tc_spec(TC-8)  [type/04_command_status]
#[ test ]
fn test_tc8_command_status_default_is_active()
{
  let default = CommandStatus::default();
  assert_eq!( default, CommandStatus::Active, "CommandStatus::default() should be Active" );
}

/// TC-9: from_str_lossy maps recognized and unrecognized strings.
// test_kind: tc_spec(TC-9)  [type/04_command_status]
#[ test ]
fn test_tc9_command_status_from_str_lossy()
{
  assert_eq!( CommandStatus::from_str_lossy( "experimental" ), CommandStatus::Experimental );
  assert_eq!( CommandStatus::from_str_lossy( "internal" ), CommandStatus::Internal );
  assert_eq!( CommandStatus::from_str_lossy( "stable" ), CommandStatus::Active );
  assert_eq!( CommandStatus::from_str_lossy( "unknown" ), CommandStatus::Active );
}

/// TC-10: Display formats simple variants as lowercase words.
// test_kind: tc_spec(TC-10)  [type/04_command_status]
#[ test ]
fn test_tc10_command_status_display_simple_variants()
{
  assert_eq!( format!( "{}", CommandStatus::Active ), "active" );
  assert_eq!( format!( "{}", CommandStatus::Experimental ), "experimental" );
  assert_eq!( format!( "{}", CommandStatus::Internal ), "internal" );
}

/// TC-11: Display formats Deprecated variant with all metadata segments.
// test_kind: tc_spec(TC-11)  [type/04_command_status]
#[ test ]
fn test_tc11_command_status_display_deprecated_full()
{
  let deprecated = CommandStatus::Deprecated
  {
    reason : "use .new".to_string(),
    since : Some( "2.0".to_string() ),
    replacement : Some( ".new".to_string() ),
  };

  assert_eq!( format!( "{}", deprecated ), "deprecated (since 2.0): use .new → .new" );
}

/// TC-12: Deserialization accepts "stable" as an alias for Active.
// test_kind: tc_spec(TC-12)  [type/04_command_status]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc12_command_status_serde_stable_alias()
{
  let deserialized : CommandStatus = serde_json::from_str( "\"stable\"" )
    .expect( "deserialization of \"stable\" should succeed" );
  assert_eq!( deserialized, CommandStatus::Active );
}

/// TC-13: Deserialization of unrecognized string defaults to Active.
// test_kind: tc_spec(TC-13)  [type/04_command_status]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc13_command_status_serde_unrecognized_defaults_to_active()
{
  let deserialized : CommandStatus = serde_json::from_str( "\"bogus\"" )
    .expect( "deserialization of unrecognized value should not error" );
  assert_eq!( deserialized, CommandStatus::Active );
}

/// TC-14: Deserialization of simple "deprecated" string yields empty metadata.
// test_kind: tc_spec(TC-14)  [type/04_command_status]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc14_command_status_serde_plain_deprecated_string()
{
  let deserialized : CommandStatus = serde_json::from_str( "\"deprecated\"" )
    .expect( "deserialization of plain \"deprecated\" string should succeed" );

  assert!( deserialized.is_deprecated() );
  let ( reason, since, replacement ) = deserialized.deprecation_info().unwrap();
  assert_eq!( reason, "" );
  assert!( since.is_none() );
  assert!( replacement.is_none() );
}

/// TC-15: Display formats Deprecated variant with reason only.
// test_kind: tc_spec(TC-15)  [type/04_command_status]
#[ test ]
fn test_tc15_command_status_display_deprecated_reason_only()
{
  let deprecated = CommandStatus::Deprecated
  {
    reason : "use .new".to_string(),
    since : None,
    replacement : None,
  };

  assert_eq!( format!( "{}", deprecated ), "deprecated: use .new" );
}

/// TC-16: Display formats Deprecated variant with since only.
// test_kind: tc_spec(TC-16)  [type/04_command_status]
#[ test ]
fn test_tc16_command_status_display_deprecated_since_only()
{
  let deprecated = CommandStatus::Deprecated
  {
    reason : String::new(),
    since : Some( "2.0".to_string() ),
    replacement : None,
  };

  assert_eq!( format!( "{}", deprecated ), "deprecated (since 2.0)" );
}

/// TC-17: Map-form "status" field is case-sensitive, unlike string form (TC-5).
// test_kind: tc_spec(TC-17)  [type/04_command_status]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc17_command_status_map_form_status_is_case_sensitive()
{
  let deserialized : CommandStatus = serde_json::from_str( r#"{"status": "DEPRECATED", "reason": "obsolete"}"# )
    .expect( "deserialization should not error on unrecognized map status" );

  assert_eq!(
    deserialized,
    CommandStatus::Active,
    "Uppercase \"DEPRECATED\" in map form should NOT match \"deprecated\" — visit_map performs no case normalization, unlike visit_str"
  );
}

/// TC-18: Map form with missing "status" key defaults to Active.
// test_kind: tc_spec(TC-18)  [type/04_command_status]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc18_command_status_map_form_missing_status_defaults_to_active()
{
  let deserialized : CommandStatus = serde_json::from_str( r#"{"reason": "obsolete"}"# )
    .expect( "deserialization should not error when \"status\" key is absent" );

  assert_eq!( deserialized, CommandStatus::Active );
}

/// TC-19: Map form accepts explicit null since/replacement.
// test_kind: tc_spec(TC-19)  [type/04_command_status]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc19_command_status_map_form_explicit_null_fields()
{
  let json = r#"{"status": "deprecated", "reason": "obsolete", "since": null, "replacement": null}"#;
  let deserialized : CommandStatus = serde_json::from_str( json )
    .expect( "deserialization should succeed with explicit null since/replacement" );

  assert!( deserialized.is_deprecated() );
  let ( reason, since, replacement ) = deserialized.deprecation_info().unwrap();
  assert_eq!( reason, "obsolete" );
  assert!( since.is_none(), "explicit JSON null should flatten to None, same as an omitted field" );
  assert!( replacement.is_none(), "explicit JSON null should flatten to None, same as an omitted field" );
}
