# API Spec: Error Codes

### Scope

- **Purpose:** Verify the public error code taxonomy defined in `docs/api/002_error_codes.md` — typed variants, string representations, and stability guarantees
- **Responsibility:** Test cases confirming that each `ErrorCode` variant is produced under its documented condition, that string representations match the catalog, and that the enum derives are present
- **In Scope:** `CommandNotFound`, `ArgumentMissing`, `ArgumentTypeMismatch`, `TooManyArguments`, `UnknownParameter`, `ValidationRuleFailed`, `ArgumentInteractiveRequired`, `CommandAlreadyExists`, `CommandNotImplemented`, `HelpRequested` (pipeline converts to output), `InternalError`; string form stability; `ErrorCode` derives (`Display`, `Debug`, `Clone`, `PartialEq`, `Eq`)
- **Out of Scope:** Error logging or tracing configuration; panic recovery; internal error wrapping mechanics; behavioral feature tests (covered in `feature/`)

### AP-1: CommandNotFound is returned for an unregistered command path

- **Given:** A `Pipeline` with an empty registry (or a registry that does not contain `.unknown`)
- **When:** `pipeline.run(".unknown")` is called
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::CommandNotFound`; `error_data.message` is non-empty

### AP-2: ArgumentMissing is returned when a required argument is absent

- **Given:** A `Pipeline` with `.greet` registered; `greet` has a required argument `"name"` with no default; input is `".greet"` (name argument omitted)
- **When:** `pipeline.run(".greet")` is called
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::ArgumentMissing`; `error_data.message` contains `"name"`

### AP-3: UnknownParameter is returned for a named argument not in the command definition

- **Given:** A `Pipeline` with `.greet` registered; `greet` has one argument `"name"`; input is `".greet typo::value"`
- **When:** `pipeline.run(".greet typo::value")` is called
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::UnknownParameter`; `error_data.message` contains `"typo"`

### AP-4: ArgumentTypeMismatch is returned when value cannot be coerced to declared Kind

- **Given:** A `Pipeline` with `.add` registered; `add` has argument `"x"` of `Kind::I32`; input is `".add x::not_a_number"`
- **When:** `pipeline.run(".add x::not_a_number")` is called
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::ArgumentTypeMismatch`

### AP-5: CommandAlreadyExists is returned when registering a duplicate command name

- **Given:** A `CommandRegistry` that already contains `.dup`
- **When:** `command_add_runtime` is called a second time with a definition named `".dup"`
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::CommandAlreadyExists`; the registry still contains the original definition

### AP-6: ErrorCode enum derives Clone, PartialEq, and Eq

- **Given:** Two instances of `ErrorCode::CommandNotFound`
- **When:** `a == b` is evaluated and `a.clone() == a` is evaluated
- **Then:** Both expressions return `true`; the code compiles without deriving these manually
