## 러스트 설치

우선 러스트를 설치해야겠죠. 설치는 `rustup` 이라는
러스트 버전 및 러스트 관련 툴을 관리하는 커맨드라인 도구를 이용할 겁니다.
인터넷이 연결되어 있어야 하니 미리 인터넷 연결을 확인해주세요.

> Note: `rustup` 이외에 다른 방법으로 설치하길 원하신다면
> [공식 페이지 - 러스트 설치](https://www.rust-lang.org/tools/install) 를 참고하시기 바랍니다.

다음은 러스트 컴파일러 최신 stable 버전을 설치하는 내용입니다.
혹여나 이 책을 읽는 시점에, 이 책에서 사용한 버전이 낮아서 걱정되시는
분들을 위해 말씀드리자면, 러스트에는 안전성 보증(stability guarantees)이
적용되어있습니다. 간혹 에러나 경고 메시지가 변경되는 일이 있기에 출력은
버전마다 조금씩 다를 수 있으나, 이 책에 등장하는 모든 예제는
향후 버전에서도 책 내용에서 설명하는 대로 동작할 겁니다.

> ### 커맨드라인 표기
>
> 이번 장을 비롯해 터미널에 명령어를 입력할 일이 많습니다.
> 입력할 명령어와 출력을 구분하실 수 있도록, 명령어에는
> 각 행 앞에 `$` 가 붙습니다. `$` 가 붙지 않은 행은
> 보통 앞선 명령어의 결과를 나타낸다고 보시면 됩니다.
> 예외적으로, `$` 대신 `>` 가 붙은 예제는
> PowerShell 한정 예제입니다.

### `rustup` 설치 - Linux 및 macOS

Linux 나 macOS 사용자는 터미널을 열고 다음 명령어를 입력해주세요:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

최신 stable 버전 러스트를 설치하는 데 사용할 `rustup` 툴을 설치하는
명령어입니다. (설치할때 여러분 비밀번호를 묻는 메세지가 나타날 수 있습니다.)
설치가 완료되면 다음 문장이 나타납니다:

```text
Rust is installed now. Great!
```

링커는 기본으로 설치되나, 러스트 컴파일 시에 링커를
실행할 수 없다는 에러가 나타나면 따로 설치하셔야 합니다.
이 에러는 C 컴파일러를 설치할때 같이 설치되는 링커로 해결되므로
플랫폼에 맞는 C 컴파일러를 찾아서 설치하시기 바랍니다.

딱히 에러가 나타나지 않은 분들도, 러스트에서 흔히 사용하는 패키지 중 C 코드에
의존하는 패키지들이 있으니 미리 C 컴파일러도 설치하시길 권장드립니다.

### `rustup` 설치 - Windows

윈도우 사용자는 [https://www.rust-lang.org/tools/install][install] 에서
안내를 따라주시기 바랍니다. 설치시 C++ 빌드 툴이 필요하다는 메시지가
나타날 텐데, Visual studio 2013 이상 버전에 포함된 빌드 툴이 알맞습니다.
가장 쉬운 방법은 [Microsoft C++ 빌드 도구][visualstudio] 를 설치하시는 겁니다.
설치시 워크로드에서 "C++ 빌드 도구"가 선택되어 있는지, 그리고 개별 컴포넌트 중
Windows 10 SDK가, 언어 팩 중에서 영어 언어 팩이 포함되어 있는지 확인하세요.

[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/visual-cpp-build-tools/

이 뒷내용부턴 *cmd.exe* 와 PowerShell 에서 혼용되는 명령어만
사용할 예정이며, 서로 다른 부분이 있을 경우엔 따로 명시하겠습니다.

### 업데이트 및 삭제

`rustup` 으로 러스트를 설치했다면 최신 버전 업데이트도 간편합니다.
셸에 다음 명령어를 입력해주세요:

```console
$ rustup update
```

`rustup` 과 러스트를 삭제하는 방법은 다음과 같습니다
(지금 입력하진 마세요!):

```console
$ rustup self uninstall
```

### 트러블 슈팅

러스트가 제대로 설치되었는지
확인하는 방법은 다음과 같습니다:

```console
$ rustc --version
```

최신 릴리즈된 stable 버전 정보가 다음 포맷대로 나타나며,
나타난 정보는 순서대로 버전 숫자, 커밋 해쉬(hash), 커밋 날짜입니다:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

이 명령어들이 제대로 실행되지 않으면
윈도우 사용자는 환경변수 `%PATH%` 에 러스트가 잘 등록됐는지 확인해주세요.
잘못된 것을 찾을 수 없는데 계속 작동하지 않으면
[한국 러스트 사용자 그룹 디스코드][korean_discord] 에 질문해주세요.
한국 러스트 사용자들이 서로 대화하는 곳입니다. 영어가 능숙한 분들은
[공식 러스트 디스코드(영문)][official_discord] 의 #beginners 채널이나
[사용자 포럼][users], [스택오버플로우][stackoverflow] 등에 질문하시는 것도 좋은 방법입니다.

[korean_discord]: https://discord.gg/uqXGjEz
[official_discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[stackoverflow]: https://stackoverflow.com/questions/tagged/rust

### 로컬 문서

러스트 설치 시 로컬 문서(Local Documentation)도 같이 설치됩니다. 오프라인
상태로도 이용 가능하며, `rustup doc` 명령어로 여러분 브라우저에서 열어볼 수 있습니다.

표준 라이브러리에서 제공하는 타입이나 함수 중 이게 무슨 기능을 하는지,
혹은 사용법을 모르면 로컬 API(Application Programming Language) 문서에서
모르는 내용을 찾아볼 수도 있겠죠!
