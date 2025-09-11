## 설치

첫 번째 단계는 Rust를 설치하는 것입니다. Rust는 `rustup`이라는 명령줄 도구를
통해 다운로드합니다. 이 도구는 Rust 버전과 관련 도구들을 관리합니다. 설치를
위해 인터넷 연결이 필요합니다.

> 참고: 어떤 이유로든 `rustup`을 사용하지 않으시려면, [기타 Rust 설치 방법][otherinstall]
> 페이지에서 다른 선택지를 확인해 보실 수 있습니다.

다음 단계는 최신 안정(stable) 버전의 Rust 컴파일러를 설치합니다. Rust의 안정성
보장은, 이 책의 예제 코드가 현재 버전에서 컴파일된다면 앞으로의 새로운 Rust
버전에서도 그대로 컴파일될 것임을 의미합니다. 버전마다 출력 결과가 약간씩
다를 수는 있는데, 이는 Rust가 오류 메시지와 경고를 더 개선하기 때문입니다.
즉, 여기서 안내하는 절차로 설치한 어떤 최신 안정 버전의 Rust도 이 책의 내용을
문제 없이 실행할 수 있습니다.

> ### 명령줄 표기법
>
> 이 장과 책 전반에서는 터미널에서 사용하는 명령을 보여 줍니다. 터미널에
> 입력해야 하는 줄은 `$` 기호로 시작합니다. `$`는 실제 입력하는 문자가 아니라,
> 명령줄의 시작을 알리는 프롬프트 표시입니다. `$`로 시작하지 않는 줄은 보통
> 이전 명령의 출력 결과를 보여 줍니다. PowerShell 예시는 `$` 대신 `>`를 사용합니다.

### Linux 또는 macOS에서 `rustup` 설치하기

Linux나 macOS를 사용하신다면, 터미널을 열고 다음 명령을 입력하세요:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

이 명령은 스크립트를 다운로드하여 `rustup` 설치를 시작합니다. 그러면 최신
안정 버전의 Rust가 설치됩니다. 설치 과정에서 비밀번호 입력을 요청할 수 있습니다.
설치가 성공하면 다음과 같은 메시지가 표시됩니다:

```text
Rust is installed now. Great!
```

Rust는 또한 _링커(linker)_ 를 필요로 합니다. 링커는 Rust가 컴파일한 결과물을
하나의 실행 파일로 묶는 프로그램입니다. 대부분은 이미 시스템에 링커가 있을
가능성이 큽니다. 만약 링커 오류가 발생한다면, 보통 C 컴파일러를 설치하면
함께 제공됩니다. Rust의 일부 패키지는 C 코드에 의존하기 때문에 C 컴파일러는
추가로 유용합니다.

macOS에서는 다음 명령으로 C 컴파일러를 설치할 수 있습니다:

```console
$ xcode-select --install
```

Linux 사용자는 보통 배포판 설명서에 따라 GCC나 Clang을 설치하면 됩니다.
예를 들어, Ubuntu에서는 `build-essential` 패키지를 설치하면 됩니다.

### Windows에서 `rustup` 설치하기

Windows에서는 [https://www.rust-lang.org/tools/install][install] 페이지로 이동해
안내에 따라 Rust를 설치하면 됩니다. 설치 과정에서 Visual Studio 설치를
요청받을 수 있는데, 이는 링커와 프로그램을 컴파일하는 데 필요한 네이티브
라이브러리를 제공합니다. 이 과정에서 더 자세한 안내가 필요하다면
[Windows MSVC 설치 가이드][msvc]를 참고하십시오.

이 책의 나머지 예제는 _cmd.exe_와 PowerShell에서 모두 실행할 수 있는 명령을
사용합니다. 만약 두 환경에서 차이가 있다면, 어떤 것을 써야 하는지 따로
설명드리겠습니다.

### 문제 해결

Rust가 올바르게 설치되었는지 확인하려면, 셸에서 다음 명령을 실행해 보세요:

```console
$ rustc --version
```

그러면 아래와 같은 형식으로 최신 안정 버전의 버전 번호, 커밋 해시, 커밋 날짜가
출력됩니다:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

이 정보가 보인다면 Rust가 성공적으로 설치된 것입니다! 보이지 않는다면, 시스템의
`%PATH%` 환경 변수에 Rust가 포함되어 있는지 확인해야 합니다.

Windows CMD에서는 다음 명령으로 확인합니다:

```console
> echo %PATH%
```

PowerShell에서는:

```powershell
> echo $env:Path
```

Linux와 macOS에서는:

```console
$ echo $PATH
```

이 모든 것이 올바른데도 Rust가 여전히 동작하지 않는다면, 도움을 받을 수 있는
곳이 있습니다. Rust 커뮤니티 페이지[community]에서 다른 Rustacean(러스테이시안,
Rust 사용자들이 스스로를 부르는 애칭)과 소통하는 방법을 찾아보세요.

### 업데이트와 제거

`rustup`을 통해 설치한 Rust는 새 버전으로 손쉽게 업데이트할 수 있습니다.
셸에서 다음 명령을 실행하세요:

```console
$ rustup update
```

Rust와 `rustup`을 제거하려면, 다음 명령을 실행합니다:

```console
$ rustup self uninstall
```

### 로컬 문서

Rust를 설치하면 로컬 문서도 함께 제공되므로 오프라인에서도 읽을 수 있습니다.
브라우저에서 문서를 열려면 `rustup doc` 명령을 실행하세요.

표준 라이브러리가 제공하는 타입이나 함수가 무엇을 하는지 잘 모르겠다면, API
문서를 열어 확인해 보시면 됩니다.

### 텍스트 에디터와 IDE

이 책은 여러분이 어떤 도구를 사용해 Rust 코드를 작성하는지 전혀 가정하지
않습니다. 사실 거의 모든 텍스트 에디터에서 Rust 코드를 작성할 수 있습니다.
하지만 많은 에디터와 IDE(통합 개발 환경)는 Rust를 기본 지원합니다. Rust
웹사이트의 [도구 페이지][tools]에서 비교적 최신의 지원 목록을 확인하실 수
있습니다.

### 오프라인으로 이 책 활용하기

몇몇 예제에서는 표준 라이브러리 외의 Rust 패키지를 사용합니다. 이를 실행하려면
인터넷 연결이 필요하거나, 미리 의존성을 다운로드해 두셔야 합니다. 사전에
다운로드하려면 다음 명령을 실행하세요. (`cargo`와 각 명령의 의미는 뒤에서
자세히 설명합니다.)

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0
```

이 명령은 해당 패키지를 캐시에 저장하여 이후 다시 다운로드할 필요가 없도록
만듭니다. 한 번 실행했다면 `get-dependencies` 폴더를 그대로 두지 않아도
괜찮습니다. 이렇게 준비했다면, 책의 나머지 예제에서 `cargo` 명령을 실행할 때
`--offline` 플래그를 사용해 네트워크 대신 캐시된 버전을 활용할 수 있습니다.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
