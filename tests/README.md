# `tests` README

`tests/`는 모듈 단위 unit test와 별개로 "크로스모듈 계약"을 검증합니다.

## File Ownership (MECE)

1. `tests/parity_fixtures.rs`
- `tests/fixtures/parity_cases.json` 기반 정합성 테스트
- config/observer/reflection/activation/pipeline 결과가 fixture 기대값과 일치하는지 검증
- 동일 입력에서 동일 출력(결정성) 검증 포함

2. `tests/runtime_behavior_validation.rs`
- 현실형 입력(노이즈/깨진 태그/멀티스레드)에서 런타임 동작 검증
- parser accuracy-first, plan 계산, observer 합성 등 end-to-end 경로 확인

## Why This Matters

1. unit test가 놓치기 쉬운 모듈 간 연결 계약을 고정
2. 리팩터링 시 행동 회귀를 조기에 감지
3. "pure transform + explicit contract" 설계가 실제 런타임에서 유지되는지 확인
