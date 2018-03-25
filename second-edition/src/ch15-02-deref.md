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
### 함수와 메소드를 이용한 암묵적 역참조 강제

*역참조 강제(deref coercion)* 는 러스트가 함수 및 메소드의 인자에
수행하는 편의성 기능입니다. 역참조 강제는 `Deref`를 구현한 어떤
타입의 참조자를 `Deref`가 본래의 타입으로부터 바꿀 수 있는 타입의
참조자로 바꿔줍니다. 역참조 강제는 우리가 특정 타입의 값에 대한
참조자를 함수 혹은 메소드의 인자로 넘기는 중 정의된 파라미터 타입에는
맞지 않을 때 자동적으로 발생합니다. 일련의 `deref` 메소드 호출은
우리가 제공한 타입을 파라미터가 요구하는 타입으로 변경해줍니다.

역참조 강제가 러스트에 도입되어서 함수와 메소드 호출을 작성하는 프로그래머들은
`&`와 `*`를 이용한 많은 수의 명시적 참조 및 역참조를 추가하지 않아도 됩니다.
역참조 강제 기능은 또한 우리가 참조자나 스마트 포인터 둘 중 어느 경우라도 작동할
수 있는 코드를 더 많이 작성할 수 있도록 해줍니다.

역참조 강제가 실제 작동하는 것을 보기 위해서, 우리가 Listing 15-8에서
정의했던 `MyBox<T>`과 Listing 15-10에서 추가했던 `Deref`의 구현체를
이용합시다. Listing 15-11은 스트링 슬라이스 파라미터를 갖는 함수의
정의를 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
```

<span class="caption">Listing 15-11: 타입 `&str`의 `name`이라는 파라미터를
갖는 `hello` 함수</span>

우리는 예를 들면 `hello("Rust");`와 같이 스트링 슬라이스를 인자로 하여 `hello` 함수를
호출할 수 있습니다. Listing 15-12에서 보는 바와 같이, 역참조 강제는 `MyBox<String>`
타입의 값에 대한 참조자를 이용하여 `hello`를 호출하는 것을 가능하게 해줍니다:

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

<span class="caption">Listing 15-12: 역참조 강제 때문에 작동되는,
`MyBox<String>` 값에 대한 참조자로 `hello` 호출하기</span>

여기서 우리는 `hello` 함수를 호출하는 인자로서 `&m`를 이용했는데, 이는
`MyBox<String>`의 참조자입니다. 우리가 Listing 15-10에서 `MyBox<T>`의
`Deref` 트레잇을 구현했기 때문에, 러스트는 `deref`를 호출하여 `&MyBox<String>`을
`&String`으로 바꿀 수 있습니다. 표준 라이브러리는 스트링 슬라이스를 반환하는
`String`의 `Deref` 구현체를 제공하는데, 이는 `Deref`에 대한 API 문서에도
있습니다. 러스트는 `deref`를 다시한번 호출하여 `&String`을 `&str`로 변환하고,
이는 `hello` 함수의 정의와 일치하게 됩니다.

만일 러스트가 역참조 강제 기능을 구현하지 않았다면, 우리는 `&MyBox<String>`
타입의 값을 가지고 `hello` 함수를 호출하는데 있어 Listing 15-12의 코드 대신
Listing 15-13의 코드를 작성해야 했을 것입니다:

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

<span class="caption">Listing 15-13: 만일 러스트에 역참조 강제가 없었다면
우리가 작성했어야 했을 코드</span>

`(*m)`은 `MyBox<String>`을 `String`로 역참조해 줍니다. 그런 다음 `&`과
`[..]`은 `hello` 시그니처와 일치되도록 전체 스트링과 동일한 `String`의
스트링 슬라이스를 얻습니다. 역참조 강제가 없는 코드는 이러한 모든 기호들이
수반된 상태에서 읽기도, 쓰기도, 이해하기도 더 힘들어집니다. 역참조 강제는
러스트가 우리를 위해 이러한 변환을 자동적으로 다룰 수 있도록 해줍니다.

`Deref` 트레잇이 관련된 타입에 대해 정의될 때, 러스트는 해당 타입을
분석하여 파라미터의 타입에 맞는 참조자를 얻기 위해 필요한 수만큼의
`Deref::deref`를 사용할 것입니다. `Deref::deref`가 삽입될 필요가 있는
횟수는 컴파일 타임에 분석되므로, 역참조 강제의 이점을 얻는데에 관해서
어떠한 런타임 페널티도 없습니다!

### 역참조 강제가 가변성과 상호작용 하는 법

불변 참조자에 대한 `*`를 오버라이딩 하기 위해 `Deref` 트레잇을 이용하는
방법과 비슷하게, 러스트는 가변 참조자에 대한 `*`를 오버라이딩 하기 위한
`DerefMut` 트레잇을 제공합니다.

러스트는 다음의 세 가지 경우에 해당하는 타입과 트레잇 구현을 찾았을 때
역참조 강제를 수행합니다: 

* `T: Deref<Target=U>`일때 `&T`에서 `&U`로
* `T: DerefMut<Target=U>`일때 `&mut T`에서 `&mut U`로
* `T: Deref<Target=U>`일때 `&mut T`에서 `&U`로

첫 두가지 경우는 가변성 부분만 제외하고는 동일합니다. 첫번째 경우는 만일 여러분이
`&T`를 가지고 있고, `T`가 어떤 타입 `U`에 대한 `Deref`를 구현했다면, 여러분은
명료하게 `&U`를 얻을 수 있음을 기술하고 있습니다. 두번째 경우는 동일한 역참조
강제가 가변 참조자에 대해서도 발생함을 기술합니다.

세번째 경우는 좀 더 교묘합니다: 러스트는 가변 참조자를 불변 참조자로 강제할
수도 있습니다. 하지만 그 역은 *불가능합니다*: 불변 참조자는 가변 참조자로 결코
강제되지 않을 것입니다. 빌림 규칙 때문에, 만일 여러분이 가변 참조자를 가지고
있다면, 그 가변 참조자는 해당 데이터에 대한 유일한 참조자임에 틀림 없습니다
(만일 그렇지 않다면, 그 프로그램은 컴파일되지 않을 것입니다). 가변 참조자를
불변 참조자로 변경하는 것은 결코 빌림 규칙을 깨트리지 않을 것입니다. 불변
참조자를 가변 참조자로 변경하는 것은 해당 데이터에 대한 단 하나의 불변 참조자가
있어야 한다는 요구를 하게 되고, 이는 빌림 규칙이 보장해줄 수 없습니다. 따라서,
러스트는 불변 참조자를 가변 참조자로 변경하는 것이 가능하다는 가정을 할 수
없습니다.
