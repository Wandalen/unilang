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
- **When:** `register_with_routine(&mut registry, definition_for(".bar"))` is called and then `registry.get(".bar")` is called
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
- **When:** `register_with_routine(&mut registry, definition_for(".dynamic.cmd"))` is called, then both `".db.migrate"` and `".dynamic.cmd"` are queried
- **Then:** Both lookups succeed; static commands from the builder coexist with dynamically registered commands

### FT-13: Declarative JSON loading produces valid CommandDefinition

- **Given:** A JSON string defining command `.calc` with one `I64` argument `"value"` and description `"Calculate"`
- **When:** `CommandRegistry::load_from_json_str(&json)` is called
- **Then:** The registry contains `.calc` with the correct description and one argument named `"value"` of `Kind::I64`

### FT-14: Runtime API does not transform command names

- **Given:** A `CommandDefinition` with explicit name `.chat` and empty namespace, registered via `CommandRegistry::register_with_routine`
- **When:** The registry is queried for `.chat`
- **Then:** The command is found under exactly `.chat` with no added, removed, or transformed segments; no alternate spelling (e.g., `.Chat`, `.chat.` ) is created

### FT-15: YAML Format 1, Format 2, and omitted-namespace form all produce the identical resulting full command name

- **Given:** Three YAML command definitions describing the same logical command: Format 1 (`name: ".session.list"`, `namespace: ""` — explicit empty), Format 2 (`name: "list"`, `namespace: ".session"`), and the omitted-namespace compact form (`name: ".session.list"` with no `namespace` field at all)
- **When:** All three are loaded via `CommandRegistry::load_from_yaml_str` into separate registries
- **Then:** All three registries expose the command under the identical full name `.session.list`; Format 2 and the omitted-namespace form additionally agree on `namespace() == ".session"` and `name() == ".list"`, but Format 1's explicit empty namespace does NOT re-derive those fields — `namespace()` remains `""` and `name()` remains `".session.list"` verbatim, differing internally from the other two even though `full_name()` agrees

### FT-16: Build-time validation rejects manifest with duplicate command names

- **Given:** A YAML manifest file containing two command entries that both resolve to full name `.dup.command`
- **When:** The `build.rs` validation logic processes this manifest
- **Then:** A build error is produced showing both occurrences of `.dup.command`; the build does not silently keep only one definition

### FT-17: Registration rejects `multiple:true` argument with non-List storage type

- **Given:** A `CommandDefinition` with one argument having `attributes.multiple == true` and `kind == Kind::String` (not `Kind::List`)
- **When:** `CommandRegistry::register` is called with this definition
- **Then:** Returns `Err(Error::Registration(_))` with a message explaining that `multiple:true` requires `Kind::List` storage to prevent silent data loss; the command is not registered

### FT-18: Build-time validation rejects empty version string

- **Given:** A YAML manifest entry for a valid dot-prefixed command name but with `version: ""`
- **When:** The `build.rs` validation logic processes this manifest
- **Then:** A build error is produced stating the version string cannot be empty; the build does not succeed silently

### FT-19: StaticCommandRegistry auto-generates help companion for registered command

- **Given:** A `StaticCommandRegistry` with a command `.report` registered via `register_with_routine` with `auto_help_enabled` true
- **When:** The registry (or its `CommandRegistry` conversion via `.into()`) is queried for `.report.help`
- **Then:** Returns `Some(def)` for the generated help command, matching the auto-help behavior documented for `CommandRegistry::register()`

### FT-20: Registering static commands also registers the global `.help` command

- **Given:** A fresh `StaticCommandRegistry` converted into a `CommandRegistry` via `From<StaticCommandRegistry>`
- **When:** The resulting `CommandRegistry` is queried for `.help`
- **Then:** Returns `Some(def)` for the global help command, present even though no command explicitly registered `.help` itself

### FT-21: Duplicate registration via `register()` and `register_with_routine()` return different error variants

- **Given:** A `CommandRegistry` with `.dup` already registered
- **When:** `register()` is called again with a `CommandDefinition` named `.dup`, and separately `register_with_routine()` is called again with a `CommandDefinition` named `.dup`
- **Then:** `register()` returns `Err(Error::Registration(_))`; `register_with_routine()` returns `Err(Error::Execution(ErrorData))` carrying `ErrorCode::CommandAlreadyExists`; both reject the duplicate but surface it through distinct error variants

### FT-22: `CommandRegistryBuilder::build()` silently ignores registration errors while `build_checked()` propagates them

- **Given:** A `CommandRegistryBuilder` with two `command_with_routine` calls using the same command name (the second registration fails internally)
- **When:** `.build()` is called on one copy of the builder state and `.build_checked()` is called on an equivalent copy
- **Then:** `.build()` returns a `CommandRegistry` containing only the first command, with no error surfaced; `.build_checked()` returns `Err(Error::Registration(_))` whose message references the failed duplicate registration

### FT-23: Generated help companion carries a short alias, is hidden from listings, and disables recursive auto-help

- **Given:** A `CommandDefinition` named `.example` registered with `auto_help_enabled` true
- **When:** The registry generates its `.example.help` companion during registration
- **Then:** The generated companion has alias `.example.h`, `hidden_from_list() == true`, `priority() == 999`, and `auto_help_enabled() == false` (preventing a further `.example.help.help` from being generated)

### FT-24: Builder `status("deprecated")` produces structured deprecation metadata distinct from the active default

- **Given:** A `CommandDefinition` built via `CommandDefinition::former().name(".old").description("Old command").status("deprecated")` with a deprecation message set
- **When:** The resulting command's `status()` is inspected
- **Then:** Returns `CommandStatus::Deprecated { reason, .. }` where `reason` matches the supplied deprecation message, `is_deprecated() == true`, and `is_active() == false` — distinct from a command built without `.status(...)` which defaults to `CommandStatus::Active`

### FT-25: Dynamic module aggregation does not double-register a module's auto-generated `.help` companion

- **Given:** A `CliBuilder` with one dynamic YAML module (loaded via `dynamic_module_with_prefix`) containing a single authored command `.example` under prefix `.util`
- **When:** `builder.build()` is called
- **Then:** Build succeeds with no "already registered" error; `.util.example` and `.util.example.help` are both present, correctly prefixed; no unprefixed `.example.help` entry exists in the resulting registry
