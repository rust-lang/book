## 구조체를 사용한 예제 프로그램

어떨 때 구조체를 사용하면 좋을지를,
사각형 넓이를 계산하는 프로그램을 작성하면서 익혀보도록 합시다.
단일 변수로만 구성된 프로그램으로 시작해 구조체를 사용하기까지 리팩토링하면서 말이죠.

Cargo 를 사용해 *rectangles* 라는 새로운 바이너리 프로젝트를 만들어줍시다.
이 프로그램은 픽셀 단위로 명시된 사각형의 너비와 높이로 넓이를 계산할 겁니다.
Listing 5-8 은 *src/main.rs* 에 이 기능을 간단하게 구현한
모습입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

<span class="caption">Listing 5-8: 각 변수에 지정된 너비와 높이로
사각형 넓이 계산</span>

`cargo run` 으로 실행해보죠:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Listing 5-8 는 `area` 함수에
각 치수를 전달하고 호출합니다.
작동하는 데 문제는 없지만,
한 걸음 더 나아가보죠.

`area` 함수의 시그니처를 보면 개선해야 할 점이 여실히 드러납니다:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

너비와 높이 값은 둘이 합쳐져서 하나의 사각형을 묘사합니다.
즉, 서로 연관된 값입니다. 하지만 `area` 함수의 시그니처를 비롯해,
코드 내에 이 두 값이 서로 연관돼있다는 것을 표현하는 부분은 찾아볼 수 없군요.
두 값을 하나로 묶어버리면 코드의 가독성도 높아지고 관리하기도 쉬워질 겁니다.
앞서 3장 [“The Tuple Type”][the-tuple-type]<!-- ignore --> 절에서
배운 튜플로 해결해 볼까요?

### 튜플로 리팩토링하기

다음 Listing 5-9 는 튜플을 사용한 모습입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

<span class="caption">Listing 5-9: 사각형의 너비와 높이를
튜플로 명시하는 코드</span>

튜플을 사용함으로써 더 짜임새 있는 코드가 됐고,
인자도 단 하나만 넘기면 된다는 점에선 프로그램이 발전했다고 볼 수 있습니다.
하지만 각 요소가 이름을 갖지 않는 튜플의 특성 때문에
값을 인덱스로 접근해야 해서 계산식이 난잡해졌네요.

다행히 넓이를 계산할 땐 어떤 값이 너비이고 어떤 값이 높이인지 구분하지 못해도 별문제가 없습니다.
하지만 만들어야 할 프로그램이 화면에 사각형을 그리는 프로그램이라고 가정해보면 어떨까요?
너비 값인 `width` 가 튜플 인덱스 `0` 에 위치하고 높이 값인 `height` 는
튜플 인덱스 `1` 에 위치한다는 걸 꼭 기억하고 있어야 할 겁니다.
혹여나 다른 사람이 이 코드로 작업할 일이 생기면 그 사람도 이 점을 알아내서 기억해야 하겠죠.
값을 헷갈리기라도 하면 에러가 발생할 겁니다.
이런 문제를 방지하기 위해, 코드 내에 데이터의 의미를 담아봅시다.

### 구조체로 리팩토링하여 코드에 더 많은 의미를 담기

구조체는 데이터에 이름표를 붙여서 의미를 나타낼 수 있습니다.
Listing 5-10 처럼, 기존에 사용하던 튜플을 구조체로 바꿔
각 요소에 이름을 지어줍시다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<span class="caption">Listing 5-10: `Rectangle` 구조체 정의</span>

`Rectangle` 이라는 구조체를 정의하고,
중괄호 내에 `width`, `height` 필드를 `u32` 타입으로 정의했습니다.
이후 `main` 내에선 너비 30, 높이 50짜리
`Rectangle` 구조체의 인스턴스를 생성했습니다.

`area` 함수의 매개변수는 이제 `rectangle` 하나뿐입니다.
단, 구조체의 소유권을 가져와 버리면 `main` 함수에서
`area` 함수 호출 이후에 `rect1` 을 더 사용할 수 없으므로,
`rectangle` 매개변수의 타입을 불변 참조자 타입으로 정하여
소유권을 빌려오기만 하도록 만들었습니다.
불변 참조자 타입이니 함수 시그니처와 호출시에 `&` 를 작성합니다.

`area` 함수는 `Rectangle` 인스턴스의
`width`, `height` 필드에 접근하여 `area`, 즉 넓이를 계산합니다.
이제 함수 시그니처만 봐도 의미를 정확히 알 수 있네요.
`width`, `height` 가 서로 연관된 값이라는 것도 알 수 있고,
`0` 이나 `1` 대신 필드명을 사용해 더 명확하게
구분할 수 있습니다.

## 트레잇 derive 로 유용한 기능 추가하기

프로그램을 디버깅하는 동안 `Rectangle` 인스턴스 내
모든 필드 값을 출력해서 확인할 수 있다면 좋을 것 같군요.
Listing 5-11 는 앞서 다뤄본 `println!` 매크로를 사용해본 예시이나,
작동하진 않습니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<span class="caption">Listing 5-11: `Rectangle` 인스턴스
출력을 시도해본 모습</span>

이 코드를 컴파일하면 다음과 같은 메시지가 나타납니다:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

`println!` 매크로에는 여러 출력 형식을 사용할 수 있습니다.
그리고 기본 형식인 `{}` 로 지정할 땐 `Display` 라는, 최종 사용자를 위한 출력 형식을 사용하죠.
여태 사용했던 기본 타입들은 `Display` 가 기본적으로 구현돼있었습니다.
`1` 같은 기본 타입들을 사용자에게 보여줄 수 있는 형식은 딱 한 가지뿐이니까요.
하지만 구조체라면 이야기가 달라집니다.
중간중간 쉼표를 사용해야 할 수도 있고, 중괄호도 출력해야 할 수도 있고,
필드 일부를 생략해야 할 수도 있는 등 여러 경우가 있을 수 있습니다.
러스트는 이런 애매한 상황에 우리가 원하는 걸 임의로 예상해서 제공하려 들지 않기 때문에,
구조체에는 `Display` 구현체가 기본 제공되지 않습니다.
따라서 `println!` 에서 처리할 수 없죠.

에러를 더 읽다 보면 다음과 같은 도움말을 찾을 수 있습니다:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

`{}` 대신 `{:?}` 를 사용해보라네요. 한번 해보죠.
`println!` 매크로 호출을 `println!("rect1 is {:?}", rect1);` 으로 바꿔봅시다.
`{}` 내에 `:?` 를 추가하는 건 `println!` 에 `Debug` 라는 출력 형식을 사용하고 싶다고 전달하는 것과 같습니다.
이 `Debug` 라는 트레잇은 최종 사용자가 아닌, 개발자에게 유용한 방식으로 출력하는,
즉 디버깅할 때 값을 볼 수 있게 해주는 트레잇입니다.

변경하고 나서 다시 컴파일해 보면, 어째서인지 여전히 에러가 발생하네요:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

그런데 컴파일러가 또 무언가를 알려주네요:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

러스트는 디버깅 정보를 출력하는 기능을 *자체적으로 가지고 있습니다*.
하지만 우리가 만든 구조체에 해당 기능을 적용하려면 명시적인 동의가 필요하므로,
Listing 5-12 처럼 구조체 정의 바로 이전에 `#[derive(Debug)]`
어노테이션을 작성해주어야 합니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<span class="caption">Listing 5-12: `Rectangle` 인스턴스를 디버그 출력 형식으로
사용하기 위해, 어노테이션을 추가하여 `Debug` 트레잇 derive 하기</span>

이제 프로그램을 실행해 보면 더 이상 에러가 나타나지 않고,
다음과 같은 출력이 나타날 겁니다:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

가장 예쁜 출력 형태라 할 수는 없지만,
인스턴스 내 모든 필드 값을 보여주므로 디버깅하는 동안에는 확실히 유용할 겁니다.
필드가 더 많은 구조체라면 이보다 더 읽기 편한 형태가 필요할 텐데요,
그럴 땐 `println!` 문자열 내에 `{:?}` 대신 `{:#?}` 를 사용하면 됩니다.
`{:#?}` 를 사용했을 때의 출력 예시는 다음과 같습니다.

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

러스트에선 이처럼 `derive` 어노테이션으로 우리가 만든 타입에
유용한 동작을 추가할 수 있는 트레잇을 여럿 제공합니다.
이들 목록 및 각각의 동작은 부록 C에서 확인할 수 있으니 참고해주세요.
또한, 여러분만의 트레잇을 직접 만들고, 이런 트레잇들의 동작을 원하는 대로 커스터마이징 해서 구현하는 방법은 10장에서 배울 예정입니다.

본론으로 돌아옵시다.
우리가 만든 `area` 함수는 사각형의 면적만을 계산하며,
`Rectangle` 구조체를 제외한 다른 타입으로는 작동하지 않습니다.
그렇다면 아예 `Rectangle` 구조체와 더 밀접하게 만드는 편이 낫지 않을까요?
다음에는 `area` 함수를 `Rectangle` 타입 내에 *메소드(method)* 형태로 정의하여 코드를 리팩토링하는 방법을 알아보겠습니다.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
