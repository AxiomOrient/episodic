# Context Utilities Review

Reviewed file:
- `src/context.rs`

## `src/context.rs`

Role:
- Builds bounded hint text from active observations.

What is implemented:
- `normalize_whitespace_line`: compacts whitespace to single spaces.
- `build_bounded_observation_hint`:
  - keeps only recent non-empty lines (`VecDeque` tail window),
  - enforces max line count and max char count,
  - prefixes output with `om:`,
  - returns `None` for empty/zero-budget cases.

Data-first notes:
- Input and output contract is explicit: raw observations in, optional bounded hint out.

Simplicity notes:
- Deterministic single-path algorithm.
- No hidden side effects.

Performance notes:
- Tail-window approach avoids storing all lines for large inputs.
- Char-budget loop is explicit and allocation-aware.

Risks:
- Char counting is Unicode scalar based; token semantics may diverge from model tokenization.

