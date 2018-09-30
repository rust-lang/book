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

### 모호성 방지를 위한 완전 정규화 (fully qualified) 문법: 동일한 이름의 메소드 호출하기

러스트에서는 어떤 트레잇이 다른 트레잇의 메소드와 동일한 이름의 메소드를 갖는 것을
방지할 수단이 없고, 두 트레잇을 모두 한 타입에 대해 구현 하는 것을 방지할 방법도
없습니다. 또한 어떤 타입에 대해 트레잇의 메소드와 동일한 이름을 가진 메소드를 직접
구현하는 것도 가능합니다.

동일한 이름의 메소드를 호출할 때, 우리가 어떤 걸 사용하길 원하는지 러스트에게 말해줄
필요가 있습니다. `fly`라는 이름의 메소드를 가지고 있는 `Pilot`과 `Wizard`라는
두 개의 트레잇을 정의한 Listing 19-24의 코드를 보세요. 그 다음에는 이미
`fly`라는 이름의 메소드를 가지고 있는 `Human` 타입에 대하여 두 트레잇 모두
구현하였습니다. 각각의 `fly` 메소드는 다른 일을 합니다.

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

<span class="caption">Listing 19-24: `fly` 메소드를 갖도록 정의된 두
트레잇과 `fly` 메소드를 직접 가지고 있는 `Human` 타입 상에서의 해당 트레잇들의
구현</span>

우리가 `Human` 인스턴스 상에서 `fly`를 호출할 때, Listing 19-25에서 보시는
것처럼 컴파일러는 기본적으로 그 타입에 직접 구현된 메소드를 호출합니다.

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

<span class="caption">Listing 19-25: `Human` 인스턴스 상에서 `fly`
호출하기</span>

이 코드를 실행시키면 `*waving arms furiously*`가 출력되는데, 이는 러스트가
`Human` 상에 직접 구현된 `fly` 메소드를 호출했음을 보여줍니다.

`Pilot` 트레잇 혹은 `Wizard` 트레잇으로부터 `fly` 메소드를 호출하기 위해서는
우리가 어떤 `fly` 메소드를 뜻한 것인지를 특정하기 위하여 좀더 명시적인 문법을 사용할
필요가 있습니다. Listing 19-26은 이 문법의 예시를 보여줍니다.

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

<span class="caption">Listing 19-26: 호출하길 원하는 트레잇의 `fly` 메소드
특정하기</span>

메소드 이름 앞에 트레잇 이름을 특정하는 것은 우리가 어떤 `fly` 구현체를 호출하고
싶어하는지에 대해서 러스트를 명료하게 해줍니다. 우리는 `Human::fly(&person)`이라고도
작성할 수 있는데, 이는 Listing 19-26에서 사용된 `person.fly()`와 동일한 것이나
모호하지 않기를 원할 경우 좀 더 길게 작성한 것입니다.

이 코드를 실행하면 다음과 같이 출력됩니다:

```text
This is your captain speaking.
Up!
*waving arms furiously*
```

`fly` 메소드가 `self` 파라미터를 쓰므로, 만약 하나의 *트레잇*을 구현한 두 개의
*타입*을 가지고 있다면, 러스트는 `self`의 타입에 기초하여 어떤 트레잇의 구현체인지를
알아낼 수 있습니다.

그러나, 트레잇의 일부인 연관 함수는 `self` 파라미터를 가지고 있지 않습니다.
같은 스코프 내의 두 타입이 해당 트레잇을 구현하고 있을 때, 우리가 *완전 정규화 문법*을
사용하지 않는 이상 러스트는 어떤 타입을 뜻한 것인지를 알아낼 수 없습니다. 예를 들어,
Listing 19-27에는 `baby_name`이라는 연관 함수를 가지고 있는 `animal` 트레잇,
`Dog` 구조체에 대한 `Animal`의 구현체, 그리고 `Dog`에 바로 정의된 `baby_name`
연관 함수가 있습니다.

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

<span class="caption">Listing 19-27: 연관 함수를 가지고
있는 트레잇과 이 트레잇을 구현하면서 동시에 동일한 이름의 연관 함수를
가지고 있는 타입</span>

이 코드는 모든 강아지 이름을 스팟 (Spot) 이라고 짓길 원하는 동물 보호처를 위한
것인데, 이 이름은 `Dog` 상에 정의된 `baby_name` 연관 함수 내에 구현되어 있습니다.
`Dog` 타입은 또한 `Animal` 트레잇을 구현하는데, 이는 모든 동물이 가지는 특성을
기술합니다. 아기 개는 강아지 (puppy) 라고 불는데, 이는 `Animal` 트레잇과 연관된
`baby_name` 함수 내에서 `Dog` 상에 `Animal` 트레잇을 구현한 구현체 내에
적혀 있습니다.

`main`에서는 `Dog::baby_name` 함수를 호출했는데, 이는 `Dog`에 직접 정의된
연관 함수를 호출합니다. 이 코드는 다음과 같이 출력합니다:

```text
A baby dog is called a Spot
```

이 출력은 우리가 원하던게 아니었습니다. 우리는 `Dog` 상에 구현된 `Animal`
트레잇에 속하는 `baby_name` 함수를 호출하여 코드가 `A baby dog is called a
puppy`라고 출력하길 원합니다. Listing 19-26에서 사용했던 트레잇 이름 명시
기법이 여기서는 도움이 되지 않습니다; 만일 우리가 `main`을 Listing 19-28의
코드로 변경하면, 컴파일 에러를 얻을 것입니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```

<span class="caption">Listing 19-28: `Animal` 트레잇으로부터의
`baby_name` 함수 호출 시도이지만, 러스트는 어떤 구현체를 사용하는 알지
못합니다</span>

`Animal::baby_name`이 메소드가 아닌 연관 함수이기 때문에, 그런고로 `self`
파라미터가 없기 때문에, 러스트는 `Animal::baby_name`의 어떤 구현체를 우리가
원하는 것인지 알아낼 수 없습니다. 우리는 다음과 같은 컴파일 에러를 얻게 됩니다:

```text
error[E0283]: type annotations required: cannot resolve `_: Animal`
  --> src/main.rs:20:43
   |
20 |     println!("A baby dog is called a {}", Animal::baby_name());
   |                                           ^^^^^^^^^^^^^^^^^
   |
   = note: required by `Animal::baby_name`
```

모호성을 방지하고 러스트에게 `Dog`에 대한 `Animal` 구현체를 사용하고 싶다고
알려주기 위해서는 *완전 정규화 문법*을 사용할 필요가 있는데, 이는 함수를 호출할 때
할 수 있는 한 가장 명시적인 것입니다. Listing 19-29는 완전 정규화 문법을
어떻게 사용하는지를 보여줍니다.

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

<span class="caption">Listing 19-29: 완전 정규화 문법을 사용하여
`Dog` 상에 고현된 `Animal` 트레잇의 `baby_name` 함수를 호출하고 싶다고
명시하기</span>

우리는 러스트에게 꺽쇠 괄호 내에 타입 명시를 제공하고 있는데, 이는 이번 함수를 호출할
때 `Dog` 타입을 `Animal`처럼 다루길 원한다고 말하는 것으로서 `Dog` 상에 구현된
`Animal` 트레잇의 `baby_name` 메소드를 호출하고 싶음을 나타냅니다. 이제 이 코드는
우리가 원하는 것을 출력할 것입니다:

```text
A baby dog is called a puppy
```

일반적으로, 완전 정규화 문법은 다음과 같이 정의됩니다:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

연관 함수에서는 `receiver`가 없을 것입니다: 즉 다른 인자들의 리스트만
있을 것입니다. 우리는 함수 혹은 메소드를 호출하는 모든 곳에서 완전 정규화
문법을 이용할 수도 있습니다. 그러나, 이 문법 내에서 러스트가 프로그램 내의
다른 정보로부터 알아낼 수 있는 부분은 생략이 허용됩니다. 우리는 이렇게 좀더
장황한 문법을 오직 동일한 이름을 사용하는 여러 개의 구체가 있고 러스트가 이중
어떤 것을 호출하길 원하는지를 식별하기 위해 도움이 필요할 경우만 사용하길
원합니다.

### 슈퍼트레잇 (supertrait) 을 사용하여 어떤 트레잇 내에서 다른 트레잇의 기능 요구하기

종종, 우리는 어떤 트레잇이 다른 트레잇의 기능을 이용하길 원할런지도 모릅니다.
이런 경우, 우리는 종속된 트레잇이 구현되어 있음에 의존할 필요가 있습니다.
우리가 의존 중인 트레잇이 우리가 구현하는 트레잇의 *슈퍼트레잇*입니다.

예를 들어, 어떤 값을 애스터리스크로 감싸서 출력하는 `outline_print` 라는
메소드를 가지고 있는 `OutlinePrint` 트레잇을 만들기를 원한다고 해봅시다.
즉, `(x, y)`는 결과를 내도록 `Display`를 구현한 `Point` 구조체가
주어졌을 때, `x`에 `1`과 `y`에 `3`을 가지고 있는 `Point` 인스턴스 상에서
`outline_print`를 호출하면, 다음과 같이 출력되어야 합니다:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

`outline_print`의 구현체 내에서, 우리는 `Display` 트레잇의 기능을 사용하길
원합니다. 그러므로, 우리는 `OutlinePrint` 트레잇이 `Display` 또한 구현하여
`OutlinePrint`가 필요로 하는 기능을 제공하는 타입에서만 동작할 것임을 명시할
필요가 있습니다. 이는 트레잇 정의 부분에서 `OutlinePrint: Display`라고
명시하는 것으로 할 수 있습니다. 이 기법은 트레잇에게 트레잇 바운드 추가하는 것과
유사합니다. Listing 19-30은 `OutlinePrint` 트레잇의 구현체를 보여줍니다:

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

<span class="caption">Listing 19-30: `Display`의 기능을 요구하는
`OutlinePrint` 트레잇 구현하기</span>

`OutlinePrint`가 `Display` 트레잇을 요구한다고 명시했으므로, 우리는
`Display`를 구현한 어떤 타입이든 자동으로 구현되어 있는 `to_string`
함수를 사용할 수 있습니다. 만일 트레잇 이름 뒤에 `: Display`를 추가하지
않고 `to_string`의 이용을 시도하면, 현재 스코프 내에 `&Self` 타입을
위한 `to_string` 메소드가 없다는 에러를 얻게 됩니다.

아래 `Point` 구조체처럼 `Display`를 구현하지 않은 타입에 대해
`OutlinePrint`를 구현 시도하면 어떤 일이 벌어지는지 봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
# trait OutlinePrint {}
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

우리는 `Display`가 요구되었으나 구현되지 않았다고 말하는 에러를 얻습니다:

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

이를 고치기 위해서는 아래와 같이 `Point` 상에 `Display`를 구현하여 `OutlinePrint`가
요구하는 제약사항을 만족시켜줍니다:

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

그러면 `Point` 상의 `OutlinePrint` 트레잇 구현은 성공적으로
컴파일될 것이고, 우리는 애스터리스크로 감싸진 값을 출력하기 위해
`Point` 인스턴스 상에서 `outline_print`를 호출할 수 있습니다.

### 외부 타입에 대해 외부 트레잇을 구현하기 위한 뉴타입 패턴 (newtype pattern)

10장의 “타입 상에 트레잇 구현하기”절에서, 우리는 트레잇을 구현하려면 타입 혹은
트레잇 둘 중 최소 하나는 우리의 크레이트 내의 것이어야 한다고 기술하는 고아
규칙에 대해 언급했습니다. 이러한 제약은 *뉴타입 패턴 (newtype pattern)*
을 사용하여 우회할 수 있는데, 이는 튜플 구조체 내에 사로운 타입을 만드는
것입니다. (튜플 구조체에 대해서는 5장의 “새로운 타입을 만들기 위한 이름있는
항목 없는 튜플 구조체”절에서 다루었습니다.) 튜플 구조체는 하나의 필드를 가지게
될 것이고 우리가 트레잇을 구현하길 원하는 타입을 얇게 감싸는 래퍼가 될 것입니다.
그러면 이 래퍼 타입은 우리 크레이트 내에 있게 되고, 이 래퍼에 대하여 트레잇을
구현할 수 있습니다. *뉴타입*이란  하스켈 프로그래밍 언어로부터 기원한 용어입니다.
이 패턴을 사용하는데 있어 런타임 성능 패널티는 없으며, 래퍼 타입은 컴파일할
때 생략됩니다.

한가지 예로서, 우리가 `Vec`에 대하여 `Display`을 구현하고 싶다고 가정해보면,
이는 `Display` 트레잇과 `Vec` 타입이 우리 크레이트 밖에서 정의되어 있기 때문에
고아 규칙이 이를 할 수 없게끔 방지합니다. 우리는 `Vec`의 인스턴스를 가지고 있는
`Wrapper` 구조체를 만들 수 있습니다; 그런 다음 Listing 19-31에서 보시는 것처럼
`Wrapper` 상에 `Display`를 구현하고 `Vec` 값을 이용할 수 있습니다.

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

<span class="caption">Listing 19-31: `Display`를 구현하기 위해서 `Vec<String>`을
감싼 `Wrapper` 타입 만들기</span>

`Display`의 구현체는 내부의 `Vec`에 접근하기 위해 `self.0`를 사용하는데,
이는 `Wrapper`가 튜플 구조체이고 `Vec`이 이 튜플의 0번째 아이템이기 때문입니다.
그러면 우리는 `Wrapper` 상에서 `Display` 타입의 기능을 사용할 수 있습니다.

이 기법의 부정적인 면은 `Wrapper`가 새로운 타입이므로, 들고 있는 원래
값의 메소드를 가지지 못한다는 점입니다. `Wrapper`가 정확히 `Vec`처럼
다뤄질 수 있게 하려면, `Wrapper` 상에 `Vec`의 모든 메소드들을 직접
구현하여 이를 `self.0`에게 위임할수 있게 해야할 것입니다. 만일 새로운
타입이 내부 타입이 가지고 있는 모든 메소드를 갖길 원한다면, `Wrapper`
상에 `Deref` 트레잇을 구현하는 것이 해결책이 될 수 있습니다. (`Deref`
트레잇은 15장의 “`Deref` 트레잇을 사용하여 스마트 포인터를 보통의
참조자처럼 다루기”절에서 논했었습니다.) 만일 `Wrapper` 타입이 내부
타입의 모든 메소드를 가질 필요는 없다면, 예를 들어 `Wrapper` 타입의
동작을 제약하기 위해서는, 우리가 원하는 메소드만 수동으로 구현해야 할
것입니다.

이제 여러분은 트레잇과 관련하여 뉴타입 패턴이 어떻게 사용되는지 알게 되었습니다;
이는 심지어 트레잇이 포함되어 있지 않을 때라도 윺용한 패턴입니다. 초점을 바꿔서
러스트의 타입 시스템과 상호작용하는 몇가지 고급 기법을 살펴봅시다.
