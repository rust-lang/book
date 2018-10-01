## Crates.io 에 크레이트 배포하기

우린 [crates.io](https://crates.io)<!-- ignore --> 의 패키지를 프로젝트의 디펜던시로만 사용했습니다. 하지만 여러분이 직접 여러분의 패키지를 배포해서 코드를 다른 사람들과 공유 할 수도 있습니다.

### (검수 필요) The crate registry at [crates.io](https://crates.io)<!-- ignore --> distributes the source code of your packages, so it primarily hosts code that is open source.


## (검수 필요) 러스트와 Cargo 는 배포된 패키지를 사람들이 처음부터 찾고 사용하기 쉽게 도와주는 기능이 있습니다.

## (검수 필요) 다음에 이런 기능들 중 일부에 대한 내용과 패키지를 배포하는 방법을 설명하겠습니다.


### 유용한 문서화 주석 만들기

여러분의 패키지를 시간을 들여서 자세하게 문서화하는 작업은 굉장히 가치있는 일
입니다. 문서는 다른 사람들이 그 패키지를 언제, 어떻게 써야할지 알게 해주는데
굉장히 도움이 되거든요. 3장에서 우린 슬래시 두 개(`//`) 를 이용해 러스트
코드에 주석을 남기는 법을 배웠습니다만, 러스트에는 *문서화 주석(documentation
comment)* 이라고 불리는 문서화를 위한 특별한 주석이 존재합니다. 이 주석은 HTML
문서를 생성할 수 있는데, 이 HTML 에는 여러분의 크레이트가 어떻게
*구현되었는지* 가 아닌 어떻게 *사용하는지* 에 관심 있는 프로그래머들을 위한
공개 API의 문서화 주석이 보여집니다.

문서화 주석은 슬래시 두 개가 아니라 세 개(`///`) 를 이용하며 텍스트 서식을
위한 마크다운 표기법을 지원합니다. 문서화 주석은 문서화할 대상 바로 이전에
배치하면 됩니다. Listing 14-1 은 `my_crate` 크레이트의 `add_one` 함수에 대한'
문서화 주석의 예시를 보여줍니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

<span class="caption">Listing 14-1: 함수에 대한
문서화 주석</span>

자, `add_one` 함수가 무슨 일을 하는지 설명을 적었고 `Example` 절에서
`add_one` 함수를 어떻게 사용하는지에 대한 예시 코드를 제공 했습니다.
이제 우린 `cargo doc` 을 이용해 이 문서화 주석으로부터
HTML 문서를 생성할 수 있습니다.
이 명령어는 러스트에 들어있는 `rustdoc` 툴을 실행시키고
생성된 HTML 문서를 *target/doc* 디렉토리에 저장합니다.

좀더 편리하게, `cargo doc --open` 을 실행시키면 여러분의 현재 크레이트의
문서에 대해 (심지어 여러분의 크레이트가 가진 모든 디펜던시의 문서까지)
HTML 을 생성하고 웹 브라우저에 띄워줄 겁니다. 이제 `add_one` 함수를 찾아보면
여러분은 문서화 주석의 내용이 어떻게 나타나는지 보실 수 있습니다.
Figure 14-1 처럼요:

<img alt="Rendered HTML documentation for the `add_one` function of `my_crate`" src="img/trpl14-01.png" class="center" />

<span class="caption">Figure 14-1: `add_one` 함수에 대한
HTML 문서화</span>

#### (검수 필요) 자주 사용되는 구절

우린 Listing 14-1 에서 HTML 에 "Examples." 제목을 가진 구절을 만들기 위해
`# Examples` 마크다운 헤더를 사용했습니다. 이외에 크레이트의 제작자가
일반적으로 문서에 사용하는 구절은 다음과 같습니다.

* **Panics**: 문서화된 기능이 패닉을 일으킬 수 있는 시나리오입니다.
  함수를 호출하는 사람들에게 "프로그램이 패닉을 일으키지 않게 하려면
  이러한 상황에서는 이 함수를 호출하지 않아야 합니다" 라는 내용을 알려줍니다.
* **Errors**: 해당 함수가 `Result` 를 반환할 경우에는 발생할 수 있는
  에러의 종류와 해당 에러들이 발생하는 조건을 설명해 주어서 호출하는 사람이
  여러 에러를 여러 방법으로 처리할 수 있도록 해야합니다.
* **Safety**: 함수가 `안전하지 않을(unsafe)` 경우에
  (19장에서 다루는 내용입니다)
  ## (검수 필요) 왜 이 함수가 안전하지 않은지와 호출하는 사람이 covering the invariants that the function expects callers to uphold 설명하는 구절이 있어야합니다.


대부분의 문서화 주석은 이 구절들이 모두 필요하진 않습니다.
하지만 여러분의 코드를 사람들의 관심과 흥미를 갖도록 만드는데
사용할 좋은 체크리스트가 될 수 있습니다.

### 테스트로서의 문서화 주석

여러분의 문서화 주석에 예시 코드를 추가하는 건 여러분의 라이브러리를 어떻게
사용하는지 알려줄 수 있을뿐더러 또 다른 효과도 있습니다: 무려 `cargo test` 를
실행하면 여러분의 문서에 들어있던 예시 코드들이 테스트로서 실행됩니다! 백문이
불여일견이라고, 예시를 포함한 문서보다 좋은 문서는 없습니다. (하지만 코드를
변경하고 문서를 업데이트하지 않아서 예시 코드가 작동하지 않는 일은 절대 있어선
안됩니다) 우리가 Listing 14-1 의 `add_one` 함수에 대한 문서로 `cargo test` 를
실행하면 다음과 같은 테스트 결과를 보실수 있습니다.

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

이제 우리가 함수나 예제를 변경하고 예시 코드에서 패닉이 발생하는 상태로
`cargo test` 를 실행하면, 문서 테스트 기능이 더이상 예시 코드가
기능하지 못한다고 알려줄 겁니다.

#### (검수 필요) 주석이 포함된 항목에 주석 달기

문서화 주석의 또 다른 스타일로 `//!` 가 있습니다.
이는 주석 뒤에 오는 항목을 문서화 하는게 아닌 주석을
포함하는 항목을 문서화 합니다. 일반적으로 크레이트의 루트 파일
(관례적으로 *src/lib.rs* 입니다) 이나 크레이트 혹은 모듈 전체를 문서화하는
모듈 내부에 이 문서화 주석을 작성합니다.

예시로, 만약 `add_one` 함수를 포함한 `my_crate` 크레이트를
설명하기 위한 목적으로 문서화를 진행한다면,
Listing 14-2 처럼 *src/lib.rs* 에 `//!` 로 시작하는
문서화 주석을 추가할 수 있습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

<span class="caption">Listing 14-2: `my_crate` 크레이트 전체를 위한
문서화</span>

`//!` 로 시작하는 줄 중 마지막 줄에 코드가 뒤따르지 않는다는 점을 주목하세요.
우린 주석 뒤에 따라오는 항목이 아닌, 포함하는 항목을 문서화 할 것이기에 `///`
가 아니라 `//!` 로 시작하는 주석을 사용했습니다.
이 경우, 주석을 포함하는 항목은 크레이트의 루트 파일인 *src/lib.rs* 이며
주석은 전체 크레이트를 설명하게 됩니다.

`cargo doc --open` 을 실행하면,
Figure 14-2 처럼 `my_crate` 문서 프론트 페이지 내용 중
크레이트의 공개 항목들 상단에 이 주석의 내용이 표시될 것입니다.

<img alt="전체 크레이트를 위한 주석이 렌더링 된 HTML 문서" src="img/trpl14-02.png" class="center" />

<span class="caption">Figure 14-2: 전체 크레이트를 설명하는 주석이 포함된
`my_crate` 의 문서가 렌더링된 모습</span>
