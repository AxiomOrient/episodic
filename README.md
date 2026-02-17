# episodic

`episodic` is a pure Observational Memory (OM) core for agentic runtimes.

It gives you explicit data types and deterministic transforms for:
- observing new messages
- buffering/activating observations
- deciding reflection timing
- parsing structured model output

## Design contract
- Data-first: state is explicit (`OmRecord`, `OmObservationChunk`).
- Pure transforms: decisions are return values, not hidden side effects.
- Explicit control: strict vs lenient parse modes are selectable.
- No runtime lock-in: storage/network/model calls are outside this crate.

## What this crate does not do
- no DB/storage adapter
- no network/model transport
- no host workflow orchestration

## Install

```toml
[dependencies]
episodic = "0.1.0"
```

## Quick usage

### 1) Resolve OM config

```rust
use episodic::{
    BufferTokensInput, ObservationConfigInput, OmConfigInput, OmScope, ReflectionConfigInput,
    resolve_om_config,
};

let resolved = resolve_om_config(OmConfigInput {
    scope: OmScope::Thread,
    share_token_budget: false,
    observation: ObservationConfigInput {
        message_tokens: Some(30_000),
        max_tokens_per_batch: Some(10_000),
        buffer_tokens: Some(BufferTokensInput::Ratio(0.2)),
        buffer_activation: Some(0.8),
        block_after: Some(1.2),
    },
    reflection: ReflectionConfigInput {
        observation_tokens: Some(40_000),
        buffer_activation: Some(0.5),
        block_after: Some(1.2),
    },
})
.expect("valid OM config");
```

### 2) Parse observer XML safely

```rust
use episodic::parse_memory_section_xml_accuracy_first;

let parsed = parse_memory_section_xml_accuracy_first(
    "<observations>\n* High: User prefers direct answers\n</observations>\n\
     <current-task>Primary: implement API</current-task>"
);

assert!(parsed.observations.contains("User prefers direct answers"));
```

### 3) Build a bounded runtime hint

```rust
use episodic::build_bounded_observation_hint;

let hint = build_bounded_observation_hint("a\nb\nc", 2, 32);
assert_eq!(hint.as_deref(), Some("om: b c"));
```

## Host integration flow
1. Keep `OmRecord` and buffered `OmObservationChunk` in your own store.
2. Use `plan_process_input_step` to decide observer/reflection actions.
3. Use transform functions (`decide_observer_write_action`, `decide_reflection_enqueue`, `activate_buffered_observations`) to update state deterministically.
4. Call your own observer/reflector backend through addon ports (`OmObserverAddon`, `OmReflectorAddon`, `OmApplyAddon`).

## API surface (starting points)
- model/contracts: `OmRecord`, `OmObservationChunk`, `OmObserverRequest`, `OmReflectorRequest`
- config: `resolve_om_config`
- parser: `parse_memory_section_xml_accuracy_first`, `parse_multi_thread_observer_output_accuracy_first`
- planners: `plan_process_input_step`, `plan_process_output_result`
- transforms: `select_activation_boundary`, `decide_observer_write_action`, `decide_reflection_enqueue`
