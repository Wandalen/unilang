# unilang_meta — Tests

## Directory Structure

```
tests/
  smoke_test.rs     — Basic crate compilation smoke test
  trybuild.rs       — Trybuild UI test harness (runs all tests/ui/*.rs)
  ui/               — Trybuild UI test fixture files
```

## Responsibility Table

| File / Dir | Responsibility |
|------------|----------------|
| smoke_test.rs | Basic compilation and crate import smoke test |
| trybuild.rs | Trybuild harness; orchestrates all UI test cases |
| ui/ | Trybuild UI test fixture .rs files and .stderr snapshots |
