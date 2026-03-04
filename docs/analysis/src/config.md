# Config Module Analysis (`src/config/`)

## `/Users/axient/repository/episodic/src/config/mod.rs`
- Role: Config module assembly and defaults.
- Data contracts: default token/activation constants and re-export surface.
- Control flow: None.
- Purity: Pure constants/module wiring.
- Performance: None.
- Risks: Default semantics can drift from host expectations if not versioned.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/config/mod.rs:5`, `/Users/axient/repository/episodic/src/config/mod.rs:13`.

## `/Users/axient/repository/episodic/src/config/input.rs`
- Role: Caller-facing raw config DTOs.
- Data contracts: `BufferTokensInput`, observation/reflection input structs, top-level `OmConfigInput` default.
- Control flow: `Default` sets thread scope and no shared budget.
- Purity: Pure data definitions.
- Performance: None.
- Risks: Many optional knobs require downstream resolution to prevent invalid combinations.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/config/input.rs:7`, `/Users/axient/repository/episodic/src/config/input.rs:37`.

## `/Users/axient/repository/episodic/src/config/validate.rs`
- Role: Scalar validation and conversion helpers with explicit error taxonomy.
- Data contracts: `OmConfigError` fully enumerates invalid states.
- Control flow: validates tokens/activations, resolves ratio/absolute values, computes `block_after` semantics.
- Purity: Pure functions.
- Performance: Constant-time scalar arithmetic.
- Risks: Ratio rounding near zero/one can surprise integrators if undocumented.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/config/validate.rs:6`, `/Users/axient/repository/episodic/src/config/validate.rs:61`.

## `/Users/axient/repository/episodic/src/config/resolve.rs`
- Role: Normalizes raw config into runtime-resolved policy.
- Data contracts: `ResolvedObservationConfig`, `ResolvedReflectionConfig`, `ResolvedOmConfig`.
- Control flow:
  - validates base thresholds,
  - derives async-buffering disablement,
  - enforces scope/shared-budget constraints,
  - resolves buffer/block-after/activation values,
  - computes total shared budget.
- Purity: Pure transform.
- Performance: O(1) scalar/path logic.
- Risks:
  - Policy interaction is dense (`scope`, `share_token_budget`, async buffering), easy to misconfigure without docs.
  - `block_after` dual semantic (ratio vs absolute) relies on float threshold.
- Verdict: `WARN` (policy complexity high but explicit).
- Evidence: `/Users/axient/repository/episodic/src/config/resolve.rs:52`, `/Users/axient/repository/episodic/src/config/resolve.rs:73`.

## `/Users/axient/repository/episodic/src/config/tests.rs`
- Role: Regression suite for config edge cases and policy constraints.
- Data contracts validated:
  - defaults,
  - async disable constraints,
  - resource-scope behavior,
  - invalid activation and block-after,
  - buffer token serialization shapes.
- Control flow: deterministic fixture-style assertions.
- Purity: Pure tests.
- Performance: Trivial.
- Risks: Does not exhaustively enumerate all combinatorial policy states.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/config/tests.rs:7`, `/Users/axient/repository/episodic/src/config/tests.rs:217`.
