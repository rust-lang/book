## 고급 트레잇

우리는 10장의 “트레잇: 공유 동작 정의하기”절에서 먼저 트레잇을 다루었지만,
라이프타임 사용처럼 더 고급 수준의 상세한 내용을 논하지는 않았습니다.
이제 여러분이 러스트에 대해 더 많은 것을 알고 있으니, 우리는 핵심으로 다가갈 수 있습니다.

### 연관 타입은 트레잇 정의 내에서 플레이스홀더 타입을 명시합니다

*연관 타입 (associated type)* 은 타입 플레이스홀더와 트레잇을 연결하여 트레잇
메소드 정의를 할때 이 플레이스홀더 타입을 시그니처 내에서 이용할 수 있도록 합니다.
트레잇을 구현하는 사람은 이 빈칸의 타입이 특정 구현을 위해 사용될 수 있도록
구체 타입을 명시하게 됩니다. 이러한 방법으로, 우리는 트레잇이 구현되기 전까지 어떠한
타입이 필요한지 정확히 알 필요 없이 임의의 타입을 사용하는 트레잇을 정의할 수
있습니다.

우리는 이 장에서 거의 필요하지 않은 고급 기능의 대부분을 기술했습니다.
연관 타입은 그 중간 어딘가에 있습니다: 이것은 이 책의 나머지 부분에서
설명하는 기능보다 더 희귀하게 사용되지만, 이 장에서 논의하는 많은 수의
다른 기능들보다는 더 흔하게 쓰입니다.

연관 타입을 가진 트레잇의 한 예는 표준 라이브러리에서 제공하는 `Iterator`
트레잇입니다. 그 연관 타입은 `Item`이라는 이름이 붙어있고 `Iterator`
트레잇을 구현하는 타입이 반복하는 값의 타입을 대신합니다. 13장의
“`Iterator` 트레잇과 `next` 메소드”절에서, 우리는 `Iterator`
트레잇의 정의가 Listing 19-20에서 보는 바과 같다고
언급했었습니다.

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

<span class="caption">Listing 19-20: 연관 타입 `Item`을 가진
`Iterator` 트레잇의 정의</span>

타입 `Item`은 플레이스홀더 타입이고, `next` 메소드의 정의는
`Option<Self::Item>` 타입으로 된 값을 반환할 것임을 보여주고 있습니다.
`Iterator` 트레잇을 구현하는 사람은 `Item`의 구체적인 타입을 명시할 것이고,
`next` 메소드는 해당하는 구체적 타입의 값을 담고 있는 `Option`을 반환할 것입니다.

#### 연관 타입 vs. 제네릭

연관 타입이 함수를 정의할 때 어떤 타입을 다룰지 특정하지 않고서도 정의할 수
있게 해준다는 점에서, 연관 타입은 제네릭과 유사한 개념같아 보일지도 모르겠습니다.
그럼 왜 연관 타입을 이용할까요?

13장에서 `Counter` 구조체에 대한 `Iterator` 트레잇을 구현했던
예제를 가지고 두 개념 사이의 차이점을 시험해봅시다.  Listing 13-21에서,
우리는 `Item` 타입을 `u32`로 명시했었죠:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

이 문법은 제네릭과 비슷해 보입니다. 그럼 왜 Listing 19-21처럼 그냥
제네릭을 사용하여 `Iterator` 트레잇을 정의하지 않을까요?

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

<span class="caption">Listing 19-21: 제네릭을 사용한 `Iterator` 트레잇의
가상 정의</span>

그 차이점은 Listing 19-21에서처럼 제네릭을 이용할 경우, 우리는 각 구현마다
타입을 명시해야 한다는 점입니다. 그 이유는 `Iterator<String> for Counter`
이나 어떠한 다른 타입도 구현할 수 있는데, 이는 `Counter`에 대한 `Iterator`의
복수 구현을 얻을 수 있게 됩니다. 바꿔 말하면, 트레잇이 제네릭 파라미터를 가지게 될 때,
이것이 하나의 타입에 대해서 매번 제네릭 타입 파라미터의 구체적 타입을 변경해가면서
여러번 구현이 가능해진다는 것입니다. 우리가 `Counter`의 `next` 메소드를 이용할
경우, 우리는 어떤 `Iterator`의 구현체를 이용하고자 하는지를 나타내기 위해 타입
명시를 제공해야만 할 것입니다.

연관 타입을 이용하면 하나의 트레잇에 대해 여러번의 구현을 할 수 없게 되므로
타입 명시를 할 필요가 없어집니다. 연관 타입을 이용하는 Listing 19-20에서의
정의에서, 우리는 `Item`의 타입이 무엇이 될지를 한번만 선택할 수 있는데,
이는 `impl Iterator for Counter`이 한번만 나타나게 될 것이기 때문입니다.
우리는 `Counter`의 `next`를 호출하는 것마다 `u32` 값의 반복자를 요구한다고
명시할 필요가 없습니다.

### 기본 제네릭 타입 파라미터와 연산자 오버로딩

우리가 제네릭 타입 파라미터를 사용할 때, 해당 제네릭 타입에 대한 기본 구체 타입을
명시할 수 있습니다. 이는 기본 타입이 동작할 경우 트레잇을 구현할 사람이 구체 타입을
명시해야 하는 수고를 덜어줍니다. 제네릭 타입에 대한 기본 타입의 명시 문법은
제네릭 타입을 선언할 때 `<PlaceholderType=ConcreteType>`
꼴입니다.

이 테크닉이 유용한 경우 중 좋은 예가 연산자 오버로딩과 함께 쓰이는 경우입니다.
*연산자 오버로딩 (operator overloading)* 은 특정한 상황에서 (`+` 같은)
연산자의 동작을 커스터마이징 하는 것입니다.

러스트는 여러분 만의 연산자를 만들거나 임의의 연산자를 오버로딩하는 것을 허용하지는
않습니다. 하지만 여러분은 `std::ops`에 나열되어 있는 연산자와 연관된 구현하는
것으로서 연산자 및 관련된 트레잇을 오버로딩 할 수 있습니다. 예를 들어,
Listing 19-22에서는 두 개의 `Point` 인스턴스를 함께 더하기 위해서
`+` 연산자를 오버로딩 하였습니다. 이는 `Point` 구조체 상에 `Add` 트레잇을
구현하는 것으로 되었습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```

<span class="caption">Listing 19-22: `Point` 인스턴스에 대한 `+` 연산자
오버로딩을 위하여 `Add` 트레잇 구현하기</span>

`add` 메소드는 새로운 `Point`를 생성하기 위해 두 `Point` 인스턴스의
`x` 값과 `y` 값을 각각 더합니다. `Add` 트레잇은 `Output`이라는
연관 타입을 가지고 있는데 이는 `add` 메소드로부터 반환되는 타입을
결정합니다.

이 코드에서 기본 제네릭 타입은 `Add` 트레잇 내에 있습니다. 아래는
이 트레잇의 정의입니다:

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

이 코드가 일반적으로 친숙하게 보여야 합니다: 하나의 메소드와 연관 타입을 가진
트레잇 입니다. 새로운 부분은 꺽쇠 괄호 내에 있는 `RHS=Self` 부분입니다:
이 문법을 *기본 타입 파라미터* 라고 부릅니다. `RHS` 제네릭 타입 파라미터 (
“right hand side” (우변) 의 줄임말) 은 `add` 메소드의 `rhs` 파라미터의
타입을 정의합니다. 만일 우리가 `Add` 트레잇을 구현할 때 `RHS`의 구체 타입을
지정하지 않는다면, `RHS`의 타입은 기본적으로 `Self`가 될 것인데, 이는 곧
우리가 `Add`를 구현하고 있는 그 타입이 될 것입니다.

`Point`에 대하여 `Add`를 구현했을 때, 우리는 두 `Point` 인스턴스를
더하고 싶었기 때문에 `RHS`에 대한 기본 타입을 사용했습니다. 기본 타입보다
`RHS` 타입을 커스터마이징 하고 싶은 경우에서의 `Add` 트레잇 구현 예제를
살펴봅시다.

우리는 `Millimeters`와 `Meters`라는, 서로 다른 단위의 값을 가지고 있는 두 개의
구조체를 가지고 있습니다. 우리는 밀리미터 단위의 값과 미터 단위의 값을 더하고 `Add`의
구현체가 변환을 올바르게 하기를 원합니다. Listing 19-23에서 보시는 것처럼, `RHS`로
`Meters`를 사용하여 `Millimeters`에 대한 `Add`의 구현을 할 수 있습니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

<span class="caption">Listing 19-23: `Millimeters`와 `Meters`를
더하기 위해 `Millimeters` 상에 `Add` 트레잇 구현하기</span>

`Millimeters`와 `Meters`를 더하기 위해, `impl Add<Meters>`라고 명시하여
기본값 `Self` 대신 `RHS` 타입 파라미터를 지정합니다.

우리는 두가지 주요 방식 내에서 기본 타입 파라미터를 사용합니다:

* 기존 코드를 깨는 일 없이 타입을 확장하기 위해
* 대부분의 유저는 원하지 않을 특정한 상황에 대한 커스터마이징을 허용하기 위해

표준 라이브러리의 `Add` 트레잇은 두번째 목적에 맞는 예입니다: 보통 여러분은
비슷한 타입 두 개를 더할 것이지만, `Add` 트레잇은 이를 뛰어넘어서 커스터마이징
할 수 있는 기능을 제공합니다. `Add` 트레잇 정의에 있는 기본 타입 파라미터를
사용한다는 것은 대부분의 경우 여러분이 추가적인 파라미터를 명시할 필요가 없음을
뜻합니다. 바꿔 말하면, 약간의 구현 보일러 플레이트가 필요 없어서, 트레잇의
구현을 좀 더 간편하게 해준다는 말입니다.

첫번째 목적은 두번째 것과 유사하지만 방향이 반대입니다: 만일 우리가 이미 있던
트레잇에 타입 파라미터를 추가하고자 한다면, 우리가 기존 구현 코드를 깨트리는
일 없이 트레잇의 기능을 확장할 수 있도록 하기 위해 기본 파라미터를 제공할 수
있습니다.

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Nothing in Rust prevents a trait from having a method with the same name as
another trait’s method, nor does Rust prevent us from implementing both traits
on one type. It’s also possible to implement a method directly on the type with
the same name as methods from traits.

When calling methods with the same name, we need to tell Rust which one we want
to use. Consider the code in Listing 19-24 where we’ve defined two traits,
`Pilot` and `Wizard`, that both have a method called `fly`. We then implement
both traits on a type `Human` that already has a method named `fly` implemented
on it. Each `fly` method does something different.

<span class="filename">Filename: src/main.rs</span>

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

<span class="caption">Listing 19-24: Two traits defined to have a `fly` method
and implementations of those traits on the `Human` type in addition to a `fly`
method on `Human` directly</span>

When we call `fly` on an instance of `Human`, the compiler defaults to calling
the method that is directly implemented on the type, as shown in Listing 19-25.

<span class="filename">Filename: src/main.rs</span>

```rust
# trait Pilot {
#     fn fly(&self);
# }
#
# trait Wizard {
#     fn fly(&self);
# }
#
# struct Human;
#
# impl Pilot for Human {
#     fn fly(&self) {
#         println!("This is your captain speaking.");
#     }
# }
#
# impl Wizard for Human {
#     fn fly(&self) {
#         println!("Up!");
#     }
# }
#
# impl Human {
#     fn fly(&self) {
#         println!("*waving arms furiously*");
#     }
# }
#
fn main() {
    let person = Human;
    person.fly();
}
```

<span class="caption">Listing 19-25: Calling `fly` on an instance of
`Human`</span>

Running this code will print `*waving arms furiously*`, which shows that Rust
called the `fly` method implemented on `Human` directly.

To call the `fly` methods from either the `Pilot` trait or the `Wizard` trait,
we need to use more explicit syntax to specify which `fly` method we mean.
Listing 19-26 demonstrates this syntax.

<span class="filename">Filename: src/main.rs</span>

```rust
# trait Pilot {
#     fn fly(&self);
# }
#
# trait Wizard {
#     fn fly(&self);
# }
#
# struct Human;
#
# impl Pilot for Human {
#     fn fly(&self) {
#         println!("This is your captain speaking.");
#     }
# }
#
# impl Wizard for Human {
#     fn fly(&self) {
#         println!("Up!");
#     }
# }
#
# impl Human {
#     fn fly(&self) {
#         println!("*waving arms furiously*");
#     }
# }
#
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

<span class="caption">Listing 19-26: Specifying which trait’s `fly` method we
want to call</span>

Specifying the trait name before the method name clarifies to Rust which
implementation of `fly` we want to call. We could also write
`Human::fly(&person)`, which is equivalent to `person.fly()` that we used in
Listing 19-26 but is a bit longer to write if we don’t need to disambiguate.

Running this code prints the following:

```text
This is your captain speaking.
Up!
*waving arms furiously*
```

Because the `fly` method takes a `self` parameter, if we had two *types* that
both implement one *trait*, Rust can figure out which implementation of a trait
to use based on the type of `self`.

However, associated functions that are part of traits don’t have a `self`
parameter. When two types in the same scope implement that trait, Rust can’t
figure out which type we mean unless we use *fully qualified syntax*. For
example, the `Animal` trait in Listing 19-27 has the associated function
`baby_name`, the implementation of `Animal` for the struct `Dog`, and the
associated function `baby_name` defined on `Dog` directly.

<span class="filename">Filename: src/main.rs</span>

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

<span class="caption">Listing 19-27: A trait with an associated function and a
type that has an associated function with the same name that also implements
the trait</span>

This code is for an animal shelter that wants to name all puppies Spot, which
is implemented in the `baby_name` associated function that is defined on `Dog`.
The `Dog` type also implements the trait `Animal`, which describes
characteristics that all animals have. Baby dogs are called puppies, and that
is expressed in the implementation of the `Animal` trait on `Dog` in the
`baby_name` function associated with the `Animal` trait.

In `main`, we call the `Dog::baby_name` function, which calls the associated
function defined on `Dog` directly. This code prints the following:

```text
A baby dog is called a Spot
```

This output isn’t what we wanted. We want to call the `baby_name` function that
is part of the `Animal` trait that we implemented on `Dog` so the code prints
`A baby dog is called a puppy`. The technique of specifying the trait name that
we used in Listing 19-26 doesn’t help here; if we change `main` to the code in
Listing 19-28, we’ll get a compilation error.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```

<span class="caption">Listing 19-28: Attempting to call the `baby_name`
function from the `Animal` trait, but Rust doesn’t know which implementation to
use</span>

Because `Animal::baby_name` is an associated function rather than a method, and
thus doesn’t have a `self` parameter, Rust can’t figure out which
implementation of `Animal::baby_name` we want. We’ll get this compiler error:

```text
error[E0283]: type annotations required: cannot resolve `_: Animal`
  --> src/main.rs:20:43
   |
20 |     println!("A baby dog is called a {}", Animal::baby_name());
   |                                           ^^^^^^^^^^^^^^^^^
   |
   = note: required by `Animal::baby_name`
```

To disambiguate and tell Rust that we want to use the implementation of
`Animal` for `Dog`, we need to use *fully qualified syntax*, which is the most
specific we can be when calling a function. Listing 19-29 demonstrates how to
use fully qualified syntax.

<span class="filename">Filename: src/main.rs</span>

```rust
# trait Animal {
#     fn baby_name() -> String;
# }
#
# struct Dog;
#
# impl Dog {
#     fn baby_name() -> String {
#         String::from("Spot")
#     }
# }
#
# impl Animal for Dog {
#     fn baby_name() -> String {
#         String::from("puppy")
#     }
# }
#
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

<span class="caption">Listing 19-29: Using fully qualified syntax to specify
that we want to call the `baby_name` function from the `Animal` trait as
implemented on `Dog`</span>

We’re providing Rust with a type annotation within the angle brackets, which
indicates we want to call the `baby_name` method from the `Animal` trait as
implemented on `Dog` by saying that we want to treat the `Dog` type as an
`Animal` for this function call. This code will now print what we want:

```text
A baby dog is called a puppy
```

In general, fully qualified syntax is defined as follows:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

For associated functions, there would not be a `receiver`: there would only be
the list of other arguments. We could use fully qualified syntax everywhere
that we call functions or methods. However, we’re allowed to omit any part of
this syntax that Rust can figure out from other information in the program. We
only need to use this more verbose syntax in cases where there are multiple
implementations that use the same name and Rust needs help to identify which
implementation we want to call.

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

Sometimes, we might need one trait to use another trait’s functionality. In
this case, we need to rely on the dependent trait also being implemented. The
trait we’re relying on is a *supertrait* of the trait we’re implementing.

For example, let’s say we want to make an `OutlinePrint` trait with an
`outline_print` method that will print a value framed in asterisks. That is,
given a `Point` struct that implements `Display` to result in `(x, y)`, when we
call `outline_print` on a `Point` instance that has `1` for `x` and `3` for
`y`, it should print the following:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of `outline_print`, we want to use the `Display` trait’s
functionality. Therefore, we need to specify that the `OutlinePrint` trait will
only work for types that also implement `Display` and provide the functionality
that `OutlinePrint` needs. We can do that in the trait definition by specifying
`OutlinePrint: Display`. This technique is similar to adding a trait bound to
the trait. Listing 19-30 shows an implementation of the `OutlinePrint` trait:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

<span class="caption">Listing 19-30: Implementing the `OutlinePrint` trait that
requires the functionality from `Display`</span>

Because we’ve specified that `OutlinePrint` requires the `Display` trait, we
can use the `to_string` function that is automatically implemented for any type
that implements `Display`. If we tried to use `to_string` without adding `:
Display` after the trait name, we’d get an error saying that no method named
`to_string` was found for the type `&Self` in the current scope.

Let’s see what happens when we try to implement `OutlinePrint` on a type that
doesn’t implement `Display`, such as the `Point` struct:

<span class="filename">Filename: src/main.rs</span>

```rust
# trait OutlinePrint {}
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

We get an error saying that `Display` is required but not implemented:

```text
error[E0277]: the trait bound `Point: std::fmt::Display` is not satisfied
  --> src/main.rs:20:6
   |
20 | impl OutlinePrint for Point {}
   |      ^^^^^^^^^^^^ `Point` cannot be formatted with the default formatter;
try using `:?` instead if you are using a format string
   |
   = help: the trait `std::fmt::Display` is not implemented for `Point`
```

To fix this, we implement `Display` on `Point` and satisfy the constraint that
`OutlinePrint` requires, like so:

<span class="filename">Filename: src/main.rs</span>

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

Then implementing the `OutlinePrint` trait on `Point` will compile
successfully, and we can call `outline_print` on a `Point` instance to display
it within an outline of asterisks.

### The Newtype Pattern to Implement External Traits on External Types

In Chapter 10 in the “Implementing a Trait on a Type” section, we mentioned the
orphan rule that states we’re allowed to implement a trait on a type as long as
either the trait or the type are local to our crate. It’s possible to get
around this restriction using the *newtype pattern*, which involves creating a
new type in a tuple struct. (We covered tuple structs in the “Tuple Structs
without Named Fields to Create Different Types” section of Chapter 5.) The
tuple struct will have one field and be a thin wrapper around the type we want
to implement a trait for. Then the wrapper type is local to our crate, and we
can implement the trait on the wrapper. *Newtype* is a term that originates
from the Haskell programming language. There is no runtime performance penalty
for using this pattern, and the wrapper type is elided at compile time.

As an example, let’s say we want to implement `Display` on `Vec`, which the
orphan rule prevents us from doing directly because the `Display` trait and the
`Vec` type are defined outside our crate. We can make a `Wrapper` struct that
holds an instance of `Vec`; then we can implement `Display` on `Wrapper` and
use the `Vec` value, as shown in Listing 19-31.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

<span class="caption">Listing 19-31: Creating a `Wrapper` type around
`Vec<String>` to implement `Display`</span>

The implementation of `Display` uses `self.0` to access the inner `Vec`,
because `Wrapper` is a tuple struct and `Vec` is the item at index 0 in the
tuple. Then we can use the functionality of the `Display` type on `Wrapper`.

The downside of using this technique is that `Wrapper` is a new type, so it
doesn’t have the methods of the value it’s holding. We would have to implement
all the methods of `Vec` directly on `Wrapper` so it can delegate to `self.0`,
allowing us to treat `Wrapper` exactly like a `Vec`. If we wanted the new type
to have every method the inner type has, implementing the `Deref` trait
(discussed in Chapter 15 in the “Treating Smart Pointers like Regular
References with the `Deref` Trait” section) on the `Wrapper` to return the
inner type would be a solution. If we don’t want the `Wrapper` type to have all
the methods of the inner type, in order to restrict the `Wrapper` type’s
behavior for example, we would have to implement just the methods we do want
manually.

Now you know how the newtype pattern is used in relation to traits; it’s also a
useful pattern even when traits are not involved. Let’s switch focus and look
at some advanced ways to interact with Rust’s type system.
