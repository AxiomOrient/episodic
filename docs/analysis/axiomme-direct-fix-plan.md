# AxiomMe OM Direct-Fix Plan (No Migration)

Date: 2026-02-17
Status: Proposed and re-reviewed

## Decision

Best method is **direct behavioral fixes inside `AxiomMe` `crate::om`**, without migration/replacement.

Scope:
- Keep existing module boundary and runtime wiring.
- Fix only proven behavioral drifts.
- Lock behavior with tests.

## Why This Is Best

Observed facts:
- `AxiomMe` is currently integrated around `crate::om` and documents no external `episodic` dependency.
  - Evidence: `/Users/axient/repository/AxiomMe/docs/README.md:15`
  - Evidence: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/lib.rs:22`
- Current drift exists between `episodic` and `AxiomMe` OM behavior in parser/prompt/reflection paths.
  - Evidence: `/Users/axient/repository/episodic/src/parse/mod.rs:279`
  - Evidence: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/parse/mod.rs:286`
  - Evidence: `/Users/axient/repository/episodic/src/transform/reflection/slice.rs:9`
  - Evidence: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/transform/reflection/slice.rs:10`
  - Evidence: `/Users/axient/repository/episodic/src/prompt/formatter.rs:37`
  - Evidence: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/prompt/formatter.rs:24`
- `AxiomMe` has local runtime-specific OM logic (`failure`, `rollout`) that is not in `episodic`.
  - Evidence: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/mod.rs:27`
  - Evidence: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/mod.rs:50`

Tradeoff summary:
- Full replacement now: high integration risk and broad blast radius.
- Migration/facade now: also architecture-level change and policy/doc updates first.
- Direct-fix now: smallest change set, fastest correctness gain, preserves explicit control.

## Execution Plan

1. Freeze architecture and dependency boundaries.
2. Patch four drift points only.
3. Add/adjust tests for each patched behavior.
4. Run full local verification gates.
5. Record results and any residual risk.

## Task List

1. Parser metadata selection order fix
- File: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/parse/mod.rs`
- Change: select newest non-empty metadata for primary/fallback scans.
- Done when: duplicate metadata chooses latest values deterministically.

2. Parser regression test for duplicate metadata ordering
- File: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/parse/tests.rs`
- Change: add case that fails on old-first behavior.
- Done when: test passes and guards ordering semantics.

3. Reflection slice line handling fix
- File: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/transform/reflection/slice.rs`
- Change: switch `split('\n')` to `lines()` to avoid trailing-empty skew.
- Done when: reflected line count is stable with trailing newline inputs.

4. Reflection slice regression test
- File: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/transform/tests/reflection.rs`
- Change: add trailing-newline scenario.
- Done when: test fails pre-fix and passes post-fix.

5. Observer prompt ID rendering fix
- File: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/prompt/formatter.rs`
- Change: include `[id:...]` suffix for non-empty IDs.
- Done when: formatted prompt line contains role/timestamp/id/text contract.

6. Prompt formatter tests update
- File: `/Users/axient/repository/AxiomMe/crates/axiomme-core/src/om/prompt/tests.rs`
- Change: expected strings include ID behavior.
- Done when: timestamp/unknown-role cases validate ID inclusion logic.

7. Parity fixture alignment (`max_tokens_per_batch`)
- File: `/Users/axient/repository/AxiomMe/crates/axiomme-core/tests/om_parity_fixtures.rs`
- File: `/Users/axient/repository/AxiomMe/crates/axiomme-core/tests/fixtures/parity_cases.json`
- Change: verify override path explicitly instead of assuming default only.
- Done when: override case exists and assertion checks resolved override.

8. Verification gate
- Command: `cargo fmt`
- Command: `cargo test -p axiomme-core`
- Command: `bash /Users/axient/repository/AxiomMe/scripts/quality_gates.sh`
- Done when: all commands pass without new warning regressions.

## Re-Review of This Plan

Checklist:
- Data-first: yes, behavior expressed as explicit parser/transform/prompt rules.
- Simplicity: yes, no new layers, no new dependency edges.
- Pure transformations: yes, changes concentrated in pure OM core functions.
- Explicit control: yes, deterministic selection/order rules are codified in tests.
- Performance: yes, no asymptotic cost increase; low-risk local edits.
- Blast radius: low, file-scoped patches and contract tests.

Final verdict:
- This remains the best immediate path for correctness and control.
- Revisit migration/replacement only after direct-fix parity is green and stable.
