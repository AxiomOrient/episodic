# Next-Step Validity Review (2026-02-17)

## Goal
- Re-check whether the next implementation step is justified under:
  - explicit data modeling,
  - minimal abstraction,
  - pure transformation preference,
  - removal of decorative behavior.

## File-by-File Coverage
- Core root: `/Users/axient/repository/episodic/docs/analysis/src/core.md`
- Config: `/Users/axient/repository/episodic/docs/analysis/src/config.md`
- Parse: `/Users/axient/repository/episodic/docs/analysis/src/parse.md`
- Prompt: `/Users/axient/repository/episodic/docs/analysis/src/prompt.md`
- Transform: `/Users/axient/repository/episodic/docs/analysis/src/transform.md`
- Cross-check: `/Users/axient/repository/episodic/docs/analysis/src/cross-validation.md`

All `src/**/*.rs` files were re-read in this pass, and tests/clippy were re-run after changes.

## Validity Verdict
- Verdict: **Valid to proceed**.
- Reason:
  1. No open high-severity defect was found.
  2. Two concrete correctness gaps were identified and fixed immediately.
  3. Remaining risks are complexity/coverage risks, not known runtime breakages.

## Changes Executed in This Pass

1. Enforced explicit `max_chars` control in reflection draft builder.
- Problem: `max_chars=0` still produced output (implicit fallback).
- Fix:
  - `/Users/axient/repository/episodic/src/transform/reflection/draft.rs`
  - `build_reflection_draft` now returns `None` when `max_chars == 0`.
- Regression test:
  - `/Users/axient/repository/episodic/src/transform/tests/reflection.rs`

2. Normalized session-id comparisons for observer partition/context grouping.
- Problem: whitespace-variant session ids could be misclassified (`" s-local "` vs `"s-local"`).
- Fixes:
  - `/Users/axient/repository/episodic/src/transform/observer/candidates.rs`
  - `/Users/axient/repository/episodic/src/transform/observer/context.rs`
- Regression tests:
  - `/Users/axient/repository/episodic/src/transform/tests/observer.rs`

3. Added explicit `OmRecord` invariant validator.
- Problem: record state correctness depended on caller discipline only.
- Fixes:
  - `/Users/axient/repository/episodic/src/model.rs`
  - `/Users/axient/repository/episodic/src/lib.rs`
  - `validate_om_record_invariants` returns typed violations (`OmRecordInvariantViolation`).
- Regression tests:
  - `/Users/axient/repository/episodic/src/model/tests.rs`

## Verification
- `cargo fmt`: pass
- `cargo test -q`: pass (all suites)
- `cargo clippy --all-targets --all-features -q`: pass

## Best Next Tasks (Non-breaking, High ROI)

1. Add property-style malformed-input tests for parser recovery boundaries.
- Why: parser already robust, but malformed input space is effectively unbounded.
- DoD:
  - Determinism and no-panic properties are asserted across generated malformed fragments.
  - Existing strict/lenient semantics remain unchanged.

2. Add micro-bench baselines for transform hot paths.
- Why: observer candidate sort/group and parse dual-pass paths are the practical hot zones.
- DoD:
  - Baseline benchmark cases are committed.
  - CI or local gate can detect obvious regressions.

3. Add optional integration checks that consume `validate_om_record_invariants`.
- Why: validator exists; host integration can choose strict reject or telemetry-only mode.
- DoD:
  - No behavior change in this crate by default.
  - Example integration snippets/tests show explicit opt-in handling.
