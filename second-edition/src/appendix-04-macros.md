## 부록 D: 매크로

우린 이 책에서 `println!` 등의 매크로를 사용했습니다.
하지만 아직 매크로가 정확히 무엇이고, 어떻게 동작하는지는 알아보지 않았습니다.
이번 부록에선 매크로에 대해 다음과 같은 순서로 알아볼 것입니다:

* 매크로가 무엇이고, 함수와 다른 점
* 메타프로그래밍을 하기 위한 '선언적 매크로' 정의법
* `derive` 트레잇을 커스텀하기 위한 절차적 매크로 정의법

매크로에 대한 자세한 내용을 부록에서 다루는 이유는,
러스트의 매크로는 아직 진화중이기 때문입니다. 러스트 1.0 이래로, 매크로는
언어의 나머지 부분과 표준 라이브러리보다 빠르게 바뀌었고, 향후에도 그럴 것입니다.
따라서 이 절은 책의 다른 부분보다 시대에 뒤처질 가능성이 높습니다.
러스트는 안정성을 보증하므로 여기 나오는 코드는 이후 버전에서도 동작할 테지만,
그때쯤엔 이 책이 발행된 시점에선 사용할 수 없었던 추가 기능이나,
보다 쉽게 매크로를 작성하는 여러 방법이 존재할 것입니다.
만약 이 부록을 참고해 무언가 구현하려 하신다면 이 점을 염두해두시기 바랍니다.

### 매크로와 함수의 차이

근본적으로, 매크로는 다른 코드를 작성하는 코드입니다.
이 개념은 *메타프로그래밍(metaprogramming)* 으로 잘 알려져 있죠.
우린 부록 C 에서 트레잇에 다양한 구현체를 생성해주는 `derive` 속성에 대해 다뤘고,
책 중간중간 `println!` 과 `vec!` 매크로도 사용했습니다. 이 모든 매크로들은
수동으로 코드를 작성하지 않고도 많은 코드를 생산해낼 수 있게 합니다.

메타프로그래밍은 여러분이 작성하고 관리해야 할 코드의 양을 줄여줍니다.
물론 이는 함수의 역할이기도 합니다만,
매크로는 함수가 하지 못하는 일도 할 수 있습니다.

함수 시그니처는 해당 함수가 갖는 매개변수의 개수와 타입을 선언해야만
하는 반면, 매크로는 매개변수의 개수를 가변적으로 처리할 수 있습니다:
간단한 예로, `println!("hello")` 와 같이 1 개의 매개변수를 사용하거나,
`println!("hello {}", name)` 처럼 2 개의 매개변수를 사용할 수 있습니다.
또한 매크로는 컴파일러가 코드의 의미를 해석하기 이전에 작동합니다.
따라서 주어진 타입에 트레잇을 구현하는 등, 런타임에 호출되는 함수로는
불가능한 일을 할 수 있습니다.

단, 함수 대신 매크로를 구현하는 것도 단점이 존재합니다.
매크로를 구현한다는 것은 러스트 코드를 만들어내는 코드를 작성한다는 것입니다.
이는 추상화 계층을 하나 더 만들어 낸다는 것이기 때문에,
매크로 정의는 일반적으로 함수 정의에 비해 읽고, 이해하고, 관리하기 어렵습니다.
요약해서, 매크로 정의의 단점은 함수 정의보다 복잡하다는 겁니다.

함수와 매크로의 또다른 차이는, 매크로 정의는 함수 정의와는 달리
모듈의 네임스페이스에 소속되지 않는다는 것입니다.
이로 인한 외부 크레이트 사용 시 발생하는 예기치 않은 이름 충돌을
막기 위해선, 외부 크레이트를 스코프 내로 가져오는 동시에
`#[macro_use]` 어노테이션을 사용하여 가져올 매크로를 명시해야 합니다.
다음은 `serde` 크레이트에 정의된 매크로를
현재 크레이트의 스코프로 가져오는 예제입니다:

```rust,ignore
#[macro_use]
extern crate serde;
```

만약 어노테이션을 명시하지 않더라도 `extern crate` 만으로 스코프 내에 매크로가
자동적으로 들어오게 됐더라면, 여러분은 같은 이름의 매크로가 정의된 크레이트를
동시에 사용하지 못했을 겁니다. 충돌이 실제로 자주 발생하는 건 아니지만,
많은 크레이트를 사용할수록 충돌이 발생할 확률은 높아집니다.

마지막으로, 매크로와 함수의 차이 중 중요한 한 가지가 남았습니다:
함수는 정의 위치에 관계 없이 아무 곳에서나 호출이 가능하지만,
매크로는 호출 하기 전에 반드시 해당 파일에 정의하거나 가져와야 합니다.

### 일반적인 메타프로그래밍을 위한 `macro_rules!` 를 사용하는 선언적 매크로

러스트에서 매크로는 *선언적 매크로(declarative macros)* 의 형태로
가장 널리 사용됩니다. 이는 *예제 별 매크로(macros by example)*,
*`macro_rules!` 매크로*, 아니면 그냥 *매크로* 라고 불리기도 합니다.
선언적 매크로란 것은, 러스트의 `match` 표현식과 유사하게 작성할 수 있습니다.
`match` 는 표현식을 다루는 제어구조입니다. 6 장에서 다뤘듯 `match` 는
패턴과 표현식의 결과값을 비교하고, 해당 패턴과 연관된 코드를 실행합니다.
매크로 또한 값과, 관련된 코드를 갖는 패턴을 비교합니다;
이때 값은 매크로에 넘겨진 러스트 소스코드를 말하고,
패턴에는 소스코드의 구조가 해당되며, 각 패턴에 관련된 코드는
매크로로 전달되어 대체된 코드를 말합니다.
그리고, 이 모든 과정은 컴파일 중 일어납니다.

`vec!` 매크로가 어떻게 정의되어 있는지 살펴보도록 합시다.
`vec!` 매크로는 특정 값들을 이용해 새로운 벡터를
생성하는 매크로로, 8 장에서 다뤘습니다.
다음은 이 매크로를 사용해 세 정수 값으로
새로운 벡터를 생성하는 예시입니다:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

이외에도 `vec!` 매크로는 두 정수로 이루어진 벡터나
5 개의 스트링 슬라이스로 이루어진 벡터를 만드는 데도 사용할 수 있습니다.
함수는 사전에 값의 개수나 타입을 알 수 없으므로 이러한 작업은 불가능합니다.

`vec!` 매크로의 정의를 간략화한 모습을 Listing D-1 에서
한번 살펴봅시다.

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

<span class="caption">Listing D-1: `vec!` 매크로 정의를 간략화한
모습</span>

> Note: 표준 라이브러리 내 `vec!` 매크로의 실제 정의에는
> 메모리의 정확한 양을 미리 할당하는 코드가 포함되어 있습니다.
> 이 코드에선 예제를 간략화 하기 위해서 해당 부분을 제외했습니다.

`#[macro_export]` 어노테이션은 우리가 정의한 매크로가 들어 있는 크레이트를
누군가 임포트 했을 때, 해당 매크로를 사용할 수 있도록 해줍니다. 이 어노테이션을
사용하지 않을 경우, 이 크레이트를 디펜던시로 갖는 누군가가 `#[macro_use]`
어노테이션을 사용하더라도 해당 매크로는 스코프 내로 가져와지지 않습니다.

매크로 정의는 `macro_rules!` 와 느낌표가 붙지 *않는* 매크로의
이름 으로 시작됩니다. 예시의 경우, 매크로명은 `vec` 이며
뒤따르는 중괄호가 매크로 정의의 본문을 나타냅니다.

`vec!` 의 본문 구조는 `match` 표현식 구조와 유사합니다.
여기선 `( $( $x:expr ),* )` 패턴과 그 뒤로 따라오는 `=>`, 그리고 해당 패턴에
연관된 코드 블록으로 이루어진 갈래 하나를 갖습니다. 패턴이 매치될 경우,
해당 패턴에 연관된 코드가 반환 됩니다. 이 매크로는 하나뿐인 패턴을 갖기 때문에
매치되는 경우는 하나 뿐이며, 다른 모든 경우는 에러가 발생할 것입니다.
물론 이보다 복잡한 매크로는 갈래를 하나 이상 갖겠죠.

매크로 정의에서 사용하는 패턴 문법은 18 장에서 다룬 패턴 문법과는 다릅니다.
그 이유는, 매크로는 러스트 코드 구조와 매치되기 위한 것인데 18 장의 패턴에서
사용하는 값과 러스트 코드 구조는 상당히 다르기 때문입니다.
이제 Listing D-1 의 패턴을 하나씩 살펴보면서 알아보도록 합시다; 매크로 패턴의 모든 문법에 대한 내용은 [레퍼런스] 를 참조하시기 바랍니다.

[레퍼런스]: https://doc.rust-lang.org/reference/macros.html#macros

먼저 괄호 쌍이 전체 패턴을 둘러쌉니다.
그 다음 달러 기호(`$`)뒤에 괄호 쌍이 오며,
배치할 코드에서 사용하기 위해, 괄호 안 패턴과 일치하는 값을 캡처합니다.
`$()` 내에는 `$x:expr` 가 있는데, 이는 어떤 러스트 표현식과도 매치되며,
그에 `$x` 라는 이름을 부여하는 기능을 합니다.

`$()` 에 따라오는 쉼표는 `$()` 내에 캡처되어 매치된 코드 뒤에
나타날 수도 있는 리터럴 쉼표 구분 문자를 나타냅니다.
쉼표 뒤 `*` 는 자신 앞에 위치한 0 개 이상
패턴을 지정합니다.

이 매크로를 `vec![1, 2, 3];` 으로 호출할 경우,
`$x` 패턴은 `1`, `2`, `3` 세 표현식에 맞춰 세 번 매치됩니다.

이제 패턴 갈래와 연관된 본문 코드를 살펴봅시다:
`$()*` 부분 내의 `temp_vec.push()` 코드는
패턴에서 `$()` 에 매치되는 횟수만큼 반복되어 생성되고,
코드 내 `$x` 는 각각의 매치된 표현식으로 대체됩니다.
따라서`vec![1, 2, 3]` 으로 이 매크로를 호출할 경우,
매크로로부터 생성되어 매크로 호출문을 대체할 코드는 다음과 같습니다:

```rust,ignore
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec
```

우린 인수 개수가 어느만큼이건, 어떤 타입이건 가리지 않고 특정한 요소들을
포함할 벡터를 만들어내는 코드를 생성할 수 있는 매크로를 선언했습니다.

대부분의 러스트 프로그래머는 매크로를 *작성하기* 보다는 *사용하는* 일이 더
많을 겁니다. 따라서 여기선 `macro_rules!` 에 대해서 더 이상 다루지 않습니다.
매크로를 작성하는 법에 대해 더 많은 것을 배우고 싶으신 분은
[“The Little Book of Rust Macros”][tlborm] 등의 온라인 문서를 찾아보세요.

[tlborm]: https://danielkeep.github.io/tlborm/book/index.html

### 커스텀 `derive` 를 위한 절차적 매크로

두번째 매크로 형식은 함수(프로시저(procedure) 유형)에
가깝기 때문에 *절차적 매크로(procedural macros)* 라고 불립니다.
선언적 매크로는 코드를 패턴과 매치시키고 다른 코드로 대체하는 반면,
절차적 매크로는 어떤 러스트 코드를 입력 받고, 코드를 연산하여
러스트 코드를 생성합니다. 이 내용이 작성된 시점엔 `derive` 어노테이션에
특정 트레잇 이름을 지정하여, 타입에 해당 트레잇을 구현하도록 하는 데에만
절차적 매크로를 정의할 수 있습니다.

우린 `hello_macro` 크레이트를 생성할 것이며, 이 크레이트는
`hello_macro` 라는 연관 함수 하나를 가진 `HelloMacro` 트레잇을 정의할 것입니다.
다만, 이 크레이트를 사용하는 사람이 각 타입마다 `HelloMacro` 트레잇을
구현할 필요 없도록 절차적 매크로를 제공하여, 사용자가 자신의 타입에
`#[derive(HelloMacro)]` 를 어노테이트 하는 것 만으로도 `hello_macro` 함수의
기본 구현체를 이용할 수 있도록 해봅시다. 이 기본 구현체는
`Hello, Macro! My name is TypeName` (`TypeName` 엔 이 트레잇이 정의된 타입의
이름이 들어갈 겁니다.) 을 출력할 것입니다. 쉽게 말해서, 우리 크레이트를 이용하는
다른 프로그래머가 Listing D-2 처럼 코드를 작성할 수 있도록 할 것입니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate hello_macro;
#[macro_use]
extern crate hello_macro_derive;

use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

<span class="caption">Listing D-2: 우리가 만든 크레이트의 사용자가
우리 절차적 매크로를 이용해 작성 가능할 코드</span>

완성됐을 때, 이 코드는 `Hello, Macro! My name is Pancakes!` 를 출력할 겁니다.
이제 새 라이브러리 크레이트를 만드는 첫 과정을 진행해보죠:

```text
$ cargo new hello_macro --lib
```

다음으로, `HelloMacro` 트레잇과 연관 함수를 정의합시다.

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

이제 트레잇과 함수를 만들었습니다. 또한 현재 시점에서도 이 크레이트를
이용해 다음과 같은 방식으로 우리가 원하던 기능을 구현할 수는 있습니다.

```rust,ignore
extern crate hello_macro;

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

하지만 지금으로서는 `hello_macro` 와 같이 사용하려는 타입마다
구현 내용을 직접 작성해줘야 합니다; 이 작업은 생략할 수 있도록
하는 편이 좋을 것입니다.

허나, 우린 아직 `hello_macro` 함수의 기본 구현체를 제공할 수 없습니다.
우리가 원하는 기능은 자신이 구현된 트레잇 이름을 알아내어 출력하는 것인데,
러스트엔 리플렉션 기능이 없어서 런타임 중에는 타입명을 알아낼 수 없기
때문입니다. 따라서 매크로를 이용해 컴파일 타임에 코드를 생성해야 합니다.

다음 단계는 절차적 매크로를 정의하는 것입니다. 이 내용이 작성된 시점엔
절차적 매크로가 자신의 크레이트 내부에 위치해야만 하지만, 이 제약은 언젠가
사라질 겁니다. 크레이트 및 매크로 크레이트의 구조화 규칙은 다음과 같습니다:
예를 들어 `foo` 크레이트의 경우, derive 절차적 매크로 크레이트명은
`foo_derive` 가 됩니다. 이제 `helo_macro` 프로젝트 내에
`hello_macro_derive` 라는 새 크레이트를 만들어 봅시다:

```text
$ cargo new hello_macro_derive --lib
```

이 두 크레이트는 밀접히 연관되어 있고, 따라서 절차적 매크로 크레이트를
`hello_macro` 크레이트 디렉토리 내에 생성하였습니다.
이는 우리가 만약 `hello_macro` 내 트레잇 정의를 변경할 경우,
`hello_macro_derive` 의 절차적 매크로 구현 또한 변경하도록 강제합니다.
이 두 크레이트는 각각 따로 배포될 것이고, 이를 사용할 프로그래머는
이 크레이트들을 각각 디펜던시에 추가하고 스코프 내로 가져와야 할 겁니다.
물론 우린 `hello_macro` 크레이트에 `hello_macro_derive` 를 디펜던시로 추가하고,
절차적 매크로 코드를 다시 export 할 필요 없도록 만들 수도 있습니다.
하지만 이 방법대로는 `derive` 기능을 원치 않던 사용자들도
강제적으로 `hello_macro` 를 사용해야만 합니다.

우린 `hello_macro_derive` 크레이트가 절차적 매크로 크레이트라는 것을
나타내야 합니다. 또한 잠시 후에 볼 수 있듯이 `syn` 크레이트와 `quote`
크레이트의 기능이 필요하므로 이 둘을 디펜던시로 추가합니다.
결과적으로 `hello_macro_derive` 의 *Cargo.toml* 파일은 다음과 같습니다:

<span class="filename">Filename: hello_macro_derive/Cargo.toml</span>

```toml
[lib]
proc-macro = true

[dependencies]
syn = "0.11.11"
quote = "0.3.15"
```

이제 절차적 매크로를 정의해봅시다. 먼저, 여러분의 `hello_macro_crate` 크레이트
*src/lib.rs* 파일에 Listing D-3 코드를 작성합니다. 다만, 이 코드는 우리가
`impl_hello_macro` 함수를 구현하지 않는 이상 컴파일 되진 않을 겁니다.

<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>

```rust,ignore
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_macro(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
```

<span class="caption">Listing D-3: 대부분의 절차적 매크로 크레이트가
러스트 코드를 처리하는데 사용할 코드</span>

D-3 에서 함수들이 분리된것을 주목하세요;
이는 절차적 매크로를 더 편리하게 작성하기 위한 방법이므로,
여러분이 보게 될, 혹은 만들게 될 많은 크레이트도 거의 대부분 마찬가지일 겁니다.
이 때, 호출한 `impl_hello_macro` 함수에서 어떤 작업을 할 지는
여러분이 어떤 목적으로 절차적 매크로를 만드는 지에 따라 달라질 겁니다.

우린 `proc_macro`, [`syn`], [`quote`] 3 개의 새로운 크레이트를 사용했습니다.
이 중 `proc_macro` 는 러스트에 포함되어 있으므로 *Cargo.toml* 에 디펜던시로
추가할 필요가 없으며, 러스트 코드를 문자열로 변환하는 기능을 갖습니다.
또한 `syn` 크레이트는 문자열로 변환한 러스트 코드를 연산을 수행하기 위한
자료구조로 파싱합니다. 마지막으로 `quote` 크레이트는 `syn` 의 자료구조를
러스트 코드로 복원하는 역할을 합니다. 이 크레이트들은 어떤 종류의 러스트 코드든
우리가 다루고 싶은 것을 더 쉽게 다룰 수 있도록 도와줍니다:
모든 러스트 코드를 파싱하는 파서를 작성하는 건 결코 쉬운 일은 아닙니다.

[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote

`hello_macro_derive` 함수에 `proc_macro_derive` 와 `HelloMacro` 라는
이름을 어노테이트 하였기 때문에, `hello_macro_derive` 함수는 우리
라이브러리 사용자가 타입에 `#[derive(HelloMacro)]` 를 명시했을 때 호출됩니다.
`HelloMacro` 는 우리 트레잇 이름과 매치되는데, 이름을 이런식으로 짓는 게
대부분의 절차적 매크로가 따르는 관습입니다.

이 함수는 먼저 `to_string` 을 호출하여 `TokenStream` 인 `input` 을 `String`
으로 변환합니다. 이 `String` 은 `HelloMacro` 를 derive 한 러스트 코드에
해당합니다. 즉 Listing D-2 같은 경우에 `s` 는 `#[derive(HelloMacro)]`
어노테이션을 추가한 부분의 러스트 코드인 `struct Pancakes;` 를
`String` 값으로 지닐 것입니다.

> Note: 이 내용이 작성된 시점에 `TokenStream` 은 문자열로만 변환 가능했습니다.
> 이 시점 이후엔 더 많은 API 가 제공될 겁니다.

이제 러스트 코드 `String` 을 우리가 해석하고, 연산을 수행할
수 있는 자료구조로 파싱해야합니다. `syn` 이 활약할 시간이 왔네요.
`syn` 내 `parse_derive_input` 함수는 `String` 을 인자로 받아
러스트 코드를 파싱하여 `DeriveInput` 라는 구조체로 반환합니다.
다음 코드는 `struct Pancakes` 문자열을 파싱해서 얻은 `DeriveInput`
구조체 내용 중 문자열과 관련된 부분을 나타냅니다:

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident(
        "Pancakes"
    ),
    body: Struct(
        Unit
    )
}
```

이 구조체 필드는 우리가 파싱한 러스트 코드가 `Pancakes` 라는 `ident`
(식별자, 즉 이름) 를 갖는 유닛 구조체라는 것을 나타냅니다. 물론 여기 나온
필드 이외에도 많은 필드가 모든 종류의 러스트 코드를 기술하기 위해 존재합니다;
자세한 내용을 원하시는 분은 [`syn` 의 `DeriveInput` 문서][syn-docs] 를 참고하세요.

[syn-docs]: https://docs.rs/syn/0.11.11/syn/struct.DeriveInput.html

우린 새로운 러스트 코드를 생성할 `impl_hello_macro` 함수를
아직 정의하지 않았습니다. 하지만 이 함수를 정의하기에 앞서,
`hello_macro_derive` 함수 맨 끝에서 `quote` 크레이트의 `parse` 함수를 이용해
`impl_hello_macro` 함수 출력을 `TokenStream` 으로 변환한 것에 주목해주세요.
반환된 `TokenStream` 은 크레이트 사용자가 작성한 코드에 추가될 것이고,
이로써 사용자는 자신의 크레이트를 컴파일 할 때
우리가 제공한 추가적인 기능을 갖게 됩니다.

눈치 채셨을진 모르겠지만 여기선 `parse_derive_input` 이나 `parse` 함수를
호출하는 데 실패하면 `unwrap` 을 호출해 패닉을 일으키도록 되어 있습니다.
절차적 매크로 API 에 따르면 `proc_macro_derive` 에선 `Result` 가 아닌
`TokenStream` 을 반환해야 하기 때문에, 절차적 매크로 코드에서 에러가 발생했을
경우 패닉을 일으키는 것은 필수적입니다. 우린 예제를 간단히 하기 위해 `unwrap` 을
사용했지만, 실제 프로덕션 코드에선 `panic!` 이나 `expect` 를 사용해
정확히 무엇이 잘못됐는지 자세히 설명해주는 에러 메시지를 제공해야 합니다.

어노테이션된 러스트 코드를 `TokenStream` 에서 `String` 과 `DeriveInput`
인스턴스로 변환하는 코드를 작성했으니, 어노테이션 된 타입에 `HelloMacro`
트레잇을 구현하는 코드를 만들 차례입니다.

<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>

```rust,ignore
fn impl_hello_macro(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    }
}
```

`asd.ident` 를 이용해 어노테이션 된 타입의 타입명(식별자)을 담고 있는
`Ident` 구조체 인스턴스를 가져왔습니다. Listing D-2 코드의 경우,
`name` 값은 `Ident("Pancakes")` 가 됩니다.

`quote!` 매크로는 우리가 반환하고 싶은 러스트 코드를 작성하면
`quote::Tokens` 로 변환해 줍니다. 또한 이 이외에도 `#name` 을 작성하면
`quote!` 는 해당 부분을 `name` 변수의 값으로 대체하는 등, 굉장히 멋진
템플릿 기능을 제공합니다. You can even do some repetition similar
to the way regular macros work. 자세한 내용은
[`quote` 크레이트 문서][quote-docs] 를 참고하세요.

[quote-docs]: https://docs.rs/quote

우리 목표는 절차적 매크로를 이용해 사용자가 어노테이션을 추가한 타입
(`#name` 으로 가져올 수 있는)에 `HelloMacro` 트레잇 구현체를 생성하도록 하고,
구현된 트레잇은 `hello_macro` 라는 함수 하나를 가지며, 그 함수는
`Hello, Macro! My name is` 와 그 뒤에 어노테이션 된 타입의 이름을
출력하는 기능을 갖도록 하는 것입니다.

여기서 사용한 `stringify!` 매크로는 러스트 안에 포함되어있습니다.
이 매크로는 `1 + 2` 같은 러스트 표현식을 받아서 컴파일 타임에 해당 표현식을
`"1 + 2"` 처럼 스트링 리터럴로 변환합니다. `format!` 이나 `println!` 처럼
표현식을 평가하고 결과를 `String` 으로 변환하는 것과는 다릅니다. `stringify!` 를
사용하는 이유는 `#name` 입력이 그대로 출력돼야 하는 표현식일 수도 있기 때문입니다.
또한 `stringify!` 는 컴파일 타임에 `#name` 을 스트링 리터럴로 변환하여,
할당을 절약하는 효과를 가져오기도 합니다.

이 시점에서 `cargo build` 는 `hello_macro` 와 `hello_macro_derive` 양쪽 모두에서
문제 없이 돌아갑니다. 그럼 이제 이 크레이트들과 Listing D-2 코드를 연결해
절차적 매크로가 실제 작동하는 모습을 살펴봅시다. 여러분의 *projects* 디렉토리에
`cargo new --bin pancakes` 를 실행해 새 바이너리 프로젝트를 생성한 뒤,
`pancakes` 의 *Cargo.toml* 에 `hello_macro` 와 `hello_macro_derive` 를
디펜던시로 추가하세요. 만약 여러분이 *https://crates.io/* 에 이 크레이트를
배포하셨다면 상관 없겠지만, 그렇지 않다면 이때 다음과 같이 디펜던시에
`path` 를 명시해야 합니다.

```toml
[dependencies]
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

Listing D-2 코드를 *src/main.rs* 에 작성하고, `cargo run` 을 실행해보세요:
`Hello, Macro! My name is Pancakes!` 가 출력될 겁니다.
`pancakes` 크레이트에서 따로 구현할 필요 없이,
절차적 매크로로부터 만들어진 `HelloMacro` 트레잇 구현체만으로 말이죠;
트레잇 구현체는 `#[derive(HelloMacro)]` 로 인해 추가됩니다.

### 향후의 매크로

러스트는 앞으로 선언적, 절차적 매크로를 발전시킬 겁니다.
`macro` 키워드를 이용해 더 나은 선언적 매크로 시스템을 사용하고, `derive`
보다 더 강력한 여러 작업을 위해 더 많은 종류의 절차적 매크로를 추가할 겁니다.
이 기능들은 이 내용이 작성 된 시점에선 아직 개발중이니,
최근 정보에 대해서는 러스트 온라인 문서를 참조하시기 바랍니다.
