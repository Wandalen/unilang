# Analysis Doc Entity

Codebase analysis reports: API usability studies, pattern identification, and improvement opportunities.

### Scope

- **Purpose:** Document findings from systematic analysis of the unilang codebase
- **Responsibility:** Answers: what patterns exist, what usability issues were found, what improvements are recommended
- **In Scope:** API surface analysis, usability findings, boilerplate pattern identification, improvement opportunities
- **Out of Scope:** Design decisions, implementation guides, and architectural mandates (see architecture/), feature requirements (see feature/)

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [API Analysis](001_api_analysis.md) | API surface analysis with boilerplate identification | ✅ |
| 002 | [Usability Improvements](002_usability_improvements.md) | User experience enhancement opportunities | ✅ |

### Type Declaration

- **Decision Criteria**: Use when documenting empirical findings from systematically examining the existing codebase (patterns found, issues identified, improvement opportunities ranked by severity); no standard type or recognized extension covers empirical analysis reports — this is retrospective findings documentation, not a forward-looking contract.
- **Contrast with feature/**: `feature/` documents formal behavioral requirements the system must satisfy (forward-looking, prescriptive contracts); `analysis/` documents what was empirically observed by examining the codebase (backward-looking, descriptive findings) — an analysis report may recommend future features, but the recommendation itself is not a requirement until promoted into `feature/`.
- **Required Sections**: Scope, Executive Summary, at least one findings section (e.g., Critical Issues, Root Cause Analysis — content varies per report)
- **Optional Sections**: Success Metrics
- **Overview Table Columns**: `ID`, `Name`, `Purpose`, `Status`
- **Quality Checklist**:
  - [ ] Are findings backed by concrete evidence (code references, examples) rather than unsupported opinion?
  - [ ] Are issues/opportunities ranked or prioritized (e.g., by severity or impact)?
  - [ ] Is this genuinely an empirical finding from analysis, not a design decision (belongs in `architecture/`) or a formal requirement (belongs in `feature/`)?

