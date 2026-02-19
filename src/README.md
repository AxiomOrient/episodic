# `src` Module Map (MECE)

이 문서는 `src` 내부 책임을 중복 없이 분리해 보여줍니다.

## Top-level Files

1. `src/lib.rs`
- 공개 API 재수출(facade)만 담당.

2. `src/model.rs`
- OM 핵심 상태(`OmRecord`, `OmObservationChunk`)와 불변성 검사.

3. `src/inference.rs`
- 모델 입출력 DTO(`OmObserverRequest/Response`, `OmReflectorRequest/Response`).

4. `src/pipeline.rs`
- 런타임 실행 계획 계산(`plan_process_input_step`, `plan_process_output_result`).

5. `src/addon.rs`
- 호스트 통합 포트(trait)와 reflection enqueue 커맨드.

6. `src/context.rs`
- `active_observations`를 bounded runtime hint로 축약.

7. `src/xml.rs`
- XML text/attribute escape 유틸.

## Subdirectories

1. `src/config/`
- 입력 검증, 기본값, resolved config 계산.
- 상세: `/Users/axient/repository/episodic/src/config/README.md`

2. `src/parse/`
- XML 유사 출력 파싱, strict/lenient 복구, accuracy-first 선택.
- 상세: `/Users/axient/repository/episodic/src/parse/README.md`

3. `src/prompt/`
- system/user prompt 생성과 메시지 포맷팅.
- 상세: `/Users/axient/repository/episodic/src/prompt/README.md`

4. `src/transform/`
- 활성화/관측/반사 중심 순수 변환 함수 집합.
- 상세: `/Users/axient/repository/episodic/src/transform/README.md`

## Review Notes

1. 블로킹 결함은 확인되지 않음.
2. `OmRecord`의 상태 플래그 하드 불변식 확장은 공개 API 영향으로 보류 중.
3. 보류 내역은 `/Users/axient/repository/episodic/DECISIONS.md`에서 관리.
