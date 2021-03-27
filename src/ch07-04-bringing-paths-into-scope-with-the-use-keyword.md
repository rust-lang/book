## `use` 키워드로 경로를 스코프 내로 가져오기

앞서 작성한 함수 호출 경로는
너무 길고 반복적으로 느껴지기도 합니다.
예를 들어, Listing 7-7에서는 절대 경로를 사용하건 상대 경로를 사용하건,
`add_to_waitlist` 호출할 때마다 `front_of_house`,
`hosting` 모듈을 매번 명시해 주어야 하죠.
`use` 키워드를 사용해 경로를 스코프 내로 가져오면
이 과정을 단축하여 마치 로컬 항목처럼 호출할 수 있습니다.

Listing 7-11은 `crate::front_of_house::hosting` 모듈을
`eat_at_restaurant` 함수가 존재하는 스코프로 가져와,
`eat_at_restaurant` 함수 내에서 `add_to_waitlist` 함수를
`hosting::add_to_waitlist` 경로만으로 호출하는 예제입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

<span class="caption">Listing 7-11: `use` 키워드로 모듈을 스코프 내로
가져오기</span>

스코프에 `use` 키워드와 경로를 작성하는 건
파일 시스템에서 심볼릭 링크를 생성하는 것과 유사합니다.
크레이트 루트에 `use crate::front_of_house::hosting`를 작성하면
해당 스코프에서 `hosting` 모듈을 크레이트 루트에 정의한 것처럼 사용할 수 있습니다.
`use` 키워드로 가져온 경우 또한 다른 경로와 마찬가지로 비공개 규칙이 적용됩니다.

`use` 키워드에 상대 경로를 사용할 수도 있습니다.
Listing 7-12는 Listing 7-11 코드를
상대 경로로 변경한 예제입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

<span class="caption">Listing 7-12: `use` 키워드와 상대 경로를 작성하여
모듈을 스코프에 가져오기</span>

### 보편적인 `use` 경로 작성법

Listing 7-11에서 `add_to_waitlist` 함수까지 경로를 전부 작성하지 않고,
`use crate::front_of_house::hosting` 까지만 작성한 뒤
`hosting::add_to_waitlist` 코드로 함수를 호출하는 점이 의아하실 수도 있습니다.
Listing 7-13 처럼 작성하면 안 되는 걸까요?

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

<span class="caption">Listing 7-13: `use` 키워드로 `add_to_waitlist` 함수를 직접 가져오기
(보편적이지 않은 작성 방식)</span>

Listing 7-11과 7-13의 동작은 동일하지만, Listing 7-11 코드가
`use` 키워드로 스코프에 함수를 가져올 때의 관용적인 코드입니다.
함수의 부모 모듈을 `use` 키워드로 가져올 경우, 전체 경로 대신 축약 경로만 작성하면서도,
해당 함수가 현재 위치에 정의된 함수가 아님이 명확해지기 때문입니다.
반면, Listing 7-13은 `add_to_waitlist` 함수가 어디에 정의되어 있는지
알기 어렵습니다.

한편, `use` 키워드로 구조체나 열거형 등의 타 항목을
가져올 시에는 전체 경로를 작성하는 것이 보편적입니다.
Listing 7-14는 `HashMap` 표준 라이브러리 구조체를
바이너리 크레이트의 스코프로 가져오는 관용적인 코드 예시입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<span class="caption">Listing 7-14: 보편적인 방식으로 `HashMap`을
스코프로 가져오기</span>

이러한 관용이 탄생하게 된 명확한 이유는 없습니다.
어쩌다 보니 관습이 생겼고, 사람들이 이 방식대로 러스트 코드를 읽고 쓰는 데에 익숙해졌을 뿐입니다.

하지만, 동일한 이름의 항목을 여럿 가져오는 경우는 이 방식을 사용하지 않습니다.
러스트가 허용하기 않기 때문이죠.
Listing 7-15는 각각 다른 모듈 내에 위치하지만 이름이 같은 두 개의
`Result` 타입을 스코프로 가져와 사용하는 예시입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption">Listing 7-15: 이름이 같은 두 개의 타입을 동일한 스코프에 가져오려면
부모 모듈을 반드시 명시해야 합니다.</span>

보시다시피 부모 모듈을 명시하여 두 개의 `Result` 타입을 구별하고 있습니다.
만약 `use std::fmt::Result`, `use std::io::Result` 로 작성한다면,
동일한 스코프 내에 두 개의 `Result` 타입이 존재하므로
러스트는 우리가 어떤 `Result` 타입을 사용했는지 알 수 없습니다.

### `as` 키워드로 새로운 이름 제공하기

`use` 키워드로 동일한 이름의 타입을 스코프로 여러 개 가져올 경우의 또 다른 해결 방법이 있습니다.
경로 뒤에 `as` 키워드를 작성하고, 새로운 이름이나 타입 별칭을 작성을 작성하면 됩니다.
Listing 7-16은 `as` 키워드를 이용해 Listing 7-15 코드 내
`Result` 타입 중 하나의 이름을 변경한 예제입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">Listing 7-16: 스코프 내로 가져온 타입의 이름을
`as` 키워드로 변경하기</span>

두 번째 `use` 구문에서는, 앞서 스코프 내로 가져온
`std::fmt` 의 `Result` 와 충돌을 방지하기 위해
`std::io::Result` 타입의 이름을 `IoResult` 로 새롭게 지정합니다.
Listing 7-15, Listing 7-16은 둘 다 관용적인 방식이므로, 원하는 방식을 선택하시면 됩니다!

### `pub use` 로 다시 내보내기

`use` 키워드로 이름을 가져올 경우,
해당 이름은 새 위치의 스코프에서 비공개가 됩니다.
`pub` 와 `use` 를 결합하면 우리 코드를 호출하는 코드가,
해당 스코프에 정의된 것처럼 해당 이름을 참조할 수 있습니다.
이 기법은 항목을 스코프로 가져오는 동시에 다른 곳에서 항목을 가져갈 수 있도록 만들기 때문에,
*다시 내보내기(Re-exporting)* 라고 합니다.

Listing 7-17은 Listing 7-11 코드의 `use` 를 `pub use` 로
변경한 예제입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">Listing 7-17: 다른 스코프의 코드에서 사용할 수 있도록
`pub use` 사용</span>

`pub use`를 사용하면 외부 코드에서 `add_to_waitlist` 함수를
`hosting::add_to_waitlist` 코드로 호출할 수 있습니다.
`pub use` 로 지정하지 않을 경우, `eat_at_restaurant` 함수에서는
여전히 `hosting::add_to_waitlist` 로 호출할 수 있지만, 외부 코드에서는 불가능합니다.

다시 내보내기 기법은 여러분이 작성한 코드의 구조 내부와,
여러분의 코드를 사용할 프로그래머들이 예상할법한 해당 분야의 구조가 서로 다를 때 유용합니다.
레스토랑 비유 예제를 예로 들어보죠. 레스토랑을 운영하는
직원들의 머릿속에서는 '접객 부서'와 '지원 부서'가 나뉘어있습니다.
하지만 레스토랑을 방문하는 고객들은 레스토랑의 부서를 그런 용어로 나누어 생각하지 않겠죠.
`pub use` 를 사용하면 코드를 작성할 때의 구조와, 노출할 때의 구조를 다르게 만들 수 있습니다.
라이브러리를 제작하는 프로그래머와, 라이브러리를 사용하는 프로그래머
모두를 위한 라이브러리를 구성하는데 큰 도움이 되죠.

### 외부 패키지 사용하기

우린 2장에서 `rand` 라는 외부 패키지를 사용해
추리 게임의 랜덤 숫자 생성을 구현했었습니다.
`rand` 패키지를 우리 프로젝트에서 사용하기 위해, *Cargo.toml* 에 다음 줄을 추가했었죠.

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

*Cargo.toml* 에 `rand` 를 의존성으로 추가하면 Cargo가
`rand` 패키지를 비롯한 모든 의존성을 [crates.io](https://crates.io/)에서 다운로드하므로
프로젝트 내에서 `rand` 패키지를 사용할 수 있게 됩니다.

그 후, `use` 키워드와 크레이트 이름인
`rand`를 작성하고, 가져올 항목을 나열하여,
`rand` 정의를 우리가 만든 패키지의 스코프로 가져왔습니다.
2장 ["임의의 숫자를 생성하기"][rand]<!-- ignore --> 절을 다시 떠올려보죠.
`Rng` 트레잇을 스코프로 가져오고 `rand::thread_rng` 함수를 호출했었습니다.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

러스트 커뮤니티 구성원들은 [crates.io](https://crates.io/)에서 이용 가능한
다양한 패키지를 만들어두었으니, 같은 방식으로 가져와서 여러분의 패키지를 발전시켜보세요.
여러분이 만든 패키지의 *Cargo.toml* 파일에 추가하고,
`use` 키워드를 사용해 스코프로 가져오면 됩니다.

알아 두어야 할 것은,
표준 라이브러리 `std`도 마찬가지로 외부 크레이트라는 겁니다.
러스트 언어에 포함되어 있기 때문에 *Cargo.toml* 에 추가할 필요는 없지만,
표준 라이브러리에서 우리가 만든 패키지의 스코프로 가져오려면 `use` 문을 작성해야 합니다.
예를 들어, `HashMap`을 가져오는 코드는 다음과 같습니다.

```rust
use std::collections::HashMap;
```

표준 라이브러리 크레이트의 이름인 `std` 로 시작하는
절대 경로입니다.

### 대량의 `use` 구문을 중첩 경로로 정리하기

동일한 크레이트나, 동일한 모듈 내에 정의된 항목을 여럿 사용할 경우,
각 항목당 한 줄씩 코드를 나열하면 수직 방향으로 너무 많은 영역을 차지합니다.
예시로 살펴봅시다. 추리 게임 Listing 2-4에서 작성했던
다음 두 `use` 문은 `std` 내 항목을 스코프로 가져옵니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

중첩 경로를 사용하면 한 줄로 작성할 수 있습니다.
경로의 공통된 부분을 작성하고, `::` 와 중괄호 내에
경로가 각각 다른 부분을 나열합니다.
예시는 Listing 7-18와 같습니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">Listing 7-18: 중첩 경로를 사용해, 경로의 앞부분이 같은
여러 항목을 스코프로 가져오기</span>

규모가 큰 프로그램이라면, 동일한 크레이트나 모듈에서 여러 항목을 가져오는 데에
중첩 경로를 사용함으로써 많은 `use` 구문을
줄일 수 있습니다!

중첩 경로는 경로의 모든 부위에서 사용할 수 있으며,
하위 경로가 동일한 `use` 구문이 많을 때 특히 빛을 발합니다.
다음 Listing 7-19는 두 `use` 구문의 예시입니다. 하나는 `std::io` 를 스코프로 가져오고,
다른 하나는 `std::io::Write` 를 스코프로 가져옵니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">Listing 7-19: 하위 경로가 같은
두 `use` 구문</span>

두 경로에서 중복되는 부분은 `std::io` 입니다.
또한 `std::io` 는 첫 번째 경로 그 자체이기도 합니다.
중첩 경로에 `self`를 작성하면 두 경로를 하나의 `use` 구문으로 합칠 수 있습니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">Listing 7-20: Listing 7-19의 두 경로를
`use` 구문 하나로 합치기</span>

한 줄로 `std::io`, `std::io::Write` 둘 다 스코프로 가져올 수 있습니다.

### 글롭 연산자

경로에 글롭 연산자 `*`를 붙이면 경로 내 정의된
*모든* 공개 항목을 가져올 수 있습니다.

```rust
use std::collections::*;
```

이 `use` 구문은 `std::collections` 내에 정의된
모든 공개 항목을 현재 스코프로 가져옵니다.
하지만 글롭 연산자는 코드에서 사용된 어떤 이름이 어느 곳에 정의되어 있는지
파악하기 어렵게 만들 수 있으므로, 사용에 주의해야 합니다.

글롭 연산자는 테스트할 모든 항목을 `tests` 모듈로
가져오는 용도로 자주 사용됩니다.
(11장 ["테스트 작성 방법"][writing-tests]<!-- ignore --> 에서 다룰 예정입니다.)
또한 프렐루드 패턴의 일부로 사용되기도 하며, 자세한 내용은
[표준 라이브러리 문서]([../std/prelude/index.html#other-preludes](https://doc.rust-lang.org/std/prelude/index.html#other-preludes))<!-- ignore --> 를
참고 바랍니다.

[rand]: ch02-00-guessing-game-tutorial.html#임의의-숫자를-생성하기
[writing-tests]: ch11-01-writing-tests.html#테스트-작성-방법
