# Prompt Module Analysis (`src/prompt/`)

## `/Users/axient/repository/episodic/src/prompt/mod.rs`
- Role: Prompt input DTO definitions and export map.
- Data contracts: `OmObserverPromptInput`, `OmReflectorPromptInput`, `OmObserverThreadMessages`.
- Control flow: None.
- Purity: Pure data/export layer.
- Performance: None.
- Risks: None beyond downstream formatter/prompt policy behavior.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/prompt/mod.rs:6`, `/Users/axient/repository/episodic/src/prompt/mod.rs:29`.

## `/Users/axient/repository/episodic/src/prompt/system.rs`
- Role: Long-form system prompt templates for observer and reflector behaviors.
- Data contracts: Embedded XML output contract and behavioral directives.
- Control flow: Static template concatenation.
- Purity: Pure string generation.
- Performance: Deterministic string formatting cost.
- Risks: Behavioral guarantees depend on model compliance with prompt instructions.
- Verdict: `WARN` (policy text cannot enforce runtime correctness alone).
- Evidence: `/Users/axient/repository/episodic/src/prompt/system.rs:56`, `/Users/axient/repository/episodic/src/prompt/system.rs:127`.

## `/Users/axient/repository/episodic/src/prompt/user.rs`
- Role: User prompt assembly for observer/reflector modes.
- Data contracts: Sections for previous observations, new history, optional context, optional request JSON.
- Control flow: Conditional section inclusion and mode-specific hint suppression.
- Purity: Pure string assembly.
- Performance: Linear in input string sizes.
- Risks: Prompt grows with history/context; caller must bound input sizes.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/prompt/user.rs:10`, `/Users/axient/repository/episodic/src/prompt/user.rs:103`.

## `/Users/axient/repository/episodic/src/prompt/formatter.rs`
- Role: Message-to-prompt formatter and multi-thread XML message wrapper.
- Data contracts: message blocks now include role, optional timestamp, and explicit `[id:...]` suffix.
- Control flow: role normalization, timestamp parse, per-message formatting, XML escaping for thread blocks.
- Purity: Pure transform.
- Performance: O(n) over messages and text lengths.
- Risks: Timestamp parsing failure silently omits timestamp (intentional graceful behavior).
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/prompt/formatter.rs:23`, `/Users/axient/repository/episodic/src/prompt/formatter.rs:37`.

## `/Users/axient/repository/episodic/src/prompt/tests.rs`
- Role: Prompt contract regression tests.
- Data contracts validated:
  - required section presence,
  - thread XML escaping,
  - continuation hint suppression,
  - deterministic system prompt generation,
  - formatter behavior including id suffix output.
- Control flow: deterministic assertion suite.
- Purity: Pure tests.
- Performance: Trivial.
- Risks: Does not evaluate prompt quality against model outputs, only static string contracts.
- Verdict: `OK`.
- Evidence: `/Users/axient/repository/episodic/src/prompt/tests.rs:5`, `/Users/axient/repository/episodic/src/prompt/tests.rs:130`.
