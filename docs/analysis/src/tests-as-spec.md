# Tests as Executable Specification

## Purpose
This document summarizes how `src/**/tests.rs` files define behavioral contracts for the implementation.

## Contract Coverage Map

### Core root tests
- `model/tests.rs`: serialization stability, enum string contracts, default-field backward compatibility, and explicit record invariant violation checks.
- `inference/tests.rs`: DTO roundtrip integrity and optional field semantics.
- `context/tests.rs`: bounded hint edge conditions.
- `pipeline/tests.rs`: planner decisions for read-only and initial/non-initial steps.
- `addon/tests.rs`: action-command mapping and default async delegation behavior.

### Config tests
- `config/tests.rs`: default thresholds, invalid input rejection, scope policy constraints, ratio/absolute conversion boundaries.

### Parse tests
- `parse/tests.rs`: strict vs lenient recovery behavior, anchored tag semantics, metadata extraction order, malformed input resilience.

### Prompt tests
- `prompt/tests.rs`: required prompt sections, deterministic system prompt output, escaping correctness, formatter role/timestamp/id rendering.

### Transform tests
- `transform/tests/scope.rs`: scope key required-field contract.
- `transform/tests/activation.rs`: activation boundary math and record mutation invariants.
- `transform/tests/observer.rs`: candidate selection determinism, interval/debounce semantics, context block escaping and partitioning.
- `transform/tests/reflection.rs`: reflection action state machine, compression validation, slice math, enqueue state transitions.

## Key Invariants Enforced by Tests
1. Deterministic sorting and selection in observer candidate flow.
2. Strict parser does not silently recover malformed overlaps; lenient parser may recover.
3. Reflection trigger is strictly greater than threshold (`>`), not greater-or-equal.
4. Planner functions remain pure and decision-only (no side effects).
5. XML-sensitive content is escaped in prompt and parse aggregation paths.
6. Formatter output now includes message IDs (`[id:...]`) to align observed ID constraints.
7. `OmRecord` scope identifier and `scope_key` consistency can be validated via typed invariant violations.

## Gaps to Watch
- Tests are scenario-based, not property-based; malformed input space is still larger than enumerated cases.
- State-space explosion for boolean flags in `OmRecord` is reduced by invariant checks, but not fully covered combinatorially.
