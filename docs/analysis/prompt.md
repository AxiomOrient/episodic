# Prompt Module Review

Reviewed files:
- `src/prompt/mod.rs`
- `src/prompt/system.rs`
- `src/prompt/formatter.rs`
- `src/prompt/user.rs`

## `src/prompt/mod.rs`

Role:
- Prompt input contracts and module exports.

What is implemented:
- `OmObserverPromptInput`
- `OmReflectorPromptInput`
- `OmObserverThreadMessages`
- Re-export of formatter/system/user builders.

Notes:
- Contracts are explicit and immutable by default.

## `src/prompt/system.rs`

Role:
- System prompt templates for observer and reflector modes.

What is implemented:
- Shared extraction instructions for memory quality and temporal anchoring.
- Observer output XML contract with required sections.
- Multi-thread variant requiring per-thread nested structure.
- Reflector variant for consolidation and compression guidance framing.

Simplicity notes:
- Static string templates with deterministic concatenation.

Risk:
- Policy logic is in prompt text; runtime guarantees still depend on model compliance.

## `src/prompt/formatter.rs`

Role:
- Formats pending messages and thread message blocks for prompt injection.

What is implemented:
- Role normalization with fallback to `"Unknown"`.
- RFC3339 timestamp parse and human-readable UTC formatting.
- Multi-thread formatting into escaped XML thread blocks.

Performance notes:
- Straightforward linear formatting; dominated by string allocations.

Risk:
- Observer message formatting does not include message id text, while downstream prompt instructions mention constrained `observed_message_ids`.

## `src/prompt/user.rs`

Role:
- Builds observer/reflector user prompts from runtime inputs.

What is implemented:
- Sections for previous observations, new messages, optional cross-conversation context, optional request JSON, and task instructions.
- Multi-thread variant includes an inline example output contract.
- Reflector variant includes optional manual guidance and compression guidance by level.
- Optional mode to suppress continuation hint sections.

Simplicity notes:
- Pure string composition and explicit section order.

Confirmed risks:
- Contract mismatch: text says `observed_message_ids` must use provided ids, but default formatted history does not render ids directly.

