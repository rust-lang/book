## `Result`와 함께하는 복구 가능한 에러

대부분 에러는 프로그램을 전부 중단해야 할 정도로 심각하진 않습니다.
때때로 어떤 함수가 실패할 때는, 여러분이 쉽게 해석하고 대응할 수 있는 원인 때문입니다.
예를 들어, 어떤 파일을 열려고 했는데 해당 파일이 존재하지 않아서 실패했다면,
프로세스를 종료해버리는 대신 파일을 생성하는 것을
원할지도 모르죠.

2장의 ["`Result` 타입으로 잠재된 실패 다루기"][handle_failure]<!-- ignore --> 절에서
`Result` 열거형은 다음과 같이 `Ok`와 `Err`라는 두 개의 variant를 갖도록 정의되어 있음을
상기하세요:

[handle_failure]: ch02-00-guessing-game-tutorial.html#result-%ED%83%80%EC%9E%85%EC%9C%BC%EB%A1%9C-%EC%9E%A0%EC%9E%AC%EB%90%9C-%EC%8B%A4%ED%8C%A8-%EB%8B%A4%EB%A3%A8%EA%B8%B0

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T`와 `E`는 제네릭 타입 파라미터입니다.
제네릭은 10장에서 자세히 다룰 예정입니다.
지금 당장은, `T`는 성공한 경우에 `Ok` variant 내에 반환될 값의 타입을
나타내고 `E`는 실패한 경우에 `Err` variant 내에 반환될 에러의 타입을
나타낸다는 점만 알아둡시다.
`Result`가 이러한 제네릭 타입 파라미터를 갖기 때문에,
우리가 반환하고자 하는 성공적인 값과 에러값이 다를 수 있는 다양한 상황 내에서
표준 라이브러리에 정의된 `Result` 타입과 함수들을 사용할 수 있습니다.

실패할 가능성이 있어서 `Result` 값을 반환하는 함수를 한번 호출해 봅시다.
Listing 9-3는 파일을 열어보는 코드입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">Listing 9-3: 파일 열기</span>

`File::open`이 `Result`를 반환하는지는 어떻게 알 수 있을까요?
표준 라이브러리 API 문서를 찾아보거나, 컴파일러에게 물어볼 수 있습니다!
그 함수의 반환 타입이 *아닐 거라* 확신하는 타입을 f에 명시하고 컴파일하면,
컴파일러는 해당 타입이 맞지 않는다는 것과 `f`가 어떤 타입인지 알려줄 겁니다.
한번 해봅시다. 우리는 `File::open`의 반환 타입이 적어도 `u32`는
아니라는 것을 알고 있으니, `let f` 구문을 이렇게
바꿔봅시다:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/src/main.rs:here}}
```

이제 컴파일을 시도하면 다음 메세지가 나타납니다:

```console
{{#include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/output.txt}}
```

이 메세지는 `File::open` 함수의 반환 타입이 `Result<T, E>`라는 것을 알려줍니다.
여기서 제네릭 파라미터 `T`는 성공 값의 타입인 `std::fs::File`로 채워져 있는데,
이것은 파일 핸들입니다.
에러값으로 사용되는 `E`의 타입은 `std::io::Error`입니다.

이 반환 타입은 `File::open`을 호출하는 것이 성공하여 우리가 읽거나 쓸 수 있는
파일 핸들을 반환해 줄 수도 있다는 뜻입니다. 함수 호출은 파일이 존재하지 않거나
파일에 접근할 권한이 없을 경우 실패할 수도 있습니다.
`File::open` 함수는 우리에게 성공했는지 혹은 실패했는지를 알려주면서
동시에 파일 핸들이나 에러 정보 둘 중 하나를 우리에게 제공할 방법을
가질 필요가 있습니다. 이러한 정보는 `Result` 열거형으로 전달할 수 있는
데이터와 정확히 일치합니다.

`File::open`이 성공한 경우, 변수 `f`가 가지게 될 값은
파일 핸들을 담고 있는 `Ok` 인스턴스가 될 것입니다.
실패한 경우, `f`의 값은 발생한 에러의 종류에 대한 더 많은 정보를
가지고 있는 `Err`의 인스턴스가 될 것입니다.

Listing 9-3 코드에 `File::open` 반환 값에 따라
다르게 작동하는 코드를 추가해봅시다.
Listing 9-4은 6장에서 다뤘던 `match` 표현식을 이용하여
`Result`를 처리하는 한 가지 방법을 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">Listing 9-4: 반환될 수 있는 `Result` variant들을 `match`
표현식으로 처리하기</span>

`Option` 열거형과 같이 `Result` 열거형과 variant들은 프렐루드(prelude)로부터
가져와 진다는 점을 기억하세요. 따라서 `Ok`와 `Err` 앞에 `Result::`를 특정하지 않아도
됩니다.

여기서 우리는 결과가 `Ok`일 때에는
`Ok` variant 내부의 `file` 값을 반환하고,
이 파일 핸들 값을 변수 `f`에 대입하라고 러스트에게 말해주고 있습니다.
`match` 이후에는 이 파일 핸들을 읽거나 쓰는 데에 사용할 수 있습니다.

`match`의 다른 경우는 `File::open`으로부터 `Err`를 얻은 경우를 처리합니다.
이 예제에서는 `panic!` 매크로를 호출하는 방법을 택했습니다.
디렉토리 내에 *hello.txt*라는 이름의 파일이 없는 경우에 이 코드를 실행하면,
`panic!` 매크로로부터 다음과 같은 출력을 보게 될 것입니다:

```text
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

여태 그래왔듯, 이 출력은 어떤 것이 잘못되었는지 정확하게 알려줍니다.

### 서로 다른 에러에 대해 매칭하기

Listing 9-4의 코드는 `File::open`이 실패한 원인이
무엇이든 간에 `panic!`을 일으킵니다.
어떠한 이유로 실패했느냐에 따라 다르게 작동하도록 만들어 봅시다.
파일이 없어서 `File::open`이 실패했다면 새로운 파일을 만들어서 핸들을 반환하고,
그 밖의 이유로(파일을 열 권한이 없다거나) 실패했다면
Listing 9-4처럼 `panic!`을 일으키도록 말이죠.
`match`에 내용을 추가한 Listing 9-5를 살펴봅시다:

<span class="filename">Filename: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">Listing 9-5: 다른 종류의 에러를
다른 방식으로 처리하기</span>

`Err` variant 내에 있는 `File::open`이 반환하는 값의 타입은 `io::Error`인데,
이는 표준 라이브러리에서 제공하는 구조체입니다.
이 구조체는 `kind` 메소드를 제공하는데 이를 호출하여 `io::ErrorKind`값을 얻을 수 있습니다.
`io::ErrorKind`는 `io` 연산으로부터 발생할 수 있는 여러 종류의 에러를 표현하는 variant를 가진,
표준 라이브러리에서 제공하는 열거형입니다.
우리가 사용하고자 하는 variant는 `ErrorKind::NotFound`인데, 이는 열고자
하는 파일이 아직 존재하지 않음을 나타냅니다. 따라서 이를 `f`와 매치시켰습니다.
그런데 `error.kind()` 내부에 매치가 하나 더 있군요.

내부 매치에서는 `error.kind()`가 반환한 값이 `ErrorKind` 열거형의
`NotFound` variant가 맞는지 확인하고, 맞다면 `File::create`로 파일을 생성합니다.
하지만 `File::create`도 실패할 수 있으니, 내부 `match` 표현식의
두 번째 갈래 또한 작성해야 합니다.
파일을 생성하지 못한 경우에는 별도의 오류 메세지가 출력됩니다.
외부 `match`의 두 번째 갈래 또한 동일 하므로,
파일을 찾을 수 없는 오류인 경우 외에는 모두 패닉이 발생합니다.

`match`가 정말 많군요! `match` 표현식은 매우 유용하지만 굉장히 원시적이기도 합니다.
13장에서는 클로저에 대해서 배워볼 텐데, `Result<T, E>` 타입에는
클로저를 사용하는 여러 메소드가 있으며, 이는 `match` 표현식을 이용해 구현됐습니다.
이 메소드들로 여러분의 코드를 더 간결하게 만들 수 있습니다.
러스트를 오래 사용한 사람들은 Listing 9-5 대신 다음과 같이 코드를 작성할 겁니다:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-03-closures/src/main.rs}}
```

이 코드는 Listing 9-5와 완벽히 동일하게 작동하지만,
`match` 표현식을 전혀 사용하지 않았으며 가독성도 더 뛰어납니다.
13장을 읽고 이 예제로 돌아와서, `unwrap_or_else` 메소드를 표준 라이브러리 문서에서 찾아보세요.
여러분이 오류를 다룰 때 이런 메소드들을 사용하면 거대하게 중첩된
`match` 표현식 덩어리를 지우고 코드를 깔끔하게 만들 수 있습니다.

### 에러가 났을 때 패닉을 위한 숏컷: `unwrap`과 `expect`

`match`의 사용은 충분히 잘 동작하지만, 살짝 장황하기도 하고 의도를 항상 잘 전달하는 것도 아닙니다.
`Result<T, E>` 타입은 다양한 작업을 하기 위해 정의된 수많은 헬퍼 메소드를 가지고 있습니다.
그 중 하나인 `unwrap` 이라 부르는 메소드는 Listing 9-4에서 작성한 `match` 구문과
비슷한 구현을 한 숏컷 메소드입니다. 만일 `Result` 값이 `Ok` variant라면,
`unwrap`은 `Ok` 내의 값을 반환할 것입니다. 만일 `Result`가 `Err` variant라면,
`unwrap`은 우리를 위해 `panic!` 매크로를 호출할 것입니다.
아래에 `unwrap`이 작동하는 예가 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

*hello.txt* 파일이 없는 상태에서 이 코드를 실행시키면, `unwrap` 메소드에 의한 `panic!`
호출로부터의 에러 메세지를 보게 될 것입니다:

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

또 다른 메소드인 `expect`는 `unwrap`과 유사한데, 우리가 `panic!` 에러 메세지를
선택할 수 있게 해줍니다. `unwrap` 대신 `expect`를 이용하고 좋은 에러 메세지를
제공하는 것은 여러분의 의도를 전달해주고 패닉의 근원을 추적하는 걸 쉽게 해 줄 수 있습니다.
`expect`의 문법은 아래와 같이 생겼습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

`unwrap`과 똑같이 파일 핸들을 반환하거나 `panic!` 매크로를 호출하도록 하는 데에
`expect`를 사용했습니다. `unwrap`은 `panic!`의 기본 메세지가 출력되지만,
`expect`는 매개변수로 넘긴 메세지를 에러 메세지로 출력합니다.
나타나는 모습은 다음과 같습니다.

```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

이 에러 메세지는 우리가 직접 명시한 텍스트인 `Failed to open hello.txt`로 시작하기 때문에,
이 에러 메세지가 어디서부터 왔는지를 코드 내에서 찾기가 더 수월해질 것입니다.
만일 우리가 여러 군데에 `unwrap`을 사용하면, 정확히 어떤 `unwrap`이 패닉을
일으켰는지 찾기에 좀 더 많은 시간이 걸릴 수 있는데, 그 이유는 패닉을
호출하는 모든 `unwrap`이 동일한 메세지를 출력하기 때문입니다.

### 에러 전파하기

실패할지도 모르는 무언가를 호출하는 구현을 가진 함수를 작성할 때,
이 함수 내에서 에러를 처리하는 대신, 에러를 호출하는 코드 쪽으로 반환하여
그쪽에서 어떻게 할지 결정하도록 할 수 있습니다.
이는 에러 *전파하기*로 알려져 있으며, 에러가 어떻게 처리해야 좋을지 좌우해야 할 상황에서,
여러분의 코드 내용 내에서 이용 가능한 것들보다 더 많은 정보와 로직을
가지고 있을 수도 있는 호출하는 코드 쪽에 더 많은 제어권을 줍니다.

예를 들면, Listing 9-6는 파일로부터 사용자 이름을 읽는 함수를 작성한 것입니다.
만일 파일이 존재하지 않거나 읽을 수 없다면, 이 함수는 호출하는 코드 쪽으로
해당 에러를 반환할 것입니다:

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">Listing 9-6: `match`를 이용하여 호출 코드 쪽으로
에러를 반환하는 함수</span>

이 함수는 더 간결하게 작성할 수 있지만, 에러 처리를 배우기 위해
과정을 하나씩 직접 작성해보고, 간결한 버전은 마지막에 살펴보도록 하겠습니다.
함수의 반환 타입인 `Result<String, io::Error>` 부터 먼저 살펴봅시다.
이는 함수가 `Result<T, E>` 타입의 값을 반환하는데
제네릭 파라미터 `T`는 구체적 타입(concrete type)인 `String`로 채워져 있고,
제네릭 타입 `E`는 구체적 타입인 `io::Error`로 채워져 있다는 의미입니다.
만일 이 함수가 어떤 문제 없이 성공하면,
함수를 호출한 코드는 `String`을 담은 값
(이 함수가 파일로부터 읽어 들인 사용자 이름이겠지요)
을 받을 것입니다.
만일 어떤 문제가 발생한다면, 이 함수를 호출한 코드는 문제가 무엇이었는지에 대한 더 많은
정보를 담고 있는 `io::Error`의 인스턴스를 담은 `Err` 값을 받을 것입니다.
이 함수의 반환 타입으로서 `io::Error`를 선택했는데,
그 이유는 우리가 이 함수 내부에서 호출하고 있는
실패 가능한 연산 두 가지가 모두 이 타입의 에러 값을 반환하기 때문입니다:
`File::open` 함수와 `read_to_string` 메소드 말이죠.

함수의 본체는 `File::open` 함수를 호출하면서 시작합니다.
그다음에는 Listing 9-4에서 본 `match`와 유사한 식으로
`match`을 이용해서 `Result` 값을 처리하는데, `Err` 경우에
`panic!`을 호출하는 대신 이 함수를 일찍 끝내고 `File::open`으로부터의
에러 값을 마치 이 함수의 에러 값인것처럼 호출하는 쪽의 코드에게 전달합니다.
만일 `File::open`이 성공하면, 파일 핸들을 `f`에 저장하고 계속합니다.

그 뒤 변수 `s`에 새로운 `String`을 생성하고 파일의 콘텐츠를 읽어 `s`에 넣기 위해
`f`에 있는 파일 핸들의 `read_to_string` 메소드를 호출합니다. `File::open`가
성공하더라도 `read_to_string` 메소드가 실패할 수 있기 때문에 이 함수 또한 `Result`를 반환합니다.
따라서 이 `Result`를 처리하기 위해서 또 다른 `match`가 필요합니다:
만일 `read_to_string`이 성공하면, 우리의 함수가 성공한 것이고,
이제 `s` 안에 있는 파일로부터 읽어 들인 사용자 이름을 `Ok`에 싸서 반환합니다.
만일 `read_to_string`이 실패하면, `File::open`의 반환 값을 처리했던 `match`에서 에러값을
반환하는 것과 같은 방식으로 에러 값을 반환합니다.
하지만 여기서는 명시적으로 `return`이라 말할 필요는 없는데,
그 이유는 이 함수의 마지막 표현식이기 때문입니다.

그러면 이 코드를 호출하는 코드는 사용자 이름을 담은 `Ok` 값 혹은 `io::Error`를 담은
`Err` 값을 얻는 처리를 하게 될 것입니다. 호출하는 코드가 이 값을 가지고 어떤 일을
할 것인지 우리는 알지 못합니다. 만일 그쪽에서 `Err` 값을 얻었다면, 예를 들면
`panic!`을 호출하여 프로그램을 종료시키는 선택을 할 수도 있고, 기본 사용자 이름을
사용할 수도 있으며, 혹은 파일이 아닌 다른 어딘가에서 사용자 이름을 찾을 수도 있습니다.
호출하는 코드가 정확히 어떤 것을 시도하려 하는지에 대한 충분한 정보가 없기 때문에,
우리는 모든 성공 혹은 에러 정보를 위로 전파하여 호출하는 코드가
적절하게 처리를 하도록 합니다.

러스트에서 에러를 전파하는 패턴은 너무 흔하여 러스트에서는 이를 더 쉽게 해주는
물음표 연산자 `?`를 제공합니다.

### 에러를 전파하기 위한 숏컷: `?`

Listing 9-7은 Listing 9-6과 같은 기능을 가진
`read_username_from_file`의 구현을 보여주는데,
다만 이 구현은 `?` 연산자를 이용하고 있습니다:

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">Listing 9-7: `?` 연산자를 이용하여 에러를 호출 코드쪽으로
반환하는 함수</span>

`Result` 값 뒤의 `?`는 Listing 9-6에서 `Result` 값을 다루기 위해 정의했던
`match` 표현식과 거의 같은 방식으로 동작하게끔 정의되어 있습니다.
만일 `Result`의 값이 `Ok`라면, `Ok` 내의 값이 이 표현식으로부터
얻어지고 프로그램이 계속됩니다.
만일 값이 `Err`라면, 우리가 `return` 키워드를 사용하여 에러 값을 호출하는
코드에게 전파하는 것과 같이 전체 함수로부터 `Err` 내의 값이
반환될 것입니다.

Listing 9-6의 `match` 표현식과 `?` 연산자의 차이점은,
`?` 연산자를 사용할 때의 에러 값들은 `from` 함수를 거친다는 것입니다.
`from` 함수는 표준 라이브러리 내의 `From` 트레잇에 정의되어 있으며
에러의 타입을 다른 타입으로 변환하는 데에 사용합니다.
`?` 연산자가 `from` 함수를 호출하면, `?` 연산자가 얻게 되는 에러를 `?` 연산자가
사용된 현재 함수의 반환 타입에 정의된 에러 타입으로 변환합니다.
이는 어떤 함수가 다양한 종류의 에러로 인해 실패할 수 있지만,
모든 에러를 하나의 에러 타입으로 반환할 때 유용합니다.
각각의 에러 타입이 자신의 타입으로부터, 반환되어야 하는 에러 타입으로 변환하는
방법을 `from` 함수로 구현하기만 한다면 `?` 연산자가
자동으로 변환합니다.

Listing 9-7의 내용에서, `File::open` 호출 부분의 끝에 있는 `?`는
`Ok`내의 값을 변수 `f`에게 반환해줄 것입니다. 만일 에러가 발생하면
`?`는 전체 함수로부터 일찍 빠져나와 호출하는 코드에게 어떤 `Err` 값을 줄 것입니다.
`read_to_string` 호출의 끝부분에 있는 `?`도 같은 것이 적용되어
있습니다.

`?`는 많은 양의 보일러플레이트를 제거해주고 이 함수의 구현을 더 단순하게 만들어 줍니다.
심지어는 Listing 9-8과 같이 `?` 뒤에 바로 메소드 호출을 연결하는 식으로 이 코드를 더
줄일 수도 있습니다:

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">Listing 9-8: `?` 연산자 뒤에 메소드 호출을
연결하기</span>

새로운 `String`을 만들어 `s`에 넣는 부분을 함수의 시작 부분으로 옮겼습니다.
이 부분은 달라진 것이 없습니다. `f` 변수를 만드는 대신,
`File::open("hello.txt")?`의 결과 바로 뒤에 `read_to_string`의 호출을 연결했습니다.
`read_to_string` 호출의 끝에는 여전히 `?`가 남아있고, `File::open`과
`read_to_string`이 모두 에러를 반환하지 않고 성공할 때 `s` 안의 사용자 이름을
담은 `Ok`를 여전히 반환합니다.
함수의 기능 또한 Listing 9-6과 Listing 9-7의 것과 동일하고,
다만 작성하기에 더 인체공학적인 방법이라는 차이만 있을 뿐입니다.

이 함수를 작성하는 또 다른 방법으로는, Listing 9-9에 더 짧게 만든
예시가 있습니다.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">Listing 9-9: 파일을 열고, 읽는 대신 `fs::read_to_string`를
사용하기</span>

파일에서 문자열을 읽는 코드는 굉장히 흔하게 사용되기 때문에,
러스트는 파일을 열고, 새 `String`을 생성하고, 파일 내용을 읽고,
내용을 `String`에 집어넣고 반환하는
편리한 `fs::read_to_string` 함수를 제공합니다.
다만 `fs::read_to_string`를 사용해버리면 여러분에게 에러를 다루는 법을
자세히 설명할 수 없으니 긴 코드로 먼저 설명했습니다.

### `?`는 `Result`를 반환하는 함수에서 사용될 수 있습니다

`?`는 `Result` 타입을 반환하는 함수에서 사용할 수 있는데, 이것이 Listing 9-6에
정의된 `match` 표현식과 동일한 방식으로 동작하도록 정의되어 있기 때문입니다.
`Result` 반환 타입을 요구하는 `match` 부분은 `return Err(e)`이며,
따라서 함수의 반환 타입은 반드시 이 `return`과 호환 가능한
`Result`가 되어야 합니다.

`main`의 반환 타입이 `()`라는 것을 상기하면서, 만약 `main` 함수 내에서 `?`를
사용하면 어떤 일이 생길지 살펴봅시다:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-06-question-mark-in-main/src/main.rs}}
```

이걸 컴파일하면, 아래와 같은 에러 메세지가 뜹니다:

```console
{{#include ../listings/ch09-error-handling/no-listing-06-question-mark-in-main/output.txt}}
```

이 에러는 `Result`나 `Option`, 혹은 `std::ops::Try`를 구현한
그 밖의 타입을 반환하는 함수 내에서 `?` 연산자를 사용할 수 있음을
지적합니다. 위의 타입 중 하나를 반환하지 않는 함수 내에서,
`Result<T, E>`를 반환하는 함수를 호출할 때 `?`를
사용하고 싶다면 두 가지 방법이 있습니다.
하나는 함수의 반환형을 변경해도 별문제가 없다면
`Result<T, E>`로 변경하는 방법입니다. 또 하나는 `match`나
`Result<T, E>` 메소드 중 하나를 이용해 `Result<T, E>`를
적절한 방식으로 처리하는 방법입니다.

`main` 함수는 특수하므로 함수의 반환형에 제약이 있고,
허용된 반환형은 `()`가 있습니다. 그런데 형편 좋게도,
`Result<T, E>`도 허용된 반환형입니다. 다음에서 볼 수 있습니다.

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-07-main-returning-result/src/main.rs}}
```

`Box<dyn Error>` 타입은 트레잇 객체 라고 합니다.
이는 17장의 ["트레잇 객체를 사용하여 다른 타입 간의 값 허용하기"][trait-objects]<!-- ignore -->
절에서 다룰 예정입니다.
현재로서는 "어떤 종류의 에러"라는 뜻으로 받아들이시면 됩니다.
이 반환형의 `main` 함수에서는 `?`를 사용할 수 있습니다.

`panic!`을 호출하거나 `Result`를 반환하는 것의 자세한 부분을 논의했으니,
어떤 경우에 어떤 방법을 사용하는 것이 적합할지를 어떻게 결정하는가에 대한
주제로 돌아갑시다.

[trait-objects]: ch17-02-trait-objects.html#%ED%8A%B8%EB%A0%88%EC%9E%87-%EA%B0%9D%EC%B2%B4%EB%A5%BC-%EC%82%AC%EC%9A%A9%ED%95%98%EC%97%AC-%EB%8B%A4%EB%A5%B8-%ED%83%80%EC%9E%85-%EA%B0%84%EC%9D%98-%EA%B0%92-%ED%97%88%EC%9A%A9%ED%95%98%EA%B8%B0
