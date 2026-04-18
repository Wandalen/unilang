# API: CLI Commands

`cargo_unilang` exposes three commands via the `unilang` CLI framework: `.new` (scaffold a new project), `.check` (validate an existing project for anti-patterns), and `.help` (display usage information); all commands accept a `verbosity::<0-5>` parameter and return documented exit codes (0 = success, 1 = issues found, 2 = invalid parameters, 3 = path/creation error).
