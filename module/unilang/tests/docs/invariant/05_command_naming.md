# Invariant Spec: Command Naming Conventions

### Scope

- **Purpose:** Verify the dot-prefix naming contract defined in `docs/invariant/005_command_naming.md` is enforced at every registration boundary
- **Responsibility:** Test cases confirming dot-prefix enforcement at runtime API and build-time manifest; namespace construction correctness
- **In Scope:** Runtime dot-prefix enforcement (runtime API rejects non-dot names), build-time enforcement (build.rs rejects manifest entries without dot), namespace construction invariant (compute_full_name always produces dot-prefixed result)
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

- **Given:** A YAML command entry using the separate `namespace` + `name` fields (e.g., `namespace: "math"`, `name: "add"`)
- **When:** The build-time `compute_full_name()` function processes this entry
- **Then:** The resulting full name is `".math.add"` — exactly one leading dot regardless of whether namespace is empty or populated

### IN-4: Build-time validation rejects manifest entry without dot prefix

- **Given:** A YAML manifest containing `name: "invalid"` with `namespace: ""` (produces `"invalid"` — no dot prefix)
- **When:** The `build.rs` validation pass processes this entry
- **Then:** The build fails with an actionable error referencing the missing dot prefix and the offending manifest entry; no static command is generated for the invalid entry
