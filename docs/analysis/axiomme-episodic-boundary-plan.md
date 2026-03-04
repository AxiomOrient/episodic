# AxiomMe + episodic Boundary-Preserving Plan

Date: 2026-02-17
Status: Implemented (Phase 1), re-reviewed

## Final Decision

Best method is a dual-path boundary model with explicit control:

1. Realtime path (ephemeral):
- `episodic` output is converted into request-scoped runtime hints.
- Hints are used for retrieval/context only.
- Hints are not persisted.

2. Checkpoint path (durable):
- At session or phase boundary, curated facts are explicitly promoted.
- Promotion uses deterministic, local, pure transforms only.
- Auto memory extraction is disabled for this checkpoint path.

This is the smallest change that preserves short-term vs long-term boundaries.

## Why This Is Best

Evidence:
- `episodic` is pure OM core and excludes storage/runtime wiring.
  - `/Users/axient/repository/episodic/README.md:16`
- `AxiomMe` is standalone runtime with integrated OM/search/state/memory.
  - `/Users/axient/repository/AxiomMe/docs/README.md:15`
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/lib.rs:22`
- Current append coupling persists episodic output as regular message and can enter OM + commit path.
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/client/om_bridge_service.rs:40`
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/session/lifecycle.rs:90`
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/session/commit.rs:198`

## Design Principles Applied

- Model data explicitly with typed enums/structs.
- Avoid unnecessary abstraction by extending existing search/session services.
- Prefer pure transforms for normalization, dedup, and apply planning.
- Remove decorative behavior (no hidden fallback semantics in promotion path).
- Keep performance predictable by banning network/LLM in promotion path.

## Code-Level Change Spec

### A) Realtime Request Hints (Ephemeral Only)

Files:
- `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/models/search.rs`
- `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/client/search/mod.rs`

Add explicit types:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeHintKind {
    Observation,
    CurrentTask,
    SuggestedResponse,
    External,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeHint {
    pub kind: RuntimeHintKind,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>, // e.g. "episodic"
}
```

Add field:
- `SearchRequest.runtime_hints: Vec<RuntimeHint>` with `#[serde(default)]`

Pure transform functions:
- `normalize_runtime_hints(runtime_hints: &[RuntimeHint], max_chars: usize) -> Vec<String>`
- `merge_runtime_om_recent_hints(runtime: &[String], om: Option<&str>, recent: &[String], policy: OmHintPolicy) -> Vec<String>`

Deterministic rules:
- trim and whitespace normalize
- drop empty
- dedup normalized duplicates
- preserve existing OM policy semantics while adding runtime hints:
  - if OM hint exists, reserve one OM slot first
  - reserve up to `OmHintPolicy.keep_recent_with_om` recent slots
  - runtime hints fill only the remaining budget after reserved OM/recent slots
  - final stable order is `recent_reserved -> OM(if any) -> runtime_budgeted -> recent_tail_budgeted`
- if OM hint does not exist, fill `runtime` first, then `recent`
- enforce `OmHintPolicy.total_hint_limit`
- enforce per-item char cap from `OmHintBounds.max_chars`

### B) Checkpoint Promotion (Durable Only)

Files:
- `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/models/session.rs`
- `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/session/commit.rs`
- `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/client/runtime_service.rs`

Add explicit typed models:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryCategory {
    Profile,
    Preferences,
    Entities,
    Events,
    Cases,
    Patterns,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PromotionApplyMode {
    AllOrNothing,
    BestEffort,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CommitMode {
    ArchiveAndExtract,
    ArchiveOnly,
}
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryPromotionFact {
    pub category: MemoryCategory,
    pub text: String,
    pub source_message_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>, // e.g. "episodic"
    pub confidence_milli: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryPromotionRequest {
    pub session_id: String,
    pub checkpoint_id: String,
    pub apply_mode: PromotionApplyMode,
    pub facts: Vec<MemoryPromotionFact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryPromotionResult {
    pub session_id: String,
    pub checkpoint_id: String,
    pub accepted: usize,
    pub persisted: usize,
    pub skipped_duplicates: usize,
    pub rejected: usize,
}
```

Add APIs:
- `Session::promote_memories(request: &MemoryPromotionRequest) -> Result<MemoryPromotionResult>`
- `Session::commit_with_mode(mode: CommitMode) -> Result<CommitResult>`

Required behavior:
- `promote_memories` uses deterministic local dedup only.
- `promote_memories` does not call network/LLM paths.
- `commit_with_mode(ArchiveOnly)` archives messages but skips auto extraction.
- Existing `commit()` remains backward compatible and maps to `ArchiveAndExtract`.
- `CommitMode` is intentionally two-value only to avoid unnecessary mode surface.

### C) Deterministic Promotion Path (No Hidden Fallback)

Promotion dedup policy:
- exact normalized text + category match only.
- stable canonical key generation.
- no LLM scoring.
- no network I/O.

Pure transform functions:
- `normalize_promotion_facts(facts: &[MemoryPromotionFact]) -> Vec<MemoryPromotionFact>`
- `dedup_promotion_facts(facts: &[MemoryPromotionFact]) -> Vec<MemoryPromotionFact>`
- `plan_promotion_apply(existing: &[ExistingMemoryFact], incoming: &[MemoryPromotionFact]) -> PromotionApplyPlan`

### D) Atomicity and Partial-Failure Policy

`PromotionApplyMode::AllOrNothing`:
- validate all facts first.
- if any fail, persist none.
- guarantee scope is **operation-level atomicity during one process execution** for promoted memory docs.
- implementation rule:
  - acquire per-session promotion lock before apply
  - compute full apply plan first
  - render final target file contents in memory
  - capture pre-write snapshots for touched targets
  - write each target via temp-file + atomic rename
  - if an in-process write fails, restore all touched targets from snapshots and return error
  - enqueue/reindex is follow-up; if it fails, facts remain persisted and reconcile path repairs index state
- explicit non-goal:
  - no cross-file crash-global transaction guarantee is claimed
  - crash recovery is handled by checkpoint phase reconciliation in Section F

`PromotionApplyMode::BestEffort`:
- validate and apply per fact.
- return `rejected` and `persisted` counts explicitly.

This removes ambiguity and makes failure semantics explicit.

### E) Boundary Invariants (Must Hold)

- Runtime hints are request-scoped and never persisted.
- Runtime hints never touch `messages.jsonl`, OM outbox, or commit extractor inputs.
- Durable memory creation in boundary-safe flow occurs only via explicit promotion.
- Checkpoint flow uses `CommitMode::ArchiveOnly` after promotion.
- Promotion path performs no network/LLM calls.

### F) Idempotency Storage Contract

Promotion idempotency is keyed by `(session_id, checkpoint_id)`.

State store additions:
- table `memory_promotion_checkpoints`
  - `session_id TEXT NOT NULL`
  - `checkpoint_id TEXT NOT NULL`
  - `request_hash TEXT NOT NULL`
  - `request_json TEXT NOT NULL`
  - `phase TEXT NOT NULL` (`pending` | `applying` | `applied`)
  - `result_json TEXT`
  - `applied_at TEXT`
  - `attempt_count INTEGER NOT NULL DEFAULT 0`
  - `updated_at TEXT NOT NULL`
  - primary key `(session_id, checkpoint_id)`

Rules:
- new `(session_id, checkpoint_id)`:
  - insert checkpoint row as `phase=pending` before applying file writes.
- apply claim is CAS-based:
  - promote worker must claim by `UPDATE ... SET phase='applying', attempt_count=attempt_count+1 WHERE session_id=? AND checkpoint_id=? AND request_hash=? AND phase='pending'`.
  - only rows-affected `== 1` worker may execute file writes.
- same `(session_id, checkpoint_id)` and same `request_hash` with `phase=applied`:
  - return stored `result_json` without re-applying writes.
- same `(session_id, checkpoint_id)` and same `request_hash` with `phase=pending`:
  - caller attempts CAS claim (`pending -> applying`).
  - single winner applies writes; non-winners retry and observe `applying` or `applied`.
- same `(session_id, checkpoint_id)` and same `request_hash` with `phase=applying`:
  - return retryable `checkpoint_busy` error; caller retries with backoff.
- claimer finalization:
  - after successful writes, update `phase='applied'` with `result_json` and `applied_at`.
- stale apply reconcile:
  - startup or next promotion checks stale `phase=applying` rows (timeout threshold).
  - stale rows are moved back to `phase=pending` and replayed from stored `request_json` deterministically.
- same `(session_id, checkpoint_id)` and different `request_hash`:
  - fail with validation conflict.
- session deletion:
  - deleting session data must also delete checkpoint rows for the same `session_id`.
- `request_hash` must be deterministic:
  - normalize facts first (trim, whitespace normalize, stable category/text ordering, dedup source ids)
  - serialize canonical JSON with stable key order
  - hash with `blake3`

### G) Promotion Input Bounds

To keep performance predictable and enforce explicit control:
- `facts.len()` max: `64`
- `fact.text` max chars: `512`
- `source_message_ids` max per fact: `32`
- `confidence_milli` range: `0..=1000`

Out-of-bounds input is rejected during validation before apply planning.

### H) Performance Gate (Phased, Non-Decorative)

- Phase 1 (required now): preserve functional correctness and boundary invariants with zero network/LLM in promotion path.
- Phase 2 (after feature stabilization): add numeric regression budgets for promotion and commit latencies.
- hard-fail performance gate is enabled only after baseline is captured on representative hardware profile.

## Task Plan

1. Add runtime hint types and request field.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/models/search.rs`
- Done when:
  - `SearchRequest` supports `runtime_hints` with backward-compatible deserialize.

2. Wire runtime hints into search pipeline.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/client/search/mod.rs`
- Done when:
  - retrieval hints include runtime hints with deterministic limits.
  - merge preserves OM slot and recent reservation policy when OM hint exists.

3. Add pure hint normalization and merge functions.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/client/search/mod.rs`
- Done when:
  - no IO/state side effects and testable deterministic outputs.

4. Add typed promotion and commit mode models.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/models/session.rs`
- Done when:
  - `MemoryCategory`, `PromotionApplyMode`, two-value `CommitMode` exist and are serialized.

5. Export new model types for public API surface.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/models/mod.rs`
- Done when:
  - newly added runtime/promotion/commit model types are re-exported consistently.

6. Implement deterministic promotion transform helpers.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/session/commit.rs`
- Done when:
  - normalize/dedup/apply-plan functions are pure and LLM-free.

7. Implement `Session::promote_memories` with explicit apply mode.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/session/commit.rs`
- Done when:
  - `AllOrNothing` and `BestEffort` semantics are both implemented and tested.
  - input bounds validation is implemented before apply plan.
  - `AllOrNothing` guarantees in-process rollback on write failure.

8. Implement `Session::commit_with_mode`.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/session/commit.rs`
- Done when:
  - `ArchiveOnly` is available and `commit()` remains backward compatible.

9. Expose runtime API entrypoint for promotion/checkpoint flow.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/client/runtime_service.rs`
- Done when:
  - caller can run promotion plus archive-only checkpoint without internal coupling.

10. Add idempotency state storage and accessors with checkpoint phase.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/state/migration.rs`
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/state/mod.rs`
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/state/promotion_checkpoint.rs` (new)
- Done when:
  - checkpoint record table exists.
  - lookup/insert/update methods enforce hash conflict rule and `pending/applying/applied` CAS transition.
  - same-hash concurrent apply has single winner and others receive retryable busy response.

11. Implement checkpoint-phase reconcile path.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/session/commit.rs`
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/state/mod.rs`
- Done when:
  - replay from `request_json` can finalize `pending` to `applied` deterministically.
  - stale `applying` rows are demoted to `pending` and reconciled safely.

12. Add session-delete cleanup for promotion checkpoints.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/client/runtime_service.rs`
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/state/mod.rs`
- Done when:
  - deleting a session removes corresponding checkpoint records.

13. Update integration example to dual-path boundary flow.
- Files:
  - `/Users/axient/repository/examples/axiomme-episodic-example/src/lib.rs`
  - `/Users/axient/repository/examples/axiomme-episodic-example/src/main.rs`
  - `/Users/axient/repository/examples/axiomme-episodic-example/README.md`
- Done when:
  - realtime uses runtime hints only.
  - checkpoint uses promote plus `CommitMode::ArchiveOnly`.

14. Add runtime-hint tests.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/client/search/mod.rs`
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/client/search/backend_tests.rs`
- Done when:
  - runtime hints change retrieval context but leave persisted state unchanged.
  - runtime merge keeps OM/recent reservations stable under `total_hint_limit`.

15. Add promotion and checkpoint tests.
- Files:
  - `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/session/tests.rs`
- Done when:
  - typed category validation is enforced.
  - promotion is idempotent.
  - `ArchiveOnly` prevents auto extraction.
  - `AllOrNothing` applies with process-scope atomicity.
  - `BestEffort` returns explicit reject counts.
  - same checkpoint same hash is no-op replay.
  - same checkpoint different hash returns conflict.
  - same checkpoint applying state returns retryable busy.
  - concurrent same-checkpoint same-hash requests produce single CAS winner.
  - stale applying state is reconciled deterministically.
  - semantically same facts produce same `request_hash` regardless of input order.
  - out-of-bounds facts are rejected pre-apply.
  - session delete removes promotion checkpoint rows.

16. Add end-to-end integration tests.
- Files:
  - `/Users/axient/repository/examples/axiomme-episodic-example/tests/integration_steps.rs`
- Done when:
  - “episodic -> runtime hints -> promote -> archive-only checkpoint” passes.

17. Sync canonical docs.
- Files:
  - `/Users/axient/repository/AxiomMe/docs/API_CONTRACT.md`
  - `/Users/axient/repository/AxiomMe/docs/FEATURE_SPEC.md`
- Done when:
  - runtime hint and promotion/checkpoint contracts are documented.

18. Verification gate (functional).
- Commands:
  - `cargo fmt`
  - `cargo test -p axiomme-core`
  - `cargo test` (example project)
  - `bash /Users/axient/repository/AxiomMe/scripts/quality_gates.sh`
- Done when:
  - all commands pass without new regressions.

19. Verification gate (performance, phased).
- Commands:
  - baseline capture:
    - `bash /Users/axient/repository/AxiomMe/scripts/perf_regression_gate.sh --window-size 1 --required-passes 1 --min-cases 6 --output /tmp/axiomme-perf-baseline.json`
  - post-baseline hard gate:
    - `bash /Users/axient/repository/AxiomMe/scripts/perf_regression_gate.sh --window-size 2 --required-passes 2 --min-cases 6`
- Done when:
  - baseline is captured first.
  - regression threshold is enforced only after baseline ratification.

## Test Cases (Concrete)

1. `runtime_hint_serde_backward_compat`
- request payload without `runtime_hints` remains valid.

2. `normalize_runtime_hints_trims_dedups_and_caps_chars`
- deterministic normalization and char caps.

3. `merge_runtime_om_recent_hints_preserves_om_slot_and_recent_reservation`
- OM slot and `keep_recent_with_om` reservation remain stable with runtime hints present.

4. `search_with_runtime_hints_has_no_message_or_outbox_side_effect`
- retrieval only, no persistence artifacts.

5. `promotion_rejects_invalid_category_and_empty_text`
- typed category and text validation enforced.

6. `promotion_idempotent_on_same_checkpoint_and_same_facts`
- repeated promotion does not duplicate stored memory facts.

7. `promotion_all_or_nothing_restores_snapshots_on_in_process_write_failure`
- touched targets are restored on in-process failure; no partial persisted state remains.

8. `promotion_best_effort_persists_valid_and_reports_rejected`
- mixed inputs produce explicit persisted/rejected counts.

9. `promotion_same_checkpoint_same_hash_returns_cached_result`
- no duplicate writes; result replayed from checkpoint record.

10. `promotion_same_checkpoint_different_hash_rejected`
- deterministic conflict error.

11. `promotion_same_checkpoint_same_hash_pending_reconciles_and_applies_once`
- pending checkpoint is replayed deterministically and finalized as applied exactly once.

12. `promotion_same_checkpoint_same_hash_applying_returns_retryable_busy`
- concurrent in-flight apply does not double-write and returns retryable busy to non-owner callers.

13. `promotion_concurrent_claim_has_single_cas_winner`
- only one request can transition checkpoint phase `pending -> applying`.

14. `promotion_stale_applying_reconcile_replays_deterministically`
- stale in-flight apply rows are demoted and replayed without duplicate persistence.

15. `promotion_request_hash_canonicalization_is_order_independent`
- same facts in different order yield same hash.

16. `promotion_rejects_out_of_bounds_facts_before_apply`
- max facts/chars/source ids/confidence range enforced pre-write.

17. `commit_mode_archive_only_skips_auto_extraction`
- archive occurs, auto extraction path skipped.

18. `delete_session_cleans_promotion_checkpoints`
- session deletion removes checkpoint rows for that session id.

19. `dual_path_integration_runtime_then_checkpoint`
- boundary-safe end-to-end flow validated.

20. `promotion_and_commit_latency_regression_within_budget`
- enabled only after baseline ratification; protects performance predictability.

## Re-Review (Validity and Consistency)

Checklist:
- Explicit data modeling: pass (`MemoryCategory`, `CommitMode`, `PromotionApplyMode`).
- No unnecessary abstraction: pass (extends existing search/session/runtime layers).
- Pure transforms prioritized: pass (normalize, dedup, apply-plan separated and side-effect free).
- Decorative behavior removed: pass (no hidden fallback in promotion path).
- Persistence boundary explicit: pass (runtime hint vs checkpoint promotion).
- Performance predictability: pass (promotion path disallows network/LLM).
- Backward compatibility: pass (`commit()` preserved; `commit_with_mode` is additive).
- Failure semantics explicit: pass (`AllOrNothing` and `BestEffort` specified with process-scope guarantee).
- Crash-consistency overclaim avoided: pass (checkpoint phase reconcile path documented).
- Idempotency lifecycle explicit: pass (pending/applying/applied transition, CAS claim, replay, and delete cleanup defined).
- Runtime hint merge compatibility: pass (OM reservation policy preserved).
- API exposure completeness: pass (`models/mod.rs` export task added).
- Input bounds explicit: pass (facts/chars/source ids/confidence constrained).

Final verdict:
- This plan is concrete, internally consistent, and currently the best method for dual-use with strict boundary preservation and calibrated guarantees.

## Implementation Reality Check (2026-02-17)

Completed against this plan:
- Runtime hint model + merge path implemented in `AxiomMe` search request flow.
- Typed promotion models + `CommitMode::ArchiveOnly` + runtime service APIs implemented.
- Promotion checkpoint table/state machine (`pending|applying|applied`) implemented.
- Stale `applying` demotion + deterministic replay path implemented in promotion entrypoint.
- Session delete cleanup removes promotion checkpoints.
- Promotion `persisted` metric is fact-write count (not unique file count), aligned across apply modes.
- Checkpoint pending insert is conflict-tolerant (`INSERT OR IGNORE`) to avoid false errors under same-checkpoint races.
- Dual-path example (`episodic` + `AxiomMe`) updated to runtime-hint phase and explicit checkpoint phase.
- Canonical docs updated:
  - `/Users/axient/repository/AxiomMe/docs/API_CONTRACT.md`
  - `/Users/axient/repository/AxiomMe/docs/FEATURE_SPEC.md`

Validation run:
- `cargo test -p axiomme-core`: pass
- `cargo test` (`/Users/axient/repository/examples/axiomme-episodic-example`): pass
- `bash /Users/axient/repository/AxiomMe/scripts/quality_gates.sh`: pass
- `bash /Users/axient/repository/AxiomMe/scripts/perf_regression_gate.sh --window-size 1 --required-passes 1 --output /tmp/axiomme-perf-baseline-v2.json`: pass
  - observed: `executed_cases=21`, `top1_accuracy=0.95238096`, `stress_top1_accuracy=0.93333334`
- `bash /Users/axient/repository/AxiomMe/scripts/perf_regression_gate.sh --window-size 2 --required-passes 2 --min-cases 12`: pass

Scope note:
- Phase 2 performance gate is now runnable with stronger signal (`include_stress=true`, `MIN_CASES=12`) in `scripts/perf_regression_gate.sh`.
- Remaining limitation: `p95_latency_ms` can still quantize to `0` on fast local runs; if stricter latency sensitivity is required, additive `p95_latency_us` metric should be introduced in benchmark report/gate model.
