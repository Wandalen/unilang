# Invariant Spec: Command Naming Conventions

### Scope

- **Purpose:** Verify the dot-prefix naming contract defined in `docs/invariant/005_command_naming.md` is enforced at every registration boundary
- **Responsibility:** Test cases confirming dot-prefix enforcement at runtime API and build-time manifest; namespace construction correctness
- **In Scope:** Runtime dot-prefix enforcement (runtime API rejects non-dot names), build-time enforcement (build.rs rejects manifest entries without dot), namespace construction invariant (build-time `compute_full_name()` and runtime `construct_full_command_name()` both always produce a dot-prefixed result), explicit-vs-omitted empty namespace distinction
- **Out of Scope:** Business naming choices (e.g., command categories, verb vs noun); help text conventions; FR-REG feature behavior

### IN-1: Runtime registration rejects command name without leading dot

- **Given:** A `CommandRegistry` and a `CommandDefinition` whose name is `"nodot"` (no leading dot)
- **When:** `register_with_routine(&mut registry, &definition)` is called
- **Then:** Returns an `Err` variant; the error message references the missing dot prefix; the registry remains unmodified

### IN-2: Runtime registration accepts command name with leading dot

- **Given:** A `CommandRegistry` and a `CommandDefinition` whose name is `".valid"`
- **When:** `register_with_routine(&mut registry, &definition)` is called
- **Then:** Returns `Ok(())`; `registry.get(".valid")` subsequently returns `Some(_)`

### IN-3: Namespace construction always produces dot-prefixed full name

- **Given:** A YAML command entry using the separate `namespace` + `name` fields, where `namespace` already carries its required dot prefix per FR-REG-6 Format 2 (e.g., `namespace: ".math"`, `name: "add"`)
- **When:** The build-time `compute_full_name()` function processes this entry
- **Then:** The resulting full name is `".math.add"` — exactly one leading dot

### IN-3b: Namespace lacking required dot prefix is rejected before full-name construction

- **Given:** A YAML command entry with `namespace: "math"` (no leading dot) and `name: "add"`
- **When:** `validate_namespace_core()` processes this entry ahead of `compute_full_name()`
- **Then:** Returns `Err` referencing the missing dot prefix on the namespace; `compute_full_name()` is never reached for this entry — the malformed namespace is rejected, not silently corrected

### IN-4: Build-time validation rejects manifest entry without dot prefix

- **Given:** A YAML manifest containing `name: "invalid"` with `namespace: ""` (produces `"invalid"` — no dot prefix)
- **When:** The `build.rs` validation pass processes this entry
- **Then:** The build fails with an actionable error referencing the missing dot prefix and the offending manifest entry; no static command is generated for the invalid entry

### IN-5: Explicit empty namespace is honored verbatim, not re-split from a compound name

- **Given:** A `CommandDefinition` deserialized from YAML with `name: ".session.delete"` and an explicit `namespace: ""`
- **When:** The command is deserialized and `full_name()` is called
- **Then:** `namespace()` remains `""` and `name()` remains `".session.delete"` verbatim — the compact-form convenience split (which activates only when `namespace` is *omitted*) does not run; `full_name()` returns `".session.delete"`

### IN-6: Runtime namespace concatenation is unconditional, regardless of the local name's shape

- **Given:** A `CommandDefinition` with a non-empty `namespace` and a `name` that itself contains a dot — including the case where `name` textually equals `namespace` (e.g. namespace `.enabled`, name `.enabled`), and the case of an auto-generated `.help` companion name
- **When:** `construct_full_command_name()` (via `full_name()`) computes the fully-qualified name
- **Then:** `namespace` and `name` are concatenated unconditionally — the result is always `"{namespace}{name}"` — never short-circuited by any dot-presence heuristic on `name`
