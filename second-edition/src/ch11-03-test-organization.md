## 테스트 조직화

이 장의 시작 부분에서 언급했듯이, 테스팅은 복잡한 분야이고, 여러 사람들이 서로 다른 용어와 조직화 방식을
이용합니다. 러스트 커뮤니티에서는 테스트에 대해서 두 개의 주요한 카테고리로 나눠 생각합니다:
*단위 테스트(unit test)* 그리고 *통합 테스트(integration test)*입니다. 단위 테스트는 작고
하나에 더 집중하며, 한 번에 하나의 모듈만 분리하여 테스트하고, 비공개 인터페이스 (private
interface)를 테스트합니다. 통합 테스트는 완전히 여러분의 라이브러리 외부에 있으며, 공개 인터페이스
(public interface)를 이용하고 테스트마다 여러 개의 모듈을 잠재적으로 실험함으로써, 다른 외부의 코드가
하는 방식과 동일한 형태로 여러분의 코드를 이용합니다.

두 종류의 테스트 작성 모두가 여러분의 라이브러리 코드 조각들이 따로따로 혹은 함께 사용되었을 때 여러분이
기대하는 바와 같이 작동하는 지를 확신시키는데 중요합니다.

### 단위 테스트

단위 테스트의 목적은 각 코드의 단위를 나머지 부분과 분리하여 테스트하는 것인데, 이는 코드가 어디 있고
어느 부분이 기대한 대로 동작하지 않는지를 빠르게 정확히 찾아낼 수 있도록 하기 위함입니다. 단위 테스트는
*src* 디렉토리 내에 넣는데, 각 파일마다 테스트하는 코드를 담고 있습니다. 관례는 각 파일마다 테스트
함수를 담고 있는 `tests`라는 이름의 모듈을 만들고, 이 모듈에 `cfg(test)`라고 어노테이션 하는
것입니다.

#### 테스트 모듈과 `#[cfg(test)]`

테스트 모듈 상의 `#[cfg(test)]` 어노테이션은 러스트에게 우리가 `cargo build`를 실행시킬 때가
아니라 `cargo test`를 실행시킬 때에만 컴파일하고 실행시키라고 말해줍니다. 이는 우리가 오직 라이브러리만
빌드하고 싶을 때 컴파일 시간을 절약시켜주고, 테스트가 포함되어 있지 않으므로 컴파일 결과물의 크기를
줄여줍니다. 통합 테스트는 다른 디렉토리에 위치하기 때문에, 여기에는 `#[cfg(test)]` 어노테이션이
필요치 않음을 앞으로 보게 될 것입니다. 하지만, 단위 테스트가 해당 코드와 동일한 파일에 위치하기 때문에,
`#[cfg(test)]`를 사용하여 컴파일 결과물에 이들이 포함되지 않아야 함을 특정합니다.

이 장의 첫 번째 절에서 새로운 `adder` 프로젝트를 생성했을 때, 카고가 우리를 위하여 아래와 같은 코드를
생성했던 것을 상기하세요:

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

이 코드는 자동으로 생성되는 테스트 모듈입니다. `cfg` 속성은 *환경 설정(configuration)* 을 의미하며,
러스트에게 뒤따르는 아이템이 특정한 환경 값에 대해서만 포함되어야 함을 말해줍니다. 위의 경우, 환경 값이
`test`인데, 테스트를 컴파일하고 실행하기 위해 러스트로부터 제공되는 것입니다. 이 속성을 이용함으로써,
카고는 우리가 능동적으로 `cargo test`를 이용해서 테스트를 실행시킬 경우에만 우리의 테스트 코드를
컴파일합니다. 이는 이 모듈 내에 있을지도 모를 어떠한 헬퍼 함수들, 추가적으로 `#[test]`라고
어노테이션 된 함수들을 포함합니다.

#### 비공개 함수 테스트하기

테스팅 커뮤니티 내에서 비공개 함수가 직접적으로 테스트되어야 하는지 혹은 그렇지 않은지에 대한 논쟁이
있었고, 다른 언어들은 비공개 함수를 테스트하는 것이 어렵거나 불가능하게 만들어두었습니다. 여러분이
어떤 테스트 이데올로기를 고수하는지와는 상관없이, 러스트의 비공개 규칙은 여러분이 비공개 함수를
테스트하도록 허용해줍니다. 비공개 함수 `internal_adder`가 있는 Listing 11-12 내의 코드를
고려해 보시죠:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

<span class="caption">Listing 11-12: 비공개 함수 테스트하기</span>

`internal_adder` 함수는 `pub`으로 표시되어 있지 않지만, 테스트가 그저 러스트 코드일 뿐이고
`tests` 모듈도 그냥 또 다른 모듈이기 때문에, `internal_adder`를 불러들여 호출하는 것이 그냥 되는
것을 주목하세요. 만약 여러분이 비공개 함수를 테스트해야 한다고 생각하지 않는다면, 러스트에서는 여러분이
그렇게 하도록 강제할 일은 없습니다.

### 통합 테스트

러스트에서 통합 테스트들은 완전히 여러분의 라이브러리 외부에 있습니다. 이들은 여러분의 라이브러리를
다른 코드들과 동일한 방식으로 이용하는데, 이는 이 외부 테스트들이 오직 여러분의 라이브러리의 공개
API 부분에 속하는 함수들만 호출할 수 있다는 의미입니다. 이들의 목적은 여러분의 라이브러리의 수많은
파트들이 함께 올바르게 동작하는지를 시험하는 것입니다. 그 자체로서는 올바르게 동작하는 코드의 단위들도
통합되었을 때는 문제를 일으킬 수 있으므로, 통합된 코드의 테스트 커버율 또한 중요합니다. 통합 테스트를
만들기 위해서는 *tests* 디렉토리를 먼저 만들 필요가 있습니다.

#### *tests* 디렉토리

프로젝트 디렉토리의 최상위, 그러니까 *src* 옆에 *tests* 디렉토리를 만듭니다.
카고는 이 디렉토리 내의 통합 테스트 파일들을 찾을 줄 압니다. 그런 후에는 이 디렉토리에 원하는
만큼 많은 테스트 파일을 만들 수 있으며, 카고는 각각의 파일들을 개별적인 크레이트처럼 컴파일할 것입니다.

한 번 통합 테스트를 만들어봅시다. Listing 11-12의 *src/lib.rs* 코드를 그대로 유지한 채로, *tests*
디렉토리를 만들고, *tests/integration_test.rs*라는 이름의 새 파일을 만든 다음, Listing 11-13의
코드를 집어넣으세요.

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

<span class="caption">Listing 11-13: `adder` 크레이트 내의 함수에 대한 통합 테스트
</span>

코드의 상단에 `extern crate adder`를 추가했는데, 이는 단위 테스트에서는 필요 없었지요. 이는 `tests`
디렉토리 내의 각 테스트가 모두 개별적인 크레이트이라서, 우리의 라이브러리를 각각에 가져올 필요가 있기
때문입니다.

*tests/integration_test.rs*에는 `#[cfg(test)]`를 이용한 어노테이션을 해줄 필요가
없습니다. 카고는 `test` 디렉토리를 특별 취급하여 `cargo test`를 실행시켰을 때에만 이 디렉토리
내의 파일들을 컴파일합니다. 이제 `cargo test` 실행을 시도해봅시다:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running target/debug/deps/adder-abcabcabc

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-ce99bcc2479f4607

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

출력에 세 개의 섹션이 생겼습니다: 단위 테스트, 통합 테스트, 그리고 문서 테스트입니다. 단위 테스트를
위한 첫 번째 섹션은 우리가 봐오던 것과 동일합니다: 각각의 단위 테스트마다 한 라인 (Listing 11-12에서
우리가 추가한 `intenal`이라는 이름의 것이 있었죠), 그다음 단위 테스트들의 정리 라인이 있습니다.

통합 테스트 섹션은 `Running target/debug/deps/integration-test-ce99bcc2479f4607`
이라고 말하는 라인과 함께 시작합니다 (여러분의 출력 값 끝의 해쉬값은 다를 것입니다). 그다음 이 통합
테스트 안의 각 테스트 함수를 위한 라인이 있고, `Doc-tests adder` 섹션이 시작되기 직전에
통합 테스트의 결과를 위한 정리 라인이 있습니다.

어떠한 *src* 파일에 단위 테스트 함수를 더 추가하는 것이 단위 테스트 섹션의 테스트 결과 라인을 더
늘린다는 점을 상기하세요. 통합 테스트 파일에 테스트 함수를 더 추가하는 것은 그 파일의 섹션의 라인을
더 늘릴 것입니다. 각 통합 테스트 파일은 고유의 섹션을 가지고 있으므로, 만일 우리가 *tests* 디렉토리에
파일을 더 추가하면, 통합 테스트 생선이 더 생길 것입니다.

`cargo test`의 인자로서 테스트 함수의 이름을 명시하는 식으로 특정 통합 테스트 함수를 실행시키는 것도
여전히 가능합니다. 특정한 통합 테스트 파일 내의 모든 테스트를 실행시키기 위해서는, `cargo test`에
파일 이름을 뒤에 붙인 `--test` 인자를 사용하세요:

```text
$ cargo test --test integration_test
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/integration_test-952a27e0126bb565

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

이 커맨드는 *tests/integration_test.rs* 내의 테스트만 실행합니다.

#### 통합 테스트 내의 서브모듈

더 많은 통합 테스트를 추가하게 되면, 이들을 조직화하기 쉽도록 *tests* 디렉토리 내에 하나 이상의
파일을 만들고 싶어 할지도 모릅니다; 예를 들면, 여러분은 이들이 테스트하는 기능별로 테스트 함수들을
묶을 수 있습니다. 앞서 언급했듯이, *tests* 디렉토리 내의 각 파일은 고유의 개별적인 크레이트인 것처럼
컴파일됩니다.

각 통합 테스트 파일을 고유한 크레이트인 것 처럼 다루는 것은 여러분의 크레이트를 이용하게 될
사용자들의 방식과 더 유사하게 분리된 스코프를 만들어 내기에 유용합니다. 하지만, 이는 *src* 내의
파일들이 동일한 동작을 공유하는 것을 *tests* 디렉토리 내의 파일들에서는 할 수 없음을 의미하는데,
이는 여러분이 7장에서 코드를 모듈과 파일로 나누는 법에 대해 배웠던 것입니다.

만일 여러분이 여러 개의 통합 테스트 파일들 내에서 유용하게 사용될 헬퍼 함수들 묶음을 가지고 있으며,
이들을 공통 모듈로 추출하기 위해 7장의 "모듈을 다른 파일로 옮기기"절에 있는 단계를 따르는 시도를 한다면,
이러한 *tests* 디렉토리 내의 파일에 대한 이색적인 동작 방식은 가장 주목할 만 점입니다. 이를테면, 만일
우리가 *tests/common.rs* 이라는 파일을 만들어서 그 안에 아래와 같이 `setup`이라는 이름의 함수를
위치시키고, 여기에 여러 테스트 파일들 내의 여러 테스트 함수로부터 호출될 수 있기를 원하는 어떤 코드를
집어넣는다면:

<span class="filename">Filename: tests/common.rs</span>

```rust
pub fn setup() {
    // 여러분의 라이브러리 테스트에 특화된 셋업 코드가 여기 올 것입니다
}
```

만약 테스트를 다시 실행시키면, 비록 이 코드가 어떠한 테스트 함수도 담고 있지 않고, `setup` 함수를
다른 어딘가에서 호출하고 있지 않을지라도, *common.rs* 파일을 위한 테스트 출력 내의 새로운 섹션을
보게 될 것입니다:

```text
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b8b07b6f1be2db70

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-d993c68b431d39df

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`running 0 tests`이 표시되는 테스트 출력이 보이는 `common`을 만드는 건 우리가 원하던 것이
아닙니다. 우리는 그저 다른 통합 테스트 파일들에서 어떤 코드를 공유할 수 있기를 원했지요.

`common`이 테스트 출력에 나타나는 것을 막기 위해서는, *tests/common.rs*을 만드는 대신,
*tests/common/mod.rs*를 만듭니다. 7장의 "모듈 파일 시스템의 규칙"절에서 서브모듈을 가지고 있는
모듈의 파일들을 위해 *module_name/mod.rs*라는 이름 규칙을 이용했었고, 여기서 `common`에 대한
서브모듈을 가조기 있지는 않지만, 이러한 방식으로 파일명을 정하는 것이 러스트에게 `common` 모듈을
통합 테스트 파일로 취급하지 않게끔 전달해줍니다. `setup` 함수 코드를 *tests/common/mod.rs*로
옮기고 *tests/common.rs* 파일을 제거하면, 테스트 출력에서 해당 섹션이 더 이상 나타나지 않을 것입니다.
*tests* 디렉토리의 서브 디렉토리 내의 파일들은 개별적인 크레이트처럼 컴파일되지도, 테스트 출력의
섹션을 갖지도 않습니다.

*tests/common/mod.rs*를 만든 뒤에는, 어떤 통합 테스트 파일에서라도 이를 모듈처럼 쓸 수 있습니다.
아래에 *tests/integration_test.rs* 내에 `it_adds_two` 테스트로부터 `setup` 함수를 호출하는
예제가 있습니다:

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

`mod common;` 선언은 Listing 7-4에서 보여주었던 모듈 선언과 동일한 점을 주목하세요. 그런 다음
테스트 함수 내에서 `common::setup()` 함수를 호출 할 수 있습니다.

#### 바이너리 크레이트를 위한 통합 테스트

만약 우리의 프로젝트가 *src/lib.rs* 파일이 없고 *src/main.rs* 파일만 갖고 있는 바이너리 프로젝트라면,
*tests* 디렉토리 내에 통합 테스트를 만들어서 *src/main.rs*에 정의된 함수를 가져오기 위하여
`extern crate`를 이용할 수 없습니다. 오직 라이브러리 크레이트만 다른 크레이트에서 호출하고 사용할
수 있는 함수들을 노출시킵니다; 바이너리 크레이트는 그 스스로 실행될 것으로 여겨집니다.

이는 바이너리를 제공하는 러스트 프로젝트들이 *src/lib.rs*에 위치한 로직을 호출하는 간단한 형태의
*src/main.rs*를 가지고 있는 이유 중 하나입니다. 이러한 구조와 함께라면, `extern crate`를
이용하여 중요한 기능들을 커버하도록 하기 위해 통합 테스트가 라이브러리 크레이트를 *테스트할 수 있습니다*.
만일 중요 기능이 작동한다면, *src/main.rs* 내의 소량의 코드 또한 동작할 것이고, 이 소량의 코드는
테스트할 필요가 없습니다.

## 정리

러스트의 테스트 기능은 코드를 변경하더라도 계속하여 우리가 기대한 대로 동작할 것이라는 확신을 주기 위하여
코드가 어떻게 기능하는지 명시하는 방법을 제공합니다. 단위 테스트는 라이브러리의 서로 다른 부분을 개별적으로
시험하며 비공개된 구현 세부사항을 테스트할 수 있습니다. 통합 테스트는 라이브러리의 많은 부분이 함께 작동하는
사용 상황을 다루며, 외부 코드가 사용하게 될 똑같은 방식대로 테스트하기 위해 그 라이브러리의 공개 API를
이용합니다. 비록 러스트의 타입 시스템과 소유권 규칙이 몇 가지 종류의 버그를 방지하는데 도움을 줄지라도,
테스트는 여러분의 코드가 어떻게 동작하기를 기대하는지와 함께 해야 하는 논리 버그를 잡는 일을 도와주는 데에
있어 여전히 중요합니다.

이 장과 이전 장들의 지식을 합쳐서 다음 장의 프로젝트 작업을 해봅시다!
