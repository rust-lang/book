## 테스트를 작성하는 방법

테스트는 테스트 아닌 코드가 프로그램 내에서 기대했던 대로 기능을 하는지 검증하는 러스트
함수입니다. 테스트 함수의 본체는 통상적으로 다음의 세 가지 동작을 수행합니다:

1. 필요한 데이터 혹은 상태를 설정하기
2. 우리가 테스트하고 싶은 코드를 실행하기
3. 그 결과가 우리 예상대로인지 단언하기(assert)

이러한 동작을 하는 테스트 작성을 위해 러스트가 특별히 제공하는 기능들을 살펴봅시다.
`test` 속성, 몇 가지 매크로, 그리고 `should_panic` 속성들을 포함해서 말이죠.

### 테스트 함수의 해부

가장 단순하게 말하면, 러스트 내의 테스트란 `test` 속성(attribute)이 주석으로 달려진
(annotated) 함수입니다. 속성은 러스트 코드 조각에 대한 메타데이터입니다: 한 가지 예로
5장에서 우리가 구조체와 함께 사용했던 `derive` 속성이 있습니다. 함수를 테스트 함수로
변경하기 위해서는, `fn` 전 라인에 `#[test]`를 추가합니다. `cargo test` 커맨드를
사용하여 테스트를 실행시키면, 러스트는 `test` 속성이 달려있는 함수들을 실행하고
각 테스트 함수가 성공 혹은 실패했는지를 보고하는 테스트 실행용 바이너리를 빌드할
것입니다.

7장에서 여러분이 카고를 통해 새로운 라이브러리 프로젝트를 만들었을 때, 테스트 함수를
갖고 있는 테스트 모듈이 자동으로 생성되는 것을 보았습니다. 이 모듈은 우리의 테스트를
작성하기 시작하도록 도움을 주는데, 즉 우리가 새로운 프로젝트를 시작할 때마다 매번 테스트
함수를 위한 추가적인 구조 및 문법을 찾아보지 않아도 되게 해 줍니다.
우리는 원하는 만큼 추가적인 테스트 함수들과 테스트 모듈들을 추가할 수 있습니다!

우리는 실제 코드를 테스팅하지는 않으면서 자동으로 만들어진 템플릿 테스트를 가지고
실험하는 식으로 테스트가 어떻게 동작하는지를 몇 가지 관점에서 탐구할 것입니다.
그러고 나서 우리가 작성한 몇몇 코드를 호출하고 동작이 정확한지를 확고히 하는
실제의 테스트를 작성해 볼 것입니다.

`adder`라고 하는 새로운 라이브러리 프로젝트를 만듭시다:

```text
$ cargo new adder
     Created library `adder` project
$ cd adder
```

여러분의 adder 라이브러리 내에 있는 `src/lib.rs` 파일의 내용물은 Listing 11-1과 같아야
합니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

<span class="caption">Listing 11-1: `cargo new`를 이용하여 자동으로 생성된
테스트 모듈과 함수</span>

지금은 제일 위의 두 줄은 무시하고 함수가 어떻게 작동하는지 알아보는데 집중합시다.
`fn` 라인 전의 `#[test]` 어노테이션을 주목하세요: 이 속성이 바로 이것이 테스트 함수임을
나타내므로, 테스트 실행기는 이 함수를 테스트로 다루어야 한다는 것을 알게 됩니다.
또한 우리는 `tests` 모듈 내에 일반적인 시나리오를 셋업 하거나 일반적인 연산을 수행하는
것을 돕기 위한 테스트 아닌 함수를 넣을 수 있으므로, 어떤 함수가 테스트 함수인지
`#[test]`를 이용하여 나타낼 필요가 있습니다.

이 함수의 본체는 2 + 2가 4와 같음을 단언하기 위해 `assert_eq!` 매크로를 사용합니다.
이 단언은 통상적인 테스트에 대한 형식 예제로서 제공됩니다. 실행하여 이 테스트가
통과되는지 확인해봅시다.

`cargo test` 커맨드는 Listing 11-2에서 보는 바와 같이 우리 프로젝트에 있는
모든 테스트를 실행합니다:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

<span class="caption">Listing 11-2: 자동으로 생성된 테스트를 실행한 결과 </span>

카고는 테스트를 컴파일하고 실행했습니다. `Compiling`, `Finished`, 그리고
`Running` 라인 이후에는 `running 1 test` 라인이 있습니다. 그다음 라인에는
생성된 테스트 함수의 이름인 `it_works`가 나타나고, 테스트의 실행 결과 `ok`가
나타납니다. 그러고 나서 테스트 실행의 전체 요약이 나타납니다.
`test result: ok.`는 모든 테스트가 통과했다는 뜻입니다. `1 passed; 0 failed`는
통과하거나 실패한 테스트의 개수를 추가적으로 보여줍니다.

우리가 무시하라고 표시한 테스트가 없기 때문에, 요약문에 `0 ignored`라고 표시됩니다.
다음 절인 "테스트의 실행방식 제어하기"에서 테스트를 무시하는 것에 대해 다룰 것입니다.

`0 measured` 통계는 성능을 측정하는 벤치마크 테스트를 위한 것입니다.
벤치마크 테스트는 이 글이 쓰인 시점에서는 오직 나이틀리(nightly) 러스트에서만 사용
가능합니다. 나이틀리 러스트에 대한 더 많은 정보는 1장을 보세요.

`Doc-tests adder`로 시작하는 테스트 출력의 다음 부분은 문서 테스트의 결과를 보여주기
위한 것입니다. 아직 어떠한 문서 테스트도 없긴 하지만, 러스트는 우리의 API 문서 내에
나타난 어떠한 코드 예제라도 컴파일할 수 있습니다. 이 기능은 우리의 문서와 코드가
동기화를 유지하도록 돕습니다! 우리는 14장의 "문서 주석"절에서 문서 테스트를 작성하는
방법에 대해 이야기할 것입니다. 지금은 `Doc-tests` 출력을 무시할 것입니다.

우리의 테스트의 이름을 변경하고 테스트 출력이 어떻게 변하는지를 살펴봅시다.
다음과 같이 `it_works` 함수의 이름을 `exploration`으로 변경하세요:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
```

그러고 나서 `cargo test`를 다시 실행시킵니다. 이제 출력 부분에서 `it_works` 대신
`exploration`을 볼 수 있을 것입니다:

```text
running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

다른 테스트를 추가해봅시다. 하지만 이번에는 실패하는 테스트를 만들 것입니다! 테스트
함수 내의 무언가가 패닉을 일으키면 테스트는 실패합니다. 각 테스트는 새로운 스레드
내에서 실행되며, 테스트 스레드가 죽은 것을 메인 스레드가 알게 되면, 테스트는 실패한
것으로 표시됩니다. 9장에서 패닉을 유발하는 가장 단순한 방법에 대해 이야기했었습니다:
바로 `panic!` 매크로를 호출하는 것이죠! 새로운 테스트를 입력하여 여러분의 `src/lib.rs`가
Listing 11-3과 같은 모양이 되게 해 보세요:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

<span class="caption">Listing 11-3: `panic!` 매크로를 호출하기 떄문에 실패하게
될 두번째 테스트 추가 </span>

`cargo test`를 이용하여 다시 한번 테스트를 실행시키세요. 결과 출력은 Listing 11-4와
같이 나올 것인데, 이는 `exploration` 테스트는 통과하고 `another`는 실패했음을 보여줍니다:

```text
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
	thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:10:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed
```

<span class="caption">Listing 11-4: 한 테스트는 통과하고 다른 한 테스트는
실패할 때의 테스트 결과</span>

`test tests::another` 라인은 `ok` 대신 `FAILED`을 보여줍니다. 개별 결과
부분과 요약 부분 사이에 새로운 두 개의 섹션이 나타납니다: 첫번째 섹션은 테스트 실패에
대한 구체적인 이유를 표시합니다. 이 경우, `another`는 `panicked at 'Make this test fail'`
때문에 실패했는데, 이는 *src/lib.rs*의 9번 라인에서 발생했습니다. 다음 섹션은
실패한 모든 테스트의 이름만 목록화한 것인데, 이는 테스트들이 많이 있고 구체적인 테스트
실패 출력이 많을 때 유용합니다. 실패하는 테스트의 이름은 이를 더 쉽게 디버깅하기 위해서
해당 테스트만을 실행시키는데 사용될 수 있습니다; "테스트의 실행방식 제어하기" 절에서
테스트를 실행시키는 방법에 대한 더 많은 내용을 이야기할 것입니다.

요약 라인이 가장 마지막에 표시됩니다: 전체적으로, 우리의 테스트 결과는 `FAILED`입니다.
우리는 하나의 테스트에 통과했고 하나의 테스트에 실패했습니다.

이제 서로 다른 시나리오에서 테스트 결과가 어떻게 보이는지를 알았으니,
`panic!` 외에 테스트 내에서 유용하게 쓰일 수 있는 몇 가지 매크로를 봅시다.

### `assert!` 매크로를 이용하여 결과 확인하기

표준 라이브러리에서 제공하는 `assert!` 매크로는 여러분이 테스트이 어떤 조건이 `true`임을 보장하기를
원하는 경우 유용합니다. `assert!` 매크로에는 부울린 타입으로 계산되는 인자가 제공됩니다. 만일 값이
`true`라면 `assert!`는 아무일도 하지 않고 테스트는 통과됩니다. 만일 값이 `false`라면, `assert!`는
`panic!` 매크로를 호출하는데, 이것이 테스트를 실패하게 합니다. 이는 우리의 코드가 우리 의도대로
기능하고 있는지를 체크하는 것을 도와주는 매크로 중 하나입니다.

5장에 있는 Listing 5-9에서, `Rectangle` 구조체와 `can_hold` 메소드를 다루었는데,
여기 Listing 11-5에 다시 나왔습니다. 이 코드를 *src/main.rs* 대신 *src/lib.rs*에
넣고, `assert!` 매크로를 사용하여 테스트를 작성해봅시다.

<!-- Listing 5-9 wasn't marked as such; I'll fix it the next time I get Chapter
5 for editing. /Carol -->

<span class="filename">Filename: src/lib.rs</span>

```rust
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
```

<span class="caption">Listing 11-5: 5장의 `Rectangle` 구조체와 `can_hold` 메소드
이용하기</span>

`can_hold` 메소드는 부울린 값을 반환하는데, 이는 `assert!` 매크로를 위한 완벽한 사용 사례라는
의미입니다! Listing 11-6에서는 길이 8에 너비 7인 `Rectangle` 인스턴스를 만들고, 이것이
길이 5에 너비 1인 다른 `Rectangle` 인스턴스를 포함할 수 있는지 단언(assert)해보는 것으로
`can_hold` 메소드를 시험하는 테스트를 작성합니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
}
```

<span class="caption">Listing 11-6: 큰 사각형이 작은 사각형을 정말로 담을 수 있는지 검사하는
`can_hold`를 위한 테스트 </span>

`tests` 모듈 내에 새로운 라인이 추가된 것을 주목하세요: `use super::*;`. `tests` 모듈은
우리가 7장에서 다루었던 보통의 가시성 규칙을 따르는 일반적인 모듈입니다. 우리가 내부 모듈 내에 있기
때문에, 외부 모듈에 있는 코드를 내부 모듈의 스코프로 가져올 필요가 있습니다. 여기서는 글롭(`*`)을
사용하기로 선택했고 따라서 우리가 외부 모듈에 정의한 어떠한 것이듯 이 `tests`모듈에서 사용 가능합니다.

우리의 테스트는 `larger_can_hold_smaller`로 명명되었고, 요구된 바와 같이 `Rectangle`
인스턴스를 두 개 생성했습니다. 그 뒤 `assert!` 매크로를 호출하고 `larger.can_hold(&smaller)`
호출의 결과값을 인자로서 넘겼습니다. 이 표현식은 `true`를 반환할 예정이므로, 우리의 테스트는
통과해야 합니다. 자, 이제 알아봅시다!

```text
running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

통과되었군요! 이번에는 작은 사각형이 큰 사각형을 포함시킬수 없음을 단언하는 또 다른 테스트를
추가합시다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}
```

이 경우 `can_hold` 함수의 올바른 결과값은 `false`이므로, `assert!` 매크로에게 넘기기 전에
이 결과를 반대로 만들 필요가 있습니다. 결과적으로, 우리의 테스트는 `can_hold`가 `false`를
반환할 경우에만 통과할 것입니다:

```text
running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

통과하는 테스트가 두 개가 되었습니다! 이제는 만약 우리의 코드에 버그가 있을 때는 테스트 결과가
어찌되는지 봅시다. `can_hold` 메소드의 구현 부분 중 큰(>) 부등호를 이용해 길이를 비교하는 부분을
작은(<) 부등호로 바꿔봅시다:

```rust
# #[derive(Debug)]
# pub struct Rectangle {
#     length: u32,
#     width: u32,
# }
// --snip--

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length < other.length && self.width > other.width
    }
}
```

테스트를 실행시키면 이제 아래와 같이 출력됩니다:

```text
running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED

failures:

---- tests::larger_can_hold_smaller stdout ----
	thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed:
    larger.can_hold(&smaller)', src/lib.rs:22:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

우리의 테스트가 버그를 찾았습니다! `larger.length`는 8이고 `smaller.length`는 5이므로,
`can_hold`의 길이 부분에 대한 비교값은 이제 `false`를 반환합니다: 8이 5보다 작지 않으니까요.

### `aseert_eq!`와 `assert_ne!`를 이용한 동치(equality) 테스트

기능성을 테스트하는 일반적인 방법은 테스트 내의 코드의 결과값과 우리가 기대하는 값을 비교하여 둘이
서로 같은지를 확실히 하는 것입니다. 이를 `assert!` 매크로에 `==`를 이용한 표현식을 넘기는 식으로
할 수도 있습니다. 그러나 이러한 테스트를 더 편리하게 수행해주는 표준 라이브러리가 제공하는 한 쌍의
매크로 - `assert_eq!`와 `assert_ne!` - 가 있습니다. 이 매크로들은 각각 동치(equality)와
부동(inequality)을 위해 두 인자를 비교합니다. 또한 이들은 만일 단언에 실패한다면 두 값을 출력해
주는데, 이는 *왜* 테스트가 실패했는지를 포기 더 쉬워집니다; 반면, `assert!`는 `==` 표현식에 대해
`false` 값을 얻었음을 가리킬 뿐, 어떤 값이 `false`값을 야기했는지는 알려주지 않습니다.

Listing 11-7와 같이, 파라미터에 `2`를 더하여 결과를 반환하는 `add_two` 함수를 작성합시다. 그 후
`assert_eq!` 매크로를 이용하여 이 함수를 테스트하겠습니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

<span class="caption">Listing 11-7: `assert_eq!` 매크로를 이용하는 `add_two` 함수
테스트</span>

이게 통과하는지 확인해 봅시다!

```text
running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`assert_eq!` 매크로에 제공한 첫번째 인자 4는 `add_two(2)` 호출의 결과와 동일합니다.
이 테스트에 대한 라인은 `test tests::it_adds_two ... ok`이고, `ok` 문자열은 테스트가
통과했음을 나타냅니다!

`assert_eq!`를 이용하는 테스트가 실패했을때는 어떻게 보이는지를 알아보기 위해 테스트에 버그를
집어넣어 봅시다. `add_two` 함수에 `3`을 대신 더하는 형태로 구현을 변경해 보세요:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

테스트를 다시 실행해 보세요:

```text
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
        thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:8

failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

우리의 테스트가 버그를 잡았어요! `it_adds_two` 테스트는
`` assertion failed: `(left == right)` ``라는 메세지와 `left`는 4였고 `right`는 5였다는
것으로 보여줌과 함께 실패했습니다. 이 메세지는 우리가 디버깅을 시작하는데 유용한 도움을 줍니다:
`assert_eq!`의 `left` 인자는 4였는데, `add_two(2)`를 넣은 right` 인자는 5라고 말해주고 있습니다.

몇몇 언어와 테스트 프레임워크 내에서는, 두 값이 같은지를 단언하는 함수의 파라미터를 `expected`와
`actual`로 부르며, 우리가 인자를 넣는 순서가 중요하다는 점을 기억하세요. 하지만 러스트에서는
그 대신 `left`와 `right`라고 불리며 우리가 기대한 값과 테스트 내의 코드가 생성하는 값을
지정하는 순서는 중요치 않습니다. 이 테스트의 단언을 `assert_eq!(add_two(2), 4)`로 작성할
수도 있는데, 이는 `` assertion failed: `(left == right)` ``와 `left`는 `5`고 `right`는
`4`라는 실패 메세지를 만들어낼 것입니다.

`assert_ne!` 매크로는 우리가 제공한 두 개의 값이 서로 갖지 않으면 통과하고 동일하면 실패할 것입니다.
이 매크로는 어떤 값이 *될 것인지*는 정확히 확신하지 못하지만, 어떤 값이라면 절대로 *될 수 없는지*는
알고 있을 경우에 가장 유용합니다. 예를 들면, 만일 어떤 함수가 입력값을 어떤 방식으로든 변경한다는 것을
보장하지만, 그 입력값이 우리가 테스트를 실행한 요일에 따라 달라지는 형태라면, 단언을 하는 가장 좋은
방법은 함수의 결괏값이 입력값과 같지 않다는 것일지도 모릅니다.

표면 아래에서, `assert_eq!`와 `assert_ne!` 매크로는 각각 `==`과 `!=` 연산자를 이용합니다.
단언에 실패하면, 이 매크로들은 디버그 포맷팅을 사용하여 인자들을 출력하는데, 이는 비교되는 값들이
`PartialEq`와 `Debug` 트레잇을 구현해야 한다는 의미입니다. 모든 기본 타입과 표준 라이브러리가
제공하는 대부분의 타입들은 이 트레잇들을 구현하고 있습니다. 여러분이 정의한 구조체나 열거형에 대해서,
해당 타입의 값이 서로 같은지 혹은 다른지를 단언하기 위해서는 `PartialEq`를 구현할 필요가 있습니다.
단언에 실패할 경우에 값을 출력하기 위해서는 `Debug`를 구현해야 합니다. 5장에서 설명한 바와 같이
두 트레잇 모두 추론 가능한(derivable) 트레잇이기 때문에, 이 트레잇의 구현은 보통
`#[derive(PartialEq, Debug)]` 어노테이션을 여러분의 구조체나 열거형 정의부에 추가하는 정도로
간단합니다. 이에 대한 것과 다른 추론 가능한 트레잇에 대한 더 자세한 내용은 부록 C를 참고하세요.

### 커스텀 실패 메세지 추가하기

또한 우리는 `assert!`, `assert_eq!` 및 `assert_ne!` 매크로의 추가 인자로서 커스텀 메세지를 입력하여
실패 메세지와 함께 출력되도록 할 수 있습니다. `assert!`가 요구하는 하나의 인자 후에 지정된 인자들이나
`assert_eq!`와 `assert_ne!`가 요구하는 두 개의 인자 후에 지정된 인자들은 우리가 8장의 “`+` 연산자나
`format!` 매크로를 이용한 접합”절에서 다루었던 `format!` 매크로에 넘겨지므로, 여러분은 `{}` 변경자
(placeholder)를 갖는 포맷 스트링과 이 변경자에 입력될 값들을 넘길 수 있습니다. 커스텀 메세지는 해당
단언의 의미를 문서화하기 위한 용도로서 유용하므로, 테스트가 실패했을 때, 코드에 어떤 문제가 있는지에
대해 더 좋은 생각을 가질 수 있습니다.

예를 들어, 이름을 부르며 사람들을 환영하는 함수가 있고, 이 함수에 넘겨주는 이름이 출력 내에 있는지
테스트하고 싶다고 칩시다:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

여기서 이 프로그램의 요구사항은 아직 합의되지 않았고, 인사말의 시작 지점에 있는 `Hello` 텍스트가
변경될 것이라는 점이 꽤나 확실한 상태라고 칩시다. 우리는 그런 변경사항이 생기더라도 이름에 대한 테스트를
갱신할 필요는 없다고 결정했고, 따라서 `greeting` 함수로부터 반환된 값과 정확히 일치하는 체크 대신,
출력 값이 입력 파라미터의 텍스트를 포함하고 있는지만 단언할 것입니다.

`greeting`이 `name`을 포함하지 않도록 변경하는 것으로 버그를 집어넣어 테스트 실패가 어떻게 보이는지
살펴봅시다:

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```

이 테스트를 수행하면 다음을 출력합니다:

```text
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
    thread 'tests::greeting_contains_name' panicked at 'assertion failed:
    result.contains("Carol")', src/lib.rs:12:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::greeting_contains_name
```

이 결과는 그저 단언이 실패했으며 몇 번째 줄의 단언이 실패했는지만을 나타냅니다. 이 경우에서
더 유용한 실패 메세지는 `greeting` 함수로부터 얻은 값을 출력하는 것일 테지요. 테스트 함수를 바꿔서
`greeting` 함수로부터 얻은 실제 값으로 채워질 변경자를 이용한 포맷 스트링으로부터 만들어지는 커스텀
실패 메세지를 줄 수 있도록 해봅시다:

```rust,ignore
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}
```

이제 테스트를 다시 실행시키면, 더 많은 정보를 가진 에러 메세지를 얻을 것입니다:

```text
---- tests::greeting_contains_name stdout ----
	thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain
    name, value was `Hello!`', src/lib.rs:12:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

이제 실제로 테스트 출력에서 얻어진 값을 볼 수 있고, 이는 우리가 기대했던 일 대신 실제 어떤 일이
일어났는지 디버깅하는데 도움을 줄 것입니다.

### `should_panic`을 이용한 패닉에 대한 체크

우리의 코드가 우리가 기대한 정확한 값을 반환하는 것을 체크하는 것에 더하여, 우리의 코드가
우리가 기대한 대로 에러가 나는 경우를 처리할 수 있는지 체크하는 것 또한 중요합니다. 예를 들어,
9장의 Listing 9-9에서 우리가 만들었던 `Guess` 타입을 떠올려보세요. `Guess`를 이용하는
다른 코드는 `Guess` 인스턴스가 1과 100 사이의 값만 가질 것이라는 보장에 의존적입니다.
우리는 범위 밖의 값으로 `Guess` 인스턴스를 만드는 시도가 패닉을 일으킨다는 것을 확실히 하는
테스트를 작성할 수 있습니다.

이는 또 다른 속성인 `should_panic`를 테스트 함수에 추가함으로써 할 수 있습니다. 이 속성은
함수 내의 코드가 패닉을 일으키면 테스트가 통과하도록 만들어줍니다; 함수 내의 코드가 패닉을
일으키지 않는다면 테스트는 실패할 것입니다.

Listing 11-8은 `Guess::new`의 에러 조건이 우리 예상대로 발동되는지를 검사하는 테스트를
보여줍니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

<span class="caption">Listing 11-8: 어떤 조건이 `panic!`을 일으키는지에 대한
테스트</span>

`#[should_panic]` 속성이 `#[test]` 속성 뒤, 그리고 적용될 테스트 함수 앞에 붙었습니다.
이 테스트가 통과될 때의 결과를 봅시다:

```text
running 1 test
test tests::greater_than_100 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

좋아 보이는군요! 이제 `new` 함수가 100 이상의 값일 때 패닉을 발생시키는 조건을 제거함으로써 코드에
버그를 넣어봅시다:

```rust
# pub struct Guess {
#     value: u32,
# }
#
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1  {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}
```

Listing 11-8의 테스트를 실행시키면, 아래와 같이 실패할 것입니다:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

이 경우에는 그다지 쓸모 있는 메세지를 얻지 못하지만, 한번 테스트 함수를 살펴보게 되면,
함수가 `#[should_panic]`으로 어노테이션 되었다는 것을 볼 수 있습니다. 우리가 얻은 실패는
함수 내의 코드가 패닉을 일으키지 않았다는 의미가 됩니다.

`should_panic` 테스트는 애매할 수 있는데, 그 이유는 이 속성이 단지 코드에서 어떤 패닉이
유발되었음만을 알려줄 뿐이기 때문입니다. `should_panic` 테스트는 일어날 것으로 예상한 것 외의
다른 이유로 인한 패닉이 일어날 지라도 통과할 것입니다. `should_panic` 테스트를 더 엄밀하게 만들기
위해서, `should_panic` 속성에 `expected` 파라미터를 추가할 수 있습니다. 이 테스트 도구는
실패 메세지가 제공된 텍스트를 담고 있는지 확실히 할 것입니다. 예를 들면, Listing 11-9와 같이
입력된 값이 너무 작거나 혹은 너무 클 경우에 대해 서로 다른 메세지를 가진 패닉을 일으키는 `new` 함수를
갖고 있는 수정된 `Guess` 코드를 고려해봅시다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Guess {
#     value: u32,
# }
# 
// --snip

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

<span class="caption">Listing 11-9: 어떤 조건이 특정 패닉 메세지를 가진 `panic!`을
일으키는 테스트 </span>

이 테스트는 통과할 것인데, 그 이유는 `should_panic` 속성에 추가한 `expected` 파라미터 값이
`Guess::new` 함수가 패닉을 일으킬 때의 메세지의 서브 스트링이기 때문입니다. 우리가 예상하는
전체 패닉 메세지로 특정할 수도 있는데, 그러한 경우에는 `Guess value must be less
than or equal to 100, got 200.`이 되겠지요. 여러분이 `should_panic`에 대한 기대하는 파라미터를
특정하는 것은 패닉 메세지가 얼마나 유일한지 혹은 유동적인지, 그리고 여러분의 테스트가 얼마나
정확하기를 원하는지에 따라서 달라집니다. 위의 경우, 패닉 메세지의 서브 스트링은 실행된 함수의 코드가
`else if value > 100` 경우에 해당함을 확신하기에 충분합니다.

`expect` 메세지를 가진 `should_panic` 테스트가 실패하면 어떻게 되는지 보기 위해서, 다시 한번
`if value < 1` 아래 코드 블록과 `else if value > 100` 아래 코드 블록을 바꿔서 버그를
만들어봅시다:

```rust,ignore
if value < 1 {
    panic!("Guess value must be less than or equal to 100, got {}.", value);
} else if value > 100 {
    panic!("Guess value must be greater than or equal to 1, got {}.", value);
}
```

이번에는 `should_panic` 테스트를 실행하면, 아래와 같이 실패합니다:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
        thread 'tests::greater_than_100' panicked at 'Guess value must be greater than or equal to 1, got 200.', src/lib.rs:11:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
note: Panic did not include expected string 'Guess value must be less than or
equal to 100'

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

실패 메세지는 이 테스트가 우리 예상에 맞게 실제로 패닉에 빠지기는 했으나, 패닉 메세지가 예상하는
스트링을 포함하지 않고 있다고 말하고 있습니다 (`did not include expected string 'Guess
value must be less than or equal to 100'`.) 우리가 얻어낸 패닉 메세지를 볼 수 이는데,
이 경우에는 `Guess value must be greater than or equal to 1, got 200.` 이었습니다.
그러면 우리는 어디에 우리의 버그가 있는지를 찾아내기 시작할 수 있습니다!

이제까지 테스트를 작성하는 몇 가지 방법을 알게 되었으니, 우리의 테스트를 실행할 때 어떤 일이
벌어지는지를 살펴보고 `cargo test`와 함께 사용할 수 있는 어려가지 옵션들에 대해서 탐구해봅시다.
