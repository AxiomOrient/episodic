# `src/prompt` README

`prompt`는 observer/reflector 프롬프트를 결정적으로 생성합니다.

## File Ownership (MECE)

1. `src/prompt/mod.rs`
- 입력 DTO 정의
- 하위 빌더 재수출

2. `src/prompt/system.rs`
- 시스템 프롬프트 템플릿
- 출력 XML 계약 및 행동 원칙 고정

3. `src/prompt/user.rs`
- 런타임 입력값을 결합해 user prompt 구성
- 옵션(`skip_continuation_hints`) 분기

4. `src/prompt/formatter.rs`
- 메시지 히스토리 포맷팅
- role 정규화, timestamp 렌더링, message id 안전화

## Key Contracts

1. observer 단일 스레드 모드
- `<thread>` 태그를 직접 쓰지 않도록 명시

2. observer 멀티 스레드 모드
- `<thread id="...">` 블록 기반 입력/출력 규약 명시

3. formatter 안전성
- `message.id`는 `normalize_message_id_for_prompt`로 정규화
- 비정상 문자/개행으로 프롬프트 구조가 깨지지 않음
- 정규화 결과가 비면 id suffix 생략

## Test-backed Guarantees

`src/prompt/tests.rs`에서 아래를 검증합니다.

1. 필수 섹션 포함
2. 시스템 프롬프트 결정성
3. XML 민감 문자 escaping
4. message id 정규화/생략 규칙
