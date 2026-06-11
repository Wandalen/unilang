# Valid Build-Time Fixtures

Well-formed YAML fixture files used to verify that the build-time code generator produces correct static registry output.

## Files

| File | Responsibility |
|------|----------------|
| `complete_command.yaml` | Command with all fields populated; validates full-field generation |
| `with_aliases.yaml` | Command with alias definitions; validates alias array generation |
