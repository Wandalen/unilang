//! Manual Test: Edge Cases and Error Conditions
//!
//! Tests boundary conditions and error scenarios.

#[test]
#[cfg(feature = "static_registry")]
fn test_empty_map() {
  use unilang::phf::{self, Map};

  static EMPTY: Map<&str, i32> = phf::phf_map! {};

  assert_eq!(EMPTY.len(), 0);
  assert!(EMPTY.is_empty());
  assert_eq!(EMPTY.get("anything"), None);

  println!("✅ Empty map works correctly");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_empty_set() {
  use unilang::phf::{self, Set};

  static EMPTY: Set<&str> = phf::phf_set! {};

  assert_eq!(EMPTY.len(), 0);
  assert!(EMPTY.is_empty());
  assert!(!EMPTY.contains("anything"));

  println!("✅ Empty set works correctly");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_single_element() {
  use unilang::phf::{self, Map, Set};

  static SINGLE_MAP: Map<&str, i32> = phf::phf_map! {
    "only" => 42,
  };

  static SINGLE_SET: Set<&str> = phf::phf_set! {
    "only"
  };

  assert_eq!(SINGLE_MAP.len(), 1);
  assert_eq!(SINGLE_MAP.get("only"), Some(&42));

  assert_eq!(SINGLE_SET.len(), 1);
  assert!(SINGLE_SET.contains("only"));

  println!("✅ Single element collections work correctly");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_large_map() {
  use unilang::phf::{self, Map};

  static LARGE: Map<u32, &str> = phf::phf_map! {
    0u32 => "zero", 1u32 => "one", 2u32 => "two", 3u32 => "three",
    4u32 => "four", 5u32 => "five", 6u32 => "six", 7u32 => "seven",
    8u32 => "eight", 9u32 => "nine", 10u32 => "ten", 11u32 => "eleven",
    12u32 => "twelve", 13u32 => "thirteen", 14u32 => "fourteen", 15u32 => "fifteen",
    16u32 => "sixteen", 17u32 => "seventeen", 18u32 => "eighteen", 19u32 => "nineteen",
    20u32 => "twenty",
  };

  assert_eq!(LARGE.len(), 21);
  assert_eq!(LARGE.get(&0), Some(&"zero"));
  assert_eq!(LARGE.get(&10), Some(&"ten"));
  assert_eq!(LARGE.get(&20), Some(&"twenty"));
  assert_eq!(LARGE.get(&100), None);

  println!("✅ Large map works correctly");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_unicode_keys() {
  use unilang::phf::{self, Map};

  static UNICODE: Map<&str, &str> = phf::phf_map! {
    "hello" => "English",
    "привет" => "Russian",
    "你好" => "Chinese",
    "こんにちは" => "Japanese",
    "안녕하세요" => "Korean",
    "مرحبا" => "Arabic",
  };

  assert_eq!(UNICODE.get("hello"), Some(&"English"));
  assert_eq!(UNICODE.get("привет"), Some(&"Russian"));
  assert_eq!(UNICODE.get("你好"), Some(&"Chinese"));
  assert_eq!(UNICODE.get("こんにちは"), Some(&"Japanese"));
  assert_eq!(UNICODE.len(), 6);

  println!("✅ Unicode keys work correctly");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_special_characters() {
  use unilang::phf::{self, Map};

  static SPECIAL: Map<&str, i32> = phf::phf_map! {
    "" => 0,  // Empty string key
    " " => 1,  // Space
    "\t" => 2,  // Tab
    "\n" => 3,  // Newline
    "\"" => 4,  // Quote
    "\\" => 5,  // Backslash
  };

  assert_eq!(SPECIAL.get(""), Some(&0));
  assert_eq!(SPECIAL.get(" "), Some(&1));
  assert_eq!(SPECIAL.get("\t"), Some(&2));
  assert_eq!(SPECIAL.len(), 6);

  println!("✅ Special characters work correctly");
}
