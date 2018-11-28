## 라이프타임을 이용한 참조자 유효화

4장에서 참조자에 대한 이야기를 할 때, 중요한 디테일을 한 가지 남겨두었습니다: 러스트에서 모든 참조자는
*라이프타임(lifetime)* 을 갖는데, 이는 해당 참조자가 유효한 스코프입니다. 대부분의 경우에서 타입들이
추론되는 것과 마찬가지로, 대부분의 경우에서 라이프타임 또한 암묵적이며 추론됩니다. 여러 가지 타입이
가능하기 때문에 우리가 타입을 명시해야 하는 때와 비슷하게, 참조자의 라이프타임이 몇몇 다른 방식으로
연관될 수 있는 경우들이 있으므로, 러스트는 우리에게 제네릭 라이프타임 파라미터를 이용하여 이 관계들을
명시하길 요구하여 런타임에 실제 참조자가 확실히 유효하도록 확신할 수 있도록 합니다.

네 그렇습니다. 이러한 개념은 다소 흔치 않으며, 여러분들이 다른 프로그래밍 언어에서 사용해온 도구들과는
다른 것입니다. 몇 가지 측면에서, 라이프타임은 러스트의 가장 독특한 기능입니다.

라이프타임은 이 장에서 전체를 다룰 수 없는 큰 주제이므로, 이 장에서는 여러분이 이 개념에 친숙해질 수
있도록 여러분이 라이프타임 문법을 맞닥뜨릴 흔한 경우에 대해 다룰 것입니다. 19장에서는 라이프타임이
할 수 있는 좀 더 상급 정보를 다룰 것입니다.

### 라이프타임은 댕글링 참조자를 방지합니다

라이프타임의 주목적은 댕글링 참조자(dangling reference)를 방지하는 것인데, 댕글링 참조자는
프로그램이 우리가 참조하기로 의도한 데이터가 아닌 다른 데이터를 참조하는 원인이 됩니다.
Listing 10-16의 프로그램과 같이 외부 스코프와 내부 스코프를 가진 프로그램을 생각해봅니다.
외부 스코프는 `r`이라는 이름의 변수를 초기값 없이 선언하였고, 내부 스코프는 `x`라는 이름의 변수를
초기값 5와 함께 선언했습니다. 내부 스코프 내에서, `x`의 참조자를 `r`에 대입하도록 시도했습니다.
그 후 내부 스코프는 끝났고, `r`의 값을 출력하도록 시도했습니다:

```rust,ignore
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

<span class="caption">Listing 10-16: 스코프 밖으로 벗어난 값에 대한 참조자를 사용하는
시도</span>

> #### 초기화되지 않은 변수는 사용할 수 없습니다
>
> 다음에 나올 몇 가지 예제는 초기값을 주지 않고 변수를 선언하고 있으며, 따라서 해당 변수의 이름이 외부
> 스코프에 존재하고 있습니다. 이는 러스트가 널(null) 값을 갖지 않는다는 개념과 충돌을 일으키는 것처럼
> 보일지도 모릅니다. 그러나, 우리가 값을 제공하기 전에 변수를 사용하고자 시도하면, 컴파일 에러가 나올
> 것입니다. 시도해 보세요!

이 코드를 컴파일하면, 다음과 같은 에러가 나타날 것입니다:

```text
error: `x` does not live long enough
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here
```

변수 `x`는 "충분히 오래 살지 못한다(does not live long enough)"고 합니다. 왜 안될까요?
`x`는 7번 라인의 닫는 중괄호 기호에 도달했을 때 내부 스코프가 끝나면서 스코프 밖으로 벗어날
것입니다. 그러나 `r`은 외부 스코프에 대해 유효합니다; 이쪽의 스코프가 더 크고 우리는 이쪽이
"더 오래 산다"라고 말합니다. 만일 러스트가 이 코드를 작동하도록 허용한다면, `r`은 `x`가 스코프
밖으로 벗어났을 때 할당이 해제되는 메모리를 참조하게 될 것이고, `r`을 가지고 시도하려 했던
어떤 것이든 정확히 동작하지 않게 될 것입니다. 그렇다면 러스트는 이 코드가 허용되어서는 안 된다는
것을 어떻게 결정할까요?

#### 빌림 검사기(Borrow checker)

*빌림 검사기(borrow checker)* 라고 불리는 컴파일러의 부분이 모든 빌림이 유효한지를 결정하기
위해 스코프를 비교합니다. Listing 10-17은 변수들의 라이프타임을 보여주는 주석과 함께
Listing 10-16과 동일한 예제를 보여줍니다:

```rust,ignore
{
    let r;         // -------+-- 'a
                   //        |
    {              //        |
        let x = 5; // -+-----+-- 'b
        r = &x;    //  |     |
    }              // -+     |
                   //        |
    println!("r: {}", r); // |
                   //        |
                   // -------+
}
```

<span class="caption">Listing 10-17: 각각 `'a`과 `'b`로 명명된 `r`과 `x`의
라이프타임에 대한 주석</span>

<!-- Just checking I'm reading this right: the inside block is the b lifetime,
correct? I want to leave a note for production, make sure we can make that
clear -->
<!-- Yes, the inside block for the `'b` lifetime starts with the `let x = 5;`
line and ends with the first closing curly brace on the 7th line. Do you think
the text art comments work or should we make an SVG diagram that has nicer
looking arrows and labels? /Carol -->


우리는 `r`의 라이프타임을 `'a`라고 명명하였고, `x`의 라이프타임을 `'b`라고 명명하였습니다.
보시다시피, 내부의 `'b` 블록은 외부의 `'a` 라이프타임 블록에 비해 훨씬 작습니다. 컴파일 타임에서,
러스트는 두 라이프타임의 크기를 비교하고 `r`이 `'a` 라이프타임을 가지고 있지만, `'b` 라이프타임을
가지고 있는 어떤 오브젝트를 참조하고 있음을 보게 됩니다. `'b` 라이프타임이 `'a` 라이프타임에 비해
작기 때문에 러스트 컴파일러는 이 프로그램을 거부합니다: 참조자의 주체가 참조자만큼 오래 살지 못하고 있으니까요.

댕글링 참조자를 만드는 시도가 없고 에러 없이 컴파일되는 Listing 10-18의 예제를 살펴봅시다:

```rust
{
    let x = 5;            // -----+-- 'b
                          //      |
    let r = &x;           // --+--+-- 'a
                          //   |  |
    println!("r: {}", r); //   |  |
                          // --+  |
}                         // -----+
```

<span class="caption">Listing 10-18: 데이터가 참조자에 비해 더 긴 라이프타임을 갖고
있기 때문에 유효한 참조자</span>

여기서 `x`는 라이프타임 `'b`를 갖고 있는데, 위의 경우 `'a`에 비해 더 큽니다. 이는 `r`이 `x`를
참고할 수 있음을 의미합니다: 러스트는 `r`의 참조자가 `x`가 유효한 동안 언제나 유효할 것이라는 점을
알고 있습니다.

지금까지 참조자의 라이프타임이 구제적인 예제 어디에 나오는지를 보았고 러스트가 어떻게 라이프타임을
분석하여 참조자가 항상 유효하도록 확신시키는지를 논의했으니, 이제 함수의 내용물 내에 있는 파라미터와
반환 값에 대한 제네릭 라이프타임에 대하여 이야기해 봅시다.


### 함수에서의 제네릭 라이프타임

두 스트링 슬라이스 중에서 긴 쪽을 반환하는 함수를 작성해 봅시다. 이 함수에 두 개의 스트링 슬라이스를
넘겨서 호출할 수 있기를 원하고, 스트링 슬라이스를 반환하기를 원합니다. Listing 10-19의 코드는
`longest` 함수를 구현하면 `The longest string is abcd`를 출력해야 합니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

<span class="caption">Listing 10-19: 두 스트링 슬라이스 중 긴 쪽을 찾기 위해
`longest` 함수를 호출하는 `main` 함수</span>

`longest` 함수가 인자의 소유권을 얻는 것을 원치 않기 때문에 스트링 슬라이스들을 (4장에서 이야기했던
것처럼 이들은 참조자입니다) 파라미터로서 갖는 함수를 원한다는 점을 주목하세요. 우리는 함수가 `String`의
슬라이스 (이는 변수 `string1`의 타입입니다)는 물론 스트링 리터럴 (이는 변수 `string2`가 담고
있는 것이지요) 또한 받아들일 수 있기를 원하고 있습니다.

<!-- why is `a` a slice and `b` a literal? You mean "a" from the string "abcd"? -->
<!-- I've changed the variable names to remove ambiguity between the variable
name `a` and the "a" from the string "abcd". `string1` is not a slice, it's a
`String`, but we're going to pass a slice that refers to that `String` to the
`longest` function (`string1.as_str()` creates a slice that references the
`String` stored in `string1`). We chose to have `string2` be a literal since
the reader might have code with both `String`s and string literals, and the way
most readers first get into problems with lifetimes is involving string slices,
so we wanted to demonstrate the flexibility of taking string slices as
arguments but the issues you might run into because string slices are
references.
All of the `String`/string slice/string literal concepts here are covered
thoroughly in Chapter 4, which is why we put two back references here (above
and below). If these topics are confusing you in this context, I'd be
interested to know if rereading Chapter 4 clears up that confusion.
/Carol -->

왜 이들이 우리가 원하는 인자 들인 지에 대한 더 많은 논의에 대해서는 4장의 "인자로서의 스트링 슬라이스"를
참조하세요.

만일 Listing 10-20에서 보는 바와 같이 `longest` 함수를 구현하는 시도를 한다면, 이는 컴파일되지
않을 것입니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

<span class="caption">Listing 10-20: 두 스트링 슬라이스 중 긴 쪽을 반환하는
`longest` 함수의 구현체, 그러나 아직 컴파일되지 않음</span>

대신 우리는 라이프타임에 대해 이야기하는 다음과 같은 에러를 얻습니다:

```text
error[E0106]: missing lifetime specifier
   |
1  | fn longest(x: &str, y: &str) -> &str {
   |                                 ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the
   signature does not say whether it is borrowed from `x` or `y`
```

이 도움말은 반환 타입에 대하여 제네릭 라이프타임 파라미터가 필요하다는 것을 말해주고 있는데, 왜냐하면
반환되는 참조자가 `x`를 참조하는지 혹은 `y`를 참조하는지를 러스트가 말할 수 없기 때문입니다. 사실,
우리 또한 모르는데, 이 함수의 본체 내의 `if` 블록은 `x`의 참조자를 반환하고 `else` 블록은
`y`의 참조자를 반환하기 때문입니다!

우리가 이 함수를 정의하고 있는 시점에서, 우리는 이 함수에 넘겨지게 될 구체적인 값을 모르므로, `if`
케이스가 실행될지 혹은 `else` 케이스가 실행될지는 알 수 없습니다. 또한 함수에 넘겨지게 될 참조자의
구체적인 라이프타임을 알지 못하므로, 우리가 반환하는 참조자가 항상 유효한지를 결정하기 위해서
Listing 10-17과 10-18에서 했던 것과 같이 스코프를 살펴볼 수도 없습니다. 빌림 검사기 또한
이를 결정할 수 없는데, 그 이유는 `x`와 `y`의 라이프타임이 반환 값의 라이프타임과 어떻게 연관되어
있는지 알지 못하기 때문입니다. 우리는 참조자들 간의 관계를 정의하는 제네릭 라이프타임 파라미터를 추가하여
빌림 검사기가 분석을 수행할 수 있도록 할 것입니다.

### 라이프타임 명시 문법

라이프타임 명시는 연관된 참조자가 얼마나 오랫동안 살게 되는지를 바꾸지는 않습니다. 함수의 시그니처가
제네릭 타입 파라미터를 특정할 때 이 함수가 어떠한 타입이든 허용할 수 있는 것과 같은 방식으로,
함수의 시그니처가 제네릭 라이프타임 파라미터를 특정할 때라면 이 함수는 어떠한 라이프타임을 가진
참조자라도 허용할 수 있습니다. 라이프타임 명시가 하는 것은 여러 개의 참조자에 대한 라이프타임들을
서로 연관 짓도록 하는 것입니다.

라이프타임 명시는 약간 독특한 문법을 갖고 있습니다: 라이프타임 파라미터의 이름은 어퍼스트로피 `'`로
시작해야 합니다. 라이프타임 파라미터의 이름은 보통 모두 소문자이며, 제네릭 타입과 비슷하게 그들의
이름은 보통 매우 짧습니다. `'a`는 대부분의 사람들이 기본적으로 사용하는 이름입니다. 라이프타임 파라미터
명시는 참조자의 `&` 뒤에 오며, 공백 문자가 라이프타임 명시와 참조자의 타입을 구분해줍니다.

여기 몇 가지 예제가 있습니다: 라이프타임 파라미터가 없는 `i32`에 대한 참조자, `'a`라고 명명된
라이프타임 파라미터를 가지고 있는 `i32`에 대한 참조자, 그리고 역시 라이프타임 `'a`를 갖고 있는
`i32`에 대한 가변 참조자입니다:

```rust,ignore
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

스스로에 대한 하나의 라이프타임 명시는 큰 의미를 가지고 있지 않습니다: 라이프타임 명시는
러스트에게 여러 개의 참조자에 대한 제네릭 라이프타임 파라미터가 서로 어떻게 연관되는지를
말해줍니다. 만일 라이프타임 `'a`를 가지고 있는 `i32`에 대한 참조자인 `first`를
파라미터로, 그리고 또한 라이프타임 `'a`를 가지고 있는 `i32`에 대한 또 다른 참조자인
`second`를 또 다른 파라미터로 가진 함수가 있다면, 이 두 개의 같은 이름을 가진 라이프타임
명시는 참조자 `first`와 `second`가 돌다 동일한 제네릭 라이프타임만큼 살아야 한다는
것을 가리킵니다.

### 함수 시그니처 내의 라이프타임 명시

우리가 작업하고 있던 `longest` 함수의 내용 중에서 라이프타임 명시 부분을 살펴봅시다.
제네릭 타입 파라미터와 마찬가지로, 제네릭 라이프타임 파라미터도 함수 이름과 파라미터 리스트
사이에 꺾쇠괄호를 쓰고 그 안에 정의가 되어야 합니다. 우리가 파라미터들과 반환 값에서의 참조자들에
대해 러스트에게 말해주고 싶은 제약사항은 그들이 모두 동일한 라이프타임을 갖고 있어야 한다는
것인데, 이는 Listing 10-21에서 보는 바와 같이 우리가 `'a`라고 명명하여 각각의 참조자에
추가할 것입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

<span class="caption">Listing 10-21: 시그니처 내의 모든 참조자들이 동일한
라이프타임 `'a`를 가지고 있어야 함을 특정한 `longest` 함수 정의</span>

이는 컴파일될 것이고 Listing 10-19에 있는 `main` 함수에서 사용되었을 때 우리가 원하는 결과를
만들어줄 것입니다.

이 함수 시그니처는 이제 어떤 라이프타임 `'a`에 대하여, 이 함수는 두 개의 파라미터를 갖게 될 것인데,
두 개 모두 적어도 라이프타임 `'a`만큼 살아있는 스트링 슬라이스임을 말해줍니다. 이 함수는 또한
적어도 라이프타임 `'a`만큼 살아있는 스트링 슬라이스를 반환할 것입니다. 이는 러스트에게 우리가 강제하고
싶은 것을 말해주는 계약입니다.


이 함수 시그니처 내에 라이프타임 파라미터를 특정함으로써, 우리는 함수에 넘겨지거나 반환되는 어떠한
값들의 라이프타임도 바꾸지 않지만, 이 계약에 부합하지 않는 어떠한 값들도 빌림 검사기에 의해 거부되어야
함을 말해주는 것입니다. 이 함수는 `x`와 `y`가 정확히 얼마나 오래 살게 될지 알지 못하지만 (혹은
알 필요가 없지만), 다만 이 시그니처를 만족시킬 `'a`에 대입될 수 있는 어떤 스코프가 있음을
알아야 할 필요가 있을 뿐입니다.

함수 안에 라이프타임을 명시할 때, 이 명시는 함수 시그니처에 붙어 있으며, 함수의 본체 내에의 어떠한
코드에도 붙어있지 않습니다. 이는 러스트가 다른 도움 없이 함수 내의 코드를 분석할 수 있지만,
함수가 그 함수 밖의 코드에서의 참조자를 가지고 있을 때, 인자들 혹은 반환 값들의 라이프타임이
함수가 호출될 때마다 달라질 가능성이 있기 때문입니다. 이는 러스트가 발견해내기에는 너무나 비용이
크고 종종 불가능할 것입니다. 이 경우, 우리는 스스로 라이프타임을 명시할 필요가 있습니다.

구체적인 참조자들이 `longest`로 넘겨질 때, `'a`에 대입되게 되는 구체적인 라이프타임은 `y`의
스코프와 겹치는 `x` 스코프의 부분입니다. 스코프는 언제나 중첩되기 때문에, 이것이 제네릭 라이프타임
`'a`이다라고 말하는 또 다른 방법은 `x`와 `y`의 라이프타임 중에서 더 작은 쪽과 동일한 구체적인
라이프타임을 구하는 것일 겁니다. 반환되는 참조자에 대해서도 같은 라이프타임 파라미터인 `'a`를
명시했으므로, 반환되는 참조자도 `x` 와 `y`의 라이프타임 중 짧은 쪽만큼은 길게 유효함을 보장할
것입니다.

서로 다른 구체적인 라이프타임을 가진 참조자들을 넘김으로써 이것이 `longest` 함수의 사용을 어떻게
제한하는지 봅시다. Listing 10-22는 아무 언어에서나 여러분의 직관에 부합될 간단한 예제입니다:
`string1`은 외부 스코프가 끝날 때까지 유효하고 `string2`는 내부 스코프가 끝날 때까지 유효하며,
`result`는 내부 스코프가 끝날 때까지 유효한 무언가를 참조합니다. 빌림 검사기는 이 코드를 승인합니다;
이는 컴파일되며 실행했을 때 `The longest string is long string is long`를 출력합니다:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
#     if x.len() > y.len() {
#         x
#     } else {
#         y
#     }
# }
#
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

<span class="caption">Listing 10-22: 서로 다른 구체적인 라이프타임을 가진 `String`
값의 참조자들을 이용한 `longest` 함수의 사용 </span>

다음으로, `result`의 참조자의 라이프타임이 두 인자들의 라이프타임보다 작아야 함을 보여줄 예제를
시도해봅시다. 우리는 `result`의 선언부를 내부 스코프 밖으로 옮길 것이지만, `result` 변수에 대만
값의 대입은 `string2`가 있는 스코프 내에 남겨둘 것입니다. 다음으로, `result`를 이용하는
`println!` 구문을 내부 스코프 바깥에, 내부 스코프가 끝나는 시점으로 옮기겠습니다. 이렇게 수정한
Listing 10-23의 코드는 컴파일되지 않을 것입니다:


<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

<span class="caption">Listing 10-23: `string2`가 스코프 밖으로 벗어난 후에
`result`를 사용하고자 하는 시도는 컴파일되지 않습니다</span>

만일 이를 컴파일하고자 시도하면, 다음과 같은 에러를 얻습니다:

```text
error: `string2` does not live long enough
   |
6  |         result = longest(string1.as_str(), string2.as_str());
   |                                            ------- borrow occurs here
7  |     }
   |     ^ `string2` dropped here while still borrowed
8  |     println!("The longest string is {}", result);
9  | }
   | - borrowed value needs to live until here
```

이 에러는 `result`가 `println!`에서 유효하기 위해서는. `string2`가 외부 스코프의 끝까지
유효할 필요가 있음을 말해줍니다. 러스트는 이를 알고 있는데, 그 이유는 우리가 함수의 파라미터들과
반환 값에 대해 동일한 라이프타임 파라미터 `'a`를 명시했기 때문입니다.

우리는 인간으로서 이 코드를 살펴볼 수 있고 `string1`이 더 길기 때문에 `result`는 `string1`의
참조자를 담게 될 것이라는 점을 알 수 있습니다. `string1`이 스코프 밖으로 아직 벗어나지 않았기
때문에, `string1`의 참조자는 `println!` 구문에서 여전히 유효할 것입니다. 그렇지만, 우리가
러스트에게 라이프타임 파라미터를 가지고 말해준 것은 `longest` 함수에 의해 반환되는 참조자의
라이프타임이 인자로 넘겨준 라이프타임들 중 작은 쪽과 동일하다는 것이었지요. 따라서, 빌림 검사기는
잠재적으로 유효하지 않은 참조자를 가질 수 있는 문제로 인해 Listing 10-23의 코드를 허용하지
않습니다.

`longest` 함수에 넘겨질 참조자들의 값과 라이프타임들, 그리고 반환된 참조자를 어떻게 이용하는지를
다양화하여 더 많은 실험들을 디자인해 시도해보세요. 컴파일하기 전에 여러분의 실험이 빌림 검사기를
통과할지 안 할지에 대한 가설을 세워보고, 여러분이 맞았는지 확인해보세요!


### 라이프타임의 측면에서 생각하기

라이프타임 파라미터를 특정하는 정확한 방법은 여러분의 함수가 어떤 일을 하고 있는가에 따라 달린 문제입니다.
예를 들면, `longest` 함수의 구현을 제일 긴 스트링 슬라이스 대신 항상 첫 번째 인자를 반환하도록
바꾸었다면, `y` 파라미터에 대한 라이프타임을 특정할 필요는 없을 것입니다. 아래 코드는 컴파일됩니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

이 예제에서, 파라미터 `x`와 반환 값에 대한 라이프타임 파라미터 `'a`는 특정하였지만, 파라미터 `y`는
특정하지 않았는데, 그 이유는 `y`의 라이프타임이 `x` 혹은 반환 값의 라이프타임과 어떠한 관련도 없기
때문입니다.

함수로부터 참조자를 반환할 때, 반환 타입에 대한 라이프타임 파라미터는 인자 중 하나의 라이프타임
파라미터와 일치할 필요가 있습니다. 만일 반환되는 참조가 인자들 중 하나를 참조하지 *않는다면*,
다른 유일한 가능성은 이 함수 내에서 생성된 값을 참조하는 경우인데, 이 값은 함수가 끝나는 시점에서
스코프 밖으로 벗어나기 때문에 댕글링 참조자가 될 것입니다. `longest` 함수에 대한 아래와 같은
구현 시도는 컴파일되지 않습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

우리가 반환 타입에 대해 라이프타임 파라미터 `'a`를 특정했을지라도, 이러한 구현은 컴파일에 실패하게 되는데
이는 반환되는 값의 라이프타임이 파라미터의 라이프타임과 아무런 관련이 없기 때문입니다. 여기 우리가 얻게
되는 에러 메시지를 보시죠:

```text
error: `result` does not live long enough
  |
3 |     result.as_str()
  |     ^^^^^^ does not live long enough
4 | }
  | - borrowed value only lives until here
  |
note: borrowed value must be valid for the lifetime 'a as defined on the block
at 1:44...
  |
1 | fn longest<'a>(x: &str, y: &str) -> &'a str {
  |                                             ^
```

문제는 `result`가 `longest` 함수가 끝나는 지점에서 스코프 밖으로 벗어나게 되어 메모리 해제가
일어나게 되는데, 이 함수로부터 `result`의 참조자를 반환하려는 시도를 한다는 점입니다. 이 댕글링
참조자를 변경시킬 라이프타임 파라미터를 특정할 방법은 없으며, 러스트는 우리가 댕글링 참조자를 만들게끔
놔두지 않습니다. 이 경우, 가장 좋은 수정 방법은 참조자보다는 차라리 값을 소유한 데이터 타입을 리턴하도록
하여 호출하는 함수가 값을 할당 해제하도록 하는 것입니다.

궁극적으로, 라이프타임 문법은 함수들의 다양한 인자들과 반환 값 사이를 연결하는 것에 대한 것입니다.
이들이 일단 연결되고 나면, 러스트는 메모리에 안전한 연산들을 허용하고 댕글링 포인터를 생성하거나
그렇지 않은 경우 메모리 안전을 위배하게 될 연산들을 배제하기에 충분한 정보를 갖게 됩니다.

### 구조체 정의 상에서의 라이프타임 명시

현재까지 우리는 소유권 있는 타입만 들고 있는 구조체들만 정의해왔습니다. 구조체가 참조자를 들고
있도록 할 수 있지만, 구조체 정의 내의 모든 참조자들에 대하여 라이프타임을 표시할 필요가 있습니다.
Listing 10-24에 스트링 슬라이스를 들고 있는 `ImportantExcerpt`라고 명명된 구조체가 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

<span class="caption">Listing 10-24: 참조자를 들고 있는 구조체, 따라서
정의 부분에 라이프타임 명시가 필요합니다</span>

이 구조체는 스트링 슬라이스를 담을 수 있는 `part`라는 하나의 필드를 갖고 있는데, 이것이
참조자입니다. 제네릭 데이터 타입과 마찬가지로, 제네릭 라이프타임 파라미터의 이름을
구조체의 이름 뒤편에 꺾쇠괄호 안에다 선언하여 구조체 정의의 본체 내에서 이 라이프타임 파라미터를
이용할 수 있도록 해야 합니다.

여기 이 `main` 함수는 변수 `novel`이 소유하고 있는 `String`의 첫 문장에 대한 참조자를
들고 있는 `ImportantExcerpt` 구조체의 인스턴스를 생성합니다.

### 라이프타임 생략

이 절에서, 우리는 모든 참조자가 라이프타임을 가지고 있으며, 참조자를 사용하는 함수나 구조체에 대하여
라이프타임 파라미터를 특정할 필요가 있다고 배웠습니다. 하지만, Listing 10-25에서 다시 보여주듯이,
4장의 "스트링 슬라이스"절의 함수는 라이프타임 명시 없이도 컴파일이 됐었지요:

<span class="filename">Filename: src/lib.rs</span>

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

<span class="caption">Listing 10-25: 파라미터와 반환 값의 타입이 참조자임에도 불구하고
라이프타임 명시 없이 컴파일되었던, 4장에서 정의한 바 있는 함수</span>

이 함수가 라이프타임 없이 컴파일되는 이유는 역사가 있습니다: 1.0 이전 시절의 러스트에서는 이 코드가
실제로 컴파일되지 않았습니다. 모든 참조자들은 명시적인 라이프타임이 필요했지요. 그 시절, 함수 시그니처는
아래와 같이 작성되었습니다:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

수많은 러스트 코드를 작성하고 난 후, 러스트 팀은 러스트 프로그래머들이 특정항 상황에서 똑같은 라이프타임
명시를 계속하여 타이핑하고 있다는 사실을 발견하게 되었습니다. 이 상황들은 예측 가능하며 몇 가지 결정론적인
패턴을 따르고 있었습니다. 그리하여 러스트 팀은 러스트 컴파일러 코드 내에 이 패턴들을 프로그래밍하여
이러한 상황 내에서는 프로그래머가 명시적으로 라이프타임 명시를 추가하도록 강제하지 않고 빌림 검사기가
라이프타임을 추론할 수 있도록 하였습니다.

더 많은 결정론적인 패턴들이 출현하여 컴파일러 내에 추가될 가능성이 충분하기에 이러한 러스트의 역사에
대해 언급하였습니다. 나중에는 더욱 적은 라이프타임 명시만이 필요할지도 모르지요.

참조자에 대한 러스트의 분석 기능 내에 프로그래밍된 패턴들을 일컬어
*라이프타임 생략 규칙(lifetime elision rules)* 이라고 합니다. 이들은 프로그래머가 따라야 하는
규칙들이 아닙니다; 이 규칙들은 컴파일러가 고려할 특정한 경우의 집합이고, 여러분의 코드가 이러한
경우에 들어맞으면, 여러분은 명시적으로 라이프타임을 작성할 필요가 없어집니다.

생략 규칙들은 모든 추론을 제공하지는 않습니다: 만일 러스트가 결정론적으로 이 규칙들을 적용했지만
여전히 참조자들이 어떤 라이프타임을 가지고 있는지에 대하여 모호하다면, 해당하는 남은 참조자들의
라이프타임이 어떻게 되어야 하는지에 대해 추측하지 않을 것입니다. 이러한 경우, 컴파일러는 여러분에게
이 참조자들이 서로 어떻게 연관되는지에 대하여 여러분의 의도에 맞게끔 라이프타임을 추가함으로써
해결 가능한 에러를 표시할 것입니다.

먼저 몇 가지 정의들을 봅시다: 함수나 메소드의 파라미터에 대한 라이프타임을
*입력 라이프타임(input lifetime)* 이라고 하며, 반환 값에 대한 라이프타임을
*출력 라이프타임(output lifetime)* 이라고 합니다.

이제 명시적인 라이프타임이 없을 때 참조자가 어떤 라이프타임을 가져야 하는지 알아내기 위해서 컴파일러가
사용하는 규칙들을 봅시다. 첫 번째 규칙은 입력 라이프타임에 적용되고, 다음의 두 규칙들은 출력 라이프타임에
적용됩니다. 만일 컴파일러가 이 세 가지 규칙의 끝에 도달하고 여전히 라이프타임을 알아낼 수 없는 참조자가
있다면, 컴파일러는 에러와 함께 멈출 것입니다.

1. 참조자인 각각의 파라미터는 고유한 라이프타임 파라미터를 갖습니다. 바꿔 말하면, 하나의 파라미터를
   갖는 함수는 하나의 라이프타임 파라미터를 갖고: `fn foo<'a>(x: &'a i32)`, 두 개의 파라미터를
   갖는 함수는 두 개의 라이프타임 파라미터를 따로 갖고: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`,
   이와 같은 식입니다.

2. 만일 정확히 딱 하나의 라이프타임 파라미터만 있다면, 그 라이프타임이 모든 출력 라이프타임 파라미터들에
   대입됩니다: `fn foo<'a>(x: &'a i32) -> &'a i32`.

3. 만일 여러 개의 입력 라이프타임 파라미터가 있는데, 메소드라서 그중 하나가 `&self` 혹은 `&mut
   self`라고 한다면, `self`의 라이프타임이 모든 출력 라이프타임 파라미터에 대입됩니다. 이는
   메소드의 작성을 더욱 멋지게 만들어줍니다.

우리가 직접 컴파일러가 된 척하여 Listing 10-25의 `first_word` 함수의 시그니처에 있는 참조자들의
라이프타임이 무엇인지 알아내기 위해 이 규칙들을 적용해 봅시다. 이 시그니처는 참조자들과 관련된 아무런
라이프타임도 없이 시작합니다:

```rust,ignore
fn first_word(s: &str) -> &str {
```

그러면 (컴파일러로서의) 우리는 첫 번째 규칙을 적용하는데, 이는 각각의 파라미터가 고유의 라이프타임을 갖는다고
말해주고 있습니다. 우리는 이를 평범하게 `'a`라고 명명할 것이며, 따라서 이제 시그니처는 다음과 같습니다:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

두 번째 규칙 상에 놓이게 되는데, 이는 정확히 단 하나의 입력 라이프타임만 존재하기 때문에 적용됩니다.
두 번째 규칙은 그 하나의 입력 파라미터에 대한 라이프타임이 출력 라이프타임에 대입된다고 말하고 있으므로,
이제 시그니처는 다음과 같아집니다:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

이제 이 함수 시그니처의 모든 참조자들이 라이프타임을 갖게 되었고, 컴파일러는 프로그래머에게
이 함수 시그니처 내의 라이프타임을 명시하도록 요구하지 않고도 분석을 계속할 수 있게 되었습니다.

또 다른 예제를 해보려는데, 이번에는 Listing 10-20에서와 같이 우리가 처음 시작할 때의
아무런 라이프타임 파라미터도 가지고 있지 않은 `longest` 함수를 가지고 해 봅시다:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

다시 한번 우리가 컴파일러가 된 척하여, 첫 번째 규칙을 적용해봅시다: 각각의 파라미터는 고유의 라이프타임을
갖습니다. 이번에는 두 개의 파라미터들이 있으므로, 두 개의 라이프타임을 갖게 됩니다:

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

두 번째 규칙을 살펴봤을 때, 하나 이상의 입력 라이프타임이 있으므로 적용되지 않습니다. 세번째 규칙을
살펴봤을 때, 이 또한 적용되지 않는데 이는 이것이 메소드가 아니라 함수이고, 따라서 어떠한 파라미터도
`self`가 아니기 때문입니다. 따라서 규칙이 더 이상 남아있지 않은데, 우리는 아직 반환 다임의
라이프타임이 무엇인지 알아내지 못했습니다. 이것이 바로 Listing 10-20의 코드를 컴파일하려 시도했을
때 에러가 발생한 이유입니다: 컴파일러는 자신이 알고 있는 라이프타임 생략 규칙들을 통해 작업을 수행했지만,
여전히 이 시그니처의 참조자들에 대한 모든 라이프타임을 알아낼 수 없으니까요.

Because the third rule only really applies in method signatures, let's look at
lifetimes in that context now, and see why the third rule means we don't have
to annotate lifetimes in method signatures very often.
세번째 규칙이 오직 메소드 시그니처에 대해서만 실제로 적용되므로, 이제 그러한 경우에서의 라이프타임을
살펴보고, 어째서 세번서 규칙이 메소드 시그니처의 라이프타임을 매우 흔하게 생략해도 된다는 것을
의미하는지 알아봅시다.

### 메소드 정의 내에서의 라이프타임 명시

<!-- Is this different to the reference lifetime annotations, or just a
finalized explanation? -->
<!-- This is about lifetimes on references in method signatures, which is where
the 3rd lifetime elision rule kicks in. It can also be confusing where lifetime
parameters need to be declared and used since the lifetime parameters could go
with the struct's fields or with references passed into or returned from
methods. /Carol -->

라이프타임을 가진 구조체에 대한 메소드를 구현할 때, 문법은 또다시 Listing 10-10에서 보신 바와 같이
제네릭 타입 파라미터의 그것과 같습니다: 라이프타임 파라미터가 선언되고 사용되는 곳은 라이프타임
파라미터가 구조체의 필드들 혹은 메소드 인자와 반환 값과 연관이 있는지 없는지에 따라 달린 문제입니다.

구조체 필드를 위한 라이프타임 이름은 언제나 `impl` 키워드 뒤에 선언되어야 하며, 그러고 나서 구조체의
이름 뒤에 사용되어야 하는데, 이 라이프타임들은 구조체 타입의 일부이기 때문입니다.

`impl` 블록 안에 있는 메소드 시그니처에서, 참조자들이 구조체 필드에 있는 참조자들의 라이프타임과
묶일 수도 있고, 혹은 서로 독립적일 수도 있습니다. 여기에 더해, 라이프타임 생략 규칙이 종종 적용되어
메소드 시그니처 내에 라이프타임 명시를 할 필요가 없습니다. Listing 10-24에서 정의했던
`ImportantExcerpt`라는 이름의 구조체를 이용한 몇 가지 예제를 봅시다.

먼저, 여기 `level`라는 이름의 메소드가 있습니다. 파라미터는 오직 `self`에 대한 참조자이며,
반환 값은 무언가에 대한 참조자가 아닌, 그냥 `i32`입니다:

```rust
# struct ImportantExcerpt<'a> {
#     part: &'a str,
# }
#
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

`impl`뒤의 라이프타임 파라미터 선언부와 타입 이름 뒤에서 이를 사용하는 것이 필요하지만, 첫 번째 생략
규칙때문에 `self`로의 참조자의 라이프타임을 명시할 필요는 없습니다.

아래는 세번째 라이프타임 생략 규칙이 적용되는 예제입니다:

```rust
# struct ImportantExcerpt<'a> {
#     part: &'a str,
# }
#
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

두 개의 입력 라이프타임이 있으므로, 러스트는 첫 번째 라이프타임 생략 규칙을 적용하여 `&self`와
`announcement`에게 각각 라이프타임을 부여합니다. 그다음, 파라미터 중 하나가 `&self`이므로,
반환 타입은 `&self`의 라이프타임을 얻고, 모든 라이프타임들이 추론되었습니다.

### 정적 라이프타임(Static lifetime)

우리가 논의할 필요가 있는 특별한 라이프타임이 *딱 하나* 있습니다: 바로 `'static`입니다.
`'static` 라이프타임은 프로그램의 전체 생애주기를 가리킵니다. 모든 스트링 리터럴은 `'static`
라이프타임을 가지고 있는데, 아래와 같이 명시하는 쪽을 선택할 수 있습니다:

```rust
let s: &'static str = "I have a static lifetime.";
```

이 스트링의 텍스트는 여러분의 프로그램의 바이너리 내에 직접 저장되며 여러분 프로그램의 바이너리는
항상 이용이 가능하지요. 따라서, 모든 스트링 리터럴의 라이프타임은 `'static`입니다.

<!-- How would you add a static lifetime (below)? -->
<!-- Just like you'd specify any lifetime, see above where it shows `&'static str`. /Carol -->

여러분은 어쩌면 에러 메시지 도움말에서 `'static` 라이프타임을 이용하라는 제안을 보셨을지도 모릅니다만,
참조자의 라이프타임으로서 `'static`으로 특정하기 전에, 여러분이 가지고 있는 참조자가 실제로 여러분
프로그램의 전체 라이프타임 동안 사는 것인지 대해 생각해보세요 (혹은 가능하다면 그렇게 오래 살게끔 하고
싶어 할지라도 말이죠). 대부분의 경우, 코드 내의 문제는 댕글링 참조자를 만드는 시도 혹은 사용 가능한
라이프타임들의 불일치이며, 해결책은 이 문제들을 해결하는 것이지 `'static` 라이프타임으로 특정하는
것이 아닙니다.

### 제네릭 타입 파라미터, 트레잇 바운드, 라이프타임을 함께 써보기

그럼 제네릭 타입 파라미터, 트레잇 바운드, 그리고 라이프타임이 하나의 함수에 모두 특정된 문법을
간단하게 살펴봅시다!

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

이것은 Listing 10-21에 나온 바 있는 두 스트링 슬라이스 중 긴 쪽을 반환하는 `longest` 함수지만,
`ann`이라는 이름의 추가 인자를 가지고 있습니다. `ann`의 타입은 제네릭 타입 `T`인데, `where` 절을
가지고 특정한 바와 같이 `Display` 트레잇을 구현한 어떤 타입으로도 채워질 수 있습니다. 이 추가 인자는
함수가 스트링 슬라이스들의 길이를 비교하기 전 출력될 것인데, 이것이 `Display` 트레잇 바운드가 필요한
이유지요. 라이프타임이 제네릭의 한 종류이므로, 라이프타임 파라미터 `'a`와 제네릭 타입 파라미터 `T`
둘 모두에 대한 선언이 함수 이름 뒤 꺾쇠괄호 내에 나열되어 있습니다.


## 정리

이번 절에서 참 많은 것을 다루었습니다! 이제 여러분은 제네릭 타입 파라미터, 트레잇과 트레잇 바운드,
그리고 제네릭 라이프타임 파라미터에 대해 알게되었으니, 여러분은 중복되지 않지만 많은 서로 다른
상활들에서 사용 가능한 코드를 작성할 준비가 되었습니다. 제네릭 타입 파라미터는 코드가 서로 다른
타입에 대해서 적용될 수 있음을 의미합니다. 트레잇과 트레잇 바운드는 그 타입이 제네릭일지라도
해당 타입들이 코드에 필요한 동작을 할 수 있음을 보장합니다. 라이프타임 명시에 의해 특정된
참조자들의 라이프타임 간의 관계는 이 유연한 코드가 어떠한 댕글링 참조자도 만들지 않을 것임을
확신시켜줍니다. 그리고 이 모든 것들이 컴파일 타임에 이루어지므로 런타임 성능에는 영향을 주지 않지요!

믿을진 모르겠지만, 이 부분에 대해 배울 것이 심지어 더 있습니다: 17장에서는 트레잇 객체(trait object)에
대해 다룰 예정인데, 이는 트레잇을 사용하는 또 다른 방법입니다. 19장에서는 라이프타임 명시를 포함하는
더 복잡한 시나리오를 다룰 것입니다. 20장에서는 더 고급 수준의 타입 시스템 특성을 다룰 것입니다. 하지만,
다음 절에서는 러스트에서 어떻게 테스트를 작성하여 우리의 코드가 우리가 원했던 방식대로 모든 기능들을
작동시킨다는 것을 확신할 수 있도록 하는 방법에 대해 이야기해봅시다!
