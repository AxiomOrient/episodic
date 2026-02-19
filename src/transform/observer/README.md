# `src/transform/observer` README

observer 경로는 "무엇을 관측할지"를 결정하고 관측 텍스트를 합성합니다.

## File Ownership (MECE)

1. `src/transform/observer/candidates.rs`
- 후보 필터링/정렬/분리
- `last_observed_at`, `observed_message_ids`, session 분리 처리

2. `src/transform/observer/context.rs`
- 타 스레드 컨텍스트 블록 생성
- buffered 표시 결합(`BUFFERED_OBSERVATIONS_SEPARATOR`)

3. `src/transform/observer/decision.rs`
- observer 실행 여부 결정
- interval boundary + debounce + threshold/block_after 조합 계산

4. `src/transform/observer/synthesis.rs`
- pending 메시지에서 관측 라인 합성
- 중복 제거 + forward progress fallback

5. `src/transform/observer/mod.rs`
- 위 기능 재수출

## Decision Contract

`ObserverWriteDecision`은 아래를 분리해서 반환합니다.

1. `threshold_reached`
2. `interval_triggered`
3. `block_after_exceeded`
4. `should_run_observer`
5. `should_activate_after_observer`

호스트는 이 값을 그대로 실행 계획에 반영하면 됩니다.

## Test-backed Guarantees

`src/transform/tests/observer.rs`에서 아래를 검증합니다.

1. 후보 정렬의 결정성(동률 정렬 포함)
2. session 정규화(trim) 기반 pending/other 분리
3. XML escaping 보장
4. interval crossing + debounce 정책
5. continuation hint skip 조건
