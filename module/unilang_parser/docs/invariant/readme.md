# Invariant Doc Entity

Non-negotiable constraints that must hold for `unilang_parser` correctness and architecture.

### Scope

- **Purpose:** Document constraints that are never relaxed regardless of implementation approach
- **Responsibility:** Answers: what must always be true, what external dependencies are mandated
- **In Scope:** Tokenization strategy constraints, dependency mandates
- **Out of Scope:** API surface definitions, feature requirements, performance targets

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Parser Tokenization Mandate](001_parser_mandate.md) | strs_tools-only tokenization — no from-scratch splitting | ✅ |
