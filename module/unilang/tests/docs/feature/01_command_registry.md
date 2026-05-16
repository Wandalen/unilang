# Feature Spec: Command Registry

### Scope

- **Purpose:** Verify all FR-REG behavioral requirements for the command registry feature
- **Responsibility:** Test cases covering static registration, dynamic registration, namespace resolution, alias resolution, explicit naming enforcement, and feature parity
- **In Scope:** FR-REG-1 (static PHF registration), FR-REG-2 (dynamic runtime registration), FR-REG-3 (declarative YAML/JSON loading), FR-REG-4 (namespace support), FR-REG-5 (alias resolution), FR-REG-6 (explicit naming enforcement), FR-REG-7 (CLI module aggregation), FR-REG-8 (static/dynamic parity), FR-REG-9 (build-time validation)
- **Out of Scope:** Argument binding (FR-ARG); pipeline orchestration (FR-PIPE); help output formatting (FR-HELP)

### FT-1: Static PHF registry lookup returns registered command

- **Given:** A `StaticCommandRegistry` built from a `StaticCommandMap` containing command `.foo` with a known description
- **When:** `registry.get(".foo")` is called
- **Then:** Returns `Some(def)` where `def.name() == ".foo"` and description matches the registered value

### FT-2: Dynamic runtime registration makes command accessible

- **Given:** A `CommandRegistry` initialized from the static map
- **When:** `command_add_runtime(&mut registry, definition_for(".bar"))` is called and then `registry.get(".bar")` is called
- **Then:** Returns `Some(def)` with the dynamically registered definition; `.foo` from static initialization still accessible

### FT-3: Dot prefix is mandatory; command without leading dot is rejected

- **Given:** A `CommandDefinition` being constructed with name `"noDotPrefix"`
- **When:** The definition is validated (either at build time or registration time)
- **Then:** An error is returned or compilation fails with a message indicating that command names must begin with `.`

### FT-4: Alias resolution returns canonical command definition

- **Given:** A registry where alias `".f"` maps to canonical command `".foo"`
- **When:** `registry.get(".f")` is called
- **Then:** Returns `Some(def)` where `def.name() == ".foo"` (the canonical name, not the alias)

### FT-5: Multi-YAML conflict detection returns error for duplicate names

- **Given:** Two YAML sources both defining a command named `".dup"` with different descriptions
- **When:** The declarative loader merges the two sources
- **Then:** Returns an error indicating a conflict on `".dup"`; no partial merge is applied

### FT-6: Namespace-nested command distinguishable from top-level command

- **Given:** A registry containing both `".foo"` and `".foo.bar"` as distinct commands
- **When:** `registry.get(".foo")` and `registry.get(".foo.bar")` are called separately
- **Then:** Each call returns the respective distinct definition; neither lookup returns the other

### FT-7: StaticCommandRegistry and CommandRegistry return identical definitions

- **Given:** The same `StaticCommandMap` used to initialize both a `StaticCommandRegistry` and a `CommandRegistry`
- **When:** `get(name)` is called on both registries for the same command name
- **Then:** Both return equivalent definitions (same name, same argument count, same description)
