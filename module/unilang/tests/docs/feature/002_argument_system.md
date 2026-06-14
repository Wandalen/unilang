# Feature Spec: Argument System

### Scope

- **Purpose:** Verify all FR-ARG behavioral requirements for argument parsing and binding
- **Responsibility:** Test cases covering type support, positional binding, named binding, alias binding, default values, validation, multiple-parameter collection, and unknown-parameter detection
- **In Scope:** FR-ARG-1 (15 Kind variants), FR-ARG-2 (positional binding), FR-ARG-3 (named binding), FR-ARG-4 (alias binding), FR-ARG-5 (default values), FR-ARG-6 (validation rules), FR-ARG-7 (multiple collection), FR-ARG-8 (unknown parameter detection with Levenshtein suggestions)
- **Out of Scope:** Registry lookup (FR-REG); pipeline orchestration (FR-PIPE); help output (FR-HELP)

### FT-1: Named binding with param::value syntax extracts correct value

- **Given:** A command `.cmd` with one defined `String` argument named `"url"` and input tokens `[".cmd", "url::https://example.com"]`
- **When:** The semantic analyzer processes the parsed tokens
- **Then:** `VerifiedCommand.arguments["url"]` equals `Value::String("https://example.com")`; no error is returned

### FT-2: Unknown parameter produces error with Levenshtein suggestion

- **Given:** A command `.cmd` with argument `"output"` and input tokens `[".cmd", "ouput::foo"]` (one character transposition)
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error of kind `UnknownArgument` with a suggestions list that includes `"output"`

### FT-3: Default value is used when argument is absent

- **Given:** A command `.cmd` with argument `"verbose"` of type `Bool` with default `false`, and input tokens `[".cmd"]`
- **When:** The semantic analyzer processes the input
- **Then:** `VerifiedCommand.arguments["verbose"]` equals `Value::Bool(false)`; no missing-argument error

### FT-4: Multiple-parameter collection accumulates repeated values into Vec

- **Given:** A command `.cmd` with argument `"tag"` marked as multiple, and input `[".cmd", "tag::alpha", "tag::beta", "tag::gamma"]`
- **When:** The semantic analyzer processes the input
- **Then:** `VerifiedCommand.arguments["tag"]` equals `Value::Array([String("alpha"), String("beta"), String("gamma")])`

### FT-5: Positional binding assigns value by position when no name given

- **Given:** A command `.cmd` with one positional `String` argument defined at position 0, and input `[".cmd", "hello"]`
- **When:** The semantic analyzer processes the input
- **Then:** The positional argument receives `Value::String("hello")` without requiring `name::value` syntax

### FT-6: Type coercion — integer token parsed into Kind::I64 value

- **Given:** A command `.cmd` with argument `"count"` of type `Kind::I64` and input `[".cmd", "count::42"]`
- **When:** The semantic analyzer processes the input
- **Then:** `VerifiedCommand.arguments["count"]` equals `Value::I64(42)` without error

### FT-7: Missing required argument produces structured error

- **Given:** A command `.cmd` with one required `String` argument `"name"` (no default) and input `[".cmd"]`
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error indicating `"name"` is required and missing; no panic occurs

### FT-8: Alias-based named binding resolves to canonical argument

- **Given:** A command `.cmd` with argument `"output"` that has alias `"o"`, and input `[".cmd", "o::result.txt"]`
- **When:** The semantic analyzer processes the input
- **Then:** `VerifiedCommand.arguments["output"]` equals `Value::String("result.txt")`; the alias `"o"` is resolved to canonical name `"output"`

### FT-9: ValidationRule MinLength rejects too-short value

- **Given:** A command `.cmd` with argument `"name"` of type `String` with `ValidationRule::MinLength(3)`, and input `[".cmd", "name::ab"]`
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error with code `UNILANG_VALIDATION_RULE_FAILED` indicating the value is shorter than the minimum length of 3

### FT-10: ValidationRule Pattern rejects non-matching value

- **Given:** A command `.cmd` with argument `"email"` of type `String` with `ValidationRule::Pattern("^[a-z]+@[a-z]+\\.[a-z]+$")`, and input `[".cmd", "email::INVALID"]`
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error with code `UNILANG_VALIDATION_RULE_FAILED` indicating the value does not match the required pattern

### FT-11: Type coercion — float token parsed into Kind::F32 value

- **Given:** A command `.cmd` with argument `"ratio"` of type `Kind::F32` and input `[".cmd", "ratio::3.14"]`
- **When:** The semantic analyzer processes the input
- **Then:** `VerifiedCommand.arguments["ratio"]` equals `Value::F32(3.14)` without error

### FT-12: Type coercion — path token parsed into Kind::Path value

- **Given:** A command `.cmd` with argument `"file"` of type `Kind::Path` and input `[".cmd", "file::/tmp/data.csv"]`
- **When:** The semantic analyzer processes the input
- **Then:** `VerifiedCommand.arguments["file"]` equals `Value::Path("/tmp/data.csv")` without error

### FT-13: ValidationRule Max rejects over-limit integer value

- **Given:** A command `.cmd` with argument `"count"` of type `Kind::I64` with `ValidationRule::Max(100)`, and input `[".cmd", "count::101"]`
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error with code `UNILANG_VALIDATION_RULE_FAILED` indicating the value exceeds the maximum of 100
