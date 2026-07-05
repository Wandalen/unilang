# Type Spec: Command Name

### Scope

- **Purpose:** Verify the CommandName validated newtype defined in `docs/type/001_command_name.md` enforces non-empty dot-prefix construction rules
- **Responsibility:** Test cases confirming valid construction, empty rejection, missing dot rejection, serde roundtrip, Display, accessors, equality
- **In Scope:** `CommandName::new()` validation, serde serialization/deserialization roundtrip, accessor correctness (`as_str()`, `into_inner()`), `Display` formatting, `PartialEq` equality
- **Out of Scope:** Registry behavior on valid names (see `feature/001_command_registry.md`); naming policy (see `invariant/005_command_naming.md`)

### TC-1: Valid dot-prefixed name is accepted

- **Given:** Input string `".hello"`
- **When:** `CommandName::new(".hello")` is called
- **Then:** Returns `Ok(name)` where `name.as_str() == ".hello"`

### TC-2: Empty string is rejected

- **Given:** Input string `""`
- **When:** `CommandName::new("")` is called
- **Then:** Returns `Err(Error::EmptyCommandName)`

### TC-3: Name without dot prefix is rejected

- **Given:** Input string `"nodot"`
- **When:** `CommandName::new("nodot")` is called
- **Then:** Returns `Err(Error::MissingDotPrefix("nodot"))`

### TC-4: Single dot is accepted as valid

- **Given:** Input string `"."`
- **When:** `CommandName::new(".")` is called
- **Then:** Returns `Ok(name)` where `name.as_str() == "."`

### TC-5: Nested dot-prefixed name is accepted

- **Given:** Input string `".video.convert"`
- **When:** `CommandName::new(".video.convert")` is called
- **Then:** Returns `Ok(name)` where `name.as_str() == ".video.convert"`

### TC-6: Serde deserialization rejects invalid name

- **Given:** JSON string `"\"nodot\""`
- **When:** `serde_json::from_str::<CommandName>(json)` is called
- **Then:** Returns `Err` with a message referencing the missing dot prefix

### TC-7: Display trait formats as the inner string

- **Given:** A `CommandName` constructed from `".build"`
- **When:** The value is formatted with `format!("{}", name)`
- **Then:** Produces the string `".build"` (identical to `as_str()`)

### TC-8: Serialize produces a plain JSON string

- **Given:** A `CommandName` constructed from `".video.convert"`
- **When:** `serde_json::to_string(&name)` is called
- **Then:** Returns `Ok("\".video.convert\"")` — a plain string, not a map

### TC-9: Serde deserialization accepts a valid name

- **Given:** JSON string `"\".hello\""`
- **When:** `serde_json::from_str::<CommandName>(json)` is called
- **Then:** Returns `Ok(name)` where `name.as_str() == ".hello"`

### TC-10: into_inner consumes and returns the owned String

- **Given:** A `CommandName` constructed from `".build"`
- **When:** `into_inner()` is called
- **Then:** Returns owned `String` equal to `".build"`, and the `CommandName` is consumed

### TC-11: Equal names compare as equal

- **Given:** Two `CommandName` values both constructed from `".build"`
- **When:** They are compared with `==`
- **Then:** The comparison returns `true` (derived `PartialEq`/`Eq`)
