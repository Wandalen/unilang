# Feature Doc Entity

Behavioral requirements defining what the `unilang_meta` crate must provide.

### Scope

- **Purpose:** Document functional requirements for procedural macro behavior
- **Responsibility:** Answers: what macros must the crate provide, what code must they generate
- **In Scope:** Macro behavioral contracts, generated code specifications, inference rules
- **Out of Scope:** Macro implementation details, tooling strategy, compiler internals

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [`#[unilang::command]` Attribute Macro](001_command_macro.md) | Command registration boilerplate elimination via attribute macro | ✅ |
