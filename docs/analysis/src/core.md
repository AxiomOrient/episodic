# Core Folder Analysis (`src/` root)

## `/Users/axient/repository/episodic/src/lib.rs`
- Role: Crate facade and public API re-export hub.
- Data contracts: Re-exports model, config, parse, prompt, transform, and addon contracts.
- Control flow: No runtime flow; compile-time module wiring only.
- Purity: Pure, no side effects.
- Performance: Negligible runtime cost.
- Risks: Very wide re-export surface can increase semver coupling.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/lib.rs:12`, `/Users/axient/repository/episodic/src/lib.rs:47`.

## `/Users/axient/repository/episodic/src/model.rs`
- Role: Canonical OM state model (`OmRecord`, `OmObservationChunk`) and scope/origin enums.
- Data contracts: Explicit fields for token counts, buffering flags, generation, timestamps, and typed invariant violations.
- Control flow: `OmScope::parse/as_str`, `OmOriginType::parse/as_str`, `validate_om_record_invariants`.
- Purity: Pure data definitions and pure validation/helpers.
- Performance: Struct-only module; cost deferred to consumers.
- Risks: Multiple boolean flags still exist, but invalid scope/scope_key and orphan reflection metadata can now be detected explicitly.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/model.rs:56`, `/Users/axient/repository/episodic/src/model.rs:108`.

## `/Users/axient/repository/episodic/src/model/tests.rs`
- Role: Model contract regression suite.
- Data contracts validated: enum string roundtrip, `serde` defaulting, optional-field omission, invalid scope rejection, explicit invariant validation.
- Control flow: Builds sample record and performs explicit serialization/deserialization assertions.
- Purity: Pure tests, no external IO.
- Performance: Fast deterministic unit tests.
- Risks: Invariant coverage is targeted; full combinatorial state-space is still larger than current tests.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/model/tests.rs:43`, `/Users/axient/repository/episodic/src/model/tests.rs:212`.

## `/Users/axient/repository/episodic/src/inference.rs`
- Role: Observer/reflector transport-neutral DTO contracts.
- Data contracts: Model config, usage, pending message, observer and reflector request/response structs.
- Control flow: None.
- Purity: Pure data module.
- Performance: No algorithmic cost.
- Risks: Role and timestamps are stringly-typed; validation is delegated upstream.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/inference.rs:29`, `/Users/axient/repository/episodic/src/inference.rs:61`.

## `/Users/axient/repository/episodic/src/inference/tests.rs`
- Role: DTO wire-compatibility regression tests.
- Data contracts validated: optional field defaults/omission, request-response roundtrip stability.
- Control flow: JSON encode/decode assertions.
- Purity: Pure tests.
- Performance: Trivial.
- Risks: No schema versioning tests beyond current fields.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/inference/tests.rs:15`, `/Users/axient/repository/episodic/src/inference/tests.rs:78`.

## `/Users/axient/repository/episodic/src/context.rs`
- Role: Bounded hint builder for active observation context.
- Data contracts: Input string + max line/char budgets -> optional `om:` hint.
- Control flow: whitespace normalization, tail-window retention (`VecDeque`), explicit char budget truncation.
- Purity: Pure transform.
- Performance: Single pass over lines/chars with bounded retained lines.
- Risks: Char-based cutoff may diverge from model token budgets.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/context.rs:16`.

## `/Users/axient/repository/episodic/src/context/tests.rs`
- Role: Boundary tests for bounded hint behavior.
- Data contracts validated: zero budgets, whitespace compaction, tail selection.
- Control flow: deterministic assertions only.
- Purity: Pure tests.
- Performance: Trivial.
- Risks: No fuzzing for large unicode inputs.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/context/tests.rs:4`, `/Users/axient/repository/episodic/src/context/tests.rs:17`.

## `/Users/axient/repository/episodic/src/pipeline.rs`
- Role: Pure step planner for input/output processing.
- Data contracts: `ProcessInputStepPlan`, `ProcessOutputResultPlan`.
- Control flow: derives observer and reflection decisions; gates writes by `read_only` and step options.
- Purity: Pure decision function layer.
- Performance: O(1) scalar logic.
- Risks: Correctness inherits assumptions from record/config consistency.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/pipeline.rs:26`, `/Users/axient/repository/episodic/src/pipeline.rs:59`.

## `/Users/axient/repository/episodic/src/pipeline/tests.rs`
- Role: Planner decision regression tests.
- Data contracts validated: read-only gating, initial-step activation semantics, output-save condition.
- Control flow: fixed fixtures with explicit assertions.
- Purity: Pure tests.
- Performance: Trivial.
- Risks: Limited state-space sampling.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/pipeline/tests.rs:43`, `/Users/axient/repository/episodic/src/pipeline/tests.rs:110`.

## `/Users/axient/repository/episodic/src/addon.rs`
- Role: Host port definitions for apply/observer/reflector, and reflection command mapping.
- Data contracts: `OmReflectionCommand`, `OmCommand`, addon traits, async future aliases.
- Control flow: `reflection_command_from_action` maps `ReflectionAction` to concrete enqueue command.
- Purity: Mapping function is pure; trait methods abstract side effects to host.
- Performance: Default async wrappers allocate boxed futures.
- Risks: Runtime behavior quality depends on host trait implementations.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/addon.rs:26`, `/Users/axient/repository/episodic/src/addon.rs:69`.

## `/Users/axient/repository/episodic/src/addon/tests.rs`
- Role: Contract tests for command mapping and default async delegation.
- Data contracts validated: action-to-command mapping and sync->async delegation invariants.
- Control flow: local echo addons + manual future polling.
- Purity: Pure tests (no network/storage).
- Performance: Minimal.
- Risks: Does not test host-specific async executors; only default behavior.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/addon/tests.rs:72`, `/Users/axient/repository/episodic/src/addon/tests.rs:148`.

## `/Users/axient/repository/episodic/src/xml.rs`
- Role: XML escaping utility for text and attributes.
- Data contracts: Escapes core entities, plus quotes for attribute mode.
- Control flow: Single-pass character scan with explicit match table.
- Purity: Pure transform.
- Performance: Linear scan with one output buffer (`with_capacity`).
- Risks: Does not validate full XML grammar, only escapes text.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/xml.rs:1`, `/Users/axient/repository/episodic/src/xml.rs:20`.
