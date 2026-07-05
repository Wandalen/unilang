# API Spec: Error Codes

### Scope

- **Purpose:** Verify the public error code taxonomy defined in `docs/api/002_error_codes.md` — typed variants, string representations, and stability guarantees
- **Responsibility:** Test cases confirming that each `ErrorCode` variant is produced under its documented condition, that string representations match the catalog, and that the enum derives are present
- **In Scope:** `CommandNotFound`, `ArgumentMissing`, `ArgumentTypeMismatch`, `TooManyArguments`, `UnknownParameter`, `ValidationRuleFailed`, `ArgumentInteractiveRequired`, `CommandAlreadyExists`, `CommandNotImplemented`, `TypeMismatch`, `HelpRequested` (pipeline converts to output), `InternalError`; string form stability; `ErrorCode` derives (`Display`, `Debug`, `Clone`, `PartialEq`, `Eq`)
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
- **When:** `register_with_routine` is called a second time with a definition named `".dup"`
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::CommandAlreadyExists`; the registry still contains the original definition

### AP-6: ErrorCode enum derives Clone, PartialEq, and Eq

- **Given:** Two instances of `ErrorCode::CommandNotFound`
- **When:** `a == b` is evaluated and `a.clone() == a` is evaluated
- **Then:** Both expressions return `true`; the code compiles without deriving these manually

### AP-7: TooManyArguments is returned for excess positional arguments

- **Given:** A `Pipeline` with `.cmd` registered; `.cmd` has exactly one positional argument; input is `".cmd val1 val2 val3"` (three values for one slot)
- **When:** `pipeline.run(".cmd val1 val2 val3")` is called
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::TooManyArguments`

### AP-8: ValidationRuleFailed is returned for constraint violation

- **Given:** A `Pipeline` with `.cmd` registered; argument `"count"` of `Kind::I64` has `ValidationRule::Min(1)`; input is `".cmd count::0"`
- **When:** `pipeline.run(".cmd count::0")` is called
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::ValidationRuleFailed`; `error_data.message` references the violated rule

### AP-9: ArgumentInteractiveRequired is returned for missing interactive argument

- **Given:** A `Pipeline` with `.login` registered; argument `"password"` has `interactive: true` and is required with no default; input is `".login"`
- **When:** `pipeline.run(".login")` is called
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::ArgumentInteractiveRequired`; `error_data.message` contains `"password"`

### AP-10: CommandNotImplemented is returned for command with no bound routine

- **Given:** A `CommandRegistry` where `.stub` is registered with a definition but no `Routine` closure bound
- **When:** The pipeline attempts to execute `.stub`
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::CommandNotImplemented`

### AP-11: HelpRequested is converted to successful OutputData by pipeline

- **Given:** A `Pipeline` with `.greet` registered; input is `".greet ??"`
- **When:** `pipeline.run(".greet ??")` is called
- **Then:** Returns `Ok(output_data)` (not `Err`); the pipeline converts the internal `HelpRequested` signal to a successful `OutputData` containing help text

### AP-12: InternalError produced for unexpected system error

- **Given:** A scenario that triggers an unexpected internal failure (e.g., corrupted registry state or an internal invariant violation)
- **When:** The framework catches the unexpected condition
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::InternalError`; `error_data.message` is non-empty and does not expose internal implementation details

### AP-13: TypeMismatch is returned for internal type conversion error

- **Given:** A scenario triggering a `TypeMismatch` condition — e.g., attempting to extract a `Value::String` as an integer via typed extraction methods
- **When:** The type conversion fails
- **Then:** Returns `Err(error_data)` where `error_data.code == ErrorCode::TypeMismatch`; `error_data.message` is non-empty

### AP-14: ErrorCode string representations match the documented catalog

- **Given:** Each `ErrorCode` variant (`CommandNotFound`, `ArgumentMissing`, `TooManyArguments`, etc.)
- **When:** `format!("{}", error_code)` or `.to_string()` is called on each variant
- **Then:** `CommandNotFound` produces `"UNILANG_COMMAND_NOT_FOUND"`, `ArgumentMissing` produces `"UNILANG_ARGUMENT_MISSING"`, `HelpRequested` produces `"HELP_REQUESTED"`, and all others match their documented `UNILANG_*` string representation

### AP-15: ErrorCode enum derives Debug for diagnostic formatting

- **Given:** An instance of `ErrorCode::ValidationRuleFailed`
- **When:** `format!("{:?}", error_code)` is called
- **Then:** The output contains the variant name `"ValidationRuleFailed"`; the code compiles without a manually written `Debug` implementation, confirming the `Debug` derive is present

### AP-16: Non-exhaustive matching on ErrorCode remains forward-compatible with a wildcard arm

- **Given:** Integrator code that matches on `ErrorCode` using explicit arms for all 12 currently-documented variants plus a trailing `_ => ErrorCode::InternalError` wildcard arm
- **When:** The match expression is compiled and evaluated against each of the 12 variants
- **Then:** Compilation succeeds (the wildcard arm satisfies exhaustiveness checking); each of the 12 variants is matched by its own explicit arm and the wildcard is never reached — demonstrating the pattern integrators should use per the documented forward-compatibility guarantee for future minor-release variants
