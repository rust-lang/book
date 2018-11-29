## 슬라이스(Slices)

소유권을 갖지 않는 또다른 데이터 타입은 *슬라이스*입니다. 슬라이스는 여러분이 컬렉션(collection)
전체가 아닌 컬렉션의 연속된 일련의 요소들을 참조할 수 있게 합니다.

여기 작은 프로그래밍 문제가 있습니다: 스트링을 입력 받아 그 스트링에서 찾은 첫번째 단어를 반환하는
함수를 작성하세요. 만일 함수가 공백문자를 찾지 못한다면, 이는 전체 스트링이 한 단어라는 의미이고,
이때는 전체 스트링이 반환되어야 합니다.

이 함수의 시그니처(signature)에 대해 생각해봅시다:

```rust,ignore
fn first_word(s: &String) -> ?
```

이 함수 `first_word`는 `&String`을 파라미터로 갖습니다. 우리는 소유권을 원하지 않으므로, 이렇게
해도 좋습니다. 하지만 뭘 반환해야할까요? 우리는 스트링의 *일부*에 대해 표현할 방법이 없습니다. 하지만
단어의 끝부분의 인덱스를 반환할 수는 있겠습니다. Listing 4-10의 코드처럼 시도해 봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

<span class="caption">Listing 4-10: `String` 파라미터의 바이트 인덱스 값을 반환하는
`first_word` 함수</span>

이 코드를 쪼개서 봅시다. 입력된 `String`를 요소 단위 보면서 그 값이 공백인지 확인할 필요가 있기
때문에, `String`은 `as_bytes` 메소드를 이용하여 바이트 배열로 변환됩니다:

```rust,ignore
let bytes = s.as_bytes();
```

다음으로, `iter` 메소드를 이용하여 바이트 배열의 반복자(iterator)를 생성합니다:

```rust,ignore
for (i, &item) in bytes.iter().enumerate() {
```

반복자에 대한 것은 13장에서 더 자세히 다루겠습니다. 지금은 `iter`가 컬렉션의 각 요소를 반환하는 함수이며,
`enumerate`은 `iter`의 결과값을 직접 반환하는 대신 이를 감싸서 튜플의 일부로 만들어 반환한다는 정도만
알아두세요. 반환된 튜플의 첫번째 요소는 인덱스이며, 두번째 요소는 요소에 대한 참조값입니다. 이는 우리 스스로
인덱스를 계산하는 것보다 조금 더 편리합니다.

`enumerate` 메소드가 튜플을 반환하기 때문에, 우리는 러스트의 다른 모든 부분에서 그러하듯이 이 튜플을
해체하기 위해 패턴을 이용할 수 있습니다. 따라서 `for` 루프 내에서, `i`는 튜플 내의 인덱스에 대응하고
`&item`은 튜플 내의 한 바이트에 대응하는 패턴을 기술한 것입니다. `.iter().enumerate()`의
요소에 대한 참조자를 갖는 것이므로, `&`을 패턴 내에 사용했습니다.

우리는 바이트 리터럴 문법을 이용하여 공백 문자를 나타내는 바이트를 찾습니다. 공백 문자를 찾았다면,
이 위치를 반환합니다. 그렇지 않으면 `s.len()`을 통해 스트링의 길이값을 반환합니다:

```rust,ignore
    if item == b' ' {
        return i;
    }
}
s.len()
```

이제 우리에게 스트링의 첫번째 단어의 끝부분의 인덱스를 찾아낼 방법이 생겼습니다. `usize`를
그대로 반환하고 있지만, 이는 `&string`의 내용물 내에서만 의미가 있습니다. 바꿔 말하면,
이것이 `String`로부터 분리되어 있는 숫자이기 때문에, 이것이 나중에도 여전히 유효한지를 보장할
길이 없습니다. Listing 4-10의 `first_word` 함수를 사용하는 Listing 4-11의 프로그램을
보시죠:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn first_word(s: &String) -> usize {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return i;
#         }
#     }
#
#     s.len()
# }
#
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word는 5를 갖게 될 것입니다.

    s.clear(); // 이 코드는 String을 비워서 ""로 만들게 됩니다.

    // word는 여기서 여전히 5를 갖고 있지만, 5라는 값을 의미있게 쓸 수 있는 스트링은 이제 없습니다.
    // word는 이제 완전 유효하지 않습니다!
}
```

<span class="caption">Listing 4-11: `first_word` 함수를 호출하여 결과를 저장한 뒤
`String`의 내용물을 바꾸기</span>

이 프로그램은 아무런 오류 없이 컴파일되고, `s.clear()`을 호출한 뒤 `word`를 사용한다 해도
역시 컴파일될 것입니다. `word`는 `s`의 상태와 전혀 연결되어 있지 않으므로, `word`는 여전히 값
`5`를 담고 있습니다. 우리는 첫번째 단어를 추출하고자 하기 위해 `s`와 값 `5`를 사용할 수 있지만,
`word`에 `5`를 저장한 뒤 `s`의 내용물이 변경되었기 때문에 이러한 사용은 버그가 될 것입니다.

`word`의 인덱스가 `s`의 데이터와 싱크가 안맞을 것을 걱정하는 건 지겹고 쉽게 발생할 수 있는 오류입니다!
이러한 인덱스들을 관리하는 것은 우리가 `second_word` 함수를 작성했을 때 더더욱 다루기 어려워집니다.
이 함수의 시그니처는 아래와 같은 모양이 되어야 할 것입니다:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

이제 우리는 시작, *그리고* 끝 인덱스를 추적하고 있고, 특정 상태에 있는 데이터로부터 계산되었지만
그 상태와 전혀 묶여있지 않은 더 많은 값들을 갖게 됩니다. 이제 우리는 동기화를 유지할 필요가 있는
주위를 떠다니는 세 개의 관련없는 변수들을 갖게 되었습니다.

운좋게도, 러스트는 이러한 문제에 대한 해결책을 갖고 있습니다: 바로 스트링 슬라이스(string slice)
입니다.

###  스트링 슬라이스

*스트링 슬라이스*는 `String`의 일부분에 대한 참조자고, 아래와 같이 생겼습니다:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

이는 전체 `String`의 참조자를 갖는 것과 비슷하지만, 추가적으로 `[0..5]`라는 코드가 붙어 있습니다.
전체 `String`에 대한 참조자 보다는, `String`의 일부분에 대한 참조자입니다. `start..end` 문법은
`start`부터 시작하여 `end`를 포함하지 않는 연속된 범위를 기술합니다.

우리는 대괄호 내에 `[starting_index..ending_index]`를 특정한 범위를 이용하여 슬라이스를 만들
수 있는데, 여기서 `starting_index`는 슬라이스에 포함되는 첫번째 위치이고 `ending_index`는
슬라이스에 포함될 마지막 위치보다 1을 더한 값입니다. 내부적으로 슬라이스 데이터 구조는 시작 위치와
슬라이스의 길이를 저장하는데, 이 길이 갚은 `ending_index`에서 `starting_index`를 뺸 값입니다.
따라서 `let world = &[6..11];`의 경우, `world`는 `s`의 6번째 바이트를 가리키고 있는 포인터와
길이값 5를 갖고 있는 슬라이스가 될 것입니다.

Figure 4-12는 이를 다이어그램으로 보여줍니다.

<img alt="world containing a pointer to the 6th byte of String s and a length 5" src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-12: `String`의 일부를 참조하는 스트링 슬라이스</span>

러스트의 `..` 범위 문법을 사용하여, 여러분이 만일 첫번째 인덱스(즉 0)에서부터 시작하길 원한다면,
두 개의 마침표 전의 값은 생략할 수 있습니다. 다시 말하면, 아래의 두 줄은 동일한 표현입니다:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

비슷한 이치로, 만일 여러분의 슬라이스가 `String`의 마지막 바이트까지 포함한다면, 여러분은 끝의
숫자를 생략할 수 있습니다. 이는 아래 두 줄의 표현이 동일하다는 의미입니다:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

여러분은 또한 전체 스트링의 슬라이스를 만들기 위해 양쪽 값을 모두 생략할 수 있습니다. 따라서 아래
두 줄의 표현은 동일합니다:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

이 모든 정보를 잘 기억하시고, `first_word`가 슬라이스를 반환하도록 다시 작성해봅시다.
“스트링 슬라이스”를  나타내는 타입은 `&str`로 씁니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

우리는 Listing 4-10에서 작성한 것과 같은 방법으로 공백 문자가 첫번째로 나타난 지점을 찾아서 단어의
끝 인덱스를 얻어냅니다. 공백 문자를 찾으면, 스트링의 시작과 공백 문자의 인덱스를 각각 시작과 끝 인덱스로
사용하여 스트링 슬라이스를 반환합니다.

이제 `first_word`가 호출되면, 해당 데이터와 묶여있는 하나의 값을 반환받게 되었습니다. 이 값은
슬라이스의 시작 위치에 대한 참조자와 슬라이스의 요소 개수로 이루어져 있습니다.

`second_word` 함수에 대해서도 마찬가지로 슬라이스를 반환하는 형식이 잘 동작할 것입니다:

```rust,ignore
fn second_word(s: &String) -> &str {
```

우리는 이제 엉망이 되기 훨씬 힘든 직관적인 API를 갖게 되었는데, 이는 컴파일러가 `String`에 대한
참조자들이 유효한 상태로 남아있게끔 보장할 것이기 때문입니다. 첫번째 단어의 끝 인덱스를 찾았지만,
그 후 스트링을 비워버려서 인덱스가 유효하지 않게되는 Listing 4-11의 프로그램 내의 버그를 기억하시나요?
그런 코드는 논리적으로 맞지 않지만 어떠한 즉각적인 오류도 보여주지 못합니다. 그런 문제는 우리가
비어 있는 스트링에 대해 첫번째 단어의 인덱스를 사용하고자 시도할 경우에나 나타나게 될 것입니다.
슬라이스는 이러한 버그를 불가능하게 만들고 우리가 코드 내에서 발생할 수 있는 문제를 훨씬 일찍 알게
해줍니다. `first_word`의 슬라이스 버젼을 이용하는 것은 컴파일 타임 오류를 발생시킬 것입니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

여기 컴파일 오류 메세지를 보시죠:

```text
17:6 error: cannot borrow `s` as mutable because it is also borrowed as
            immutable [E0502]
    s.clear(); // Error!
    ^
15:29 note: previous borrow of `s` occurs here; the immutable borrow prevents
            subsequent moves or mutable borrows of `s` until the borrow ends
    let word = first_word(&s);
                           ^
18:2 note: previous borrow ends here
fn main() {

}
^
```

빌림 규칙에서 우리가 만일 무언가에 대한 불변 참조자를 만들었을 경우, 가변 참조자를 만들 수 없다는
점을 상기해보세요. `clear` 함수가 `String`을 잘라낼 필요가 있기 때문에, 이 함수는 가변 참조자를
갖기 위한 시도를 할 것이고, 이는 실패하게 됩니다. 러스트는 우리의 API를 사용하기 쉽게 해줄 뿐만 아니라
이러한 종류의 오류 전체를 컴파일 타임에 제거해 줍니다!

#### 스트링 리터럴은 슬라이스입니다

스트링 리터럴이 바이너리 안에 저장된다고 하는 얘기를 상기해봅시다. 이제 슬라이스에 대해 알았으니,
우리는 스트링 리터럴을 적합하게 이해할 수 있습니다:

```rust
let s = "Hello, world!";
```

여기서 `s`의 타입은 `&str`입니다: 이것은 바이너리의 특정 지점을 가리키고 있는 슬라이스입니다.
이는 왜 스트링 리터럴이 불변인가도 설명해줍니다; `&str`은 불번 참조자이기 때문입니다.

#### 파라미터로서의 스트링 슬라이스

여러분이 리터럴과 `String`의 슬라이스를 얻을 수 있다는 것을 알게 되었다면 `first_word` 함수를
한번 더 개선시킬 수 있는데, 바로 이 함수의 시그니처입니다:

```rust,ignore
fn first_word(s: &String) -> &str {
```

더 경험이 많은 러스트인이라면 대신 아래와 같이 작성하는데, 그 이유는 `String`과 `&str` 둘 모두에
대한 같은 함수를 사용할 수 있도록 해주기 때문입니다.

```rust,ignore
fn first_word(s: &str) -> &str {
```

만일 우리가 스트링 슬라이스를 갖고 있다면, 이를 바로 넘길 수 있습니다. `String`을 갖고 있다면,
이 `String`의 전체 슬라이스를 넘길 수 있습니다. 함수가 `String`의 참조자 대신 스트링 슬라이스를
갖도록 정의하는 것은 우리의 API를 어떠한 기능적인 손실 없이도 더 일반적이고 유용하게 해줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn first_word(s: &str) -> &str {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return &s[0..i];
#         }
#     }
#
#     &s[..]
# }
fn main() {
    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작합니다.
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작합니다.
    let word = first_word(&my_string_literal[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word = first_word(my_string_literal);
}
```

### 그 밖의 슬라이스들

스트링 슬라이스는 여러분이 상상하는 바와 같이, 스트링에 특정되어 있습니다. 하지만 더 일반적인
슬라이스 타입도 역시 있습니다. 아래 배열을 보시죠:

```rust
let a = [1, 2, 3, 4, 5];
```

우리가 스트링의 일부를 참조하고 싶어할 수 있는 것처럼, 배열의 일부를 참조하고 싶을 수 있고, 그러면
아래와 같이 쓸 수 있습니다:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

이 슬라이스는 `&[i32]` 타입을 갖습니다. 이는 스트링 슬라이스가 동작하는 방법과 똑같이, 슬라이스의
첫번째 요소에 대한 참조자와 슬라이스의 길이를 저장하는 방식으로 동작합니다. 여러분은 다른 모든 종류의
컬렉션들에 대하여 이런 종류의 슬라이스를 이용할 수 있습니다. 벡터에 대해서 8장에서 이야기할 때 이러한
컬렉션에 대해 더 자세히 다루겠습니다.

## 정리

소유권, 빌림, 그리고 슬라이스의 개념은 러스트 프로그램의 메모리 안정성을 컴파일 타임에 보장하는 것입니다.
러스트 언어는 다른 시스템 프로그래밍 언어와 같이 여러분의 메모리 사용에 대한 제어권을 주지만, 데이터의
소유자가 스코프 밖으로 벗어났을 때 소유자가 자동적으로 데이터를 버리도록 하는 것은 곧 여러분이 이러한
제어를 위해 추가적인 코드 작성이나 디버깅을 하지 않아도 된다는 뜻입니다.

소유권은 러스트의 다른 수많은 부분이 어떻게 동작하는지에 영향을 주므로, 이 책의 남은 부분 전체에 걸쳐
이 개념들에 대해 더 이야기할 것입니다. 다음 장으로 넘어가서 데이터들을 함께 그룹짓는 `struct`를
보겠습니다.
