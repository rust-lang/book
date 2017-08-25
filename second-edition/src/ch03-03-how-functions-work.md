## 함수 동작 원리

함수는 Rust에 녹아들어있습니다. 여러분은 이미 언어에서 가장 중요하게 생각하는 `main`함수를 보셨습니다. 
이는 다수의 프로그램에서 실행 지점입니다. 여러분은 또한 `fn` 키워드도 보셨을텐데, 이는 새로운 함수의 선언을
가용하게 합니다.

Rust 코드는 *뱀 형태*를 변수나 함수 이름의 형식 규칙으로 사용합니다. 뱀 형태에서, 모든 문자는 소문자를 사용하며
밑줄 표시로 단어를 구분합니다. 다음은 예제로 함수를 선언하는 프로그램 입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Rust 에서의 함수 선언은 `fn`으로 시작하며 함수 이름 뒤에 괄호의 형식으로 되어 있습니다. 중괄호는 컴파일러에게
함수의 시작과 종료 지점을 알려주게 됩니다. 

우리는 함수의 이름과 괄호 형식을 기입하는 것을 통해 우리가 선언했던 어떤 함수든 호출할 수 있습니다. 
`another_function`이 프로그램 내에 정의되어 있으므로, `main` 함수에서 해당 함수를 호출할 수 있습니다. 
주의할 점은, 소스 코드 내에서 `another_function`이 `main` 함수 *이후에* 정의했다는 점 입니다. 우리는
이를 이전에도 정의할 수 있습니다. Rust는 당신의 함수의 위치를 신경쓰지 않습니다, 어디든 정의만 되어 있으면 됩니다.


함수를 추가로 탐색하기 위해 *functions* 라는 이름의 새로운 바이너리 프로젝트를 시작합시다.
`another_function` 예제를 * src / main.rs *에 넣고 실행해보세요.
다음과 같은 결과가 나타납니다:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
Hello, world!
Another function.
```

`main` 함수 안의 내용이 줄의 순서대로 수행됩니다. 처음으로, "Hello, world!" 메시지가 출력되고, 
`another_function`이 호출되고 그의 메시지를 출력합니다. 

### 함수 인자들 Function Parameters

함수는 함수 고유한 부분인 특별한 변수 *인자*를 갖는 형식으로 선언될 수 있습니다. 함수가 인자를 취할 때, 우리는 
상수를 그들의 인자로 제공할 수 있습니다. 기술적으로, 여기서 전달되는 상수를 *매개변수*라고 부릅니다만, 사람들은 보통
“인자”와 “매개변수”를 혼용해서 사용하는 경향이 있습니다.

다음의 재작성 된 `another_function`은 Rust에서 인자가 어떤 것이지 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

이 프로그램을 실행해보시면 다음과 같은 결과가 출력되는 것을 보게 될 겁니다:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

`another_function`의 선언은 `x`로 명명된 하나의 인자를 갖습니다. `x`의 유형은 `i32`로 정의됩니다.
`5`가 `another_function`으로 전달되면, `println!` 매크로는 중괄호 짝으로 된 형식 문자열에 `5`를
전달합니다. 함수의 선언부에서, 여러분은 *반드시* 각 인자의 유형을 정의해야 합니다. 이 사항은 Rust를 설계하며 
내린 신중한 결정사항 입니다: 함수의 정의에서 유형에 대한 명시를 요구하여 코드내 다른 부분에서 이들을 사용하는 것을 
통해 당신의 의도를 추측하지 않아도 되게 됩니다. 

여러분이 함수에 여러 개의 인자를 사용하고 싶으면, 인자들을 다음처럼 쉼표와 함께 구분해서 사용할 수 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

이 예제는 각각 `i32` 유행인 두개의 인자들을 갖는 함수를 생성합니다. 함수는 그의 양 인자 값을 출력합니다. 
주의할 점은, 함수 인자는 이번 예제처럼 굳이 같은 유형이 아니여도 된다는 점 입니다. 한번 코드를 실행해봅시다. 
여러분의 *function* 프로젝트의 *src/main.rs* 내용을 위의 예제로 변경한 뒤에, 
`cargo run`을 통해 수행시키면 됩니다:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

우리가 `5`를 `x`로 `6`을 `y`로 전달했기 때문에, 두 문장은 이 값들을 출력합니다.

### 함수 본문 Function Bodies

함수 본문은 선택적 종결자를 갖는 서술식이 나열된 수식으로 구성됩니다. 지금까지 우리는 종결자가 없는 수식만 다뤘기에,
수식이 서술식의 일부처럼 여겨질지 모르겠습니다. Rust가 수식에 기반한 언어기 때문에, 이것은 이해해야 할 중요한 
차이입니다. 다른 언어들은 이와 같은 차이가 없으니, 서술식과 수식이 함수의 본문에 어떤 형태로 다르게 적용되는지를
살펴보도록 하겠습니다. 

### 서술식과 수식 Statements and Expressions

사실 우리는 이미 서술식과 수식들을 사용했습니다. *서술식*은 명령들로 반환 값을 갖지 않는 어떤 수행을 합니다. 
*수식*은 결과 값을 산출해냅니다. 다음 몇 개의 예제를 살펴보도록 합시다. `let` 키워드를 통해 변수를 만들고 
값을 할당하는 서술식을 만듭니다. 항목 3-3의, `let y = 6;`은 서술식 입니다:


<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let y = 6;
}
```

<span class="caption">항목 3-3: 하나의 서술기을 갖는 `main` 함수를 선언하였다. </span>

함수 정의는 또한 하나의 서술식입니다; 상기의 예제는 스스로가 하나의 선언문 입니다. 서술식은 값을 반환하지 않습니다. 
그러니, 여러분은 다음의 시도처럼 `let` 서술식으로 다른 변수에 값을 대입할 수 없습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

여러분이 이 프로그램을 수행하면, 다음과 같은 에러를 보게 될 겁니다:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: variable declaration using `let` is a statement
```

`let y = 6` 서술식은 반환 값이 없으므로, `x`에는 bind 시킬 것이 없습니다. 이것이 다른 C와 Ruby와 
같은 언어와 다른점 입니다. 이들 언어들은 `x = y = 6`를 작성하여 `x`와 `y`에 모두 `6`의 값을 대입할 수 
있습니다; Rust에서는 허용되지 않습니다. 수식은 어떤 값을 산출하며 여러분이 작성하는 Rust의 대부분을 차지합니다. 
`5 + 6`과 같은 간단한 수학 연산을 살펴보면, 이는 `11`이란 값을 산출하는 수식입니다. 

수식은 서술식의 부분일 수 있습니다: 항목 3-3은 `let y = 6;`이란 서술식을 갖는데, `6`은 `6`이란 값을 산출하는
수식입니다. 함수를 호출하는 것은 수식입니다. 매크로를 호출하는 것은 수식입니다. 예제처럼 새로운 범위를 생성하는데 
사용하는 block, `{}`, 은 수식입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

수식 부:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

이번 경우에 해당 block은 `4`를 산출합니다. 이 값은 `let`서술식의 일부로 `y`에 bound 됩니다. 
여러분이 앞서 봐온 것과 다르게 줄의 마지막이 세미콜론으로 끝나지 않은 점을 주목하세요. 수식은 종결 세미콜론을 
사용하지 않습니다. 만약 세미콜론을 수식 마지막에 추가하면, 이는 서술식으로 변경되고 값을 반환하지 않게 됩니다.
이후부터 함수 반환 값과 수식을 살펴보면서 이 점을 유의하세요. 


### 반환 값을 갖는 함수

함수는 그들을 호출한 코드에 값을 반환할 수 있습니다. 우리는 반환되는 값을 명명해야 할 필요는 없지만, 그들의 유형은
화살표(`->`) 뒤에 선언해야 합니다. Rust에서 반환 값은 함수 본문의 최종 수식의 값과 동의어 입니다. 여기 반환
값에 대한 예제가 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

`five` 함수에는 함수 호출, 매크로, 심지어 `let` 서술식도 없이 그저 `5`란 숫자를 갖습니다. 이는 Rust에서
완벽히 적합한 함수 입니다. 함수 반환 값의 유형이 `-> i32`로 명시되어 있다는 점 또한 주목하세요. 해당 코드를
수행하면 다음과 같은 결과를 얻게 될 겁니다:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

`5`는 `five` 함수가 반환한 값이고, 이 때문에 반환 유형은 `i32`로 한 겁니다. 좀더 자세히 설명해보겠습니다. 
The `5` in `five` is the function’s return value, which is why the return type
is `i32`. Let’s examine this in more detail. 

두 중요한 지점이 있습니다: 첫 째, `let x = five();` 줄은 우리가 반환 값을 변수의 초기 값으로 사용하는 것을 
보여줍니다. `five`의 반환 값이 `5`이기 때문에, 해당 줄은 다음과 동일하죠:

```rust
let x = 5;
```

둘 째, `five` 함수는 인자 없이 반환 값에 대한 유형만 정의되어 있지만, 본문에는 `5`만이 세미콜론 없이 외로이 
있는 이유는 이것이 우리가 값을 반환하고자 할때 사용하는 하는 수식이기 때문입니다. 다른 예제를 통해 살펴보겠습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

이 코드를 수행하면 `The value of x is: 6`를 출력할 겁니다. 우리가 `x + 1` 끝에 세미콜론을 추가하여 
수식을 서술식으로 변경하면 어떤 일이 일어날까요?

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

이 코드를 실행하면 다음과 같은 에러를 얻게 됩니다:

```text
error[E0308]: mismatched types
 --> src/main.rs:7:28
  |
7 |   fn plus_one(x: i32) -> i32 {
  |  ____________________________^
8 | |     x + 1;
9 | | }
  | |_^ expected i32, found ()
  |
  = note: expected type `i32`
             found type `()`
help: consider removing this semicolon:
 --> src/main.rs:8:10
  |
8 |     x + 1;
  |          ^
```

에러 메시지의 핵심인, “mismatched types,”은 이 코드의 주요 문제를 보여줍니다. `plus_one` 함수의 
정의는 `i32` 값을 반환하겠다고 하였으나, 서술식은 값을 산출하지 않기에 `()`처럼 비어있는 튜플로 표현됩니다. 
그런 이유로, 반환할 것이 없기에 함수가 정의된 내용과 상충하게 되고 이는 에러를 발생시킵니다. 이번 결과에서는, 
Rust가 문제를 해결할 수 있도록 도와주는 메시지를 제공합니다: 세미콜론을 제거하면 에러가 교정될 수도 있다고 
제안하네요. 
