## 제어문 

조건의 상태가 참인지에 따라 어떤 코드의 실행 여부를 결정하거나 조건이 만족되는 동안 반복 수행을 하는 것은
대부분의 프로그래밍 언어의 기초 문법입니다. 우리가 실행 흐름을 제어할 수 있는 가장 보편적인 작성 방식은
`if`표현식과 반복문 입니다.

### `if`표현식

`if`표현식은 우리의 코드가 조건에 따라 분기할 수 있게 합니다. 우리가 조건을 제공하는 것은 다음 서술과 같죠.
“만약 조건이 충족되면, 이 코드 블럭을 실행하세요. 만약 충족되지 않았다면 코드 블럭을 실행하지 마세요."

*branches*로 명명된 새 프로젝트를 우리의 *projects* 디렉토리에 생성하고 `if`식을 탐구합시다.
*src/main.rs* 파일에 다음의 내용을 기입하세요:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

모든 `if`표현식은 `if`란 키워드로 시작하며 뒤이어 조건이 옵니다. 이번 경우에 조건은 변수 `number`가 
5보다 작은 값을 가지는지 여부가 됩니다. 조건이 참일 때 실행하는 코드 블록은 조건 바로 뒤 중괄호로 된 블록에 
배치됩니다. `if`식의 조건과 관련된 코드 블럭은 우리가 2장의 “비밀번호 추리 게임”에서 다뤘던 `match`식의 
갈래(arms)와 마찬가지로 *갈래(arms)*로 불립니다. 선택적으로, 우리는 이번 경우에서 처럼 `else`식을 
포함시킬 수 있는데, 이는 조건이 거짓으로 산출될 경우 실행시킬 코드 블럭을 프로그램에 제공합니다. 당신이 
`else`식을 제공하지 않는데 조건이 거짓이 되면, 프로그램은 `if`블록을 생략하고 다음 순서의 코드를 실행하게 
될 겁니다.


이 코드를 실행해보세요; 다음과 같은 결과를 얻을 수 있을 겁니다:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was true
```

`number`의 값을 조건을 `거짓`으로 만들 값으로 변경하면 무슨 일이 일어날지 살펴보도록 합시다:

```rust,ignore
let number = 7;
```

프로그램을 다시 실행시키면, 다음과 같은 결과를 보게 됩니다:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was false
```

주의해야 할 중요한 점은 이번 코드의 조건은 *반드시* `bool`이어야 합니다. 만약 `bool`이 아닐 경우 
어떤 일이 일어나는지는 다음의 코드를 실행하면 알 수 있을 겁니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

`if`의 조건이 `3`으로 산출되고, Rust는 에러를 발생시킵니다.


```text
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected bool, found integral variable
  |
  = note: expected type `bool`
             found type `{integer}`
```

이 에러가 나타내는 것은 Rust가 `bool`을 기대하였으나 정수형이 왔다는 겁니다. Rust는 boolean 타입이 
아닌 것을 boolean 타입으로 자동 변환하지 않습니다. Ruby나 Javascript와는 다르죠. 우리는 반드시 
명시적으로 `boolean`을 `if`의 조건으로 사용해야 합니다. 만약 우리가 `if`표현식의 코드 블록을 숫자가 
`0`이 아닐 시에 실행하고 싶다면, 다음처럼, 우리는 `if`표현식을 변경할 수 있습니다. 

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

이번 코드를 실행시키면 `number was something other than zero`가 출력 될 겁니다.

#### `else if`와 다수 조건 

우리는 `if`와 `else` 사이에 `else if`식을 추가 결합하여 다양한 조건을 다룰 수 있습니다. 
예제를 보시죠:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

이번 프로그램은 분기할 수 있는 네 개의 경로를 갖습니다. 이를 수행하면, 다음과 같은 결과를 얻게 될 겁니다:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
number is divisible by 3
```

이 프로그램이 실행될 때, `if`식을 차례대로 검사하고 검사 조건이 참일 때의 첫 번째 본문을 실행합니다. 주목할 점은 
6을 2로 나누어 떨어짐에도 불구하고 `number is divisible by 2`이 출력되지 않는데, `else`의 
블럭에 위치한 `number is not divisible by 4, 3, or 2`도 마찬가지입니다. 이렇게 되는 이유는 
Rust가 첫 번째로 조건이 참이 되는 블록만 찾아 실행하고, 한번 찾게 되면 나머지는 검사하지 않기 때문입니다. 

너무 많은 `else if`식의 사용은 당신의 코드를 이해하기 어렵게 하므로, 둘 이상일 경우 코드를 리팩토링하게 
될 수도 있습니다. 이런 경우를 위해 6장에서 `match`라 불리는 강력한 분기 생성자를 다룹니다. 

#### `let`구문에서 `if` 사용하기 

`if`가 표현식이기 때문에, 항목 3-4에서 처럼, 우리는 이를 `let` 구문의 우측에 사용할 수 있죠. 

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

<span class="caption">Listing 3-4: Assigning the result of an `if` expression
to a variable</span>

변수 `number`에는 `if`식에서 산출된 값이 bound되게 됩니다. 어떤 일이 일어날지 코드를 실행해보죠:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
The value of number is: 5
```

기억하세요! 코드 블록은 그들의 마지막에 위치한 표현식을 산출하며 숫자는 그 자체로 표현식이라는 것을요. 이 경우 
전체 `if`식의 값은 실행되는 코드 블럭에 따라 다릅니다. 그렇기에 `if`식에 속한 각 갈래의 결과는 반드시 같은 
타입이여야 합니다. 항목 3-4에서 `if`갈래와 `else`갈래는 모두 `i32` 정수형을 결과 값으로 가집니다. 
하지만 만약 다음 예제처럼 유형이 다르면 어떻게 될까요?

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

우리가 이번 코드를 실행시키려고 하면 에러를 얻게 됩니다. `if`와 `else` 갈래의 값 타입이 호환되지 않고, 
Rust는 정확히 프로그램의 어느 지점에 문제가 있는지 보여줍니다. 

```text
error[E0308]: if and else have incompatible types
 --> src/main.rs:4:18
  |
4 |       let number = if condition {
  |  __________________^
5 | |         5
6 | |     } else {
7 | |         "six"
8 | |     };
  | |_____^ expected integral variable, found reference
  |
  = note: expected type `{integer}`
             found type `&str`
```

`if` 블록이 정수형을 산출하는 식이고 `else` 블록은 문자열을 산출하는 식 입니다. 이런 경우가 성립하지 않는 
이유는 변수가 가질 수 있는 타입이 오직 하나이기 때문입니다. Rust는 컴파일 시에 `number` 변수의 타입이 뭔지 
확실히! 정의해야 합니다. 그래야 `number`가 사용되는 모든 곳에서 유효한지 검증할 수 있으니까요. Rust는 
`number`의 타입을 실행 시에 정의되도록 할 수 없습니다. 컴파일러가 모든 변수의 다양한 타입을 추적해서 알아내야  
한다면 컴파일러는 보다 복잡해지고 보증할 수 있는 것은 적어지게 됩니다.

### 반복문과 반복

코드 블록을 한 번 이상 수행하는 것은 자주 유용합니다. 반복 작업을 위해서, Rust는 몇 가지 *반복문*을 제공합니다. 
반복문은 반복문 시작부터 끝까지 수행하고 다시 처음부터 수행합니다. 반복문의 실험해보기 위해 *loops*으로 명명된 
새 프로젝트를 작성해 봅시다.

Rust가 제공하는 세 가지 반복문: `loop`, `while`, 그리고 `for`을 모두 사용해 봅시다.


#### `loop`와 함께 코드의 반복 수행 

`loop` keyword는 Rust에게 그만두라고 명시하여 알려주기 전까지 코드 블럭을 반복 수행합니다. 
예제로, 우리의 *loops*디렉토리에 *src/main.rs*를 다음처럼 변경하세요:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    loop {
        println!("again!");
    }
}
```

이 프로그램을 실행시키면, 우리는 프로그램을 강제 정지하기 전까지 `again!`이 반복 출력되는 것을 보게 됩니다.
대부분의 터미널은 단축키 <span class="keystroke">ctrl-C</span>를 통해서 무한루프에 빠진 프로그램을
정지시키는 기능을 지원합니다. 한번 시도해 보세요:

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```


기호 `^C`는 우리가 <span class="keystroke">ctrl-C</span>를 눌렀을 때의 위치입니다. 코드가 정지
신호를 받은 시점에 따라 `^C` 이후에 `again!`이 출력될 수도 아닐 수도 있습니다. 

다행스럽게도, Rust는 보다 안정적으로 루프에서 벗어날 수 있는 방법을 제공합니다. 우리는 `break` keyword
를 위치시켜 프로그램이 언제 루프를 멈춰야 하는지 알려줄 수 있습니다. 상기시켜 드리자면 2장 “추리 게임”에서 
사용자가 모든 숫자를 정확히 추리했을 경우 프로그램을 종료시키기 위해 사용했었습니다. 

#### `while`와 함께하는 조건부 반복 

반복문 내에서 조건을 산출하는 것은 자주 유용합니다. 조건이 참인 동안 반복문을 수행합니다. 조건이 참이 아니게 된 경우에 
`break`을 호출하여 반복을 정지시킵니다. 이런 패턴의 반복문을 구현하자면 `loop`, `if`, `else`, 그리고
`break`를 혼합해야 합니다; 원한다면 이렇게 사용해도 됩니다.

하지만, 이런 패턴은 매우 보편적이기 때문에 이와 동일한 구조자가 Rust에는 내장되어 있으며, 이를 `while` 
반복문이라 부릅니다. 다음의 예제를 통해 `while`을 사용해 봅시다: 프로그램은 세 번 반복되고, 반복 때마다 
카운트 다운됩니다. 마침내 반복이 끝나면 다른 메시지를 출력하고 종료됩니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

이 구조자는 loop, if, else 및 break를 사용하는 경우 필요한 많은 중첩을 제거하며, 더 깔끔합니다.
조건이 true인 동안 코드가 실행되고; 그렇지 않으면 루프에서 벗어납니다.

#### `for`와 함께하는 콜렉션 반복하기

우리는 `while` 구조자를 통해 배열과 같은, 콜렉션의 각 요소에 걸쳐 반복 수행 할 수 있습니다.
예를 들어서, Listing 3-5을 살펴봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

<span class="caption">Listing 3-5: Looping through each element of a collection
using a `while` loop</span>

여기서, 코드는 배열의 요소에 걸쳐 카운트를 증가시킵니다. 이 색인은 `0`에서 시작하고, 배열의 마지막 순서까지 반복됩니다
(즉, `index < 5`가 참이 아닐 때까지). 이 코드를 수행하면 배열의 모든 요소가 출력되게 됩니다. 

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

예상했던 대로, 5개인 배열 모든 값이 터미널에 표시됩니다. `index` 값이 `5`에 오는 시점에, 
그러니까 배열의 6번째 값에 접근하기 전에 반복은 중지되어야 합니다. 

그러나 이런 방식은 에러가 발생하기 쉽습니다; 우리가 정확한 길이의 색인을 사용하지 못하면 프로그램은 패닉을 발생합니다. 
또한 느린데, 이유는 컴파일러가 실행 간에 반복문을 통해 반복될 때마다 요소에 대한 조건 검사를 수행하는 런타임 코드를 
추가하기 때문입니다.

보다 효율적은 대안으로, 우리는 `for` 반복문을 사용하여 콜렉션의 각 요소에 대한 코드를 수행할 수 있습니다.
`for` 반복문은 다음 Listing 3-6과 같습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

<span class="caption">Listing 3-6: Looping through each element of a collection
using a `for` loop</span>

우리가 이 코드를 수행하면, 항목 3-5와 같은 결과를 볼 수 있습니다. 더 중요한 것은, 우리는 이제 코드의 안전성을 높이고 
배열의 끝을 넘어가거나 충분한 길이를 지정하지 못해 일부 아이템이 누락되어 발생할 수있는 버그의 가능성을 제거했습니다.

예를 들어, 코드 3-5의 코드에서 a 배열 에서 항목을 제거 했지만 조건을 `while index < 4`로 업데이트하지 
않으면 코드는 패닉을 발생합니다. for루프를 사용하면, 당신이 배열의 수를 변경 한 경우에도 다른 코드를 변경해야 
할 필요가 없습니다. (역주 : 당신은 살면서 변경한 배열의 수를 기억하고 있는가?)

`for`반복문이 안전하고 간결하기 때문에 이들은 가장 보편적으로 사용되는 반복문 구조자입니다. 항목 3-5에서처럼 
`while`반복문을 사용하여 특정 횟수만큼 코드를 반복하려는 경우에도, 대부분의 Rust 사용자들은 `for`반복문
을 사용하고자 할 것 입니다. 이런 사용을 위해 Rust에서 기본 라이브러리로 제공하는 `Range`를 사용하게 됩니다.
`Range`는 한 숫자에서 다른 숫자 전까지 모든 숫자를 차례로 생성합니다. 

여기 `for`반복문과 아직 설명하지 않은 range를 역순하는 `rev`메소드를 사용하는 카운트다운 프로그램이 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

꽤 괜찮은 코드인것 같죠? 

## 결론 

해냈어요! 무지 긴 장이었어: 우리는 변수, 스칼라, `if`식과 반복문까지 배웠어요! 혹시 이번 장에서 나온 내용을
연습해보고 싶으면 다음을 수행하는 프로그램을 만들어 보세요.

* 화씨와 섭씨를 상호 변환.
* n번째 피보나치 수열 생성.
* 크리스마스 캐롤 “The Twelve Days of Christmas”의 가사를 반복문을 활용해 출력.

다음으로 넘어갈 준비가 되셨습니까? 우리는 이제 일반적인 다른 언어에는 존재하지 않는 개념에 대해서 다루고자 합니다
: 소유권.
