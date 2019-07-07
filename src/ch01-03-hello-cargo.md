## Cargo 를 사용해봅시다

Cargo 는 러스트 사용자라면 대부분 사용하는 러스트 빌드 시스템 및
패키지 매니저입니다. 이 툴은 코드 빌드나, 코드 작성에 필요한 외부 라이브러리를
다운할때나, 라이브러리를 제작할때 겪는 귀찮은 일들을 상당수 줄여주는 편리한 툴입니다.
(앞으로 외부 라이브러리는 *의존성(dependencies)* 이라고 지칭하겠습니다.)

여태 우리가 작성해온 간단한 러스트 프로그램에선 의존성을 추가하지 않았습니다.
Hello, world! 프로젝트를 만들면서도 Cargo 기능 중 코드를 생성하는 기능만 사용했었죠.
하지만 훗날 복잡한 프로그램을 작성하게 되면 이야기가 다를 겁니다.
그렇더라도, 프로젝트를 생성할 때부터 Cargo 를 이용하면
의존성을 추가할 일이 생겨도 간단히 해결할 수 있을 겁니다.

러스트 사용자 대부분이 사용하는 툴인 만큼,
이후 내용도 여러분이 Cargo 를 사용한다는 전제로 작성했습니다.
["러스트 설치"][installation]<!-- ignore --> 절을 따라 하셨다면
이미 Cargo 가 설치돼있을 테니 따로 설치하실 필요는 없으나,
다른 방법을 이용하신 경우엔 다음 명령어로
Cargo 가 설치돼있는지 확인하시기 바랍니다:

```text
$ cargo --version
```

버전 숫자가 나타나면 정상입니다.
`command not found` 등 에러가 나타날 경우 여러분이 설치하면서
참고한 문서에서 Cargo 를 따로 설치하는 방법을 찾아보세요.

### Cargo 로 프로젝트 생성하기

Cargo 로 프로젝트를 생성해보고
우리가 앞서 만들었던 Hello, world! 프로젝트와 비교해봅시다.
*projects* 디렉토리로 (다른 곳에 코드를 만드신 분은 해당 위치로)
돌아가 다음 명령어를 실행해보세요.

```text
$ cargo new hello_cargo
$ cd hello_cargo
```

첫번째 명령어는 *hello_cargo* 라는 프로젝트를 생성하겠단 의미입니다.
이 명령어를 실행하면 Cargo 는 *hello_cargo* 디렉토리를 생성하고
해당 디렉토리 내에 프로젝트명과 동일한 이름의 파일을 생성합니다.

*hello_cargo* 디렉토리로 이동해 파일을 살펴보면
*Cargo.toml* 파일과 *src* 디렉토리를 확인할 수 있습니다.
그 외에도 *.gitignore* 파일과 초기화된 새 Git 저장소를 확인할 수 있으며,
*src* 디렉토리 내에는 *main.rs* 파일이 있는 것도 볼 수 있습니다.

> Note: Git 은 일반적으로 사용하는 버전 관리 시스템입니다. 따라서 기본 설정되어 있으며,
> 이 설정은 `cargo new` 명령어의 `--vcs` 플래그로 변경 가능합니다.
> 그 외의 다른 옵션들은 `cargo new --help` 로 확인할 수 있습니다.

이제 텍스트 에디터로 *Cargo.toml* 을 열어보세요.
Listing 1-2 처럼 나오면 정상입니다.

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

<span class="caption">Listing 1-2: `cargo new` 로 생성한
*Cargo.toml* 파일의 내용</span>

이 포맷은 [*TOML*][toml]<!-- ignore --> (*Tom’s Obvious, Minimal Language*) 포맷으로,
Cargo 설정에서 사용하는 포맷입니다.

[toml]: https://github.com/toml-lang/toml/blob/master/versions/ko/toml-v0.5.0.md

`[package]` 라 적힌 첫 줄은 부문 제목으로,
뒤에 패키지 설정 구문들이 따라오는 걸 보실 수 있습니다.
나중에 우리가 이 파일에 내용을 추가하며 새로운 부문을 만들어 볼 겁니다.

다음 4줄은 Cargo 가 코드를 컴파일하는데 필요한 설정 정보로,
각각 패키지명, 버전, 작성자, 사용하는 러스트 에디션을 나타냅니다.
작성자 정보에 포함되는 이름, 이메일 등은 여러분 환경에서
자동으로 가져오니 잘못된 정보는 수정 후 저장하시기 바랍니다.
러스트 에디션은 부록 E에서 자세히 설명하니 참고하세요.

마지막 `[dependencies]` 는 프로젝트에서 사용하는 의존성 목록입니다.
러스트에선 의존성 코드 패키지들을 *crates* 라 부르며,
이번 프로젝트에는 크레이트가 필요 없는 관계로
2장 첫 프로젝트에서 사용할 예정입니다.

이제 *src/main.rs* 를 확인해봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo 가 Hello, world! 프로그램을 만들어 놨네요.
Listing 1-1 에서 만든 프로젝트와 다른 점은
이번엔 코드 위치가 *src* 디렉토리라는 점과
최상위 디렉토리에 *Cargo.toml* 설정 파일이 존재한다는 점입니다.

Cargo 는 최상위 프로젝트 디렉토리를 README, 라이센스, 설정 파일 등
코드 자체와는 관련 없는 파일들을 저장하는 데 사용하기 때문에,
소스 파일은 *src* 내에 저장합니다.
이처럼 Cargo 는 각각의 파일을 알맞은 위치에 배치하여
여러분이 프로젝트를 조직화하는 걸 돕습니다.

추가로, 프로젝트 생성 시 Cargo 를 사용하지 않았어도
*Cargo.toml* 파일을 알맞게 작성하고 프로젝트 코드를
*src* 디렉토리로 옮기면 해당 프로젝트에서의
사용이 가능하게 만들 수도 있습니다.

### Cargo 로 프로젝트를 생성하고 실행하기

Cargo 로 생성한 Hello, world! 프로그램은
실행했을 때 어떤 점이 다른지 확인해봅시다!
*hello_cargo* 디렉토리에서 다음 명령어를 이용해 프로젝트를 빌드해주세요:

```text
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

이 명령어는 *target/debug/hello_cargo*
(Windows 에선 *target\debug\hello_cargo.exe*) 에 실행 파일을 생성합니다.
실행 파일은 다음 명령어로 실행할 수 있습니다:

```text
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

터미널에 `Hello, world!` 가 출력되면 제대로 진행된 겁니다.
처음 `cargo build` 명령어를 실행하면 최상위 디렉토리에
*Cargo.lock* 파일이 생성될 텐데, 이 파일은 프로젝트에서
사용하는 의존성들의 정확한 버전을 자동으로 기록해두는
파일이니 여러분이 직접 수정할 필요는 없습니다.
물론 이번 프로젝트는 의존성을 갖지 않으므로 현재는 파일에 특별한 내용이 없습니다.

방금은 `cargo build` 로 빌드한 후
`./target/debug/hello_cargo` 명령어로 실행했지만,
컴파일과 실행을 한 번에 진행하는 `cargo run` 명령어도 있습니다:

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

출력 내용에 `hello cargo` 를 컴파일 중이라는 내용이 없는 걸 눈치채셨나요?
이는 Cargo 가 파일 변경 사항이 없음을 알아채고 기존 바이너리를 그대로 실행했기 때문입니다.
소스 코드를 수정한 뒤 명령어를 다시 실행해보면 다음과 같이
프로젝트를 다시 빌드한 후에 바이너리를 실행함을 알 수 있습니다.

```text
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

`cargo check` 라는 명령어도 존재하는데, 이 명령어는 실행 파일은 생성하지 않은 채,
작성한 소스가 문제없이 컴파일되는지만 빠르게 확인하는 명령어입니다.

```text
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

실행 파일도 생성하지 않는 명령어가 왜 필요할까요? 이유는 바로,
코드를 작성하는 도중에 문제가 없는지 중간중간 확인하는 용도로 사용하는 명령어이기 때문입니다.
아직 코드를 완성하지 않았으니 실행 파일을 만들 필요가 없고, 따라서 실행 파일을 생성하는 단계를 건너뜀으로써 `cargo build` 보다 빠른 속도를 얻는 것이죠.

러스트 사용자는 대부분 주기적으로 이 명령어를 실행해 코드에서 컴파일 문제가
발생하지 않는지 확인하고 실행 파일이 필요할 경우에만 `cargo build` 를 사용합니다.

이제, 여태 배운 내용을 복습해봅시다:

* 프로젝트 빌드 관련 명령어는 `cargo build` 와 `cargo check` 가 존재합니다.
* `cargo run` 명령어는 한번에 프로젝트를 빌드하고 실행할 수 있습니다.
* 빌드로 만들어진 파일은 우리가 작성한 소스 코드와 뒤섞이지 않도록
  *target/debug* 디렉토리에 저장됩니다.

운영체제에 상관 없이 같은 명령어를 사용한다는 것도 Cargo 사용으로 얻는 장점입니다.

같은 이유로, 앞으로는 운영체제 별로 명령어를 따로 알려드리지 않습니다.

### 릴리즈 빌드 생성

프로젝트를 완성해서 배포(릴리즈)할 준비가 끝났다면, `cargo build --release` 명령어를 사용해 릴리즈 빌드를 생성할 수 있습니다.
일반 빌드와 차이점은 *target/debug* 가 아닌 *target/release* 에 실행 파일이 생성된다는 점과
컴파일 시 최적화를 진행해, 컴파일이 오래 걸리는 대신 러스트 코드가 더 빠르게 작동하는 점입니다.

릴리즈 빌드가 더 빠르게 작동한다면, 왜 일반 빌드시에는 최적화를 진행하지 않을까요?
이에 대한 해답은 빌드가 두 종류로 나뉘게 된 이유이기도 한데, 개발 중에는 빌드가 잦으며 작업의 흐름을 끊지 않기 위해 빌드 속도 또한 빠를수록 좋은 반면,
배포용 프로그램은 잦은 빌드가 필요 없으며 빌드 속도보단 프로그램의 작동 속도가 더 중요하기 때문입니다.

이와 같은 이유로, 작성한 코드 작동 속도를 벤치마킹할 시에는 릴리즈 빌드를
기준으로 해야 한다는 것도 알아두시기 바랍니다.

### Cargo 사용법

`hello_cargo` 프로젝트는 단순하지만, 이미 여러분은 앞으로 러스트를 사용하며 쓸 Cargo 명령어 중 대부분을 써본 것과 다름없습니다.

예시로, 기존에 있던 러스트 프로젝트에서 작업하는 데 필요한 준비 과정은 다음과 같습니다.

```text
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

각각 차례대로 Git 으로 코드를 가져오고, 프로젝트 디렉토리로 이동하고, 빌드하는 명령어입니다. 앞서 여러분이 해본 것과 비슷하지 않나요?

Cargo 와 `rustc` 에서 큰 차이를 느끼지 못한 분도 있을 겁니다.
하지만 Cargo 를 사용함으로써 얻는 장점은 단순한 프로젝트가 아닌,
프로젝트에 여러 크레이트가 추가되고 코드가 복잡해질수록, 즉 복잡한 프로젝트일수록 극대화됩니다.

더 자세한 내용은 [The Cargo Book (영문)] 에서 읽어보세요!

[its documentation]: https://doc.rust-lang.org/cargo/

## 요약

이번 장에서 배운 내용은
다음과 같습니다:

* `rustup` 으로 최신 stable 버전 러스트를 설치하는 방법
* 러스트를 새 버전으로 업데이트하는 방법
* 로컬 설치된 문서 열어보기
* 직접 `rustc` 를 사용해 `Hello, world!` 프로그램을 작성하고 실행해보기
* Cargo 로 프로젝트를 생성하고 실행하는 방법

코드를 읽고 쓰는데 익숙해질 수 있도록 프로그램을 몇 번 더 생성해보셔도 좋습니다.
다음 장은 추리 게임 프로그램을 만들어보는 내용이니,
러스트에서 사용되는 보편적인 프로그래밍 개념부터 살펴보실 분들은
3장부터 읽고 2장을 읽는 것도 나쁘지 않습니다.

[installation]: ch01-01-installation.html#러스트-설치
