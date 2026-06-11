# Type Spec: Version Type

### Scope

- **Purpose:** Verify the VersionType validated newtype defined in `docs/type/003_version_type.md` enforces non-empty construction rule
- **Responsibility:** Test cases confirming valid construction, empty rejection, serde roundtrip
- **In Scope:** `VersionType::new()` validation, serde deserialization rejection for empty strings
- **Out of Scope:** Semantic versioning policy; version comparison

### TC-1: Non-empty version string is accepted

- **Given:** Input string `"1.0.0"`
- **When:** `VersionType::new("1.0.0")` is called
- **Then:** Returns `Ok(ver)` where `ver.as_str() == "1.0.0"`

### TC-2: Empty string is rejected

- **Given:** Input string `""`
- **When:** `VersionType::new("")` is called
- **Then:** Returns `Err(Error::EmptyVersion)`

### TC-3: Single character version is accepted

- **Given:** Input string `"1"`
- **When:** `VersionType::new("1")` is called
- **Then:** Returns `Ok(ver)` where `ver.as_str() == "1"`

### TC-4: Arbitrary non-empty string is accepted (no format constraint)

- **Given:** Input string `"beta-rc.1+build.42"`
- **When:** `VersionType::new("beta-rc.1+build.42")` is called
- **Then:** Returns `Ok(ver)` where `ver.as_str() == "beta-rc.1+build.42"`

### TC-5: Serde deserialization rejects empty version string

- **Given:** JSON string `"\"\""` (empty string)
- **When:** `serde_json::from_str::<VersionType>(json)` is called
- **Then:** Deserialization fails with a validation error indicating an empty version is not permitted
