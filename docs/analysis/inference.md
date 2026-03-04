# Inference Contracts Review

Reviewed file:
- `src/inference.rs`

## `src/inference.rs`

Role:
- Defines request/response contracts exchanged with observer/reflector model adapters.

What is implemented:
- `OmInferenceModelConfig`: provider/model/output-token/temperature fields.
- `OmInferenceUsage`: input/output token accounting.
- `OmPendingMessage`: id/role/text + optional RFC3339 timestamp.
- `OmObserverRequest` and `OmObserverResponse`.
- `OmReflectorRequest` and `OmReflectorResponse`.

Data-first notes:
- DTOs are plain and explicit with no hidden behavior.
- Optional fields use deterministic serialization defaults.

Simplicity notes:
- Contracts are direct and composable.
- No dynamic dispatch or transport assumptions inside model contracts.

Performance notes:
- No algorithmic work here, only struct definitions.

Risks:
- `role` is free-form string; invalid role values are not normalized at contract level.
- Timestamp is optional and stringly-typed; strict temporal invariants are delegated to callers.

