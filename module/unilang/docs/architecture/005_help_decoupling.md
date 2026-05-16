# Architecture: Help Decoupling

### Scope

- **Purpose:** Document the help system decoupling migration rationale, steps, and status
- **Responsibility:** ADR for separating help generation from command registration
- **In Scope:** Migration rationale, migration steps, completion status
- **Out of Scope:** Current help system behavior (see feature/004_help_system.md)

### Migration Overview

**Status:** ✅ COMPLETE (as of 2025_12_04)
**Final State:** 0 domain-specific patterns, 2 generic algorithms, 100% tests passing

This migration successfully removed all application-specific coupling from the unilang help system, making it truly generic and reusable across any domain. The help system is now completely domain-agnostic and implements only generic transformation algorithms.

### Migration Goals

1. **Generic Algorithm:** Replace pattern-matching `auto_categorize()` with algorithm that returns empty string (categories must be explicit via `CommandDefinition::category()`)
2. **Universal Formatting:** Replace hardcoded category mappings in `format_category_name()` with generic snake_case → Title Case transformation
3. **Self-Contained Documentation:** Remove all application-specific references from comments and documentation
4. **Test Independence:** Update test assertions to validate generic behavior, not specific CLI patterns

### Target Architecture

**Before (Coupled):** The `auto_categorize()` function contained 12+ domain-specific pattern matches on command name prefixes (e.g., `.git`, `.remove`), returning domain-specific category strings such as `"git_operations"` and `"removal_operations"`. The `format_category_name()` function contained 15+ hardcoded category-to-display-name mappings (e.g., `"repository_management"` → `"REPOSITORY MANAGEMENT"`).

**After (Generic):** `auto_categorize()` returns an empty string — categories must be specified explicitly via `CommandDefinition::category()`; the framework never infers categories from command names. `format_category_name()` uses a generic snake_case → Title Case transformation: split on underscores, capitalize the first character of each word, rejoin with spaces.

### Migration Phases

**Phase 0: Baseline Measurement** ✅ COMPLETE
- Baseline metrics: 37 old patterns identified
- Category 1 (auto_categorize): 6 old patterns
- Category 2 (format_category_name): 16 old patterns
- Category 3 (Documentation): 8 old patterns
- Category 4 (Tests): 7 old patterns

**Phase 1a: TDD — auto_categorize Simplification** ✅ COMPLETE
- Created failing tests expecting empty string return (5 tests)
- Replaced pattern matching with empty string return
- Documented architectural requirement: categories must be explicit
- Result: Eliminated all domain-specific pattern matching

**Phase 1b: TDD — format_category_name Genericization** ✅ COMPLETE
- Created failing tests for Title Case transformation (7 tests)
- Implemented generic split/map/join algorithm
- Documented transformation: snake_case → Title Case
- Result: Eliminated all hardcoded category mappings

**Phases 2-8:** ✅ COMPLETE — All domain-specific coupling removed from help system. Comprehensive genericization (25+ files updated):

**Example Files (21 files):**
- Namespaces: `.math`/`.file`/`.text`/`.fs`/`.db`/`.network` → `.cmd1`/`.cmd2`/`.cmd3`/`.svc1`
- Hints: "Mathematical", "Text processing", "File system" → "Generic operation/processing/listing"
- Comments: All domain references removed
- Variable names: `math_command`, `math_routine` → `cmd1_*`
- Example strings: `"math.add"`, `"text.upper"` → `"cmd1.add"`, `"cmd3.upper"`
- Tags: `"math"`, `"arithmetic"` → `"cmd1"`, `"generic"`
- Module names: `math_cli_static`, `MathCliModule` → `cmd1_cli_static`, `Cmd1CliModule`

**Source Files (5 files):**
- `help.rs`: Comment examples genericized
- `registry.rs`: Help examples genericized, application attribution removed
- `simd_tokenizer.rs`: Test strings genericized
- `command_validation.rs`: Doc examples genericized, domain-specific bug references replaced with generic descriptions
- `pipeline.rs`: Doc comment examples genericized

**Verification Results:**
- Domain references in src/: 0
- Domain references in examples/: 0
- Application references: 0
- Test suite: 100% success rate (845+ tests)
- Zero clippy warnings

### Acceptance Criteria

Migration is considered complete when ALL of the following conditions are met:

1. **Zero Old Patterns:** Measurement script reports 0 old patterns across all categories
2. **Full New Patterns:** Measurement script reports 4/4 new pattern score
3. **100% Migration Progress:** Measurement script reports 100% completion
4. **All Tests Passing:** Full test suite passes with `w3 .test l::3`
5. **No Application References:** Zero mentions of specific application names in `src/help.rs`
6. **Generic Documentation:** All comments are self-contained and domain-agnostic
7. **Test Validation:** Tests validate generic behavior, not specific CLI patterns

### Verification Strategy

The migration uses 7-layer verification:

1. **Quantitative Metrics:** Automated measurement script tracks old/new pattern counts
2. **Test-Driven Development:** RED-GREEN-REFACTOR cycle for each change
3. **Rulebook Compliance:** All changes follow CLAUDE.md rulebook requirements
4. **Absence Verification:** Explicit validation that old patterns are gone
5. **Authenticity Verification:** New code demonstrates truly generic behavior
6. **Impossibility Verification:** Architecture makes coupling impossible to reintroduce
7. **Irreversibility Verification:** Changes are complete replacements, not toggles

### Migration Insights

1. **Test Data vs Documentation**: Test files (tests/*.rs) appropriately contain domain-specific test data as fixtures. The migration targeted "Examples" per objectives, not test fixtures. Test data is distinct from documentation examples.

2. **Knowledge Preservation vs Coupling**: Bug patterns discovered in specific applications should be documented generically to preserve the knowledge without creating coupling. Example:
   - ❌ Coupling: "Prevents the [application] bug pattern where..."
   - ✅ Generic: "Prevents silent data loss where..."
   The technical knowledge is preserved, but application attribution is removed.

3. **Comprehensive Genericization Scope**: Genericization must extend to ALL documentation artifacts:
   - Source code comments (`*.rs`)
   - Example documentation (`examples/*.md`)
   - Error messages and diagnostic strings
   - Inline documentation comments

### Success Metrics

Final verification results:
- ✅ Test suite: All 845+ tests passing with zero failures (100% success rate)
- ✅ Code review: No domain-specific pattern matching in help system
- ✅ Architectural validation: Generic algorithms incapable of domain inference
- ✅ Functional verification: `auto_categorize()` returns empty string for all inputs
- ✅ Functional verification: `format_category_name()` uses generic Title Case algorithm
- ✅ No hardcoded category mappings remain
- ✅ All clippy checks passing

**Status:** ✅ MIGRATION COMPLETE
**Completed:** 2025_12_04
**Files Modified:** `src/help.rs`, 2 new test files, 1 existing test updated
**Breaking Changes:** Applications relying on auto-categorization must now specify categories explicitly

### Feature Instances

| File | Relationship |
|------|--------------|
| [004_help_system.md](../feature/004_help_system.md) | FR-HELP-* requirements this migration supports |

### Invariant Instances

| File | Relationship |
|------|--------------|
| [003_governing_principles.md](../invariant/003_governing_principles.md) | Minimum implicit magic principle motivating this migration |
