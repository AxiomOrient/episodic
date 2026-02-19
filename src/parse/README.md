# `src/parse` README

`parse`는 모델 출력 문자열을 구조화된 메모리 섹션으로 변환합니다.

## File Ownership (MECE)

1. `src/parse/tokens.rs`
- XML 유사 태그 토크나이저
- `TagToken`/`TagSectionRange`와 line-anchored 정보 생성

2. `src/parse/sections.rs`
- 태그 섹션 추출/삭제
- Strict/Lenient 모드별 중첩/오버랩 처리

3. `src/parse/thread.rs`
- `<thread id="...">` 블록 추출
- thread 내부 `current-task`/`suggested-response` 분리

4. `src/parse/mod.rs`
- 퍼블릭 API
- accuracy-first 중재 로직
- multi-thread 집계(`aggregate_multi_thread_observer_sections`)

## Parse Modes

1. `OmParseMode::Strict`
- 모호한 중첩/복구를 최소화
- malformed overlap을 되도록 거부

2. `OmParseMode::Lenient`
- 모델 출력의 흔한 깨짐을 복구
- strict가 신호를 못 찾는 경우 백업 경로

3. accuracy-first API
- `parse_memory_section_xml_accuracy_first`
- `parse_multi_thread_observer_output_accuracy_first`
- 규칙: strict에 유효 관측 신호가 있으면 strict 우선, 없을 때만 lenient

## Key Contracts

1. 태그 인식은 line-anchored 우선
2. metadata(`<current-task>`, `<suggested-response>`)는 마지막 유효 블록 우선
3. multi-thread aggregate는 최신(primary 우선, 없으면 최신 fallback) 메타 선택
4. 집계 시 thread id와 observations는 XML escape 적용

## Test-backed Guarantees

`src/parse/tests.rs`에서 아래를 검증합니다.

1. strict/lenient 경계 동작
2. inline literal과 실제 태그 구분
3. malformed 입력 복구/거부 일관성
4. accuracy-first 선택 정책
