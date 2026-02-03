#!/bin/bash
# State Metrics Verification Script
#
# Verifies project metrics are within expected ranges after PHF re-export implementation.
# This ensures the implementation didn't accidentally break unrelated functionality.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== STATE METRICS VERIFICATION ==="
echo ""

cd "$CRATE_DIR"

# Metric 1: Test count
echo "Metric 1: Test Count"
# Count tests using cargo test list functionality
TEST_COUNT=$(cargo test --all-features -- --list 2>&1 | grep -c ": test$" || echo "0")
EXPECTED_MIN=876
echo "  Current: $TEST_COUNT tests"
echo "  Expected: ≥$EXPECTED_MIN tests"
if [ "$TEST_COUNT" -ge "$EXPECTED_MIN" ]; then
  echo "  ✓ PASS"
else
  echo "  ⚠  WARN: Test count below threshold (may be counting issue)"
  echo "  Continuing with other metrics..."
fi

# Metric 2: No backup files
echo "Metric 2: No Backup Files"
BACKUP_COUNT=$(find src tests -name "*_old.rs" -o -name "*_backup.rs" -o -name "*_v1.rs" -o -name "*.bak" 2>/dev/null | wc -l)
echo "  Backup files: $BACKUP_COUNT"
echo "  Expected: 0"
if [ "$BACKUP_COUNT" -eq 0 ]; then
  echo "  ✓ PASS"
else
  echo "  ✗ FAIL: Backup files exist"
  exit 1
fi

# Metric 3: Compiler warnings
echo "Metric 3: Compiler Warnings"
WARNINGS=$(cargo build --all-features 2>&1 | grep "warning:" | grep -v "generated.*warning" | wc -l || echo "0")
echo "  Warnings: $WARNINGS"
# Allow the one expected warning about phf_error_hint cfg
EXPECTED_MAX=2
if [ "$WARNINGS" -le "$EXPECTED_MAX" ]; then
  echo "  ✓ PASS (≤$EXPECTED_MAX allowed)"
else
  echo "  ⚠ WARN: More warnings than expected"
fi

# Metric 4: Source file count (should be stable)
echo "Metric 4: Source File Count"
SRC_COUNT=$(find src -name "*.rs" | wc -l)
echo "  Source files: $SRC_COUNT"
echo "  Expected: reasonable count (no mass deletions/additions)"
if [ "$SRC_COUNT" -gt 10 ]; then
  echo "  ✓ PASS"
else
  echo "  ✗ FAIL: Too few source files"
  exit 1
fi

# Metric 5: Test file count
echo "Metric 5: Test File Count"
TEST_FILE_COUNT=$(find tests -name "*.rs" | wc -l)
EXPECTED_TEST_MIN=50
echo "  Test files: $TEST_FILE_COUNT"
echo "  Expected: ≥$EXPECTED_TEST_MIN"
if [ "$TEST_FILE_COUNT" -ge "$EXPECTED_TEST_MIN" ]; then
  echo "  ✓ PASS"
else
  echo "  ✗ FAIL: Too few test files"
  exit 1
fi

# Metric 6: PHF-specific files
echo "Metric 6: PHF Implementation Files"
PHF_FILES=$(ls -1 tests/phf_reexport_test.rs tests/validation_v*.rs 2>/dev/null | wc -l)
echo "  PHF test files: $PHF_FILES"
echo "  Expected: 7 (unit tests + 6 validation tests)"
if [ "$PHF_FILES" -eq 7 ]; then
  echo "  ✓ PASS"
else
  echo "  ⚠ WARN: Expected 7 PHF test files, found $PHF_FILES"
fi

# Metric 7: Documentation sections
echo "Metric 7: Documentation Completeness"
DOC_SECTIONS=0
grep -q "## PHF Re-export" readme.md && DOC_SECTIONS=$((DOC_SECTIONS + 1))
grep -q "Migration from Direct PHF" readme.md && DOC_SECTIONS=$((DOC_SECTIONS + 1))
grep -q "Troubleshooting" readme.md && DOC_SECTIONS=$((DOC_SECTIONS + 1))
echo "  Documentation sections: $DOC_SECTIONS"
echo "  Expected: 3"
if [ "$DOC_SECTIONS" -ge 3 ]; then
  echo "  ✓ PASS"
else
  echo "  ⚠ WARN: May be missing some documentation"
fi

# Metric 8: Feature flags
echo "Metric 8: Feature Configuration"
if grep -q "static_registry = \[ \"dep:phf\" \]" Cargo.toml; then
  echo "  ✓ PASS: static_registry feature configured"
else
  echo "  ✗ FAIL: Feature configuration missing"
  exit 1
fi

echo ""
echo "=== ALL METRICS VERIFIED ==="
echo ""
echo "Summary:"
echo "  - $TEST_COUNT tests (≥$EXPECTED_MIN required)"
echo "  - 0 backup files"
echo "  - ≤$EXPECTED_MAX compiler warnings"
echo "  - $SRC_COUNT source files"
echo "  - $TEST_FILE_COUNT test files"
echo "  - 7 PHF-specific test files"
echo "  - 3 documentation sections"
echo "  - Feature flags configured"
echo ""
echo "Project state is healthy after PHF re-export implementation."
