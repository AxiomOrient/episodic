# Transform Module Analysis (`src/transform/`)

## `/Users/axient/repository/episodic/src/transform/mod.rs`
- Role: Public transform API surface.
- Data contracts: Re-exports activation/observer/reflection/scope/types contracts.
- Control flow: None.
- Purity: Pure module wiring.
- Performance: None.
- Risks: Wide exports increase API coupling.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/mod.rs:8`.

## `/Users/axient/repository/episodic/src/transform/types.rs`
- Role: Decision/result types used across transform layer.
- Data contracts: `ActivationBoundary`, `ActivationResult`, `ObserverWriteDecision`, `ReflectionEnqueueDecision`, etc.
- Control flow: None.
- Purity: Pure type definitions.
- Performance: None.
- Risks: Boolean-rich state models may allow invalid combinations if misused.
- Verdict: `WARN`.
- Evidence: `/Users/axient/repository/episodic/src/transform/types.rs:59`, `/Users/axient/repository/episodic/src/transform/types.rs:68`.

## `/Users/axient/repository/episodic/src/transform/helpers.rs`
- Role: Shared low-level helper functions.
- Data contracts: token estimator, id merge, whitespace normalizer.
- Control flow: straightforward loops.
- Purity: Pure functions.
- Performance: Linear in input length; uses saturation and dedupe set.
- Risks: token estimator (`chars/4`) is heuristic, not tokenizer-accurate.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/helpers.rs:3`, `/Users/axient/repository/episodic/src/transform/helpers.rs:12`.

## `/Users/axient/repository/episodic/src/transform/scope.rs`
- Role: Scope key constructor.
- Data contracts: `OmScope + optional ids -> Result<String, OmTransformError>`.
- Control flow: explicit scope match and required identifier validation.
- Purity: Pure.
- Performance: O(1).
- Risks: None significant.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/scope.rs:5`.

## `/Users/axient/repository/episodic/src/transform/activation.rs`
- Role: Observation activation math and record mutation path.
- Data contracts: dynamic threshold, activation boundary, activation result.
- Control flow:
  - computes target activation boundary,
  - merges activated observations,
  - updates tokens, timestamps, message ids, buffering flags.
- Purity: Mixed. Boundary calculators are pure; `activate_buffered_observations` mutates record/chunks.
- Performance: boundary selection is linear; mutation path uses bounded drains.
- Risks:
  - exported `normalize_observation_buffer_boundary` currently not used in non-test code.
  - mutation correctness depends on caller providing ordered chunks.
- Verdict: `WARN`.
- Evidence: `/Users/axient/repository/episodic/src/transform/activation.rs:19`, `/Users/axient/repository/episodic/src/transform/activation.rs:116`.

## `/Users/axient/repository/episodic/src/transform/observer/mod.rs`
- Role: Observer submodule export surface.
- Data contracts: Re-exports candidate/context/decision/synthesis functions.
- Control flow: None.
- Purity: Pure module wiring.
- Performance: None.
- Risks: None.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/observer/mod.rs:6`.

## `/Users/axient/repository/episodic/src/transform/observer/candidates.rs`
- Role: Candidate filtering and partitioning for observer input.
- Data contracts: candidate list transforms and selected subsets.
- Control flow:
  - exclude observed IDs,
  - deterministic sort and recent truncation,
  - split pending vs other by session,
  - select observed subset.
- Purity: Pure transformations.
- Performance: sort dominates (`O(n log n)`).
- Risks: Cloning vectors multiple times may be heavy for very large candidate sets.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/observer/candidates.rs:7`, `/Users/axient/repository/episodic/src/transform/observer/candidates.rs:77`.

## `/Users/axient/repository/episodic/src/transform/observer/context.rs`
- Role: Build cross-conversation blocks and merge active+buffered observations.
- Data contracts: other-conversation XML blocks and combined observation text.
- Control flow: group by source session (`BTreeMap`), sort by time/id, truncate by char budget.
- Purity: Pure transformations.
- Performance: grouped sorting cost; linear XML escaping.
- Risks: Char truncation can cut semantic units mid-sentence.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/observer/context.rs:8`, `/Users/axient/repository/episodic/src/transform/observer/context.rs:82`.

## `/Users/axient/repository/episodic/src/transform/observer/decision.rs`
- Role: Observer trigger/state decision engine.
- Data contracts: `AsyncObservationIntervalState`, `ObserverWriteDecision`.
- Control flow:
  - threshold computation,
  - async interval boundary + debounce,
  - block-after fallback,
  - sync/async branching.
- Purity: Pure decision logic.
- Performance: O(1).
- Risks: Policy complexity is high; requires careful caller understanding.
- Verdict: `WARN`.
- Evidence: `/Users/axient/repository/episodic/src/transform/observer/decision.rs:18`, `/Users/axient/repository/episodic/src/transform/observer/decision.rs:53`.

## `/Users/axient/repository/episodic/src/transform/observer/synthesis.rs`
- Role: Synthesize concise observer input from pending messages.
- Data contracts: active observations + pending messages -> bounded synthesized text.
- Control flow: dedupe against historical and in-batch lines, fallback to ensure forward progress.
- Purity: Pure transformation.
- Performance: linear with hash set dedupe.
- Risks: Forward-progress fallback can re-emit duplicates by design when all candidates dedupe out.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/observer/synthesis.rs:7`.

## `/Users/axient/repository/episodic/src/transform/reflection/mod.rs`
- Role: Reflection submodule export surface.
- Data contracts: re-exports decision/draft/guidance/slice APIs.
- Control flow: None.
- Purity: Pure module wiring.
- Performance: None.
- Risks: None.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/reflection/mod.rs:6`.

## `/Users/axient/repository/episodic/src/transform/reflection/decision.rs`
- Role: Reflection action selection and enqueue decision synthesis.
- Data contracts: `ReflectionAction`, `ReflectionEnqueueDecision`.
- Control flow:
  - strict `>` reflector trigger,
  - async buffer/reflection branching,
  - command generation with expected generation/request timestamp.
- Purity: Pure decision logic.
- Performance: O(1).
- Risks: Multiple boolean flags (`is_reflecting`, `is_buffering_reflection`, `has_buffered_reflection`) increase state-combination complexity.
- Verdict: `WARN`.
- Evidence: `/Users/axient/repository/episodic/src/transform/reflection/decision.rs:11`, `/Users/axient/repository/episodic/src/transform/reflection/decision.rs:69`.

## `/Users/axient/repository/episodic/src/transform/reflection/draft.rs`
- Role: Draft reflection generation and merge logic.
- Data contracts: `ReflectionDraft` values + merged reflection text.
- Control flow: normalize non-empty lines, compute token estimates, char clamp output.
- Purity: Pure transform.
- Performance: linear with string joins/splits.
- Risks: aggressive whitespace normalization can remove formatting signal.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/reflection/draft.rs:25`.

## `/Users/axient/repository/episodic/src/transform/reflection/guidance.rs`
- Role: Prompt guidance text by compression level and compression validation predicate.
- Data contracts: `level -> &'static str`, `(reflected,target) -> bool`.
- Control flow: capped level mapping (0/1/2).
- Purity: Pure.
- Performance: constant.
- Risks: Guidance text is static and may drift from actual compression behavior.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/reflection/guidance.rs:1`, `/Users/axient/repository/episodic/src/transform/reflection/guidance.rs:36`.

## `/Users/axient/repository/episodic/src/transform/reflection/slice.rs`
- Role: Buffered reflection slice planner.
- Data contracts: returns `BufferedReflectionSlicePlan` with slice text and token targets.
- Control flow: computes avg tokens per line and derives line count from activation point.
- Purity: Pure transform.
- Performance: linear in line count.
- Risks: token-per-line average is heuristic, can misestimate compression target.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/reflection/slice.rs:3`, `/Users/axient/repository/episodic/src/transform/reflection/slice.rs:9`.

## `/Users/axient/repository/episodic/src/transform/tests/mod.rs`
- Role: Shared fixtures/helpers for transform test modules.
- Data contracts: `chunk` helper produces deterministic `OmObservationChunk` fixtures.
- Control flow: centralized fixture generation.
- Purity: Pure tests.
- Performance: trivial.
- Risks: Shared fixture assumptions can hide scenario diversity.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/tests/mod.rs:12`.

## `/Users/axient/repository/episodic/src/transform/tests/scope.rs`
- Role: Scope-key rule tests.
- Data contracts validated: required identifier by scope, trim behavior, error paths.
- Control flow: scenario assertions.
- Purity: Pure tests.
- Performance: trivial.
- Risks: None significant.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/tests/scope.rs:4`, `/Users/axient/repository/episodic/src/transform/tests/scope.rs:58`.

## `/Users/axient/repository/episodic/src/transform/tests/activation.rs`
- Role: Activation algorithm/state mutation regression tests.
- Data contracts validated: threshold math, boundary selection, saturation, activation mutation semantics.
- Control flow: synthetic records/chunks with explicit assertions.
- Purity: Pure tests.
- Performance: trivial.
- Risks: Does not prove optimal boundary selection for every distribution, but covers primary invariants.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/tests/activation.rs:21`, `/Users/axient/repository/episodic/src/transform/tests/activation.rs:145`.

## `/Users/axient/repository/episodic/src/transform/tests/observer.rs`
- Role: Observer selection/decision/context/synthesis regression tests.
- Data contracts validated:
  - candidate sorting/dedup,
  - interval/debounce state,
  - run/activate decision flags,
  - context block escaping and partitioning.
- Control flow: broad scenario matrix.
- Purity: Pure tests.
- Performance: trivial.
- Risks: Complex policy still has combinatorial state space beyond explicit cases.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/tests/observer.rs:356`, `/Users/axient/repository/episodic/src/transform/tests/observer.rs:471`.

## `/Users/axient/repository/episodic/src/transform/tests/reflection.rs`
- Role: Reflection decision/slice/draft regression tests.
- Data contracts validated:
  - trigger strictness,
  - buffer vs reflect action selection,
  - draft construction,
  - slice boundary math,
  - enqueue decision state transitions.
- Control flow: deterministic scenario assertions.
- Purity: Pure tests.
- Performance: trivial.
- Risks: Guidance text quality not semantically validated, only presence.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/transform/tests/reflection.rs:30`, `/Users/axient/repository/episodic/src/transform/tests/reflection.rs:132`.
