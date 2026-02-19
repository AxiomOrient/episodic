# episodic

`episodic`은 에이전트용 관찰 메모리(Observational Memory, OM) 코어 크레이트입니다.  
핵심 목표는 데이터 모델을 명시적으로 유지하고, 의사결정을 순수 함수로 분리하는 것입니다.

## Scope (MECE)

아래 책임은 상호배타(Mutually Exclusive)하며 전체를 포괄(Collectively Exhaustive)합니다.

1. Domain State
- 소유: 영속 메모리 상태와 불변성 검증
- 파일: `src/model.rs`

2. Inference DTO
- 소유: observer/reflector 요청·응답 계약
- 파일: `src/inference.rs`

3. Config Resolution
- 소유: 입력 검증, 기본값, 런타임 설정 해석
- 파일: `src/config/*`

4. Prompt Construction
- 소유: system/user 프롬프트 조립, 메시지 포맷 안전화
- 파일: `src/prompt/*`

5. Parse Engine
- 소유: XML 유사 출력 파싱(Strict/Lenient + accuracy-first 중재)
- 파일: `src/parse/*`

6. Pure Transform Engine
- 소유: 활성화/관측/반사 의사결정 및 텍스트 변환
- 파일: `src/transform/*`

7. Pipeline Planning
- 소유: 입력/출력 단계 실행 계획 계산(부작용 없음)
- 파일: `src/pipeline.rs`

8. Host Ports
- 소유: 런타임 어댑터 경계(적용/관측/반사 트레이트, 커맨드 타입)
- 파일: `src/addon.rs`

9. Utility
- 소유: bounded hint, XML escape
- 파일: `src/context.rs`, `src/xml.rs`

## Non-goals

- 스토리지/DB 어댑터
- 네트워크 전송
- 모델 호출 런타임 오케스트레이션

위 항목은 호스트 통합 레이어 책임입니다.

## Readme Index

- `/Users/axient/repository/episodic/src/README.md`
- `/Users/axient/repository/episodic/src/config/README.md`
- `/Users/axient/repository/episodic/src/parse/README.md`
- `/Users/axient/repository/episodic/src/prompt/README.md`
- `/Users/axient/repository/episodic/src/transform/README.md`
- `/Users/axient/repository/episodic/src/transform/observer/README.md`
- `/Users/axient/repository/episodic/src/transform/reflection/README.md`
- `/Users/axient/repository/episodic/src/transform/tests/README.md`
- `/Users/axient/repository/episodic/tests/README.md`

## Deferred Decisions

- `/Users/axient/repository/episodic/DECISIONS.md`
