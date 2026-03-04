# episodic

Reusable Observational Memory (OM) core crate for agentic automation.

## What this crate does
- models OM state explicitly (`OmRecord`, `OmObservationChunk`)
- defines inference contracts (`OmObserverRequest/Response`, `OmReflectorRequest/Response`)
- provides deterministic pure transforms for activation, observer writes, and reflection enqueues
- builds deterministic pipeline plans (`plan_process_input_step`, `plan_process_output_result`)
- parses model output with explicit recovery modes (`OmParseMode::Strict`, `OmParseMode::Lenient`)
- exposes strict-first accuracy entrypoints (`parse_*_accuracy_first`)
- keeps runtime integration behind addon ports (`OmApplyAddon`, `OmObserverAddon`, `OmReflectorAddon`)

## Non-goals
- storage/DB adapters
- network/model transport
- host runtime wiring

Those remain in host integration layers (for example, AxiomMe bridge code).

## Data-first boundaries
- all major decisions are explicit return values, not hidden side effects
- async/sync behavior is resolved by config and action enums (`ReflectionAction`, `ObserverWriteDecision`)
- XML and prompt formatting paths are deterministic and escaped explicitly
- parser behavior is explicit: strict rejects malformed overlap, lenient recovers when possible

## Key modules
- `src/model.rs`: core memory record and invariant checks
- `src/config/resolve.rs`: config resolution and async-buffering constraints
- `src/parse/mod.rs`: structured parse + strict/lenient arbitration
- `src/transform/*`: pure decision and synthesis transforms
- `src/pipeline.rs`: host-call planning layer (no IO side effects)

## Verification status (2026-03-04)
Validated in this repository with:
- `cargo test`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo check --release`
- `cargo test --release`
- `cargo audit`
- `cargo package --allow-dirty`
- `cargo test --test contract_fixtures`

Result:
- no failing tests
- no clippy warnings
- no RustSec vulnerabilities reported

Non-blocking note:
- package metadata is configured (`repository`/`homepage`/`documentation`) and `cargo package` warning is cleared.

## File-by-file review coverage (2026-03-04)
- root files: `.gitattributes`, `.gitignore`, `Cargo.toml`, `README.md`, `RELEASE_REVIEW.md`
- source files: all `47` Rust files under `src/`
- integration tests: all files under `tests/`
- parity fixture: `tests/fixtures/parity_cases.json`
