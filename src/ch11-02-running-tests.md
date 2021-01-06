## 테스트 실행 제어하기

`cargo run` 명령어가 여러분의 코드를 컴파일하고 만들어진 바이너리를 실행하는 것과 마찬가지로,
`cargo test` 명령어는 여러분의 코드를 테스트 모드에서 컴파일하고 만들어진 바이너리를 실행합니다.
`cargo test` 명령어에 옵션을 지정하면 기본 동작을 변경할 수 있습니다.
기본 동작의 예시로는, `cargo test` 명령어로 생성된 바이너리가
테스트 결과 관련 출력을 읽기 쉽게 만들기 위해,
모든 테스트를 병렬로 실행하는 동안 테스트의 출력을 캡처해두고
화면 표시를 막는 것이 있습니다.

명령어 옵션은 `cargo test` 에 전달되는 것도 있고,
테스트 바이너리에 전달되는 것도 있습니다.
이 둘을 구분하기 위해 `cargo test` 에 전달할 인자를 먼저 나열하고,
`--` 구분자(separator)를 작성하고, 그 이후 테스트 바이너리에 전달할 인자를 나열합니다.
`cargo test --help` 명령어는 `cargo test` 명령어에 사용 가능한 옵션을 표시하고,
`cargo test -- --help` 명령어는 `--` 구분자 이후에 사용 가능한 옵션을 표시합니다.

### 테스트를 병렬 혹은 순차적으로 실행하기

여러 테스트를 실행할 때,
기본적으로는 스레드를 사용해 병렬 실행됩니다.
테스트를 더 빨리 끝내서 코드 작동 여부를 빠르게 보고하기 위함입니다.
하지만 여러 테스트가 동시에 실행되므로,
각 테스트가 공유 상태(공유 자원, 현재 작업 디렉토리, 환경 변수 등)를 갖거나
다른 테스트에 의존해서는 안 됩니다.

예시를 생각해보죠. 여러분은 각각의 테스트가 *test-output.txt* 파일을 생성하고
어떤 데이터를 작성하는 코드를 실행하도록 만들었습니다.
각 테스트는 파일 내 데이터를 읽고, 파일이 특정 값을 포함하고 있는지 확인하며,
특정 값은 테스트마다 다릅니다.
여러 테스트가 동시에 실행되므로, 어떤 테스트가 파일에 작성하고 읽는 사이
다른 테스트가 파일의 내용을 덮어쓸 수도 있습니다. 이 경우 방해받은 테스트는 실패할 겁니다.
코드에 문제가 있어서가 아니라, 병렬 실행되는 도중 방해받아서 말이죠.
한 가지 해결책은 각 테스트가 서로 다른 파일에 작성하도록 만드는 것일 테고,
다른 해결책은 테스트를 한 번에 하나씩 실행하는 것입니다.

테스트를 병렬로 실행하지 않거나,
사용할 스레드의 개수를 섬세히 조절해야 할 때에는
`--test-threads` 플래그와 함께 테스트 바이너리에서 사용할 스레드 개수를 지정할 수 있습니다.
다음과 같이 사용합니다.

```console
$ cargo test -- --test-threads=1
```

스레드 개수를 `1` 로 설정하여 프로그램이
어떠한 병렬 처리도 사용하지 않도록 하였습니다.
스레드 하나만 사용해 테스트를 실행하면 병렬 실행에 비해 더 느려지겠지만,
서로 상태를 공유하는 테스트가 방해받을 일이 사라집니다.

### 함수 출력 표시하기

기본적으로, 러스트 테스트 라이브러리는 성공한 테스트의 모든 표준 출력(standard output)을 캡쳐합니다.
테스트에서 `println!` 매크로를 호출해도, 해당 테스트가 성공하면
터미널에서 `println!` 의 출력을 찾아볼 수 없습니다.
해당 테스트가 성공했다고 표시된 줄만 볼 수 있죠.
테스트가 실패하면 표준 출력으로 출력됐던 모든 내용이 실패 메세지 아래에 표시됩니다.

Listing 11-10은 매개변수를 출력하고 10을 반환하는 단순한 함수와,
성공하는 테스트와 실패하는 테스트를 작성한 예시입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

<span class="caption">Listing 11-10: `println!` 을 호출하는
함수 테스트</span>

`cargo test` 명령어를 실행하면 다음 결과가 나타납니다.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

성공한 테스트에서 출력했던
`I got the value 4` 은 캡쳐되었으므로 찾아볼 수 없습니다.
실패한 테스트에서 출력한 `I got the value 8` 는 테스트 실패 원인과 함께
테스트 출력 요약 절에 나타납니다.

성공한 테스트에서 출력한 내용도 보고 싶다면, `--show-output` 옵션을 이용해
성공한 테스트의 출력도 표시하도록 러스트에게 전달할 수 있습니다.

```console
$ cargo test -- --show-output
```

Listing 11-10의 테스트를 `--show-output` 플래그로 실행한 결과는
다음과 같습니다.

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### 이름을 지정해 일부 테스트만 실행하기

간혹 테스트 모음을 전부 실행하는 데 시간이 오래 걸리기도 합니다.
코드의 특정 영역에서 작업 중이라면 해당 영역에 관련된 테스트만 실행하고 싶을지도 모르죠.
`cargo test` 명령어에 테스트의 이름을 인자로 넘겨
어떤 테스트를 실행할지 선택할 수 있습니다.

일부 테스트만 실행하는 법을 알아보기 위해, Listing 11-11처럼
`add_two` 함수에 대한 세 가지 테스트를 작성하고 하나만 골라 실행해보겠습니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

<span class="caption">Listing 11-11: 세 가지 서로 다른 이름의
테스트</span>

앞서 살펴본 것처럼, 테스트를 아무 인자도 없이 실행하면
모든 테스트가 병렬로 실행됩니다.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### 테스트 하나만 실행하기

`cargo test` 명령어에 테스트 함수 이름을 전달하여 해당 테스트만 실행할 수 있습니다.

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

`one_hundred` 테스트만 실행되었습니다. 나머지 두 테스트는 이름이 맞지 않았습니다.
테스트 결과는 마지막 요약 줄에서 `2 filtered out` 을 표시하여,
우리가 이 명령어로 실행한 테스트 이외에도 다른 테스트가 존재함을 알려줍니다.

이 방법으로 여러 테스트의 이름을 명시할 수는 없습니다. `cargo test` 명령어는 첫 번째 값만 사용합니다.
하지만 여러 테스트를 실행하는 방법이 없지는 않습니다.

#### 테스트를 필터링하여 여러 테스트 실행하기

테스트 이름의 일부만 지정하면 해당 값에 맞는 모든 테스트가 실행됩니다.
예를 들어, `cargo test add` 명령어를 실행하면 우리가 작성한 세 개의 테스트 중
`add` 가 포함된 두 개가 실행됩니다.

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

이 명령어는 `add` 가 이름에 포함된 모든 테스트를 실행하고,
`one_hundred` 테스트를 필터링했습니다.
테스트가 위치한 모듈도 테스트 이름의 일부로 나타나는 점을 기억해두세요.
모듈 이름으로 필터링하면 해당 모듈 내 모든 테스트를 실행할 수 있습니다.

### 따로 지정하지 않는 한 일부 테스트 무시하기

간혹 몇몇 특정 테스트는 실행하는데 굉장히 오랜 시간이 걸려서,
`cargo test` 실행 시 이런 테스트는 제외하고 싶어질 수 있습니다.
여러분은 실행할 모든 테스트를 인자로 열거할 필요 없이,
시간이 오래 걸리는 테스트에 `ignore` 속성을 어노테이션하면
됩니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs}}
```

제외할 테스트의 `#[test]` 다음 줄에 `#[ignore]` 줄을 추가했습니다.
이제 테스트를 실행하면 `it_works` 테스트는 실행되지만, `expensive_test` 테스트는 실행되지 않습니다.

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

`expensive_test` 테스트는 `ignored` 로 표시되었습니다.
`cargo test -- --ignored` 명령어를 사용하면 무시된 테스트만 실행할 수 있습니다.

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

실행할 테스트를 선별하여 `cargo test` 결과는 빨리 확인할 수 있습니다.
무시한 테스트의 결과를 확인해야 할 때가 되었고
그 결과를 기다릴 시간이 있다면,
`cargo test -- --ignored` 명령어를 실행합니다.
