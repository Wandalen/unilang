# Development Rules for Unilang

**CRITICAL: Read before making ANY changes to this codebase**

This project strictly follows the design rules documented in `docs/invariant/003_governing_principles.md` and the architecture mandates in `docs/architecture/001_mandates.md`.

## Quick Reference Card

### ✅ ALLOWED
| What | Where | Example |
|------|-------|---------|
| Unit tests | `tests/` | `#[test] fn test_correctness() { assert_eq!(result, expected); }` |
| Integration tests | `tests/` | Testing public APIs and workflows |
| Performance optimizations | `src/` | LRU cache, static registries, SIMD in production code |
| Production monitoring | `src/` | `metrics.cache_hit_rate()` for logging |

### ❌ PROHIBITED
| What | Where | Why | Use Instead |
|------|-------|-----|-------------|
| Custom timing | `tests/` | `std::time::Instant` in tests | `benchkit` framework |
| Performance assertions | `tests/` | `assert!(ops_per_sec > 1000)` | Functional assertions only |
| Benchmarks as tests | `tests/` | Speed comparisons | Separate `benchkit` infrastructure |
| Missing Test Matrix | `tests/` | No `//! Test Matrix` comment | Add mandatory documentation |

## References

- **Primary Rules:** `docs/invariant/003_governing_principles.md`
- **Architecture Mandates:** `docs/architecture/001_mandates.md`
- **Benchmarking:** Use `benchkit` framework only
- **Test Organization:** `tests/` for correctness, `benchkit` for performance

---

**Remember: Separation of concerns is not optional. Performance belongs in production code and benchkit. Tests belong in tests/ for correctness only.**