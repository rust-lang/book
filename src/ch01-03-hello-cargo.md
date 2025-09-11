## Hello, Cargo!

Cargo는 Rust의 빌드 시스템이자 패키지 관리자입니다. 대부분의 Rustacean은
이 도구로 프로젝트를 관리합니다. Cargo는 코드 빌드, 코드가 의존하는 라이브러리
다운로드, 해당 라이브러리 빌드 등 많은 일을 대신 처리해 줍니다. (코드가
필요로 하는 라이브러리를 우리는 _의존성(dependencies)_ 이라고 부릅니다.)

지금까지 작성한 가장 단순한 Rust 프로그램은 의존성이 없습니다. 만약 우리가
“Hello, world!” 프로젝트를 Cargo로 빌드했다면, Cargo의 기능 중 코드 빌드만
사용했을 겁니다. 더 복잡한 Rust 프로그램을 작성하게 되면 의존성을 추가하게
되고, 프로젝트를 Cargo로 시작했다면 의존성 추가가 훨씬 쉬워집니다.

대다수의 Rust 프로젝트가 Cargo를 사용하므로, 이 책의 나머지 부분도 여러분이
Cargo를 사용한다는 가정하에 진행합니다. [“설치”][installation] 절에서 소개한
공식 설치 프로그램을 사용했다면 Rust와 함께 Cargo도 설치됩니다. 다른 방법으로
Rust를 설치했다면, 터미널에서 다음을 입력해 Cargo가 설치되어 있는지 확인하세요:

```console
$ cargo --version
```

버전 번호가 보인다면 설치 완료입니다! `command not found`와 같은 오류가 나온다면
설치 방식의 문서를 확인하여 Cargo를 별도로 설치하는 방법을 찾으세요.

### Cargo로 프로젝트 생성하기

이제 Cargo를 사용해 새 프로젝트를 만들어 보고, 처음 만든 “Hello, world!”와 어떤
점이 다른지 살펴봅시다. _projects_ 디렉터리(혹은 코드를 보관하기로 한 곳)로
돌아간 뒤, 운영체제와 관계없이 다음을 실행하세요:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

첫 번째 명령은 _hello_cargo_라는 새 디렉터리와 프로젝트를 생성합니다. 우리는
프로젝트 이름을 _hello_cargo_로 정했고, Cargo는 같은 이름의 디렉터리에 파일들을
생성합니다.

_hello_cargo_ 디렉터리로 들어가 파일 목록을 확인하세요. Cargo가 두 개의 파일과
하나의 디렉터리를 생성한 것을 볼 수 있습니다: _Cargo.toml_ 파일 하나와, 그 안에
_main.rs_ 파일이 들어 있는 _src_ 디렉터리 하나입니다.

또한 새 Git 저장소와 _.gitignore_ 파일도 함께 초기화했습니다. 기존 Git 저장소
안에서 `cargo new`를 실행하면 Git 관련 파일은 생성하지 않습니다. 이 동작을
바꾸려면 `cargo new --vcs=git`을 사용하세요.

> 참고: Git은 널리 쓰이는 버전 관리 시스템입니다. `cargo new`의 `--vcs` 플래그로
> 다른 버전 관리 시스템을 사용하거나, 아예 사용하지 않도록 선택할 수 있습니다.
> 가능한 옵션은 `cargo new --help`로 확인하세요.

선호하는 텍스트 에디터로 _Cargo.toml_ 을 여세요. Listing 1-2와 비슷하게 보일
것입니다.

<Listing number="1-2" file-name="Cargo.toml" caption="`cargo new`가 생성한 *Cargo.toml* 내용">

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

이 파일은 Cargo의 설정 형식인 [_TOML_][toml](_Tom’s Obvious, Minimal Language_)
형식을 사용합니다.

첫 줄의 `[package]`는 이후의 설정이 패키지에 관한 것임을 나타내는 섹션 헤더입니다.
이 파일에 정보를 더 추가할수록 다른 섹션도 추가하게 됩니다.

다음 세 줄은 프로그램을 컴파일하는 데 Cargo가 필요로 하는 설정을 지정합니다:
패키지 이름, 버전, 그리고 사용할 Rust 에디션입니다. `edition` 키에 대해서는
[부록 E][appendix-e]에서 다룹니다.

마지막 줄 `[dependencies]`는 프로젝트의 의존성을 나열하는 섹션의 시작입니다.
Rust에서 코드 패키지는 _크레이트(crates)_ 라고 부릅니다. 이 프로젝트에는 다른
크레이트가 필요 없지만, 2장의 첫 번째 프로젝트에서는 필요하게 될 것이므로 그때
이 섹션을 사용하겠습니다.

이제 _src/main.rs_ 를 열어 보세요:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo는 Listing 1-1에서 우리가 작성했던 것과 동일한 “Hello, world!” 프로그램을
미리 만들어 주었습니다. 지금까지의 차이점은, 우리 프로젝트는 코드를 _src_
디렉터리에 두고, 최상위 디렉터리에는 _Cargo.toml_ 설정 파일이 있다는 점입니다.

Cargo는 소스 파일이 _src_ 디렉터리 안에 있다고 가정합니다. 최상위 프로젝트
디렉터리는 README, 라이선스, 설정 파일 등, 코드와 직접 관련 없는 것들을 위한
공간입니다. Cargo를 사용하면 프로젝트 구성이 자연스럽게 정돈됩니다. 모든 것은
자리에, 자리는 모든 것을 위해 준비되어 있습니다.

만약 “Hello, world!”처럼 Cargo 없이 시작한 프로젝트가 있다면, Cargo 프로젝트로
변환할 수 있습니다. 프로젝트 코드를 _src_ 디렉터리로 옮기고, 알맞은
_Cargo.toml_ 파일을 만들어 주세요. 가장 쉬운 방법은 `cargo init`을 실행하여
_Cargo.toml_ 을 자동으로 생성하는 것입니다.

### Cargo 프로젝트 빌드와 실행

이제 Cargo로 “Hello, world!”를 빌드하고 실행할 때 무엇이 다른지 살펴봅시다.
_hello_cargo_ 디렉터리에서 다음 명령으로 프로젝트를 빌드하세요:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

이 명령은 현재 디렉터리가 아니라 _target/debug/hello_cargo_ (Windows에서는
_target\debug\hello_cargo.exe_)에 실행 파일을 생성합니다. 기본 빌드는 디버그
빌드이므로, Cargo는 바이너리를 _debug_ 라는 디렉터리에 둡니다. 실행 파일은 다음
명령으로 실행할 수 있습니다:

```console
$ ./target/debug/hello_cargo # Windows에서는 .\target\debug\hello_cargo.exe
Hello, world!
```

모든 것이 잘 되었다면 `Hello, world!`가 터미널에 출력될 것입니다. 처음으로
`cargo build`를 실행하면 최상위 디렉터리에 _Cargo.lock_ 파일도 생성됩니다. 이
파일은 프로젝트에서 사용 중인 의존성의 정확한 버전을 기록합니다. 이 프로젝트는
의존성이 없으므로 내용은 거의 없습니다. 이 파일을 수동으로 수정할 일은
없습니다. Cargo가 내용을 관리합니다.

방금 우리는 `cargo build`로 빌드하고 `./target/debug/hello_cargo`로 실행했지만,
`cargo run`을 사용하면 컴파일과 실행을 한 번에 할 수 있습니다:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

대부분의 개발자는 `cargo build` 후 바이너리 경로를 입력하는 것보다 `cargo run`
을 더 편하게 사용합니다.

이번에는 Cargo가 `hello_cargo`를 컴파일한다는 출력이 보이지 않았습니다. 파일이
변경되지 않았음을 Cargo가 감지하여, 다시 빌드하지 않고 기존 바이너리를 실행한
것입니다. 만약 소스 코드를 수정했다면 Cargo는 실행 전에 프로젝트를 재빌드하며,
다음과 같은 출력을 보게 됩니다:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo에는 `cargo check`라는 명령도 있습니다. 이 명령은 코드가 컴파일되는지만
빠르게 확인하고, 실행 파일은 생성하지 않습니다:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

왜 실행 파일이 필요 없을까요? 보통 `cargo check`는 실행 파일 생성 단계를 생략하기
때문에 `cargo build`보다 훨씬 빠릅니다. 코드를 작성하면서 지속적으로 확인하려면
`cargo check`를 사용해 프로젝트가 여전히 컴파일되는지 빠르게 피드백을 받을 수
있습니다. 그래서 많은 Rustacean이 코드를 쓰는 동안 주기적으로 `cargo check`를
실행하고, 실행 파일이 필요할 때 `cargo build`를 사용합니다.

지금까지 배운 Cargo의 핵심을 정리하면 다음과 같습니다:

- `cargo new`로 프로젝트를 생성할 수 있습니다.
- `cargo build`로 프로젝트를 빌드할 수 있습니다.
- `cargo run`으로 빌드와 실행을 한 번에 할 수 있습니다.
- `cargo check`로 실행 파일을 만들지 않고 오류만 빠르게 확인할 수 있습니다.
- Cargo는 빌드 결과를 코드와 같은 디렉터리가 아니라 _target/debug_ 에 저장합니다.

Cargo의 또 다른 장점은, 사용하는 운영체제와 관계없이 명령이 동일하다는 점입니다.
따라서 이제부터는 Linux와 macOS, Windows 간의 세부 지침을 별도로 제공하지
않겠습니다.

### 릴리스 빌드

프로젝트를 배포할 준비가 되면 `cargo build --release`로 최적화를 적용한 상태로
컴파일할 수 있습니다. 이 명령은 _target/debug_ 대신 _target/release_ 에 실행
파일을 생성합니다. 최적화를 적용하면 Rust 코드는 더 빠르게 실행되지만, 그만큼
컴파일 시간은 길어집니다. 그래서 개발용 프로필과 배포용 프로필이 구분되어
있습니다. 개발 중에는 자주 빠르게 빌드하는 것이 중요하고, 배포용 최종 프로그램은
가능한 한 빠르게 실행되는 것이 중요하기 때문입니다. 실행 시간 벤치마크를 할 때는
반드시 `cargo build --release`로 빌드하여 _target/release_ 의 실행 파일로
벤치마크하세요.

### 관례로서의 Cargo

단순한 프로젝트에서는 `rustc`만으로도 충분할 수 있지만, 프로그램이 복잡해질수록
Cargo의 가치가 커집니다. 파일이 여러 개로 늘어나거나 의존성이 필요해지는 순간,
빌드를 Cargo에 맡기는 편이 훨씬 수월합니다.

`hello_cargo` 프로젝트는 단순하지만, 이제 여러분은 Rust 경력 전반에서 사용하게 될
진짜 도구 체인의 큰 부분을 경험했습니다. 기존 프로젝트를 작업할 때는 다음 명령으로
Git에서 코드를 내려받고, 디렉터리를 이동한 뒤, 빌드할 수 있습니다:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Cargo에 대한 더 자세한 정보는 [공식 문서][cargo]를 참고하세요.

## 요약

여러분의 Rust 여정은 이미 훌륭하게 시작되었습니다! 이 장에서 여러분은 다음을
학습했습니다:

- `rustup`으로 최신 안정 버전의 Rust를 설치하는 방법
- 새 Rust 버전으로 업데이트하는 방법
- 로컬에 설치된 문서를 여는 방법
- `rustc`로 직접 “Hello, world!” 프로그램을 작성하고 실행하는 방법
- Cargo 관례를 따르는 새 프로젝트를 만들고 실행하는 방법

이제 실제로 Rust 코드를 읽고 쓰는 데 익숙해지기 위해, 좀 더 알찬 프로그램을
만들어 보기에 좋은 시점입니다. 2장에서는 숫자 맞추기 게임을 만들어 보겠습니다.
공통 프로그래밍 개념이 Rust에서 어떻게 동작하는지부터 보고 싶다면, 3장을 먼저
읽고 2장으로 돌아와 프로젝트를 진행해도 좋습니다.

[installation]: ch01-01-installation.html#installation
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
