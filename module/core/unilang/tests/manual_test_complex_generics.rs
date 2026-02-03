//! Manual Test: Complex Generic Parameters
//!
//! Tests that PHF works with complex generic parameters and nested types.

#[test]
#[cfg(feature = "static_registry")]
fn test_nested_types() {
  use unilang::phf::{self, Map};

  // Map with Vec values
  static MAP_WITH_VEC: Map<&str, &[i32]> = phf::phf_map! {
    "numbers" => &[1, 2, 3],
    "primes" => &[2, 3, 5, 7],
  };

  assert_eq!(MAP_WITH_VEC.get("numbers"), Some(&&[1, 2, 3][..]));
  assert_eq!(MAP_WITH_VEC.get("primes").unwrap().len(), 4);

  println!("✅ Nested types work correctly");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_tuple_values() {
  use unilang::phf::{self, Map};

  // Map with tuple values
  static COORDS: Map<&str, (i32, i32)> = phf::phf_map! {
    "origin" => (0, 0),
    "point_a" => (10, 20),
    "point_b" => (-5, 15),
  };

  assert_eq!(COORDS.get("origin"), Some(&(0, 0)));
  assert_eq!(COORDS.get("point_a"), Some(&(10, 20)));

  println!("✅ Tuple values work correctly");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_integer_keys() {
  use unilang::phf::{self, Map};

  // Map with integer keys
  static INT_MAP: Map<u32, &str> = phf::phf_map! {
    1u32 => "one",
    2u32 => "two",
    100u32 => "hundred",
  };

  assert_eq!(INT_MAP.get(&1), Some(&"one"));
  assert_eq!(INT_MAP.get(&100), Some(&"hundred"));
  assert_eq!(INT_MAP.get(&50), None);

  println!("✅ Integer keys work correctly");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_byte_string_keys() {
  use unilang::phf::{self, Map};

  // Map with byte string keys
  static BYTE_MAP: Map<&[u8], &str> = phf::phf_map! {
    b"hello" => "greeting",
    b"world" => "noun",
  };

  assert_eq!(BYTE_MAP.get(b"hello" as &[u8]), Some(&"greeting"));
  assert_eq!(BYTE_MAP.get(b"world" as &[u8]), Some(&"noun"));

  println!("✅ Byte string keys work correctly");
}
