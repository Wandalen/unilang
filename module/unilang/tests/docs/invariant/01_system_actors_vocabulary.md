# Invariant Spec: System Actors Vocabulary

### Scope

- **Purpose:** Verify that canonical actor taxonomy and ubiquitous language terms are used consistently and exclusively throughout the codebase and documentation
- **Responsibility:** Test cases confirming that canonical terms appear in source code and no synonym or paraphrase variant exists
- **In Scope:** All 12 canonical vocabulary terms from `docs/invariant/001_system_actors_vocabulary.md`; canonical actor names (Integrator, End User, OS, External Service, Build Script, Command Registry, Parser, Semantic Analyzer, Interpreter); absence of deprecated synonyms
- **Out of Scope:** Behavioral features (FR-REG, FR-ARG); NFR thresholds (invariant 002)

### IN-1: Canonical internal actor names appear in source documentation without synonyms

- **Given:** The source code and doc comments of the `unilang` crate
- **When:** The codebase is searched for the deprecated synonym `"executor"` (deprecated synonym for `"Interpreter"`)
- **Then:** Zero occurrences of `"executor"` as an actor name are found; `"Interpreter"` is used exclusively for the execution actor

### IN-2: Actor taxonomy is complete — all three categories represented in documentation

- **Given:** The `docs/invariant/001_system_actors_vocabulary.md` document
- **When:** The document is read and its actor sections are enumerated
- **Then:** At least one entry exists in each of the three categories: Human actors (Integrator, End User), System actors (OS, External Service), and Internal actors (Build Script, Command Registry, Parser, Semantic Analyzer, Interpreter)

### IN-3: Canonical term "Semantic Analyzer" used in pipeline stage naming

- **Given:** The struct or type that performs semantic validation in the `unilang` or `unilang_parser` crate
- **When:** Its public name (struct name, module name, or doc comment) is inspected
- **Then:** The name contains `SemanticAnalyzer` or `semantic_analyzer` (not `validator`, `checker`, or `verifier`)

### IN-4: Canonical term "Command Registry" used without synonym variants in source

- **Given:** The source code of the `unilang` crate, where `CommandRegistry` is the dual-defined term (both an Internal System Actor and a Ubiquitous Language term)
- **When:** The codebase is searched for the deprecated synonyms `"CommandStore"`, `"CommandCache"`, and `"CommandDatabase"` as type names
- **Then:** Zero occurrences of these synonym type names are found; `CommandRegistry` (and `StaticCommandMap` for the static variant) are used exclusively for the runtime command database

### IN-5: Canonical term "Kind" used for argument data type without synonym variants

- **Given:** The source code of the `unilang` crate, where `Kind` is the canonical Ubiquitous Language term for an argument's data type
- **When:** The codebase is searched for the deprecated synonym type names `"ArgType"`, `"DataType"`, and `"ValueType"` used to represent an argument's data type
- **Then:** Zero occurrences of these synonym type names are found; `Kind` is used exclusively as the enum name for argument data types
