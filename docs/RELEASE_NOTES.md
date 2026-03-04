# Release Notes

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
