# Release Notes

## 0.2.2 (2026-03-05)

### Highlights
- Search hint high-priority slot accounting corrected in `render_search_hint(...)`:
  - high-priority slot counters now increase only when a line is actually inserted.
  - duplicate high-priority lines no longer consume reserved high slots.
- Added regression coverage for duplicate high-priority observations:
  - `render_search_hint_fills_high_priority_slots_after_duplicate_high_entries`

### Compatibility Impact
- Crate version bumped from `0.2.1` to `0.2.2`.
- Protocol version remains `om-v2`; prompt contract remains `2.0.0`.
- No public protocol field removals/renames in this patch release.

## 0.2.1 (2026-03-05)

### Highlights
- Prompt/user XML data block handling hardened:
  - `request_json` is escaped and isolated as explicit data payload.
- Reflection apply and model tests extended for deterministic entry-level behavior.
- Prompt/contract and model coverage expanded to reduce protocol drift risk in host integrations.

### Compatibility Impact
- Crate version bumped from `0.2.0` to `0.2.1`.
- Protocol version remains `om-v2`; prompt contract remains `2.0.0`.
- No public protocol field removals/renames in this patch release.

## 0.2.0 (2026-03-04)

### Highlights
- OMv2 protocol + pure transform boundary completed in `episodic`.
- Entry-coverage reflection apply added:
  - `apply_reflection_response_v2(...)`
- Search-visible snapshot and deterministic hint rendering added:
  - `OmSearchVisibleSnapshotV2`
  - `OmHintPolicyV2`
  - `materialize_search_visible_snapshot(...)`
  - `render_search_hint(...)`
- Prompt contract parser diagnostics added:
  - `parse_observer_prompt_contract_v2(...)`
  - `parse_reflector_prompt_contract_v2(...)`
  - `OmPromptContractParseError`

### Compatibility Impact
- Crate version bumped from `0.1.x` to `0.2.0`.
- Protocol version remains `om-v2`; prompt contract remains `2.0.0`.
- New public protocol symbols are additive; consumers should adopt new snapshot/hint and reflection-coverage APIs where needed.
- Prompt contract parsing now provides deterministic typed diagnostics for:
  - contract version mismatch
  - missing required fields
  - request kind mismatch

### Conformance Evidence
- Added fixture suite under `tests/fixtures/contracts/*`.
- Added integration verification `tests/contract_fixtures.rs`.
- Quality gates:
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`
  - `cargo audit -q`
