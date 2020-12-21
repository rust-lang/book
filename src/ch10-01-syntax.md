## 제네릭 데이터 타입

제네릭을 이용하면 함수 시그니처나 구조체의 요소가
다양한 구체적 데이터 타입을 사용할 수 있도록 정의할 수 있습니다.
함수, 구조체, 열거형, 메소드를 제네릭으로 정의하는 방법을 알아보고,
제네릭이 코드 성능에 미치는 영향을 알아보겠습니다.

### 제네릭 함수 정의

제네릭 함수를 정의할 때는, 함수 시그니처 내 매개변수와
반환 값의 데이터 타입 위치에 제네릭을 사용합니다.
이렇게 작성된 코드는 더 유연해지고, 우리 함수를 호출하는 쪽에서
더 많은 기능을 사용할 수 있도록 하며 코드 중복 또한 방지합니다.

`largest` 함수를 이용해 계속해보겠습니다. Listing 10-4는
슬라이스에서 가장 큰 값을 찾는 두 함수를 보여줍니다.

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
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
#    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
#    assert_eq!(result, 'y');
}
```

<span class="caption">Listing 10-4: 이름과 타입 시그니처만 다른
두 함수</span>

함수 `largest_i32` 는 Listing 10-3에서 봤던 슬라이스에서 가장 큰 `i32` 를 찾는 함수이고,
`largest_char` 함수는 슬라이스에서 가장 큰 `char` 를 찾는 함수입니다.
이 두 함수의 본문은 완벽히 동일하니, 제네릭을 이용해
이 두 함수를 하나로 만들어서 코드 중복을 제거해보겠습니다.

새롭게 정의할 함수의 시그니처 내 타입을 매개변수화하려면 타입 매개변수의 이름을 지어줄 필요가 있습니다.
방법은 함수 매개변수와 비슷합니다.
타입 매개변수의 이름에는 아무 식별자나 사용할 수 있지만, `T` 로 정하는 것이 일반적입니다.
러스트에서는 타입 이름을 지어줄 때는 낙타 표기법(CamelCase)을 따르고, 매개변수의 이름은 짧게
(한 글자로만 된 경우도 종종 있습니다) 짓는 것이 관례이기 때문에,
대부분의 러스트 프로그래머는 'type'을 줄인 `T` 를 사용합니다.

함수 본문에서 매개변수를 사용하려면 함수 시그니처에 매개변수의 이름을 선언하여
컴파일러에게 해당 이름이 무엇을 의미하는지 알려주어야 해야 하는 것처럼,
타입 매개변수를 사용하기 전에도 타입 매개변수의 이름을 선언해야 합니다.
예를 들어, 제네릭 `largest` 함수를 정의하려면 함수명과
매개변수 목록 사이의 꺾쇠괄호(`<>`)에 타입 매개변수 이름을 선언해야 합니다.
다음과 같습니다:

```rust,ignore
fn largest<T>(list: &[T]) -> T {
```

이는 "`largest` 함수는 어떤 타입 `T` 에 대한 제네릭 함수" 라고 읽습니다.
이 함수는 `T` 타입 값들의 슬라이스인 `list` 매개변수를 가지며,
동일한 `T` 타입의 값을 반환합니다.

Listing 10-5는 제네릭 데이터 타입을 사용해 하나로 통합한 `largest` 함수 정의를 나타냅니다.
코드에서 볼 수 있듯, 우린 이 함수를 `i32` 값들의 슬라이스로 호출할 수도 있고
`char` 값들의 슬라이스로도 호출할 수 있습니다.
다만 이 코드는 컴파일되지 않으니 유의해주세요! 에러 해결법은 이번 장에서 천천히 알아보겠습니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
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
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

<span class="caption">Listing 10-5: 제네릭 타입 매개변수를 이용한 `largest` 함수
(아직 컴파일되지는 않음)</span>

이 코드를 지금 바로 컴파일해 보면 다음과 같은 에러가 나타납니다:

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:12
  |
5 |         if item > largest {
  |            ^^^^^^^^^^^^^^
  |
  = note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

`std::cmp::PartialOrd` 가 언급되는데, 이는 *트레잇(trait)* 입니다.
트레잇은 다음 절에서 살펴볼 예정이지만 지금 이 에러를 간단히 설명하자면,
`largest` 함수는 `T` 가 될 수 있는 모든 타입으로 동작할 수는 없다는 뜻입니다.
함수 본문에서 `T` 타입 값들을 서로 비교하기 때문에,
서로 비교할 수 없는 타입들은 사용할 수 없습니다. 어떤 타입의 값을 서로 비교하려면
`std::cmp::PartialOrd` 트레잇을 해당 타입이 구현해야만 하기 때문입니다.
(이 트레잇의 자세한 설명은 부록 C를 참고해주세요.)
제네릭 타입이 특정 트레잇을 구현하도록 명시하는 방법은
["매개변수로서의 트레잇"][traits-as-parameters]<!-- ignore --> 에서 배울 예정이니,
일단은 또 다른 제네릭 타입 매개변수 사용법을 알아보도록 하죠.

### 제네릭 구조체 정의

`<>` 문법으로 구조체 필드에서 제네릭 타입 매개변수를 사용하도록
구조체를 정의할 수도 있습니다. Listing 10-6은 임의의 타입으로 된
`x`, `y` 를 갖는 `Point<T>` 구조체를 정의하는 방법을 나타냅니다.

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

<span class="caption">Listing 10-6: `T` 타입의 값 `x`, `y`를 갖는
`Point<T>` 구조체</span>

문법은 함수 정의에서 제네릭을 사용하는 것과 유사합니다.
먼저 구조체 이름 바로 뒤 꺾쇠괄호에
타입 매개변수 이름을 선언하고,
구조체 정의 내 구체적 데이터 타입을 명시하던 곳에
제네릭 타입을 대신 사용합니다.

`Point<T>` 선언에 하나의 제네릭 타입만 사용했으므로,
이 선언은 `Point<T>` 가 어떤 타입 `T` 에 대한 제네릭이며
`x`, `y` 필드는 실제 타입이 무엇이건 간에 *서로* 동일한 타입이라는 것을 의미합니다.
만약 Listing 10-7처럼 서로 다른 타입의 값을 갖는 `Point<T>` 인스턴스를 생성하려고 할 경우,
코드는 컴파일되지 않습니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

<span class="caption">Listing 10-7: `x`와 `y` 필드는 둘 다 동일한
제네릭 데이터 타입 `T` 이므로, 서로 동일한 타입이어야 합니다</span>

컴파일러는 우리가 `x` 에 정수 값 5를 대입할 때 `Point<T>` 인스턴스의
제네릭 타입 `T` 를 정수 타입으로 인지합니다.
우린 그다음 `y` 에 4.0을 지정했는데, `y` 는 `x` 와 동일한 타입을 갖도록
정의되었으므로 컴파일러는 타입 불일치 에러를 발생시킵니다:

```text
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integral variable, found
floating-point variable
  |
  = note: expected type `{integer}`
             found type `{float}`
```

제네릭 `Point` 구조체의 `x`, `y`가 서로 다른 타입일 수 있도록
정의하고 싶다면 여러 개의 제네릭 타입 매개변수를 사용해야 합니다.
Listing 10-8에서는 `x`는 `T` 타입으로, `y`는 `U` 타입으로 정의한
제네릭 `Point` 정의를 나타냅니다.

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

<span class="caption">Listing 10-8: 두 타입의 제네릭을 사용하여,
`x`와 `y`가 서로 다른 타입의 값이 될 수 있는 `Point<T, U>`</span>

이제 위와 같이 모든 `Point` 인스턴스를 생성할 수 있습니다!
제네릭 타입 매개변수는 여러분이 원하는 개수만큼 정의할 수 있지만,
많으면 많아질수록 코드 가독성은 떨어집니다. 만약 코드에서 수많은 제네릭 타입이 필요하다면,
코드를 리팩토링해서 작은 부분들로 나누는 것을 고려해보셔야 할지도 모릅니다.

### 제네릭 열거형 정의

구조체처럼, 열거형도 variant 내에서 제네릭 데이터 타입을 갖도록 정의할 수 있습니다.
6장에서 사용했었던 표준 라이브러리의 `Option<T>` 열거형을
다시 살펴봅시다:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

이제 여러분은 이 코드를 이해할 수 있습니다.
보시다시피 `Option<T>` 는 `T` 타입에 대한 제네릭이며,
`T` 타입을 들고 있는 `Some` variant와
아무런 값도 들고 있지 않는 `None` variant를 갖습니다.
`Option<T>` 열거형이 제네릭으로 되어있는 덕분에,
옵션 값이 어떤 타입이건 상관없이 추상화하여 사용할 수 있죠.

열거형에서도 여러 개의 제네릭 타입을 이용할 수 있습니다.
우리가 9장에서 사용한 `Result` 열거형의 정의가 대표적인 예시입니다:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 열거형은 `T`, `E` 두 타입을 이용한 제네릭이며,
`T` 타입 값을 갖는 `Ok`와 `E` 타입 값을 갖는 `Err` variant를 갖습니다.
제네릭으로 정의되어있는 덕분에,
`Result` 열거형을 연산이 성공할지(따라서 `T` 타입 값을 반환할지)
실패할지(`E` 타입 값을 반환할지) 알 수 없는 어떤 곳에서든 편리하게 사용할 수 있습니다.
우리가 Listing 9-3 에서 파일을 열 때도 사용했었죠.
이때는 파일을 여는 데 성공하면 `T` 는 `std::fs::File` 타입이 되고,
파일을 열다가 문제가 생기면 `E` 는 `std::io::Error` 타입이 됐었습니다.

여러분이 작성한 코드에 보유하는 값의
타입만 다른 구조체나 열거형이 여럿 있다면,
제네릭 타입을 사용해 코드 중복을 제거해보세요.

### 제네릭 메소드 정의

5장에서 했던 것처럼 구조체나 열거형에 메소드를 구현하되,
제네릭 타입을 이용해 정의할 수도 있습니다. Listing 10-9는
Listing 10-6에서 정의했던 `Point<T>` 구조체에 `x` 메소드를 구현한 모습입니다.

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

<span class="caption">Listing 10-9: `T` 타입의 `x` 필드에 대한
참조자를 반환하는 `x` 메소드를 `Point<T>` 에
정의</span>

`x` 필드 데이터의 참조자를 반환하는 `x` 메소드를 `Point<T>` 에
정의해보았습니다.

`impl` 바로 뒤에 `T` 를 선언하여 `Point<T>` 타입에
메소드를 구현한다는 것을 명시했음을 유의하세요.
이렇게 하면 러스트는 `Point` 의 꺾쇠괄호 내 타입이
구체적인 타입이 아닌 제네릭 타입임을 인지합니다.

임의 제네릭 타입의 `Point<T>` 가 아닌, `Point<f32>` 인스턴스에만
메소드를 정의할 수도 있습니다. Listing 10-10은 `impl` 뒤에 타입을 선언하지 않고
구체적인 타입인 `f32` 를 사용한 모습입니다.

```rust
# struct Point<T> {
#     x: T,
#     y: T,
# }
#
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

<span class="caption">Listing 10-10: 구조체의 제네릭 타입 매개변수 `T`가
특정 구체적 타입인 경우에만 적용되는 `impl` 블록</span>

이 코드에서 `Point<f32>` 타입 인스턴스는
`distance_from_origin` 메소드를 사용할 수 있지만,
`T` 가 `f32` 타입이 아닌 `Point<T>` 인스턴스는 이 메소드를 사용할 수 없습니다.
이 메소드는 우리가 만든 점과 원점 (0.0, 0.0) 간의 거리를 측정하며
부동 소수점 타입에서만 사용 가능한 수학적 연산을 이용합니다.

구조체 정의에서 사용한 제네릭 타입 매개변수와, 구조체의 메소드 시그니처 내에서
사용하는 제네릭 타입 매개변수가 항상 같은 것은 아닙니다.
Listing 10-11은 Listing 10-8 코드의 `Point<T, U>` 구조체에 `mixup` 이라는 메소드를 정의합니다.
이 메소드는 또 다른 `Point`를 매개변수로 전달받는데,
이는 우리가 `mixup`을 호출하는 `self` `Point` 와 다른 타입일 수도 있습니다.
이 메소드는 `self` `Point` 의 `T` 타입 `x` 값과, 매개변수로 넘겨받은
`Point` 의 `W` 타입 `y` 값으로 새로운 `Point` 인스턴스를 생성합니다.

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

<span class="caption">Listing 10-11: 구조체 정의와 다른 제네릭 타입을
사용하는 메소드</span>

`main` 에서, 우린 `i32` 타입 `x` (`5`)와
`f64` 타입 `y` (`10.4`)를 갖는 `Point` 를 정의했습니다.
`p2` 는 문자열 슬라이스 타입 `x` (`"Hello"`)와 `char` 타입 `y` (`c`)를 갖는 `Point` 입니다.
`p3` 는 `p1` 상에서 `p2` 를 인자로 `mixup` 메소드를 호출해 반환된 값입니다.
`p3` 의 `x` 는 `p1` 에서 온 `i32` 타입이며,
`y` 는 `p2` 에서 온 `char` 타입입니다.
`println!` 매크로는 `p3.x = 5, p3.y = c`를 출력합니다.

이 예제는 일부 제네릭 매개변수는 `impl` 에 선언되고
일부는 메소드 정의에 선언되는 경우를 보여주기 위한 예제입니다.
제네릭 매개변수 `T`, `U` 는 구조체 정의와 한 묶음이니 `impl` 뒤에 선언해야 하지만,
제네릭 매개변수 `V`, `W` 는 `mixup` 메소드에만 연관되어 있으므로
`fn mixup` 뒤에만 선언합니다.

### 제네릭 코드의 성능

여러분은 제네릭 타입 매개변수를 사용하면 런타임 비용이 발생하는지
궁금해하고 있을는지도 모르겠습니다.
좋은 소식은, 러스트는 제네릭을 사용한 코드가 구체적인 타입을 사용했을 때와
비교해서 전혀 느려지지 않도록 구현한다는 것입니다.

러스트는 컴파일 타임에
제네릭을 사용하는 코드를 *단형성화(monomorphization)* 합니다.
단형성화란 제네릭 코드를, 실제로 채워질 구체적인 타입으로 된 특정 코드로
바꾸는 과정을 말합니다.

이 과정에서, 컴파일러는 우리가 Listing 10-5에서
제네릭 함수를 만들 때 거친 과정을 정반대로 수행합니다.
컴파일러는 제네릭 코드가 호출된 곳을 전부 찾고, 제네릭 코드가
호출할 때 사용된 구체적인 타입으로 코드를 생성합니다.

표준 라이브러리의 `Option` 열거형을 사용하는
예제를 통해 알아봅시다:

```rust
let integer = Some(5);
let float = Some(5.0);
```

러스트는 이 코드를 컴파일할 때 단형성화를 수행합니다.
이 과정 중 컴파일러는 `Option<T>` 인스턴스에 사용된 값을 읽고,
`i32`, `f64` 두 종류의 `Option<T>` 가 있다는 것을 인지합니다.
그리고 제네릭 정의를 명시적인 것으로 대체하여
제네릭 `Option<T>` 정의를 `Option_i32`, `Option_f64`로
확장합니다.

단형성화된 코드는 다음과 같습니다.
제네릭 `Option<T>` 는 컴파일러에 의해 생성된 명시적인 정의로 변경되었습니다:

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

러스트 컴파일러가 제네릭 코드를 각 인스턴스의
명시적인 타입으로 변경해주는 덕분에,
굳이 런타임 비용을 줄이기 위해 우리 손으로 직접
각 타입마다 중복된 코드를 작성할 필요가 없습니다.
단형성화 과정은 러스트 제네릭을 런타임에 극도로 효율적이도록 만들어줍니다.

[traits-as-parameters]: ch10-02-traits.html#%EB%A7%A4%EA%B0%9C%EB%B3%80%EC%88%98%EB%A1%9C%EC%84%9C%EC%9D%98-%ED%8A%B8%EB%A0%88%EC%9E%87
