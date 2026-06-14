# API Spec: Public Types

### Scope

- **Purpose:** Verify the public API surface and compatibility guarantees defined in `docs/api/001_public_types.md`
- **Responsibility:** Test cases covering all 10 public types (`CommandDefinition`, `ArgumentDefinition`, `CommandRegistry`, `Pipeline`, `Value`, `Kind`, `StaticCommandDefinition`, `StaticArgumentDefinition`, `StaticCommandMap`, `OutputData`, `ErrorData`) and the Phase 2 redesign invariants (private fields, newtypes, type-state builder)
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
