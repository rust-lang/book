## Deref 트레잇을 가지고 스마트 포인터를 평범한 참조자와 같이 취급하기

Deref 트레잇을 구현하는 것은 우리가 (곱하기 혹은 글롭 연산자와는 반대측에 있는)
*역참조 연산자 (dereference operator)* `*` 의 동작을 커스터마이징 하는
것을 허용합니다. 스마트 포인터가 평범한 참조자처럼 취급될 수 있는 방식으로 Deref를
구현함으로써, 우리는 참조자에 대해 작동하는 코드를 작성하고 이 코드를 또한 스마트
포인터에도 사용할 수 있습니다.

먼저 `*`가 보통의 참조자와 어떤식으로 동작하는지를 살펴보고, 그런 다음 `Box<T>`와
비슷한 우리만의 타입을 정의하는 시도를 해서 왜 `*`가 우리의 새로 정의된 타입에서는
참조자처럼 작동하지 않는지를 봅시다. 우리는 `Defer` 트레잇을 구현하는 것이 어떻게
스마트 포인터가 참조자와 유사한 방식으로 동작하는 것을 가능하게 해주는지를 탐구할
것입니다. 그런 뒤 러스트의 *역참조 강제 (deref corecion)* 기능과 이 기능이 어떻게
참조자 혹은 스마트 포인터와 함께 동작하도록 하는지 살펴보겠습니다.

### `*`와 함께 포인터를 따라가서 값을 얻기

보통의 참조자는 포인터 타입이며, 포인터를 생각하는 한가지 방법은 다른 어딘가에
저장된 값을 가리키는 화살표로서 생각하는 것입니다. Listing 15-6에서는 `i32`
값의 참조자를 생성하고는 참조자를 따라가서 값을 얻기 위해 역참조 연산자를
사용합니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<span class="caption">Listing 15-6: 역참조 연산자를 사용하여 `i32` 값에 대한
참조자를 따라가기</span>

변수 `x`는 `i32` 값을 가지고 있습니다. `y`에는 `x`의 참조자를 설정했습니다.
우리는 `x`가 `5`와 동일함을 단언할 수 있습니다. 하지만, 만일 `y` 안의 값에
대한 단언을 만들고 싶다면, 참조자를 따라가서 이 참조자가 가리키고 있는 값을
얻기 위해 `*y`를 사용해야 합니다 (그래서 *역참조*라 합니다). 일단 `y`를
역참조하면, `5`와 비교 가능한 `y`가 가리키고 있는 정수값에 접근하게
됩니다.

대신 `assert_eq!(5, y);`라고 작성하길 시도했다면, 아래와 같은 컴파일 에러를
얻을 것입니다:

```text
error[E0277]: the trait bound `{integer}: std::cmp::PartialEq<&{integer}>` is
not satisfied
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^^ can't compare `{integer}` with `&{integer}`
  |
  = help: the trait `std::cmp::PartialEq<&{integer}>` is not implemented for
  `{integer}`
```

숫자와 숫자에 대한 참조자를 비교하는 것은 허용되지 않는데 그 이유는 이들이 서로
다른 타입이기 때문입니다. `*`를 사용하여 해당 잠조자를 따라가서 그것이 가리키고 있는
값을 얻어야 합니다.

### `Box<T>`를 참조자처럼 사용하기

Listing 15-7에서 보는 바와 같이, Listing 15-6의 코드는 참조자 대신
`Box<T>`를 이용하여 재작성될 수 있으며, 역참조 연산자는 동일한 방식으로
작동될 것입니다:


<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<span class="caption">Listing 15-7: `Box<i32>` 상에 역참조 연산자
사용하기</span>

Listing 15-7와 Listing 15-6 사이의 차이점은 오직 `x`의 값을 가리키는
참조자보다는 `x`를 가리키는 박스의 인스턴스로 `y`를 설정했다는 것입니다.
마지막 단언문에서, 우리는 `y`가 참조자일때 했던 것과 동일한 방식으로 박스
포인터 앞에 역참조 연산자를 사용할 수 있습니다. 다음으로, 우리만의 박스 타입을
정의함으로써 `Box<T>`가 우리에게 역참조 연산자를 사용 가능하게끔 해주는
특별함이 무엇인지 탐구해 보겠습니다.

### 우리만의 스마트 포인터 정의하기

어떤 식으로 스마트 포인터가 기본적으로 참조자와는 다르게 동작하는지를
경험하기 위해서, 표준 라이브러리가 제공하는 `Box<T>` 타입과 유사한
스마트 포인터를 만들어 봅시다. 그런 다음 어떻게 역참조 연산자를 사용할
수 있는 기능을 추가하는지 살펴보겠습니다.

`Box<T>` 타입은 궁극적으로 하나의 요소를 가진 튜플 구조체로 정의되므로,
Listing 15-8은 `MyBox<T>` 타입을 동일한 방식으로 정의하였습니다. 또한
`Box<T>`에 정의되어 있는 `new` 함수에 맞추기 위해 `new` 함수도 정의겠습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

<span class="caption">Listing 15-8: `MyBox<T>` 타입 정의하기</span>

우리는 `MyBox`라는 이름의 구조체를 정의하고 제네릭 파라미터 `T`를 선언했는데, 이는
우리의 타입이 어떠한 종류의 타입 값이든 가질 수 있길 원하기 때문입니다. `MyBox`
타입은 `T` 타입의 하나의 요소를 가진 튜플 구조체입니다. `MyBox::new` 함수는
`T` 타입인 하나의 파라미터를 받아서 그 값을 갖는 `MyBox` 인스턴스를 반환합니다.

Lisint 15-7의 `main` 함수를 Listing 15-8에 추가하고 `Box<T>` 대신 우리가
정의한 `MyBox<T>`를 이용하도록 수정해봅시다. Listing 15-9는 컴파일되지
않을 것인데 그 이유는 러스트가 `MyBox`를 어떻게 역참조하는지 모르기
때문입니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<span class="caption">Listing 15-9: 참조자와 `Box<T>`를 사용한 것과 동일한
방식으로 `MyBox<T>` 사용 시도하기</span>

아래는 그 결과 발생한 컴파일 에러입니다:

```text
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^
```

우리의 `MyBox<T>` 타입은 역참조될 수 없는데 그 이유는 우리의 타입에 대해
해당 기능을 아직 구현하지 않았기 때문입니다. `*` 연산자로 역참조를 가능케
하기 위해서, 우리는 `Deref` 트레잇을 구현합니다.

### `Deref` 트레잇을 구현하여 임의의 타입을 참조자처럼 다루기

10장에서 논의한 바와 같이, 트레잇을 구현하기 위해서는 트레잇의 요구 메소드들에
대한 구현체를 제공할 필요가 있습니다. 표준 라이브러리가 제공하는 `Deref` 트레잇은
우리에게 `self`를 빌려서 내부 데이터에 대한 참조자를 반환하는 `deref`라는 이름의
메소드 하나를 구현하도록 요구합니다. Listing 15-10은 `MyBox`의 정의에 덧붙여
`Deref`의 구현을 담고 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::ops::Deref;

# struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

<span class="caption">Listing 15-10: `MyBox<T>` 상의 `Deref` 구현</span>

`type Target = T;` 문법은 `Deref` 트레잇이 사용할 연관 타입 (associated type)
을 정의합니다. 연관 타입은 제네릭 파라미터를 정의하는 것과 약간 다른 방식이지만,
여러분은 지금 이를 걱정할 필요는 없습니다; 우리는 이를 19장에서 더 자세히 다룰
것입니다.

우리는 `deref` 메소드의 본체를 `&self.0`로 채웠으므로 `deref`는 우리가 `*`
연산자를 이용해 접근하고자 하는 값의 참조자를 반환합니다. `MyBox<T>` 값에
대하여 `*`을 호출하는 Listing 15-9의 `main` 함수는 이제 컴파일되고 단언문은
통과됩니다!

`Deref` 트레잇 없이, 컴파일러는 오직 `&` 참조자들만 역참조할 수 있습니다.
`deref` 메소드는 컴파일러에게 `Deref`를 구현한 어떠한 타입의 값을 가지고
`&` 참조자를 가져오기 위해서 어떻게 역참조하는지 알고 있는 `deref` 메소드를
호출하는 기능을 부여합니다.

Listing 15-9의 `*y`에 들어설 때, 무대 뒤에서 러스트는 실제로 아래의 코드를
실행했습니다:

```rust,ignore
*(y.deref())
```

러스트는 `*` 연산자에 `deref` 메소드 호출 후 보통의 역참조를 대입하므로
프로그래머로서 우리는 `deref` 메소드를 호출할 필요가 있는지 혹은 없는지를
생각하지 않아도 됩니다. 이 러스트의 기능은 우리가 보통의 참조자를 가지고
있는 경우 혹은 `Deref`를 구현한 타입을 가지고 있는 경우에 대하여 동일하게
기능하는 코드를 작성하도록 해줍니다.

`deref` 메소드가 값의 참조자를 반환하고 `*(y.deref())`에서의 괄호
바깥의 평범한 역참조가 여전히 필요한 이유는 소유권 시스템 때문입니다.
만일 `deref` 메소드가 값의 참조자 대신 값을 직접 반환했다면, 그 값은
`self` 바깥으로 이동될 것입니다. 위의 경우 및 우리가 역참조 연산자를
사용하는 대부분의 경우에서 우리는 `MyBox<T>` 내부의 값에 대한 소유권을
얻길 원치 않습니다.

우리의 코드에 `*`를 한번 타이핑할 때마다, `*`는 `deref` 함수의 호출 후
`*`를 한번 호출하는 것으로 대치된다는 점을 기억하세요. `*`의 대입이
무한히 재귀적으로 실행되지 않기 때문에, 우리는 결국 `i32` 타입의 데이터를
얻는데, 이는 Listing 15-9의 `assert_eq!` 내의 `5`와 일치합니다.

### Implicit Deref Coercions with Functions and Methods

*Deref coercion* is a convenience that Rust performs on arguments to functions
and methods. Deref coercion converts a reference to a type that implements
`Deref` into a reference to a type that `Deref` can convert the original type
into. Deref coercion happens automatically when we pass a reference to a
particular type’s value as an argument to a function or method that doesn’t
match the parameter type in the function or method definition. A sequence of
calls to the `deref` method converts the type we provided into the type the
parameter needs.

Deref coercion was added to Rust so that programmers writing function and
method calls don’t need to add as many explicit references and dereferences
with `&` and `*`. The deref coercion feature also lets us write more code that
can work for either references or smart pointers.

To see deref coercion in action, let’s use the `MyBox<T>` type we defined in
Listing 15-8 as well as the implementation of `Deref` that we added in Listing
15-10. Listing 15-11 shows the definition of a function that has a string slice
parameter:

<span class="filename">Filename: src/main.rs</span>

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
```

<span class="caption">Listing 15-11: A `hello` function that has the parameter
`name` of type `&str`</span>

We can call the `hello` function with a string slice as an argument, such as
`hello("Rust");` for example. Deref coercion makes it possible to call `hello`
with a reference to a value of type `MyBox<String>`, as shown in Listing 15-12:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::ops::Deref;
#
# struct MyBox<T>(T);
#
# impl<T> MyBox<T> {
#     fn new(x: T) -> MyBox<T> {
#         MyBox(x)
#     }
# }
#
# impl<T> Deref for MyBox<T> {
#     type Target = T;
#
#     fn deref(&self) -> &T {
#         &self.0
#     }
# }
#
# fn hello(name: &str) {
#     println!("Hello, {}!", name);
# }
#
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

<span class="caption">Listing 15-12: Calling `hello` with a reference to a
`MyBox<String>` value, which works because of deref coercion</span>

Here we’re calling the `hello` function with the argument `&m`, which is a
reference to a `MyBox<String>` value. Because we implemented the `Deref` trait
on `MyBox<T>` in Listing 15-10, Rust can turn `&MyBox<String>` into `&String`
by calling `deref`. The standard library provides an implementation of `Deref`
on `String` that returns a string slice, which is in the API documentation for
`Deref`. Rust calls `deref` again to turn the `&String` into `&str`, which
matches the `hello` function’s definition.

If Rust didn’t implement deref coercion, we would have to write the code in
Listing 15-13 instead of the code in Listing 15-12 to call `hello` with a value
of type `&MyBox<String>`:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::ops::Deref;
#
# struct MyBox<T>(T);
#
# impl<T> MyBox<T> {
#     fn new(x: T) -> MyBox<T> {
#         MyBox(x)
#     }
# }
#
# impl<T> Deref for MyBox<T> {
#     type Target = T;
#
#     fn deref(&self) -> &T {
#         &self.0
#     }
# }
#
# fn hello(name: &str) {
#     println!("Hello, {}!", name);
# }
#
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

<span class="caption">Listing 15-13: The code we would have to write if Rust
didn’t have deref coercion</span>

The `(*m)` dereferences the `MyBox<String>` into a `String`. Then the `&` and
`[..]` take a string slice of the `String` that is equal to the whole string to
match the signature of `hello`. The code without deref coercions is harder to
read, write, and understand with all of these symbols involved. Deref coercion
allows Rust to handle these conversions for us automatically.

When the `Deref` trait is defined for the types involved, Rust will analyze the
types and use `Deref::deref` as many times as necessary to get a reference to
match the parameter’s type. The number of times that `Deref::deref` needs to be
inserted is resolved at compile time, so there is no runtime penalty for taking
advantage of deref coercion!

### How Deref Coercion Interacts with Mutability

Similar to how we use the `Deref` trait to override `*` on immutable
references, Rust provides a `DerefMut` trait for overriding `*` on mutable
references.

Rust does deref coercion when it finds types and trait implementations in three
cases:

* From `&T` to `&U` when `T: Deref<Target=U>`
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
* From `&mut T` to `&U` when `T: Deref<Target=U>`

The first two cases are the same except for mutability. The first case states
that if you have a `&T`, and `T` implements `Deref` to some type `U`, you can
get a `&U` transparently. The second case states that the same deref coercion
happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an
immutable one. But the reverse is *not* possible: immutable references will
never coerce to mutable references. Because of the borrowing rules, if you have
a mutable reference, that mutable reference must be the only reference to that
data (otherwise, the program wouldn’t compile). Converting one mutable
reference to one immutable reference will never break the borrowing rules.
Converting an immutable reference to a mutable reference would require that
there is only one immutable reference to that data, and the borrowing rules
don’t guarantee that. Therefore, Rust can’t make the assumption that converting
an immutable reference to a mutable reference is possible.
