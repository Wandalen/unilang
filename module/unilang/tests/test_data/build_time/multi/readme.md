# Multi-File Build-Time Fixtures

YAML/JSON fixture files used to test multi-file command aggregation, conflict detection, and namespace merging during build-time static registry generation.

## Files

| File | Responsibility |
|------|----------------|
| `commands_a.yaml` | First YAML source; commands in namespace A |
| `commands_b.yml` | Second YAML source (.yml extension); namespace B commands |
| `commands_c.json` | First JSON source; commands in namespace C |
| `commands_d.json` | Second JSON source; tests JSON + YAML mixed aggregation |
