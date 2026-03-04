# Documentation Consistency Review

## Scope
- Compared documents in:
  - `/Users/axient/repository/episodic/docs/analysis/*.md`
  - `/Users/axient/repository/episodic/docs/analysis/src/*.md`
- Goal: detect duplication and contradictory statements.

## Duplication Map

The following topics are intentionally duplicated at different granularity:

1. `config`:
- Summary: `/Users/axient/repository/episodic/docs/analysis/config.md`
- Full `src` pass: `/Users/axient/repository/episodic/docs/analysis/src/config.md`

2. `parse`:
- Summary: `/Users/axient/repository/episodic/docs/analysis/parse.md`
- Full `src` pass: `/Users/axient/repository/episodic/docs/analysis/src/parse.md`

3. `prompt`:
- Summary: `/Users/axient/repository/episodic/docs/analysis/prompt.md`
- Full `src` pass: `/Users/axient/repository/episodic/docs/analysis/src/prompt.md`

4. `transform`:
- Summary: `/Users/axient/repository/episodic/docs/analysis/transform.md`
- Full `src` pass: `/Users/axient/repository/episodic/docs/analysis/src/transform.md`

5. Cross-validation:
- Summary pass: `/Users/axient/repository/episodic/docs/analysis/cross-validation.md`
- Full `src` pass: `/Users/axient/repository/episodic/docs/analysis/src/cross-validation.md`

## Consistency Findings

1. Fixed: stale evidence text in summary cross-validation doc.
- Updated `/Users/axient/repository/episodic/docs/analysis/cross-validation.md` to mark old behavior as historical evidence.

2. Fixed: test-depth issue status mismatch for parity max batch tokens.
- Updated both cross-validation docs to reflect resolution.

## Source-of-Truth Rule

To prevent drift:
- Detailed and current source of truth: `/Users/axient/repository/episodic/docs/analysis/src/*`
- Top-level `/Users/axient/repository/episodic/docs/analysis/*` should be treated as summaries.

## Recommendation

- Keep both layers, but only update issue status in one place first:
  1. update `/docs/analysis/src/*`,
  2. then mirror concise status in `/docs/analysis/*`.

