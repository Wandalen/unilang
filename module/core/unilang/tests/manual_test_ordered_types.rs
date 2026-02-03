//! Manual Test: PHF Ordered Types
//!
//! Tests that OrderedMap and OrderedSet types work correctly through re-export.

#[test]
#[cfg(feature = "static_registry")]
fn test_ordered_map() {
  use unilang::phf::{self, OrderedMap};

  static ORDERED: OrderedMap<&str, i32> = phf::phf_ordered_map! {
    "first" => 1,
    "second" => 2,
    "third" => 3,
  };

  // Verify lookups work
  assert_eq!(ORDERED.get("first"), Some(&1));
  assert_eq!(ORDERED.get("second"), Some(&2));
  assert_eq!(ORDERED.len(), 3);

  // Verify iteration maintains insertion order
  let keys: Vec<&&str> = ORDERED.keys().collect();
  assert_eq!(keys, vec![&"first", &"second", &"third"]);

  println!("✅ OrderedMap works correctly");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_ordered_set() {
  use unilang::phf::{self, OrderedSet};

  static ORDERED: OrderedSet<&str> = phf::phf_ordered_set! {
    "alpha", "beta", "gamma"
  };

  // Verify membership
  assert!(ORDERED.contains("alpha"));
  assert!(ORDERED.contains("beta"));
  assert!(!ORDERED.contains("delta"));
  assert_eq!(ORDERED.len(), 3);

  // Verify iteration maintains insertion order
  let items: Vec<&&str> = ORDERED.iter().collect();
  assert_eq!(items, vec![&"alpha", &"beta", &"gamma"]);

  println!("✅ OrderedSet works correctly");
}
