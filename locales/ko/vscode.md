# VS Code용 Drizzle 확장

Drizzle은 VS Code에서 `.zz`, `.drizzle`, `.rd` 파일의 구문을 강조합니다.

![Drizzle](../../images/logo_drizzle_language.png)

## 데모

![데모](../../images/drizzle_demo.png)

## 열차 제어 예제

```drizzle
&& Train control requirements
Estimate @0,0 = /Accurate estimate\nof speed & position/
Tracking @1,0 = <Tracking system>
Train @2,0 = [Train]
Duty @0,1 = {Train controller responsibility\nCompute safe acceleration for all trains between this station and the next.}
Safe @0,2 = /Safe command\nmessage/
Controller @1,2 = <Train controller>
Timing @0,3 = /Command in time/
Send @1,3 = (Send\ncommand)
Command @2,3 = [Command]

Estimate __ Tracking [o]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train __> Controller : As monitor\nmeasuredSpeed\nmeasuredLocation
Controller __> Command : As control\ncommandedSpeed\ncommandedAccel
Safe __ Controller [o]
Timing __ Controller [o] : Responsibility
Controller __ Send [o] : Performance
Duty --> Controller
```

## 컴파일 결과

![컴파일 결과](../../output.png)

도형, 연결 표식 및 레이아웃 옵션은 [Drizzle 문법 안내서](./README.md#drizzle-syntax)를 참조하세요.

## 확장 개발과 테스트

VS Code에서 `drizzle/` 폴더를 엽니다. 매니페스트 `package.json`은 확장과 문법 파일의 위치를 정의합니다. `syntaxes/drizzle.tmLanguage.json`은 TextMate 문법을, `language-configuration.json`은 주석과 괄호 설정을 담습니다.

**F5**를 눌러 Extension Development Host를 엽니다. 호스트에서 예제 파일을 열고 구문 강조와 언어 설정을 확인합니다. 파일을 변경한 뒤 호스트를 다시 시작하거나 창을 다시 로드합니다.

Marketplace에서 확장을 설치하거나 VS Code의 **Extensions: Install from VSIX...**로 VSIX 패키지를 설치합니다. 개발과 게시 방법은 [VS Code 확장 문서](https://code.visualstudio.com/api)를 참조하세요.

## 확장 릴리스 노트

### 1.1.3

- Marketplace에서 로고, 구문 데모 및 렌더링 결과의 이미지 URL을 수정했습니다.
- 저장소 README의 전체 Drizzle 안내서로 바로 연결했습니다.

### 1.1.0 – 1.1.2

- Pest 문법에 이름이 있는 노드, 전방 참조, UTF-8 레이블, 이스케이프, 줄바꿈 및 줄과 열을 표시하는 진단을 추가했습니다. 이름 없는 목표, 위험, 에이전트 및 줄 전체 주석을 유지했습니다.
- 객체, 동작, 메모, 구름 주석, 서술용 사다리꼴과 마름모를 추가했습니다. 요구사항 및 가정 같은 잎 노드는 굵은 윤곽선을 사용할 수 있습니다.
- 실선과 파선, 선택 가능한 화살촉, 레이블, 루프 및 공유 분기점을 추가했습니다. 빈 원과 채운 원, 충돌 지그재그 및 장애물 막대를 지원합니다.
- 네 방향의 레이아웃, 명시적 열과 행 배치, 직각 경로 및 외곽 우회를 추가했습니다. 평행 연결을 분리하고 화살표를 도형 경계에 연결합니다.
- VS Code 문법과 확장 README를 갱신하고 GIF 데모를 PNG로 바꿨으며 Drizzle 1.1.0 VSIX를 포함했습니다.

### 1.0.0

Drizzle 첫 릴리스입니다.
