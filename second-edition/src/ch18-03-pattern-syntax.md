## 패턴 문법의 모든 것

여러분은 이 책을 읽는 내내 수많은 종류의 패턴 예시를 보셨을 겁니다.
이번 절에선 유효한 패턴 구문을 모두 살펴보고,
그것들을 각각 왜 사용해야 하는지 알아보도록 하겠습니다.

### 리터럴 매칭

6장에서 보신 것 처럼 여러분은 패턴과 리터럴을 직접 매칭할 수 있습니다.
다음 코드가 예시입니다.

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

`x` 의 값이 1 이기 때문에 이 코드는 `one` 을 출력합니다.
이 구문은 특정한 구체적인 값을 가질때 행동하도록
여러분의 코드를 작성하는데 도움이 됩니다.

### 명명 변수 매칭

명명 변수는 어떠한 값에도 매칭되는 반증 불가능한 패턴이며,
우린 이걸 이 책에서 여러번 써왔습니다. 어찌됐건, 여러분이 `match`
표현에서 명명 변수를 사용할때 문제가 있습니다.
바로 `match` 는 새로운 스코프를 만들기 때문에 `match` 표현 내에서
패턴의 일부로서 선언된 변수는 `match` 구조 외부의 동일한 이름을 가진
변수를 가려버린다는 겁니다.
Listing 18-11 에서 `Some(5)` 값으로 `x` 변수를 선언하고, `10` 값으로
`y` 를 선언했습니다. 한번 코드를 실행하거나 뒷부분의 설명을 읽지 않고 매칭
갈래 내의 패턴과 마지막의 `println!` 을 보고 이 코드가 뭘 출력할지
맞춰보세요:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

<span class="caption">Listing 18-11: 갈래에서 `y` 변수를 새로 만들어
기존의 것이 가려지도록 한 `match` 표현</span>

`match` 표현이 실행되었을때 어떤 일이 일어나는지 살펴봅시다.
일단 첫번째 갈래는 정의된 변수 `x` 와 매칭되지 않으니,
해당 코드는 실행되지 않고 넘어갑니다.

두번째 갈래 패턴에서는 `Some` 값 안에 있는 값에 대응될 새 변수 `y` 가
등장합니다. 현재 우린 `match` 표현 내의 새로운 스코프에 위치해 있기
때문에, 이 `y` 는 처음에 10 의 값을 갖도록 선언한 것이 아니라 새로운
변수입니다. 이 새로운 `y` 는 우리가 `x` 로 가지고 있는 `Some` 내부의
어떠한 값으로 바인딩될 것입니다. 따라서, 이 새 `y` 는 `x` 의 `Some` 내부
값인 `5` 로 바인딩 되고, 해당 갈래의 표현이 실행되어 `Matched, y = 5` 가
출력됩니다.

만약 `x` 가 `Some(5)` 이 아니라 `None` 값을 갖고 있었다면 첫번째와 두번째
갈래는 매치되지 않고 언더스코어와 매칭되었을 겁니다.
언더스코어 갈래에선 `x` 변수를 새로 만들지 않았기에 `x` 는
가려지지 않은 상태로 여전히 바깥의 `x` 변수를 나타내고,
만약 코드를 실행한다면 `match` 는
`Default case, x = None` 을 출력 할 겁니다.

`match` 표현이 끝나면 안쪽의 `y` 를 갖던 스코프도 끝납니다. 그리고
마지막 `println!` 은 `at the end: x = Some(5), y = 10` 를 출력합니다.

기존 변수를 가리는 변수를 만들지 않고 외부의 `x` 와 `y` 의 값을 비교하는
`match` 표현을 만들기 위해선 조건부(conditional) 매치 가드(match gaurd) 를
사용해야 하는데, 매치 가드에 대해선 이후 “매치 가드를 이용한 추가 조건”
절에서 다루도록 하겠습니다.

### 다중 패턴

여러분은 `match` 표현 내에서 *or* 을 뜻하는 `|` 구문을 이용해 여러개의 패턴과
매치시킬 수 있습니다. 예를 들어, 다음 코드는 `x` 값을 매치 갈래와 매치시키는데,
첫번째 갈래에서 *or* 옵션을 사용하고 있습니다. 이럴 경우 해당 갈래 내의 값 중
일치하는 값이 있으면, 즉 `x` 가 `1` 이나 `2` 일 경우 해당 갈래의 코드가
실행됩니다:

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

따라서 코드는 `one or two` 를 출력합니다.

### `...` 를 이용한 값의 범위 매칭

우린 `...` 구문을 이용해 값의 범위 내에 매치시킬 수 있습니다.
다음 코드에선 패턴이 범위 내 값에 매칭될 경우
해당 갈래가 실행됩니다.

```rust
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```

만약 `x` 가 1, 2, 3, 4, 5 중 하나라면 첫번째 갈래와 매치됩니다.
우리가 같은 코드를 앞서 설명한 `|` 이용해 작성했다면
`1 | 2 | 3 | 4 | 5` 라고 써야 했겠지만, 이 구문을 이용하면
`1 ... 5` 로 더 간편하게 작성할 수 있습니다. 만약 우리가 1 에서 1,000 까지의
숫자중 아무 숫자나 매치시켜야 하는 상황이라면 이는 훨씬 더 짧고, 유용할 겁니다.

이 값의 범위를 이용한 매칭 방식은 숫자 값이나 `char` 값에만 사용할 수 있습니다.
컴파일러가 컴파일 타임에 해당 범위가 비어있지 않은지 검사하는데,
러스트가 해당 범위가 비어있는지 알아낼 수 있는 타입의 종류가 정수 값과 `char` 뿐이기 때문입니다.

`char` 값의 범위를 이용하는 예시는 여기 있습니다:

```rust
let x = 'c';

match x {
    'a' ... 'j' => println!("early ASCII letter"),
    'k' ... 'z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

러스트는 `c` 가 첫번째 패턴(앞쪽 순서의 알파벳들)의 범위 안에 속한다는 것을
알아낼 수 있고, `early ASCII letter` 을 출력합니다.

### 값을 해체하여 분리하기

우린 패턴을 이용해 구조체, 열거체, 튜플, 참조자 등의 값들을
해체(destructuring)할 수도 있습니다. 각각에 대해 알아봅시다.

#### 구조체 해체하기

Listing 18-12 는 `x` 와 `y` 두개의 필드를 가진 `Point` 구조체를 나타냅니다.
이는 `let` 구문과 패턴을 이용해 이를 해체해봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

<span class="caption">Listing 18-12: 구조체의 필드를
여러 분리된 변수로 해체하기</span>

이 코드는 `p` 변수의 필드인 `x` 와 `y` 에 각각 대응되는 `a` 와 `b` 변수를
생성합니다. 이 예시를 보면 패턴 내 변수의 이름이 꼭 구조체의 필드명과 일치할
필요는 없다는 것을 알 수 있습니다.
하지만 해당 변수가 어떤 필드를 나타내는지 기억하기 쉽도록
필드명과 일치하도록 작성하는게 일반적입니다.

다만 일치하도록 작성할 때 `let Point { x: x, y: y } = p;` 는
`x` 와 `y` 이 중복됩니다. 여러분은 이때,
즉 패턴이 구조체 필드명과 일치할때 약칭 구문을 사용할 수 있습니다:
여러분은 구조체 필드명을 나열하는 것만으로
해당 필드의 값을 가진 변수를 만들어낼 수 있습니다.
Listing 18-13 은 Listing 18-12 의 코드를 `let` 패턴내의 `a` 와 `b` 대신
`x` 와 `y` 변수를 사용하도록 변경한 예시입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

<span class="caption">Listing 18-13: 약칭 구문을 이용해
구조체 필드 해체하기</span>

이 코드는 `p` 변수의 `x` 와 `y` 필드에
대응되는 `x` 와 `y` 변수를 만들어냅니다.
결과는 `x` 와 `y` 변수가 `p` 구조체 내의 값을 갖는 것으로 나옵니다.

또한, 모든 필드에 대응하는 변수를 만들지 않고 구조체 패턴의 일부에
리터럴 값을 이용해 해체할 수도 있습니다. 이렇게 함으로써
어떤 필드가 특정 값에 해당하는지를 검사하면서
나머지 필드를 해체한 변수를 만들 수 있습니다.

Listing 18-14 는 `Poin` 값을 3가지 경우로 나눈 `match` 표현을 나타냅니다:
`x` 축 위의 점(`y = 0` 일때 참) 인 경우, `y` 축 위인 경우(`x = 0`),
혹은 그 외일 경우:

<span class="filename">Filename: src/main.rs</span>

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

<span class="caption">Listing 18-14: 한 패턴 내에서
해체와 리터럴 값 매칭</span>

첫 번째 갈래는 `x` 축 위의 점인 경우에 해당하기에, `y` 필드가 `0` 값과
매치될 경우에 매치될 수 있도록 하였습니다. 또한 패턴은 여전히 `x` 변수를
생성하기 때문에 해당 변수를 갈래 내 코드에 사용할 수 있습니다.

비슷하게, 두 번째 갈래는 `x` 필드를 `0` 값과 매치시킬 경우에 매치되도록 하여
`y` 축 위의 점인지 판별합니다. 마찬가지로 `y` 필드에 해당하는 `y` 변수도
생성됩니다. 세 번째 갈래는 어떤 리터럴도 특정하지 않습니다. 따라서 모든
`Point` 에 매치되고 모든 `x` 와 `y` 필드에 대한 변수를 생성합니다.

이 예제에서 `p` 값은 `x` 가 `0` 이기 떄문에 두 번째 갈래에 매치됩니다.
따라서 이 코드는 `On the y axis at 7` 를 출력합니다.

#### 열거형 해체

우린 이미 이 책 6장의 Listing 6-5 에서 열거형을 해체해 봤습니다.
다만 한 가지 다루지 않은 내용이 있는데, 열거형을 해체하기 위한 패턴은
해당 열거형에 내장된 데이터의 정의 방식과 일치해야 합니다.
예시를 들기 위해, Listing 18-15 에 Listing 6-2 에서 사용했던
`Message` 열거형을 이용하고 각 내부 값을 해체하기 위한
`match` 패턴을 작성했습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}
```

<span class="caption">Listing 18-15: 다른 종류의 값들을 갖는
열거형 variant 해체</span>

이 코드는 `Change the color to red 0, green 160, and blue 255` 를 출력합니다.
한번 `msg` 값을 이리저리 변경해보고 다른 갈래들을 실행시켜보세요.

아무 값도 갖지 않는 `Message::Quit` 같은 variant 는 값을 해체할 수 없습니다.
따라서 리터럴 `Message::Quit` 값만 매치시킬 수 있으며,
패턴 내의 변수는 없습니다.

`Message::Move` 등의 유사 구조체 variant 의 경우는 우리가 구조체를
매칭시킬때의 패턴과 유사한 패턴을 사용합니다.
variant 명 뒤에 중괄호를 작성하고, 해당 갈래에서
사용할 변수를 나타내는 필드들을 나열합니다.
이땐 Listing 18-13 에서 사용했던 약칭 구문을 사용했습니다.

하나의 요소를 가진 `Message::Write` 튜플과 세 개의 요소를 가진
`Message::ChangeColor` 튜플 등의 유사 튜플 variant 에 사용하는
패턴은 튜플을 매치시킬때 지정한 패턴과 유사합니다.
이때 패턴 내 변수의 개수는 우리가 매칭하려는 variant 내 요소의
개수와 일치해야합니다.

#### 참조자 해체

우리가 패턴과 매칭하려는 값이 참조자를 포함하고 있을 땐
패턴 내에서 `&` 를 사용해 값으로부터 참조자를 해체해야 합니다.
이렇게 하면 참조자를 갖는 변수를 가져오는 대신
참조자가 가리키는 값을 갖는 변수를 가져올 수 있습니다.
이는 참조자들을 반복하는 반복자에서 클로저를 사용할 때,
해당 클로저 내에서 참조자가 아닌 참조자가 가리키는 값을
사용하길 원할 경우에 유용합니다.

Listing 18-16 의 예시는 `vector` 내의 `Point` 객체들을 가리키는
참조자들을 반복하며 구조체 참조자를 해체하는 것으로
간단하게 `x` 와 `y` 값을 계산할 수 있습니다:

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
let points = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 5 },
    Point { x: 10, y: -3 },
];

let sum_of_squares: i32 = points
    .iter()
    .map(|&Point { x, y }| x * x + y * y)
    .sum();
```

<span class="caption">Listing 18-16: 구조체 참조자를
구조체 필드 값들로 해체하기</span>

이 코드의 `sum_of_squares` 변수는 135 의 값을 갖습니다.
이 값은 `points` 벡터 내 `Point` 각각의
`x` 와 `y` 값을 제곱하고,
더한 값들을 모두 합친 값입니다.

만약 `&Point { x, y }` 에서 `&` 를 뺀다면 타입 불일치(type mismatch) 오류가
발생합니다. `iter` 는 벡터 내 요소들의 실제 값이 아닌 참조자이기 때문입니다.
실제 오류는 다음과 같습니다:

```text
error[E0308]: mismatched types
  -->
   |
14 |         .map(|Point { x, y }| x * x + y * y)
   |               ^^^^^^^^^^^^ expected &Point, found struct `Point`
   |
   = note: expected type `&Point`
              found type `Point`
```

이 오류는 러스트에선 클로저에서 `&Point` 를 사용하길 원했지만, 우리가 `Point`
참조자가 가리키는 값이 아니라 `Point` 값을 직접 매치시키려고 시도했기 때문입니다.

#### 구조체와 튜플 해체

우린 패턴 해체를 더 복잡한 방법으로 섞고, 비교하고, 중첩시킬 수 있습니다.
다음 예제는 튜플 내에 구조체와 튜플을 갖는, 즉 중첩된 구조에서 본래 값을
얻는 복잡한 해체를 보여줍니다:

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

이 코드는 우리가 복잡한 타입의 컴포넌트를 분리하고
각각의 값을 사용할 수 있게 합니다.

패턴을 이용한 해체는 구조체의 각 필드값과 같은 일부의 값을 서로 별도로
사용하는 편리한 방법입니다.

### 패턴 내에서 값 무시하기

여러분은 패턴 내에서 값들을 무시하는 게 유용한 것을 종종 보셨을 겁니다.
예를 들면, `match` 의 마지막 갈래에 될 수 있지만 아무것도 하지 않는
모든 나머지 값들을 매칭시킬 때요. 전체 혹은 일부 값을 무시하는 방법은
몇 가지 있습니다: 여러분이 여태 보신 것 처럼 `_` 패턴을 이용하거나,
다른 패턴 내에서 `_` 패턴을 사용하거나, 언더스코어( _ ) 로 시작하는
이름을 사용하거나, 값의 나머지 부분을 무시하기 위해 `..` 를 사용하는 것이죠.
한번 이들에 대해서 각각 어떻게 사용하고, 왜 사용하는지 알아봅시다.

#### `_` 를 이용해 전체 값 무시하기

`_` 언더스코어 와일드카드 패턴은 어떤 값과도 매치되지만
값으로 바인드(bind) 하지는 않는 것으로 사용되어 왔습니다.`_` 언더스코어 패턴은
`match` 의 마지막 갈래 표현에서 특히 유용하지만, Listing 18-17 처럼
함수 매개변수를 포함한 모든 패턴에서 사용할 수 있습니다.

<span class="filename">Filename: src/main.rs</span>

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

<span class="caption">Listing 18-17: 함수 시그니처에서 `_` 사용하기</span>

이 코드는 첫번째 인자로 전달된 값인 `3` 을 완벽히 무시하고,
`This code only uses the y parameter: 4` 를 출력합니다.

대부분의 경우 특정 함수의 매개변수가 더 이상 필요하지 않다면, 해당 함수의
시그니처를 더 이상 사용되지 않는 매개변수가 포함되지 않도록 변경해야 합니다.
다만 몇몇 경우에는 함수의 매개변수를 무시하는 것이 유용할 때도 있습니다:
예를 들어, 여러분이 트레잇을 구현할때 특정 타입의 시그니처가 필요한데
함수 본문에선 매개변수중 하나가 필요하지 않은 경우입니다.
컴파일러는 이때 사용되지 않은 매개 변수에 관해서 경고하지 않습니다.
단, 언더스코어가 아닌 이름을 사용할 경우엔 경고합니다.

#### 중첩된 `_` 를 이용해 값의 일부 무시하기

`_` 를 다른 패턴 내에 사용해서 값의 일부를 무시할 수도 있습니다:
값의 일부를 테스트 하려는데 해당 코드에서 그 외의 나머지 부분은
필요하지 않을때 이 기능을 사용할 수 있습니다.
Listing 18-18 은 설정 값을 관리하는 코드를 보여줍니다.
비즈니스 요구 사항은 사용자가 기존 커스텀을 덮어쓰진 않아야 하지만
기존 설정을 해제할 순 있으며 해제된 상태에선 값을 설정하는 것이
가능해야 한다는 것입니다.

```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);
```

<span class="caption">Listing 18-18: `Some` 내부 값이 필요하지 않은
상황에서 패턴 내에 언더스코어를 사용해 `Some` varaint 와
매치시키기</span>

이 코드는 `Can't overwrite an existing customized value` 와
`setting is Some(5)` 를 출력합니다. 우린 첫 번째 매치 갈래에서
두 `Some` variant 모두 내부 값을 매치시키거나 사용할 필요가 없고,
그저 `setting_value` 와 `new_setting_value` 가 모두 `Some` variant 인지만
확인하면 됩니다. 조건을 만족하면, 왜 `setting_value` 를 변경하지 않는지
이유를 출력하고 값을 변경하지 않습니다.

나머지 모든 경우는 (`setting_value` 나 `new_setting_value` 둘 중 하나가
`None` 인 경우) 두 번째 갈래에서 `_` 로 표현되고, `setting_value` 는
`new_setting_value` 로 변경됩니다.

우린 특정 값들을 무시하기 위해 한 패턴 내에서 언더스코어를 여러번
사용할 수도 있습니다. Listing 18-19 는 5개의 요소를 가진 튜플에서
두 번째와 네 번째 요소만 무시하는 예제입니다:

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

<span class="caption">Listing 18-19: 튜플의 여러 부분 무시하기</span>

이 코드는 `Some numbers: 2, 8, 32` 를 출력하고,
4 와 16 은 무시됩니다.

#### 언더스코어로 시작하는 이름을 이용해 쓰이지 않는 변수 무시하기

만약 변수를 생성했는데 아무 곳에서도 사용하지 않는다면, 보통 러스트는
이를 버그가 될 수 있다고 경고합니다. 하지만 프로토타입을 만드는 중이거나
프로젝트를 막 시작했을 때와 같이 아직 사용하진 않아도 미리 변수를 만들어
두는 것이 유용할 때도 있습니다. 이럴 경우 해당 변수명을 언더스코어로
시작하도록 만들면 러스트는 해당 미사용 변수에 대해 경고를 생성하지 않습니다.
Listing 18-20 에선 두 개의 미사용 변수를 생성하지만 이 코드를 실행할 땐
하나의 경고만 나타납니다.

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

<span class="caption">Listing 18-20: 미사용 변수 경고를 피하기 위해
변수명을 언더스코어로 시작하도록 하기</span>

`y` 변수를 사용하지 않았다는 경고가 나타나지만,
언더스코어로 시작하는 `x` 변수는 경고가 나타나지 않습니다.

`_` 하나만 쓰는것과 언더스코어로 시작하는 변수명의 차이점을 알아두세요.
`_` 는 어떠한 값도 바인드되지 않지만, `_x` 는 여전히 값이 바인드됩니다.
Listing 18-21 은 이 미묘한 차이가 중요한 포인트가 되는 좋은 예시입니다.
이 코드는 에러가 발생합니다:

```rust,ignore
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);
```

<span class="caption">Listing 18-21: 언더스코어로 시작하는 변수는
여전히 값이 바인드되기 때문에 해당 값의 소유권을 가져갑니다</span>

`s` 값이 `_s`로 이동되었기 때문에, `s` 를 다시 사용할 수 없다는 오류가 발생합니다.
반면 언더스코어만 사용하는 경우는 값이 바인드되지 않습니다.
따라서 `s` 는 `_` 로 이동하지 않고, Listing 18-22 는
컴파일 시 어떤 에러도 발생하지 않습니다:

```rust
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
```

<span class="caption">Listing 18-22: 언더스코어를 사용하면
값이 바인드되지 않습니다</span>

우린 `s` 를 어느 것에도 바인드하지 않았습니다; 따라서 `s` 는 이동하지 않고, 이 코드는 잘 작동합니다.

#### `..` 를 이용해 값의 나머지 부분 무시하기

여러 요소를 갖는 값을 다룰 때, 값의 일부만 사용하고 나머지는
무시하기 위해 `..` 구문을 사용할 수 있습니다. 이 구문은 무시할
각 값에 언더스코어를 하나하나 작성하는 끔찍한 사고를 막아주기도 합니다.
`..` 패턴은 우리가 패턴에서 명시하지 않은 값의 나머지 부분을 모두
무시합니다. Listing 18-23 은 3차원 공간의 좌표를 갖는 `Point` 구조체를
갖습니다. 이 `match` 표현에서는 `x` 좌표만 사용하고 `y` 와 `z` 필드의 값은
무시합니다:

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

<span class="caption">Listing 18-23: `..` 를 사용해
`Point` 의 `x` 필드 외 모든 필드 무시하기</span>

`x` 를 나열하고, `..` 패턴을 포함했습니다.
이는 `y: _` 와 `z: _` 를 나열하는 것 보다 간결하고,
이보다 더 많은 필드를 갖는 구조체에서 한 두개의 필드만 필요할
상황에선 훨씬 더 간결합니다.

`..` 구문은 필요한 만큼 많은 값으로 확대될 수 있습니다.
Listing 18-24 는 `..` 를 튜플과 사용하는 법을 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}
```

<span class="caption">Listing 18-24: 튜플의 첫 번째와 마지막 값만 매칭시키고
나머지 값은 모두 무시하기</span>

이 코드에선 첫 번째와 마지막 값이 `first` 와 `last` 에 매치되고,
`..` 는 중간의 모든 값과 매치됩니다.

다만 `..` 를 사용할 땐 모호하지 않아야(unambiguous) 합니다. 만약 어떤 값이
매치되고 어떤 값이 무시되어야 하는지 명확하지 않다면 러스트는 에러를 발생시킵니다.
Listing 18-25 는 `..` 를 모호하게 사용해 컴파일 되지 않는 경우의 예시를 보여줍니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```

<span class="caption">Listing 18-25: `..` 를 모호하게
사용해보기</span>

이 예시를 컴파일하면 다음과 같은 에러가 나타납니다:

```text
error: `..` can only be used once per tuple or tuple struct pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |                      ^^
```

러스트는 무시할 튜플 요소 중 몇개를 `second` 전에 두고,
후에 몇개를 둘 지 결정할 수 없습니다. 이 코드는 `2` 를 무시하고
`4` 를 바인드한 뒤 `8`, `16`, `32` 를 무시하거나; `2` 와 `4` 를 무시하고
`8` 을 바인드한 뒤 `16` 과 `32` 를 무시하는 등을 의미할 수 있습니다.
러스트에서 `second` 변수명은 그 어떤 특별한 의미도 없고,
이렇게 두 곳에 `..` 를 사용하는 것은 모호하므로
우리는 컴파일러 에러를 받습니다.

### `ref` 와 `ref mut` 를 이용해 패턴 내에서 참조자 생성하기

`ref` 를 사용해 참조자를 만들어서 패턴 내 변수로
값의 소유권이 이동하지 않도록 하는 법을 알아봅시다.
보통 패턴과 매치시킬 경우 패턴에 나타난 변수에 값이 바인드됩니다.
러스트의 소유권 규칙에 따르면 값은 `match` 내부 혹은
여러분이 패턴을 사용하는 모든 곳으로 이동됩니다.
Listing 18-26 은 `match` 의 패턴에서 변수로 받고,
값을 `match` 이후 `println!` 구문에서 사용하는 예시입니다.
이 코드에서는 `robot_name` 값의 소유권이 `match` 의 첫 번째 갈래에서
`name` 변수로 이전되었기 때문에 컴파일 시 오류가 발생합니다:

```rust,ignore
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">Listing 18-26: `match` 갈래의 패턴에서
값의 소유권을 갖는 변수 생성하기</span>

`robot_name` 의 소유권이 `name` 으로 이동했기 때문에
`robot_name` 은 더 이상 소유권을 갖지 않고, 따라서 `match` 이후
`println!` 에서 `robot_name` 을 사용할 수 없습니다.

이 코드를 고치기 위해선 `Some(name)` 에서 `robot_name` 의 소유권을 가져가는
것이 아닌 *빌려야(borrow)* 합니다. 패턴을 벗어나서, 값을 빌리는 방법은 `&` 를
이용해 참조자를 생성하는 것이라고 우린 배웠습니다. 따라서 여러분은 `Some(name)`
을 `Some(&name)` 으로 변경하는 것이 해결책이라 생각할 것 입니다.

하지만 여러분이 "값을 해체하여 분리하기" 절에서 보신 것 처럼, 패턴 내에서의
`&` 구문은 참조자를 *생성* 하는 것이 아닌, 이미 존재하는 참조자를 값으로
*매치* 합니다. `&` 는 이미 패턴 내에서 다른 뜻을 갖기 때문에, 우린 패턴 내에서
참조자를 생성하기 위해 `&` 를 사용할 수 없습니다.

대신 우린 Listing 18-27 에 나오는 것 처럼 새 변수 앞에 `ref` 키워드를 사용해
패턴 내에서 참조자를 생성할 수 있습니다:

```rust
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">Listing 18-27: 패턴 변수가 값의 소유권을 갖지 않도록
참조자 생성하기</span>

`robot_name` 의 `Some` 내 variant 값이 `match` 로 이동하지 않기 때문에
이 예제는 정상적으로 컴파일 됩니다; `match` 는 `robot_name` 의 데이터를
이동시키는 대신 참조자만 갖습니다.

매치된 패턴 내에서 값을 변경하기 위해 가변 참조자를 생성하려면
`&mut` 대신 `ref mut` 을 사용해야 합니다. 이유는 똑같이 패턴 내에서의
`&` 는 새 참조자를 생성하는 것이 아닌 이미 존재하는 가변 참조자를
매치시키는데 사용되기 때문입니다. Listing 18-28 은 가변 참조자를
생성하는 패턴의 에시를 보여줍니다:

```rust
let mut robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref mut name) => *name = String::from("Another name"),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">Listing 18-28: `ref mut` 를 이용해 패턴 내에서
가변 참조자 생성하기</span>

이 예제는 문제 없이 컴파일되고 `robot_name is: Some("Another name")` 를
출력합니다. `name` 은 가변 참조자이기 때문에 매치 갈래 코드 내에서
값을 변경하기 위해선 `*` 연산자를 이용해 역참조해야 합니다.

### 매치 가드를 이용한 추가 조건

*매치 가드(match guard)* 는 `match` 갈래 뒤에 추가로 붙는 `if` 조건으로,
이것이 있을 경우 패턴 매칭과 해당 조건이 모두 만족되어야 해당 갈래가 선택됩니다.
매치 가드는 패턴만 사용하는 것 보다 복잡한 아이디어를 표현하는데
유용합니다.

조건은 패턴 내에서 생성된 변수를 사용할 수 있습니다.
Listing 18-29 에서 `match` 의 첫 번째 갈래가 `Some(x)` 패턴과
`if x < 5` 매치 가드를 갖는 것을 볼 수 있습니다:

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

<span class="caption">Listing 18-29: 패턴에 매치 가드 추가</span>

이 예제는 `less than five: 4` 를 출력합니다. `num` 이 첫 번째 갈래에서
비교될 때, `Some(4)` 는 `Some(x)` 에 매치되기 때문에 매치됩니다.
그리고 매치 가드는 `x` 가 `5` 보다 작은지 검사합니다.
이 경우 참이므로, 첫 번째 갈래가 선택됩니다.

`num` 이 `Some(10)` 이었다면, 10 은 5보다 크기 때문에 첫 번째 갈래의
매치 가드는 거짓이 됩니다. 러스트는 두 번째 갈래로 이동하고, 두 번째 갈래는
매치 가드를 갖지 않으니 모든 `Some` variant 에 매치됩니다.
따라서 두 번째 갈래와 매치됩니다.

`if x < 5` 조건문을 패턴 내부에서 표현할 방법은 없지만,
매치 가드는 우리에게 이 로직을 표현할 수 있는 능력을 부여해 줍니다.

Listing 18-11 에서 매치 가드를 이용해
패턴 가림 문제를 해결할 수 있다고 말했었습니다.
`match` 표현의 패턴 내에서 새로 생성한 변수가 기존에 있던
`match` 바깥의 변수를 가려버려서 기존의 변수를 테스트 할 수 없다는 문제였죠.
Listing 18-30 은 매치 가드를 사용해 이 문제를
해결하는 방법을 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

<span class="caption">Listing 18-30: 외부 변수와 같은지 테스트하기 위해
매치 가드를 사용</span>

현재 이 코드는 `Default case, x = Some(5)` 를 출력합니다.
두 번째 매치 갈래는 새 변수 `y` 를 생성하지 않으며, 바깥의 `y` 를 가리지
않습니다. 즉, 우린 매치 가드에서 바깥의 `y` 를 사용할 수 있습니다.
그리고 바깥의 `y` 를 가리게 될 `Some(y)` 패턴을 지정하는
대신 `Some(n)` 을 사용했습니다. 이는 새 변수 `n` 를 생성하지만,
`match` 밖에 `n` 변수는 존재하지 않기 때문에 아무것도 가리지 않습니다.

`if n == y` 매치 가드는 패턴이 아니므로 새 변수를 생성하지 않습니다.
여기서의 `y` 는 바깥의 `y` 를 가린 새 변수가 *아닌* 바깥의 `y` 입니다.
그리고 `n` 을 바깥의 `y` 와 비교하여 같은 값인지
판별합니다.

여러분은 *or* 연산자인 `|` 를 사용해 다중 패턴을 지정한 것에도 매치 가드를
사용할 수 있습니다. 이때 매치 가드 조건은 모든 패턴에 적용됩니다.
Listing 18-31 은 매치 가드 앞에 `|` 를 이용한 다중 패턴을 결합한 모습을 보여줍니다.
여기서 중요한 점은 `if y` 매치 가드가 `6` 에만 적용되는 것 처럼 보여도
사실은 `4`, `5`, `6` 모두에 적용된다는
것입니다:

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

<span class="caption">Listing 18-31: 다중 패턴과
매치 가드의 결합</span>

갈래의 매치 조건 상태는 `x` 가 `4`, `5`, `6` 중 하나이고
`y` 가 `true` 여야 합니다. 이 코드를 실행하면, `x` 가 `4` 이기에
첫 번째 갈래의 패턴에 매치되지만, `if y` 매치 가드가 거짓이 되기 때문에
첫 번째 갈래는 선택되지 않습니다. 코드는 두 번째 갈래로 이동하고 매치되며,
프로그램은 `no` 를 출력합니다. 이렇게 되는 이유는 `if` 조건이 마지막 값인
`6` 에만 적용되는 것이 아닌 `4 | 5 | 6` 패턴 전체에 적용되기 때문입니다.
즉, 매치 가드와 앞의 패턴과의 관계는
다음과 같습니다:

```text
(4 | 5 | 6) if y => ...
```

다음은 틀린 관계입니다:

```text
4 | 5 | (6 if y) => ...
```

이 코드를 실행하고 나면 전자가 맞다는 것이 명확해집니다:
만약 매치 가드가 `|` 로 연결한 값 리스트의 마지막 값에만 적용되었다면,
해당 갈래는 매치되고 프로그램은
`yes` 를 출력했을 것입니다.

### `@` 바인딩

*at* 연산자인 `@` 는 해당 값이 패턴과 매치되는지 확인하는 동시에
해당 값을 갖는 변수를 생성할 수 있도록 해줍니다. Listing 18-32 는
`Message::Hello` 의 `id` 필드가 `3...7` 범위 내에 있는지 테스트하는
예시입니다. 하지만 우린 그 값을 `id_variable` 값에 바인드하고,
그 갈래의 코드에서 사용하길 원합니다. 우린 이 변수를
필드명과 똑같이 `id` 라는 이름으로 만들 수도 있지만,
이번 예제에선 다른 이름을 사용하겠습니다:

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3...7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10...12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

<span class="caption">Listing 18-32: 패턴 내에서 값을 테스트하는 동시에
`@` 를 이용해 값 바인드하기</span>

이 예제는 `Found an id in range: 5` 를 출력할 겁니다.
`3...7` 범위 앞에 `id_variable @` 를 지정하는 것으로 값이 범위 패턴과
매치되는지 테스트하면서 해당 값을 캡쳐할(capturing) 수 있습니다.

두 번째 갈래엔 범위 패턴만 존재합니다.
이 갈래의 코드에는 `id` 필드 값을 담는 변수가 없습니다.
`id` 필드의 값은 10, 11, 12 중 하나가 될 수 있지만,
코드는 값이 몇인지 알지 못하고,
`id` 값을 변수로 저장하지 않았기 때문에
`id` 필드의 값을 사용할 수도 없습니다.

마지막 갈래에선 범위를 명시하지 않았고,
`id` 라는 이름의 변수를 해당 갈래의 코드 안에서 사용할 수 있습니다.
이유는 구조체 필드 약칭 구문을 사용했기 때문입니다.
하지만 첫 두 갈래와는 달리 `id` 필드에 대해 어떠한 테스트도 적용하지 않았습니다:
모든 값은 이 패턴과 매치될 겁니다.

`@` 를 사용하면 값을 테스트하고 변수로 저장하는 일을 한 패턴 내에서 할 수 있습니다.

## 정리

러스트의 패턴은 다른 종류의 데이터를 구별하는데 굉장히 유용합니다. `match` 표현
내에서 패턴을 사용하면 러스트가 여러분의 패턴이 모든 가능한 값을 커버할 수 있다는
것을 보장합니다. 아닐 경우에는 여러분의 프로그램은 컴파일 되지 않을 것입니다.
`let` 구문과 함수 매개 변수에서의 패턴은 그들을 더 유용하게 구성할 수 있게 해주고,
변수의 할당과 동시에 작은 부분의 값들로 해체하는 것을 가능하게 해줍니다.
우린 우리의 필요에 맞게 간단하거나 복잡한 패턴을 만들 수 있습니다.

다음으로, 이 책의 끝에서 두번째 장에선 러스트의 고급 기능에 대해서
알아보도록 하겠습니다.
