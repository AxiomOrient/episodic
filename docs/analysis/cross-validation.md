# Cross Validation Against Previous Analysis

This document compares the previous analysis pass and the latest full re-review pass.

Reference baseline:
- Previous pass included both source and test review and reported four main findings.

Current scope:
- Full `src` file-by-file re-review with targeted test revalidation.

## Comparison Table

1) Previous finding:
- Duplicate-thread metadata in parser aggregation can keep older metadata.

Current result:
- Confirmed.
- Historical evidence (before fix): `aggregate_multi_thread_observer_sections` selected first matching primary section (`iter().find(...)`).

Status:
- Resolved in code.
- Change: primary/fallback metadata selection now scans newest-first (`iter().rev()`) in `src/parse/mod.rs`.

2) Previous finding:
- Reflection slice line math is sensitive to trailing empty lines due to `split('\n')`.

Current result:
- Confirmed.
- Historical evidence (before fix): line split was `full_observations.split('\n')`.

Status:
- Resolved in code.
- Change: line split now uses `lines()` in `src/transform/reflection/slice.rs`.

3) Previous finding:
- Parity test assertion did not validate `max_tokens_per_batch` fixture input.

Current result:
- Resolved in follow-up full-`src` pass.
- Evidence:
  - assertion now validates expected/input override in `tests/parity_fixtures.rs`,
  - fixture case `max_tokens_per_batch_override_is_applied` added to `tests/fixtures/parity_cases.json`.

Status:
- Resolved in code/tests.

4) Previous finding:
- Prompt instruction mentions constrained `observed_message_ids`, but default message formatter does not show ids.

Current result:
- Confirmed.
- Evidence:
  - Instruction text in `src/prompt/user.rs` references provided ids.
  - Historical behavior (before fix): rendered message format in `src/prompt/formatter.rs` included role/text/timestamp, not id.

Status:
- Resolved in code.
- Change: observer message formatter now includes `[id:...]` in `src/prompt/formatter.rs`.

## New Findings Introduced in This Pass

1) New finding:
- `build_reflection_draft` ignored explicit zero char budget (`max_chars=0`) by forcing at least one char.

Current result:
- Confirmed and fixed.
- Evidence:
  - Guard now returns `None` when `max_chars == 0` in `src/transform/reflection/draft.rs`.
  - Regression test in `src/transform/tests/reflection.rs`.

Status:
- Resolved in code/tests.

2) New finding:
- Observer `source_session_id` handling could diverge on whitespace (`" s-local "` vs `"s-local"`).

Current result:
- Confirmed and fixed.
- Evidence:
  - Session id normalization (`trim`) applied in:
    - `src/transform/observer/candidates.rs`
    - `src/transform/observer/context.rs`
  - Regression tests in:
    - `src/transform/tests/observer.rs`

Status:
- Resolved in code/tests.

3) New finding:
- `OmRecord` did not provide a reusable explicit invariant validator for scope/scope-key and buffered-reflection metadata.

Current result:
- Confirmed and fixed.
- Evidence:
  - Added typed violation model and validator in `src/model.rs`.
  - Exported from crate surface in `src/lib.rs`.
  - Added invariant regression tests in `src/model/tests.rs`.

Status:
- Resolved in code/tests.

## Confidence Statement

- High confidence on parser, prompt, transform, config, and pipeline conclusions because this pass re-read source files, added targeted regressions, and passed full test and clippy gates.
