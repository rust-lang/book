## `Result`와 함께하는 복구 가능한 에러

대부분의 에러는 프로그램을 전부 멈추도록 요구될 정도로 심각하지는 않습니다. 종종 어떤 함수가 실패할
때는, 우리가 쉽게 해석하고 대응할 수 있는 이유에 대한 것입니다. 예를 들어, 만일 우리가 어떤 파일을
여는데 해당 파일이 존재하지 않아서 연산에 실패했다면, 프로세스를 멈추는 대신 파일을 새로 만드는
것을 원할지도 모릅니다.

2장의 “[`Result` 타입으로 잠재된 실패 다루기][handle_failure]<!-- ignore -->” 절에서
`Result` 열거형은 다음과 같이 `Ok`와 `Err`라는 두 개의 variant를 갖도록 정의되어 있음을
상기하세요:

[handle_failure]: ch02-00-guessing-game-tutorial.html#result-타입으로-잠재된-실패-다루기

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T`와 `E`는 제네릭 타입 파라미터입니다; 10장에서 제네릭에 대해 더 자세히 다룰 것입니다. 지금으로서
여러분이 알아둘 필요가 있는 것은, `T`는 성공한 경우에 `Ok` variant 내에 반환될 값의 타입을 나타내고
`E`는 실패한 경우에 `Err` variant 내에 반환될 에러의 타입을 나타내는 것이라는 점입니다. `Result`가
이러한 제네릭 타입 파라미터를 갖기 때문에, 우리가 반환하고자 하는 성공적인 값과 에러 값이 다를 수 있는
다양한 상황 내에서 표준 라이브러리에 정의된 `Result` 타입과 함수들을 사용할 수 있습니다.

실패할 수도 있기 때문에 `Result` 값을 반환하는 함수를 호출해 봅시다:
Listing 9-3에서는 파일 열기를 시도합니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

<span class="caption">Listing 9-3: 파일 열기</span>

`File::open`이 `Result`를 반환하는지 어떻게 알까요? 표준 라이브러리 API 문서를 찾아보거나,
컴파일러에게 물어볼 수 있습니다! 만일 `f`에게 우리가 알고 있고 그 함수의 반환 타입은 *아닐* 어떤 타입에
대한 타입 명시를 주고 그 코드의 컴파일을 시도한다면, 컴파일러는 우리에게 타입이 맞지 않는다고
알려줄 것입니다. 그후 에러 메세지는 `f`의 타입이 *무엇인지* 알려줄 것입니다. 한번 해봅시다:
우리는 `File::open`의 반환 타입이 `u32`는 아니라는 것을 알고 있으니, `let f` 구문을 이렇게
바꿔봅시다:

```rust,ignore
let f: u32 = File::open("hello.txt");
```

이제 컴파일을 시도하면 다음 메세지가 나타납니다:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
  = note:    found type `std::result::Result<std::fs::File, std::io::Error>`
```

이 메세지는 `File::open` 함수의 반환 타입이 `Result<T, E>`라는 것을 알려줍니다. 여기서 제네릭
파라미터 `T`는 성공값의 타입인 `std::fs::File`로 체워져 있는데, 이것은 파일 핸들입니다. 에러에
사용되는 `E`의 타입은 `std::io::Error`입니다.

이 반환 타입은 `File::open`을 호출하는 것이 성공하여 우리가 읽거나 쓸 수 있는 파일 핸들을 반환해
줄 수도 있다는 뜻입니다. 함수 호출은 또한 실패할 수도 있습니다: 예를 들면 파일이 존재하지 않거나
파일에 접근할 권한이 없을지도 모릅니다. `File::open` 함수는 우리에게 성공했는지 혹은 실패했는지를
알려주면서 동시에 파일 핸들이나 에러 정보 둘 중 하나를 우리에게 제공할 방법을 가질 필요가 있습니다.
바로 이러한 정보가 `Result` 열거형이 전달하는 것과 정확히 일치합니다.

`File::open`이 성공한 경우, 변수 `f`가 가지게 될 값은 파일 핸들을 담고 있는 `Ok` 인스턴스가
될 것입니다. 실패한 경우, `f`의 값은 발생한 에러의 종류에 대한 더 많은 정보를 가지고 있는 `Err`의
인스턴스가 될 것입니다.

우리는 Listing 9-3의 코드에 `File::open`이 반환하는 값에 따라 다른 행동을 취하는 코드를 추가할
필요가 있습니다. Listing 9-4은 우리가 6장에서 다뤘던 기초 도구 `match` 표현식을 이용하여
`Result`를 처리하는 한 가지 방법을 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

<span class="caption">Listing 9-4: `match` 표현식을 사용하여 발생 가능한 `Result`
variant들을 처리하기</span>

`Option` 열거형과 같이 `Result` 열거형과 variant들은 프렐루드(prelude)로부터 가져와진다는 점을
기억하세요. 따라서 `match`의 각 경우에 대해서 `Ok`와 `Err` 앞에 `Result::`를 특정하지 않아도
됩니다.

여기서 우리는 러스트에게 결과가 `Ok`일 때에는 `Ok` variant로부터 내부의 `file` 값을 반환하고,
이 파일 핸들 값을 변수 `f`에 대입한다고 말해주고 있습니다. `match` 이후에는 읽거나 쓰기 위해
이 파일 핸들을 사용할 수 있습니다.

`match`의 다른 경우는 `File::open`으로부터 `Err`를 얻은 경우를 처리합니다. 이 예제에서는
`panic!` 매크로를 호출하는 방법을 택했습니다. 우리의 현재 디렉토리 내에 *hello.txt*라는 이름의
파일이 없는데 이 코드를 실행하게 되면, `panic!` 매크로로부터 다음과 같은 출력을 보게 될 것입니다:

```text
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
```

늘 그렇듯이, 이 출력은 어떤 것이 잘못되었는지 정확히 알려줍니다.

### 서로 다른 에러에 대해 매칭하기

Listing 9-3의 코드는 `File::open`이 실패한 이유가 무엇이든 간에 `panic!`을 일으킬 것입니다.
대신 우리가 원하는 것은 실패 이유에 따라 다른 행동을 취하는 것입니다: 파일이 없어서
`File::open`이 실패한 것이라면, 새로운 파일을 만들어서 핸들을 반환하고 싶습니다. 만일 그밖의
이유로 `File::open`이 실패한 거라면, 예를 들어 파일을 열 권한이 없어서라면, 예를 들어 우리가
파일을 열 권한이 없기 때문이라면, Listing 9-4에서 했던 것과 마찬가지로 `panic!`을 일으키고
싶습니다. `match`에 새로운 경우를 추가한 Listing 9-5를 봅시다:

<span class="filename">Filename: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```

<span class="caption">Listing 9-5: 다른 종류의 에러를 다른 방식으로 처리하기</span>

`Err` variant 내에 있는 `File::open`이 반환하는 값의 타입은 `io::Error`인데, 이는
표준 라이브러리에서 제공하는 구조체입니다. 이 구조체는 `kind` 메소드를 제공하는데 이를 호출하여
`io::ErrorKind`값을 얻을 수 있습니다. `io::ErrorKind`는 `io` 연산으로부터 발생할 수 있는
여러 종류의 에러를 표현하는 variant를 가진, 표준 라이브러리에서 제공하는 열거형입니다. 우리가
사용하고자 하는 variant는 `ErrorKind::NotFound`인데, 이는 열고자 하는 파일이 아직 존재하지
않음을 나타냅니다.

조건문 `if error.kind() == ErrorKind::NotFound`는 *매치 가드(match guard)* 라고
부릅니다: 이는 `match` 줄기 상에서 줄기의 패턴을 좀더 정제해주는 추가 조건문입니다. 그 줄기의 코드가
실행되기 위해서는 이 조건문이 참이어야 합니다; 그렇지 않다면, 패턴 매칭은 `match`의 다음 줄기에
맞춰보기 위해 이동할 것입니다. 패턴에는 `ref`가 필요하며 그럼으로써 `error`가 가드 조건문으로
소유권 이동이 되지 않고 그저 참조만 됩니다. 패턴 내에서 참조자를 얻기 위해 `&`대신 `ref`이 사용되는
이유는 18장에서 자세히 다룰 것입니다. 짧게 설명하면, `&`는 참조자를 매치하고 그 값을 제공하지만,
`ref`는 값을 매치하여 그 참조자를 제공합니다.

매치 가드 내에서 확인하고자 하는 조건문은 `error.kind()`에 의해 반환된 값이 `ErrorKind` 열거형의
`NotFound` variant인가 하는 것입니다. 만일 그렇다면, `File::create`로 파일 생성을 시도합니다.
그러나, `File::create` 또한 실패할 수 있기 때문에, 안쪽에 `match` 구문을 바깥쪽과 마찬가지로 추가할
필요가 있습니다. 파일이 열수 없을 때, 다른 에러 메세지가 출력될 것입니다. 바깥쪽 `match`의 마지막 갈래는
똑같이 남아서, 파일을 못 찾는 에러 외에 다른 어떤 에러에 대해서도 패닉을 일으킵니다.

### 에러가 났을 때 패닉을 위한 숏컷: `unwrap`과 `expect`

`match`의 사용은 충분히 잘 동작하지만, 살짝 장황하기도 하고 의도를 항상 잘 전달하는 것도 아닙니다.
`Result<T, E>` 타입은 다양한 작업을 하기 위해 정의된 수많은 헬퍼 메소드를 가지고 있습니다. 그 중
하나인 `unwrap` 이라 부르는 메소드는 Listing 9-4에서 작성한 `match` 구문과 비슷한 구현을 한 숏컷
메소드입니다. 만일 `Result` 값이 `Ok` variant라면, `unwrap`은 `Ok` 내의 값을 반환할 것입니다.
만일 `Result`가 `Err` variant라면, `unwrap`은 우리를 위해 `panic!` 매크로를 호출할 것입니다.
아래에 `unwrap`이 작동하는 예가 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

*hello.txt* 파일이 없는 상태에서 이 코드를 실행시키면, `unwrap` 메소드에 의한 `panic!`
호출로부터의 에러 메세지를 보게 될 것입니다:

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
/stable-dist-rustc/build/src/libcore/result.rs:868
```

또다른 메소드인 `expect`는 `unwrap`과 유사한데, 우리가 `panic!` 에러 메세지를 선택할 수 있게
해줍니다. `unwrap`대신 `expect`를 이용하고 좋은 에러 메세지를 제공하는 것은 여러분의 의도를
전달해주고 패닉의 근원을 추적하는 걸 쉽게 해 줄수 있습니다. `expect`의 문법은 아래와 같이
생겼습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

`expect`는 `unwrap`과 같은 식으로 사용됩니다: 파일 핸들을 리턴하거나 `panic!` 매크로를 호출하는
것이죠. `expect`가 `panic!` 호출에 사용하는 에러 메세지는 `unwrap`이 사용하는 기본 `panic!`
메세지보다는 `expect`에 넘기는 파라미터로 설정될 것입니다. 아래에 어떻게 생겼는지에 대한 예가 있습니다:

```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }',
/stable-dist-rustc/build/src/libcore/result.rs:868
```

이 에러 메세지는 우리가 특정한 텍스트인 `Failed to open hello.txt`로 시작하기 때문에, 이 에러 메세지가
어디서부터 왔는지를 코드 내에서 찾기가 더 수월해질 것입니다. 만일 우리가 여러 군데에 `unwrap`을 사용하면,
정확히 어떤 `unwrap`이 패닉을 일으켰는지 찾기에 좀 더 많은 시간이 걸릴 수 있는데, 그 이유는 패닉을
호출하는 모든 `unwrap`이 동일한 메세지를 출력하기 때문입니다.

### 에러 전파하기

실패할지도 모르는 무언가를 호출하는 구현을 가진 함수를 작성할때, 이 함수 내에서 에러를 처리하는 대신,
에러를 호출하는 코드쪽으로 반환하여 그쪽에서 어떻게 할지 결정하도록 할 수 있습니다. 이는 에러
*전파하기*로 알려져 있으며, 에러가 어떻게 처리해야 좋을지 좌우해야 할 상황에서, 여러분의 코드 내용 내에서
이용 가능한 것들보다 더 많은 정보와 로직을 가지고 있을 수도 있는 호출하는 코드쪽에 더 많은 제어권을 줍니다. 

예를 들면, Listing 9-6는 파일로부터 사용자 이름을 읽는 함수를 작성한 것입니다. 만일 파일이 존재하지
않거나 읽을 수 없다면, 이 함수는 호출하는 코드쪽으로 해당 에러를 반환할 것입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

<span class="caption">Listing 9-6: `match`를 이용하여 호출 코드 쪽으로 에러를 반환하는 함수
</span>

함수의 반환 타입부터 먼저 살펴봅시다: `Result<String, io::Error>`. 이는 함수가 `Result<T, E>`
타입의 값을 반환하는데 제네릭 파라미터 `T`는 구체적 타입(concrete type)인 `String`로 채워져 있고,
제네릭 타입 `E`는 구체적 타입인 `io::Error`로 채워져 있습니다. 만일 이 함수가 어떤 문제 없이 성공하면,
함수를 호출한 코드는 `String`을 담은 값을 받을 것입니다 - 이 함수가 파일로부터 읽어들인 사용자
이름이겠지요. 만일 어떤 문제가 발생한다면, 이 함수를 호출한 코드는 문제가 무엇이었는지에 대한 더 많은
정보를 담고 있는 `io::Error`의 인스턴스를 담은 `Err` 값을 받을 것입니다. 이 함수의 반환 타입으로서
`io::Error`를 선택했는데, 그 이유는 우리가 이 함수 내부에서 호출하고 있는 실패 가능한 연산 두 가지가
모두 이 타입의 에러 값을 반환하기 때문입니다: `File::open` 함수와 `read_to_string` 메소드 말이죠.

함수의 본체는 `File::open` 함수를 호출하면서 시작합니다. 그 다음에는 Listing 9-4에서 본 `match`와
유사한 식으로 `match`을 이용해서 `Result` 값을 처리하는데, `Err` 경우에 `panic!`을 호출하는 대신
이 함수를 일찍 끝내고 `File::open`으로부터의 에러 값을 마치 이 함수의 에러 값인것처럼 호출하는 쪽의
코드에게 전달합니다. 만일 `File::open`이 성공하면, 파일 핸들을 `f`에 저장하고 계속합니다.

그 뒤 변수 `s`에 새로운 `String`을 생성하고 파일의 콘텐츠를 읽어 `s`에 넣기 위해 `f`에 있는
파일 핸들의 `read_to_string` 메소드를 호출합니다. `File::open`가 성공하더라도 `read_to_string`
메소드가 실패할 수 있기 때문에 이 함수 또한 `Result`를 반환합니다. 따라서 이 `Result`를 처리하기
위해서 또다른 `match`가 필요합니다: 만일 `read_to_string`이 성공하면, 우리의 함수가 성공한
것이고, 이제 `s` 안에 있는 파일로부터 읽어들인 사용자 이름을 `Ok`에 싸서 반환합니다. 만일
`read_to_string`이 실패하면, `File::open`의 반환값을 처리했던 `match`에서 에러값을
반환하는 것과 같은 방식으로 에러 값을 반환합니다. 하지만 여기서는 명시적으로 `return`이라 말할
필요는 없는데, 그 이유는 이 함수의 마지막 표현식이기 때문입니다.

그러면 이 코드를 호출하는 코드는 사용자 이름을 담은 `Ok` 값 혹은 `io::Error`를 담은 `Err` 값을
얻는 처리를 하게 될 것입니다. 호출하는 코드가 이 값을을 가지고 어떤 일을 할 것인지 우리는 알지 못합니다.
만일 그 쪽에서 `Err` 값을 얻었다면, 예를 들면 `panic!`을 호출하여 프로그램을 종료시키는 선택을 할
수도 있고, 기본 사용자 이름을 사용할 수도 있으며, 혹은 파일이 아닌 다른 어딘가에서 사용자 이름을
찾을 수도 있습니다. 호출하는 코드가 정확히 어떤 것을 시도하려 하는지에 대한 충분한 정보가 없기 때문에,
우리는 모든 성공 혹은 에러 정보를 위로 전파하여 호출하는 코드가 적절하게 처리를 하도록 합니다.

러스트에서 에러를 전파하는 패턴은 너무 흔하여 러스트에서는 이를 더 쉽게 해주는 물음포 연산자 `?`를
제공합니다.

### 에러를 전파하기 위한 숏컷: `?`

Listing 9-7은 Listing 9-6과 같은 기능을 가진 `read_username_from_file`의 구현을 보여주는데,
다만 이 구현은 물음표 연산자를 이용하고 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

<span class="caption">Listing 9-7: `?`를 이용하여 에러를 호출 코드쪽으로 반환하는 함수</span>

`Result` 값 뒤의 `?`는 Listing 9-6에서 `Result` 값을 다루기 위해 정의했던 `match` 표현식과
거의 같은 방식으로 동작하게끔 정의되어 있습니다. 만일 `Result`의 값이 `Ok`라면, `Ok` 내의 값이
이 표현식으로부터 얻어지고 프로그램이 계속됩니다. 만일 값이 `Err`라면, 우리가 `return` 키워드를
사용하여 에러 값을 호출하는 코드에게 전파하는 것과 같이 전체 함수로부터 `Err` 내의 값이 반환될
것입니다.

Listing 9-6에 있는 `match` 표현식과 물음표 연산자가 수행하는 한가지 차이점은 물음표 연산자를
사용할 때 에러값들이 표준 라이브러리 내에 있는 `From` 트레잇에 정의된 `from` 함수를 친다는
것입니다. 많은 에러 타입들이 어떤 타입의 에러를 다음 타입의 에러로 변환하기 위해 `from` 함수를
구현하였습니다. 물음표 연산자가 사용되면, `from` 함수의 호출이 물음표 연산자가 얻게 되는 에러
타입을 `?`이 사용되고 있는 현재 함수의 반환 타입에 정의된 에러 타입으로 변환합니다. 이는
어떤 함수의 부분들이 수많은 다른 이유로 인해 실패할 수 있지만 이 함수는 실패하는 모든 방식을
하나의 에러 타입으로 반환할 때 유용합니다. 각각의 에러 타입이 그 자신을 반환되는 에러 타입으로
변경할 방법을 정의하기 위해 `from` 함수를 구현하기만 한다면, 물음표 연산자는 이 변환을 자동적으로
다룹니다.

Listing 9-7의 내용에서, `File::open` 호출 부분의 끝에 있는 `?`는 `Ok`내의 값을 변수 `f`에게
반환해줄 것입니다. 만일 에러가 발생하면 `?`는 전체 함수로부터 일찍 빠져나와 호출하는 코드에게
어떤 `Err` 값을 줄 것입니다. `read_to_string` 호출의 끝부분에 있는 `?`도 같은 것이 적용되어
있습니다.

`?`는 많은 수의 보일러플레이트(boilerplate)를 제거해주고 이 함수의 구현을 더 단순하게 만들어 줍니다.
심지어는 Listing 9-8과 같이 `?` 뒤에 바로 메소드 호출을 연결하는 식으로 (chaining) 이 코드를 더
줄일 수도 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

<span class="caption">Listing 9-8: 물음표 연산자 뒤에 메소드 호출을 연결하기</span>

새로운 `String`을 만들어 `s`에 넣는 부분을 함수의 시작 부분으로 옮겼습니다; 이 부분은 달라진 것이
없습니다. `f` 변수를 만드는 대신, `File::open("hello.txt")?`의 결과 바로 뒤에
`read_to_string`의 호출을 연결시켰습니다. `read_to_string` 호출의 끝에는 여전히 `?`가
남아있고, `File::open`과 `read_to_string`이 모두 에러를 반환하지 않고 성공할 때
`s` 안의 사용자 이름을 담은 `Ok`를 여전히 반환합니다. 함수의 기능 또한 Lsting 9-6와
Listing 9-7의 것과 동일하고, 다만 작성하기에 더 인체공학적인 방법이라는 차이만 있을 뿐입니다.

### `?`는 `Result`를 반환하는 함수에서만 사용될 수 있습니다

`?`는 `Result` 타입을 반환하는 함수에서만 사용이 가능한데, 이것이 Listing 9-6에 정의된 `match`
표현식과 동일한 방식으로 동작하도록 정의되어 있기 때문입니다. `Result` 반환 타입을 요구하는
`match` 부분은 `return Err(e)`이며, 따라서 함수의 반환 타입은 반드시 이 `return`과 호환 가능한
`Result`가 되어야 합니다.

`main`의 반환 타입이 `()`라는 것을 상기하면서, 만약 `main` 함수 내에서 `?`를 사용하면 어떤일이 생길지
살펴봅시다:

```rust,ignore
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

이걸 컴파일하면, 아래와 같은 에러 메세지가 뜹니다:

```text
error[E0277]: the `?` operator can only be used in a function that returns
`Result` (or another type that implements `std::ops::Try`)
 --> src/main.rs:4:13
  |
4 |     let f = File::open("hello.txt")?;
  |             ------------------------
  |             |
  |             cannot use the `?` operator in a function that returns `()`
  |             in this macro invocation
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`
```

이 에러는 오직 `Result`를 반환하는 함수 내에서만 물음표 연산자를 사용할 수 있음을 지적합니다.
`Result`를 반환하지 않는 함수 내에서, 여러분이 `Result`를 반환하는 다른 함수를 호출했을 때,
여러분은 `?`를 사용하여 호출하는 코드에게 잠재적으로 에러를 전파하는 대신 `match`나 `Result`에서
제공하는 메소드들 중 하나를 사용하여 이를 처리할 필요가 있을 것입니다.

`panic!`을 호출하거나 `Result`를 반환하는 것의 자세한 부분을 논의했으니, 어떤 경우에 어떤 방법을
사용하는 것이 적합할지를 어떻게 결정하는가에 대한 주제로 돌아갑시다.
