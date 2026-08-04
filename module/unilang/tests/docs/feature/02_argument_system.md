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

### FT-14: Kind::Enum accepts only predefined choices

- **Given:** A command `.cmd` with argument `"level"` of type `Kind::Enum(["low", "medium", "high"])` and input `[".cmd", "level::extreme"]`
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error with code `UNILANG_ARGUMENT_TYPE_MISMATCH` indicating `"extreme"` is not one of the allowed choices; when input is instead `"level::medium"`, `VerifiedCommand.arguments["level"]` equals `Value::Enum("medium".to_string())` with no error

### FT-15: Kind::File and Kind::Directory validate filesystem existence and category

- **Given:** A command `.cmd` with argument `"target"` of type `Kind::File`, and input tokens naming (a) an existing regular file, (b) an existing directory, and (c) a nonexistent path
- **When:** The semantic analyzer processes each input in turn
- **Then:** Case (a) binds `Value::File(path)` with no error; case (b) returns a type-mismatch error stating a file was expected but a directory was found; case (c) returns a type-mismatch error stating no file was found at the path — and the symmetric behavior holds for `Kind::Directory` with the file/directory cases reversed

### FT-16: Kind::Url and Kind::DateTime parse into their typed values

- **Given:** A command `.cmd` with argument `"endpoint"` of type `Kind::Url` and input `[".cmd", "endpoint::https://api.example.com/v1"]`, and separately an argument `"when"` of type `Kind::DateTime` with input `[".cmd", "when::2024-01-15T10:30:00+00:00"]`
- **When:** The semantic analyzer processes each input
- **Then:** `VerifiedCommand.arguments["endpoint"]` equals `Value::Url(...)` parsed from the given URL with no error; `VerifiedCommand.arguments["when"]` equals `Value::DateTime(...)` parsed as RFC 3339 with no error; malformed input for either (e.g. `"not a url"` or `"not-a-date"`) produces a type-mismatch error instead of a panic

### FT-17: Kind::Pattern compiles input into a regular expression value

- **Given:** A command `.cmd` with argument `"regex"` of type `Kind::Pattern` and input `[".cmd", "regex::^[a-z]+$"]`
- **When:** The semantic analyzer processes the input
- **Then:** `VerifiedCommand.arguments["regex"]` holds a `Value::Pattern` whose compiled regex matches the source string `"^[a-z]+$"`; an invalid regex source (e.g. `"regex::[unclosed"`) produces a type-mismatch error instead of a panic

### FT-18: Kind::List and Kind::Map parse with default and custom delimiters

- **Given:** A command `.cmd` with argument `"tags"` of type `Kind::List(Kind::String, None)` and input `[".cmd", "tags::a,b,c"]`, and separately argument `"tags2"` of type `Kind::List(Kind::String, Some(';'))` with input `[".cmd", "tags2::a;b;c"]`; and argument `"opts"` of type `Kind::Map(Kind::String, Kind::String, None, None)` with input `[".cmd", "opts::k1=v1,k2=v2"]`
- **When:** The semantic analyzer processes each input
- **Then:** `tags` binds `Value::List([String("a"), String("b"), String("c")])` using the default `,` delimiter; `tags2` binds the same three-element list using the custom `;` delimiter instead; `opts` binds `Value::Map({"k1": String("v1"), "k2": String("v2")})` using the default `,` entry delimiter and `=` key-value delimiter

### FT-19: Kind::JsonString and Kind::Object parse and validate JSON payloads (requires `json_parser` feature)

- **Given:** A command `.cmd` with argument `"payload"` of type `Kind::JsonString` and input `[".cmd", "payload::{\"a\":1}"]`, and separately argument `"data"` of type `Kind::Object` with the same input value
- **When:** The semantic analyzer processes each input under the `json_parser` feature
- **Then:** `payload` binds `Value::JsonString("{\"a\":1}".to_string())` after successful JSON validation; `data` binds `Value::Object(...)` as a parsed `serde_json::Value`; malformed JSON input (e.g. `"payload::{not json}"`) produces a type-mismatch error for both kinds instead of a panic

### FT-20: ValidationRule Min rejects under-limit numeric value

- **Given:** A command `.cmd` with argument `"age"` of type `Kind::Integer` with `ValidationRule::Min(0.0)`, and input `[".cmd", "age::-1"]`
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error with code `UNILANG_VALIDATION_RULE_FAILED` indicating the value is less than the minimum allowed value of 0

### FT-21: ValidationRule MaxLength rejects too-long string value

- **Given:** A command `.cmd` with argument `"code"` of type `String` with `ValidationRule::MaxLength(4)`, and input `[".cmd", "code::abcdef"]`
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error with code `UNILANG_VALIDATION_RULE_FAILED` indicating the value's length exceeds the maximum allowed length of 4

### FT-22: ValidationRule MinItems rejects a list with too few elements

- **Given:** A command `.cmd` with argument `"tags"` of type `Kind::List(Kind::String, None)` with `ValidationRule::MinItems(2)`, and input `[".cmd", "tags::solo"]`
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error with code `UNILANG_VALIDATION_RULE_FAILED` indicating the list has fewer than the minimum required 2 items

### FT-23: Sensitive argument attribute redacts the value in validation error messages

- **Given:** A command `.cmd` with argument `"password"` of type `String` marked `attributes.sensitive = true` with `ValidationRule::MinLength(8)`, and input `[".cmd", "password::abc"]`
- **When:** The semantic analyzer processes the input and validation fails
- **Then:** Returns a `UNILANG_VALIDATION_RULE_FAILED` error whose message contains `"[REDACTED]"` (or an equivalent redaction marker) and does **not** contain the literal raw value `"abc"`

### FT-24: Interactive argument attribute signals a distinct error instead of a missing-argument failure

- **Given:** A command `.cmd` with a required argument `"token"` marked `attributes.interactive = true` and no value provided, with input `[".cmd"]`
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error with code `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` (distinct from the generic `UNILANG_ARGUMENT_MISSING` error), signaling the calling modality to prompt the user rather than treating the absence as a validation failure

### FT-25: VerifiedCommand typed extraction methods retrieve, coerce-check, and report missing arguments

- **Given:** A `VerifiedCommand` whose `arguments` map contains a `String` value bound to `"name"`, an `Integer` value bound to `"count"`, and no value bound to `"missing"`
- **When:** Callers invoke the typed accessor pairs — `get_string`/`require_string`, `get_integer`/`require_integer`, `get_float`/`require_float`, `get_boolean`/`require_boolean`, `get_path`/`require_path`, `get_list`/`require_list` — alongside `has_argument()` and `get_value()`
- **Then:** For `"name"`, `get_string("name")` returns `Some("...")` and `require_string("name")` returns `Ok(...)`; for `"count"`, `get_string("count")` returns `None` (wrong type) and `require_string("count")` returns an `Err` with `ArgumentTypeMismatch`; for `"missing"`, every `get_*` returns `None`, every `require_*` returns an `Err` with `ArgumentTypeMismatch`, `has_argument("missing")` returns `false`, and `get_value("missing")` returns `None`; `has_argument("name")` returns `true` and `get_value("name")` returns `Some(&Value::String(...))`

### FT-26: Normalized string extraction trims surrounding whitespace

- **Given:** A `VerifiedCommand` whose `arguments` map contains `Value::String("  Alice  ".to_string())` bound to `"name"`
- **When:** Callers invoke `get_string_normalized("name")` and `require_string_normalized("name")`
- **Then:** Both return the trimmed value `"Alice"` (no surrounding whitespace); a whitespace-only value normalizes to `Some("")` / `Ok("")` rather than `None`/error

### FT-27: ValidationRule::Pattern with a syntactically-invalid regex fails closed, not open

- **Given:** A command `.cmd` with argument `"code"` of type `String` with `ValidationRule::Pattern("[unclosed")` (a malformed regex — the pattern string is never eagerly compile-checked when the rule is attached to the argument definition), and input `[".cmd", "code::anything"]`
- **When:** The semantic analyzer processes the input
- **Then:** Returns an error with code `UNILANG_VALIDATION_RULE_FAILED` — the value is REJECTED (fail-closed), not silently accepted; no panic occurs. Distinct from FT-10 (well-formed pattern, non-matching value) and FT-17 (`Kind::Pattern` argument-value parse-time regex compilation) — this covers `ValidationRule::Pattern`'s own rule-string malformation, which reaches the same generic "does not match the required pattern" message as a true non-match, since the compile failure and match failure are not distinguished in the error text
