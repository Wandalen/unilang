# Type Doc Entity

Domain types with construction-time validation guarantees following the "parse don't validate" pattern.

### Scope

- **Purpose:** Document validated newtypes and enums that enforce correctness at construction time
- **Responsibility:** Answers: what validation contract does each type enforce, what errors can construction produce
- **In Scope:** Validated newtypes (CommandName, NamespaceType, VersionType), lifecycle enums (CommandStatus), construction APIs, error conditions
- **Out of Scope:** Public API surface listing (see api/), behavioral requirements (see feature/), performance characteristics (see invariant/)

### Overview Table

| ID | Name | domain | ddd | Status |
|----|------|--------|-----|--------|
| 001 | [Command Name](001_command_name.md) | command identity | value object | ✅ |
| 002 | [Namespace Type](002_namespace_type.md) | command identity | value object | ✅ |
| 003 | [Version Type](003_version_type.md) | command metadata | value object | ✅ |
| 004 | [Command Status](004_command_status.md) | command lifecycle | enum | ✅ |
