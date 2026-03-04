# Episodic Source Analysis (Non-Test)

This folder contains a file-by-file review of the non-test code in this repository.

Scope rules used for this pass:
- Included: root project files and `src/**` non-test files.
- Excluded: all `tests/**` and all `src/**/tests.rs`.
- Style target: data-first, explicit control, pure transformations, minimal decoration.

## Latest Full Validation Pass (2026-02-17)
- full repository was re-reviewed and re-validated, including test files and parity fixtures
- gates executed: `cargo test`, `cargo fmt --check`, `cargo clippy -D warnings`, `cargo check --release`, `cargo test --release`, `cargo audit`, `cargo package --allow-dirty`
- result: no blocking defects found
- release note source: `/Users/axient/repository/episodic/RELEASE_REVIEW.md`

## Documents

- `docs/analysis/root.md`
- `docs/analysis/lib.md`
- `docs/analysis/model.md`
- `docs/analysis/inference.md`
- `docs/analysis/context.md`
- `docs/analysis/xml.md`
- `docs/analysis/addon.md`
- `docs/analysis/config.md`
- `docs/analysis/parse.md`
- `docs/analysis/prompt.md`
- `docs/analysis/transform.md`
- `docs/analysis/pipeline.md`
- `docs/analysis/cross-validation.md`
- `docs/analysis/doc-consistency-review.md` (duplication/consistency check)
- `docs/analysis/axiomme-direct-fix-plan.md` (AxiomMe OM direct-fix plan and re-review)
- `docs/analysis/axiomme-episodic-boundary-plan.md` (dual-use boundary-preserving design plan)
- `docs/analysis/src/README.md` (full `src/` coverage including tests)

## Maintenance Rule
- Detailed source of truth: `docs/analysis/src/*`
- Top-level `docs/analysis/*` files are summary-layer documents.
