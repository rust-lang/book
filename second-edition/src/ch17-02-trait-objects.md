## 트레잇 객체를 사용하여 다른 타입 간의 값 허용하기

8장에서는 벡터가 한 번에 하나의 타입만 보관할 수 있다는 제약사향이 있다고
언급했습니다. 우리가 만들었던 Listing 8-10의 작업내역에서는 정수, 부동소수점,
그리고 문자를 보관하기 위한 variant들을 가지고 있는 `SpreadsheetCell`
열거형을 정의했습니다. 이것은 우리가 각 칸마다 다른 타입의 데이터를 저장할 수
있으면서도 여전히 그 칸들의 한 묶음을 대표하는 벡터를 가질 수 있다는 것을
의미했습니다. 이는 우리의 교환가능한 아이템들이 코드를 컴파일할 때 알 수
있는 정해진 몇 개의 타입인 경우 완벽한 해결책입니다.

하지만, 가끔 우리는 우리의 라이브러리 사용자가 특정 상황에서 유효한 타입
묶음을 확장할 수 있도록 하길 원합니다. 우리가 원하는 바를 이룰 수 있는지를
보이기 위해, 우리는 아이템들의 리스트에 걸쳐 각각에 대해 `draw` 메소드를
호출하여 이를 화면에 그리는 그래픽 유저 인터페이스(GUI) 도구는 만들 것입니다 -
GUI 도구들에게 있어서는 흔한 방식이죠. 우리가 만들 라이브러리 크레이트는
`gui`라고 호명되고 GUI 라이브러리 구조를 포괄합니다. 이 크레이트는 사용자들이
사용할 수 있는 몇 가지 타입들, `Button`이나 `TextField` 들을 포함하게
될 것이구요. 추가로, `gui` 사용자들은 그들 고유의 타입을 만들어 그리고자
할 것입니다: 일례로, 어떤 프로그래머는 `Image`를 추가할지도 모르고 또다른
누군가는 `SelectBox`를 추가할지도 모르겠습니다.

우리는 이번 예제에서 총체적인 GUI 라이브러리를 구현하지 않겠지만 어떻게 이 조각들이
맞물려 함께 동작할 수 있는지 보여주고자 합니다. 라이브러리를 작성하는 시점에서는
다른 프로그래머들이 만들고자 하는 모든 타입들을 알 수 없죠. 하지만 우리가 알 수 있는
것은 `gui`가 다른 타입들의 다양한 값에 대해 계속해서 추적해야 하고, `draw` 메소드가
이 다양한 값들 각각에 호출되어야 한다는 겁니다. 우리가 `draw` 메소드를 호출했을 때
벌어지는 일에 대해서 정확히 알 필요는 없고, 그저 우리가 호출할 수 있는 해당 메소드를
그 값이 가지고 있음을 알면 됩니다.

상속이 있는 언어를 가지고 이 작업을 하기 위해서는 `draw` 라는 이름의 메소드를
갖고 있는 `Component` 라는 클래스를 정의할 수도 있습니다. 다른 클래스들, 이를테면
`Button`, `Image`, 그리고 `SelectBox` 같은 것들은 `Component`를 상속받고
따라서 `draw` 메소드를 물려받게 됩니다. 이들은 각각 `draw` 메소드를 오버라이딩하여
그들의 고유 동작을 정의할 수 있으나, 프레임워크는 모든 유형을 마치 `Component`인
것처럼 다룰 수 있고 `draw`를 호출할 수 있습니다. 하지만 러스트가 상속이 없는 관계로,
`gui` 라이브러리를 구축하는 다른 방법을 찾아 사용자들이 새로운 타입을 정의하고 확장할
수 있도록 할 필요가 있습니다.

### 공통된 동작을 위한 트레잇 정의하기

`gui`가 갖길 원하는 동작을 구현하기 위해, 우리는 `draw`라는 이름의 메소드
하나를 갖는 `Draw`라는 이름의 트레잇을 정의할 것입니다. 그러면 *트레잇 객체
(trait object)* 를 취하는 벡터를 정의할 수 있습니다. 트레잇 객체는 특정
트레잇을 구현한 타입의 인스턴스를 가리킵니다. 우리는 `&` 참조자나 `Box<T>`
스마트 포인터 같은 포인터 종류로 명시함으로서 트레잇 객체를 만들고, 그런 다음
관련된 트레잇을 특정합니다. (우리가 트레잇 객체에 포인터를 사용해야 하는 이유는
19장의 “동적인 크기의 타입과 Sized” 절에서 다룰 겁니다.) 우리는 제네릭
타입이나 구체 타입 대신 트레잇 객체를 사용할 수 있습니다. 트레잇 객체를
사용하는 곳이 어디든, 러스트의 타입 시스템은 컴파일 타임에 해당 문맥 안에
사용된 값이 트레잇 객체의 트레잇을 구현할 것을 보장합니다. 결론적으로, 우리는
컴파일 타임에 모든 가능한 타입을 알 필요가 없습니다.

러스트에서는 구조체와 열거형을 다른 언어의 객체와 구분하기 위해
“객체”라고 부르는 것을 자제한다고 언급했었습니다. 구조체 또는
열거형에서는 구조체 필드의 데이터와 `impl` 블록의 동작이
분리되는 반면, 다른 언어에서는 데이터와 동작이 결합되어 객체로
명명됩니다. 그러나 트레잇 객체들은 데이터와 동작을 결합한다는
의미에서 다른 언어의 객체와 *비슷합니다*. 하지만 트레잇 객체는
트레잇 객체에 데이터를 추가 할 수 없다는 점에서 전통적인
객체들과 다릅니다. 트레잇 객체는 다른 언어들의 객체만큼 범용적으로
유용하지는 않습니다: 그들의 명확한 목적은 공통된 동작들에 걸친
추상화를 가능하도록 하는 것이죠.

Listing 17-3은 `draw`라는 이름의 메소드를 갖는 `Draw `라는 트레잇을 정의하는
방법을 보여줍니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait Draw {
    fn draw(&self);
}
```

<span class="caption">Listing 17-3: `Draw` 트레잇의 정의</span>

이 문법은 10장에 있는 트레잇을 정의하는 방법에서 다뤘으니 익숙하실 겁니다.
다음에 새로운 문법이 등장합니다: Listing 17-4는 `components` 라는
벡터를 보유하고 있는 `Screen`이라는 구조체를 정의합니다. `Box<Draw>`
타입의 벡터인데, 이것이 트레잇 객체입니다; 이것은 `Draw` 트레잇을 구현한
`Box`에 담긴 임의의 타입에 대한 대역입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
pub struct Screen {
    pub components: Vec<Box<Draw>>,
}
```

<span class="caption">Listing 17-4: `Draw` 트레잇을
구현하는 트레잇 객체들의 벡터 항목 `components`를 소유한 구조체
`Screen`</span>

`Screen` 구조체에서는 Listing 17-5와 같이 각 `components` 마다
`draw`메소드를 호출하는 `run` 메소드를 정의합니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
# pub struct Screen {
#     pub components: Vec<Box<Draw>>,
# }
#
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

<span class="caption">Listing 17-5: 각 컴포넌트에 대해 `draw` 메소드를
호출하는 `Screen`의 `run` 메소드</span>

이것은 트레잇 바운드와 함께 제네릭 타입 파라미터를 사용하는 구조체를
정의하는 것과는 다르게 작동합니다. 제네릭 타입 파라미터는 한 번에
하나의 구체 타입으로만 대입될 수 있는 반면, 트레잇 객체를 사용하면
런타임에 여러 구체 타입을 트레잇 객체에 대해 채워넣을 수 있습니다.
예를 들면, Listing 17-6처럼 제네릭 타입과 트레잇 바운드를 사용하여
`Screen` 구조체를 정의할 수도 있을 겁니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

<span class="caption">Listing 17-6: 제네릭과 트레잇 바운드를 사용한
`Screen` 구조체와 `run` 메소드의 대체 구현</span>

이렇게하면 전부 `Button` 타입 혹은 전부 `TextField` 타입인 컴포넌트
리스트를 가지는 `Screen` 인스턴스로 제한됩니다. 동일 유형의 콜렉션만 사용한다면
제네릭과 특성 범위를 사용하는 것이 바람직한데, 왜냐하면 그 정의들은 구체 타입을
사용하기 위해 컴파일 타임에 단형성화 (monomorphize) 되기 때문입니다.

반면에 트레잇 객체를 사용하는 메소드를 이용할때는 하나의 `Screen` 인스턴스가
`Box<Button>` 혹은  `Box<TextField>`도 담을 수 있는 `Vec<T>`를 보유할
수 있습니다. 이것이 어떻게 작동하는지 살펴보고 런타임 성능에 미치는 영향에 대해
설명하겠습니다.

### 트레잇 구현하기

이제 우리는 `Draw` 트레잇을 구현하는 몇가지 타입을 추가하려고 합니다. 우리는
`Button` 타입을 제공할 것입니다. 다시금 말하지만, 실제 GUI 라이브러리를
구현하는 것은 이 책의 범위를 벗어나므로, 우리는 `draw`에는 별다른 구현을 하지
않을 겁니다. 구현하려는 것을 상상해보자면, `Button` 구조체는 Listing 17-7에서
보시는 바와 같이 `width`, `height` 그리고 `label` 항목들을 가지게 될 것입니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

<span class="caption">Listing 17-7: `Draw` 트레잇을 구현하는
`Button` 구조체</span>

`Button`의 `width`, `height` 및 `label` 필드는 다른 컴포넌트와는
차이가 있는데, `TextField` 타입을 예로 들면, 이 필드들에 추가로
`placeholder` 필드를 소유할 겁니다. 우리가 화면에 그리고자 하는 각각의
타입은 `Draw` 트레잇을 구현할테지만 해당 타입을 그리는 방법을 정의하기
위하여 `draw` 메소드 내에 서로 다른 코드를 사용하게 될 것이며, `Button`이
그러한 경우죠 (이 챕터의 범주를 벗어나기 때문에 실질적인 GUI 코드느 없지만요).
예를 들어, `Button` 타입은 추가적인 `impl` 블록에 사용자가 버튼을 클릭했을
때 어떤 일이 벌어질지와 관련된 메소드들을 포함할 수 있습니다. 이런 종류의
메소드는 `TextField`와 같은 타입에는 적용할 수 없죠.

우리의 라이브러리를 사용하는 누군가가 `width`, `height` 및 `options`
필드가 있는 `SelectBox` 구조체를 구현하기로 했다면, Listing 17-8과 같이
`SelectBox` 타입에도 `Draw` 트레잇을 구현합니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate gui;
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
```

<span class="caption">Listing 17-8: `gui`를 사용하고 `Draw` 트레잇을
`SelectBox` 구조체에 구현한 또 다른 크레이트</span>

우리 라이브러리의 사용자는 이제 `Screen` 인스턴스를 만들기 위해 `main` 함수를
구현할 수 있습니다. `Screen` 인스턴스에는 `SelectBox`와 `Button`가
트레잇 객체가 되도록 하기 위해 `Box<T>` 안에 넣음으로서 이들을 추가할 수 있습니다.
그러면 `Screen` 인스턴스 상의 `run` 메소드를 호출할 수 있는데, 이는 각 컴포넌트들에
대해 `draw`를 호출할 것입니다. Listing 17-9는 이러한 구현을 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use gui::{Screen, Button};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

<span class="caption">Listing 17-9: 트레잇 객체를 사용하여 동일한 트레잇을
구현하는 서로 다른 타입들의 값 저장하기</span>

우리가 라이브러리를 작성할 때는, 누군가 `SelectBox` 타입을 추가할 수도
있다는 것을 알 수 없었지만, 우리의 `Screen` 구현체는 새로운 타입에 대해서도
동작하고 이를 그려낼수 있는데, 그 이유는 `SelectBox`가 `Draw` 타입을 구현했기
때문이고, 이는 `draw` 메소드가 구현되어 있음을 의미합니다.

이러한 개념 —값의 구체적인 타입이 아닌 값이 응답하는 메시지 만을 고려하는 개념—
은 동적 타입 언어들의 *오리 타이핑 (duck typing)* 이란 개념과 유사합니다:
만약 오리처럼 뒤뚱거리고 오리처럼 꽥꽥거리면, 그것은 오리임에 틀림없습니다!
Listing 17-5에 나오는 `Screen`에 구현된 `run`을 보면, `run`은 각
컴포넌트가 어떤 구체적 타입인지 알 필요가 없습니다. 이 함수는 컴포넌트가
`Button`의 인스턴스인지 혹은 `SelectBox`의 인스턴스인지 검사하지 않고
그저 각 컴포넌트의 `draw` 메소드를 호출할 뿐입니다. `components` 벡터에
담기는 값의 타입을 `Box<Draw>`로 특정함으로서 우리는 `draw` 메소드를
호출할 수 있는 값을 요구하는 `Screen`을 정의했습니다.

오리 타이핑을 사용하는 코드와 유사한 코드를 작성하기 위해서 트레잇 객체와
러스트의 타입 시스템을 사용하는 것의 장점은 어떤 값이 특정한 메소드를 구현했는지를
검사해야 하거나 혹은 값이 메소드를 구현하지 않았는데 우리가 그걸 어쨌든 호출한다면
생길 수 있는 에러에 대한 걱정을 전혀 할 필요가 없다는 겁니다. 러스트는 트레잇
객체가 요구하는 트레잇을 해당 값이 구현하지 않았다면 컴파일하지 않을 겁니다.

예를 들어, Listing 17-10은 `String`을 컴포넌트로 사용하여 `Screen`을
생성하는 시도를 하면 어떤 일이 벌어지는지 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate gui;
use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(String::from("Hi")),
        ],
    };

    screen.run();
}
```

<span class="caption">Listing 17-10: 트레잇 객체의 트레잇을 구현하지 않은
타입의 사용 시도하기</span>

우리는 아래와 같은 에러를 보게 될 것이며 이유는 `String` 이 `Draw` 트레잇을 구현하지 않기 때문입니다:

```text
error[E0277]: the trait bound `std::string::String: gui::Draw` is not satisfied
  --> src/main.rs:7:13
   |
 7 |             Box::new(String::from("Hi")),
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait gui::Draw is not
   implemented for `std::string::String`
   |
   = note: required for the cast to the object type `gui::Draw`
```

이 에러는 우리가 넘길 뜻이 없었던 무언가를 `Screen`에게 넘기는 중이고 이를
다른 타입으로 교체해야 하거나, 혹은 우리가 `String`에 대해 `Draw`를 구현하여
`Screen`이 이것에 대해 `draw`를 호출할 수 있도록 해야한다는 것을 알려줍니다.

### 트레잇 객체는 동적 디스패치를 수행합니다

10장의 “제네릭을 사용한 코드의 성능“ 절에서 우리가 제네릭에 트레잇
바운드를 사용했을 때 컴파일러에 의해 이뤄지는 단형성화 프로세스의 실행에
대한 논의를 상기해보세요: 컴파일러는 우리가 제네릭 타입 파라미터를 사용한
각각의 구체 타입을 위한 함수와 메소드의 제네릭 없는 구현체를 생성합니다.
단형성화로부터 야기된 코드는 *정적 디스패치 (static dispatch)* 를
수행하는데, 이는 여러분이 호출하고자 하는 메소드가 어떤 것인지 컴파일러가
컴파일 시점에 알고 있는 것입니다. 이는 *동적 디스패치 (dynamic
dispatch)* 와 반대되는 개념으로, 동적 디스패치는 컴파일러가 여러분이
호출하는 메소드를 컴파일 시에 알 수 없을 경우 수행됩니다. 동적 디스패치의
경우, 컴파일러는 런타임에 어떤 메소드가 호출되는지 알아내는 코드를 생성합니다.

우리가 트레잇 객체를 사용할 때, 러스트는 동적 디스패치를 이용해야 합니다. 컴파일러는
트레잇 객체를 사용중인 코드와 함께 사용될 수도 있는 모든 타입을 알지 못하기 때문에,
어떤 타입에 구현된 어떤 메소드를 호출할지 알지 못합니다. 대신 런타임에서, 러스트는
트레잇 객체 내에 존재하는 포인터를 사용하여 어떤 메소드가 호출될지 알아냅니다.
정적 디스패치 시에는 일어나지 않는 이러한 탐색이 발생할 때 런타임 비용이 있습니다.
동적 디스패치는 또한 컴파일러가 메소드의 코드를 인라인 (inline) 화하는 선택을
막아버리는데, 이것이 결과적으로 몇가지 최적화를 수행하지 못하게 합니다. 하지만,
우리는 추가적인 유연성을 얻어 Listing 17-5와 같은 코드를 작성할 수 있었고,
Listing 17-9과 같은 지원이 가능해졌으니, 여기에는 고려할 기회비용이 있다고
하겠습니다.

### 트레잇 객체에 대하여 객체 안전성이 요구됩니다

여러분은 *객체-안전 (object-safe)* 한 트레잇만 트레잇 객체로 만들 수 있습니다.
트레잇 객체를 안전하게 만드는 모든 속성들을 관장하는 몇가지 복잡한 규칙이 있지만,
실전에서는 두 가지 규칙만 관련되어 있습니다. 어떤 트레잇 내의 모든 메소드들이 다음과
같은 속성들을 가지고 있다면 해당 트레잇은 객체 안전합니다:

* 반환값의 타입이 `Self`가 아닙니다.
* 제네릭 타입 매개변수가 없습니다.

`Self` 키워드는 우리가 트레잇 혹은 메소드를 구현하고 있는 타입의
별칭입니다. 트레잇 객체가 반드시 객체 안전해야 하는 이유는 일단
여러분이 트레잇 객체를 사용하면, 러스트가 트레잇에 구현된 구체(concrete)
타입을 알 수 없기 때문입니다. 만약 트레잇 메소드가 고정된 `Self`
타입을 반환하는데 트레잇 객체는 `Self`의 정확한 타입을 잊었다면,
메소드가 원래 구체 타입을 사용할 수 있는 방법이 없습니다. 트레잇을
사용할 때 구체 타입 파라미터로 채워지는 제네릭 타입 파라미터도
마찬가지입니다: 그 구체 타입들은 해당 트레잇을 구현하는 타입의 일부가
됩니다. 트레잇 객체를 사용을 통해 해당 타입을 잊게되면, 제네릭 타입
파라미터를 채울 타입을 알 수 없습니다. 

메소드가 객채 안전하지 않은 트레잇의 예는 표준 라이브러리의 `Clone`
트레잇입니다. `Clone` 트레잇의 `clone` 메소드에 대한 시그니처는
다음과 같습니다:

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

`String` 타입은 `Clone` 트레잇을 구현하고, `String` 인스턴스에 대하여
`clone` 메소드를 호출하면 우리는 `String`의 인스턴스를 반환받을 수 있습니다.
비슷하게, 우리가 `Vec<T>`의 인스턴스 상의 `clone`을 호출하면, 우리는 `Vec<T>`
인스턴스를 얻을 수 있습니다. `clone` 선언은 `Self`에 어떤 타입이 사용되는지 알
필요가 있는데, 왜냐면 그게 반환 타입이기 때문이죠. 

컴파일러는 여러분이 트레잇 객체와 관련하여 객체 안전성 규칙을 위반하는
무언가를 시도하려고 하면 알려줍니다. 예를 들어, Listing 17-4에서
`Screen` 구조체가 `Draw` 트레잇 대신 `Clone` 트레잇을 구현하는
타입을 보관하도록 아래처럼 구현 시도를 해봅시다:

```rust,ignore
pub struct Screen {
    pub components: Vec<Box<Clone>>,
}
```

우리는 이런 에러를 얻게 될 겁니다:

```text
error[E0038]: the trait `std::clone::Clone` cannot be made into an object
 --> src/lib.rs:2:5
  |
2 |     pub components: Vec<Box<Clone>>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` cannot be
made into an object
  |
  = note: the trait cannot require that `Self : Sized`
```

이 에러가 의미하는 바는 이러한 방식으로 이 트레잇을 트레잇 객체로사 사용할 수 없다는 겁니다.
혹시 객체 안전에 대해 보다 자세하게 알고 싶으시면 [Rust RFC 255]를 참고하세요.

[Rust RFC 255]: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
