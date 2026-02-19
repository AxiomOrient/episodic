# `src/transform/reflection` README

reflection 경로는 "언제 반사할지"와 "어떤 관측 범위를 반사할지"를 계산합니다.

## File Ownership (MECE)

1. `src/transform/reflection/decision.rs`
- `ReflectionAction` 선택 (`None | Buffer | Reflect`)
- enqueue용 `ReflectionEnqueueDecision` 계산

2. `src/transform/reflection/draft.rs`
- reflection draft 생성
- buffered reflection을 기존 observation과 병합

3. `src/transform/reflection/slice.rs`
- buffer activation 기반 반사 대상 slice 계획 계산

4. `src/transform/reflection/guidance.rs`
- 압축 가이던스 텍스트 레벨별 제공
- 압축 검증(`reflected_tokens < target`)

5. `src/transform/reflection/mod.rs`
- 기능 재수출

## Key Rules

1. trigger 규칙
- `should_trigger_reflector`: `observation_tokens > threshold` (strict greater-than)

2. async 경로 규칙
- buffer 활성화 시 threshold 전에는 activation point에서 `Buffer`
- threshold 도달 후 buffered가 있거나 block_after 초과면 `Reflect`

3. draft 규칙
- 공백 정규화 후 라인 기준으로 처리
- 완전하게 포함된 라인 수만 `reflected_observation_line_count`로 계산
- 완전한 1개 라인도 못 담으면 `None`

4. merge 규칙
- `reflected_line_count`만큼 prefix를 대체하고 suffix 보존

## Test-backed Guarantees

`src/transform/tests/reflection.rs`에서 아래를 검증합니다.

1. strict greater-than trigger
2. async/sync action 분기
3. truncated draft의 line-count 의미 보존
4. no-full-line 케이스의 `None` 반환
