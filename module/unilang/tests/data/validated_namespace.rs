//! Tests for the `NamespaceType` validated newtype.
//!
//! Covers construction, validation, traits, accessors, root detection, and serde.

use unilang::data::NamespaceType;

/// TC-1: Empty namespace is accepted (root-level commands).
// test_kind: tc_spec(TC-1)
#[ test ]
fn namespace_valid_empty()
{
  let empty = NamespaceType::new( "" );

  assert!(
    empty.is_ok(),
    "NamespaceType::new(\"\") should succeed - empty namespace is valid"
  );

  let ns = empty.unwrap();
  assert_eq!(
    ns.as_str(),
    "",
    "Empty namespace should have empty string value"
  );

  assert!(
    ns.is_root(),
    "Empty namespace should be identified as root"
  );
}

/// TC-2 / TC-4: Dot-prefixed namespace is accepted (including nested).
// test_kind: tc_spec(TC-2, TC-4)
#[ test ]
fn namespace_valid_with_dot_prefix()
{
  let namespaces = vec!
  [
    ".video",
    ".git",
    ".config",
    ".integration.test",
  ];

  for ns_str in namespaces
  {
    let result = NamespaceType::new( ns_str );
    assert!(
      result.is_ok(),
      "NamespaceType::new({:?}) should succeed",
      ns_str
    );

    let ns = result.unwrap();
    assert_eq!(
      ns.as_str(),
      ns_str,
      "Namespace should preserve original value"
    );

    assert!(
      !ns.is_root(),
      "Non-empty namespace should not be root"
    );
  }
}

/// TC-3: Non-empty non-dot-prefixed namespace is rejected.
// test_kind: tc_spec(TC-3)
#[ test ]
fn namespace_rejects_missing_dot_prefix()
{
  let invalid_namespaces = vec!
  [
    "video",
    "git",
    "config",
  ];

  for ns_str in invalid_namespaces
  {
    let result = NamespaceType::new( ns_str );

    assert!(
      result.is_err(),
      "NamespaceType::new({:?}) should fail - missing dot prefix",
      ns_str
    );

    let err = result.unwrap_err();
    let err_msg = err.to_string();

    assert!(
      err_msg.contains( ns_str ),
      "Error message should include invalid namespace {:?}: {}",
      ns_str,
      err_msg
    );
  }
}

#[ test ]
fn namespace_display_trait()
{
  let empty = NamespaceType::new( "" ).unwrap();
  assert_eq!( format!( "{}", empty ), "" );

  let ns = NamespaceType::new( ".video" ).unwrap();
  assert_eq!( format!( "{}", ns ), ".video" );
}

#[ test ]
fn namespace_accessors()
{
  let ns_str = ".video";
  let ns = NamespaceType::new( ns_str ).unwrap();

  assert_eq!( ns.as_str(), ns_str );

  let inner = ns.into_inner();
  assert_eq!( inner, ns_str );
}

#[ test ]
fn namespace_clone_and_equality()
{
  let ns1 = NamespaceType::new( ".video" ).unwrap();
  let ns2 = ns1.clone();

  assert_eq!( ns1, ns2, "Cloned namespace should equal original" );

  let ns3 = NamespaceType::new( ".git" ).unwrap();
  assert_ne!( ns1, ns3, "Different namespaces should not be equal" );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn namespace_serde_json_serialize()
{
  let ns = NamespaceType::new( ".video" ).unwrap();
  let json = serde_json::to_string( &ns ).expect( "serialization should succeed" );

  assert_eq!( json, "\".video\"" );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn namespace_serde_json_deserialize_valid()
{
  let json = "\".video\"";
  let ns : NamespaceType = serde_json::from_str( json )
    .expect( "deserialization should succeed" );
  assert_eq!( ns.as_str(), ".video" );

  let json_empty = "\"\"";
  let empty : NamespaceType = serde_json::from_str( json_empty )
    .expect( "deserialization of empty namespace should succeed" );
  assert_eq!( empty.as_str(), "" );
  assert!( empty.is_root() );
}

/// TC-5: Serde deserialization rejects non-dot-prefixed namespace.
// test_kind: tc_spec(TC-5)
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn namespace_serde_json_deserialize_rejects_invalid()
{
  let json_invalid = "\"video\"";
  let result : Result< NamespaceType, _ > = serde_json::from_str( json_invalid );
  assert!(
    result.is_err(),
    "Deserialization should fail for namespace without dot prefix"
  );
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn namespace_serde_yaml_ng_deserialize_valid()
{
  let yaml = ".video";
  let ns : NamespaceType = serde_yaml_ng::from_str( yaml )
    .expect( "YAML deserialization should succeed" );
  assert_eq!( ns.as_str(), ".video" );

  let yaml_empty = "\"\"";
  let empty : NamespaceType = serde_yaml_ng::from_str( yaml_empty )
    .expect( "YAML deserialization of empty namespace should succeed" );
  assert!( empty.is_root() );
}
