# Src Analysis Index

## Objective
- Analyze every file under `src` folder, folder-by-folder and file-by-file.
- Apply data-first, explicit-control, pure-transformation criteria.
- Keep evidence grounded with path and line references.

## Execution Checklist
- [x] Inventory `src` files
- [x] Analyze core root files
- [x] Analyze `config/` files
- [x] Analyze `parse/` files
- [x] Analyze `prompt/` files
- [x] Analyze `transform/` files
- [x] Cross-validate against previous analysis docs
- [x] Final coverage verification

## Module Documents
- `docs/analysis/src/core.md`
- `docs/analysis/src/config.md`
- `docs/analysis/src/parse.md`
- `docs/analysis/src/prompt.md`
- `docs/analysis/src/transform.md`
- `docs/analysis/src/tests-as-spec.md`
- `docs/analysis/src/cross-validation.md`
- `docs/analysis/src/next-step-validity-review.md`

## Source File Inventory (47)

### Root (`src/`)
1. `/Users/axient/repository/episodic/src/lib.rs`
2. `/Users/axient/repository/episodic/src/model.rs`
3. `/Users/axient/repository/episodic/src/model/tests.rs`
4. `/Users/axient/repository/episodic/src/inference.rs`
5. `/Users/axient/repository/episodic/src/inference/tests.rs`
6. `/Users/axient/repository/episodic/src/context.rs`
7. `/Users/axient/repository/episodic/src/context/tests.rs`
8. `/Users/axient/repository/episodic/src/pipeline.rs`
9. `/Users/axient/repository/episodic/src/pipeline/tests.rs`
10. `/Users/axient/repository/episodic/src/addon.rs`
11. `/Users/axient/repository/episodic/src/addon/tests.rs`
12. `/Users/axient/repository/episodic/src/xml.rs`

### Config (`src/config/`)
13. `/Users/axient/repository/episodic/src/config/mod.rs`
14. `/Users/axient/repository/episodic/src/config/input.rs`
15. `/Users/axient/repository/episodic/src/config/validate.rs`
16. `/Users/axient/repository/episodic/src/config/resolve.rs`
17. `/Users/axient/repository/episodic/src/config/tests.rs`

### Parse (`src/parse/`)
18. `/Users/axient/repository/episodic/src/parse/mod.rs`
19. `/Users/axient/repository/episodic/src/parse/tokens.rs`
20. `/Users/axient/repository/episodic/src/parse/sections.rs`
21. `/Users/axient/repository/episodic/src/parse/thread.rs`
22. `/Users/axient/repository/episodic/src/parse/tests.rs`

### Prompt (`src/prompt/`)
23. `/Users/axient/repository/episodic/src/prompt/mod.rs`
24. `/Users/axient/repository/episodic/src/prompt/system.rs`
25. `/Users/axient/repository/episodic/src/prompt/user.rs`
26. `/Users/axient/repository/episodic/src/prompt/formatter.rs`
27. `/Users/axient/repository/episodic/src/prompt/tests.rs`

### Transform (`src/transform/`)
28. `/Users/axient/repository/episodic/src/transform/mod.rs`
29. `/Users/axient/repository/episodic/src/transform/types.rs`
30. `/Users/axient/repository/episodic/src/transform/helpers.rs`
31. `/Users/axient/repository/episodic/src/transform/scope.rs`
32. `/Users/axient/repository/episodic/src/transform/activation.rs`
33. `/Users/axient/repository/episodic/src/transform/observer/mod.rs`
34. `/Users/axient/repository/episodic/src/transform/observer/candidates.rs`
35. `/Users/axient/repository/episodic/src/transform/observer/context.rs`
36. `/Users/axient/repository/episodic/src/transform/observer/decision.rs`
37. `/Users/axient/repository/episodic/src/transform/observer/synthesis.rs`
38. `/Users/axient/repository/episodic/src/transform/reflection/mod.rs`
39. `/Users/axient/repository/episodic/src/transform/reflection/decision.rs`
40. `/Users/axient/repository/episodic/src/transform/reflection/draft.rs`
41. `/Users/axient/repository/episodic/src/transform/reflection/guidance.rs`
42. `/Users/axient/repository/episodic/src/transform/reflection/slice.rs`
43. `/Users/axient/repository/episodic/src/transform/tests/mod.rs`
44. `/Users/axient/repository/episodic/src/transform/tests/scope.rs`
45. `/Users/axient/repository/episodic/src/transform/tests/activation.rs`
46. `/Users/axient/repository/episodic/src/transform/tests/observer.rs`
47. `/Users/axient/repository/episodic/src/transform/tests/reflection.rs`
