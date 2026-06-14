# Type Spec: Namespace Type

### Scope

- **Purpose:** Verify the NamespaceType validated newtype defined in `docs/type/002_namespace_type.md` enforces empty-or-dot-prefixed construction rules
- **Responsibility:** Test cases confirming valid construction for both empty and dot-prefixed namespaces, non-dot rejection
- **In Scope:** `NamespaceType::new()` validation, empty namespace special case (root-level commands), serde deserialization
- **Out of Scope:** Namespace isolation in aggregation (see `feature/001_command_registry.md`)

### TC-1: Empty namespace is accepted (root-level commands)

- **Given:** Input string `""`
- **When:** `NamespaceType::new("")` is called
- **Then:** Returns `Ok(ns)` where `ns.as_str() == ""`

### TC-2: Dot-prefixed namespace is accepted

- **Given:** Input string `".video"`
- **When:** `NamespaceType::new(".video")` is called
- **Then:** Returns `Ok(ns)` where `ns.as_str() == ".video"`

### TC-3: Non-empty non-dot-prefixed namespace is rejected

- **Given:** Input string `"video"`
- **When:** `NamespaceType::new("video")` is called
- **Then:** Returns `Err(Error::InvalidNamespace("video"))`

### TC-4: Nested dot-prefixed namespace is accepted

- **Given:** Input string `".tools.media"`
- **When:** `NamespaceType::new(".tools.media")` is called
- **Then:** Returns `Ok(ns)` where `ns.as_str() == ".tools.media"`

### TC-5: Serde deserialization rejects non-dot-prefixed namespace

- **Given:** JSON string `"\"video\""` (non-empty, no dot prefix)
- **When:** `serde_json::from_str::<NamespaceType>(json)` is called
- **Then:** Deserialization fails with a validation error; the error indicates an invalid namespace
