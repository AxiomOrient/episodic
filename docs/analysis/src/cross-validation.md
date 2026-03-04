# Cross Validation (`docs/analysis` vs `docs/analysis/src`)

## Baseline Compared
- Previous module docs (non-test focused):
  - `/Users/axient/repository/episodic/docs/analysis/core-like docs` (`root.md`, `lib.md`, `model.md`, `inference.md`, `context.md`, `xml.md`, `addon.md`, `config.md`, `parse.md`, `prompt.md`, `transform.md`, `pipeline.md`, `cross-validation.md`)
- Current pass:
  - Full `src` scope including tests, file-by-file.

## Agreement Items
1. Project architecture remains data-first and transform-centric.
2. Config/parse/transform logic is explicit and mostly pure.
3. Parser and observer/reflection state logic are the highest complexity zones.
4. Prompt behavior is contract-heavy and correctness depends on model adherence.

## Previously Reported Issues Re-check
1. Parser primary metadata older-value retention risk.
- Status: fixed in source by reverse scan.
- Evidence: `/Users/axient/repository/episodic/src/parse/mod.rs:279`.

2. Reflection slice trailing newline sensitivity (`split('\n')`).
- Status: fixed in source by `lines()`.
- Evidence: `/Users/axient/repository/episodic/src/transform/reflection/slice.rs:9`.

3. `observed_message_ids` instruction mismatch with formatter input visibility.
- Status: fixed in source by explicit `[id:...]` formatter suffix.
- Evidence: `/Users/axient/repository/episodic/src/prompt/formatter.rs:37`.

4. Parity fixture `max_tokens_per_batch` assertion gap (test quality issue).
- Status: fixed.
- Evidence:
  - Assertion path now checks expected or input override in `/Users/axient/repository/episodic/tests/parity_fixtures.rs`.
  - Added parity fixture case `max_tokens_per_batch_override_is_applied` in `/Users/axient/repository/episodic/tests/fixtures/parity_cases.json`.

## Latest Re-review (2026-02-17, round 2)
1. `build_reflection_draft` ignored explicit zero budget by forcing `max_chars >= 1`.
- Status: fixed.
- Change:
  - `max_chars == 0` now returns `None` directly.
  - Evidence: `/Users/axient/repository/episodic/src/transform/reflection/draft.rs:29`.
  - Regression test: `/Users/axient/repository/episodic/src/transform/tests/reflection.rs:30`.

2. Observer session-id comparison/grouping did not normalize whitespace consistently.
- Status: fixed.
- Change:
  - `split_pending_and_other_conversation_candidates` now normalizes `current_session_id` and `source_session_id` with `trim`.
  - `build_other_conversation_blocks` now normalizes the same ids before local filtering and group keys.
  - Evidence:
    - `/Users/axient/repository/episodic/src/transform/observer/candidates.rs:60`
    - `/Users/axient/repository/episodic/src/transform/observer/context.rs:17`
  - Regression tests:
    - `/Users/axient/repository/episodic/src/transform/tests/observer.rs:192`
    - `/Users/axient/repository/episodic/src/transform/tests/observer.rs:279`

## Latest Re-review (2026-02-17, round 3)
1. `OmRecord` lacked an explicit, reusable invariant validator.
- Status: fixed.
- Change:
  - Added typed violation model and pure validator:
    - `OmRecordInvariantViolation`
    - `validate_om_record_invariants`
  - Evidence:
    - `/Users/axient/repository/episodic/src/model.rs`
    - `/Users/axient/repository/episodic/src/lib.rs`
  - Regression tests:
    - `/Users/axient/repository/episodic/src/model/tests.rs`

## New Findings in Full-`src` Pass (Current State)
1. No open high-severity functional defect found in source flow.
2. Complexity hotspots remain:
- `config/resolve.rs` policy interaction,
- `parse` lenient recovery heuristics,
- `observer/reflection` decision state combinations.

## Confidence
- High confidence for covered behavior due to complete file-level source review and existing broad test suite.
- Residual risk remains in unenumerated malformed-input and state-combination edge cases.
