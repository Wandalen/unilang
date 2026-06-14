# Feature: Command Path and Argument Parsing Rules

### Scope

- **Purpose:** Define the behavioral contract for command path and argument tokenization
- **Responsibility:** Rules governing path segment delimitation, argument transition, dot handling, help operator, and argument kinds
- **In Scope:** Space handling, path segment splitting, path-to-argument transition, dot edge cases, `?` operator, positional and named argument recognition
- **Out of Scope:** Semantic validation, command definition lookup, error recovery strategy

The parser must implement the following rules when transforming raw instruction input into a `GenericInstruction`:

**Rule 0 — Spaces are ignored:** Any number of spaces at any position in the instruction string are discarded and have no effect on the parsed result.

**Rule 1 — Command Path Delimitation:** The command path consists of one or more segments separated by `.`. Spaces before or after `.` are ignored per Rule 0.
- `.cmd.subcmd` → `["cmd", "subcmd"]`
- `.cmd. subcmd` → `["cmd", "subcmd"]`
- `.cmd   .  subcmd` → `["cmd", "subcmd"]`

**Rule 2 — Transition to Arguments:** The command path ends and argument parsing begins when:
- A token is encountered that is not an identifier, a space, or a dot (e.g., `::`, `?`, or a quoted string); or
- An identifier is followed by a token that is not a dot and not `::` — the identifier is the last path segment and the next token is the first argument; or
- The end of input is reached after an identifier (input ending with a dot is a syntax error per Rule 3).

**Rule 3 — Leading and Trailing Dots:** Leading dots (`.cmd`) are ignored. Trailing dots (`cmd.`) are a syntax error in all cases.

**Rule 4 — Help Operator (`?`):** `?` is valid anywhere after the command path — not only as the first post-path token but also after other arguments. When arguments precede `?`, the semantic meaning covers both the command and those specific arguments. `?` must always be the last token.

**Rule 5 — Positional Arguments:** Any non-named token following the command path is a positional argument.

**Rule 6 — Named Arguments:** Named arguments use `name::value` syntax. The `::` separator distinguishes them from positional arguments and from the command path.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | [feature/001_parsing_api.md](001_parsing_api.md) | Public method signatures that produce the GenericInstruction output |
| doc  | [invariant/001_parser_mandate.md](../invariant/001_parser_mandate.md) | Tokenization engine constraint governing Rule 0–6 implementation |
