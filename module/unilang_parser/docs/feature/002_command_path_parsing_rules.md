# Feature: Command Path and Argument Parsing Rules

### Scope

- **Purpose:** Define the behavioral contract for command path and argument tokenization
- **Responsibility:** Rules governing path segment delimitation, argument transition, dot handling, help token, and argument kinds
- **In Scope:** Space handling, path segment splitting, path-to-argument transition, dot edge cases, `??` help token, `was_quoted` recording, positional and named argument recognition
- **Out of Scope:** Semantic validation, command definition lookup, error recovery strategy

The parser must implement the following rules when transforming raw instruction input into a `GenericInstruction`:

**Rule 0 — Spaces are ignored:** Any number of spaces at any position in the instruction string are discarded and have no effect on the parsed result.

**Rule 1 — Command Path Delimitation:** The command path consists of one or more segments separated by `.`. Spaces before or after `.` are ignored per Rule 0.
- `.cmd.subcmd` → `["cmd", "subcmd"]`
- `.cmd. subcmd` → `["cmd", "subcmd"]`
- `.cmd   .  subcmd` → `["cmd", "subcmd"]`

**Rule 2 — Transition to Arguments:** The command path ends and argument parsing begins when:
- A token is encountered that is not an identifier, a space, or a dot (e.g., `::` or a quoted string); or
- An identifier is followed by a token that is not a dot and not `::` — the identifier is the last path segment and the next token is the first argument; or
- The end of input is reached after an identifier (input ending with a dot is a syntax error per Rule 3).

**Rule 3 — Leading and Trailing Dots:** Leading dots (`.cmd`) are ignored. Trailing dots (`cmd.`) are a syntax error in all cases.

**Rule 4 — Help Token (`??`):** `?` and `??` are ordinary value-capable tokens, not operators. An exact unquoted `?` or `??` token parses as a normal argument — positional at any position, or a named value after `::` — and when it is the very first token it lands in the command path slot (`["??"]`). Every parsed `Argument` records `was_quoted`; for a value merged from several tokens, quoting anywhere in the merge sets it. The parser attaches no help semantics: a semantic layer (e.g. `unilang`) maps an unquoted `??` to help output and treats a quoted `"??"` as the literal two-character string.

**Rule 5 — Positional Arguments:** Any non-named token following the command path is a positional argument.

**Rule 6 — Named Arguments:** Named arguments use `name::value` syntax. The `::` separator distinguishes them from positional arguments and from the command path.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | [feature/001_parsing_api.md](001_parsing_api.md) | Public method signatures that produce the GenericInstruction output |
| doc  | [invariant/001_parser_mandate.md](../invariant/001_parser_mandate.md) | Tokenization engine constraint governing Rule 0–6 implementation |
