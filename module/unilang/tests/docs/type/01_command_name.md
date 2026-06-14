# Type Spec: Command Name

### Scope

- **Purpose:** Verify the CommandName validated newtype defined in `docs/type/001_command_name.md` enforces non-empty dot-prefix construction rules
- **Responsibility:** Test cases confirming valid construction, empty rejection, missing dot rejection, serde roundtrip
- **In Scope:** `CommandName::new()` validation, serde deserialization rejection, accessor correctness
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
