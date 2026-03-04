# Config Module Review

Reviewed files:
- `src/config/mod.rs`
- `src/config/input.rs`
- `src/config/validate.rs`
- `src/config/resolve.rs`

## `src/config/mod.rs`

Role:
- Module wiring and default constant definitions.

What is implemented:
- Central defaults for observer/reflector thresholds and activation behavior.
- Re-exports for input, resolved config, and errors.

Notes:
- Keeps config contract centralized and explicit.

## `src/config/input.rs`

Role:
- Raw caller input types before validation and normalization.

What is implemented:
- `BufferTokensInput` tagged enum: `Disabled | Absolute(u32) | Ratio(f64)`.
- `ObservationConfigInput`, `ReflectionConfigInput`, `OmConfigInput`.
- Default `OmConfigInput` chooses `OmScope::Thread` and `share_token_budget=false`.

Notes:
- Data-first approach: all knobs are plain option fields.

## `src/config/validate.rs`

Role:
- Validation and conversion helpers.

What is implemented:
- `OmConfigError` with explicit domain errors.
- Numeric validators for message thresholds and activation values.
- `resolve_buffer_tokens` for disabled/absolute/ratio modes.
- `resolve_block_after` accepts ratio range `[1.0, 2.0)` or absolute `>=2.0`.

Performance:
- Constant-time scalar logic only.

Risks:
- Ratio rounding may produce edge behavior near zero and one (intended and validated).

## `src/config/resolve.rs`

Role:
- Produces normalized runtime config used by planner/decision code.

What is implemented:
- Resolved structs:
  - `ResolvedObservationConfig`
  - `ResolvedReflectionConfig`
  - `ResolvedOmConfig`
- `resolve_om_config` pipeline:
  - validates base thresholds,
  - applies async-buffering policy by scope,
  - enforces resource-scope async restriction,
  - enforces shared-budget requires async disabled,
  - computes buffer interval, activation, and block-after values,
  - computes shared total budget if requested.

Data-first and explicit-control notes:
- Every derived value is represented as a concrete field.
- No implicit global state.

Performance notes:
- No dynamic allocation beyond error text and return structs.

Risks:
- Policy coupling is tight: scope + share budget + async buffering have interdependent constraints that must be understood by integrators.

