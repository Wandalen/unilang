# Invariant Doc Entity

Non-negotiable constraints that must hold for `unilang_meta` correctness and architecture.

### Scope

- **Purpose:** Document constraints that are never relaxed regardless of macro implementation approach
- **Responsibility:** Answers: what dependencies are mandated, what direct dependencies are forbidden
- **In Scope:** Macro tooling dependency constraints
- **Out of Scope:** Macro behavior specifications, generated code contracts, feature requirements

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Macro Tooling Mandate](001_macro_mandate.md) | macro_tools-only dependency — no direct syn/quote/proc-macro2 | ✅ |
