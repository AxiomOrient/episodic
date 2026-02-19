# Decision Log

## D-001 (2026-02-19): Defer hard state-combination invariant expansion

Status:
- Deferred

Context:
- `OmRecord` currently has multiple processing flags (`is_reflecting`, `is_buffering_reflection`).
- We added non-breaking metadata invariant checks in `validate_om_record_invariants`.
- Additional hard invariants for flag combinations may require expanding `OmRecordInvariantViolation` (public enum), which can affect downstream exhaustive matches.

Decision:
- Keep current non-breaking validation improvements.
- Defer public enum expansion and strict flag-combination invariant enforcement to a separate semver-reviewed change.

Rationale:
- Preserves API stability now.
- Keeps behavior explicit without introducing breaking public contracts accidentally.

Revisit trigger:
- Revisit when preparing next semver-planned change for model invariant API.
