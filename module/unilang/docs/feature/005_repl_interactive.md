# Feature: REPL Interactive

### Scope

- **Purpose:** Define behavioral requirements for REPL and interactive CLI modalities
- **Responsibility:** FR-REPL-1, FR-INTERACTIVE-1, FR-MOD-WASM-REPL: read-eval-print loop, interactive mode, WASM
- **In Scope:** REPL behavior requirements, interactive mode requirements, WASM modality
- **Out of Scope:** REPL implementation details, feature flag configuration (see architecture/006)

Functional requirements for REPL-style execution, interactive argument prompting, and WebAssembly modality.

### FR-REPL-1 (REPL Support)

The framework's core components (`Pipeline`, `Parser`, `SemanticAnalyzer`, `Interpreter`) **must** be structured to support a REPL-style execution loop. They **must** be reusable for multiple, sequential command executions within a single process lifetime.

**Implementation Notes:**
- Pipeline components are fully stateless and reusable
- Each command execution is independent with no state accumulation
- Memory efficient operation verified through performance benchmarks
- Reference implementations available in `examples/12_repl_loop.rs`, `examples/15_interactive_repl_mode.rs`, `examples/17_advanced_repl_features.rs`

**Implementation status:** ✅ Implemented with comprehensive examples and verified stateless operation.

### REPL Technical Requirements

**Stateless Operation Requirements:**
- Each command execution cycle must be completely independent
- No state accumulation between command executions to prevent memory leaks
- Components (`Parser`, `SemanticAnalyzer`, `Interpreter`) must be reusable without internal state corruption
- Performance requirement: Command execution overhead must remain constant regardless of session length

**Interactive Argument Handling:**
- The error code `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` must be catchable at the REPL level
- REPL implementations must handle secure input (passwords, API keys) without logging or state persistence
- Optional interactive arguments with defaults must not trigger interactive prompts
- Interactive argument validation must occur during semantic analysis, not execution

**Memory Management Insights:**
- Pipeline component reuse provides 20-50% performance improvement over creating new instances
- Command history storage should be bounded to prevent unbounded memory growth
- Large command outputs should be handled with streaming or pagination for long-running REPL sessions

**Error Recovery Patterns:**
- Parse errors should provide contextual suggestions for command correction
- Semantic analysis errors should indicate available commands and proper syntax
- Execution errors should not terminate the REPL session
- Error history tracking enables improved user experience with "last-error" functionality

**User Experience Requirements:**
- Auto-completion suggestions require command registry introspection capabilities
- Command history must support search and replay functionality
- Session statistics provide valuable debugging information
- Clear screen and session reset capabilities are essential for productive use

**Performance Considerations:**
- Optimized static command registry provides zero-cost lookups even in REPL context
- Dynamic command registration during REPL sessions should be supported for development workflows
- Batch command processing capabilities enable script-like functionality within REPL
- Command validation without execution supports syntax checking workflows

### FR-INTERACTIVE-1 (Interactive Argument Prompting)

When a mandatory argument with the `interactive: true` attribute is not provided, the `Semantic Analyzer` **must** return a distinct, catchable error (`UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`). This allows the calling modality to intercept the error and prompt the user for input.

**Implementation Notes:**
- Error code `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` is returned as specified
- Implemented in `src/semantic.rs`
- Comprehensive test coverage in `tests/inc/phase5/interactive_args_test.rs`
- REPL examples demonstrate proper error handling and secure input simulation

**Implementation status:** ✅ Implemented in semantic analyzer with comprehensive test coverage and REPL integration.

### FR-MOD-WASM-REPL (WebAssembly REPL Modality)

The framework **must** support a web-based REPL modality that can operate entirely on the client-side without a backend server. This requires the core `unilang` library to be fully compilable to the `wasm32-unknown-unknown` target.

**Implementation status:** ❌ Not yet implemented.

This requirement is also connected to NFR-PLATFORM-1 (WASM Compatibility), which requires the core logic of the `unilang` and `unilang_parser` crates to be platform-agnostic and fully compatible with the `wasm32-unknown-unknown` target. See `docs/invariant/002_non_functional_requirements.md` for the full NFR.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/003_pipeline.md](003_pipeline.md) | Pipeline used by REPL for command processing |
| doc | [invariant/002_non_functional_requirements.md](../invariant/002_non_functional_requirements.md) | NFR-PLATFORM-1 WASM compatibility requirement |
| doc | [architecture/006_repl_implementation.md](../architecture/006_repl_implementation.md) | REPL implementation guide and feature flags |
