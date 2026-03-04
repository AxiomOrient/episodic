# Transform Module Review

Reviewed files:
- `src/transform/mod.rs`
- `src/transform/types.rs`
- `src/transform/helpers.rs`
- `src/transform/scope.rs`
- `src/transform/activation.rs`
- `src/transform/observer/mod.rs`
- `src/transform/observer/candidates.rs`
- `src/transform/observer/context.rs`
- `src/transform/observer/decision.rs`
- `src/transform/observer/synthesis.rs`
- `src/transform/reflection/mod.rs`
- `src/transform/reflection/decision.rs`
- `src/transform/reflection/draft.rs`
- `src/transform/reflection/guidance.rs`
- `src/transform/reflection/slice.rs`

## `src/transform/mod.rs`

Role:
- Exposes transform-layer public API.

What is implemented:
- Re-export map for activation, observer, reflection, scope, and type contracts.

## `src/transform/types.rs`

Role:
- Defines explicit data contracts for decisions, results, and errors.

What is implemented:
- Activation and reflection result structs.
- Observer candidate struct.
- Reflection action enum and enqueue decision.
- Observer decision state and async interval state.
- Error enum for scope-key validation.

Data-first notes:
- State transitions are represented as data, not hidden side effects.

## `src/transform/helpers.rs`

Role:
- Small pure helper functions.

What is implemented:
- Char-based token estimate (`chars / 4` with saturation).
- Deduplicating merge of message id lists preserving first-seen order.
- Whitespace normalizer.

Performance:
- Linear and allocation-light.

## `src/transform/scope.rs`

Role:
- Builds scope key string from scope and identifiers.

What is implemented:
- Explicit identifier selection by scope with trim and required-field error.

## `src/transform/activation.rs`

Role:
- Handles threshold math and buffered observation activation.

What is implemented:
- Dynamic threshold with shared-budget support.
- Activation boundary selection based on retention target and pending tokens.
- Merge function for active + activated observations.
- Boundary normalization helper for buffered token counters.
- Stateful activation mutation over record and buffered chunk list.

Performance notes:
- Boundary selection is one-pass with saturated arithmetic for overflow safety.

Risk:
- `normalize_observation_buffer_boundary` is exported but currently not used by non-test code.

## `src/transform/observer/mod.rs`

Role:
- Observer submodule export surface.

What is implemented:
- Re-exports candidate/context/decision/synthesis helpers.

## `src/transform/observer/candidates.rs`

Role:
- Candidate filtering and partition logic.

What is implemented:
- Select candidates excluding already observed ids, deterministic sort, recent truncation.
- Time cutoff filtering against `last_observed_at`.
- Partition into pending vs other conversations by session id.
- Post-hoc selection for observed ids.

Simplicity and performance:
- Deterministic order and linear-to-`n log n` cost due to sorting.

## `src/transform/observer/context.rs`

Role:
- Builds cross-conversation context and combines active/buffered observations.

What is implemented:
- Groups other-session messages with deterministic order and XML escaping.
- Truncates message text by char limit.
- Combines active and buffered observations with explicit separator.

Performance:
- Grouping map and per-group sort are explicit and predictable.

## `src/transform/observer/decision.rs`

Role:
- Observer trigger policy and async interval/debounce logic.

What is implemented:
- Pending token computation with saturation.
- Threshold trigger.
- Async interval state computation with detailed output fields.
- Observer write decision including:
  - threshold reached,
  - interval crossing,
  - block-after check,
  - run/activate booleans for async and sync modes.

Data-first notes:
- Decision object captures full reasoning state.

## `src/transform/observer/synthesis.rs`

Role:
- Synthesizes minimal observation text from pending messages.

What is implemented:
- Deduplicates against active observations and within current batch.
- Preserves progress via fallback when all lines dedupe out.
- Enforces max char bound on final output.

## `src/transform/reflection/mod.rs`

Role:
- Reflection submodule export surface.

## `src/transform/reflection/decision.rs`

Role:
- Chooses reflection action and maps to enqueue decision.

What is implemented:
- Reflect trigger uses strict `>` threshold.
- Action rules:
  - sync mode: reflect only above threshold,
  - async mode: buffer near activation, reflect when buffered/blocked/over threshold.
- Enqueue decision includes next-state booleans and optional command.

## `src/transform/reflection/draft.rs`

Role:
- Produces draft reflection text and merges buffered reflection output.

What is implemented:
- Reflection draft flattens non-empty lines, normalizes whitespace, clamps to char budget.
- Merge function replaces reflected prefix and keeps suffix.

## `src/transform/reflection/guidance.rs`

Role:
- Compression guidance text and compression validation.

What is implemented:
- Guidance levels 0/1/2.
- Compression valid only if reflected tokens are strictly less than target.

## `src/transform/reflection/slice.rs`

Role:
- Plans partial slice for buffered reflection.

What is implemented:
- Derives average tokens per line.
- Calculates lines to reflect from activation point.
- Returns slice text, line count, estimated slice tokens, compression target.

Confirmed risk:
- Uses average tokens-per-line heuristics; compression target can still diverge from actual model tokenization.
