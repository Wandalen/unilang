# API Spec: Public Types

### Scope

- **Purpose:** Verify the public API surface and compatibility guarantees defined in `docs/api/001_public_types.md`
- **Responsibility:** Test cases covering all 17 public types (`CommandDefinition`, `ArgumentDefinition`, `ArgumentAttributes`, `Kind`, `ValidationRule`, `OutputData`, `ErrorData`, `Value`, `StaticCommandMap`, `StaticCommandDefinition`, `StaticArgumentDefinition`, `StaticArgumentAttributes`, `StaticKind`, `StaticValidationRule`, `VerifiedCommand`, `Pipeline`, `CommandRegistry`) and the Phase 2 redesign invariants (private fields, newtypes, type-state builder)
- **In Scope:** All named public types in `docs/api/001_public_types.md`; semver compatibility guarantees (stable env var names, additive `StaticCommandDefinition` fields); Phase 2 private-field enforcement
- **Out of Scope:** Internal module layout; behavioral feature tests (covered in `feature/`)

### AP-1: CommandDefinition builder requires name field — omitting it is a compile error

- **Given:** Rust code calling `CommandDefinition::former().description("desc").end()`  (name field omitted)
- **When:** The code is compiled
- **Then:** Compilation fails with a type error from the type-state builder; the error message references the missing `name` field

### AP-2: Full Pipeline round-trip returns VerifiedCommand with correct arguments

- **Given:** A `Pipeline` with `.echo` registered; `echo` has one `String` argument `"msg"` and input `".echo msg::hello"`
- **When:** `pipeline.run(".echo msg::hello")` is called
- **Then:** Returns `Ok(VerifiedCommand { name: ".echo", arguments: { "msg": Value::String("hello") } })` with correct type

### AP-3: All 15 Kind variants are constructable without error

- **Given:** Code that constructs a `Value` for each of the 15 `Kind` variants (Bool, I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, String, Path)
- **When:** Each `Value` is constructed from a valid representative input
- **Then:** All 15 constructions succeed; no `Kind` variant panics or returns an error on valid input

### AP-4: CommandRegistry lookup returns expected definition for registered command

- **Given:** A `CommandRegistry` containing `.query` with description `"Run a query"`
- **When:** `registry.get(".query")` is called
- **Then:** Returns `Some(def)` where `def.description()` returns `"Run a query"`

### AP-5: CommandDefinition fields are private — direct field access does not compile

- **Given:** Rust code attempting to read `definition.name` as a direct struct field (bypassing the accessor method)
- **When:** The code is compiled
- **Then:** Compilation fails with a private-field access error; the `name()` accessor method is the only valid access path

### AP-6: StaticCommandDefinition equality with CommandDefinition for same command

- **Given:** A command `.greet` represented as both a `StaticCommandDefinition` (in the PHF map) and a `CommandDefinition` (constructed via builder)
- **When:** The name and argument count are compared between the two representations
- **Then:** Both agree on name and argument count; conversion via `From<&StaticCommandDefinition>` produces a `CommandDefinition` with identical observable attributes

### AP-7: OutputData serializes to JSON without loss of command name

- **Given:** An `OutputData` instance with `command_name: ".run"` and one key-value result entry
- **When:** `serde_json::to_string(&output_data)` is called
- **Then:** The resulting JSON string contains `"command_name":".run"`; deserialization round-trips back to the original value

### AP-8: Stable env var names — UNILANG_HELP_VERBOSITY recognized across minor versions

- **Given:** `UNILANG_HELP_VERBOSITY=2` set in the environment and the help API called
- **When:** The help output is generated
- **Then:** The verbosity is applied (output is at level 2, between minimal and maximal); the env var name `UNILANG_HELP_VERBOSITY` is stable and not renamed between patch versions

### AP-9: process_command_from_argv preserves argument boundaries without re-quoting

- **Given:** A `Pipeline` with `.echo` registered; `echo` has one `String` argument `"msg"` and argv array `["prog", ".echo", "msg::hello world"]`
- **When:** `pipeline.process_command_from_argv(argv)` is called
- **Then:** Returns `Ok` with `arguments["msg"] == Value::String("hello world")`; the space is preserved because argv boundaries prevent shell re-splitting

### AP-10: process_batch collects all results regardless of individual failures

- **Given:** A `Pipeline` with `.ok` (always succeeds) and `.fail` (always errors) registered; batch input `[".fail", ".ok", ".fail"]`
- **When:** `pipeline.process_batch(inputs)` is called
- **Then:** Returns a list of 3 results: `[Err, Ok, Err]`; all three commands are executed and no short-circuiting occurs

### AP-11: StaticArgumentAttributes conversion to ArgumentAttributes preserves all five fields

- **Given:** A `StaticArgumentAttributes` constructed via `StaticArgumentAttributes::new().with_optional(true).with_multiple(true).with_default("fallback").with_sensitive(true).with_interactive(true)`
- **When:** `ArgumentAttributes::from(&static_attrs)` is called (via the `From<&StaticArgumentAttributes>` impl)
- **Then:** The resulting `ArgumentAttributes` has `optional == true`, `multiple == true`, `default == Some("fallback".to_string())`, `sensitive == true`, and `interactive == true` — all five fields match the static source

### AP-12: StaticKind conversion to Kind preserves nested List and Map structure

- **Given:** A `StaticKind::List(&StaticKind::Integer, Some(','))` and a `StaticKind::Enum(&["red", "green", "blue"])`
- **When:** `Kind::from(&static_kind)` is called on each
- **Then:** The `List` variant converts to `Kind::List(Box::new(Kind::Integer), Some(','))`; the `Enum` variant converts to `Kind::Enum(vec!["red".to_string(), "green".to_string(), "blue".to_string()])` — nested type structure and delimiter are preserved

### AP-13: StaticValidationRule conversion to ValidationRule preserves rule parameters for all 6 variants

- **Given:** One `StaticValidationRule` instance for each of the 6 variants (`Min(1.0)`, `Max(100.0)`, `MinLength(3)`, `MaxLength(50)`, `Pattern("^[a-z]+$")`, `MinItems(2)`)
- **When:** `ValidationRule::from(&static_rule)` is called on each instance
- **Then:** Each converts to the matching `ValidationRule` variant with the identical parameter value (e.g., `StaticValidationRule::Min(1.0)` → `ValidationRule::Min(1.0)`, `StaticValidationRule::Pattern("^[a-z]+$")` → `ValidationRule::Pattern("^[a-z]+$".to_string())`)

### AP-14: StaticArgumentDefinition conversion to ArgumentDefinition preserves name, kind, and attributes

- **Given:** A `StaticArgumentDefinition` constructed via `StaticArgumentDefinition::new("count", StaticKind::Integer, "A count value").with_attributes(StaticArgumentAttributes::new().with_optional(true))`
- **When:** `ArgumentDefinition::from(&static_arg)` is called (via the `From<&StaticArgumentDefinition>` impl)
- **Then:** The resulting `ArgumentDefinition` has `name == "count"`, `kind == Kind::Integer`, `description == "A count value"`, and `attributes.optional == true`

### AP-15: StaticCommandMap get and contains_key return O(1) lookups matching len and is_empty

- **Given:** A `StaticCommandMap` built from a PHF map containing exactly one entry `.greet` (via `StaticCommandMap::from_phf_internal`)
- **When:** `map.get(".greet")`, `map.contains_key(".greet")`, `map.contains_key(".missing")`, `map.len()`, and `map.is_empty()` are called
- **Then:** `get(".greet")` returns `Some(&StaticCommandDefinition)` with `name == ".greet"`; `contains_key(".greet")` returns `true`; `contains_key(".missing")` returns `false`; `len()` returns `1`; `is_empty()` returns `false`

### AP-16: UNILANG_VERBOSITY env var controls CLI binary logging verbosity

- **Given:** `UNILANG_VERBOSITY=2` set in the environment before a CLI binary using `unilang` starts
- **When:** The binary reads the verbosity setting at startup
- **Then:** The debug-level (2) logging verbosity is applied; the env var name `UNILANG_VERBOSITY` is distinct from `UNILANG_HELP_VERBOSITY` and governs general CLI logging, not help output detail

### AP-17: UNILANG_HELP_HIDE_VERSION sets the HelpDisplayOptions.show_version field

- **Given:** `UNILANG_HELP_HIDE_VERSION=1` set in the environment
- **When:** `HelpDisplayOptions::default().with_env_overrides()` is called
- **Then:** The resulting `show_version` field is `false`; unsetting the variable restores `show_version` to `true`
- **Known gap:** no rendering call site consults `HelpDisplayOptions` today (both `HelpGenerator` and `.command.help` check `CommandDefinition::show_version_in_help()` instead), so this env var has no observable effect on rendered help output — use `CommandDefinition::with_show_version_in_help(false)` for that

### AP-18: VerifiedCommand typed extraction methods return None/Err appropriately for missing arguments

- **Given:** A `VerifiedCommand` with no `"count"` argument bound (argument was optional and omitted)
- **When:** `verified_command.get_integer("count")`, `verified_command.has_argument("count")`, and `verified_command.get_value("count")` are called
- **Then:** `get_integer("count")` returns `None`; `has_argument("count")` returns `false`; `get_value("count")` returns `None` — all typed extraction methods agree the argument is absent without panicking

### AP-19: Configuration Utilities typed extraction parses u32 and bool from ConfigMap

- **Given:** A `ConfigMap<&str>` (feature `json_parser` enabled) containing key `"port"` mapped to `JsonValue::Number(8080)` and key `"enabled"` mapped to `JsonValue::Bool(true)`
- **When:** The typed `u32` extraction function is called for `"port"` and the typed `bool` extraction function is called for `"enabled"`
- **Then:** The `"port"` extraction returns `Ok(8080u32)`; the `"enabled"` extraction returns `Ok(true)`; both preserve the declared type without manual `JsonValue` matching
