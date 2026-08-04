# Type Spec: Command Status

### Scope

- **Purpose:** Verify the CommandStatus enum defined in `docs/type/004_command_status.md` provides correct lifecycle stage classification and serde behavior
- **Responsibility:** Test cases confirming variant query methods, deprecation metadata, serde roundtrip for all forms, Default, from_str_lossy, Display
- **In Scope:** Four variants (Active, Deprecated, Experimental, Internal), query methods, deprecation_info(), `Default` derive, `from_str_lossy()`, `Display` formatting, serde serialization/deserialization for simple string, map, alias ("stable"), and unrecognized-value forms
- **Out of Scope:** How status affects registry behavior (see `feature/001_command_registry.md`); help output formatting

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

### TC-8: Default trait produces Active

- **Given:** No input
- **When:** `CommandStatus::default()` is called
- **Then:** Returns `CommandStatus::Active`

### TC-9: from_str_lossy maps recognized and unrecognized strings

- **Given:** Input strings `"experimental"`, `"internal"`, `"stable"`, `"unknown"`
- **When:** `CommandStatus::from_str_lossy(s)` is called for each
- **Then:** Returns `Experimental`, `Internal`, `Active`, `Active` respectively (unrecognized values default to `Active`)

### TC-10: Display formats simple variants as lowercase words

- **Given:** `CommandStatus::Active`, `CommandStatus::Experimental`, `CommandStatus::Internal` values
- **When:** Each is formatted with `format!("{}", status)`
- **Then:** Produces `"active"`, `"experimental"`, `"internal"` respectively

### TC-11: Display formats Deprecated variant with all metadata segments

- **Given:** A `CommandStatus::Deprecated` with `reason: "use .new"`, `since: Some("2.0")`, `replacement: Some(".new")`
- **When:** The value is formatted with `format!("{}", status)`
- **Then:** Produces `"deprecated (since 2.0): use .new → .new"`

### TC-12: Deserialization accepts "stable" as an alias for Active

- **Given:** JSON string `"\"stable\""`
- **When:** `serde_json::from_str::<CommandStatus>(json)` is called
- **Then:** Produces `CommandStatus::Active`

### TC-13: Deserialization of unrecognized string defaults to Active

- **Given:** JSON string `"\"bogus\""`
- **When:** `serde_json::from_str::<CommandStatus>(json)` is called
- **Then:** Produces `CommandStatus::Active` (unknown values do not error; they default)

### TC-14: Deserialization of simple "deprecated" string yields empty metadata

- **Given:** JSON string `"\"deprecated\""` (plain string form, no map)
- **When:** `serde_json::from_str::<CommandStatus>(json)` is called
- **Then:** Produces `CommandStatus::Deprecated` with `reason == ""`, `since == None`, `replacement == None`

### TC-15: Display formats Deprecated variant with reason only

- **Given:** A `CommandStatus::Deprecated` with `reason: "use .new"`, `since: None`, `replacement: None`
- **When:** The value is formatted with `format!("{}", status)`
- **Then:** Produces `"deprecated: use .new"` (no `(since ...)` segment, no `→` segment)

### TC-16: Display formats Deprecated variant with since only

- **Given:** A `CommandStatus::Deprecated` with `reason: ""`, `since: Some("2.0")`, `replacement: None`
- **When:** The value is formatted with `format!("{}", status)`
- **Then:** Produces `"deprecated (since 2.0)"` (empty reason segment is omitted entirely, no `→` segment)

### TC-17: Map-form "status" field is case-sensitive, unlike string form

- **Given:** JSON `{"status": "DEPRECATED", "reason": "obsolete"}` (uppercase status in map form)
- **When:** `serde_json::from_str::<CommandStatus>(json)` is called
- **Then:** Produces `CommandStatus::Active` — the map-form `visit_map` path matches `status` verbatim with no `.to_lowercase()` normalization, unlike the string-form `visit_str` path (TC-5); an uppercase map `status` value silently falls through to the default rather than matching `"deprecated"`

### TC-18: Map form with missing "status" key defaults to Active

- **Given:** JSON `{"reason": "obsolete"}` (map form, no `"status"` key present at all)
- **When:** `serde_json::from_str::<CommandStatus>(json)` is called
- **Then:** Produces `CommandStatus::Active` (absent `status` key does not error; it defaults, same as an unrecognized value)

### TC-19: Map form accepts explicit null since/replacement

- **Given:** JSON `{"status": "deprecated", "reason": "obsolete", "since": null, "replacement": null}`
- **When:** `serde_json::from_str::<CommandStatus>(json)` is called
- **Then:** Produces `CommandStatus::Deprecated` with `since == None`, `replacement == None` — explicit JSON `null` deserializes identically to the field being omitted (`Option<Option<String>>` flattened via `.flatten()`)
