# Invalid Build-Time Fixtures

YAML test fixture files with intentionally invalid command definitions, used to verify that the build-time validator correctly rejects malformed input.

## Files

| File | Responsibility |
|------|----------------|
| `empty_name.yaml` | Command with empty name; must fail validation |
| `empty_version.yaml` | Command with empty version; must fail validation |
| `missing_dot_prefix.yaml` | Command name missing leading dot; must fail validation |
