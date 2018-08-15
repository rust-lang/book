## 특성 객체를 사용하여 다른 타입 간의 값 허용하기.  

8장에서, 우리가 언급했던 벡터의 한 가지 제약사항은 그들이 한 번에 하나의 타입만 보관할 수 있다는 것입니다. 우리가 만들었던 항목 8-10의 작업내역에서 우리는 `SpreadsheetCell` enum을 정의했고 정수, 부동소수점, 그리고 문자를 보관하게 하고자 했습니다. 이것은 우리가 각 셀에 다른 타입의 데이터를 저장해야 한다는 것을 의미하죠. 완벽하게 훌륭한 솔루션은 우리의 교환가능한 아이템들을 우리가 코드가 컴파일 될 때 우리가 알 수 있는 타입 유형으로 고정하는 것 입니다. 

하지만, 종종 우리가 원하는 것은 사용자가 우리의 라이브러리를 다양한 타입으로 확장할 수 있게 하여 어떤 상황에서도 유효하게 하는 것이죠. 우리가 원하는 바를 이룰 수 있는지를 보이기 위해, 우리가 만들 그래픽 유저 인터페이스(GUI) 도구는 아이템들의 리스트에 걸쳐 `draw` 메소드를 호출하고 그를 화면에 그리게 될 겁니다. 우리가 만들 라이브러리 크레이트는 `gui`라고 호명되고 GUI 라이브러리 구조를 포괄합니다. 이 크레이트는 사용자들이 사용할 수 있는 몇 가지 타입들, `Button`이나 `TextField` 들을 포함하게 될 것이구요. 추가로, `gui` 사용자들은 그들 고유의 타입을 만들어 그리고자 할 수 있습니다: 일례로, 한 프로그래머는 `Image`를 추가하고자 하고 다른 누군가는 `SelectBox`를 추가하고자 할 수 있습니다.

우리는 이번 예제에서 총체적인 GUI 라이브러리를 구현하지 않겠지만 부분적으로 그들 어떻게 함께 적절하게 동작할 수 있는지 보여주고자 합니다. 라이브러리를 작성하는 시간 동안, 우리는 다른 프로그래머들이 만들고자 하는 모든 타입들을 알 수 없죠. 하지만 우리가 알 수 있는 것은 `gui`가 다른 타입들의 다양한 값에 대해 계속해서 추적해야 하고, `draw` 메소드가 이 다양한 값들 가각에 호출되어야 한다는 겁니다. 정확히 우리가 `draw`메소드를 호출했을 때 벌어지는 일에 대해서 알 필요는 없고, 그저 우리가 메소드를 호출 할 수 있도록 어떤 값을 가지고 있으면 됩니다.

상속을 통해 이를 하기 위해서, `Component`라고 이름지은 클래스를 만들고 `draw`라는 이름의 메소드를 만들어줍니다. 다른 클래스들, `Button`, `Image` 그리고 `SelectBox`같은 다른 클래스들은 `Component`를 상속받고 또한 `draw`메소드를 물려받게 됩니다. 이들은 각각 `draw` 메소드를 재정의 하여 그들 고유의 행위를 정의할 수 있으나, 프레임워크는 모든 유형을 마치 `Component`인 것처럼 다룰 수 있고 `draw`를 호출할 수 있습니다. 하지만 Rust가 상속이 없는 관계로, `gui` 라이브러리를 구축하는 다른 방법을 찾아 상요자들이 새로운 타입을 정의하고 확장할 수 있도록 해야 합니다. 


### 일반적인 행위에 대한 특성 정의하기

`gui`에 원하는 바를 구현하기 위해, 우리는 `Draw`라는 이름의 특성을 정의하여 `draw`라는 이름의 메소드 하나를 줄겁니다. 그러면 *특성 객체*를 취하는 벡터를 정의할 수 있습니다. 특성 객체가 가르키는 타입은 우리가 특별히 구현한 특성이죠. 우리는 특성 객체를 어떤 포인터의 정렬로 지정하여 만들 수 있는데, 마치 `&` 참조자나 `Box<T>` 스마트 포인터처럼 말이죠, 그리고 관련된 특성을 지정합니다. (우리가 특성 객체에 포인터를 사용해야 하는 이유는 19장의 “Dynamically Sized Types & Sized” 절에서 다룰 겁니다.) 우리는 제네릭(단일형)이나 콘트렛(복합형) 대신 특성 객체를 사용할 수 있습니다. 특성 객체를 사용하는 곳이 어디든, Rust의 타입 시스템은 컴파일 타임에 어떤 값이 사용되든 문맥 안에서 특성 객체의 특성이 구현되도록 합니다. 

우리가 Rust에 대해 다뤘던 내용 중에, 구조체와 열거형을 되도록이면 “객체”로 부르는 것을 자제하여 다른 언어의 객체와 구분한다고 하였습니다. 구조체 또는 열거형에서는 구조체 필드의 데이터와 `impl` 블록의 동작이 분리되는 반면, 다른 언어에서는 데이터와 동작이 결합되어 객체로 명명됩니다. 그러나 특성 객체*들은* 데이터와 동작을 결합한다는 의미에서 다른 언어의 객체와 비슷합니다. 그러나 특성 객체는 특성 객체에 데이터를 추가 할 수 없다는 점에서 이전의 객체와 다릅니다. 특성 객체는 다른 언어의 객체 은 방식으로는 유용하지 않습니다: 그들의 특수한 목적은 공통행위를 추상화 할 수 있게 만드는 것이죠. 

항목 17-3은 `draw`라는 이름의 메소드를 갖는 `Draw `라는 특성을 정의하는 방법을 보여줍니다.


<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait Draw {
    fn draw(&self);
}
```

<span class="caption">Listing 17-3: Definition of the `Draw` trait</span>


이 문법은 10장에서 특성을 정의하는 방법에서 다뤘으니 익숙하실 겁니다. 다음은 새로운 문법입니다 : 항목 17-4는 `components` 라는 벡터를 보유하고있는 `Screen`이라는 구조체를 정의합니다.  `Box<Draw>` 타입의 벡터이며,  `Draw` 특성을 구현하는 `Box`에 속하는 모든 타입을 표준으로 취합니다.


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

<span class="caption">항목 17-4 : `Draw` 특성을 구현한 객체의 벡터 필드 `components`를 소유한 구조체 `Screen`의 정의 </span>


`Screen` 구조체에서는 항목 17-5와 같이 각 `components` 마다 `draw`메소드를 호출하는 `run` 메소드를 정의합니다.


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


<span class="caption">항목 17-5 : 각 컴포넌트에서 draw 메소드를 호출하는 Screen 에서 run 메소드 구현하기</span>


이것은 특성 범위에 제네릭 형식 매개 변수를 사용하는 구조체를 정의하는 것과 다른 작업입니다. 제네릭 형식 매개 변수는 한 번에 하나의 고정된 타입으로만 대입될 수 있습니다. 반면 특성 객체를 사용하면 런타임에 특성 객체에 대해 여러 고정 유형을 포함시킬 수 있습니다. 예를 들어, 항목 17-6처럼 특성 범위내 같이 제네릭 형식과 특성 범위을 사용하여 `Screen` 구조체를 정의 할 수 있습니다.


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


<span class="caption">항목 17-6 : 제네릭과 특성 범위를 사용하여 `Screen` 구조체와 `run` 메소드의 대체 구현</span>

이렇게하면 `Button` 유형의 모든 구성 요소 목록 또는 모든 유형의 `TextField`가 있는 `Screen` 인스턴스로 제한됩니다. 동일 유형의 콜렉션만 사용한다면 제네릭과 특성 범위를 사용하는 것이 바람직합니다. 왜냐하면 정의들은 컴파일 시에 단일 형태로 되어 고정 타입으로 사용되기 때문입니다.

반면에 특성 객체를 사용하는 메서드를 사용하면 하나의 `Screen` 인스턴스가 `Box<Button>` 혹은  `Box<TextField>`도 포함할 수 있는 `Vec<T>` 를 보유 할 수 있습니다. 이것이 어떻게 작동하는지 살펴보고 런타임 성능에 미치는 영향에 대해 설명하겠습니다.


### 특성 구현하기

이제 우리가 추가하려는 몇가지 타입은 `Draw` 특성을 구현합니다.  우리가 제공하려는 것은 `Button` 타입입니다.  다시금 말하지만, 실제 GUI 라이브러리를 구현하는 것은 이 부그의 범위를 벗어나므로, 우리는 `draw`에는 별다른 구현을 하지 않을 겁니다. 구현하려는 것을 그려보자면, `Button` 구조체는 `width`, `height` 그리고 `label`를 위한 필드를 갖습니다, 항목 17-7을 통해 확인할 수 있습니다.


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

<span class="caption">항목 17-7 : `Draw` 특성을 구현하는 `Button` 구조체</span>


`Button`의 필드인 `width`, `height` 및 `label`필드는 다른 컴포넌트와는 차이가 있는데, `TextField` 타입을 예로 들면,  이 필드들에 추가로 `placeholder` 필드를 추가로 소유할 겁니다. 우리가 화면에 그려주려는 각 필드의는 `Draw`특성을 구현할테지만 `draw`메소드는 각자가 다른 코드를 사용하여 특정 타입을 그리는 방법을 정의할 겁니다.  `Button`이 이 경우죠(이 챕터의 범주를 벗어나기 때문에 실질적인 GUI 코드 없이).  예를 들어, `impl` 블록에 추가적으로 사용자가 버튼을 클릭했을 때와 관련된 메소드들이 포함될 수 있습니다. 이런 종류의 메소드는 `TextField`와 같은 타입에는 적용할 수 없죠.

라이브러리를 사용하는 누군가가 `width`, `height` 및 `options` 필드가 있는 `SelectBox` 구조체를 구현하기로 했다면,  항목 17-8과 같이 `SelectBox` 타입에도 `Draw` 특성을 구현합니다. 


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

<span class="caption">항목 17-8: `gui`를 사용하고 `Draw`특성을 `SelectBox` 구조체에 구현한 또 다른 크레이트</span>


우리의 라이브러리 사용자는 `Screen` 인스턴스를 만들어 `main` 함수를 구현할 수 있습니다. `Screen`인스턴스에, 그들은 `SelectBox`와 `Button`을 추가할 수 있도록 각각을 `Box<T>`에 특성 객체로서 추가하면 됩니다. 그리고 `Screen` 인스턴스에 `run` 메소드를 호출함으로, 각 컴포넌트들에 대해 `draw`를 호출할 수 있습니다. 항목 17-9는 이런 대한 구현을 보여줍니다.


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

<span class="caption">항목 17-9: 특성 객체를 사용하여 동일 특성을 구현하는 다른 타입들의 값 저장하기</span>


우리가 라이브러리를 작성할 때는, 누군가 `SelectBox` 타입을 추가할 수도 있다는 것을 알 수 없었습니다.  하지만 우리가 구현한 `Screen`이 새로운 타입에 대해서도 동작하며 그를 그려낼 수 있는 이유는 
`SelectBox` 구현이 `Draw` 타입이기 때문이고, 이는 `draw` 메소드를 구현했다는 것을 의미합니다. 이런 개념 — 값이 어떤 고정 타입이냐가 아닌 값이 전달하고자 하는 메시지에 중심을 두는 — 은 *오리 타이핑*이란 개념으로 동적 타입 언어들에서 사용하는 것과 유사합니다: 만약 오리처럼 뒤뚱거리고 오리처럼 꽉꽉거리면, 그것은 오리여야 합니다! 

항목 17-5에 나오는 `Screen`에 구현된 `run`을 보면, `run`은 각 구서요소가 어떤 고정 타입인지 알 필요가 없습니다. 왜냐면 컴포넌트가 `Button`인지 `SelectBox`인지 타입에 대한 확인을 하지 않고, 그저 컴포넌트에 대해 `draw`메소드를 호출합니다.

특성 객체를 사용하고 Rust의 타입 시스템을 사용하여 코드를 작성하는 것은 오리타이핑을 사용하는 것과 같은 장점이 있는데, 이는 우리가 런타임에 값이 특정 메소드를 구현하지 않아서 에러가 발생하지 않을까 걱정되서 값을 체크할 필요가 전혀 없으면서도 어쨌든 이를 호출하게 된다는 겁니다. Rust는 값이 특성 객체가 필요한 특성을 구현하지 않았다면 컴파일하지 않을 겁니다.

예제 항목 17-10은 `String`을 컴포넌트로 `Screen`을 구현하면 어떤 일이 벌어지는지 보여줍니다.

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


<span class="caption">항목 17-10: 특성 객체의 특성을 구현하지 않은 타입의 사용을 시도</span>

우리는 에러를 보게 될 것이며 이유는 `String` 이 `Draw`특성을 구현하지 않기 때문입니다.


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


이 에러가 우리에게 알려주는 것은 우리가 통과하지 못할 것이라고 의도한 대로 `Screen`에 뭔가를 넘겨주었고, 
다른 타입을 전달하거나 `String`에 `Draw`를 구현하여 여기서 `draw`를 호출할 수 있도록 해야한다는 것 입니다.


### 특성 객체의 동적 디스패치 수행

10장에 나오는 “제네릭을 사용한 코드의 성능“ 섹션에서 우리가 제네릭에 특성 경계를 사용했을 때 컴파일러에 의해 이뤄지는 다형성 프로세스의 실행을 복기해보면: 다형성을 활용하여 작성된 코드는 *정적 디스패치*를 수행하는데, 이를 위해선 컴파일 타임에 당신이 호출하고자 하는 메소드를 컴파일러가 알고 있어야 합니다. 이는 *동적 디스패치*와 반대되는 개념으로, 이는 컴파일러가 당신이 호출하는 메소드를 컴파일 시에 알 수 없을 경우 수행됩니다. 동적 디스패치의 경우, 컴파일러가 생성된 코드는 런타임에 어떤 메소드가 호출될 지 알 수 있습니다. 

우리가 특성 객체를 사용할 때, Rust는 동적 디스패치만을 수행합니다. 컴파일러는 코드에서 사용될 가능성이 있는 모든 타입을 알지 못하기 때문에, 어떤 타입에 구현된 어떤 메소드를 호출할지 알지 못합니다.  대신 런타임에는, Rust는 특성 객체 내에 존재하는 포인터를 사용하여 어떤 메소드가 호출 될지 알아냅니다. 이렇게 탐색하는 데는 비용이 들며, 정적 디스패치 시에는 일어나지 않습니다. 동적 디스패치는 또한 메소드의 코드를 인라인할지 판단할 수 없게 만들기 때문에 몇가지 최적화를 수행하지 못하게 됩니다. 반면, 우리는 추가적인 유연성을 얻어 항목 17-5와 같은 코드를 작성할 수 있고, 항목 17-9과 같은 활용이 가능해지니, 여기에는 고려해야 할 기회비용이 있다고 하겠습니다.


### 개체의 안전성은 특성 객체의 요구사항

여러분은 *객체-안전*한 특성만을 특성 객체로 만들 수 있습니다. 특성 개체를 안전하게 만드는 모든 속성들을 관장하는 몇가지 복잡한 규칙이 있지만, 연습삼아 두 가지 규칙에 관련해서만 알아보고자 합니다. 특성은 개체 안전하고 만약 모든 메소드들이 특성에 정의되었다면 다음의 속성들을 갖습니다:

* 반환되는 값의 타입은  `Self`여서는 안된다.
* 제네릭 타입의 매개변수는 존재하지 않는다.

`Self` 키워드는 우리가 구현하는 특성이나 메소드의 별칭입니다. 특성 개체가 반드시 개체 안전해야 하는 이유는 여러분이 특성 객체를 한번 사용한 뒤에는 Rust가 특성에 구현된 고정 타입을 알 수 없기 때문입니다. 만약 특성 메소드가 고정된 `Self` 타입을 반환하는데 특성 개체는 `Self`의 정확한 타입을 모른다면, 원래 고정 타입을 메소드에서 사용할 수 있는 방법이 없습니다. 특성을 사용할 때 고정 타입 매개변수를 사용하는 제네릭 타입 매개변수도 동일합니다. 고정 타입은 특성이 구현되는 타입으로 결정됩니다. 특성 개체를 사용할 때의 타입을 모르면, 제네릭 타입 매개변수를 채울 타입을 알 수 없습니다. 

메소드가 객채 안전하지 않은 특성의 예는 표준 라이브러리의 `Clone` 특성 입니다.  
`Clone` 객체의 `clone` 메소드의 선언구는 다음과 같습니다:


```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```


`String` 타입은 `Clone` 특성을 구현하고, `String` 인스턴스에 `clone`메소드를 호출하면 우리는 `String`의 인스턴스를 반환받을 수 있습니다. 
비슷하게, 우리가 `Vec<T>`의 인스턴스에 `clone`을 호출하면, 우리는 `Vec<T>` 인스턴스를 얻을 수 있습니다. 
`clone` 선언은 `Self`에 어떤 타입이 사용되는지 알 필요가 있습니다, 왜냐면 그게 반환 타입이기 때문이죠. 

컴파일러는 여러분이 개체 안전성을 위반하는 무언가를 특성 개체에서 하려고 하면 알려줍니다.
예를 들어, 항목 17-4에서 `Screen` 구현을 보관하는 타입이 `Draw`특성 구현체가 대신 `Clone` 특성의 구현체가 되도록 바꿔봅시다: 이렇게요:


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


이 에러가 의미하는 바는, 여러분은 해당 특성을 해당 특성 개체에 이런 방식으로 사용해서는 안된다는 겁니다. 
혹시 개체 안전에 대해 보다 자세하게 알고 싶으시면 여기를 참고하세요 [Rust RFC 255].


[Rust RFC 255]: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md