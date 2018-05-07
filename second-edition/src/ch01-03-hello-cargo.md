## Hello, Cargo!

Cargo(카고)는 러스트의 빌드 시스템 및 패키지 매니저입니다. 대부분의 러스트인들이 이 도구를 이용하여
그들의 러스트 프로젝트를 관리하는데, 그 이유는 Cargo가 여러분의 코드를 빌드하고, 여러분의 코드가
의존하고 있는 라이브러리를 다운로드해주고, 그 라이브러리들을 빌드하는 등 여러분을 위한 많은 작업들을
다루기 때문입니다. (여러분의 코드가 필요로 하는 라이브러리를 *의존성 (dependency)* 이라고 부릅니다)

여러분이 이제껏 작성한 것과 같은 가장 단순한 러스트 프로그램은 어떠한 의존성도 없습니다.
따라서 만일 Cargo를 가지고 “Hello, world!” 프로젝트를 빌드했다면, 여러분의 코드를
빌드하는 것을 다루는 카고의 일부분만일 이용하게 되었을 것입니다. 여러분이 더 복접한
러스트 프로그램을 작성할 때면, 여러분은 의존성을 추가할 것이고, 여러분이 Cargo를
이용하여 프로젝트를 시작한다면, 의존성 추가가 훨씬 더 하기 쉬워질 것입니다.

압도적인 숫자의 러스트 프로젝트가 Cargo를 이용하기 때문에, 이 책의 나머지 부분에서는
여러분 또한 Cargo를 이용하고 있음을 가정합니다. 만일 여러분이 “설치하기” 절에서 다룬대로
공식 인스톨러를 이용했다면 Cargo는 러스트와 함께 설치되어 있습니다. 만일 여러분이 다른
수잔을 통해 러스트를 설치했다면, Cargo가 설치되어 있는지 확인하기 위해서 여러분의
터미널에 다음을 입력해보세요:

```text
$ cargo --version
```

버전 숫자가 보인다면, 가지고 있는 것입니다! `command not fount` 같은 에러를
보게 된다면, 여러분이 설치한 방법에 대한 문서에서 Cargo를 개별적으로 어떻게
설치하는지 찾아보세요.

### Cargo를 사용하여 프로젝트 생성하기

Cargo를 사용하여 새 프로젝트를 만들고 우리의 원래 “Hello, world!” 프로젝트와
얼마나 차이가 나는지 살펴봅시다. 여러분의 *projects* 디렉토리로 (혹은 여러분의
코드를 저장하기로 결정한 어느 곳이든) 이동하세요. 그 다음, 어떤 운영체제이든 상관없이
다음을 실행하세요:

```text
$ cargo new hello_cargo --bin
$ cd hello_cargo
```

첫번째 커맨드는 *hello_cargo*라고 불리우는 새로운 실행 가능한 바이너리를 생성합니다.
`cargo new`에게 넘겨지는 `--bin` 인자가 라이브러리가 아닌 실행 가능한 애플리케이션으로
만들어줍니다 (흔히들 그냥 *바이너리 (binary)* 라고 부릅니다). 우리의 프로젝트는 *hello_cargo*
라고 이름지었고, Cargo는 동일한 이름의 디렉토리에 이 프로젝트의 파일들을 생성합니다.

*hello_cargo* 디렉토리로 가서 파일 리스트를 보세요. 여러분은 Cargo가 우리를 위해
두 개의 파일과 하나의 디렉토리를 생성한 것을 볼 수 있을 것입니다: *Cargo.toml* 파일 및
안에 *main.rs* 파일을 담고 있는 *src* 디렉토리가 그것입니다. 안에는 또한 *.gitignore*과
함께 새로운 Git 저장소도 초기화되어 있습니다.

> 노트: Git은 보편적인 버전 관리 시스템입니다. 여러분은 `--vcs` 플래그를 사용하여
> `cargo new`가 다른 버전 관리 시스템을 사용하거나 혹은 버전 관리 시스템을 사용하지
> 않도록 변경할 수 있습니다. 사용 가능한 옵션을 보려면 `cargo new --help`를 실행하세요.

*Cargo.toml*을 여러분이 원하는 텍스트 에디터로 여세요. 이 파일은 Listing 1-2의
코드와 유사하게 보여야 합니다.

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

<span class="caption">Listing 1-2: `cargo new`가 생성한 *Cargo.toml*
내용</span>

이 파일은 [*TOML*][toml]<!-- ignore --> (Tom’s Obvious, Minimal Language)
포맷으로 작성되었는데, 이것이 Cargo의 환경설정 포맷입니다.

[toml]: https://github.com/toml-lang/toml

첫번째 라인 `[package]`은 이후의 문장들이 패키지 환경설정이라는 것을 나타내는
섹션의 시작지점입니다. 우리가 이 파일에 더 많은 정보를 추가하기 위해, 다른 섹션들을
추가할 것입니다.

그 다음 세 라인들은 Cargo가 여러분의 프로그램을 컴팡ㄹ하기 위해 필요로 하는
정보에 대한 설정을 합니다: 이름, 버전, 그리고 누가 작성했는가 입니다. Cargo는
여러분의 환경으로부터 여러분의 이름과 이메일 정보를얻어내므로, 만일 그 정보가
정확하지 않다면, 지금 수정하고 파일을 저장하세요.

마지막 라인 `[dependencies]`은 여러분 프로젝트의 의존성들의 리스트를 적을 수
있는 섹션의 시작점입니다. 러스트에서는 코드의 패키지를 *크레이트 (crate)* 라고 부릅니다.
이 프로젝트를 위해서는 어떤 다른 크레이트도 필요없지만, 2장의 첫 프로젝트에서는 필요할
것이므로, 그때 이 의존성 섹션을 사용하겠습니다.

이제 *src/main.rs*을 열어서 살펴봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo는 우리가 Listing 1-1에서 작성했던 것과 똑같이 여러분을 위해 “Hello, world!”
프로그램을 작성해놨습니다! 여기까지, 우리의 이전 프로젝트와 Cargo가 만든 프로젝트 간의
차이점은 Cargo가 코드를 *src* 디렉토리 안에 위치시킨다는 점, 그리고 최상위 디렉토리에
*Cargo.toml* 환경 파일을 가지게 해준다는 점입니다.

Cargo는 여러분의 소스 파일들이 *src* 디렉토리 안에 있을 것으로 예상합니다.
최상위 프로젝트 디렉토리는 그저 README 파일들, 라이센스 정보, 환경 파일들,
그리고 여러분의 코드와는 관련이 없는 다른 것들 뿐입니다. Cargo를 이용하는
것은 여러분이 프로젝트를 조직화하는 데에 도움을 줍니다. 모든 것을 위한 공간이
있고, 모든 것은 자신의 공간 안에 있습니다.

만일 여러분이 Hello, world! 프로젝트에서 했던 것처럼 Cargo를 사용하지
않은 프로젝트를 시작했다면, Cargo를 사용한 프로젝트로 이를 바꿀 수 있습니다.
프로젝트 크드를 *src* 디렉토리로 옮기고 적합한 *Cargo.toml* 파일을
생성하세요.

### Cargo 프로젝트를 빌드하고 실행하기

이제 Cargo로 만든 “Hello, world!” 프로젝트를 빌드하고 실행할 떄의
차이점을 살펴봅시다! *hello_cargo* 디렉토리에서, 다음 커맨드를 입력하는
것으로 여러분의 프로젝트를 빌드하세요:

```text
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

이 커맨드는 여러분의 현재 디렉토리 대신 *target/debug/hello_cargo*에
(혹은 Windows에서는 *target\debug\hello_cargo.exe*에) 실행 파일을
생성합니다. 여러분은 아래 커맨드를 통해 이 실행 파일을 실행할 수 있습니다:

```text
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

만일 모든 것이 잘 진행되었다면, 터미널에 `Hello, world!`가 출력되어야
합니다. 처음으로 `cargo build`를 실행하는 것은 또한 Cargo가 최상위
디렉토리에 *Cargo.lock* 이라는 새로운 파일을 생성하도록 합니다. 이
프로젝트는 어떠한 의존성도 가지고 있지 않으므로, 파일의 내용이 얼마 없습니다.
여러분이 이 파일을 손수 변경할 필요는 전혀 없습니다; Cargo가 여러분을 위해
이 파일의 내용을 관리합니다.

우리는 그저 `cargo build`로 프로젝트를 빌드하고 `./target/debug/hello_cargo`로
이를 실행했지만, 또한 `cargo run`를 사용하여 한번의 커맨드로 코드를 컴파일한
다음 결과 실행파일을 실행할 수 있습니다:

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

이번에는 Cargo가 `hello_cargo`를 컴파일하는 중이었다는 것을 나타내는 출력을 볼 수
없음을 주목하세요. Cargo는 파일들이 변경된 적이 없음을 알아내고, 따라서 해당 바이너리를
그저 실행했을 뿐입니다. 만일 여러분이 여러분의 코드를 수정한 적 있다면, Cargo는 그
프로젝트를 실행하기 전에 다시 빌드할 것이고, 여러분은 아래와 같은 출력을 보게될 것입니다:

```text
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo는 또한 `cargo check`라고 하는 커맨드를 제공합니다. 이 커맨드는 여러분의 코드가
컴파일되는지를 빠르게 확인해주지만 실행파일을 생성하지는 않습니다:

```text
$ cargo check
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

왜 여러분이 실행파일을 원치 않게 될까요? 종종 `cargo check`가 `cargo build`에 비해
훨씬 빠른데, 그 이유는 이 커맨드가 실행파일을 생성하는 단계를 생각혀기 때문입니다. 만일 여러분이
코드를 작성하는 동안 계속적으로 여러분의 작업물을 검사하는 중이라면, `cargo check`를 이용하는
것이 그 과정의 속도를 높여줄 것입니다! 그런 이유로, 많은 러스트인들이 자신들의 프로그램을 작성하면서
이것이 컴파일 되는지 확인하기 위해 주기적으로 `cargo check`을 실행합니다. 그런 다음 실행파일을
사용할 준비가 되었을 때 `cargo build`를 실행합니다.

여태까지 Cargo에 대하여 우리가 배운 것들을 정리하자면:

* 우리는 `cargo build`나 `cargo check`를 사용하여 프로젝트를 빌드할 수 있습니다.
* 우리는 `cargo run`를 사용하여 단숨에 프로젝트를 빌드하고 실행할 수 있습니다.
* 우리 코드가 있는 동일한 디렉토리에 빌드의 결과물이 저장되는 대신, Cargo는 이를 *target/debug*
디렉토리에 저장합니다.

Cargo를 사용하면 생기는 추가적인 장점은 여러분이 어떠한 운영체제로 작업을 하든
상관없이 커맨드들이 동일하다는 점입니다. 따라서 이러한 점 때문에 우리는 더 이상
Linux와 macOS 및 Windows를 위한 특정 명령을 제공하지 않을 것입니다.

### 릴리즈 빌드

여러분의 프로젝트가 마침내 배포(릴리즈)를 위한 준비가 되었다면, `cargo build --release`를
사용하여 최적화와 함께 이를 컴파일할 수 있습니다. 이 커맨드는 *target/debug* 대신
*target/release*에 실행파일을 생성할 것입니다. 최적화는 여러분의 러스트 코드를 더
빠르게 만들어주지만, 최적화를 켜는 것은 여러분의 프로그램을 컴파일하는데 드는 시간을 길게
할 것입니다: 이것이 바로 두 개의 서로 다른 프로파일이 있는 이유입니다: 하나는 여러분이
빠르게 그리고 자주 다시 빌드하기를 원하는 개발용, 그리고 다른 하나는 반복적으로 다시
빌드를 할 필요 없고 가능한 빠르게 실행되어 여러분이 사용자들에게 제공할 최종 프로그램을
빌드하기 위한 용도입니다. 만일 여러분이 코드의 실행 시간을 벤치마킹 중이라면, `cargo
build --release`를 실행하고 *target/release*의 실행파일을 가지고 밴치마킹하고
있음을 확인하세요. 

### 관례로서의 Cargo

단순한 프로젝트와 함께 Cargo를 사용하는 것은 그냥 `rustc`을 이용하는 것에 비해
큰 가치를 제공해주지는 못합니다만, 여러분의 프로그램이 점점 더 복잡해질수록 Cargo는
자신의 가치를 증명할 것입니다. 여러 개의 크레이트들로 구성된 복잡한 프로젝트와 함께라면
Cargo가 빌드를 조직화하도록 하는것이 훨씬 쉽습니다.

비록 `hello_cargo` 프로젝트가 단순했을지라도, 이 프로젝트는 이제 여러분의 남은 러스트
경력 생활 내에 사용하게될 진짜배기 도구를 사용하였습니다. 사실, 어떤 기존 프로젝트들 상에서
작업을 하기 위해서, 여러분은 Git을 사용하여 코드를 체크 아웃하고 그 프로젝트 디렉토리로 가서
빌드하기 위해 다음 커맨드를 사용할 수 있습니다:

```text
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

Cargo에 대해 더 많은 정보를 보려면 [문서][its documentation]를 참고하세요.

[its documentation]: https://doc.rust-lang.org/cargo/

## 정리

여러분은 이미 여러분의 러스트 여정에서 아주 좋은 출발을 하고 있습니다! 이 장에서는
아래 항목들을 어떻게 하는지에 대해 배웠습니다:

* `rustup`을 사용하여 최신의 안정화된 러스트 버전 설치하기
* 더 최근에 나온 러스트 버전으로 업데이트하기
* 로컬에 설치된 문서 열기
* `rustc`를 직접 사용하여 “Hello, world!” 프로그램을 작성하고 실행하기
* Cargo의 관례를 사용하여 새로운 프로젝트를 만들고 실행하기

이제 러스트 코드를 읽고 쓰는데 익숙해지기 위해서 좀더 상당한 프로그램을 빌드하기
좋은 시간입니다. 따라서 다음 장에서는 추리 게임 프로그램을 빌드해 볼 것입니다.
만일 여러분이 차라리 러스트에서 어떻게 보편적인 프로그래밍 개념이 동작하는지를 배우는
것으로 시작하길 원한다면, 3장을 먼저 보시고 2장으로 돌아오세요.

