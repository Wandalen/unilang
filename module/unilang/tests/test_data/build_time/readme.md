# Build-Time Test Fixtures

YAML/JSON fixture files for build-time code generation and static registry tests.

## Files / Directories

| File / Directory | Responsibility |
|------------------|----------------|
| `invalid/` | Intentionally malformed fixtures for validator rejection tests |
| `multi/` | Multi-file aggregation fixtures for conflict and namespace tests |
| `valid/` | Well-formed fixtures for successful build-time generation tests |
| `test_commands.yaml` | Single-file YAML command definitions fixture |
| `test_commands.json` | Single-file JSON command definitions fixture |
