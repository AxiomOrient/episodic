# `src/transform` README

`transform`은 상태 변경 의사결정을 "순수 계산 결과"로 분리합니다.

## File Ownership (MECE)

1. `src/transform/types.rs`
- 변환 레이어 공통 데이터 타입

2. `src/transform/helpers.rs`
- 토큰 추정/공백 정규화/메시지 id 병합 유틸

3. `src/transform/scope.rs`
- scope별 `scope_key` 생성

4. `src/transform/activation.rs`
- buffered chunk 활성화 경계 계산/병합

5. `src/transform/observer/*`
- observer 후보 선택, 컨텍스트 조립, 트리거 결정, 관측 합성
- 상세: `/Users/axient/repository/episodic/src/transform/observer/README.md`

6. `src/transform/reflection/*`
- reflection 트리거 결정, draft/slice 생성, 병합
- 상세: `/Users/axient/repository/episodic/src/transform/reflection/README.md`

7. `src/transform/mod.rs`
- public API 재수출

8. `src/transform/tests/*`
- transform 계약 검증 스위트
- 상세: `/Users/axient/repository/episodic/src/transform/tests/README.md`

## Design Rules

1. IO 없음
- 입력 인자와 반환값만으로 결정

2. Saturating 연산 우선
- 토큰 합산에서 overflow 안전성 유지

3. Flag 전이 분리
- enqueue decision에서 다음 플래그 값을 명시적으로 반환

## Deferred Area

`OmRecord`의 플래그 조합 하드 불변식 확장은 공개 API 영향으로 보류.

- 참조: `/Users/axient/repository/episodic/DECISIONS.md`
