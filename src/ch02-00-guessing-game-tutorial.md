# 숫자 맞추기 게임 만들기

이번에는 실습 프로젝트를 통해 Rust에 본격적으로 뛰어들어 보겠습니다!  
이 장에서는 실제 프로그램 안에서 몇 가지 흔한 Rust 개념을 사용하는 방법을
보여 드립니다. `let`, `match`, 메서드, 연관 함수(associated functions), 외부
크레이트 등 다양한 개념을 만나 보실 거예요. 이어지는 장들에서 이 아이디어들을
더 자세히 살펴볼 것이고, 이 장에서는 기초를 연습하는 데 집중합니다.

우리가 구현할 내용은 입문자용으로 유명한 숫자 맞추기 게임입니다. 동작 방식은
이렇습니다: 프로그램이 1부터 100 사이의 무작위 정수를 하나 생성합니다. 그런
다음 플레이어에게 추측 값을 입력하라고 요구합니다. 값을 입력하면, 프로그램은
그 값이 너무 작았는지, 너무 컸는지 알려 줍니다. 값이 정확하면 축하 메시지를
출력하고 게임을 종료합니다.

## 새 프로젝트 준비하기

새 프로젝트를 만들려면, 1장에서 만들었던 _projects_ 디렉터리로 이동한 다음,
Cargo로 새 프로젝트를 생성하세요:

```console
$ cargo new guessing_game
$ cd guessing_game
```

첫 번째 명령 `cargo new`는 프로젝트 이름(`guessing_game`)을 첫 번째 인자로
받습니다. 두 번째 명령은 새 프로젝트의 디렉터리로 이동합니다.

생성된 _Cargo.toml_ 파일을 확인해 보세요:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

1장에서 보았듯이, `cargo new`는 “Hello, world!” 프로그램을 자동으로 생성합니다.  
_src/main.rs_ 파일을 확인해 보세요:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

이제 `cargo run` 명령으로 컴파일과 실행을 한 번에 해 보겠습니다:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

`run` 명령은 이번 게임처럼 빠르게 반복 개발(iteration)할 때 유용합니다. 각
단계를 빠르게 시험해 보고 다음으로 넘어갈 수 있습니다.

_src/main.rs_ 파일을 다시 여세요. 이 장에서 작성하는 코드는 모두 이 파일에
넣습니다.

## 추측값 처리하기

게임의 첫 부분은 사용자 입력을 받고, 그 입력을 처리하며, 입력 형식이 예상과
맞는지 확인합니다. 먼저 플레이어가 추측값을 입력할 수 있도록 하겠습니다.
Listing 2-1의 코드를 _src/main.rs_ 에 입력하세요.

<Listing number="2-1" file-name="src/main.rs" caption="사용자로부터 추측값을 입력받아 출력하는 코드">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing>

이 코드는 많은 정보를 담고 있으니 한 줄씩 살펴보겠습니다. 사용자 입력을 받아
출력하려면 입출력 라이브러리 `io`를 스코프로 가져와야 합니다. `io` 라이브러리는
표준 라이브러리 `std`에서 제공합니다:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Rust는 기본적으로 모든 프로그램의 스코프로 가져오는 표준 라이브러리 항목
모음을 갖고 있으며, 이를 _prelude(프렐루드)_ 라고 부릅니다. 프렐루드의 목록은
[표준 라이브러리 문서][prelude]에서 확인할 수 있습니다.

사용하려는 타입이 프렐루드에 없다면, `use` 구문으로 명시적으로 스코프로
가져와야 합니다. `std::io` 라이브러리를 사용하면 사용자 입력을 받을 수 있는
기능 등 여러 유용한 기능을 쓸 수 있습니다.

1장에서 보았듯이 `main` 함수는 프로그램의 진입점입니다:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` 구문은 새 함수를 선언합니다. 괄호 `()`는 매개변수가 없다는 뜻이고,
중괄호 `{`는 함수 본문의 시작을 뜻합니다.

1장에서 배웠듯이, `println!`은 화면에 문자열을 출력하는 매크로입니다:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

이 코드는 게임이 무엇인지 안내하고 사용자 입력을 요청하는 프롬프트를 출력합니다.

### 변수에 값 저장하기

다음으로 사용자 입력을 저장할 _변수_ 를 만듭니다:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

이제 흥미로워집니다! 짧은 줄에 많은 일이 일어납니다. `let` 구문으로 변수를
만듭니다. 예를 하나 더 보겠습니다:

```rust,ignore
let apples = 5;
```

이 줄은 `apples`라는 새 변수를 만들고 값 5를 바인딩합니다. Rust에서 변수는
기본적으로 불변입니다. 즉, 한 번 값을 주면 그 값은 바뀌지 않습니다. 이 개념은
3장의 [“변수와 가변성”][variables-and-mutability] 절에서 자세히 다룹니다.
변수를 가변으로 만들려면 변수 이름 앞에 `mut`를 붙입니다:

```rust,ignore
let apples = 5; // 불변
let mut bananas = 5; // 가변
```

> 참고: `//` 뒤부터 줄 끝까지는 주석입니다. Rust는 주석의 내용을 무시합니다.
> 주석에 대해서는 3장 [“주석”][comments]에서 더 설명합니다.

숫자 맞추기 코드로 돌아가면, `let mut guess`는 `guess`라는 가변 변수를
도입합니다. 등호(`=`)는 지금 그 변수에 무언가를 바인딩하겠다는 뜻입니다.
오른쪽에는 `String::new` 호출 결과가 있으며, 이는 새로운 `String` 인스턴스를
반환합니다. [`String`][string]은 표준 라이브러리가 제공하는 가변 길이의
UTF-8 문자열 타입입니다.

`::` 구문은 `new`가 `String` 타입의 _연관 함수(associated function)_ 임을
나타냅니다. 연관 함수는 어떤 타입에 구현된 함수입니다. `new`는 새로운, 비어
있는 문자열을 만듭니다. 새로운 값을 생성하는 함수 이름으로 `new`를 쓰는 경우가
많습니다.

정리하면, `let mut guess = String::new();`는 현재 비어 있는 새 `String`
인스턴스를 가리키는 가변 변수를 만든 것입니다.

### 사용자 입력 받기

프로그램 첫 줄에서 `use std::io;`로 표준 라이브러리의 입출력 기능을 가져온
것을 기억하세요. 이제 `io` 모듈의 `stdin` 함수를 호출해 사용자 입력을
다루겠습니다:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

프로그램 시작 부분에서 `use std::io;`를 하지 않았다면 `std::io::stdin`처럼
완전한 경로로도 호출할 수 있습니다. `stdin`은
[`std::io::Stdin`][iostdin] 인스턴스를 반환하는데, 이는 터미널의 표준 입력을
나타내는 핸들입니다.

다음 줄 `.read_line(&mut guess)`는 표준 입력 핸들에서 [`read_line`][read_line]
메서드를 호출해 사용자 입력을 받습니다. `read_line`이 입력을 어디에 저장할지
알 수 있도록 인자로 `&mut guess`를 넘깁니다. `read_line`은 사용자가 입력한
내용을 문자열에 _덧붙이기_ 때문에(기존 내용을 덮어쓰지 않음) 그 문자열을
인자로 전달합니다. 문자열 내용을 바꾸려면 가변이어야 하므로 `&mut`가 필요합니다.

`&`는 이 인자가 _참조(reference)_ 임을 나타냅니다. 참조는 데이터를 여러 곳에서
복사 없이 접근할 수 있게 해 줍니다. 참조는 복잡한 주제지만, Rust는 참조를
안전하고 쉽게 사용할 수 있게 해 줍니다. 지금은 변수와 마찬가지로 참조도
기본은 불변이므로, 가변 참조가 필요할 때는 `&guess`가 아니라 `&mut guess`라고
써야 한다는 점만 기억하세요. (4장에서 자세히 다룹니다.)

<!-- Old heading. Do not remove or links may break. -->

<a id="handling-potential-failure-with-the-result-type"></a>

### `Result`로 실패 가능성 다루기

여전히 같은 줄을 다루고 있습니다. 긴 한 줄을 읽기 좋게 나눈 것뿐입니다. 다음은
이 메서드입니다:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

이 코드는 이렇게 한 줄로도 쓸 수 있습니다:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

하지만 너무 긴 줄은 읽기 불편하므로, `.method_name()` 체이닝을 사용할 때는
적절히 줄바꿈과 공백을 넣는 것이 좋습니다. 이제 이 줄이 하는 일을 설명하겠습니다.

앞서 언급했듯이 `read_line`은 우리가 넘긴 문자열에 사용자가 입력한 내용을
넣어주지만, 동시에 `Result` 값을 반환합니다. [`Result`][result]는 흔히 _열거형_
이라고 부르는 [_enumeration_][enums] 타입으로, 여러 가능한 상태 중 하나를
가질 수 있습니다. 각각의 가능한 상태를 _변형(variant)_ 이라고 합니다.

`Result`의 변형은 `Ok`와 `Err`입니다. `Ok`는 연산이 성공했음을 나타내며,
성공적으로 생성된 값을 담습니다. `Err`는 연산이 실패했음을 나타내며, 실패한
이유를 담습니다.

모든 타입과 마찬가지로 `Result` 값에도 메서드가 정의되어 있습니다. `Result`
인스턴스에는 [`expect` 메서드][expect]가 있어 호출할 수 있습니다. 만약
`Result`가 `Err`라면 `expect`는 프로그램을 중단(panic)시키고 우리가 넘긴
메시지를 출력합니다. `read_line`이 `Err`를 반환한다면 보통 운영체제에서
발생한 오류 때문일 것입니다. 반대로 `Result`가 `Ok`라면, `expect`는 `Ok`가
담고 있는 값을 꺼내 반환합니다. 이 경우 사용자가 입력한 바이트 수가 됩니다.

`expect`를 호출하지 않으면 컴파일은 되지만 경고가 발생합니다:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

이는 `read_line`이 반환한 `Result` 값을 사용하지 않았다는 경고로, 잠재적 오류를
처리하지 않았음을 의미합니다.

경고를 없애는 올바른 방법은 실제로 오류를 처리하는 코드를 작성하는 것입니다.
하지만 여기서는 문제가 생기면 프로그램을 바로 중단시키고자 하므로 `expect`를
사용합니다. 오류에서 복구하는 방법은 9장 [“`Result`로 복구 가능한 오류 처리”][recover]
에서 다룹니다.

### `println!` 자리 표시자로 값 출력하기

닫는 중괄호를 제외하면, 지금까지 코드에서 논의할 줄이 하나 더 있습니다:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

이 줄은 이제 사용자 입력이 담긴 문자열을 출력합니다. `{}` 중괄호는 자리
표시자입니다. `{}`를 값이 들어갈 자리를 표시하는 집게발로 생각해도 좋습니다.
변수의 값을 출력할 때는 변수 이름을 중괄호 안에 넣을 수 있습니다. 표현식의
결과를 출력할 때는 포맷 문자열에 빈 `{}`를 넣고, 그 뒤에 쉼표로 구분된
표현식들을 나열해 각 `{}`에 대응시킵니다. 변수와 표현식 결과를 한 번의
`println!`으로 출력하는 예는 다음과 같습니다:

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

이 코드는 `x = 5 and y + 2 = 12`를 출력합니다.

### 첫 부분 테스트하기

이제 첫 부분을 테스트해 봅시다. `cargo run`으로 실행하세요:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

이 시점에서 게임의 첫 부분이 완료되었습니다. 키보드에서 입력을 받고, 그 값을
출력하고 있습니다.

## 비밀 숫자 생성하기

이제 사용자가 맞힐 비밀 숫자를 생성해야 합니다. 매번 다른 숫자가 되어야 여러 번
플레이해도 재미있겠죠. 난이도를 적당히 맞추기 위해 1부터 100 사이의 임의
정수를 사용하겠습니다. Rust 표준 라이브러리에는 아직 난수 기능이 포함되어
있지 않습니다. 하지만 Rust 팀이 이 기능을 제공하는 [`rand` 크레이트][randcrate]를
제공하고 있습니다.

### 크레이트로 기능 확장하기

크레이트는 Rust 소스 코드 파일들의 모음입니다. 우리가 만드는 프로젝트는
_바이너리 크레이트_ 로, 실행 파일을 생성합니다. `rand` 크레이트는 _라이브러리
크레이트_ 로, 다른 프로그램에서 사용하도록 만든 코드 묶음이며 단독으로 실행할
수는 없습니다.

외부 크레이트 조정은 Cargo가 진가를 발휘하는 부분입니다. `rand`를 사용하려면
코드를 작성하기 전에 _Cargo.toml_ 파일을 수정해 `rand` 크레이트를 의존성으로
추가해야 합니다. 지금 파일을 열고, Cargo가 만들어 준 `[dependencies]` 섹션
헤더 아래 맨 아래 줄에 다음을 추가하세요. 여기 적힌 버전 번호를 포함해
정확히 `rand`를 지정해야 이 튜토리얼의 예제가 문제없이 동작합니다:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

_Cargo.toml_ 에서, 어떤 헤더 다음의 내용은 다음 섹션이 시작될 때까지 그 섹션의
구성에 속합니다. `[dependencies]` 섹션에서는 프로젝트가 어떤 외부 크레이트에
의존하는지와 그 버전을 지정합니다. 이 경우 우리는 `rand` 크레이트를
의미론적 버전(semantic version) `0.8.5`로 지정합니다. Cargo는 [Semantic
Versioning][semver](줄여서 _SemVer_)을 이해합니다. `0.8.5`는 사실 `^0.8.5`의
축약으로, 최소 0.8.5 이상이면서 0.9.0 미만의 어떤 버전이든 허용한다는 뜻입니다.

Cargo는 이런 버전들이 0.8.5와 호환되는 공개 API를 갖는다고 간주합니다. 따라서
이 지정은 이 장의 코드와 여전히 컴파일 가능한 최신 패치 릴리스를 사용하게
보장합니다. 0.9.0 이상은 아래 예제와 동일한 API를 보장하지 않습니다.

이제 코드를 바꾸지 않고 프로젝트를 빌드해 보겠습니다(Listing 2-2 참고).

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

<Listing number="2-2" caption="`rand`를 의존성으로 추가한 뒤 `cargo build` 실행 결과">

```console
$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```

</Listing>

여러분의 화면에서는 버전 번호나 출력 라인이 조금 다를 수 있습니다(운영체제에
따라서도 달라집니다). 하지만 SemVer 덕분에 코드는 호환될 겁니다.

외부 의존성을 추가하면, Cargo는 _레지스트리(registry)_ 에서 그 의존성이 필요로
하는 최신 버전들을 가져옵니다. 레지스트리는 [Crates.io][cratesio]의 데이터를
복제한 것입니다. Crates.io는 Rust 생태계의 사람들이 오픈 소스 Rust 프로젝트를
공유하는 곳입니다.

레지스트리를 갱신한 뒤, Cargo는 `[dependencies]` 섹션을 확인하고 아직
다운로드되지 않은 크레이트를 내려받습니다. 여기서는 `rand`만 추가했지만,
`rand`가 필요로 하는 다른 크레이트들도 함께 가져옵니다. 그런 다음 의존성들을
컴파일하고, 이들을 사용할 수 있도록 한 상태에서 프로젝트를 컴파일합니다.

바로 이어서 변경 없이 다시 `cargo build`를 실행하면 `Finished` 한 줄만 보일
겁니다. Cargo는 이미 의존성을 다운로드하고 컴파일했으며, _Cargo.toml_ 도
바뀌지 않았음을 압니다. 여러분의 코드도 바뀌지 않았으니 재컴파일하지 않습니다.
할 일이 없으면 그냥 종료합니다.

_src/main.rs_ 를 열어 사소한 변경을 한 뒤 저장하고 빌드하면, 출력은 다음처럼
두 줄만 보일 것입니다:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

이는 Cargo가 _src/main.rs_ 의 작은 변경만 반영하여 빌드를 갱신했음을 보여 줍니다.  
의존성은 바뀌지 않았으므로 다운로드/컴파일 결과를 재사용합니다.

#### _Cargo.lock_ 으로 재현 가능한 빌드 보장하기

Cargo에는 여러분이나 다른 누군가가 코드를 빌드할 때마다 동일한 결과물을
재현할 수 있게 해 주는 메커니즘이 있습니다. 여러분이 달리 지시하지 않는 한,
Cargo는 지정한 버전만 사용합니다. 예를 들어, 다음 주에 `rand` 0.8.6이 나왔다고
합시다. 중요한 버그가 고쳐졌지만, 여러분의 코드를 망가뜨리는 회귀(regression)
도 포함하고 있을 수 있습니다. 이를 다루기 위해 Rust는 `cargo build`를 처음
실행할 때 _Cargo.lock_ 파일을 생성하며, 이제 이 파일이 _guessing_game_
디렉터리에 있습니다.

처음 빌드할 때 Cargo는 조건에 맞는 의존성 버전을 모두 계산해 _Cargo.lock_ 에
기록합니다. 이후 빌드에서는 _Cargo.lock_ 이 존재함을 확인하고, 버전 계산을
다시 하지 않고 그 파일의 버전을 사용합니다. 이렇게 하면 자동으로 재현 가능한
빌드가 됩니다. 즉, 여러분이 명시적으로 업그레이드하기 전까지 프로젝트는
0.8.5에 머무릅니다. _Cargo.lock_ 은 재현 가능한 빌드에 중요하므로, 보통 다른
코드와 함께 버전 관리에 커밋합니다.

#### 크레이트를 새 버전으로 업데이트하기

업데이트 _하고 싶을 때_ 는 `update` 명령을 사용합니다. 이 명령은 _Cargo.lock_
을 무시하고, _Cargo.toml_ 의 제약에 맞는 최신 버전을 다시 계산해
_Cargo.lock_ 에 기록합니다. 이 경우 Cargo는 0.8.5보다 크고 0.9.0 미만의
버전만 찾습니다. `rand`가 0.8.6과 0.9.0을 새로 릴리스했다면, `cargo update`
실행 결과는 다음과 같을 것입니다:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.9.0)
```

Cargo는 0.9.0 릴리스를 무시합니다. 이 시점에서 _Cargo.lock_ 의 `rand` 버전이
0.8.6으로 바뀐 것을 확인할 수 있습니다. 0.9.0이나 0.9.x 시리즈를 사용하려면
_Cargo.toml_ 을 다음처럼 바꿔야 합니다:

```toml
[dependencies]
rand = "0.9.0"
```

다음에 `cargo build`를 실행하면, Cargo는 크레이트 레지스트리를 갱신하고 새
버전에 맞춰 `rand` 요구 사항을 재평가합니다.

[Cargo 자체][doccargo]와 [생태계][doccratesio]에 대해서는 14장에서 더 많이
다룰 예정입니다. 지금은 이 정도만 알면 충분합니다. Cargo 덕분에 라이브러리
재사용이 쉬워져, Rustacean들은 여러 패키지를 조합해 더 작은 단위로 프로젝트를
작성할 수 있습니다.

### 무작위 수 생성하기

이제 `rand`를 사용해 맞힐 숫자를 생성해 봅시다. _src/main.rs_ 를 Listing 2-3처럼
수정합니다.

<Listing number="2-3" file-name="src/main.rs" caption="무작위 수를 생성하는 코드 추가">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

먼저 `use rand::Rng;` 줄을 추가합니다. `Rng` 트레이트는 난수 생성기가 구현하는
메서드를 정의하며, 이 메서드를 사용하려면 트레이트가 스코프에 있어야 합니다.
트레이트는 10장에서 자세히 다룹니다.

다음으로 중간에 두 줄을 추가합니다. 첫 줄은 현재 스레드에 지역적이며 운영체제가
시드를 제공하는 난수 생성기를 반환하는 `rand::thread_rng` 함수를 호출합니다.
그런 다음 그 생성기에서 `gen_range` 메서드를 호출합니다. 이 메서드는 우리가
`use rand::Rng;`로 스코프에 가져온 `Rng` 트레이트에서 정의합니다. `gen_range`는
범위 표현식을 인자로 받아 그 범위 안의 무작위 수를 생성합니다. 여기서는
`start..=end` 형태의 범위를 사용하며, 하한과 상한을 _포함_ 합니다. 따라서
1부터 100까지를 원한다면 `1..=100`을 지정해야 합니다.

> 참고: 어떤 크레이트에서 어떤 트레이트와 메서드, 함수를 써야 하는지는 외워서
> 알 수 있는 게 아닙니다. 각 크레이트는 사용법을 안내하는 문서를 제공합니다.
> Cargo의 멋진 기능 중 하나로, `cargo doc --open`을 실행하면 의존성들의 문서를
> 로컬에서 빌드하여 브라우저로 열어 줍니다. 예를 들어 `rand`의 다른 기능이
> 궁금하다면 `cargo doc --open`을 실행한 다음 왼쪽 사이드바에서 `rand`를
> 클릭해 보세요.

두 번째 새 줄은 비밀 숫자를 출력합니다. 개발 중에는 테스트에 도움이 되지만,
최종 버전에서는 지워야 합니다. 시작하자마자 정답을 알려주는 게임은 재미없겠죠!

프로그램을 몇 번 실행해 보세요:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

매번 서로 다른, 1에서 100 사이의 숫자가 생성되어야 합니다. 잘하셨습니다!

## 추측값과 비밀 숫자 비교하기

이제 사용자 입력과 난수를 비교할 수 있습니다. Listing 2-4에 그 단계가
나와 있습니다. 다만 아직은 이 코드가 컴파일되지 않는데, 곧 설명하겠습니다.

<Listing number="2-4" file-name="src/main.rs" caption="두 수 비교의 가능한 반환값 처리">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

먼저 표준 라이브러리에서 `std::cmp::Ordering`이라는 타입을 스코프로 가져오는
`use` 구문을 추가합니다. `Ordering`은 또 다른 enum으로, `Less`, `Greater`,
`Equal` 세 가지 변형을 갖습니다. 이는 두 값을 비교했을 때 가능한 세 가지
결과입니다.

그리고 `Ordering` 타입을 사용하는 다섯 줄을 아래에 추가합니다. `cmp` 메서드는
두 값을 비교하며, 비교 가능한 어떤 것에도 호출할 수 있습니다. 비교 대상의
참조를 인자로 받습니다. 여기서는 `guess`와 `secret_number`를 비교합니다.
그리고 `use`로 스코프에 가져온 `Ordering`의 변형 중 하나를 반환합니다. 우리는
[`match`][match] 표현식으로 어떤 변형이 반환됐는지에 따라 다음 동작을
결정합니다.

`match` 표현식은 여러 개의 _팔(arm)_ 로 이루어집니다. 각 팔은 일치시킬
_패턴_ 과, 그 패턴에 값이 맞을 때 실행할 코드를 가집니다. Rust는 `match`에
주어진 값을 각 팔의 패턴과 차례대로 비교합니다. 패턴과 `match`는 매우 강력한
기능으로, 코드가 마주칠 수 있는 다양한 상황을 표현하고 빠짐없이 처리하도록
해 줍니다. 이 기능들은 각각 6장과 19장에서 자세히 다룹니다.

여기 사용한 `match` 표현식의 예를 따라가 봅시다. 사용자가 50을 추측했고,
이번에 생성된 비밀 숫자가 38이라고 합시다.

코드가 50과 38을 비교하면, `cmp`는 50이 38보다 크므로 `Ordering::Greater`를
반환합니다. `match`는 `Ordering::Greater` 값을 받아 첫 번째 팔의 패턴
`Ordering::Less`와 비교합니다. 일치하지 않으므로 넘어가고, 다음 팔의
`Ordering::Greater`와 비교합니다. 이번에는 _일치_ 하므로 그 팔의 코드가
실행되어 `Too big!`을 출력합니다. `match`는 첫 번째 성공적인 일치 이후에는
나머지 팔을 보지 않습니다.

하지만 Listing 2-4의 코드는 아직 컴파일되지 않습니다. 한번 시도해 봅시다:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

핵심 오류 메시지는 _타입 불일치(mismatched types)_ 입니다. Rust는 강력한
정적 타입 시스템을 갖고 있지만, 동시에 타입 추론도 합니다. `let mut guess =
String::new()`라고 썼을 때 Rust는 `guess`가 `String`이어야 함을 추론했으므로
타입을 적지 않아도 됐습니다. 반면 `secret_number`는 숫자 타입입니다. 1에서
100 사이 값을 가질 수 있는 숫자 타입은 `i32`, `u32`, `i64` 등 여럿입니다.
명시하지 않으면 Rust는 기본적으로 `i32`를 선택하므로, 다른 단서가 없다면
`secret_number`의 타입은 `i32`입니다. 문제의 이유는 문자열과 숫자 타입을
비교할 수 없기 때문입니다.

결국, 입력으로 읽은 `String`을 숫자 타입으로 변환하여 비밀 숫자와 수치적으로
비교해야 합니다. 이를 위해 `main` 함수 본문에 다음 줄을 추가합니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

이 줄은 다음과 같습니다:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

`guess`라는 변수를 만듭니다. 그런데 이미 `guess` 변수가 있지 않나요? 맞습니다.
하지만 Rust에는 _섀도잉(shadowing)_ 이라는 기능이 있어, 기존 `guess`를 새
`guess`로 가릴 수 있습니다. 이렇게 하면 `guess_str`, `guess`처럼 두 개의
서로 다른 이름을 만들 필요가 없습니다. 섀도잉은 3장 [“섀도잉”][shadowing]에서
자세히 다루지만, 지금은 한 타입의 값을 다른 타입으로 변환할 때 자주 쓰인다는
정도만 기억하세요.

새 변수는 `guess.trim().parse()` 표현식에 바인딩됩니다. 여기서 `guess`는
문자열을 담고 있던 원래 `guess` 변수를 가리킵니다. `String`의 `trim` 메서드는
앞뒤 공백을 제거합니다. 문자열을 `u32`로 변환하려면 숫자만 있어야 하므로
필수입니다. 사용자는 <kbd>enter</kbd>를 눌러 `read_line`을 완료해야 하며, 이때
문자열 끝에 줄바꿈 문자가 붙습니다. 예를 들어 <kbd>5</kbd>를 입력하고
<kbd>enter</kbd>를 누르면 `guess`는 `5\n`처럼 됩니다. `\n`은 줄바꿈을 뜻합니다.
(Windows에서는 `\r\n`이 붙습니다.) `trim`은 `\n` 또는 `\r\n`을 제거하여
`5`만 남깁니다.

[문자열의 `parse` 메서드][parse]는 문자열을 다른 타입으로 변환합니다. 여기서는
문자열을 숫자로 바꿉니다. 우리는 `let guess: u32`처럼 원하는 정확한 숫자
타입을 알려 주어야 합니다. `:` 뒤의 타입 표기는 변수의 타입을 명시한다는 뜻입니다.
`u32`는 부호 없는 32비트 정수로, 작은 양의 정수에 적합한 기본 선택입니다.
다른 숫자 타입은 3장 [“정수 타입”][integers]에서 배웁니다.

또한 이 예제에서 `u32`를 사용하고 `secret_number`와 비교하므로, Rust는
`secret_number`도 `u32`여야 함을 추론합니다. 이제 두 값은 같은 타입으로
비교됩니다!

`parse`는 논리적으로 숫자로 변환될 수 있는 문자에만 동작하므로 실패하기 쉽습니다.
예를 들어 문자열이 `A👍%`라면 숫자로 바꿀 방법이 없습니다. 따라서 `parse`는
실패 가능성을 표현하기 위해 `read_line`과 마찬가지로 `Result`를 반환합니다
(앞의 [“`Result`로 실패 가능성 다루기”](#handling-potential-failure-with-result)
에서 설명). 여기서도 `expect`를 사용해 같은 방식으로 처리합니다. 숫자 변환에
실패해 `Err`가 반환되면 `expect`가 게임을 중단시키고 메시지를 출력합니다.
성공하면 `Ok` 안의 숫자 값을 꺼내 반환합니다.

이제 프로그램을 실행해 봅시다:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
touch src/main.rs
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

입력 앞에 공백이 있어도 76으로 잘 인식했습니다. 정답, 너무 큼, 너무 작음 등
여러 경우를 시험해 보세요.

이제 거의 완성했습니다. 하지만 현재는 사용자에게 한 번만 기회를 줍니다. 루프를
추가해 바꿔 보겠습니다!

## 루프로 여러 번 추측 허용하기

`loop` 키워드는 무한 루프를 만듭니다. 사용자에게 여러 번 기회를 주기 위해
루프를 추가합니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

보시다시피 입력 프롬프트 이후의 모든 코드를 루프로 옮겼습니다. 루프 안의
줄들은 네 칸씩 들여쓰기 한 뒤 프로그램을 다시 실행하세요. 이제 프로그램은
계속해서 추측을 요구합니다. 그런데 사용자가 종료할 수 없는 문제가 생겼네요!

언제든 <kbd>ctrl</kbd>-<kbd>c</kbd>로 프로그램을 중단할 수 있습니다. 또 다른
방법은 앞서 [“추측값과 비밀 숫자 비교하기”](#comparing-the-guess-to-the-secret-number)
에서 언급했듯이, 숫자가 아닌 값을 입력하면 프로그램이 중단되는 점을 이용하는
것입니다. 다음과 같이 할 수 있습니다:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
touch src/main.rs
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit

thread 'main' panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`quit`을 입력해도 종료되지만, 사실 숫자가 아닌 어떤 입력으로도 종료됩니다.
그다지 바람직하지 않죠. 정답을 맞혔을 때도 게임이 종료되길 원합니다.

### 정답 시 종료하기

이제 정답일 때 게임을 끝내도록 `break`를 추가합니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

`You win!` 이후에 `break`를 넣으면 정답일 때 루프를 빠져나옵니다. 루프가
`main`의 마지막 부분이므로 프로그램도 종료됩니다.

### 잘못된 입력 처리하기

사용자가 숫자가 아닌 값을 입력했을 때 프로그램이 중단되지 않고, 무시하고 다음
입력을 받도록 동작을 더 다듬어 봅시다. `guess`를 `String`에서 `u32`로
변환하는 줄을 Listing 2-5처럼 바꾸면 됩니다.

<Listing number="2-5" file-name="src/main.rs" caption="숫자가 아닌 입력을 무시하고 다시 입력 받기">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

`expect` 대신 `match`를 사용해, 오류 시 중단에서 오류 처리로 전환합니다.
앞서 본 것처럼 `parse`는 `Result`를 반환하고, `Result`는 `Ok`와 `Err` 변형을
가집니다. 여기서도 `cmp`의 결과를 다룰 때처럼 `match`를 사용합니다.

`parse`가 문자열을 성공적으로 숫자로 바꾸면 결과 숫자를 담은 `Ok`를 반환합니다.
이 값은 첫 번째 팔의 `Ok(num)` 패턴과 일치하며, `match`는 `num` 값을 그대로
반환합니다. 이 숫자는 우리가 새로 만드는 `guess` 변수로 들어갑니다.

반대로 `parse`가 문자열을 숫자로 바꾸지 못하면 오류 정보를 담은 `Err`가
반환됩니다. `Err`는 첫 번째 팔 `Ok(num)`과는 일치하지 않지만, 두 번째 팔
`Err(_)`와는 일치합니다. 밑줄 `_`은 무엇이든 매칭하는 와일드카드입니다. 여기서는
어떤 오류든 모두 받아들이겠다는 뜻입니다. 이 경우 `continue`가 실행되어 루프의
다음 반복으로 넘어가 새 입력을 받습니다. 즉, `parse`에서 생길 수 있는 모든
오류를 무시하는 효과가 됩니다!

이제 프로그램은 기대한 대로 동작해야 합니다. 실행해 봅시다:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

아주 좋습니다! 마지막으로 한 가지 소소한 다듬기만 하면 게임이 완성됩니다.
프로그램이 아직 비밀 숫자를 출력하고 있죠. 테스트에는 편리했지만, 게임을 망칩니다.
비밀 숫자를 출력하는 `println!`을 삭제하세요. Listing 2-6이 최종 코드입니다.

<Listing number="2-6" file-name="src/main.rs" caption="완성된 숫자 맞추기 게임 코드">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

이제 숫자 맞추기 게임을 성공적으로 완성했습니다. 축하합니다!

## 요약

이번 프로젝트는 `let`, `match`, 함수, 외부 크레이트 사용 등 여러 새로운 Rust
개념을 실습으로 소개했습니다. 다음 장들에서 이 개념들을 더 자세히 배웁니다.
3장은 대부분의 프로그래밍 언어에 공통적인 변수, 데이터 타입, 함수 같은 개념을
Rust에서 어떻게 사용하는지 다룹니다. 4장은 Rust만의 특징인 소유권(ownership)을
살펴봅니다. 5장은 구조체와 메서드 문법을, 6장은 열거형의 동작을 설명합니다.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
