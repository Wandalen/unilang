# Feature Doc Entity

Behavioral requirements defining what commands `cargo_unilang` must provide.

### Scope

- **Purpose:** Document functional requirements for each cargo_unilang command
- **Responsibility:** Answers: what commands must the tool expose, what behavior must each command exhibit
- **In Scope:** Command behavior, parameter specifications, file generation contracts, detection logic
- **Out of Scope:** CLI API signatures, governing scope constraints, exit code definitions

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [`.new` Project Scaffolding](001_new_command.md) | Project creation command with template, author, license parameters | ✅ |
| 002 | [`.check` Health Check](002_check_command.md) | Anti-pattern detection command for existing unilang projects | ✅ |
