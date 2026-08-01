# Invariant Test Surface

Test spec files for `docs/invariant/` doc instances.
Case prefix: `IN-`. Minimum 2 cases per spec.

### Scope

- **Purpose:** Enumerate test cases verifying system properties that must always hold
- **Responsibility:** One spec file per invariant doc instance; each case exercises an invariant boundary
- **In Scope:** System actors vocabulary, NFR thresholds, governing principles, workspace dependency standards, command naming conventions, build-runtime separation
- **Out of Scope:** Behavioral feature testing (covered in `feature/`); CLI-level invariants (no `docs/cli/`)

### Overview Table

| Name | Purpose | Status |
|------|---------|--------|
| `01_system_actors_vocabulary.md` | `invariant` spec for canonical actor taxonomy and vocabulary | ✅ |
| `02_non_functional_requirements.md` | `invariant` spec for NFR-PERF, NFR-SEC, NFR-ROBUST, NFR-PLATFORM, NFR-MODULARITY | ✅ |
| `03_governing_principles.md` | `invariant` spec for Minimum Implicit Magic, Fail-Fast, Illegal States | ✅ |
| `04_workspace_dependency_standards.md` | `invariant` spec for dependency format rules R1–R4 | ✅ |
| `05_command_naming.md` | `invariant` spec for dot-prefix naming contract at all registration boundaries | ✅ |
| `06_build_runtime_separation.md` | `invariant` spec for compile-time-only YAML/JSON processing guarantee | ✅ |
