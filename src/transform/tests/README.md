# `src/transform/tests` README

이 폴더는 `transform` 모듈의 실행 가능한 계약(spec)입니다.

## File Ownership (MECE)

1. `src/transform/tests/activation.rs`
- activation 경계 계산, saturating, merge/activate 동작 검증

2. `src/transform/tests/observer.rs`
- 후보 선택/분리, observer decision, 합성, 컨텍스트 블록 검증

3. `src/transform/tests/reflection.rs`
- reflection trigger/action, draft/slice/merge 정책 검증

4. `src/transform/tests/scope.rs`
- scope key 생성 규칙과 에러 경로 검증

5. `src/transform/tests/mod.rs`
- 공통 fixture helper와 하위 테스트 모듈 wiring
