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

### FT-8: Declarative YAML loading produces valid CommandDefinition

- **Given:** A YAML string defining command `.greet` with one `String` argument `"name"` and description `"Say hello"`
- **When:** `CommandRegistry::load_from_yaml_str(&yaml)` is called
- **Then:** The registry contains `.greet` with the correct description and one argument named `"name"` of `Kind::String`

### FT-9: Build-time validation rejects manifest with invalid command name

- **Given:** A YAML manifest file containing a command entry with name `"no_dot"` and empty namespace
- **When:** The `build.rs` validation logic processes this manifest
- **Then:** A build error is produced with an actionable message referencing the missing dot prefix; the build does not succeed silently

### FT-10: CliBuilder module registration produces prefixed commands

- **Given:** A `CliBuilder` with module `"db"` registered via `static_module_with_prefix("db_module", ".db", db_commands)` containing `".migrate"` and `".rollback"`
- **When:** `builder.build_static()` is called and the resulting map is queried for `".db.migrate"`
- **Then:** Returns `Some(def)` with the prefix-applied name `".db.migrate"`; `".migrate"` alone is NOT found in the map

### FT-11: CliBuilder conflict detection rejects duplicate command names across modules

- **Given:** Two modules both producing a command with the final name `".shared.run"` (via different prefix+name combinations)
- **When:** Both modules are registered in the same `CliBuilder`
- **Then:** Returns an error indicating a naming conflict on `".shared.run"`; the builder does not produce a partial map

### FT-12: CliBuilder build_hybrid produces registry supporting both static and dynamic commands

- **Given:** A `CliBuilder` with one static module containing `".db.migrate"` and `build_hybrid()` called to produce a `CommandRegistry`
- **When:** `command_add_runtime(&mut registry, definition_for(".dynamic.cmd"))` is called, then both `".db.migrate"` and `".dynamic.cmd"` are queried
- **Then:** Both lookups succeed; static commands from the builder coexist with dynamically registered commands

### FT-13: Declarative JSON loading produces valid CommandDefinition

- **Given:** A JSON string defining command `.calc` with one `I64` argument `"value"` and description `"Calculate"`
- **When:** `CommandRegistry::load_from_json_str(&json)` is called
- **Then:** The registry contains `.calc` with the correct description and one argument named `"value"` of `Kind::I64`
