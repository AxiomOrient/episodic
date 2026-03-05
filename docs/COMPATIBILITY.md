# Compatibility Matrix (episodic)

Status: Active  
Last Updated: 2026-03-05

## 1. Policy
- Crate semver and protocol version are tracked separately.
- Breaking protocol changes require:
  - semver major/minor bump according to change scope
  - explicit matrix update in this file
  - conformance fixture refresh

## 2. Matrix
| episodic crate | Protocol version | Contract baseline | Compatibility status |
|---|---|---|---|
| `0.1.x` | `om-v2` | prompt contract `2.0.0` | legacy OMv2 subset |
| `0.2.0` | `om-v2` | prompt contract `2.0.0` + OMv2 full protocol set | maintained |
| `0.2.1` | `om-v2` | prompt contract `2.0.0` + OMv2 full protocol set | maintained |
| `0.2.2` | `om-v2` | prompt contract `2.0.0` + OMv2 full protocol set | active |

## 3. Compatibility Guarantees
- Within a patch release (`0.1.a -> 0.1.b`):
  - no breaking serde field removal/rename in public protocol structs
  - deterministic behavior changes require fixture evidence
- For `0.1.x -> 0.2.0`:
  - breaking additions/renames are allowed only if documented in release notes and checklist

## 4. Consumer Impact Rules
- If a symbol is renamed/removed:
  - add mapping guidance in release notes
  - provide fixture-based example for migration
- If deterministic ordering changes:
  - include before/after fixture diff and reason

## 5. Required Evidence for Compatibility Updates
- Updated matrix row
- Updated `tests/fixtures/contracts/*`
- Passing `cargo test`
- Passing `cargo audit -q`
