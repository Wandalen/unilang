# Compile-Fail Fixtures

Fixture files for trybuild compile-fail tests. Each file is a minimal Rust
program that must fail to compile. The trybuild test runner in
`../build/compile_fail_tests.rs` verifies each fixture fails with the expected
compiler error.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `t40_builder_missing_name.rs` | T40: builder without `.name()` call must not compile |
| `t50_private_field_name.rs` | T50: direct access to private `name` field must not compile |
| `t50b_private_field_description.rs` | T50b: direct access to private `description` field must not compile |
