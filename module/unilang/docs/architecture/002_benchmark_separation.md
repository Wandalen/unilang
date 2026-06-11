# Architecture: Benchmark Separation

### Scope

- **Purpose:** Document the rationale for keeping benchmarks in a separate crate
- **Responsibility:** Explains why benchmark code is isolated from the main crate
- **In Scope:** Benchmark separation rationale, dependency isolation, crate structure
- **Out of Scope:** Benchmark implementation details, performance results

### Overview

Performance benchmarks for unilang are maintained in a separate workspace crate (`unilang_benchmarks`) rather than the main `unilang` crate. This architectural decision improves dependency isolation and production build performance.

### Rationale

#### Dependency Isolation

Benchmark tooling requires specialized dependencies that production users never need — benchmark frameworks with statistical analysis, CLI argument parsing for benchmark runners, CPU detection for parallel benchmarking, random data generation for realistic test scenarios, and system information gathering. By separating benchmarks into their own crate, these dependencies never pollute production dependency trees, do not increase compilation time for end users, do not bloat production binaries, and can be versioned independently.

#### Build Performance

Production builds of `unilang` are faster because they have fewer optional features to check, a smaller dependency graph, no benchmark code in compilation units, and a cleaner feature flag matrix.

#### Maintenance Benefits

Separation provides a clear boundary between production and performance testing code, enables independent benchmark versioning and releases, simplifies benchmark infrastructure evolution, and presents a cleaner API surface for production users.

### Structure

The main `unilang` crate lives at `module/unilang/` with source, functional tests, and no benchmark dependencies in its manifest. The `unilang_benchmarks` crate at `module/unilang_benchmarks/` contains benchmark configuration and data modules (e.g., `benchmark_config.rs`, `benchmark_data_sizes.rs`, `realistic_test_data.rs`), all performance benchmark suites (throughput, SIMD JSON, etc.) in the `benches/` directory, benchmark validation tests, and all benchmark-specific dependencies.

### Usage

#### Running Benchmarks

Run all benchmarks with `cargo bench -p unilang_benchmarks` from the workspace root. Target a specific benchmark suite by name. Configure the benchmark environment via the `BENCHMARK_ENV` variable.

#### Adding New Benchmarks

Add benchmark code to `unilang_benchmarks/benches/`, import common utilities from `unilang_benchmarks::prelude::*`, follow benchkit conventions for measurement, and document the benchmark's purpose and methodology.

#### Benchmark Development

The `unilang_benchmarks` crate imports `unilang` with `features = ["full"]` to access all functionality for comprehensive performance testing.

### Migration Notes

All benchmark code moved from `unilang/benches/` to `unilang_benchmarks/benches/`. Benchmark modules moved from `unilang/src/` to `unilang_benchmarks/src/`. Feature flags `benchmarks` and `advanced_benchmarks` were removed from the main crate. Documentation updated to reference the separate benchmark crate.

### Architectures

| File | Relationship |
|------|--------------|
| [001_mandates.md](001_mandates.md) | Broader architectural mandates |

### Invariants

| File | Relationship |
|------|--------------|
| [002_non_functional_requirements.md](../invariant/002_non_functional_requirements.md) | NFRs validated by benchmarks |

### Sources

| File | Relationship |
|------|--------------|
| `build/codegen.rs` | PHF codegen whose performance benchmarks motivate separation |
