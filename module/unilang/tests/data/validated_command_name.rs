//! Tests for the `CommandName` validated newtype.
//!
//! Covers construction, validation, traits, accessors, serde, and edge cases.

#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::expect_fun_call ) ]

use unilang::data::CommandName;

/// TC-1: Valid dot-prefixed names are accepted.
// test_kind: tc_spec(TC-1)  [type/01_command_name]
#[ test ]
fn test_tc1_valid_dot_prefix_construction()
{
  let names = vec!
  [
    ".build",
    ".test",
    ".integration.test",
    ".a.b.c",
    ".help",
  ];

  for name in names
  {
    let result = CommandName::new( name );
    assert!(
      result.is_ok(),
      "CommandName::new({:?}) should succeed",
      name
    );

    let cmd_name = result.unwrap();
    assert_eq!(
      cmd_name.as_str(),
      name,
      "as_str() should return the original name"
    );
  }
}

/// TC-2: Empty name is rejected.
// test_kind: tc_spec(TC-2)  [type/01_command_name]
#[ test ]
fn test_tc2_empty_name_rejected()
{
  let result = CommandName::new( "" );

  assert!(
    result.is_err(),
    "CommandName::new(\"\") should fail with EmptyCommandName error"
  );

  let err = result.unwrap_err();
  let err_msg = err.to_string();

  assert!(
    err_msg.contains( "empty" ) || err_msg.contains( "cannot be empty" ),
    "Error message should mention 'empty': {}",
    err_msg
  );
}

/// TC-3: Non-dot-prefixed names are rejected.
// test_kind: tc_spec(TC-3)  [type/01_command_name]
#[ test ]
fn test_tc3_missing_dot_prefix_rejected()
{
  let invalid_names = vec!
  [
    "build",
    "test",
    "integration.test",
    "a.b.c",
  ];

  for name in invalid_names
  {
    let result = CommandName::new( name );

    assert!(
      result.is_err(),
      "CommandName::new({:?}) should fail - missing dot prefix",
      name
    );

    let err = result.unwrap_err();
    let err_msg = err.to_string();

    assert!(
      err_msg.contains( "dot prefix" ) || err_msg.contains( "start with" ),
      "Error message should mention 'dot prefix' for {:?}: {}",
      name,
      err_msg
    );

    assert!(
      err_msg.contains( name ),
      "Error message should include the invalid name {:?}: {}",
      name,
      err_msg
    );
  }
}

#[ test ]
fn command_name_display_trait()
{
  let name = CommandName::new( ".build" ).unwrap();
  let display_str = format!( "{}", name );

  assert_eq!(
    display_str,
    ".build",
    "Display trait should show the command name"
  );
}

#[ test ]
fn command_name_debug_trait()
{
  let name = CommandName::new( ".test" ).unwrap();
  let debug_str = format!( "{:?}", name );

  assert!(
    debug_str.contains( ".test" ),
    "Debug trait should include the command name: {}",
    debug_str
  );
}

#[ test ]
fn command_name_accessors()
{
  let name_str = ".integration";
  let name = CommandName::new( name_str ).unwrap();

  assert_eq!(
    name.as_str(),
    name_str,
    "as_str() should return the name as &str"
  );

  let inner = name.into_inner();
  assert_eq!(
    inner,
    name_str,
    "into_inner() should return the owned String"
  );
}

#[ test ]
fn command_name_clone_and_equality()
{
  let name1 = CommandName::new( ".build" ).unwrap();
  let name2 = name1.clone();

  assert_eq!(
    name1,
    name2,
    "Cloned CommandName should equal the original"
  );

  let name3 = CommandName::new( ".test" ).unwrap();
  assert_ne!(
    name1,
    name3,
    "Different CommandNames should not be equal"
  );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_name_serde_json_serialize()
{
  let name = CommandName::new( ".build" ).unwrap();
  let json = serde_json::to_string( &name ).expect( "serialization should succeed" );

  assert_eq!(
    json,
    "\".build\"",
    "CommandName should serialize as a JSON string"
  );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_name_serde_json_deserialize_valid()
{
  let json = "\".build\"";
  let name : CommandName = serde_json::from_str( json )
    .expect( "deserialization should succeed for valid name" );

  assert_eq!(
    name.as_str(),
    ".build",
    "Deserialized CommandName should have correct value"
  );
}

/// TC-6: Serde deserialization rejects empty and non-dot-prefixed names.
// test_kind: tc_spec(TC-6)  [type/01_command_name]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_tc6_serde_rejects_invalid_command_name()
{
  let json_empty = "\"\"";
  let result : Result< CommandName, _ > = serde_json::from_str( json_empty );
  assert!(
    result.is_err(),
    "Deserialization should fail for empty name"
  );

  let json_no_prefix = "\"build\"";
  let result : Result< CommandName, _ > = serde_json::from_str( json_no_prefix );
  assert!(
    result.is_err(),
    "Deserialization should fail for name without dot prefix"
  );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_name_serde_json_roundtrip()
{
  let original = CommandName::new( ".integration.test" ).unwrap();

  let json = serde_json::to_string( &original )
    .expect( "serialization should succeed" );

  let deserialized : CommandName = serde_json::from_str( &json )
    .expect( "deserialization should succeed" );

  assert_eq!(
    original,
    deserialized,
    "Roundtrip serialization should preserve equality"
  );

  assert_eq!(
    original.as_str(),
    deserialized.as_str(),
    "Roundtrip serialization should preserve value"
  );
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn command_name_serde_yaml_ng_deserialize_valid()
{
  let yaml = ".build";
  let name : CommandName = serde_yaml_ng::from_str( yaml )
    .expect( "YAML deserialization should succeed for valid name" );

  assert_eq!(
    name.as_str(),
    ".build",
    "Deserialized CommandName from YAML should have correct value"
  );
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn command_name_serde_yaml_ng_deserialize_rejects_invalid()
{
  let yaml_empty = "\"\"";
  let result : Result< CommandName, _ > = serde_yaml_ng::from_str( yaml_empty );
  assert!(
    result.is_err(),
    "YAML deserialization should fail for empty name"
  );

  let yaml_no_prefix = "build";
  let result : Result< CommandName, _ > = serde_yaml_ng::from_str( yaml_no_prefix );
  assert!(
    result.is_err(),
    "YAML deserialization should fail for name without dot prefix"
  );
}

#[ test ]
fn command_name_with_special_characters()
{
  let names = vec!
  [
    ".test-command",
    ".test_command",
    ".test.sub-command",
  ];

  for name in names
  {
    let result = CommandName::new( name );
    assert!(
      result.is_ok(),
      "CommandName::new({:?}) should succeed - special chars are allowed",
      name
    );
  }
}

#[ test ]
fn command_name_long_names()
{
  let long_name = format!( ".{}", "a".repeat( 100 ) );
  let result = CommandName::new( &long_name );

  assert!(
    result.is_ok(),
    "CommandName should accept long names"
  );

  assert_eq!(
    result.unwrap().as_str(),
    long_name.as_str(),
    "Long name should be preserved exactly"
  );
}

/// TC-4: Single dot is a valid command name.
// test_kind: tc_spec(TC-4)  [type/01_command_name]
#[ test ]
fn test_tc4_single_dot_valid()
{
  let result = CommandName::new( "." );

  assert!(
    result.is_ok(),
    "CommandName::new(\".\") should succeed - single dot is valid"
  );

  assert_eq!(
    result.unwrap().as_str(),
    ".",
    "Single dot should be preserved"
  );
}

#[ test ]
fn command_name_single_char_after_dot()
{
  let name = ".a";
  let result = CommandName::new( name );

  assert!(
    result.is_ok(),
    "CommandName::new(\".a\") should succeed - single char is valid"
  );
}

#[ test ]
fn command_name_multiple_dots()
{
  let names = vec!
  [
    "..",
    "...",
    ".a..b",
    ".a.b.c.d.e",
  ];

  for name in names
  {
    let result = CommandName::new( name );
    assert!(
      result.is_ok(),
      "CommandName::new({:?}) should succeed - multiple dots are allowed",
      name
    );
  }
}

/// TC-5: Nested dot-separated command names are accepted.
// test_kind: tc_spec(TC-5)  [type/01_command_name]
#[ test ]
fn test_tc5_nested_dot_name_valid()
{
  let names = vec!
  [
    ".video.convert",
    ".git.remote.add",
    ".a.b",
    ".cloud.storage.upload",
  ];

  for name in names
  {
    let result = CommandName::new( name );
    assert!(
      result.is_ok(),
      "CommandName::new({:?}) should succeed — nested dot-separated names are valid",
      name
    );
    assert_eq!(
      result.unwrap().as_str(),
      name,
      "Nested name should be preserved exactly"
    );
  }
}
