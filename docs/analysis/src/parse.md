# Parse Module Analysis (`src/parse/`)

## `/Users/axient/repository/episodic/src/parse/tokens.rs`
- Role: XML-like tag tokenizer with offsets and anchoring metadata.
- Data contracts: `TagToken`, `TagSectionRange`, `TagKind`.
- Control flow: byte scan with quote-aware tag-end detection and lowercase tag normalization.
- Purity: Pure transform.
- Performance: Single pass scan over input bytes.
- Risks: Not a full XML parser by design; recovers only expected tag subset.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/parse/tokens.rs:66`.

## `/Users/axient/repository/episodic/src/parse/sections.rs`
- Role: Tag section extraction and overlap recovery in strict/lenient modes.
- Data contracts: section range list and extracted content list/last-content.
- Control flow: mode-specific overlap handling, line-anchor checks, same-line close exception.
- Purity: Pure transform.
- Performance: Linear over token list.
- Risks: Lenient mode can accept malformed structures for salvage, which can preserve noisy content.
- Verdict: `WARN` (recovery tradeoff intentional).
- Evidence: `/Users/axient/repository/episodic/src/parse/sections.rs:17`, `/Users/axient/repository/episodic/src/parse/sections.rs:112`.

## `/Users/axient/repository/episodic/src/parse/thread.rs`
- Role: Multi-thread block extraction and thread-local metadata stripping.
- Data contracts: `(thread_id, body)` blocks and `OmMultiThreadObserverSection` conversion.
- Control flow:
  - parses `<thread id="...">`,
  - strips all metadata tag blocks,
  - retains latest non-empty task/response,
  - strict/lenient overlap behavior for malformed nested thread tags.
- Purity: Pure transform.
- Performance: Token scan + content slicing.
- Risks: Attribute parsing is ad-hoc and intentionally narrow.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/parse/thread.rs:86`, `/Users/axient/repository/episodic/src/parse/thread.rs:125`.

## `/Users/axient/repository/episodic/src/parse/mod.rs`
- Role: Public parse API, accuracy-first mode selection, and multi-thread aggregation.
- Data contracts: `OmParseMode`, `OmMemorySection`, `OmMultiThreadObserverSection`, `OmMultiThreadObserverAggregate`.
- Control flow:
  - memory parse (`<observations>` or list-item fallback),
  - multi-thread parse by observation sections,
  - strict-first accuracy fallback by explicit quality metrics,
  - aggregate to escaped thread XML blocks.
- Purity: Pure transform.
- Performance: Mostly linear scans; some additional passes for strict/lenient dual parse.
- Risks:
  - accuracy-first duplicates parse work by design (strict then optional lenient) for correctness.
  - Primary metadata selection bug was fixed: now newest-first reverse scan.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/parse/mod.rs:199`, `/Users/axient/repository/episodic/src/parse/mod.rs:279`.

## `/Users/axient/repository/episodic/src/parse/tests.rs`
- Role: Comprehensive parser behavior spec.
- Data contracts validated:
  - anchored-tag behavior,
  - fallback list extraction,
  - strict-vs-lenient overlap handling,
  - multi-thread metadata strip/recovery,
  - accuracy-first preference rules.
- Control flow: scenario-driven deterministic assertions.
- Purity: Pure tests.
- Performance: Trivial unit test cost.
- Risks: Coverage is broad, but malformed XML space is unbounded.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/parse/tests.rs:4`, `/Users/axient/repository/episodic/src/parse/tests.rs:457`.
