# Parse Module Review

Reviewed files:
- `src/parse/mod.rs`
- `src/parse/tokens.rs`
- `src/parse/sections.rs`
- `src/parse/thread.rs`

## `src/parse/tokens.rs`

Role:
- Low-level tokenizer for XML-like tags with positional metadata.

What is implemented:
- Tag token model with kind/name/range/line anchoring.
- ASCII tag and attribute name char predicates.
- Quote-aware `find_tag_end` to avoid early `>` termination inside quoted attribute values.
- `parse_tag_tokens` emits normalized lowercase tag names.

Data-first notes:
- Token carries exact offsets and anchoring, enabling deterministic higher-level parsing.

Performance notes:
- Single forward scan over bytes with simple branching.

## `src/parse/sections.rs`

Role:
- Extracts tag content ranges and supports strict/lenient recovery logic.

What is implemented:
- `section_ranges_for_tag`:
  - strict: rejects ambiguous overlapping opens,
  - lenient: allows latest anchored open recovery.
- Anchoring rules reject inline tag literals unless same-line close is detected.
- Helpers to extract one-or-many sections and remove sections while returning last content.

Simplicity notes:
- Explicit mode branching; no parser generator complexity.

Risk:
- Lenient recovery intentionally favors salvage, which can accept malformed structures.

## `src/parse/thread.rs`

Role:
- Thread-specific extraction from multi-thread observer XML payloads.

What is implemented:
- Attribute parser for thread id (`id` only, case-insensitive key).
- Thread section parser that strips metadata blocks:
  - removes all `<current-task>` and `<suggested-response>`,
  - keeps last non-empty value for each.
- Thread block extraction with strict/lenient overlap behavior.

Performance:
- Linear scans over token stream and content slices.

Risk:
- Nested malformed thread blocks rely on heuristics; output quality depends on mode.

## `src/parse/mod.rs`

Role:
- Public parser API and aggregation functions.

What is implemented:
- Data contracts:
  - `OmParseMode`
  - `OmMemorySection`
  - `OmMultiThreadObserverSection`
  - `OmMultiThreadObserverAggregate`
- Memory parse:
  - parse `<observations>` blocks or fallback to list-item extraction.
  - parse optional task/response tags.
- Multi-thread parse:
  - parse thread blocks inside `<observations>`,
  - fallback to direct thread parse when no observation wrapper exists.
- Accuracy-first entrypoints:
  - strict-first, lenient fallback by quality heuristics.
- Aggregation:
  - re-encodes thread sections into escaped XML blocks,
  - selects metadata preferring primary thread then first available.

Data-first notes:
- Parse quality is represented as explicit structs and deterministic selector logic.

Performance notes:
- Reuses token scans and range slicing; no recursive descent overhead.

Confirmed risks:
- Metadata selection chooses first matching primary section, not newest matching section. If the same thread appears multiple times, earlier metadata can shadow later metadata.

