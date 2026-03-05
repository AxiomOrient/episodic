# OMv2 Episodic Protocol + Pure Transform Big-Bang Plan

Date: 2026-03-04
Status: Planning (episodic-only)
response_profile=planning_doc

## Goal
- Complete OMv2 requirements in `episodic` as a protocol and pure-transform crate.
- Deliver deterministic, versioned contracts and transforms for snapshot, continuation, canonical thread identity, reflection compaction, and hint rendering.
- Exclude runtime/DB concerns from this plan by design.

## Scope
### In Scope (episodic only)
- Protocol types and contract versions.
- Pure transforms (no IO, no DB, no network):
  - canonical thread resolution
  - continuation reducer
  - deterministic fallback inference
  - reflection entry coverage merge
  - search-visible snapshot materialization
  - priority-aware deterministic hint rendering
- Prompt contract DTO + parser/validator diagnostics.
- Contract fixtures/golden tests and compatibility docs.

### Out of Scope (host runtime)
- SQLite schema, migrations, storage adapters.
- Search/read-path wiring in AxiomMe runtime.
- Telemetry/metrics emission and CLI integration.

## Constraints
- Determinism first: identical input must produce byte-identical output.
- No hidden side effects: all behavior via explicit inputs/outputs.
- No legacy dual-path in crate internals; OMv2 model becomes canonical.
- Breaking surface is allowed and must be documented explicitly (`0.1.x -> 0.2.0`).
- Backward compatibility shims are not required in this wave.

## Approach Comparison
### Option A: Minimal Patch-on-Current
- Summary: add missing functions/types without restructuring existing contracts.
- Pros: smallest immediate diff.
- Cons: leaves contract fragmentation and keeps ambiguous boundaries.

### Option B: Protocol-First Big-Bang (Selected)
- Summary: define OMv2 contract set first, then migrate all pure transforms and tests in one cohesive release.
- Pros: strongest correctness boundary, least semantic drift, clear release story.
- Cons: larger short-term change set and tighter verification burden.

### Option C: Dual API (v1+v2 Parallel)
- Summary: keep old and new APIs simultaneously.
- Pros: incremental migration comfort.
- Cons: contradicts current requirement (no parallel path), doubles maintenance/testing complexity.

### Decision
- Select **Option B**.
- Rationale: user priority is accuracy first and explicit big-bang renewal; `episodic` is protocol/pure layer so single-cutover is feasible with robust fixture gating.

## Priority Matrix (Eisenhower)
| Quadrant | Items |
|---|---|
| Urgent + Important | OM-04 canonical thread ref, OM-02 continuation reducer, OM-06 entry-based reflection |
| Important + Not Urgent | OM-08 governance docs/compatibility matrix, OM-03 parser diagnostics hardening |
| Urgent + Less Important | Naming cleanups/export reordering after contract freeze |
| Not Urgent + Less Important | Additional convenience helpers not required by OM2 done criteria |

## Data Model
### New/Updated Protocol Types
- `OmThreadRefV2`
  - `canonical_thread_id`, `scope`, `scope_key`, `origin_thread_id`, `origin_session_id`, `resource_id`
- `OmContinuationStateV2`
  - `scope_key`, `thread_id`, `current_task`, `suggested_response`, `confidence_milli`, `source_kind`, `source_message_ids`, `updated_at_rfc3339`, `staleness_budget_ms`
- `OmContinuationCandidateV2`
- `ContinuationPolicyV2`
- `DeterministicObservationEngineV2` outputs:
  - `observations`, `current_task`, `suggested_response`, `evidence`, `confidence_milli`
- `OmObservationEntryV2`
  - `entry_id`, `scope_key`, `thread_id`, `priority`, `text`, `source_message_ids`, `origin_kind`, `created_at_rfc3339`, `superseded_by`
- `OmReflectionResponseV2`
  - `covers_entry_ids`, `reflection_text`, optional continuation
- `OmSearchVisibleSnapshotV2`
  - `scope_key`, `activated_entry_ids`, `buffered_entry_ids`, `current_task`, `suggested_response`, `rendered_hint`, `materialized_at_rfc3339`, `snapshot_version`
- `OmHintPolicyV2`

### Deterministic Invariants
- Thread identity resolver must return stable canonical id for same input tuple.
- Continuation reducer conflict resolution is total-order deterministic.
- Reflection apply is idempotent on identical entry graph + coverage set.
- Snapshot and hint renderer are byte-stable for identical source payload.

## Execution Phases
### Phase 0: Contract Freeze and Baseline Capture
- Define final OMv2 `episodic`-only boundary and breakpoints.
- Freeze naming, field sets, and deterministic tie-break rules.
- Tasks: `EPI-OMV2-015`, `EPI-OMV2-016`
- Narrow verification:
  - contract checklist review
  - fixture manifest draft exists

### Phase 1: Canonical Thread + Continuation Core
- Implement canonical thread resolver and continuation reducer.
- Integrate deterministic continuation candidate arbitration policy.
- Tasks: `EPI-OMV2-017`, `EPI-OMV2-018`
- Narrow verification:
  - thread alias/mixed-id fixtures
  - continuation conflict fixtures (same scope multi-thread)

### Phase 2: Deterministic Fallback Engine V2
- Expand fallback from continuation-only to full observer response inference with evidence/confidence.
- Ensure multilingual and identifier-preserving behavior remains deterministic.
- Tasks: `EPI-OMV2-019`
- Narrow verification:
  - evidence-presence tests
  - confidence suppression tests for low-confidence suggestions

### Phase 3: Entry-Based Reflection Model
- Introduce entry/coverage protocol and replace line-based reflection merge contract.
- Implement pure apply transform over entry sets.
- Tasks: `EPI-OMV2-020`, `EPI-OMV2-021`
- Narrow verification:
  - line-wrap variation must not alter apply result
  - replay idempotence tests

### Phase 4: Search-Visible Snapshot + Hint Renderer
- Implement snapshot materializer from activated/buffered entries + continuation state.
- Implement `render_search_hint(snapshot, policy)` with deterministic tie-break and reservation slots.
- Tasks: `EPI-OMV2-022`, `EPI-OMV2-023`
- Narrow verification:
  - current-task reservation survival
  - high-priority survival under tail-noise
  - same-input byte-identical output

### Phase 5: Prompt Contract and Parse Diagnostics Completion
- Finalize prompt contract schema behavior and strict diagnostics for mismatch/missing fields.
- Keep parser recovery deterministic and contract-version aware.
- Tasks: `EPI-OMV2-024`
- Narrow verification:
  - contract-version mismatch error snapshots
  - required-field diagnostic snapshots

### Phase 6: Governance, Compatibility, Release Artifacts
- Publish protocol docs and compatibility policy.
- Add consumer-facing conformance fixture set and release checklist.
- Tasks: `EPI-OMV2-025`
- Narrow verification:
  - docs existence + lint pass
  - fixture suite reproducibility

## Critical Path
1. `EPI-OMV2-015` Contract freeze
2. `EPI-OMV2-017` Canonical thread resolver
3. `EPI-OMV2-018` Continuation reducer
4. `EPI-OMV2-020` Entry model + reflection coverage contract
5. `EPI-OMV2-022` Snapshot materializer
6. `EPI-OMV2-023` Hint renderer
7. `EPI-OMV2-024` Prompt/parse diagnostics completion
8. `EPI-OMV2-025` Governance + release artifacts

## Decision Gates
- Gate 1 (after Phase 0): Contract freeze signed; no unresolved field naming conflicts.
- Gate 2 (after Phase 1): Mixed session/thread identity fixtures pass deterministically.
- Gate 3 (after Phase 3): Reflection entry coverage is idempotent across wrap/ordering perturbations.
- Gate 4 (after Phase 4): Snapshot+hint byte-stability proven by golden fixtures.
- Gate 5 (after Phase 6): Protocol docs + compatibility fixtures + release checklist complete.

## Verification Strategy
### Per-Phase Verification
- Every phase ships with:
  - unit tests for pure transform
  - golden fixtures under `tests/fixtures/contracts/`
  - deterministic replay test (same input xN run)

### Repository Gates
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo audit -q`

### Required Fixture Groups
- session scope, single thread
- resource scope, mixed thread/session ids
- async-only observation bursts with continuation drift risk
- strict parser failure -> deterministic fallback
- reflection rewrap/merge-split perturbation
- high-priority survival under bounded hint budget

## Risk/Rollback
### Risks
- R1: Contract rename churn causing fixture drift.
- R2: Entry-based reflection semantics misunderstood (coverage vs replacement).
- R3: Deterministic fallback overfitting to known language markers.

### Mitigation
- Freeze schema and naming before transform implementation.
- Force coverage-based tests before merge logic acceptance.
- Include multilingual negative fixtures and ambiguity suppression tests.

### Rollback
- Branch-level rollback only (no runtime migration impact in episodic).
- If gate fails, revert to last green commit and keep release untagged.
- No partial publish; release only after Gate 5 pass.

## Plan/Task Mapping
- Phase 0 -> `EPI-OMV2-015`, `EPI-OMV2-016`
- Phase 1 -> `EPI-OMV2-017`, `EPI-OMV2-018`
- Phase 2 -> `EPI-OMV2-019`
- Phase 3 -> `EPI-OMV2-020`, `EPI-OMV2-021`
- Phase 4 -> `EPI-OMV2-022`, `EPI-OMV2-023`
- Phase 5 -> `EPI-OMV2-024`
- Phase 6 -> `EPI-OMV2-025`

## Execution Update (2026-03-05)
- Completed:
  - `EPI-OMV2-015`: protocol freeze artifacts added (`docs/PROTOCOL.md`)
  - `EPI-OMV2-016`: compatibility/governance artifacts added (`docs/COMPATIBILITY.md`, `docs/RELEASE_CHECKLIST.md`)
  - `EPI-OMV2-017`: canonical thread reference contract + pure resolver implemented
  - `EPI-OMV2-018`: continuation reducer implemented and observer transform export chain connected
  - `EPI-OMV2-019`: deterministic observer response v2 fallback implemented (evidence/confidence + low-confidence suppression)
  - `EPI-OMV2-020`: entry-based reflection protocol types stabilized with serde contract tests
  - `EPI-OMV2-021`: reflection apply migrated to entry-coverage contract (`apply_reflection_response_v2`) with idempotence/line-wrap perturbation tests
  - `EPI-OMV2-022`: search-visible snapshot protocol/model and materializer transform implemented
  - `EPI-OMV2-023`: deterministic hint renderer implemented with reservation/high-priority/tie-break rules
  - `EPI-OMV2-024`: prompt contract parser diagnostics completed (version mismatch + missing required field + request-kind mismatch)
  - `EPI-OMV2-025`: conformance fixture suite and release package updates completed (`0.2.0`)
  - `EPI-OMV2-026`: reflection apply guard completed (unmatched coverage IDs now strict no-op)
  - `EPI-OMV2-027`: continuation reducer metadata consistency completed (non-winning candidate keeps prior metadata)
  - `EPI-OMV2-028`: deterministic identifier extraction completed for CJK non-whitespace error strings
  - `EPI-OMV2-029`: release checklist synchronization completed with executed gates
  - `EPI-OMV2-030`: option 2 hotspot precision pass completed (question task-signal precision, parse metadata single-tokenization, prompt contract marker single-source, snapshot/continuation clone reduction)
  - `EPI-OMV2-031`: option 2 hotspot micro-benchmark harness added (criterion bench target for parse/continuation)
- Code evidence:
  - `src/model.rs`: `OmThreadRefV2` protocol type
  - `src/transform/scope.rs`: `resolve_canonical_thread_ref(...)`
  - `src/transform/tests/scope.rs`: canonical thread resolver deterministic/mixed-id tests
  - `src/transform/observer/continuation.rs`: `resolve_continuation_update(...)`
  - `src/transform/observer/synthesis.rs`: `infer_deterministic_observer_response(...)`
  - `src/transform/{observer/mod.rs,mod.rs}`: 신규 reducer/fallback export chain
  - `src/model.rs`: `OmContinuationStateV2`, `OmObservationEntryV2`, `OmReflectionResponseV2`, `OmDeterministicObserverResponseV2`
  - `src/transform/tests/continuation.rs`: reducer deterministic conflict tests
  - `src/transform/tests/observer.rs`: observer response v2 evidence/confidence tests
  - `src/model/tests.rs`: v2 protocol serde contract tests
  - `src/transform/reflection/apply.rs`: `apply_reflection_response_v2(...)`
  - `src/transform/snapshot.rs`: `materialize_search_visible_snapshot(...)`, `render_search_hint(...)`
  - `src/model.rs`: `OmSearchVisibleSnapshotV2`, `OmHintPolicyV2`, `OM_SEARCH_VISIBLE_SNAPSHOT_V2_VERSION`
  - `src/transform/tests/{reflection.rs,snapshot.rs}`: entry-coverage apply + snapshot/hint deterministic tests
  - `src/prompt/parser.rs`: prompt contract parser + deterministic diagnostics
  - `src/transform/reflection/apply.rs`: unmatched coverage no-op guard
  - `src/transform/observer/continuation.rs`: metadata consistency rules for non-winning candidate
  - `src/transform/observer/synthesis.rs`: span-based identifier extraction for non-whitespace CJK error text
  - `src/transform/observer/synthesis.rs`: question-based task signal precision gating (`?` 단독 오탐 완화)
  - `src/parse/thread.rs`: current/suggested metadata 추출 단일 토큰화 경로
  - `src/prompt/contract.rs`, `src/prompt/{system.rs,user.rs}`: contract marker 문구 single-source 상수화
  - `src/transform/snapshot.rs`, `src/transform/observer/continuation.rs`: 불필요 clone 경감
  - `src/transform/tests/observer.rs`: question signal 정밀화 회귀 테스트 추가
  - `benches/hotpaths.rs`, `Cargo.toml`: micro-benchmark target and benchmark dependency wiring
  - `src/prompt/mod.rs`, `src/lib.rs`: parser/export wiring
  - `tests/fixtures/contracts/*`: protocol conformance fixtures (observer/reflector, valid/invalid)
  - `tests/contract_fixtures.rs`: fixture-driven parser conformance tests
  - `Cargo.toml`: package version `0.2.0`
  - `docs/RELEASE_NOTES.md`, `docs/COMPATIBILITY.md`: release/compatibility 업데이트
  - `docs/RELEASE_CHECKLIST.md`: executed gate 결과 체크 동기화
- Verification evidence:
  - `cargo test transform::tests::scope:: -- --nocapture`
  - `cargo test model::tests::om_thread_ref_v2_roundtrip_and_optional_field_omission_is_stable -- --nocapture`
  - `cargo test -q resolve_continuation_update`
  - `cargo test -q deterministic_observer_response_v2`
  - `cargo test -q observation_entry_and_reflection_response_v2_roundtrip_is_stable`
  - `cargo test -q apply_reflection_response_v2`
  - `cargo test -q materialize_search_visible_snapshot`
  - `cargo test -q render_search_hint_`
  - `cargo test -q search_visible_snapshot_v2_roundtrip_and_optional_field_omission_is_stable`
  - `cargo test -q prompt::tests::parse_observer_prompt_contract_v2_`
  - `cargo test -q prompt::tests::parse_reflector_prompt_contract_v2_reports_missing_required_field`
  - `cargo test -q --test contract_fixtures`
  - `cargo test -q apply_reflection_response_v2_noops_when_covered_ids_do_not_match_entries`
  - `cargo test -q resolve_continuation_update_preserves_previous_fields_on_weaker_candidate`
  - `cargo test -q deterministic_continuation_extracts_identifier_from_cjk_error_without_whitespace`
  - `cargo test -q transform::tests::observer::deterministic_continuation_ignores_code_like_question_token_without_request_cues`
  - `cargo bench --bench hotpaths --no-run`
  - `cargo fmt --all`
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test -q`
  - `cargo audit -q`
  - `cargo package --allow-dirty`
