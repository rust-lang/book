## 참조자(References)와 빌림(Borrowing)

앞 절의 마지막에 등장한 튜플을 이용하는 이슈는 `String`을 호출하는 함수 쪽으로
반환함으로써 `calculate_length`를 호출한 이후에도 여전히 `String`을 이용할 수 있도록
하는 것인데, 그 이유는 `String`이 `calculate_length` 안쪽으로 이동되었기 때문입니다.

여기 값의 소유권을 넘기는 대신 개체에 대한 *참조자*(*reference*)를 인자로 사용하는
`calculate_length` 함수를 정의하고 이용하는 방법이 있습니다:

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

첫번째로, 변수 선언부와 함수 반환값에 있던 튜플 코드가 모두 없어진 것에 주목하세요.
두번째로, `calculaate_length` 함수에 `&s1`를 넘기고, 함수의 정의 부분에는
`String`이 아니라 `&String`을 이용했다는 점을 기억하세요.

이 엠퍼센드(&) 기호가 *참조자*이며, 이는 여러분이 어떤 값을 소유권을 넘기지 않고
참조할수 있도록 해줍니다. Figure 4-8은 이에 대한 다이어그램입니다.

<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />

<span class="caption">Figure 4-8: `String s1`을 가리키고 있는 `&String s`</span>

함수 호출 부분을 좀더 자세히 봅시다:

```rust
# fn calculate_length(s: &String) -> usize {
#     s.len()
# }
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

`&s1` 문법은 우리가 `s1`의 값을 *참조*하지만 소유하지는 않는 참조자를 생성하도록
해줍니다. 소유권을 갖고 있지는 않기 때문에, 이 참조자가 가리키는 값은 참조자가 스코프
밖으로 벗어났을 때도 메모리가 반납되지 않을 것입니다.

비슷한 이치로, 함수 시그니처도 `&`를 사용하여 인자 `s`의 타입이 참조자라는 것을 나타내고
있습니다. 설명을 위한 주석을 달아봅시다:

```rust
fn calculate_length(s: &String) -> usize { // s는 String의 참조자입니다
    s.len()
} // 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기
  // 때문에, 아무런 일도 발생하지 않습니다.
```

변수 `s`가 유효한 스코프는 여느 함수의 파라미터의 스코프와 동일하지만, 소유권을 갖고
있지 않으므로 이 참조자가 스코프 밖으로 벗어났을 때 참조자가 가리키고 있는 값은 해제되지
않습니다. 또한 실제 값 대신 참조자를 파라미터로 갖고 있는 함수는 소유권을 갖고 있지
않기 때문에 소유권을 되돌려주기 위해 값을 다시 반환할 필요도 없다는 뜻이 됩니다.

함수의 파라미터로 참조자를 만드는 것을 *빌림*이라고 부릅니다. 실제 생활에서 만일
어떤 사람이 뭔가를 소유하고 있다면, 여러분은 그걸 빌릴 수 있습니다. 여러분의 용무가
끝났을 때는 그것을 돌려주어야 합니다.

그러니까 만일 우리가 빌린 무언가를 고치려고 시도한다면 무슨 일이 생길까요? Listing 4-9의
코드를 시험해보세요. 스포일러 경고: 작동이 안될겁니다!

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

<span class="caption">Listing 4-9: 빌린 값을 고치려 해보기</span>

여기 오류를 보시죠:

```text
error: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^
```

변수가 기본적으로 불변인 것처럼, 참조자도 마찬가지입니다. 우리가 참조하는 어떤 것을
변경하는 것은 허용되지 않습니다.

### 가변 참조자(Mutable References)

Listing 4-9의 코드를 살짝만 바꾸면 오류를 고칠 수 있습니다:

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

먼저 `s`를 `mut`로 바꿔야 합니다. 그리고 `&mut s`로 가변 참조자를 생성하고
`some_string: &mut String`으로 이 가변 참조자를 받아야 합니다.

하지만 가변 참조자는 딱 한가지 큰 제한이 있습니다: 특정한 스코프 내에 특정한 데이터
조각에 대한 가변 참조자를 딱 하나만 만들 수 있다는 겁니다. 아래 코드는 실패할 겁니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

Here’s the error:

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> borrow_twice.rs:5:19
  |
4 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
5 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
6 | }
  | - first borrow ends here
```

이 제한 사항은 가변을 허용하긴 하지만 매우 통제된 형식으로 허용합니다. 이것이 새로운
러스트인들이 투쟁하는 지점인데, 대부분의 언어들은 여러분이 원하는대로 값을 변형하도록
해주기 때문입니다. 하지만 이러한 제한이 가지는 이점은 바로 러스트가 컴파일 타임에
데이터 레이스(data race)를 방지할 수 있도록 해준다는 것입니다.

*데이터 레이스*는 아래에 정리된 세 가지 동작이 발생했을때 나타나는 특정한 레이스
조건입니다:

1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
1. 그 중 적어도 하나의 포인터가 데이터를 쓴다.
1. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.

데이터 레이스는 정의되지 않은 동작을 일으키고 런타임에 이를 추적하고자 할 때는
이를 진단하고 고치기 어려울 수 있습니다; 러스트는 데이터 레이스가 발생할 수 있는
코드가 컴파일 조차 안되기 때문에 이 문제의 발생을 막아버립니다!

항상 우리는 새로운 스코프를 만들기 위해 중괄호를 사용하는데, 이는 그저 *동시*에
만드는 것이 아니게 해줌으로써, 여러 개의 가변 참조자를 만들 수 있도록 해줍니다.

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // 여기서 r1은 스코프 밖으로 벗어났으므로, 우리는 아무 문제 없이 새로운 참조자를 만들 수 있습니다.

let r2 = &mut s;
```

가변 참조자와 불변 참조자를 혼용할 경우에 대한 비슷한 규칙이 있습니다. 아래 코드는
컴파일 오류가 발생합니다:

```rust,ignore
let mut s = String::from("hello");

let r1 = &s; // 문제 없음
let r2 = &s; // 문제 없음
let r3 = &mut s; // 큰 문제
```

여기 오류 메세지를 보시죠:

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as
immutable
 --> borrow_thrice.rs:6:19
  |
4 |     let r1 = &s; // 문제 없음
  |               - immutable borrow occurs here
5 |     let r2 = &s; // 문제 없음
6 |     let r3 = &mut s; // 큰 문제
  |                   ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

어휴! 우리는 불변 참조자를 가지고 있을 동안에도 *역시* 가변 참조자를 만들 수 없습니다.
불변 참조자의 사용자는 사용중인 동안에 값이 값자기 바뀌리라 예상하지 않습니다! 하지만
여러 개의 불변 참조자는 만들 수 있는데, 데이터를 그냥 읽기만하는 것은 다른 것들이 그
데이터를 읽는데에 어떠한 영향도 주지 못하기 때문입니다.

때때로 이러한 오류들이 여러분을 좌절시킬지라도, 이것이 러스트 컴파일러가 (런타임이 아니라
컴파일 타임에) 일찌감치 잠재된 버그를 찾아내고, 왜 여러분의 데이터가 여러분 생각대로의
값을 갖고 있지 않은지 추적해 내려가는 대신 어느 지점이 문제인지를 정확히 보여주는
기능이란 점을 기억하세요.

### 허상 참조자(Dangling References)

In languages with pointers, it’s easy to erroneously create a *dangling
pointer*, a pointer that references a location in memory that may have been
given to someone else, by freeing some memory while preserving a pointer to
that memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling references: if we have a reference to some data, the compiler
will ensure that the data will not go out of scope before the reference to the
data does.

Let’s try to create a dangling reference:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Here’s the error:

```text
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^^^^^^^
  |
  = help: this function's return type contains a borrowed value, but there is no
    value for it to be borrowed from
  = help: consider giving it a 'static lifetime

error: aborting due to previous error
```

This error message refers to a feature we haven’t covered yet: *lifetimes*.
We’ll discuss lifetimes in detail in Chapter 10. But, if you disregard the
parts about lifetimes, the message does contain the key to why this code is a
problem:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

Let’s take a closer look at exactly what’s happening at each stage of our
`dangle` code:

```rust,ignore
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished,
`s` will be deallocated. But we tried to return a reference to it. That means
this reference would be pointing to an invalid `String`! That’s no good. Rust
won’t let us do this.

The solution here is to return the `String` directly:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

This works without any problems. Ownership is moved out, and nothing is
deallocated.

### The Rules of References

Let’s recap what we’ve discussed about references:

1. At any given time, you can have *either* but not both of:
  * One mutable reference.
  * Any number of immutable references.
2. References must always be valid.

Next, we’ll look at a different kind of reference: slices.
