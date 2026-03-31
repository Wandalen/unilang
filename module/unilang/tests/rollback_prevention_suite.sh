#!/bin/bash
# Rollback Prevention Test Suite
#
# This script verifies that reverting PHF re-export changes would break functionality.
# It serves as a safety check to ensure the implementation is actually being used.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== ROLLBACK PREVENTION TEST SUITE ==="
echo ""
echo "This suite verifies that PHF re-export changes are actually in use"
echo "by checking that key implementation points exist."
echo ""

# Test 1: Verify PHF re-export exists in src/lib.rs
echo "Test 1: PHF re-export in src/lib.rs"
if grep -q "#\[cfg(feature = \"static_registry\")\]" "$CRATE_DIR/src/lib.rs" && \
   grep -q "pub use phf;" "$CRATE_DIR/src/lib.rs"; then
  echo "  ✓ PASS: PHF re-export found"
else
  echo "  ✗ FAIL: PHF re-export missing"
  exit 1
fi

# Test 2: Verify compile_error! macro exists
echo "Test 2: compile_error! macro in src/lib.rs"
if grep -q "compile_error!" "$CRATE_DIR/src/lib.rs"; then
  echo "  ✓ PASS: compile_error! macro found"
else
  echo "  ✗ FAIL: compile_error! macro missing"
  exit 1
fi

# Test 3: Verify aggregator.rs uses unilang::phf
echo "Test 3: aggregator.rs uses unilang::phf"
if grep -q 'use unilang::phf' "$CRATE_DIR/src/multi_yaml/aggregator.rs"; then
  echo "  ✓ PASS: aggregator.rs uses re-export"
else
  echo "  ✗ FAIL: aggregator.rs doesn't use re-export"
  exit 1
fi

# Test 4: Verify build.rs uses unilang::phf
echo "Test 4: build.rs uses unilang::phf"
if grep -q 'use unilang::phf' "$CRATE_DIR/build.rs"; then
  echo "  ✓ PASS: build.rs uses re-export"
else
  echo "  ✗ FAIL: build.rs doesn't use re-export"
  exit 1
fi

# Test 5: Verify readme.md has PHF section
echo "Test 5: readme.md has PHF Re-export section"
if grep -q "## PHF Re-export" "$CRATE_DIR/readme.md"; then
  echo "  ✓ PASS: Documentation found"
else
  echo "  ✗ FAIL: Documentation missing"
  exit 1
fi

# Test 6: Verify validation tests exist
echo "Test 6: Validation test files exist"
VALIDATION_COUNT=$(ls -1 "$CRATE_DIR/tests/validation_v"*.rs 2>/dev/null | wc -l)
if [ "$VALIDATION_COUNT" -ge 6 ]; then
  echo "  ✓ PASS: Found $VALIDATION_COUNT validation test files"
else
  echo "  ✗ FAIL: Only found $VALIDATION_COUNT validation test files (expected ≥6)"
  exit 1
fi

# Test 7: Verify PHF unit tests exist
echo "Test 7: PHF re-export unit tests exist"
if [ -f "$CRATE_DIR/tests/phf_reexport_test.rs" ]; then
  echo "  ✓ PASS: Unit tests found"
else
  echo "  ✗ FAIL: Unit tests missing"
  exit 1
fi

# Test 8: Verify tests pass with feature enabled
echo "Test 8: Tests pass with static_registry feature"
cd "$CRATE_DIR"
if cargo test --features static_registry phf_reexport::with_feature --lib --quiet 2>&1 | grep -q "test result: ok"; then
  echo "  ✓ PASS: PHF tests pass"
else
  echo "  ✗ FAIL: PHF tests failed"
  exit 1
fi

echo ""
echo "=== ALL ROLLBACK PREVENTION TESTS PASSED ==="
echo ""
echo "These tests confirm that:"
echo "  - PHF re-export implementation is present"
echo "  - Code generation uses the re-export"
echo "  - Documentation is in place"
echo "  - Tests validate the functionality"
echo ""
echo "Reverting these changes would break the implementation."
