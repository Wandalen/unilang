# Type Spec: Command Status

### Scope

- **Purpose:** Verify the CommandStatus enum defined in `docs/type/004_command_status.md` provides correct lifecycle stage classification and serde behavior
- **Responsibility:** Test cases confirming variant query methods, deprecation metadata, serde roundtrip for all forms
- **In Scope:** Four variants (Active, Deprecated, Experimental, Internal), query methods, deprecation_info(), serde serialization/deserialization for both simple and map forms
- **Out of Scope:** How status affects registry behavior (see `feature/01_command_registry.md`); help output formatting

### TC-1: Active variant is default and queryable

- **Given:** A `CommandStatus::Active` value
- **When:** Query methods are called
- **Then:** `is_active()` returns `true`; `is_deprecated()`, `is_experimental()`, `is_internal()` all return `false`; `deprecation_info()` returns `None`

### TC-2: Deprecated variant carries metadata

- **Given:** A `CommandStatus::Deprecated` with `reason: "use .new"`, `since: Some("2.0")`, `replacement: Some(".new")`
- **When:** `deprecation_info()` is called
- **Then:** Returns `Some(("use .new", &Some("2.0"), &Some(".new")))`

### TC-3: Simple variant serde roundtrip (lowercase string)

- **Given:** JSON string `"\"active\""` (or `"\"experimental\""` or `"\"internal\""`)
- **When:** `serde_json::from_str::<CommandStatus>(json)` is called and the result is serialized back
- **Then:** Deserialization produces the correct variant; re-serialization produces the original JSON string

### TC-4: Deprecated variant serde roundtrip (map form)

- **Given:** JSON `{"status": "deprecated", "reason": "obsolete", "since": "1.0", "replacement": ".v2"}`
- **When:** `serde_json::from_str::<CommandStatus>(json)` is called
- **Then:** Produces `CommandStatus::Deprecated` with all three metadata fields populated

### TC-5: Case-insensitive deserialization

- **Given:** JSON string `"\"ACTIVE\""` (uppercase)
- **When:** `serde_json::from_str::<CommandStatus>(json)` is called
- **Then:** Produces `CommandStatus::Active` (case-insensitive acceptance)

### TC-6: Experimental variant is queryable

- **Given:** A `CommandStatus::Experimental` value
- **When:** Query methods are called
- **Then:** `is_experimental()` returns `true`; `is_active()`, `is_deprecated()`, `is_internal()` all return `false`; `deprecation_info()` returns `None`

### TC-7: Internal variant is queryable

- **Given:** A `CommandStatus::Internal` value
- **When:** Query methods are called
- **Then:** `is_internal()` returns `true`; `is_active()`, `is_deprecated()`, `is_experimental()` all return `false`; `deprecation_info()` returns `None`
