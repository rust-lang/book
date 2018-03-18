## `RefCell<T>`와 내부 가변성 패턴

*내부 가변성 (interior mutability)* 은 어떤 데이터에 대한 불변 참조자가 있을
때라도 여러분이 데이터를 변형할 수 있게 해주는 러스트의 디자인 패턴입니다: 보통 이러한
동작은 빌림 규칙에 의해 허용되지 않습니다. 그렇게 하기 위해서, 이 패턴은 변형과 빌림을
지배하는 러스트의 통상적인 규칙을 구부리기 위하여 데이터 구조 내에서 `unsafe (안전하지
않은)` 코드를 사용합니다. 우리는 아직 안전하지 않은 코드를 다루지 않았습니다; 이는
19장에서 다룰 것입니다. 우리가 런타임에 빌림 규칙을 따를 것임을 보장할 수 있을 때라면,
심지어 컴파일러가 이를 보장하지 못하더라도 내부 가변성 패턴을 이용하는 타입을 사용할 수
있습니다. 포함되어 있는 `unsafe` 코드는 안전한 API로 감싸져 있고, 외부 타입은
여전히 불변입니다.

내부 가변성 패턴을 따르는 `RefCell<T>` 타입을 살펴보는 것으로 이 개념을
탐구해 봅시다.

### `RefCell<T>`을 가지고 런타임에 빌림 규칙을 집행하기

`Rc<T>`와는 다르게, `RefCell<T>` 타입은 가지고 있는 데이터 상에 단일 소유권을
나타냅니다. 그렇다면, `Box<T>`와 같은 타입에 비교해 `RefCell<T>`의 다른 부분은
무엇일까요? 여러분이 4장에서 배웠던 빌림 규칙을 상기해보세요:

* 어떠한 경우이든 간에, 여러분은 다음의 둘 다는 아니고 *둘 중 하나만* 가질 수 있습니다:
  하나의 가변 참조자 혹은 임의 개수의 불변 참조자들을요.
* 참조자는 항상 유효해야 합니다.

참조자와 `Box<T>`를 이용할 때, 빌림 규칙의 불변성은 컴파일 타임에 집행됩니다.
`RefCell<T>`를 이용할 때, 이 불변성은 *런타임에* 집행됩니다. 참조자를 가지고서
여러분이 이 규칙을 어기면 컴파일러 에러를 얻게 될 것입니다. `RefCell<T>`를 가지고서
여러분이 이 규칙을 어기면, 여러분의 프로그램은 `panic!`을 일으키고 종료될 것입니다.

컴파일 타임에 빌림 규칙을 검사하는 것은 개발 과정에서 에러를 더 일찍 잡을 수
있다는 점, 그리고 이 모든 분석이 사전에 완료되기 때문에 런타임 성능에 영향이
없다는 점에서 장점을 가집니다. 이러한 까닭에, 대부분의 경우 컴파일 타임에서
빌림 규칙을 검사하는 것이 가장 좋은 선택이고, 이것이 러스트의 기본 설정인
이유이기도 합니다.

대신 런타임에 빌림 규칙을 검사하는 것은 컴파일 타임 검사에 의해서는 허용되지
않는, 특정한 메모리 안정성 시나리오가 허용된다는 잇점이 있습니다. 러스트
컴파일러와 같은 정적 분석은 태생적으로 보수적입니다. 어떤 코드 속성은
코드의 분석을 이용해서는 발견이 불가능합니다: 가장 유명한 예제는 정지 문제
(halting problem) 인데, 이는 이 책의 범위를 벗어나지만 연구하기에
흘미로운 주제입니다.

몇몇 분석이 불가능하기 때문에, 만일 코드가 소유권 규칙을 준수한다는 것을 러스트
컴파일러가 확신할 수 없다면, 컴파일러는 올바를 프로그램을 거부할지도 모릅니다;
이렇게 하여, 컴파일러는 보수적입니다. 만일 러스트가 올바르지 않은 프로그램을
받아들이면, 사용자들은 러스트가 보장하는 것을 신뢰할 수 없을 것입니다. 하지만,
만일 러스트가 올바른 프로그램을 거부한다면, 프로그래머는 불편해할 것이지만,
어떠한 재앙도 일어나지 않을 수 있습니다. `RefCell<T>` 타입은 여러분의 코드가
빌림 규칙을 따르는 것을 여러분이 확신하지만, 컴파일러는 이를 이해하고 보장할 수
없을 경우 유용합니다.

`Rc<T>`와 유사하게, `RefCell<T>`은 단일 스레드 시나리오 내에서만 사용
가능하고, 만일 여러분이 이를 다중 스레드 맥락 내에서 사용을 시도할 경우 여러분에게
컴파일 타임 에러를 줄 것입니다. `RefCell<T>`의 기능을 다중 스레드 프로그램
내에서 사용하는 방법에 대해서는 16장에서 이야기할 것입니다.

`Box<T>`, `Rc<T>`, 혹은 `RefCell<T>`을 선택하는 이유의 요점은 다음과 같습니다:

* `Rc<T>`는 동일한 데이터에 대해 복수개의 소유자를 가능하게 합니다; `Box<T>`와
  `RefCell<T>`은 단일 소유자만 갖습니다.
* `Box<T>`는 컴파일 타임에 검사된 불변 혹은 가변 빌림을 허용합니다; `Rc<T>`는
  오직 컴파일 타임에 검사된 불변 빌림만 허용합니다; `RefCell<T>`는 런타임에
  검사된 불변 혹은 가변 빌림을 허용합니다.
* `RefCell<T>`이 런타임에 검사된 가변 빌림을 허용하기 때문에, `RefCell<T>`이
  불변일 때라도 `RefCell<T>` 내부의 값을 변경할 수 있습니다.

불변값 내부의 값을 변경하는 것을 *내부 가변성* 패턴이라고 합니다.
내부 가변성이 유용한 경우를 살펴보고 이것이 어떻게 가능한지 조사해
봅시다.

### 내부 가변성: 불변값에 대한 가변 빌림

빌림 규칙의 결과로 인해 우리는 불변값을 가지고 있을 때 이를 변경 가능하게
빌릴 수 없습니다. 예를 들면, 다음 코드는 컴파일되지 않을 것입니다:

```rust,ignore
fn main() {
    let x = 5;
    let y = &mut x;
}
```

이 코드의 컴파일을 시도하면, 다음과 같은 에러를 얻을 것입니다:

```text
error[E0596]: cannot borrow immutable local variable `x` as mutable
 --> src/main.rs:3:18
  |
2 |     let x = 5;
  |         - consider changing this to `mut x`
3 |     let y = &mut x;
  |                  ^ cannot borrow mutably
```

하지만, 값이 자신의 메소드 내부에서 변경되지만 다른 코드에서는 불변인
것으로 보이는 것이 유용할 수 있는 경우가 있습니다. 그 값의 메소드 바깥의
코드는 값을 변경할 수 없을 것입니다. `RefCell<T>`을 이용하는 것은
내부 가변성의 기능을 얻는 한가지 방법입니다. 그러나 `RefCell<T>`은
빌림 규칙을 완벽하게 피하는 것은 아닙니다: 컴파일러 내의 빌림 검사기는
이러한 내부 가변성을 허용하고, 빌림 규칙은 대신 런타임에 검사됩니다.
만일 이 규칙을 위반하면, 우리는 컴파일러 에러 대신 `panic!`을 얻을
것입니다.

불변 값을 변경하기 위해 `RefCell<T>`를 이용할 수 있는 실질적인 예제를
살펴보고 이것이 왜 유용한지를 알아봅시다.

#### A Use Case for Interior Mutability: Mock Objects
#### 내부 가변성에 대한 용례: 목(mock) 객체

*테스트 더블 (test double)* 은 테스트하는 동안 또다른 타입을 대신하여
사용되는 타입을 위한 일반적인 프로그래밍 개념입니다. *목 객체 (mock object)*
는 테스트 중 어떤 일이 일어났는지 기록하여 정확한 동작이 일어났음을 단언할 수
있도록 하는 테스트 더블의 특정한 타입입니다.

러스트는 다른 언어들이 객체를 가지는 것과 동일한 의미의 객체를 가지고 있지
않고, 러스트는 몇몇 다른 언어들이 제공하는 것 같은 표준 라이브러리에 미리
만들어진 목 객체 기능이 없습니다. 하지만, 우리는 목 객체와 동일한 목적을
제공할 구조체를 당연히 만들 수 있습니다.

다음은 우리가 테스트할 시나리오입니다: 우리는 최대값에 맞서 값을 추적하고
현재 값이 최대값에 얼마나 근접한지를 기반으로 메세지를 전송하는 라이브러리를
만들 것입니다. 이 라이브러리는 예를 들면 한 명의 유저에게 허용되고 있는
API 호출수의 허용량을 추적하는데 사용될 수 있습니다.

우리의 라이브러리는 오직 어떤 값이 최대값에 얼마나 근접한지를 추적하고 어떤 시간에
어떤 메세지를 보내야 할지 정하는 기능만을 제공할 것입니다. 우리의 라이브러리를 사용하는
어플리케이션이 메세지를 전송하는 것에 대한 메카니즘을 제공할 예정입니다: 그 어플리케이션은
메세지를 어플리케이션 내에 집어넣거나, 이메일을 보내거나, 문자 메세지를 보내거나, 혹은
기타 다른 것을 할 수 있습니다. 라이브러리는 그런 자세한 사항을 알 필요가 없습니다. 필요한
모든 것은 우리가 제공할 `Messenger`라는 이름의 프레잇을 구현하는 것입니다. Listing
15-20는 라이브러리 코드를 보여줍니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}
```

<span class="caption">Listing 15-20: 어떤 값이 최대값에 얼마나 근접하는지를
추적하고 특정 수준에 값이 있으면 경고해주는 라이브러리</span>

이 코드에서 한가지 중요한 부분은 `Messenger` 트레잇이 `self`에 대한
불변 참조자와 메세지의 텍스트를 인자로 갖는 `send`라는 이름의 하나의
메소드를 갖고 있다는 것입니다. 이는 우리의 목 객제가 가질 필요가 있는
인터페이스입니다. 그 외에 중요한 부분은 우리가 `LimitTracker` 상의
`set_value` 메소드의 동작을 테스트하고 싶어한다는 점입니다. 우리는
`value` 파라미터에 대해에 어떤 것을 넘길지 바꿀 수 있지만,
`set_value`는 우리가 단언을 하기 위한 어떤 것도 반환하지 않습니다.
우리는 `Messenger` 트레잇을 구현한 무언가와 `max`에 대한 특정값과 함께
`LimitTracker`를 만든다면, `value`에 대해 다른 숫자들을 넘겼을 때
메신저가 적합한 메세지를 보낸다고 말하고 싶습니다.

우리는 `send`를 호출했을 때 메일이나 텍스트 메세지를 보내는 대신 보냈다고
언급하는 메세지만 추적할 목 객체가 필요합니다. 우리는 목 객체의 새로운 인스턴스를
만들고, 이 목 객체를 사용하는 `LimitTracker`를 만들고, `LimitTracker`
상의 `set_value` 메소드를 호출하고, 그 다음 목 객체는 우리가 기대했던
메세지를 가지고 있는지를 검사할 수 있습니다.  Listing 15-21은 바로 이런
일을 하지만 빌림 검사기가 허용하지는 않을 목 객체 구현 시도를 보여주고
있습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: vec![] }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

<span class="caption">Listing 15-21: 빌림 검사기가 허용하지 않는
`MockMessenger` 구현 시도</span>

이 테스트 코드는 보내질 메세지들을 추적하기 위한 `String` 값의 `Vec`인
`sent_messages` 필드를 갖는 `MockMessenger` 구조체를 정의하고
있습니다. 우리는 또한 빈 메세지 리스트로 시작하는 새로운 `MockMessenger`
값을 생성하기 쉽도록 하기 위해 연관 함수 `new`를 정의하였습니다. 그런
다음에는 `MockMessenger`를 `LimitTracker`에 넘겨줄 수 있도록
`MockMessenger`를 위한 `Messenger` 트레잇을 구현하였습니다. `send`
메소드의 정의 부분에서는 파라미터로서 넘겨진 메세지를 가져와서 `MockMessenger`
내의 `Sent_messages` 리스트에 저장합니다.

테스트 내에서는 `max` 값의 75 퍼센트 이상의 무언가가 `value`로 설정되었을
때 `LimitTracker`는 어떤 메세지를 듣는지를 테스트하고 있습니다. 먼저 우리는
새로운 `MockMessenger`를 만드는데, 이는 비어있는 메시지 리스트로 시작할
것입니다. 그 다음에는 새로운 `LimitTracker`를 만들고 여기에 새로운
`MockMessenger`의 참조자와 `max`값 100을 파라미터로 넘깁니다. 우리는
`LimitTracker` 상의 `set_value` 메소드를 80 값으로 호출하였습니다.
그 다음 우리는 `MockMessenger`가 추적하고 있는 메세지 리스트가 이제
한 개의 메세지를 가지고 있는지를 검사합니다.

하지만, 아래에서 보는 것과 같이 이 테스트에 한가지 문제점이 있습니다:

```text
error[E0596]: cannot borrow immutable field `self.sent_messages` as mutable
  --> src/lib.rs:52:13
   |
51 |         fn send(&self, message: &str) {
   |                 ----- use `&mut self` here to make mutable
52 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ cannot mutably borrow immutable field
```

우리는 메세지를 추적하기 위해 `MockMessenger`를 수정할 수 없는데 그 이유는
`send` 메소드가 `self`의 불변 참조자를 파라미터로 갖기 때문입니다. 우리는 또한
에러 메세지로부터 `&mut self`를 대신 사용하라는 제안도 얻을 수 없는데, 그렇게
되면 `send`의 시그니처가 `Messenger` 트레잇의 정의에 있는 시그니처와 일치하지
않을 것이지 때문입니다 (마음 편하게 한번 시도해보고 어떤 에러가 나오는지 보세요).

이는 내부 가변성이 도움을 줄 수 있는 상황입니다! 우리는 `sent_messages`를
`RefCell<T>` 내에 저장할 것이고, 그러면 `send` 메소드는 우리가 보게 되는
메세지를 저장하기 위해 `sent_message`를 수정할 수 있을 것입니다. Listing
15-22는 이것이 어떤 형태인지를 보여줍니다: 

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
#         let mock_messenger = MockMessenger::new();
#         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
#         limit_tracker.set_value(75);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

<span class="caption">Listing 15-22: `RefCell<T>`를 사용하여 바깥쪽에서는
불변으로 간주되는 동안 내부의 값을 변경하기</span>

`sent_message` 필드는 이제 `Vec<String>` 대신 `RefCell<Vec<String>>`
타입입니다. `new` 함수 내에서, 우리는 빈 벡터를 감싼 새로운 `RefCell<Vec<String>>`
인스턴스를 생성합니다.

`send` 메소드의 구현부에 대하여, 첫번째 파라미터는 여전히 `self`의 불변
빌림 형태인데, 이는 트레잇의 정의와 일치합니다. 우리는 `self.sent_messages`
내의 `RefCell<Vec<String>>` 상에 있는 `borrow_mut`를 호출하여
`RefCell<Vec<String>>` 내의 값에 대한 가변 참조자를 얻는데, 이는
벡터입니다. 그런 다음에는 그 벡터에 대한 가변 참조자 상의 `push`를 호출하여
테스트하는 동안 보내진 메세지를 추적할 수 있습니다.

마지막으로 우리가 변경한 부분은 단언 부분 내에 있습니다: 내부 벡터 내에
몇개의 아이템이 있는지 보기 위해서, 우리는 `RefCell<Vec<String>>` 상의
`borrow`를 호출하여 벡터에 대한 불변 참조자를 얻습니다.

이제 여러분이 `RefCell<T>`를 어떻게 사용하는지 보았으니, 이것이 어떤 식으로 동작하는지 파고 들어 봅시다! 

#### `RefCell<T>` Keeps Track of Borrows at Runtime

When creating immutable and mutable references, we use the `&` and `&mut`
syntax, respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut`
methods, which are part of the safe API that belongs to `RefCell<T>`. The
`borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut`
returns the smart pointer type `RefMut<T>`. Both types implement `Deref` so
we can treat them like regular references.

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart
pointers are currently active. Every time we call `borrow`, the `RefCell<T>`
increases its count of how many immutable borrows are active. When a `Ref<T>`
value goes out of scope, the count of immutable borrows goes down by one. Just
like the compile time borrowing rules, `RefCell<T>` lets us have many immutable
borrows or one mutable borrow at any point in time.

If we try to violate these rules, rather than getting a compiler error like we
would with references, the implementation of `RefCell<T>` will `panic!` at
runtime. Listing 15-23 shows a modification of the implementation of `send` in
Listing 15-22. We’re deliberately trying to create two mutable borrows active
for the same scope to illustrate that `RefCell<T>` prevents us from doing this
at runtime:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

<span class="caption">Listing 15-23: Creating two mutable references in the
same scope to see that `RefCell<T>` will panic</span>

We create a variable `one_borrow` for the `RefMut<T>` smart pointer returned
from `borrow_mut`. Then we create another mutable borrow in the same way in the
variable `two_borrow`. This makes two mutable references in the same scope,
which isn’t allowed. When we run the tests for our library, the code in Listing
15-23 will compile without any errors, but the test will fail:

```text
---- tests::it_sends_an_over_75_percent_warning_message stdout ----
	thread 'tests::it_sends_an_over_75_percent_warning_message' panicked at
    'already borrowed: BorrowMutError', src/libcore/result.rs:906:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Notice that the code panicked with the message `already borrowed:
BorrowMutError`. This is how `RefCell<T>` handles violations of the borrowing
rules at runtime.

Catching borrowing errors at runtime rather than compile time means that we
would find a mistake in our code later in the development process and possibly
not even until our code was deployed to production. Also, our code will incur a
small runtime performance penalty as a result of keeping track of the borrows
at runtime rather than compile time. However, using `RefCell<T>` makes it
possible for us to write a mock object that can modify itself to keep track of
the messages it has seen while we’re using it in a context where only immutable
values are allowed. We can use `RefCell<T>` despite its trade-offs to get more
functionality than regular references give us.

### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

A common way to use `RefCell<T>` is in combination with `Rc<T>`. Recall that
`Rc<T>` lets us have multiple owners of some data, but it only gives us
immutable access to that data. If we have an `Rc<T>` that holds a `RefCell<T>`,
we can get a value that can have multiple owners *and* that we can mutate!

For example, recall the cons list example in Listing 15-18 where we used
`Rc<T>` to let us have multiple lists share ownership of another list. Because
`Rc<T>` only holds immutable values, we can’t change any of the values in the
list once we’ve created them. Let’s add in `RefCell<T>` to gain the ability to
change the values in the lists. Listing 15-24 shows that by using a
`RefCell<T>` in the `Cons` definition, we can modify the value stored in all
the lists:

<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

<span class="caption">Listing 15-24: Using `Rc<RefCell<i32>>` to create a
`List` that we can mutate</span>

We create a value that is an instance of `Rc<RefCell<i32>` and store it in a
variable named `value` so we can access it directly later. Then we create a
`List` in `a` with a `Cons` variant that holds `value`. We need to clone
`value` so both `a` and `value` have ownership of the inner `5` value rather
than transferring ownership from `value` to `a` or having `a` borrow from
`value`.

We wrap the list `a` in an `Rc<T>` so when we create lists `b` and `c`, they
can both refer to `a`, which is what we did in Listing 15-18.

After we’ve created the lists in `a`, `b`, and `c`, we add 10 to the value in
`value`. We do this by calling `borrow_mut` on `value`, which uses the
automatic dereferencing feature we discussed in Chapter 5 (see the section
“Where’s the `->` Operator?”) to dereference the `Rc<T>` to the inner
`RefCell<T>` value. The `borrow_mut` method returns a `RefMut<T>` smart
pointer, and we use the dereference operator on it and change the inner value.

When we print `a`, `b`, and `c`, we can see that they all have the modified
value of 15 rather than 5:

```text
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
```

This technique is pretty neat! By using `RefCell<T>`, we have an outwardly
immutable `List`. But we can use the methods on `RefCell<T>` that provide
access to its interior mutability so we can modify our data when we need to.
The runtime checks of the borrowing rules protect us from data races, and it’s
sometimes worth trading a bit of speed for this flexibility in our data
structures.

The standard library has other types that provide interior mutability, such as
`Cell<T>`, which is similar except that instead of giving references to the
inner value, the value is copied in and out of the `Cell<T>`. There’s also
`Mutex<T>`, which offers interior mutability that’s safe to use across threads;
we’ll discuss its use in Chapter 16. Check out the standard library docs for
more details on the differences between these types.
