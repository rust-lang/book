## 제네릭 데이터 타입

함수 시그니처나 구조체에서와 같은 방식으로, 우리가 일반적으로 타입을 쓰는 곳에다 제네릭을 이용하는 것은
여러 다른 종류의 구체적인 데이터 타입에 대해 사용할 수 있는 정의를 생성하도록 해줍니다. 제네릭을
이용하여 함수, 구조체, 열거형, 그리고 메소드를 정의하는 방법을 살펴본 뒤, 이 절의 끝에서 제네릭을
이용한 코드의 성능에 대해 논의하겠습니다.

### 함수 정의 내에서 제네릭 데이터 타입을 이용하기

우리는 함수의 시그니처 내에서 파라미터의 데이터 타입과 반환 값이 올 자리에 제네릭을 사용하는 함수를
정의할 수 있습니다. 이러한 방식으로 작성된 코드는 더 유연해지고 우리 함수를 호출하는 쪽에서 더 많은
기능을 제공할 수 있는 한편, 코드 중복을 야기하지도 않습니다.

우리의 `largest` 함수로 계속 진행하면, Listing 10-4는 슬라이스 내에서 가장 큰 값을 찾는
동일한 기능을 제공하는 두 함수를 보여주고 있습니다. 첫 번째 함수는 Listing 10-3에서 추출한
슬라이스에서 가장 큰 `i32`를 찾는 함수입니다. 두 번째 함수는 슬라이스에서 가장 큰 `char`를
찾습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&numbers);
    println!("The largest number is {}", result);
#    assert_eq!(result, 100);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&chars);
    println!("The largest char is {}", result);
#    assert_eq!(result, 'y');
}
```

<span class="caption">Listing 10-4: 이름과 시그니처만 다른 두 함수들</span>

여기서 함수 `largest_i32`와 `largest_char`는 정확히 똑같은 본체를 가지고 있으므로, 만일 우리가
이 두 함수를 하나로 바꿔서 중복을 제거할 수 있다면 좋을 것입니다. 운 좋게도, 제네릭 타입 파라미터를
도입해서 그렇게 할 수 있습니다!

우리가 정의하고자 하는 함수의 시그니처 내에 있는 타입들을 파라미터화 하기 위해서, 타입 파라미터를 위한
이름을 만들 필요가 있는데, 이는 값 파라미터들의 이름을 함수에 제공하는 방법과 유사합니다. 우리는
`T`라는 이름을 선택할 겁니다. 어떤 식별자(identifier)든지 타입 파라미터의 이름으로 사용될 수
있지만, 러스트의 타입 이름에 대한 관례가 낙타 표기법(CamelCase)이기 때문에 `T`를 사용하려고
합니다. 제네릭 타입 파라미터의 이름은 또한 관례상 짧은 경향이 있는데, 종종 그냥 한 글자로 되어
있습니다. "type"을 줄인 것으로서, `T`가 대부분의 러스트 프로그래머의 기본 선택입니다.

함수의 본체에 파라미터를 이용할 때는, 시그니처 내에 그 파라미터를 선언하여 해당 이름이 함수 본체
내에서 무엇을 의미하는지 컴파일러가 할 수 있도록 합니다. 비슷하게, 함수 시그니처 내에서 타입 파라미터
이름을 사용할 때는, 사용 전에 그 타입 파라미터 이름을 선언해야 합니다. 타입 이름 선언은 함수의 이름과
파라미터 리스트 사이에 꺾쇠괄호를 쓰고 그 안에 넣습니다.

우리가 정의하고자 하는 제네릭 `largest` 함수의 함수 시그니처는 아래와 같이 생겼습니다:

```rust,ignore
fn largest<T>(list: &[T]) -> T {
```

이를 다음과 같이 읽습니다: 함수 `largest`는 어떤 타입 `T`을 이용한 제네릭입니다. 이것은 `list`라는
이름을 가진 하나의 파라미터를 가지고 있고, `list`의 타입은 `T` 타입 값들의 슬라이스입니다.
`largest` 함수는 동일한 타입 `T` 값을 반환할 것입니다.

Listing 10-5는 함수 시그니처 내에 제네릭 데이터 타입을 이용한 통합된 형태의 `largest` 함수 정의를
보여주며, 또한 `i32` 값들의 슬라이스 혹은 `char` 값들의 슬라이스를 가지고 어떻게 `largest`를 호출할
수 있을지를 보여줍니다. 이 코드가 아직 컴파일되지 않는다는 점을 주의하세요!

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
}
```

<span class="caption">Listing 10-5: 제네릭 타입 파라미터를 이용하지만 아직
컴파일되지 않는 `largest` 함수의 정의</span>

이 코드를 지금 컴파일하고자 시도하면, 다음과 같은 에러를 얻게 될 것입니다:

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
  |
5 |         if item > largest {
  |            ^^^^
  |
note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

위 노트는 `std::cmp::PartialOrd`를 언급하는데, 이는 *트레잇(trait)* 입니다. 트레잇에
대해서는 다음 절에서 살펴볼 것이지만, 간략하게 설명하자면, 이 에러가 말하고 있는 것은 `T`가
될 수 있는 모든 가능한 타입에 대해서 동작하지 않으리라는 것입니다: 함수 본체 내에서 `T` 타입의
값을 비교하고자 하기 때문에, 어떻게 순서대로 정렬하는지 알고 있는 타입만 사용할 수 있는 것입니다.
표준 라이브러리는 어떤 타입에 대해 비교 연산이 가능하도록 구현할 수 있는 트레잇인
`std::cmp::PartialOrd`을 정의해뒀습니다. 다음 절에서 트레잇, 그리고 어떤 제네릭 타입이 특정
트레잇을 갖도록 명시하는 방법을 알아보기 위해 돌아올 것이지만, 이 예제는 잠시 옆으로 치워두고
제네릭 타입 파라미터를 이용할 수 있는 다른 곳을 먼저 돌아봅시다.

<!-- Liz: this is the reason we had the topics in the order we did in the first
draft of this chapter; it's hard to do anything interesting with generic types
in functions unless you also know about traits and trait bounds. I think this
ordering could work out okay, though, and keep a stronger thread with the
`longest` function going through the whole chapter, but we do pause with a
not-yet-compiling example here, which I know isn't ideal either. Let us know
what you think. /Carol -->

### 구조체 정의 내에서 제네릭 데이터 타입 사용하기

우리는 또한 하나 혹은 그 이상의 구조체 필드 내에 제네릭 타입 파라미터를 사용하여 구조체를 정의할 수
있습니다. Listing 10-6은 임의의 타입으로 된 `x`와 `y` 좌표값을 가질 수 있는 `Point` 구조체의
정의 및 사용법을 보여주고 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

<span class="caption">Listing 10-6: `T` 타입의 값 `x`와 `y`를 갖는 `Point`
구조체</span>

문법은 함수 정의 내에서의 제네릭을 사용하는 것과 유사합니다. 먼저, 구조체 이름 바로 뒤에 꺾쇠괄호를
쓰고 그 안에 타입 파라미터의 이름을 선언해야 합니다. 그러면 구조체 정의부 내에서 구체적인 데이터 타입을
명시하는 곳에 제네릭 타입을 이용할 수 있습니다.

`Point`의 정의 내에서 단 하나의 제네릭 타입을 사용했기 때문에, `Point` 구조체는 어떤 타입 `T`를
이용한 제네릭이고 `x`와 `y`가 이게 결국 무엇이 되든 간에 *둘 다* 동일한 타입을 가지고 있다고
말할 수 있음을 주목하세요. 만일 Listing 10-7에서와 같이 다른 타입의 값을 갖는 `Point`의
인스턴스를 만들고자 한다면, 컴파일이 되지 않을 것입니다:
`

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

<span class="caption">Listing 10-7: `x`와 `y` 필드는 둘 모두 동일한 제네릭
데이터 타입 `T`를 가지고 있기 때문에 동일한 타입이어야 합니다</span>

이 코드를 컴파일하고자 하면, 다음과 같은 에러를 얻게 될 것입니다:

```text
error[E0308]: mismatched types
 -->
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integral variable, found
  floating-point variable
  |
  = note: expected type `{integer}`
  = note:    found type `{float}`
```

`x`에 정수 5를 대입할 때, 컴파일러는 이 `Point`의 인스턴스에 대해 제네릭 타입 `T`가 정수일 것이고
알게 됩니다. 그다음 `y`에 대해 4.0을 지정했는데, 이 `y`는 `x`와 동일한 타입을 갖도록 정의되었으므로,
타입 불일치 에러를 얻게 됩니다. 

만일 `x`와 `y`가 서로 다른 타입을 가지지만 해당 타입들이 여전히 제네릭인 `Point` 구조체를 정의하길
원한다면, 여러 개의 제네릭 타입 파라미터를 이용할 수 있습니다. Listing 10-8에서는 `Point`의 정의를
`T`와 `U`를 이용한 제네릭이 되도록 변경했습니다. 필드 `x`의 타입은 `T`이고, 필드 `y`의 타입은
`U`입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

<span class="caption">Listing 10-8: 두 타입을 이용한 제네릭이어서 `x`와 `y`가
다른 타입의 값일 수도 있는 `Point`</span>

이제 위와 같은 모든 `Point` 인스턴스가 허용됩니다! 정의 부분에 여러분이 원하는 만큼 많은 수의 제네릭
타입 파라미터를 이용할 수 있지만, 몇몇 개보다 더 많이 이용하는 것은 읽고 이해하는 것을 어렵게 만듭니다.
여러분이 많은 수의 제네릭 타입을 필요로 하는 지점에 다다랐다면, 이는 아마도 여러분의 코드가 좀 더 작은
조각들로 나뉘는 재구조화가 필요할지도 모른다는 징조입니다.

### 열거형 정의 내에서 제네릭 데이터 타입 사용하기

구조체와 유사하게, 열거형도 그 variant 내에서 제네릭 데이터 타입을 갖도록 정의될 수 있습니다.
6장에서 표준 라이브러리가 제공하는 `Option<T>` 열거형을 이용해봤는데, 이제는 그 정의를 좀 더
잘 이해할 수 있겠지요. 다시 한번 봅시다:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

달리 말하면, `Option<T>`는 `T` 타입에 제네릭인 열거형입니다. 이것은 두 개의 variant를 가지고
있습니다: 타입 `T` 값 하나를 들고 있는 `Some`, 그리고 어떠한 값도 들고 있지 않는 `None` variant
입니다. 표준 라이브러리는 구체적인 타입을 가진 이 열거형에 대한 값의 생성을 지원하기 위해서 딱 이 한 가지
정의만 가지고 있으면 됩니다. "옵션 값"의 아이디어는 하나의 명시적인 타입에 비해 더 추상화된 개념이고,
러스트는 이 추상화 개념을 수많은 중복 없이 표현할 수 있도록 해줍니다.

열거형은 또한 여러 개의 제네릭 타입을 이용할 수 있습니다. 우리가 9장에서 사용해본 `Result` 열거형의
정의가 한 가지 예입니다:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 열거형은 `T`와 `E`, 두 개의 타입을 이용한 제네릭입니다. `Result`는 두 개의 variant를
가지고 있습니다: 타입 `T`의 값을 들고 있는 `Ok`, 그리고 타입 `E`의 값을 들고 있는 `Err`입니다.
이 정의는 성공하거나 (그래서 어떤 `T` 값을 반환하거나) 혹은 실패하는 (그래서 `E` 타입으로 된 에러를
반환하는) 연산이 필요한 어디에서든 편리하게 `Result` 열거형을 이용하도록 해줍니다. Listing 9-2에
우리가 파일을 열 때를 상기해보세요: 이 경우, 파일이 성공적으로 열렸을 때는 `T`에 `std::fs::File`
타입의 값이 채워지고 파일을 여는데 문제가 생겼을 때는 `E`에 `std::io::Error` 타입으로 된 값이
채워졌습니다.

여러분의 코드에서 단지 들고 있는 값의 타입만 다른 여러 개의 구조체나 열거형이 있는 상황을 인지했다면,
우리가 함수 정의에서 제네릭 타입을 대신 도입하여 사용했던 것과 똑같은 절차를 통해 그러한 중복을
제거할 수 있습니다.

### 메소드 정의 내에서 제네릭 데이터 타입 사용하기

5장에서 했던 것과 유사하게, 정의부에 제네릭 타입을 갖는 구조체와 열거형 상의 메소드를 구현할 수도
있습니다. Listing 10-9는 우리가 Listing 10-6에서 정의했던 `Point<T>` 구조체를 보여주고
있습니다. 그러고 나서 필드 `x`의 값에 대한 참조자를 반환하는 `x`라는 이름의 메소드를 `Point<T>`
상에 정의했습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

<span class="caption">Listing 10-9: `T` 타입의 `x` 필드에 대한 참조자를 반환하는
`Point<T>` 구조체 상에 `x`라는 이름의 메소드 정의</span>

`impl` 바로 뒤에 `T`를 정의해야만 타입 `Point<T>` 메소드를 구현하는 중에 이를 사용할 수 있음을
주목하세요.

구조체 정의 내에서의 제네릭 타입 파라미터는 여러분이 구조체의 메소드 시그니처 내에서 사용하고 싶어하는
제네릭 타입 파라미터와 항상 같지 않습니다. Listing 10-10에서는 Listing 10-8에서의
`Point<T, U>` 구조체 상에 `mixup` 이라는 메소드를 정의했습니다. 이 메소드는 또다른 `Point`를
파라미터로 갖는데, 이는 우리가 호출하는 `mixup` 상의 `self`의 `Point`와 다른 타입을 가지고 있을
수도 있습니다. 이 메소드는 새로운 `Point`를 생성하는데 `self` `Point`로부터 (`T` 타입인) `x`
값을 가져오고, 파라미터로 넘겨받은 `Point`로부터 (`W` 타입인) `y` 값을 가져온 것입니다:


<span class="filename">Filename: src/main.rs</span>

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

<span class="caption">Listing 10-10: 구조체 정의에서와는 다른 제네릭 타입을 사용하는
메소드</span>

`main`에서, 우리는 (`5` 값을 갖는) `x`에 대해 `i32`를, (`10.4` 값을 갖는) `y`에 대해
`f64`를 사용하는 `Point`를 정의했습니다. `p2`는 (`"Hello"` 값을 갖는) `x`에 대해 스트링
슬라이스를, (`c` 값을 갖는) `y`에 대해 `char`를 사용하는 `Point`입니다. `p1`상에서 인자로
`p2`를 넘기는 `mixup` 호출은 `p3`을 반환하는데, 이는 `x`가 `p1`으로부터 오기 때문에 `x`는
`i32` 타입을 갖게 될 것입니다. 또한 `y`는 `p2`로부터 오기 때문에 `p3`은 `y`에 대해 `char`
타입을 가지게 될 것입니다. `println!`은 `p3.x = 5, p3.y = c`를 출력하겠지요.

제네릭 파라미터 `T`와 `U`는 `impl` 뒤에 선언되었는데, 이는 구조체 정의와 함께 사용되기 때문임을
주목하세요. 제네릭 파라미터 `V`와 `W`는 `fn mixup` 뒤에 선언되었는데, 이는 이들이 오직
해당 메소드에 대해서만 관련이 있기 때문입니다.

### 제네릭을 이용한 코드의 성능

여러분이 이 절을 읽으면서 제네릭 타입 파라미터를 이용한 런타임 비용이 있는지 궁금해하고 있을런지도
모르겠습니다. 좋은 소식을 알려드리죠: 러스트가 제네릭을 구현한 방식이 의미하는 바는 여러분이 제네릭
파라미터 대신 구체적인 타입을 명시했을 때와 비교해 전혀 느려지지 않을 것이란 점입니다!

러스트는 컴파일 타임에 제네릭을 사용하는 코드에 대해 *단형성화(monomorphization)* 를 수행함으로써
이러한 성능을 이루어 냈습니다. 단형성화란 제네릭 코드를 실제로 채워질 구체적인 타입으로 된 특정 코드로
바꾸는 과정을 말합니다.

컴파일러가 하는 일은 Listing 10-5에서 우리가 제네릭 함수를 만들 때 수행한 단계들을 반대로 한 것입니다.
컴파일러는 제네릭 코드가 호출되는 모든 곳을 살펴보고 제네릭 코드가 호출될 때 사용된 구체적인 타입에 대한
코드를 생성합니다.

표준 라이브러리의 `Option` 열거형을 사용하는 예제를 통해 알아봅시다:

```rust
let integer = Some(5);
let float = Some(5.0);
```

러스트가 이 코드를 컴파일할 때, 단형성화를 수행할 것입니다. 컴파일러는 `Option`에 넘겨진 값들을 읽고
두 종류의 `Option<T>`를 가지고 있다는 사실을 알게 됩니다: 하나는 `i32`이고 나머지 하나는 `f64`
이지요. 그리하여 컴파일러는 제네릭 정의를 명시적인 것들로 교체함으로써 `Option<T>`에 대한 제네릭
정의를 `Option_i32`와 `Option_f64`로 확장시킬 것입니다.

컴파일러가 생성한 우리의 단형성화된 버전의 코드는 아래와 같이 보이게 되는데, 컴파일러에 의해 생성된
구체화된 정의로 교체된 제네릭 `Option`이 사용되었습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

우리는 제네릭을 사용하여 중복 없는 코드를 작성할 수 있고, 러스트는 이를 각 인스턴스에 대해 구체적인
타입을 갖는 코드로 컴파일할 것입니다. 이는 우리가 제네릭을 사용하는 데에 어떠한 런타임 비용도 없음을
의미합니다; 코드가 실행될 때, 손으로 각각 특정 정의를 중복시킨 것과 같이 실행될 것입니다.  단형성화의
과정은 러스트의 제네릭이 런타임에 극도로 효율적 이도록 만들어 주는 것입니다.
