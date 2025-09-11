## Hello, World!

이제 Rust를 설치했으니, 첫 번째 Rust 프로그램을 작성할 차례입니다.  
새로운 언어를 배울 때는 전통적으로 화면에 `Hello, world!`를 출력하는  
작은 프로그램을 작성하곤 합니다. 여기서도 똑같이 해 보겠습니다!

> 참고: 이 책은 명령줄에 대한 기본적인 익숙함을 전제로 합니다. Rust는 코드가
> 어디에 있든, 어떤 편집기를 쓰든 특별한 제약을 두지 않습니다. 따라서
> 명령줄 대신 IDE(통합 개발 환경)를 사용하고 싶으시다면 자유롭게 사용하셔도
> 됩니다. 요즘 많은 IDE가 Rust를 어느 정도 지원합니다. 자세한 내용은 IDE
> 문서를 참고하세요. Rust 팀은 `rust-analyzer`를 통해 훌륭한 IDE 지원을
> 제공하는 데 집중해 왔습니다. 더 자세한 내용은 [부록 D][devtools]에서
> 확인할 수 있습니다.

### 프로젝트 디렉터리 만들기

먼저 Rust 코드를 저장할 디렉터리를 만들어 보겠습니다. Rust는 코드가 어디에
있든 상관하지 않지만, 이 책의 연습 문제와 프로젝트를 위해서는 홈 디렉터리
안에 _projects_ 디렉터리를 하나 만들고 모든 프로젝트를 그 안에 두는 것을
권장합니다.

터미널을 열고 _projects_ 디렉터리를 만든 뒤, 그 안에 “Hello, world!” 프로젝트
디렉터리를 만드는 명령을 입력하세요.

Linux, macOS, Windows PowerShell에서는 다음을 입력합니다:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windows CMD에서는 다음을 입력합니다:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### Rust 프로그램 작성하고 실행하기

이제 새로운 소스 파일을 만들고 이름을 _main.rs_로 지정하세요. Rust 소스 파일은
항상 _.rs_ 확장자로 끝납니다. 파일 이름이 여러 단어로 이루어져 있다면,
언더스코어(_)로 구분하는 것이 관례입니다. 예를 들어, _hello_world.rs_라고
쓰고, _helloworld.rs_라고 쓰지는 않습니다.

방금 만든 _main.rs_ 파일을 열고, Listing 1-1의 코드를 입력하세요.

<Listing number="1-1" file-name="main.rs" caption="`Hello, world!`를 출력하는 프로그램">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

파일을 저장한 뒤, 터미널에서 _~/projects/hello_world_ 디렉터리로 돌아가세요.  
Linux나 macOS에서는 다음 명령으로 파일을 컴파일하고 실행합니다:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

Windows에서는 `./main` 대신 `.\main`을 입력합니다:

```powershell
> rustc main.rs
> .\main
Hello, world!
```

어떤 운영체제를 쓰든, 터미널에 `Hello, world!`가 출력될 것입니다. 만약 출력이
보이지 않는다면, 설치 절의 [“문제 해결”][troubleshooting] 부분을 참고해
도움을 받으세요.

만약 `Hello, world!`가 출력됐다면, 축하합니다! 이제 공식적으로 Rust 프로그램을
작성한 것입니다. 여러분은 Rust 프로그래머입니다 — 환영합니다!

### Rust 프로그램의 구조

이제 “Hello, world!” 프로그램을 자세히 살펴봅시다. 먼저 이 부분을 보겠습니다:

```rust
fn main() {

}
```

이 코드는 `main`이라는 함수를 정의합니다. `main` 함수는 특별한 함수로, 모든
실행 가능한 Rust 프로그램에서 항상 가장 먼저 실행됩니다. 첫 번째 줄은 매개변수도
없고 반환 값도 없는 `main` 함수를 선언합니다. 만약 매개변수가 있다면
괄호 `()` 안에 들어갑니다.

함수 본문은 `{}`로 감싸져 있습니다. Rust에서는 모든 함수 본문을 중괄호로
감싸야 합니다. 일반적으로 여는 중괄호는 함수 선언과 같은 줄에, 한 칸 띄고
작성하는 것이 좋은 스타일입니다.

> 참고: Rust 프로젝트 전반에서 일관된 스타일을 유지하고 싶다면, `rustfmt`
> 라는 자동 코드 포맷터 도구를 사용할 수 있습니다. `rustfmt`에 대해서는
> [부록 D][devtools]에서 더 다룹니다. Rust 팀은 `rustc`와 마찬가지로
> `rustfmt`를 표준 배포판에 포함시켰으므로, 이미 설치되어 있을 것입니다.

`main` 함수의 본문에는 다음 코드가 들어 있습니다:

```rust
println!("Hello, world!");
```

이 한 줄이 이 작은 프로그램의 모든 일을 합니다. 화면에 텍스트를 출력하는 것이죠.  
여기서 주목할 세 가지가 있습니다.

첫째, `println!`은 Rust 매크로 호출입니다. 만약 함수였다면 `println`이라고
`!` 없이 작성했을 겁니다. Rust 매크로는 문법을 확장하기 위해 코드를 생성하는
방식으로 작성되며, 자세한 내용은 [20장][ch20-macros]에서 다룹니다. 지금은
`!`가 붙으면 함수가 아니라 매크로를 호출한다는 점만 기억하시면 됩니다.

둘째, `"Hello, world!"`라는 문자열입니다. 이 문자열을 `println!` 매크로에
인자로 넘기면, 화면에 출력됩니다.

셋째, 줄 끝에 세미콜론(`;`)을 붙입니다. 이는 표현식이 끝났음을 나타내고,
다음 표현식을 시작할 준비가 되었음을 의미합니다. 대부분의 Rust 코드 줄은
세미콜론으로 끝납니다.

### 컴파일과 실행은 별개의 단계

방금 새로 만든 프로그램을 실행했으니, 이제 과정의 각 단계를 살펴봅시다.

Rust 프로그램을 실행하기 전에 먼저 `rustc` 명령으로 컴파일해야 합니다. 소스
파일 이름을 함께 넘겨줍니다:

```console
$ rustc main.rs
```

C나 C++ 경험이 있다면, `gcc`나 `clang`을 쓰는 것과 유사하다는 걸 알 수 있을
겁니다. 컴파일이 성공하면 Rust는 바이너리 실행 파일을 생성합니다.

Linux, macOS, Windows PowerShell에서는 셸에서 `ls` 명령으로 실행 파일을
확인할 수 있습니다:

```console
$ ls
main  main.rs
```

Linux와 macOS에서는 두 개의 파일이 보입니다. PowerShell을 쓰는 Windows에서는
CMD와 동일하게 세 개의 파일이 보일 것입니다. CMD에서는 다음 명령을 입력합니다:

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

여기에는 _.rs_ 확장자의 소스 파일, 실행 파일(Windows에서는 _main.exe_, 다른
플랫폼에서는 _main_), 그리고 Windows의 경우 디버깅 정보를 담은 _.pdb_ 파일이
포함됩니다. 이제 _main_ 또는 _main.exe_ 파일을 실행합니다:

```console
$ ./main # Windows에서는 .\main
```

_ main.rs_ 파일이 “Hello, world!” 프로그램이라면, 이 명령은 `Hello, world!`를
터미널에 출력합니다.

Ruby, Python, JavaScript 같은 동적 언어에 익숙하다면, 컴파일과 실행을
따로따로 하는 과정이 낯설 수 있습니다. Rust는 _사전 컴파일(ahead-of-time
compiled)_ 언어이기 때문에, 프로그램을 컴파일한 뒤 생성된 실행 파일을 다른
사람에게 주면, 그 사람이 Rust를 설치하지 않았더라도 실행할 수 있습니다.
반면, _.rb_, _.py_, _.js_ 파일을 주려면 상대방이 해당 언어의 실행 환경을
가지고 있어야 합니다. 하지만 이러한 언어들은 컴파일과 실행이 한 번에
이뤄집니다. 언어 설계는 언제나 이런 식의 트레이드오프가 있습니다.

단순한 프로그램은 `rustc`만으로 충분하지만, 프로젝트가 커질수록 다양한 옵션을
관리하고 코드를 쉽게 공유할 수 있는 방법이 필요합니다. 다음 절에서는 실전
Rust 프로그램 개발을 도와줄 Cargo 도구를 소개하겠습니다.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.html
[ch20-macros]: ch20-05-macros.html
