## Cargo 작업공간

12 장에서 바이너리 크레이트와 라이브러리 크레이트를 포함하는 패키지를 만들어
봤습니다. 하지만 여러분이 프로젝트를 개발하다 보면, 라이브러리 크레이트가 점점
거대해져서 여러분의 패키지를 여러개의 라이브러리 크레이트로 분리하고 싶으실
겁니다. Cargo 는 이런 상황에서 사용할 수 있는 *작업공간(workspace)* 이라는
기능을 제공하며, 이 기능은 함께 개발된 여러개의 관련된 패키지를 관리하는데
도움이 됩니다.

### 작업공간 생성

*작업공간(workspace)* 은 동일한 *Cargo.lock* 과 출력 디렉토리를 공유하는
패키지들의 집합입니다. 한번 이 작업공간을 이용한 프로젝트를 만들어 봅시다.
다만 작업공간의 구조에 집중할 수 있도록 간단한 코드만 사용할 겁니다.
작업공간을 구성하는 방법은 여러가지가 있지만, 일반적인 방법중 하나를
사용하도록 하겠습니다; 작업 공간은 바이너리 하나와 두 라이브러리를 포함하도록
할 것입니다. 주요 기능을 제공할 바이너리는 두 라이브러리를 의존성으로 가지게
될 것인데, 하나는 `add_one` 함수를 제공할 것이고, 또 하나는 `add_two` 함수를
제공할 것입니다. 이 세 크레이트는 같은 작업 공간의 일부가 될 겁니다.
그럼 작업공간을 위한 새 디렉토리를 만드는 것 부터 시작합시다.

```text
$ mkdir add
$ cd add
```

다음은 *add* 디렉토리 내에서 전체 작업공간을 구성 할 *Cargo.toml* 파일을
생성합시다. 이 파일은 우리가 여태 다른곳에서 봐온 *Cargo.toml* 파일들과는 달리,
`[package]` 절이나 메타데이터를 가지지 않습니다. 대신 `[workspace]` 로
시작하는 구절을 갖는데, 이걸 이용해 작업공간에 members 를 추가할 수 있습니다;
추가하는 법은 우리의 바이너리 크레이트 경로를 명시하는 것이며, 이 경우 해당 경로는 *adder* 입니다:

<span class="filename">Filename: Cargo.toml</span>

```toml
[workspace]

members = [
    "adder",
]
```

다음으로, *add* 디렉토리 안에서 `cargo new` 를 실행하여
`adder` 바이너리 크레이트를 생성합시다:

```text
$ cargo new --bin adder
     Created binary (application) `adder` project
```

이 시점에서 우린 작업 공간을 `cargo build` 로 빌드할 수 있습니다.
현재 여러분의 *add* 디렉토리의 내부 모습은 다음과 같은 형태여야 하니, 비교해 보시기 바랍니다:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

작업공간은 최상위 디렉토리에 컴파일된 결과를 배치하기 위한 하나의 *target*
디렉토리를 가집니다; 따라서 `adder` 크레이트는 자신만의 *target* 디렉토리를
갖지 않습니다. 만약 *adder* 디렉토리 내에서 `cargo build` 명령어를
실행하더라도 컴파일 결과는 *add/adder/target* 이 아닌 *add/target* 에 위치하게
될 겁니다. Cargo 가 작업공간 내에 이와 같이 *target* 디렉토리를 구성한 이유는,
작업공간 내의 크레이트들이 서로 의존하기로 되어있기 때문입니다. 만약 각 크레이트가
각각의 *target* 디렉토리를 갖게 된다면, 각각의 크레이트를 컴파일 할때마다
자신의 *target* 디렉토리에 컴파일 결과를 넣기 위해 다른 크레이트들을 매번
재컴파일 하게 될 겁니다. 이와 같은 불필요한 재빌드를 피하기 위해,
하나의 크레이트들은 *target* 디렉토리를 공유하도록 되어 있습니다.

### 작업공간에 두번째 크레이트 만들기

다음은 작업공간에 `add-one` 이라고 부를 새로운 멤버 크레이트를 생성해 봅시다.
`members` 목록에 *add-one* 경로를 지정하기 위해
최상위의 *Cargo.toml* 파일을 수정합시다.

<span class="filename">Filename: Cargo.toml</span>

```toml
[workspace]

members = [
    "adder",
    "add-one",
]
```

그리고 `add-one` 이라는 새 라이브러리 크레이트를 생성합시다.

```text
$ cargo new add-one
     Created library `add-one` project
```

이제 여러분의 *add* 디렉토리는 다음과 같은 디렉토리와 파일들을 갖게 될 겁니다:

```text
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

*add-one/src/lib.rs* 파일에 `add_one` 함수를 추가합시다:

<span class="filename">Filename: add-one/src/lib.rs</span>

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

이제 우린 작업공간 내에 라이브러리 크레이트를 가졌으니, `adder` 바이너리
크레이트를 `add-one` 라이브러리 크레이트에 의존하도록 만들 수 있습니다.
먼저, *adder/Cargo.toml* 에 `add-one` 에 대한 의존성 경로를 추가합시다.

<span class="filename">Filename: adder/Cargo.toml</span>

```toml
[dependencies]

add-one = { path = "../add-one" }
```

Cargo 는 작업공간 내 크레이트들이 서로 의존하고 있을 것이라고 추정하지 않기
때문에, 우리가 크레이트간의 의존 관계에 대해 명시해 주어야 합니다.

다음으로 `adder` 크레이트에서 `add-one` 크레이트의 `add_one` 함수를
사용해보도록 합시다. *adder/src/main.rs* 파일을 열고 상단에 `extern crate`
행을 추가해 스코프 내로 `add-one` 라이브러리를 가져오도록 한 뒤,
`main` 함수를 `add_one` 함수를 호출하도록 변경합니다. Listing 14-7 처럼요:

<span class="filename">Filename: adder/src/main.rs</span>

```rust,ignore
extern crate add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```

<span class="caption">Listing 14-7: `adder` 크레이트에서
`add-one` 라이브러리 사용하기</span>

이제 한번 최상위 *add* 디렉토리에서 `cargo build` 를 실행해
작업공간을 빌드해 봅시다!

```text
$ cargo build
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68 secs
```

*add* 디렉토리에서 바이너리 크레이트를 실행하기 위해선
`cargo run` 에 `-p` 옵션과 패키지 이름을 사용하여
우리가 작업공간 내에서 사용할 패키지를 명시해야 합니다:

```text
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

이는 `add-one` 크레이트에 의존성을 가진 *adder/src/main.rs* 코드를 실행시킵니다.

#### 작업공간의 외부 크레이트에 의존성 갖기

작업공간은 작업공간에 있는 각각의 크레이트의 디렉토리에 *Cargo.lock* 파일을
갖는게 아닌, 작업공간의 최상위에만 단 하나의 *Cargo.lock* 파일을 갖는다는 걸
기억하세요. 이는 모든 크레이트들이 모든 의존성의 같은 버전을 사용함을
보증합니다. 만약 우리가 `rand` 크레이트를 *adder/Cargo.toml* 과
*add-one/Cargo.toml* 에 추가하면 Cargo 는 둘을 모두 같은 버전을 쓰도록
결정하고 하나의 *Cargo.lock* 에 기록합니다. 작업공간의 모든 크레이트들이 같은
의존성을 갖도록 한다는 의미는 작업공간 내의 크레이트들이 항상 서로 조화를
이룬다는 의미입니다. 한번 `add-one` 크레이트에서 `rand` 크레이트를 사용할 수
있도록 *add-one/Cargo.toml* 파일의 `[dependencies]` 절에 `rand` 를 추가해
봅시다:

<span class="filename">Filename: add-one/Cargo.toml</span>

```toml
[dependencies]

rand = "0.3.14"
```

이제 우린 *add-one/src/lib.rs* 파일에 `extern crate rand;` 를 추가할 수 있으며,
*add* 디렉토리에서 `cargo build` 를 이용해 전체 작업공간을 빌드하면
`rand` 크레이트를 가져오고 컴파일 할 것입니다:

```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
   --snip--
   Compiling rand v0.3.14
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18 secs
```

이제 최상위 *Cargo.lock* 엔 `add-one` 의 `rand` 로의 종속성 정보가
포함되어 있습니다. 하지만, 작업공간의 어딘가에서 `rand` 를 사용하였다고 해도,
작업공간의 다른 크레이트에선 `rand` 를 자신의 *Cargo.toml* 파일에 추가하지
않는 한 사용이 불가능합니다. 예를 들어, 만약 `adder` 크레이트에서 `rand` 를
그냥 사용하기 위해 *adder/src/main.rs* 파일에 `extern crate rand;` 를 추가하면
에러가 나타납니다:

```text
$ cargo build
   Compiling adder v0.1.0 (file:///projects/add/adder)
error: use of unstable library feature 'rand': use `rand` from crates.io (see
issue #27703)
 --> adder/src/main.rs:1:1
  |
1 | extern crate rand;
```

이 에러를 해결하려면 `adder` 크레이트의 *Cargo.toml* 파일을 수정하여 `rand` 를
해당 크레이트의 의존성으로 나타내야합니다. 그 후 `adder` 크레이트를 빌드하면
*Cargo.lock* 의 `adder` 을 위한 의존성 목록에 `rand` 가 추가될 테지만, `rand`
가 다시 다운로드 되진 않을 겁니다.
Cargo 는 `rand` 를 사용하는 작업공간 내의 크레이트는 모두 같은 버전의 `rand`
크레이트를 사용할 것임을 보장하기 때문에 같은 크레이트를 여러개의 버전으로
다운로드 받을 필요 없고, 따라서 그만큼 공간은 절약되며, 작업공간 내의 각
크레이트는 조화를 이룰 수 있습니다.

#### 작업공간에 테스트 추가하기

또 다른 향상을 위해, `add_one` 크레이트의
`add_one::add_one` 함수에 대한 테스트를 추가해 봅시다.

<span class="filename">Filename: add-one/src/lib.rs</span>

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

이제 최상위 *add* 디렉토리에서 `cargo test` 를 실행해 봅시다:

```text
$ cargo test
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running target/debug/deps/add_one-f0253159197f7841

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/adder-f88af9d2cc175a5e

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

출력의 첫번째 절 은 `add-one` 크레이트의 `it_works` 테스트가 통과했다는
의미이고, 다음 절 은 `adder` 크레이트에서 테스트를 찾지 못했다는 의미이며,
마지막 절 은 `add-one` 크레이트에서 문서화 테스트를 찾지 못했다는 의미입니다.
이처럼 작업공간 구조 내에서 `cargo test` 를 실행하면 작업공간 내의 모든
크레이트에 대한 테스트들이 실행됩니다.

우린 작업공간 내의 하나의 특정한 크레이트에 대한 테스트도 실행할 수 있습니다.
최상위 디렉토리에서 `-p` 플래그와 테스트 하고자 하는
크레이트명을 명시해줌으로써 말이죠:

```text
$ cargo test -p add-one
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/add_one-b3235fea9a156f74

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

이 출력은 `cargo test` 가 `adder` 크레이트는 테스트하지 않고
`add-one` 크레이트에 대해서만 테스트를 실행 했음을 보여줍니다.

만약 여러분이 *https://crates.io/* 에 작업공간 내의 크레이트를 배포하시려면,
각 크레이트는 분리돼서 배포되어야 합니다.
`cargo publish` 명령어엔 `--all` 이나 `-p` 같은 플래그가 없어요.
따라서 여러분은 각 크레이트 디렉토리를 수정하고
`cargo publish` 를 실행해야 합니다.

추가 과제로는, 한번 이 작업공간에 `add-two` 크레이트를 추가해 보세요!
`add-one` 크레이트를 추가할때와 비슷한 방법으로 하시면 됩니다.

언젠가 여러분의 프로젝트가 커지면 작업공간을 사용하는 걸 고려해보세요:
하나의 거대한 코드보다 작은 개별 요소를 이해하는 일이 훨씬 쉽고,
작업공간에서 크레이트를 관리한다면 각 크레이트가 동시에 변경되는 경우도
쉽게 조정할 수 있습니다.
