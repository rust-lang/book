## 고급 라이프타임

10장의 “라이프타임을 이용한 참조자 유효화”절에서, 여러분은 러스트에게 서로 다른
참조자의 라이프타임이 어떻게 연관되는지를 알려주기 위하여 참조자에 대한 라이프타임
파라미터의 명시 방법을 배웠습니다. 여러분은 모든 참조자가 라이프타임을 갖지만,
거의 대부분의 경우 러스트가 어떻게 이 라이프타임을 생략시켜주는지도 봤습니다.
이제 우리는 아직 다루지 못했던 라이프타임의 세가지 고급 기능을 살펴볼 것입니다:

* 라이프타임 서브타이핑 (subtyping): 한 라이프타임이 다른 라이프타임보다 오래 사는 것을 보장하기
* 라이프타임 바운드: 제네릭 타입을 가리키는 참조자를 위한 라이프타임 명시하기
* 트레잇 객체 라이프타임의 추론: 컴파일러는 어떻게 트레잇 객체의 라이프타임을 추론하며
  언제 이들을 명시할 필요가 있는지에 대하여

### 라이프타임 서브타이핑은 하나의 라이프타임이 다른 것보다 오래 사는 것을 보장합니다

*라이프타임 서브타이핑*은 하나의 라이프타임이 다른 라이프타임보다 오래 살아야 함을
명시합니다. 라이프타임 서브타이핑을 탐구하기 위해서, 우리가 파서를 작성하길 원한다고
상상해 보세요. 우리가 파싱하는 중인 스트링에 대한 참조자를 가지고 있는 `Context`라는
이름의 구조체를 사용하겠습니다. 이 스트링을 파싱하고 성공 혹은 실패를 반환하는 파서를
작성할 것입니다. 이 파서는 파싱을 하기 위해 `Context`를 빌릴 필요가 있을 것입니다.
Listing 19-12는 이 파서 코드를 구현한 것인데, 필요한 라이프타임 명시가 제외되어
있고, 따라서 컴파일되지 않습니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
struct Context(&str);

struct Parser {
    context: &Context,
}

impl Parser {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

<span class="caption">Listing 19-12: 라이프타임 명시 없이 파서를
정의하기</span>

이 코드를 컴파일하면 에러를 내는데 그 이유는 러스트가 `Context`의 스트링 슬라이스와
`Parser` 내의 `Context`의 참조자에 대한 라이프타임 파라미터를 기대하기 때문입니다.

단순함을 위해서, 이 `parse` 함수는 `Result<(), &str>`를 리턴합니다.
즉, 이 함수는 성공시에 아무것도 하지 않고, 실패시에는 파싱이 올바르기 되지
않은 스트링 슬라이스 부분을 반환할 것입니다. 실제 구현은 더 많은 에러 정보를
제공하고 파싱이 성공하면 구조화된 데이터 타입을 반환할 것입니다. 우리는
이러한 상세 부분은 다루지 않을 것인데, 이 예제의 라이프타임 부분과는
관련이 없기 때문입니다.

이 코드를 계속 단순하게 유지하기 위해, 우리는 어떠한 파싱 로직도 작성하지
않고 있습니다. 하지만, 유효하지 않은 입력을 처리하기 위하여 파싱 로식의
어딘가에서 잘못된 입력 부분을 참조하는 에러를 반환하기란 매우 가능성이 큽니다;
이 참조자가 코드 예제를 라이프타임에 대한 관점에서 흥미롭게 만들어주는 것입니다.
우리 파서의 로직이 첫번째 바이트 이후의 입력은 유효하지 않다고 판단했다고
가정해봅시다. 첫번째 바이트가 유효한 문자 범위 상에 있지 않으면 이 코드는
패닉을 일으킬 수도 있음을 주의하세요; 다시 한번, 우리는 수반되는 라이프타임에
집중하도록 예제를 단순화하는 중입니다.

이 코드를 컴파일하기 위해서는 `Context` 내의 스트링 슬라이스와 `Parser`
내의 `Conext`를 가리키는 참조자에 대한 라이프타임 파라미터를 채워줄
필요가 있습니다. 이를 위한 가장 직관적인 방법은 Listing 19-13에서
보시는 것과 같이 모든 곳에 동일한 라이프타임 이름을 사용하는 것입니다. 10장의
“구조체 정의 상에서의 라이프타임 명시”절에서 본 것처럼 각각의
`struct Context<'a>`, `struct Parser<'a>`와 `impl<'a>`는 새로운
라이프타임 파라미터를 선언중이라는 점을 상기하세요. 그 이름들이 모두 동일하게 등장한
반면, 예제에서 선언된 이 3개의 라이프타임 파라미터는 모두 연관되어 있지 않습니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

<span class="caption">Listing 19-13: `Context`와 `Parser`의 모든 참조자에 라이프타임
파라미터 명시하기</span>

이 코드는 잘 컴파일됩니다. 이 코드는 러스트에게 `Parser`가 라이프타임이 `'a`인
`Context`를 가리키는 참조자를 가지고 있고, `Context`는 `Parser` 내의 `Context`
참조자만큼 오래 사는 스트링 슬라이스를 가지고 있다고 말해줍니다. 러스트의 컴파일러
에러 메세지는 이 참조자들에게 라이프타임 파라미터가 필요하다고 기술했었고, 우리가
방금 그 라이프타임 파라미터를 추가했습니다.

다음으로, Listing 19-14에서 우리는 `Context`의 인스턴스를 받아서,
이 콘텍스트를 파싱하기 위해 `Parser`를 사용하고, `parse`가 반환하는
것을 반환하는 함수를 추가할 것입니다. 아래 코드는 잘 동작하지 않습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

<span class="caption">Listing 19-14: `Context`를 받아서 `Parser`를 사용하는
`parser_context` 함수 추가 시도</span>

`parse_context` 함수를 추가하고 컴파일 시도를 하면 두 개의 장황한
에러를 얻게 됩니다:

```text
error[E0597]: borrowed value does not live long enough
  --> src/lib.rs:14:5
   |
14 |     Parser { context: &context }.parse()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ does not live long enough
15 | }
   | - temporary value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 13:1...
  --> src/lib.rs:13:1
   |
13 | / fn parse_context(context: Context) -> Result<(), &str> {
14 | |     Parser { context: &context }.parse()
15 | | }
   | |_^

error[E0597]: `context` does not live long enough
  --> src/lib.rs:14:24
   |
14 |     Parser { context: &context }.parse()
   |                        ^^^^^^^ does not live long enough
15 | }
   | - borrowed value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 13:1...
  --> src/lib.rs:13:1
   |
13 | / fn parse_context(context: Context) -> Result<(), &str> {
14 | |     Parser { context: &context }.parse()
15 | | }
   | |_^
```

이 에러들은 만들어진 `Parser` 인스턴스와 `context` 파라미터가 
`parse_context` 함수의 끝까지만 산다고 기술하고 있습니다.
그러나 이 둘 모두 함수의 전체 라이프타임보다 더 살아야 할 필요가 있습니다.

바꿔 말하면, `Parse`와 `context`는 전체 함수보다 오래 살아야 할 필요가
있고 이 코드의 모든 참조자들이 항상 유효하기 위해서 함수가 끝날 때는 물론
함수가 시작될 때도 유효해야 할 필요가 있습니다. 우리가 만든 `Parser`와
`context` 파라미터는 함수 끝에서 스코프 밖으로 벗어나는데, 그 이유는
`parse_context`가 `context`의 소유권을 갖기 때문입니다.

이 에러가 왜 발생하는지 알아내기 위해, Listing 19-13에 있는 정의 부분 중
특히 `parse` 메소드의 시그니처에 있는 참조자들을 다시 살펴봅시다:

```rust,ignore
    fn parse(&self) -> Result<(), &str> {
```

생략 규칙 기억하시죠? 만일 참조자의 라이프타임을 생략하지 않고 명시했다면,
시그니처는 다음과 같을 것입니다:

```rust,ignore
    fn parse<'a>(&'a self) -> Result<(), &'a str> {
```

즉, `parse`의 반환값의 에러 부분은 `Parser` 인스턴스의 라이프타임에
묶여 있는 라이프타임을 갖고 있다는 것입니다 (`parse` 메소드 시그니처 내의
`&self`의 것이지요). 이는 타당합니다: 반환되는 스트링 슬라이스는
`Parser`가 가지고 있는 `Context` 인스턴스의 스트링 슬라이스를 참조하고,
`Parser` 구조체의 정의는 `Context`의 참조자의 라이프타임과 `Context`가
가지고 있는 스트링 슬라이스의 라이프타임이 동일해야 함을 기술하고
있습니다.

문제는 `parse_context` 함수가 `parse`로부터 값을 반환하고 있으므로,
`parse_context`의 반환값의 라이프타임 또한 `Parser`의 라이프타임과
묶여 있다는 것입니다. 그러나 `parse_context` 함수 내에서 만들어진 `Parser`
인스턴스는 함수 끝을 벗어나 살 수 없을 것이고 (일시적인 객체입니다), `context`는
함수의 끝에서 스코프 밖으로 벗어날 것입니다 (`parse_context`가 이것의
소유권을 가지고 있습니다).

러스트는 우리가 함수의 끝에서 스코프 밖으로 벗어나는 값의 참조자를 반환 시도를
하는 중이라고 생각하는데, 이는 우리가 모든 라이프타임을 동일한 라이프타임
파라미터로 명시했기 떄문입니다. 이 어노테이션은 러스트에게 `Context`가
가지고 있는 스트링 슬라이스의 라이프타임은 `Parser`가 들고 있는 `Context`를
가리키는 참조자의 라이프타임의 것과 동일하다고 말하고 있습니다.

`parse_context` 함수는 `parse` 함수의 내부에서 반환되는 스트링 슬라이스가
`Context`와 `Parser`보다 오래살 것이라는 것, 그리고 `parse_context`가
반환하는 참조자가 `Context` 혹은 `Parser`가 아닌 스트링 슬라이스를 참조하고
있다는 것을 알 수 없습니다.

`parse`의 구현체가 무엇을 하는지 아는 것으로써, 우리는 `parse`의 반환값이
`Parser`에 묶여있는 유일한 이유가 이것이 스트링 슬라이스를 참조하고 있는
`Parser`의 `Context`를 참조하고 있기 때문이라는 것을 알게 되었습니다.
따라서, `parse_context`가 다루고자 하는 것은 실은 스트링 슬라이스의
라이프타임인 것입니다. 우리는 `Context` 내의 스트링 슬라이스와
`Parser` 내의 `Context`를 가리키는 참조자가 다른 라이프타임을 가지고 있고
`parse_context`의 반환값은 `Context`의 스트링 슬라이스의 라이프타임에
묶여있음을 알려줄 방법이 필요합니다.

먼저 Listing 19-15에서 보시는 것처럼 `Parser`와 `Context`에게 서로 다른
라이프타임 파라미터를 주는 시도를 하겠습니다. 우리는 `'s`와 `'c`라는 라이프타임 파라미터
이름을 사용하여 어떤 라이프타임이 `Context` 내의 스트링 슬라이스에 포함되고 어떤 라이프타임이
`Parser` 내의 `Context`를 가리키는 참조자에 초함되는지 명확히 할 것입니다. 이 해결책이
문제를 완전히 해결하지는 않겠지만, 이것이 시작점이라는 점을 주목하세요. 이 소스 코드
수정이 왜 컴파일 시도에 충분치 않은지 살펴보겠습니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
struct Context<'s>(&'s str);

struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

<span class="caption">Listing 19-15: 스트링 슬라이스에 대한 참조자와 `Context`에
대한 참조자에 대해 서로 다른 라이프타임 파라미터 지정하기</span>

우리가 Listing 19-13에서 명시했던 것과 모두 동일한 위치에 있는 참조자의
라이프타임을 명시했습니다. 하지만 이번에는 참조자가 스트링 슬라이스에 포함되는지
혹은 `Context`에 포함되는지 여부에 따라 다른 파라미터를 사용했습니다. 우리는
또한 `parse`의 반환값의 스트링 슬라이스 부분에도 이것이 `Context` 내의
스트링 슬라이스의 라이프타임에 포함된다는 것을 나타내기 위해서 어노테이션을
추가했습니다.

이제 컴파일 시도를 하면, 다음과 같은 에러를 얻습니다:

```text
error[E0491]: in type `&'c Context<'s>`, reference has a longer lifetime than the data it references
 --> src/lib.rs:4:5
  |
4 |     context: &'c Context<'s>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime 'c as defined on the struct at 3:1
 --> src/lib.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
note: but the referenced data is only valid for the lifetime 's as defined on the struct at 3:1
 --> src/lib.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
```

러스트는 `'c`와 `'s` 사이에 어떠한 관계도 알지 못합니다. 이를 유효화하기 위해,
`'s` 라이프타임을 가진 `Context` 내의 참조자 데이터는 `'c` 라이프타임을 가진
참조자보다 더 오래 산다는 것을 보장하기 위해 제한될 필요가 있습니다. 만일 `'s`가
`'c`보다 오래 살지 못한다면, `Context`의 참조자가 유효하지 않을 수도 있습니다.

이제 우리는 이 절의 요점을 얻었습니다: 러스트의 기능인 *라이프타임 서브타이핑*은
하나의 라이프타임 파라미터가 최소한 다른 것만큼 오래 산다는 것을 명시힙니다.
우리가 라이프타임 파라미터를 선언하는 꺽쇠 괄호 내에서, 우리는 라이프타임 `'a`을
평소처럼 선언하고, 문법 `'b: 'a`를 사용하여 `'b`를 선언함으로써 라이프타임 `'b`가
최소 `'a` 만큼 오래 산다고 선언할 수 있습니다.

우리의 `Parser` 정의부에서, `'s` (스트링 슬라이스의 라이프타임) 가 최소한 `'c`
(`Context`를 가리키는 참조자의 라이프타임) 만큼 오래 사는 것이 보장됨을 말하기
위해서, 아래와 같이 라이프타임 선언을 변경합니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Context<'a>(&'a str);
#
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}
```

이제 `Parser` 내에 있는 `Context`에 대한 참조자와 `Context` 내의 스트링 슬라이스를
가리키는 참조자는 다른 라이프타임을 갖습니다; 우리는 스트링 슬라이스의 라이프타임이
`Context`를 가리키는 참조자보다 더 오래 살 것이란 보장을 했습니다.

참 길고 지루한 예제였습니다만, 이 장의 첫 부분에서도 언급했듯,
러스트의 고급 기능들은 매우 구체적입니다. 우리가 이 예제에서 묘사한
문법이 자주 필요치는 않겠지만, 특정한 상황에서 여러분이 참조해야 하는
무언가를 참조하는 방법을 알아둬야 할 것입니다.

### 제네릭 타입에 대한 참조자 상의 라이프타임 바운드

10장의 “트레잇 바운드”절에서, 우리는 제네릭 타입 상의 트레잇 바운드를 사용하는 것에 대해
논했습니다. 우리는 또한 제네릭 타입의 제약사항으로서 라이프타임 파라미터를 추가할 수 있습니다;
이를 *라이프타임 바운드 (lifetime bound)* 라고 부릅니다. 라이프타임 바운드는 제네릭 타입
내의 참조자들이 참조하고 있는 데이터보다 오래 살지 못하도록 러스트가 확인하는 것을 돕습니다.

한 가지 예로, 참조자에 대한 래퍼 (wrapper) 인 타입을 고려해보세요. 15장의
“`RefCell<T>`와 내부 가변성 패턴”절에서 나온 `RefCell<T>` 타입을
상기해보세요: 이것의 `borrow` 및 `borrow_mut` 메소드는 각각 `Ref` 및
`RefMut` 타입을 반환합니다. 이 타입들은 런타임에 빌림 규칙을 계속 따르게
하는 참조자들의 레퍼입니다. `Ref` 구조체의 정의는 Listing 19-16과 같은데,
지금은 라이프타임 바운드 없이 쓰였습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
struct Ref<'a, T>(&'a T);
```

<span class="caption">Listing 19-16: 시작을 위해 라이프타임 바운드 없이 쓰는
제네릭 타입에 대한 참조자를 감싼 구조체 정의하기</span>

제네릭 타입 `T`과의 관계에 대한 라이프타임 `'a`의 명시적 제약이 없으면,
러스트는 에러를 내게 되는데 그 이유는 제네릭 타입 `T`가 얼마나 오래 살
것인지를 모르기 때문입니다:

```text
error[E0309]: the parameter type `T` may not live long enough
 --> src/lib.rs:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
  |
  = help: consider adding an explicit lifetime bound `T: 'a`...
note: ...so that the reference type `&'a T` does not outlive the data it points at
 --> src/lib.rs:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
```

`T`가 어떠한 타입도 될 수 있으므로, `T`는 참조자 혹은 하나 이상의 참조자를 가지고 있는
타입이 될 수 있는데, 각각은 자신의 라이프타임을 가질 수 있습니다. 러스트는 `T`가
`'a`만큼 오래 살 수 있는지 확신할 수 없습니다.

다행히도, 위의 경우 에러가 라이프타임 바운드를 어떻게 명시하는지에 대한 도움되는
조언을 제공합니다:

```text
consider adding an explicit lifetime bound `T: 'a` so that the reference type
`&'a T` does not outlive the data it points at
```

Listing 19-17은 우리가 제네릭 타입 `T`를 선언할 때 라이프타임 바운드를 명시함으로서
이 조언을 어떻게 적용하는지를 보여줍니다:

```rust
struct Ref<'a, T: 'a>(&'a T);
```

<span class="caption">Listing 19-17: `T` 상의 라이프타임 바운드를 추가하여
`T` 내의 어떠한 참조자들도 최소한 `'a`만큼 오래 살 것임을 명시하기</span>

이 코드는 이제 컴파일되는데, `T: 'a` 문법을 사용하면 `T`가 어떤 타입이든 될 수 있지만,
만일 어떠한 참조자라도 포함하고 있다면, 그 참조자들은 최소한 `'a`만큼은 오래 살아야 함을
명시하고 있기 때문입니다.

Listing 19-18의 `StaticRef` 구조체 정의 부분에서 `T`에 `'static`
라이프타임 바운드를 추가한 것처럼, 우리는 이 문제를 다른 방식으로 해결할 수도
있습니다. 이는 만일 `T`가 어떠한 참조자를 가지고 있다면, 이들은 반드시 `'static`
라이프타임을 가져야 함을 의미합니다.

```rust
struct StaticRef<T: 'static>(&'static T);
```

<span class="caption">Listing 19-18: `'static` 라이프타임 바운드를
`T`에 추가하여 `T`가 오직 `'static` 참조자만을 갖거나 아무런 참조자도 없도록
제한하기</span>

`'static`이 전체 프로그램만큼 오래 살아야 함을 뜻하기 때문에, 아무런
참조자도 없는 타입도 모든 참조자들이 전체 프로그램 만큼 오래 사는 규정을
만족합니다 (왜냐면 아무런 참조자도 없으니까요). 참조자가 충분히 오래 사는지에
대해 염려하는 빌림 검사기를 위하여, 아무런 참조자도 없는 타입과 영원이
사는 참조자들을 가진 타입 간의 실질적 구분은 없습니다: 둘다 그것이 참조하고
있는 것보다 더 짧은 라이프타임을 가진 참조자인지 아닌지를 결정하는 관점에서는
같습니다.

### 트레잇 객체 라이프타임의 추론

17장의 “서로 다른 타입의 값을 허용하는 트레잇 객체를 사용하기”절에서,
우리는 동적 디스패치를 이용할 수 있게 해주는 참조자 뒤의 트레잇으로 구성된
트레잇 객체를 논했습니다. 우리는 아직 트레잇 객체 내의 트레잇을 구현한
타입이 자신만의 라이프타임을 가지면 어떤일이 벌어지는지 논하지는 않았습니다.
트레잇 `Red`와 구조체 `Ball`를 가지고 있는 Listing 19-19을 고려해보세요.
`Ball` 구조체는 참조자를 가지고 있고 (따라서 라이프타임 파라미터를 가지고 있죠)
또한 트레잇 `Red`를 구현합니다. 우리는 `Ball`의 인스턴스를 트레잇 객체
`Box<Red>`로서 사용하기를 원합니다:

<span class="filename">Filename: src/main.rs</span>

```rust
trait Red { }

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

fn main() {
    let num = 5;

    let obj = Box::new(Ball { diameter: &num }) as Box<Red>;
}
```

<span class="caption">Listing 19-19: 트레잇 객체와 함께 라이프타임 파라미터를
갖는 타입 사용하기</span>

비록 우리가 아직 `obj`과 관련된 라이프타임을 명시적으로 적지 않았으나,
이 코드는 에러 없이 컴파일됩니다. 이 코드는 동작하는데 그 이유는
라이프타임과 트레잇 객체가 함께 동작하는 규칙이 있기 때문입니다:

* 트레잇 객체의 기본 라이프타임은 `'static` 입니다.
* `&'a Trait` 혹은 `&'a mut Trait`을 쓴 경우, 트레잇 객체의 기본
  라이프타임은 `'a` 입니다.
* 단일 `T: 'a` 구절을 쓴 경우, 트레잇 객체의 기본 라이프타임은 `'a`
  입니다.
* 여러 개의 `T: 'a` 같은 구절들을 쓴 경우, 기본 라이프타임는 없습니다;
  우리가 명시적으로 써야합니다.

우리가 명시적으로 써야 할 때, `Box<Red>` 같은 트레잇 객체에 대해 `Box<Red + 'static>`
혹은 `Box<Red + 'a>` 같은 문법을 써서 라이프타임 바운드를 추가할 수 있는데,
이는 참조자가 전체 프로그램 동안 사는지 혹은 그렇지 않은지에 따라 달려 있습니다.
다른 바운드를 사용할 때처럼, 라이프타임 바운드를 추가하는 문법은 타입 내에 참조자를
가진 어떠한 `Red` 트레잇의 구현체라도 그 타입의 참조자처럼 트레잇 객체 내에
명시된 동일한 라이프타임을 가져야 한다는 뜻입니다.

다음으로, 트레잇을 관리하는 다른 고급 기능을 살펴봅시다.
