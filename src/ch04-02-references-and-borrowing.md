## 참조자와 Borrow

앞 절 마지막에 등장한 Listing 4-5 에선 `String` 이 `calculate_length` 로
이동해버린 것 때문에 `calclulate_length` 를 호출한 함수로 `String` 을 반환하여,
함수 호출 이후에도 `String` 을 사용할 수 있게
하였습니다.

이번에는 값의 소유권을 넘기는 대신
개체의 *참조자(reference)* 를 넘겨주는 방법을 소개하도록 하겠습니다.
다음은 참조자를 인자로 받도록 구현한 `calculate_length` 함수의 정의 및 용례입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

먼저, 변수 선언부와 함수 반환 값에 위치하던
튜플 코드가 전부 사라진 것을 볼 수 있습니다.
또한 `calculate_length` 함수에 `s1` 대신 `&s1` 을 넘기고,
함수 정의에 `String` 대신 `&String` 을 사용했네요.

여기 사용된 앰퍼샌드(&) 기호가 바로 *참조자* 입니다.
참조자를 사용하면 여러분이 어떤 값의 소유권을 갖지 않고도 해당 값을 참조할 수 있죠. 어떤 원리인지 Figure 4-5 다이어그램으로 알아보겠습니다:

<img alt="&String s 는 String s1 을 가리킵니다" src="img/trpl04-05.svg" class="center" />

<span class="caption">Figure 4-5: `&String s` 는 `String s1` 을
가리킴</span>

> Note: `&` 를 이용한 참조의 반대는
> *역참조(dereferencing)* 라 합니다.
> 역참조 기호는 `*` 이며, 8장 에서 몇 번 다뤄보고
> 15장에서 자세한 내용을 배울 예정입니다.

함수 호출부를 좀 더 자세히 살펴봅시다:

```rust
# fn calculate_length(s: &String) -> usize {
#     s.len()
# }
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

`s1` 에 `&` 를 붙인 `&s1` 구문은 `s1` 값을 참조하나,
해당 값을 소유하지 않는 참조자를 생성합니다.
함수 정의에서도 마찬가지로 `&` 를 사용하여 매개변수 `s` 가 참조자 타입임을 나타내고 있죠.

참조자는 소유권을 갖지 않으므로, 스코프를 벗어나도 값은 drop 되지 않습니다.
주석으로 보여드리겠습니다.

```rust
fn calculate_length(s: &String) -> usize { // s 는 String 참조자입니다.
    s.len()
} // 이 지점에서 s 는 스코프를 벗어나지만,
  // 참조자 s 는 소유권을 갖고 있지 않기 때문에 아무 일도 일어나지 않습니다.
```

변수 `s` 가 유효한 스코프는
여타 함수의 매개변수에 적용되는 스코프와 동일합니다.
하지만 참조자에는 스코프를 벗어났을 때 값이 drop 되지 않는다는 차이점이 있고,
따라서 참조자를 매개변수로 갖는 함수는 소유권을 되돌려주기 위해
값을 다시 반환할 필요도 없습니다.

또한, 이처럼 참조자를 매개변수로 사용하는 것을 *borrow(빌림)* 이라 합니다.
현실에서도 다른 사람이 소유하고 있는 뭔가를 빌리고,
용무가 끝나면 돌려주는 것처럼요.

그럼 borrow 한 값을 수정하면 어떻게 될까요?
Listing 4-9 코드를 실행해보면 알 수 있으나, 미리 말씀드리자면 이 코드는 작동하지 않습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

<span class="caption">Listing 4-6: borrow 한 값을 수정해보는 코드</span>

나타나는 오류는 다음과 같습니다:

```text
error[E0596]: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- use `&mut String` here to make mutable
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ cannot borrow as mutable
```

변수가 기본적으로 불변성을 지니듯,
레퍼런스도 마찬가지로 참조하는 것을 수정할 수 없습니다.

### 가변 참조자 (Mutable Reference)

Listing 4-6 을 살짝 수정하여 오류를 없애보죠:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

`s` 를 `mut` 로 변경하고,
참조자 생성 코드를 `&mut s` 로 변경해 가변 참조자를 생성하게 만든 뒤,
함수에서 가변 참조자를 넘겨받도록 `some_string: &mut String` 으로 수정하는 겁니다.

다만, 가변 참조자에는 특정 스코프 내 어떤 데이터를 가리키는 가변 참조자를
딱 하나만 만들 수 있다는 제한이 있다는 걸 알아두세요.
즉, 다음 코드는 작동하지 않습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

나타나는 오류는 다음과 같습니다:

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here
```

이 제약으로 인해 가변 참조자는 남용이 불가능합니다.
대부분의 언어에선 원하는 대로 값을 변경할 수 있으니,
러스트 입문자가 익숙해지지 못해 고생하는 원인이기도 하죠.

하지만, 이 제약 덕분에 러스트에선 컴파일 타임에 *데이터 레이스(data race)* 를 방지할 수 있습니다.
데이터 레이스란 다음 세 가지 상황이 겹칠 때 일어나는
특정한 레이스 조건(race condition)입니다:

1. 둘 이상의 포인터가 동시에 같은 데이터에 접근
2. 포인터 중 하나 이상이 데이터에 쓰기 작업을 시행
3. 데이터 접근 동기화 매커니즘이 존재하지 않음

데이터 레이스는 정의되지 않은 동작을 일으키며,
런타임에 추적하려고 할 때 문제 진단 및 수정이 어렵습니다.
하지만 러스트에선 데이터 레이스가 발생할 가능성이 있는 코드는 아예 컴파일되지 않으니 걱정할 필요가 없죠.

그럼 가변 참조자를 여러 개 생성하는 법은 뭘까요?
중괄호로 새로운 스코프를 만들어, 가변 참조자가 동시에 여러개가 존재하는 상황을 회피하는 방법이 있습니다:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 은 여기서 스코프를 벗어나니, 문제없이 새 참조자를 만들 수 있습니다.

let r2 = &mut s;
```

가변 참조자와 불변 참조자를 혼용할 때도 유사한 제약이 적용되는데,
다음 코드는 컴파일 오류가 발생합니다:

```rust,ignore,does_not_compile
let mut s = String::from("hello");

let r1 = &s; // 문제 없음
let r2 = &s; // 문제 없음
let r3 = &mut s; // 이 부분이 문제가 됩니다

println!("{}, {}, and {}", r1, r2, r3);
```

나타나는 오류는 다음과 같습니다:

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // 문제 없음
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // 문제 없음
6 |     let r3 = &mut s; // 이 부분이 문제가 됩니다
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here
```

안타깝게도, 가변 참조자는 불변 참조자가 존재하는 동안에도 생성할 수 없습니다.
불변 참조자를 사용할 때 가변 참조자로 인해
값이 중간에 변경되리라 예상하지 않으니까요.
반면 불변 참조자는 데이터를 읽기만 하니 외부에 영향을 주지 않아
여러 개를 만들 수 있습니다.

참조자는 정의된 지점부터 시작해,
해당 참조자가 마지막으로 사용된 부분까지 유효합니다.
즉, 다음 코드는 가변 참조자 정의가 불변 참조자의 마지막 사용 이후에 있으므로
컴파일 오류가 발생하지 않습니다.

<!-- This example is being ignored because there's a bug in rustdoc making the
edition2018 not work. The bug is currently fixed in nightly, so when we update
the book to >= 1.35, `ignore` can be removed from this example. -->

```rust,edition2018,ignore
let mut s = String::from("hello");

let r1 = &s; // 문제 없음
let r2 = &s; // 문제 없음
println!("{} and {}", r1, r2);
// 이 지점 이후로 r1, r2 는 사용되지 않음

let r3 = &mut s; // 문제 없음
println!("{}", r3);
```

불변 참조자 `rl`, `r2` 의 스코프는 자신들이 마지막으로 사용된 `println!` 이후로 종료되고,
해당 `println!` 은 가변 참조자 `r3` 가 생성되기 이전이니
서로 스코프가 겹치지 않아서 이 코드는 문제가 없는 것이죠.

이러한 제약 때문에 머리 아플 수도 있습니다.
하지만 이는 코드에 숨어 있는 버그를 러스트 컴파일러가 컴파일 타임에 일찌감치 찾아내고,
런타임 중 원하는 데이터가 나타나지 않았을 때 원인을 찾기 위해 여러분이 하나하나 추적하는 대신
어느 부분이 문제인지 정확히 집어주는 기능이란 점을 기억해주세요.

### 댕글링 참조자(Dangling Reference)

*댕글링 포인터(dangling pointer)* 란,
어떤 메모리를 가리키는 포인터가 남아있는 상황에서 해당 메모리를 해제해버림으로써,
다른 개체가 할당받았을지도 모르는 메모리를 참조하게 된 포인터를 말합니다.
포인터가 있는 언어에선 자칫 잘못하면 이 댕글링 포인터를 만들기 쉽죠.
하지만 러스트에선 어떤 데이터의 참조자를 만들면,
해당 참조자가 스코프를 벗어나기 이전에 데이터가 먼저 스코프를 벗어나는지
컴파일러에서 확인하여 댕글링 참조자가 생성되지 않도록 보장합니다.

한번, 컴파일 타임 에러가 발생할 만한
댕글링 참조자를 만들어 봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

나타나는 오류는 다음과 같습니다:

```text
error[E0106]: missing lifetime specifier
 --> main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

아직 다루지 않은 라이프타임이라는 내용이 에러 메시지에 등장하는데,
라이프타임은 10장에서 다룰 예정이니 일단 무시하도록 하겠습니다.
이 코드가 문제가 되는 이유를 알려주는 핵심 내용은 다음과 같습니다:

```text
this function's return type contains a borrowed value, but there is no value for it to be borrowed from.
(해석: 이 함수는 borrow 한 값을 반환하고 있으나, borrow 한 실제 값이 존재하지 않습니다.)
```

`dangle` 함수에서 어떤 일이 일어나는지
단계별로 알아봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn dangle() -> &String { // dangle 함수는 String 참조자를 반환합니다

    let s = String::from("hello"); // s 는 새로운 String 입니다

    &s // String 참조자 s 를 반환합니다.
} // 하지만 s 는 스코프를 벗어났고, drop 되었습니다. 메모리도 사라졌죠.
  // 문제가 생겼네요.
```

`s` 는 `dangle` 함수 내에서 생성됐기 때문에,
함수가 끝날 때 할당 해제됩니다.
하지만 코드에선 `s` 를 반환하려 했고, 이는 유효하지 않은 `String` 을 가리키는
참조자를 반환하는 행위이기 때문에 오류가 발생합니다.

따라서, 이런 경우엔 `String` 을 직접 반환해야 합니다:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

이 코드는 정상적으로 작동합니다.
소유권은 이동되며, 할당 해제되지도 않죠.

### 참조자 규칙

배운 내용을 정리해 봅시다:

* 여러분은 단 하나의 가변 참조자만 갖거나,
  여러 개의 불변 참조자를 가질 수 있습니다.
* 참조자는 항상 유효해야 합니다.

다음으로 알아볼 것은 참조자의 또 다른 종류인 슬라이스(slice) 입니다.
