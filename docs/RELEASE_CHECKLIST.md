# Release Checklist (episodic)

Status: Active  
Last Updated: 2026-03-04

## Pre-Release
- [x] `docs/PROTOCOL.md` updated for protocol-impacting changes
- [x] `docs/COMPATIBILITY.md` matrix updated
- [x] `docs/TASKS.md` task lifecycle synced (`DOING -> DONE/BLOCKED`)
- [x] `tests/fixtures/contracts/*` refreshed when contracts changed

## Quality Gates
- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --all-targets -- -D warnings`
- [x] `cargo test`
- [x] `cargo audit -q`

## Contract Gates
- [x] protocol constants validated (`OM_PROTOCOL_VERSION`, contract name/version)
- [x] deterministic snapshot/fixture tests green
- [x] new/changed public protocol symbols documented

## Packaging
- [x] `Cargo.toml` version/changelog updated
- [x] release notes include compatibility impact
- [x] `cargo package --allow-dirty` (or clean equivalent) verified

## Publish Decision
- [x] all gates pass
- [x] no unresolved P0/P1 items in release scope
- [x] final sign-off recorded
