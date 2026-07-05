# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|-----------|
| `analysis/` | Answers: what patterns exist, what usability issues were found, what improvements are recommended | [analysis](../analysis/readme.md) | 2 |
| `api/` | Answers: what types does the public API expose, what are the stability guarantees | [api](../api/readme.md) | 2 |
| `architecture/` | Answers: why was it designed this way, what are the architectural constraints | [architecture](../architecture/readme.md) | 7 |
| `feature/` | Answers: what behaviors must the system exhibit, what are the acceptance criteria | [feature](../feature/readme.md) | 5 |
| `invariant/` | Answers: what constraints must always be true, what are the measurable thresholds | [invariant](../invariant/readme.md) | 6 |
| `tests/docs/api/` | One spec file per API doc instance; each case targets a named public type or compatibility guarantee | [tests/docs/api](../../tests/docs/api/readme.md) | 2 |
| `tests/docs/feature/` | One spec file per feature doc instance; each case maps to at least one FR | [tests/docs/feature](../../tests/docs/feature/readme.md) | 5 |
| `tests/docs/invariant/` | One spec file per invariant doc instance; each case exercises an invariant boundary | [tests/docs/invariant](../../tests/docs/invariant/readme.md) | 6 |
| `tests/docs/type/` | One spec file per type doc instance; each case exercises a validation rule boundary | [tests/docs/type](../../tests/docs/type/readme.md) | 4 |
| `type/` | Answers: what validation contract does each type enforce, what errors can construction produce | [type](../type/readme.md) | 4 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|-----|------|------|
| analysis | 001 | API Analysis | [001_api_analysis.md](../analysis/001_api_analysis.md) |
| analysis | 002 | Usability Improvements | [002_usability_improvements.md](../analysis/002_usability_improvements.md) |
| api | 001 | Public Types | [001_public_types.md](../api/001_public_types.md) |
| api | 002 | Error Codes | [002_error_codes.md](../api/002_error_codes.md) |
| architecture | 001 | Mandates | [001_mandates.md](../architecture/001_mandates.md) |
| architecture | 002 | Benchmark Separation | [002_benchmark_separation.md](../architecture/002_benchmark_separation.md) |
| architecture | 003 | Vision & Scope | [003_vision_scope.md](../architecture/003_vision_scope.md) |
| architecture | 004 | Implementation Details | [004_implementation_details.md](../architecture/004_implementation_details.md) |
| architecture | 005 | Help Decoupling | [005_help_decoupling.md](../architecture/005_help_decoupling.md) |
| architecture | 006 | REPL Implementation | [006_repl_implementation.md](../architecture/006_repl_implementation.md) |
| architecture | 007 | Migration Guide | [007_migration_guide.md](../architecture/007_migration_guide.md) |
| feature | 001 | Command Registry | [001_command_registry.md](../feature/001_command_registry.md) |
| feature | 002 | Argument System | [002_argument_system.md](../feature/002_argument_system.md) |
| feature | 003 | Pipeline | [003_pipeline.md](../feature/003_pipeline.md) |
| feature | 004 | Help System | [004_help_system.md](../feature/004_help_system.md) |
| feature | 005 | REPL Interactive | [005_repl_interactive.md](../feature/005_repl_interactive.md) |
| invariant | 001 | System Actors Vocabulary | [001_system_actors_vocabulary.md](../invariant/001_system_actors_vocabulary.md) |
| invariant | 002 | Non-Functional Requirements | [002_non_functional_requirements.md](../invariant/002_non_functional_requirements.md) |
| invariant | 003 | Governing Principles | [003_governing_principles.md](../invariant/003_governing_principles.md) |
| invariant | 004 | Workspace Dependency Standards | [004_workspace_dependency_standards.md](../invariant/004_workspace_dependency_standards.md) |
| invariant | 005 | Command Naming | [005_command_naming.md](../invariant/005_command_naming.md) |
| invariant | 006 | Build-Runtime Separation | [006_build_runtime_separation.md](../invariant/006_build_runtime_separation.md) |
| type | 001 | Command Name | [001_command_name.md](../type/001_command_name.md) |
| type | 002 | Namespace Type | [002_namespace_type.md](../type/002_namespace_type.md) |
| type | 003 | Version Type | [003_version_type.md](../type/003_version_type.md) |
| type | 004 | Command Status | [004_command_status.md](../type/004_command_status.md) |
