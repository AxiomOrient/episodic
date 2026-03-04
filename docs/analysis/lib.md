# Library Entry Review

Reviewed file:
- `src/lib.rs`

## `src/lib.rs`

Role:
- Crate entry module and public API surface.

What is implemented:
- Declares top-level modules: `addon`, `config`, `context`, `inference`, `model`, `parse`, `pipeline`, `prompt`, `transform`, `xml`.
- Re-exports most public types/functions as a flat API.

Data-first and simplicity notes:
- Public contracts are explicit and grouped by concern.
- Consumers can use a stable facade without deep module traversal.

Performance notes:
- No runtime logic in this file.

Risks:
- Wide re-export surface can make semver management harder if contracts evolve quickly.

