# Unilang Crate/Framework Implementation Roadmap

### Current Status (as of 2025_11_24)
The project has successfully completed Phase 4 (Zero-Overhead Static Command Registry), including build-time validation to make illegal states unrepresentable. The framework now features compile-time prevention of duplicate commands and parameter storage mismatches (wplan bug pattern). Phase 5 work is in progress, with 3 of 4 milestones complete. The framework is production-ready for building high-performance, type-safe command-line utilities.

**Legend:**
*   ⚫ : Not Started
*   ⏳ : In Progress
*   ✅ : Done
*   🏁 : Phase Complete / Major Milestone

---

### Phase 1: Core `unilang` Language Engine & CLI Foundations 🏁
*   **Goal:** Establish the `unilang` parsing pipeline, core data structures, command registration, basic type handling, execution flow, initial help capabilities, and error reporting to enable a functional CLI.
*   **Outcome:** A working, foundational `unilang` crate capable of handling basic CLI commands from parsing to execution.
*   **Status:** All milestones are complete.

### Phase 2: Enhanced Type System, Runtime Commands & CLI Maturity 🏁
*   **Goal:** Expand the `unilang` crate's type system, provide APIs for runtime command management, and mature CLI support.
*   **Outcome:** A feature-rich framework capable of handling complex data types, dynamic command loading, and advanced CLI interactions.
*   **Status:** All milestones are complete.

### Phase 3: Architectural Unification & Enhancement 🏁
*   **Goal:** Correct the project's architecture by removing legacy components, integrating `unilang_parser` as the single source of truth, and fully aligning data models with the specification.
*   **Outcome:** A stable, maintainable codebase with a unified architecture, ready for the implementation of core functional requirements.
*   **Status:** All milestones are complete.

### Phase 4: Zero-Overhead Static Command Registry
*   **Goal:** To implement the mandatory performance NFR for a zero-overhead static command system, enabling utilities with thousands of commands to start instantly.
*   **Outcome:** A framework with a hybrid command registry where all compile-time commands are stored in an optimized static registry, eliminating runtime registration costs and ensuring sub-millisecond command resolution.

*   [✅] **M4.1: registry_design_hybrid_architecture:**
    *   **Spec Reference:** FR-PERF-1, NFR-Performance
    *   **Deliverable:** A detailed task plan for implementing a zero-overhead static command registry.
    *   **Purpose:** Design a build-time mechanism (using `build.rs`) to generate an optimized static registry from a command manifest. This plan will outline the steps to refactor the `CommandRegistry` into a hybrid model.
*   [✅] **M4.2: static_registry_implement_build_time_generation:**
    *   **Prerequisites:** M4.1
    *   **Deliverable:** A `build.rs` script that generates a `.rs` file containing the static optimized registry from `unilang.commands.yaml`.
    *   **Purpose:** Implement the build script that parses the YAML manifest and uses compile-time optimization to construct the static registry.
*   [✅] **M4.3: registry_refactor_to_hybrid_model:**
    *   **Prerequisites:** M4.2
    *   **Deliverable:** An updated `CommandRegistry` that uses the generated static registry for compile-time commands and a `HashMap` for dynamic commands.
    *   **Purpose:** Refactor all lookup methods to query the static registry first before falling back to the dynamic `HashMap`.
*   [✅] **M4.4: test_implement_performance_stress_harness:**
    *   **Prerequisites:** M4.3
    *   **Spec Reference:** FR-PERF-1
    *   **Deliverable:** A new integration test that generates a large YAML manifest (1000+ commands) and a test binary that proves the performance NFRs are met.
    *   **Purpose:** The test will generate the manifest, compile a test binary against it, and then execute the binary to measure and assert that startup time is negligible and p99 command resolution latency is under 1ms.
*   [✅] **M4.5: build_time_validation_illegal_states:**
    *   **Prerequisites:** M4.2
    *   **Spec Reference:** FR-REG-9, Task 085
    *   **Deliverable:** Build-time validation in `build.rs` preventing illegal command states (duplicate names, wplan bug pattern).
    *   **Purpose:** Implement compile-time validation to make illegal states unrepresentable: (1) Duplicate command name detection via HashMap tracking, (2) Parameter storage type validation (multiple:true requires List), (3) Actionable error messages with fix guidance. Resolves 8 of 10 items from Task 085. All 833 tests pass.
    *   **Completed:** 2025_11_24

### Phase 5: Core API Enhancements & Modality Support
*   **Goal:** To implement the remaining mandatory functional requirements from Spec v2.2.0, ensuring the framework fully supports REPL, interactive CLI, and WebAssembly (WASM) modalities.
*   **Outcome:** A functionally complete and validated API for building sophisticated, user-friendly command-line applications that can run in native and web environments.

*   [✅] **M5.1: pipeline_refactor_for_reusability:**
    *   **Spec Reference:** FR-REPL-1
    *   **Deliverable:** An audited and confirmed stateless core pipeline and a new example file (`repl_example.rs`).
    *   **Purpose:** Audit the core pipeline components (`Parser`, `SemanticAnalyzer`, `Interpreter`) to ensure they are stateless and can be reused in a REPL loop.
*   [✅] **M5.2: argument_implement_interactive_signaling:**
    *   **Spec Reference:** FR-INTERACTIVE-1
    *   **Deliverable:** The `SemanticAnalyzer` correctly returns the `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` error for missing interactive arguments.
    *   **Purpose:** Modify the `bind_arguments` logic to check for the `interactive: true` attribute on missing mandatory arguments and return the specific error code.
*   [✅] **M5.3: test_create_interactive_prompting_verification:**
    *   **Prerequisites:** M5.2
    *   **Deliverable:** A new unit test for the `SemanticAnalyzer` and an updated CLI binary demonstrating how to catch the interactive signal.
*   [⚫] **M5.4: example_create_wasm_repl:**
    *   **Prerequisites:** M5.1
    *   **Spec Reference:** NFR-PLATFORM-1
    *   **Deliverable:** A working, browser-based REPL example compiled to WebAssembly.
    *   **Purpose:** Create a minimal web application that uses the `unilang` WASM package to provide a fully client-side REPL, proving the WASM compatibility NFR.

### Phase 6: Performance Hardening & SIMD Optimization
*   **Goal:** To meet the stringent performance NFRs by systematically eliminating bottlenecks identified in the performance analysis, with a focus on reducing string allocations and leveraging SIMD instructions.
*   **Outcome:** A framework with throughput competitive with minimalist parsers like `pico-args`, achieved through zero-copy techniques, string interning, and SIMD-accelerated operations.

*   [✅] **M6.1: optimization_implement_string_interning:**
    *   **Spec Reference:** `performance.md` (Task 001)
    *   **Deliverable:** A string interning system integrated into the `SemanticAnalyzer` to cache command names and other common strings.
    *   **Completed:** Task 001. Implemented in `src/interner.rs` as `StringInterner` with LRU-bounded lookup cache (10,000 entries), thread-safe `RwLock`-protected storage, and `Box::leak`-based `&'static str` returns. Global singleton exposed via `global_interner()`. Integrated into semantic hot path.
*   [✅] **M6.2: token_refactor_to_zero_copy:**
    *   **Prerequisites:** M6.1
    *   **Spec Reference:** `performance.md` (Task 002)
    *   **Deliverable:** The `unilang_parser` crate updated to use `&str` tokens, and the `unilang` crate updated to consume them, eliminating major allocation overhead.
    *   **Completed:** Task 002. Zero-copy token references implemented in `unilang_parser`. Parser returns borrowed `&str` slices from the input rather than owned `String`s, eliminating allocation overhead in the parsing hot path.
*   [✅] **M6.3: parser_integrate_simd_json:**
    *   **Prerequisites:** M6.2
    *   **Spec Reference:** `performance.md` (Task 009)
    *   **Deliverable:** The type system's JSON parsing logic updated to use the `simd-json` crate for a 4-25x performance improvement on JSON-heavy workloads.
    *   **Completed:** 2025_10_19. SIMD JSON parsing implemented in `src/simd_json_parser.rs`. Provides 4-25x speedup. Feature-gated with `simd-json` + `json_parser`.
*   [⚫] **M6.4: benchmark_audit_performance_final:**
    *   **Prerequisites:** M6.3
    *   **Deliverable:** An updated `performance.md` with final benchmark results proving all performance NFRs are met.

### Phase 7: Modularity & Lightweight Core Refactoring 🏁
*   **Goal:** To fulfill the modularity NFRs by refactoring the crate to use granular feature flags for all non-essential functionality, creating a minimal core profile that is as lightweight as `pico-args`.
*   **Outcome:** A highly modular framework where users can opt-in to features, ensuring minimal binary size and dependency footprint for simple use cases.

*   [✅] **M7.1: dependency_audit_features:**
    *   **Spec Reference:** NFR-MODULARITY-1, NFR-MODULARITY-2
    *   **Deliverable:** A dependency graph mapping features to the libraries they introduce.
    *   **Purpose:** Analyze `Cargo.toml` and the codebase to identify all dependencies that can be made optional.
    *   **Completed:** 2025_10_19. Identified and made optional: serde_yaml_ng, serde_json, phf, walkdir (~330KB total savings).
*   [✅] **M7.2: feature_gate_implement_granular:**
    *   **Prerequisites:** M7.1
    *   **Deliverable:** An updated `Cargo.toml` and codebase where all non-essential functionality is gated by feature flags (e.g., `declarative_loading`, `chrono_types`).
    *   **Completed:** 2025_10_19. Implemented 2-tier architecture: initially 20 approach features + 12 infrastructure features. All format parsers optional. Updated 2026_06_14: reduced to 8 approach features + 5 infrastructure features after dep hygiene audit (YAGNI) removed 14 unimplemented approach feature declarations and 7 future dep declarations with zero implementation behind them.
*   [✅] **M7.3: profile_create_minimal_core:**
    *   **Prerequisites:** M7.2
    *   **Deliverable:** A working `unilang` crate when compiled with `--no-default-features`.
    *   **Completed:** 2025_10_19. Minimal build verified: `cargo check --no-default-features --features enabled` compiles successfully.
*   [✅] **M7.4: footprint_verify_lightweight:**
    *   **Prerequisites:** M7.3
    *   **Deliverable:** Benchmark results comparing the compile time and dependency count of the minimal `unilang` profile against `pico-args`.
    *   **Completed:** 2025_10_19. Verified: Minimal build has ~30% faster compilation. Default build saves ~200KB vs full build.
*   [✅] **M7.5: implement_opinionated_defaults:**
    *   **Prerequisites:** M7.2
    *   **Deliverable:** Opinionated defaults strategy where only Approach #2 (Multi-YAML Build-Time Static) is enabled by default.
    *   **Completed:** 2025_10_19. Reduced default from 21 approaches to 1. All tests pass (513 total with --features full).

### Phase 8: Advanced Features - Web Modality
*   **Goal:** To implement a full Web API modality, building on the now stable, performant, and modular architecture.
*   **Outcome:** A versatile, multi-modal framework that can serve its command registry as a RESTful API.

*   [⚫] **M8.1: modality_design_web_api:**
    *   **Deliverable:** A plan for mapping `unilang` commands to HTTP endpoints.
*   [⚫] **M8.2: generator_implement_openapi:**
    *   **Prerequisites:** M8.1
    *   **Deliverable:** A function that generates an OpenAPI v3+ specification from the `CommandRegistry`.
*   [⚫] **M8.3: mapper_implement_http_to_command:**
    *   **Prerequisites:** M8.1
    *   **Deliverable:** A utility/adapter that converts an incoming HTTP request into a `unilang` command invocation.
*   [⚫] **M8.4: example_create_web_api:**
    *   **Prerequisites:** M8.3
    *   **Deliverable:** An example application that serves a `unilang` registry as a REST API.

### Phase 9: Advanced Features - Developer Experience 🏁
*   **Goal:** To significantly improve the developer experience by providing procedural macros that reduce boilerplate code.
*   **Outcome:** A framework that is not only powerful but also ergonomic for developers to use.

*   [✅] **M9.1: macro_design_procedural:**
    *   **Deliverable:** An API design for the `#[command]` procedural macro in the `unilang_meta` crate.
    *   **Completed:** `unilang_meta` v0.3.0 — `#[command]` macro API designed and stabilised.
*   [✅] **M9.2: macro_implement_command:**
    *   **Prerequisites:** M9.1
    *   **Deliverable:** A working `#[command]` macro that generates `CommandDefinition` structs from Rust functions.
    *   **Completed:** `unilang_meta` v0.3.0 (commit `3dc2f3a`) — `#[command]` procedural macro implemented and integrated.

### Phase 10: Release Candidate Preparation
*   **Goal:** Focus on stability, developer experience, and documentation to prepare for a v1.0 release.
*   **Outcome:** A polished, production-ready v1.0.0-rc.1 release of the `unilang` framework.

*   [⚫] **M10.1: guide_write_core_concepts:**
    *   **Deliverable:** A comprehensive guide in the documentation explaining the core architecture and philosophy of `unilang`.
*   [⚫] **M10.2: tutorial_write_modality:**
    *   **Prerequisites:** M8.4
    *   **Deliverable:** Tutorials for building a CLI, REPL, and a Web API with `unilang`.
*   [⚫] **M10.3: api_conduct_final_review:**
    *   **Deliverable:** A final review of the public API, with any necessary breaking changes made before the 1.0 release.
*   [⚫] **M10.4: release_publish_v1_candidate:**
    *   **Prerequisites:** M10.3
    *   **Deliverable:** `unilang` v1.0.0-rc.1 published to crates.io.

### Phase 11: Post-v1.0 Ecosystem & Advanced Features
*   **Goal:** Expand the `unilang` ecosystem with new modalities, improved tooling, and advanced integration capabilities.
*   **Outcome:** A mature and extensible framework that solidifies its position as a universal command-line tool.

*   [⚫] **M11.1: modality_implement_tui_framework:**
    *   **Deliverable:** Utilities and an example for building interactive Textual User Interfaces.
*   [⚫] **M11.2: routine_implement_dynamic_loading:**
    *   **Deliverable:** A robust implementation for `routine_link` that can load routines from dynamic libraries.
*   [⚫] **M11.3: system_design_plugin:**
    *   **Deliverable:** A formal specification for a plugin system, allowing third-party crates to provide `unilang` commands to a host application.

---

### Project Goals & Success Metrics

- **Primary Goal:** To create a stable, performant, and ergonomic framework for building multi-modal command-line utilities in Rust that allows developers to define a command interface once and deploy it everywhere with zero-overhead for static commands.
- **Success Metric 1 (Performance):** The framework **must** meet all performance NFRs defined in `docs/invariant/002_non_functional_requirements.md`, verified by the project's benchmark suite.
- **Success Metric 2 (Adoption):** The framework is considered successful if it is used to build at least three distinct applications with different modalities within 12 months of the v1.0 release.

### Deliverables

1. Published `unilang` Rust crate on crates.io.
2. Published `unilang_parser` Rust crate on crates.io.
3. Published `unilang_meta` Rust crate on crates.io.
4. Compiled WebAssembly (`.wasm`) package and JavaScript bindings for the core framework.
5. Full source code repository access, including all examples and benchmarks.
6. Generated API documentation hosted on docs.rs for all public crates.

### Open Questions

1. **Custom Type Registration:** What is the API and process for an integrator to define a new custom `Kind` and register its associated parsing and validation logic with the framework?
2. **Plugin System:** What would a formal plugin system look like, allowing third-party crates to provide `unilang` commands to a host application?
