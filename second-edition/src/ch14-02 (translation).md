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

#### (검수 필요) 주석이 포함된 항목에 대한 문서화 하기

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
우린 주석 뒤에 따라오는 항목이 아닌, 주석을 포함하는 항목을 문서화
할 것이기에 `///` 가 아니라 `//!` 로 시작하는 주석을 사용했습니다.
이 경우, 주석을 포함하는 항목은 크레이트의 루트 파일인 *src/lib.rs* 이며
주석은 전체 크레이트를 설명하게 됩니다.

`cargo doc --open` 을 실행하면,
Figure 14-2 처럼 `my_crate` 문서 첫 페이지 내용 중
크레이트의 공개 아이템들 상단에 이 주석의 내용이 표시될 것입니다.

<img alt="전체 크레이트를 위한 주석이 렌더링 된 HTML 문서" src="img/trpl14-02.png" class="center" />

<span class="caption">Figure 14-2: 전체 크레이트를 설명하는 주석이 포함된
`my_crate` 의 문서가 렌더링된 모습</span>

항목 내? 문서화 주석은 크레이트나 모듈을 설명하는데 유용합니다.
이를 이용해 사용자들이 크레이트의 구조를 이해할 수 있도록
컨테이너의 중심 목적을 설명하세요.

## (검수 필요) `pub use` 로 편리한 공개 API 를 export 하기

7 장에서 우린 `mod` 키워드를 이용해 우리 코드를 체계화 하는 법과,
`pub` 키워드로 공개 아이템을 만드는 법, `use` 를 이용해 스코프 내로 가져오는
법을 다뤘습니다. 다만 여러분이 크레이트를 개발할때 만들어놓은 구조는
여러분의 크레이트를 사용할 사용자들에게는 그다지 편리하지 않을 수 있습니다.
여러분은 여러 단계의 계층 구조를 이용해 크레이트를 구성하고 싶으시겠지만,
여러분이 계층 구조상에서 깊은곳에 정의한 타입을 다른 사람들이 사용하기에는
상당히 어려움을 겪을 수 있습니다. 애초에 그런 타입이 존재하는지 알아내는 것
조차 힘들테니까요. 또한 알아내더라도 `use` `my_crate::UsefulType`; 가 아니라
`use` `my_crate::some_module::another_module::UsefulType;` 를 입력 하는 일은
꽤나 짜증이 날 테죠.

공개 API 의 구조는 크레이트를 배포하는데 있어서 중요한 고려사항 중 하나입니다.
여러분의 크레이트를 이용할 사람들은 해당 구조에 있어서 여러분보다 이해도가
떨어질 것이고, 만약 여러분의 크레이트가 거대한 구조로 되어 있다면 자신들이
원하는 부분을 찾기조차 힘들 겁니다.

좋은 소식은 여러분이 만든 구조가 다른 라이브러리에서 이용하는데 편리하지
*않다고* 해서 굳이 내부 구조를 뒤엎을 필요는 없다는 겁니다. 대신에 여러분은
`pub use` 를 이용해 내부 항목을 다시 export(*re-export*) 하여 기존의 private
구조와 다른 public 구조를 만들 수 있다는 겁니다. 다시 export 한다는 것은 한
위치에서 공개 항목(public item)을 가져오고 마치 다른 위치에서 정의 된 것처럼
다른 위치의 공개 항목으로 만드는 것을 의미합니다.

예를 들어, 우리가 예술의 개념을 모델링 하기 위해 `art` 라는 라이브러리를
만들었다고 가정해 봅시다. 해당 라이브러리에는 두 모듈이 들어 있습니다:
`kinds` 모듈은 `PrimaryColor` 과 `SecondaryColor` 열거체를 포함하고,
`utils` 모듈은 `mix` 라는 이름의 함수를 포함합니다. Listing 14-3 처럼요.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --생략--
    }
}
```

<span class="caption">Listing 14-3: `kinds` 모듈과 `utils` 모듈로 이루어진
`art` 라이브러리</span>

Figure 14-3 은 `cargo doc` 으로 생성된 이 크레이트 문서의
첫 화면입니다:

<img alt="`kinds` 와 `utils` 모듈을 포함한 `art` 크레이트의 문서가 렌더링된 모습" src="img/trpl14-03.png" class="center" />

<span class="caption">Figure 14-3: `kinds` 와 `utils` 모듈을 포함한 `art`
크레이트의 문서가 렌더링된 모습</span>

`PrimaryColor`, `SecondaryColor` 타입들과 `mix` 함수가
첫 화면에 나오지 않는 걸 주목하세요.
이들을 보려면 각각 `kinds` 와 `utils` 를 클릭하셔야 합니다.

이 라이브러리를 의존성으로 가지고 있는 다른 크레이트에선 `use` 를 이용해 `art`
의 항목을 가져오기 위해선, 현재 정의된 `art` 모듈의 구조대로 일일이 입력해야
합니다. Listing 14-4 에서 다른 크레이트에서 `art` 크레이트의 `PrimaryColor` 과
`mix` 를 이용하는 예시를 볼 수 있습니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
extern crate art;

use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

<span class="caption">Listing 14-4: `art` 크레이트의 내부 구조에 정의된 항목을
이용하는 또 다른 크레이트</span>

Listing 14-4 의 코드를 작성한, 즉 `art` 크레이트를 사용하는 사람은
`PrimaryColor` 이 `kinds` 모듈에 들어있고
`mix` 가 `utils` 모듈에 들어 있단 걸 알아내야 합니다.
이처럼 현재 `art` 크레이트의 구조는 크레이트를 사용하는
사람보다 크레이트를 개발하는 사람에게 적합한 구조로 되어 있습니다.
내부 구조상에서의 `kinds` 와 `utils` 모듈의 위치 같은 정보는
`art` 크레이트를 사용하는 입장에서는 전혀 필요 없는 정보이며,
또한 직접 구조상에서 자신이 찾는 것의 위치를 알아내야 하고
`use` 뒤에 모듈의 이름을 일일이 입력해야 한다는 건
혼란스럽고 불편한 일 이니까요.

공개 API 로부터 내부 구조의 흔적를 제거하려면
Listing 14-3 처럼 맨 위에서 `pub use` 를 이용해
다시 export 하도록 `art` 크레이트의 코드를 수정해야 합니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

<span class="caption">Listing 14-5: Re-export 를 위해
`pub use` 추가</span>

`cargo doc` 를 이용해 현재 크레이트에 대한 API 문서를 생성하면 Figure 14-4
처럼 Re-exports 목록과 링크가 첫 페이지에 나타날 겁니다. 이로써 `PrimaryColor`,
`Secondary` 타입과 `mix` 함수를 훨씬 더 쉽게 찾을 수 있게 되었네요.

<img alt="첫 페이지에 Re-exports 목록이 포함된 `art` 크레이트의 문서" src="img/trpl14-04.png" class="center" />

<span class="caption">Figure 14-4: Re-exports 목록이 포함된
`art` 크레이트 문서의 첫 페이지</span>