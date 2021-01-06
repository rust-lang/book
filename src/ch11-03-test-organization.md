## 테스트 조직화

이번 장의 시작 부분에서 언급했듯, 테스트는 복잡한 분야입니다.
사람들은 저마다 다른 용어와 구조를 사용합니다.
러스트 커뮤니티는 테스트를 크게 *유닛 테스트(unit test, 단위 테스트라고도 함)*, *통합 테스트(integration test)* 두 종류로 나눕니다.
유닛 테스트는 작고 더 집중적입니다.
한 번에 하나의 모듈만 테스트하며, 모듈의 비공개 인터페이스도 테스트할 수 있습니다.
통합 테스트는 완전히 라이브러리 외부에 위치하며,
따라서 여러분이 작성한 라이브러리를 외부 코드에서 사용할 때와 똑같은 방식을 사용합니다.
하나의 테스트에서 잠재적으로 여러 모듈이 사용되기도 합니다.

여러분의 라이브러리가 각 부분이 따로따로 사용될 때도,
함께 사용될 때도 제대로 작동할 것을 보장하려면 두 종류의 테스트 모두 작성해야 합니다.

### 유닛 테스트

유닛 테스트의 목적은 각 코드 단위를 나머지 코드와 분리하여,
제대로 작동하지 않는 코드가 어느 부분인지 빠르게 파악하는 것입니다.
유닛 테스트는 *src* 디렉토리 내의 각 파일에
테스트 대상이 될 코드와 함께 작성합니다.
각 파일에 `tests` 모듈을 만들고 `cfg(test)` 를 어노테이션하는 게
일반적인 관례입니다.

#### 테스트 모듈과 `#[cfg(test)]`

테스트 모듈에 어노테이션하는 `#[cfg(test)]` 은 러스트에게 이 코드는
`cargo build` 명령어가 아니라 `cargo test` 명령어 실행 시에만 컴파일 및 실행하라는 것을 전달합니다.
라이브러리 빌드 시 테스트 코드는 제외되므로, 컴파일 소요 시간이 짧아지고,
컴파일 결과물 크기도 줄어듭니다.
이후에 알게 되겠지만, 통합 테스트는 별도의 디렉토리에 위치하기 때문에
`#[cfg(test)]` 어노테이션이 필요 없습니다.
하지만 유닛 테스트는 일반 코드와 같은 파일에 위치하기 때문에,
`#[cfg(test)]` 어노테이션을 작성해 컴파일 결과물에 포함되지 않도록 명시해야 합니다.

이번 장 첫 번째 절에서 `adder` 프로젝트를 생성했을 때
Cargo가 생성했던 코드를 다시 살펴봅시다.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

이 코드는 자동으로 생성된 테스트 모듈입니다.
`cfg` 속성은 *설정(configuration)* 을 의미하며,
러스트는 이 요소를 특정 설정 옵션 적용 시에만 포함합니다.
이 경우 옵션 값은 러스트에서 테스트를 컴파일, 실행하기 위해 제공하는 `test` 입니다.
`cfg` 속성을 사용하면 Cargo는 `cargo test` 명령어를
실행할 때만 테스트 코드를 컴파일합니다.
여기에는 `#[test]` 어노테이션된 함수뿐만 아니라
모듈 내 도우미 함수도 포함됩니다.

#### 비공개 함수 테스트하기

비공개 함수도 직접 테스트해야 하는지에 대해서는 많은 논쟁이 있습니다.
다른 언어에서는 비공개 함수를 테스트하기 어렵거나, 불가능하게 만들어두었습니다.
여러분의 테스트 철학이 어떤지는 모르겠지만, 러스트의 프라이버시 규칙은
비공개 함수를 테스트하도록 허용합니다.
Listing 11-12는 비공개 함수 `internal_adder` 를 보여줍니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

<span class="caption">Listing 11-12: 비공개 함수 테스트하기</span>

`internal_adder` 함수는 `pub` 으로 표시되지 않았지만,
테스트도 러스트 코드이며 `tests` 모듈도 그저 또 다른 모듈이므로,
테스트 스코프에 `internal_adder` 를 가져와 호출할 수 있습니다.
혹시 여러분이 비공개 함수를 테스트해서는 안 된다는 주의라면,
러스트는 여러분에게 강요하지 않습니다.

### 통합 테스트

통합 테스트는 여러분이 만든 라이브러리와 완전히 분리되어있습니다.
통합 테스트는 외부 코드와 마찬가지로, 여러분이 만든 라이브러리의 공개 API만 호출 가능합니다.
통합 테스트의 목적은 라이브러리의 수많은 부분을 함께 사용했을 때
제대로 작동하는지 확인하는 것입니다.
각각 따로 사용했을 땐 잘 작동하는 코드도 함께 사용할 땐
문제가 발생할 수 있기 때문에 통합 테스트도 중요합니다.
통합 테스트를 작성하려면 먼저 *tests* 디렉토리를 만들어야 합니다.

#### *tests* 디렉토리

프로젝트 디렉토리 최상위, 다시말해 *src* 옆에 *tests* 디렉토리를 생성합니다.
Cargo는 디렉토리 내 통합 테스트 파일을 자동으로 인식합니다.
통합 테스트는 *tests* 디렉토리 내에 원하는 만큼 작성할 수 있으며,
Cargo는 각 파일을 각각의 크레이트로 컴파일합니다.

통합 테스트를 직접 만들어보죠. Listing 11-12 코드를 *src/lib.rs* 에 작성한 채로
*tests* 디렉토리를 만들고, *tests/integration_test.rs* 파일을 생성해
Listing 11-13 코드를 작성해봅시다.

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

<span class="caption">Listing 11-13: `adder` 크레이트 내 함수를 테스트하는
통합 테스트</span>

코드 맨 위에 `use adder` 를 추가했습니다.
유닛 테스트에서는 이 코드가 필요하지 않지만 *tests* 디렉토리 내 파일은 별도의 크레이트이므로,
테스트 크레이트 스코프에 우리가 만든 라이브러리를 가져와야 합니다.

*tests/integration_test.rs* 내 코드는 `#[cfg(test)]` 가 필요 없습니다.
Cargo는 `tests` 디렉토리를 특별 취급하여, 디렉토리 내 파일을 `cargo test` 시에만 컴파일합니다.
`cargo test` 를 다시 실행시켜보죠.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

출력에 유닛 테스트, 통합 테스트, 문서 테스트 세 가지 절이 만들어졌네요.
첫 번째 절인 유닛 테스트는 앞서 본 것과 같습니다.
유닛 테스트가 한 줄씩(`internal` 은 Listing 11-12 에서 추가했었습니다)
출력되고, 유닛 테스트 결과 요약 줄이 출력됩니다.

통합 테스트 절은 `Running target/debug/deps/integration_test-ce99bcc2479f4607`
줄로 시작합니다(여러분은 출력 끝의 해쉬값이 다를 겁니다).
그다음 통합 테스트 내 각각의 테스트 함수가 한 줄씩 출력되고,
통합 테스트 결과 요약은 `Doc-tests adder` 절이 시작하기 직전에
출력됩니다.

유닛 테스트를 추가하면 유닛 테스트 출력 절이 한 줄씩 늘어나는 것처럼,
통합 테스트 파일에 테스트 함수를 추가하면
해당 통합 테스트 파일의 출력 절이 한 줄씩 늘어납니다.
각각의 통합 테스트 파일은 별도의 출력 절을 생성합니다.
즉, *tests* 디렉토리에 파일을 추가하면 여러 통합 테스트 절이 만들어집니다.

통합 테스트도 마찬가지로 `cargo test` 명령어에 테스트 함수명을
인자로 전달해 특정 통합 테스트 함수를 실행할 수 있습니다.
특정 통합 테스트 파일의 모든 테스트를 실행하려면,
`cargo test` 명령어에 `--test` 인자로 파일명을 전달하면 됩니다.

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

이 명령어는 *tests/integration_test.rs* 파일 내의 테스트만 실행합니다.

#### 통합 테스트 내 서브 모듈

통합 테스트를 추가하다 보면, 조직화를 위해 *tests* 디렉토리에
하나 이상의 파일이 필요할 수도 있습니다.
예를 들어, 테스트 함수가 테스트하는 기능별로 그룹화할 수도 있죠.
앞서 말했듯, *tests* 내 각 파일은 각각의 크레이트로 컴파일됩니다.

여러분이 만든 크레이트를 사용할 실제 사용자처럼 분리된 스코프를
만들어내는 데에는 각 통합 테스트 파일이 각각의 크레이트로 취급된다는 점이 유용합니다.
하지만 이는 7장에서 배운대로 *src* 디렉토리에서 코드를 모듈, 파일로 분리하여
파일 간 동일한 동작을 공유하는 것을 *tests* 디렉토리 내 파일에서는 할 수 없음을
의미합니다.

여러 통합 테스트 파일에서
유용하게 사용할 도우미 함수 묶음을
7장 [“Separating Modules into Different Files”][separating-modules-into-files]<!-- ignore -->
과정대로 공통 모듈로 분리하려 할 때,
*tests* 디렉토리 내 파일의 동작 방식은 걸림돌이 됩니다.
예를 들어, 우리가 *tests/common.rs* 파일을 생성하고,
여러 테스트 파일 내 함수에서 호출할 `setup` 함수를 작성한다고
가정해봅시다.

<span class="filename">Filename: tests/common.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

이제 테스트를 실행하면, 결과 출력에 새로운 절이 *common.rs* 파일 때문에
생성된 모습을 볼 수 있습니다. *common.rs* 파일은 어떤 테스트 함수도 담고 있지 않고,
다른 곳에서 `setup` 함수를 호출하지도 않았는데 말이죠.

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

우리가 원했던 건 다른 통합 테스트 파일과 일부 코드를 공유하는 것이지,
테스트 출력 결과에 `common` 과 `running 0 tests` 이 출력되는 게
아니었죠.

테스트 출력 결과에서 `common` 을 제외하려면 *tests/common.rs* 파일 대신
*tests/common/mod.rs* 파일을 생성해야 합니다.
이는 러스트에서 사용 가능한 대체 명명 규칙(alternate naming convention)입니다.
러스트는 이 파일명 규칙을 따르는 파일은 통합 테스트 파일로 취급하지 않습니다.
`setup` 함수를 *tests/common/mod.rs* 파일로 옮기고 *tests/common.rs* 파일을 삭제하면
더 이상 테스트 결과 출력에 `common` 이 나타나지 않습니다.
*tests* 디렉토리의 서브 디렉토리 내 파일은 별도 크레이트로 컴파일되지 않고,
테스트 결과 출력에서 별도의 출력 절이 생성되지도 않습니다.

*tests/common/mod.rs* 파일을 생성하고 나면 다른 통합 테스트 파일에서 모듈처럼 사용할 수 있습니다.
다음은 *tests/integration_test.rs* 파일 내 `it_adds_two` 테스트에서 `setup` 함수를
호출하는 예시입니다.

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

Listing 7-21에서 배운 모듈 선언대로 `mod common;` 를 선언했습니다.
선언하고 나면 `common::setup()` 함수를 호출할 수
있습니다.

#### 바이너리 크레이트에서의 통합 테스트

*src/lib.rs* 파일이 없고 *src/main.rs* 파일만 있는 바이너리 크레이트라면,
*tests* 디렉토리에 통합 테스트를 만들어서 *src/main.rs* 파일에 정의된 함수를
`use` 구문으로 가져올 수 없습니다.
다른 크레이트에서 사용할 수 있도록 함수를 노출하는 건 라이브러리 크레이트 뿐입니다.
바이너리 크레이트는 자체적으로 실행되게 되어있습니다.

바이너리를 제공하는 러스트 프로젝트들이
*src/main.rs* 파일은 간단하게 작성하고,
로직은 *src/lib.rs* 파일에 위치시키는 이유 중 하나가 이 때문입니다.
이런 구조로 작성하면 중요 기능을 통합 테스트에서
`use` 구문으로 가져와 *테스트 할 수 있습니다.*
중요 기능이 제대로 작동하면 *src/main.rs* 파일 내 소량의 코드도 작동할 테니,
이 소량의 코드는 테스트하지 않아도 됩니다.

## 정리

러스트의 테스트 기능을 사용하면 코드가 어떻게 작동해야 하는지 명시하여,
코드를 변경하더라도 계속하여 우리 의도대로 작동함을 보장할 수 있습니다.
유닛 테스트는 비공개 세부 구현을 포함한 라이브러리의 각 부분이
따로따로 잘 작동하는지 확인합니다.
통합 테스트는 외부 코드가 라이브러리를 사용하는 방식과 동일하게
라이브러리 공개 API를 이용해 라이브러리의 여러 부분이 함께 사용될 때 제대로 작동하는지 확인합니다.
러스트의 타입 시스템과 소유권 규칙이 일부 버그를 방지해주긴 하지만,
여러분이 작성한 코드가 의도대로 작동하지 않는 논리 버그를 제거하려면 테스트도 마찬가지로 중요합니다.

이번 장에서 배운 지식과 앞서 배워온 지식을 합쳐서 프로젝트를
한번 진행해 보죠!

[separating-modules-into-files]:
ch07-05-separating-modules-into-different-files.html
