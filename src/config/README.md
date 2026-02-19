# `src/config` README

`config`는 "입력 → 검증 → 해석" 3단계를 담당합니다.

## File Ownership (MECE)

1. `src/config/input.rs`
- 외부 입력 타입 정의 (`OmConfigInput`, `ObservationConfigInput`, `ReflectionConfigInput`, `BufferTokensInput`)
- 검증/기본값 계산 없음

2. `src/config/validate.rs`
- 스칼라 값 검증과 에러 타입(`OmConfigError`)
- 비즈니스 조합 해석 없음

3. `src/config/resolve.rs`
- 기본값 적용 + 조합 규칙 해석 + 최종 `Resolved*` 생성
- 핵심 규칙:
  - Resource scope 기본 async buffering 비활성화
  - `share_token_budget=true`일 때 async buffering 강제 비활성화
  - `buffer_tokens < message_tokens_base`
  - `block_after >= threshold`

4. `src/config/mod.rs`
- 상수/재수출 집합

## Key Contracts

1. `ResolvedObservationConfig.dynamic_threshold()`
- shared budget가 있으면 남은 예산 기반 임계치 계산

2. `async_buffering_disabled`
- observer/reflection의 buffer 관련 값이 모두 `None`으로 정규화됨

## Test-backed Guarantees

`src/config/tests.rs`에서 아래를 검증합니다.

1. 기본값 안정성
2. 잘못된 입력 형태 거부
3. scope 정책(Resource/ShareTokenBudget) 강제
4. ratio/absolute/disabled의 직렬화 및 해석 일관성
