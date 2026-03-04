# OMv2 Protocol Specification (episodic)

Status: Active  
Last Updated: 2026-03-04

## 1. Boundary
- `episodic` is a **protocol + pure transform** crate.
- `episodic` does not own runtime wiring, DB schema, network transport, or telemetry.
- Host runtimes (for example AxiomMe) consume this crate and perform side effects externally.

## 2. Protocol Versioning
- Protocol identifier: `om-v2`
- Prompt contract name: `axiomme.om.prompt`
- Prompt contract version: `2.0.0`
- Version constants are source-of-truth in:
  - `src/prompt/contract.rs`

## 3. Naming Freeze (OMv2 Core)
The following symbols are frozen for this implementation wave and must not be renamed without protocol review:

| Domain | Frozen Symbol |
|---|---|
| Thread identity | `OmThreadRefV2`, `resolve_canonical_thread_ref` |
| Continuation | `OmContinuationStateV2`, `resolve_continuation_update` |
| Deterministic fallback | `infer_deterministic_observer_response`, `infer_deterministic_continuation` |
| Reflection entry model | `OmObservationEntryV2`, `OmReflectionResponseV2` |
| Search snapshot | `OmSearchVisibleSnapshotV2`, `materialize_search_visible_snapshot` |
| Hint rendering | `render_search_hint` |

## 4. Deterministic Contract Rules
- Same input payload must produce byte-identical output.
- Tie-break order must be explicit in code and tests.
- No hidden global state, randomization, clock-dependent branching, or IO side effects inside pure transforms.

## 5. Change Control
- Any change to frozen symbols requires:
  1. `protocol_version` review
  2. compatibility update (`docs/COMPATIBILITY.md`)
  3. release checklist update (`docs/RELEASE_CHECKLIST.md`)
  4. fixture/golden refresh with explicit rationale

## 6. Non-Goals
- No in-crate migration logic.
- No runtime fallback compatibility shims for host persistence models.
- No host-specific defaults baked into protocol transforms.
