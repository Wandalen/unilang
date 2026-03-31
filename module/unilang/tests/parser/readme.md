# Parser Tests

Tests for tokenization, argument parsing, SIMD paths, and string interning.

## Files

| File | Responsibility |
|------|----------------|
| `argument_parsing.rs` | Named and positional argument parsing |
| `quoted_values.rs` | Quoted string value handling |
| `file_path_parsing.rs` | File path argument edge cases |
| `edge_case_handling.rs` | Boundary conditions and unusual inputs |
| `unicode_safety.rs` | Unicode character handling in arguments |
| `extended_ascii_safety.rs` | Extended ASCII safety in input strings |
| `simd_tokenization.rs` | SIMD-accelerated tokenizer correctness |
| `simd_json.rs` | SIMD JSON parser correctness |
| `string_interning.rs` | String interner behavior and cache semantics |
| `static_data_structures.rs` | PHF and static lookup structure correctness |
| `command_sequence_scalability.rs` | Parser performance with many commands |
