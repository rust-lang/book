## 모듈성과 에러처리의 향상을 위한 리팩토링

우리 프로그램을 향상시키기 위해 네 가지 수정하고 싶은 문제가 있는데, 이들은 프로그램을 
구조화하고 발생가능한 에러를 처리하는 방식과 관련있습니다.

첫 번째, 우리 `main` 함수는 현재 두 가지 작업을 수행합니다: 인자들을 분석하고 파일을
열지요. 이런 작은 함수에서, 이건 큰 문제가 안됩니다. 하지만 우리가 계속해서 `main`함수
안에 프로그램을 작성하여 커지게 되면, `main` 함수가 처리하는 작업의 수도 늘어나게 될
겁니다. 함수가 갖게되는 책임들만큼, 근원을 파악하기도, 테스트 하기에도, 부분 별로 나누지
않고는 수정하기도 어려워 집니다. 함수는 나뉘어 하나의 작업에 대해서만 책임을 지는 것이 더
좋은 구조입니다.

이 문제는 우리의 두 번째 문제와도 관련이 있습니다: `query` 와 `filename` 은  프로그램의 설정을 
저장하는 변수이고 `f` 와 `contents` 같은 변수는 프로그램의 논리 수행에 사용됩니다.
`main`이 길어질수록 범위 내에 더 많은 변수가 생깁니다. 범위 내에 더 많은 변수가 존재할수록,
각각의 변수를 추적하기 힘들어집니다. 목적을 분명히 하기 위해 설정 변수를 그룹화하여 하나의 구조로
결합시키는 것이 좋습니다.

세 번째 문제는 파일 열기가 실패 할 경우`expect`를 사용하여 오류 메시지를 출력해주는데,
에러 메시지가 `Something went wrong reading the file` 밖에 없습니다. 파일이 존재하지 않는 경우 외에도 파일
열기가 실패하는 경우들이 있습니다. 예를 들어 파일은 존재하지만 파일을 열 수있는 권한이 없을 수
있습니다. 현재는 이런 상황에도 `Something went wrong reading the file` 이란 오류 메시지를 출력하여 사용자에게
잘못된 조언을 해주게 됩니다.

넷째, 우리는 서로 다른 오류를 다루기 위해 `expect`를 반복적으로 사용하고 있습니다. 헌데
만약 사용자가 충분한 인수를 지정하지 않고 프로그램을 실행하면 Rust의 "index out of
bounds" 오류가 발생하는데 이는 문제를 명확하게 설명하지 않습니다. 우리가 모든 오류처리 
코드를 한 군데 모아놓으면 후에 관리자는 오류처리 로직을 변경해야 할 때 오직 이 곳의 코드만
참고하면 되니 더 좋죠. 또한, 모든 오류 처리 코드를 한 곳에 저장하면 우리가 최종 사용자에게
도움이 되는 메시지를 출력하고 있는지 확신하는데도 도움이 됩니다.

이런 문제들을 우리 프로젝트를 리팩토링하여 해결해보도록 하겠습니다.

### 바이너리 프로젝트를 위한 관심사의 분리

`main` 함수가 여러 작업에 책임을 갖게 되는 구조적 문제는 많은 바이너리 프로젝트에서 
공통적입니다. 그래서 Rust 커뮤니티는 `main`이 커지기 시작할 때 바이너리 프로그램의 
핵심기능을 나누기 위한 가이드라인 프로세스를 개발했습니다. 프로세스에는 다음 단계가 있습니다:

1. 당신의 프로그램을 *main.rs* 과 *lib.rs* 로 나누고 프로그램의 로직을 *lib.rs* 
	으로 옮깁니다.
2. 커맨드라인 파싱 로직이 크지 않으면, *main.rs* 에 남겨둬도 됩니다.
3. 커맨드라인 파싱 로직이 복잡해지기 시작할거 같으면, *main.rs* 에서 추출해서 *lib.rs*
	로 옮기세요.
4. 이런 절차를 통해 `main` 함수에는 다음의 핵심 기능들만 남아있어야 합니다:
    * 인자 값들로 커맨드라인을 파싱하는 로직 호출 
    * 다른 환경들 설정
    * *lib.rs*의 `run` 함수 호출
    * `run`이 에러를 리턴하면, 에러 처리.

이 패턴이 핵심기능을 분리하는데 관한 모든 것입니다: *main.rs* 는 프로그램 실행을
담당하고, *lib.rs*는 맡은 작업에 관한 로직을 담당합니다. `main` 함수는 직접 테스트 할
수 없지만, 이런 구조로 *lib.rs* 으로 프로그램의 모든 함수와 로직을 옮긴 후에는 테스트가
가능해집니다. *main.rs*에는 읽어서 옳바른지 여부를 검증할 수 있을 정도로 적은 코드만을 
남겨두도록 합니다. 다음의 과정을 거치며 재작업을 해봅시다.

### 인자 파서의 추출

먼저 우리는 커맨드라인 인자를 분석하는 기능을 추출할 겁니다. 항목 12-5에서 `main`의 시작 부분이 새로운 함수 `parse_config`를 호출하는 것을 볼 수 있을텐데, 이는 아직은  *src/main.rs*에 정의되어 있을 겁니다. 

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // ...snip...
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```

<span class="caption">Listing 12-5: Extract a `parse_config` function from
`main`</span>

우리는 아직 커맨드라인 인자들을 벡터로 수집하고 있는데, 인덱스 1의 인수 값을 변수 `query`
에, 인덱스 2의 인수 값을 `main` 함수 내의 변수 `filename`에 할당하는 대신에 전체
벡터를 `parse_config` 함수로 전달합니다. `parse_config` 함수는 어디에 위치한
인자가 어떤 변수에 대입되는지에 대한 로직을 보유하고, 그 값들을 `main`으로 되돌려 줍니다.
우리는 여전히 `query`와 `filename`변수를 `main`에 생성하지만, `main`은 
더 이상 커맨드라인 인자와 변수간의 상관 관계를 책임지지도 알아야 할 필요도 없죠.

이것이 우리가 작은 프로그램을 유지하기 위한 과도한 행동으로 보일수도 있지만, 우리는 조금씩 
점진적으로 리팩토링을 진행하고 있습니다. 이런 변화를 준 뒤에는, 프로그램을 다시 실행해 인자의 
파싱이 정상적으로 동작하고 있는지 확인해보십시오. 진행 상황을 자주 확인하면 문제가 생겼을 때
원인을 파악하는데 도움이 됩니다. 

#### 설정 변수들을 그룹짓기

<!-- 원문의 내용이 잘못 작성된 것 같음. At the moment, we’re returning a tuple, but then we immediately break that tuple up into individual parts again. 문장은 but 이후 문장이 부정적 의미로 와야맞는 될 것 같음 -->

우리는 이 함수의 기능을 더 향상시키기 위해 또 다른 작은 행동을 할 수 있습니다. 현재 우리는
튜플을 반환하고 있는데, 그 시점에 즉시 튜플을 개별된 부분으로 나눌 수가 없습니다. 이는 우리가
아직은 제대로 된 추상화를 하지 못하고 있다는 신호일 수 있습니다.

또 다른 의미로는 `config`의 부분인 `parse_config`에 향상시킬 지점이 있다는 것으로,
우리가 반환하는 두 개의 값은 관련되어 있으며 모두 하나의 설정 값에 대한 부분이죠. 우리는 현재
두 값을 튜플로 그룹화하는 것 이외의 다른 의미를 전달하지 않습니다. 두 값을 하나의 구조체에
넣고 각 구조체 필드에 의미있는 이름을 지정할 수 있습니다. 이렇게 하면 이 코드의 향후 유지
보수 담당자가 서로 다른 값이 서로 어떻게 관련되어 있고 그 목적이 무엇인지 쉽게 이해할 수
있습니다.


> 주의: 어떤 사람들은 복합 타입(complex type)이 더 적절할 경우에도 기본 타입(primitive type)을 사용하는데 이러한 안티 패턴을 강박적 기본타입 사용(primitive obsession) 이라 부릅니다

항목 12-6에서 `query`와 `filename`을 필드로 갖는 `Config`란 구조체 정의가
추가된 것을 볼 수 있습니다. 우리는 또한 `parse_config` 함수를 변경하여 `Config`
구조체의 객체를 반환하게 변경하였으며, `main`에서 별개의 변수가 아닌 구조체의 필드를
사용하도록 변경했습니다. 

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
# use std::env;
# use std::fs::File;
#
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    // ...snip...
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```

Listing 12-6: Refactoring `parse_config` to return an instance of a `Config` struct

이제 `parse_config`의 선언은 `Config` 값을 반환한다는 것을 알려줍니다. 
`parse_config` 의 내부에서는 `args`의 `String`값을 참조하는 문자열 조각을
반환했었지만, 이제는 `Config`를 정의하고 자체 `String`의 값을 포함하도록 선택했습니다.
`main`의 `args`변수는 인자 값들의 소유주로 `parse_config`에는 그들을 대여해줄 뿐
입니다. 그렇기에 만약 `Config`가 `args`의 값들에 대한 소유권을 가지려고 시도하면 Rust의
대여 규칙을 위반하게 됩니다.

우리가 `String` 데이터를 관리하는 방식은 여러가지가 있겠습니다만, 가장 쉽고 약간 비효율적인 방법은 `clone` 메소드를 호출하는 겁니다. 이 방식은 `Config` 객체에서 소유하게 할 data 전체에 대한 복사본을 만들 것이며, 이런 방식은 참조만 보관하는 것에 비해 약간 더 많은 비용과 메모리가 소비됩니다. 하지만 데이터의 복제본을 만드는 방식은 우리가 참조의 생명주기를 관리하지 않아도 되기 때문에 우리의 코드를 매우 직관적이게 합니다. 그래서 이런 상황에서는 약간의 성능을 포기하고 간소함을 유지하는 것이 매우 가치있는 거래입니다.

> ### `clone` 사용의 기회비용 
>
> 많은 Rust 사용자들은 런타임 비용 때문에 소유권 문제를 수정하기 위해 clone을 사용하지
> 않는 경향이 있습니다. 13장 이터레이터에서, 이런 상황에서보다 효율적인 메소드를
> 사용하는 법을 배우겠지만, 지금은 한 번만 clone하며 query와 filename이 매우 작기
> 때문에 몇 개의 문자열을 clone하여 진행하는 것이 좋습니다. 첫 번째 단계에서는 코드를 
> 최대한 최적화하는 것보다 약간 비효율적이더라도 넘어가는게 좋습니다. Rust에 대한 경험이
> 많을수록 바람직한 방법으로 곧장 진행할 수 있을 겁니다. 지금은 clone을 호출하는 것이 
> 완벽한 선택입니다. 


`parse_config`에 의해 반환된 `Config`의 객체를 `config`라는 변수에 넣고 이전에
별도로 `query`와 `filename`이란 이름으로 나뉘어 있던 변수 대신 `Config` 구조체의
필드를 사용하도록 `main`을 업데이트했습니다.

우리의 코드는 이제 보다 분명하게 `query`와 `filename`이 연관되어 있으며 이들의 목적이
프로그램이 어떻게 동작할지에 대한 설정이라는 의도를 전달할 수 있습니다. 이 값을 사용하는 모든 코드는 그들의 의도에 맞게 지정된 필드를 `config` 객체에서 찾을 수 있습니다.

#### `Config`를 위한 생성자 만들기.


지금까지 우리는 `main`에서 `parse_config`함수로 커맨드라인 인자를 파싱하는 로직을 
추출했습니다. 이를 통해 우리 코드에서 `query`와 `filename`값이 연관되어 있고 그
연결성이 전달되어야 한다는 것을 알았습니다. 그래서 우리는 `Config` 구조체를 추가하고
그 의도와 목적에 맞게 `query`와 `filename`을 명명했으며 `parse_config` 함수에서
변수의 이름을 구조체 필드 이름으로 반환 할 수 있게 했습니다.

그래서 이제 `parse_config` 함수의 목적은 `Config` 객체를 생성하는 것인데, 우리는 
`parse_config`라는 평범한 함수를 `Config` 구조체와 관련된 `new`라는 함수로 변경
할 수 있습니다. 이런 변경은 우리의 코드를 보다 자연스럽게 만들어 줍니다:`String::new`를
호출하여 `String`형의 객체를 생성하는 것처럼 표준 라이브러리들의 객체를 생성할 수 있습니다. 
그리고 `parse_config`를 `Config`와 연관된 `new`함수로 변경하게 되면, 우리는 
`Config`의 객체를 `Config::new`를 호출하여 생성할 수 있게 됩니다. 항목 12-7는
우리가 해야할 변동사항 보여줍니다. 

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
# use std::env;
#
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // ...snip...
}

# struct Config {
#     query: String,
#     filename: String,
# }
#
// ...snip...

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```

<span class="caption">Listing 12-7: Changing `parse_config` into
`Config::new`</span>

우리는 `main`을 갱신하여 `parse_config`를 호출하는 대신 `Config::new`를 
호출하게 되었습니다. 우리는 `parse_config`의 이름을 `new`로 바꾸고 그를 `impl`블록 
안으로 옮겼는데, 이를 통해 `new`함수가 `Config`와 연결되게 됩니다. 다시 컴파일을 하고 
제대로 동작하는지 확인해보도록 합시다. 

### 에러 처리 수정하기

이번에는 우리의 에러 처리를 수정해 볼 겁니다. 만일 `args` 벡터가 3개 미만의 아이템을  
가지고 있을 때 인덱스 `2` 혹은 `3`의 값에 접근하려는 시도를 하면 프로그램은 패닉을 
일으키게 된다고 했던 것을 상기시켜 드립니다. 프로그램을 인자 없이 실행해보시면; 
다음같이 될 겁니다.

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/greprs`
thread 'main' panicked at 'index out of bounds: the len is 1
but the index is 1',  /stable-dist-rustc/build/src/libcollections/vec.rs:1307
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

`index out of bounds: the len is 1 but the index is 1` 줄은 
프로그래머를 위해 의도된 에러 메시지이지, 최종 사용자에게는 무슨 일이 있었는지 무엇을 해야 하는지
이해하는데 아무런 도움이 되지 않습니다. 당장 한번 고쳐보겠습니다.

#### 에러 메시지 향상시키기

항목 12-8에서 `new`함수에 검사를 추가하여 인덱스 `1`과 `2`에 접근하기 전에 조각이 
충분한 길이인지를 확인합니다. 조각이 충분히 길지 않다면, 프로그램은 더 좋은 에러메시지
`index out of bounds`를 보여주고 패닉을 일으킵니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// ...snip...
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // ...snip...
```

<span class="caption">항목 12-8: 인자의 숫자가 몇 개인지 검증 추가</span>

이것은 항목 9-8에서 작성한 `Guess::new`함수와 유사합니다. 이 함수는 `value`인수가 유효한 값의 범위를 벗어난 경우 `panic!`을 호출했습니다. 값의 범위를 검사하는 대신에, 우리는`args`의 길이가 적어도 3개인지 검사하면, 함수의 나머지 부분은 이 조건이 이미 충족되었다는 가정 하에서 동작할 수 있습니다. `args`가 3개 보다 적은 아이템을 가진다면, 이 조건은 true가 되고 우리는 `panic!` 매크로를 호출해 프로그램을 즉시 종료 시킬겁니다.

이런 몇 줄의 추가 코드들을 `new`상에 추가하고, 우리 프로그램을 아무 인자도 없이 다시 실행시키면 
다음과 같은 에러를 볼 수 있을 겁니다. 

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/greprs`
thread 'main' panicked at 'not enough arguments', src/main.rs:29
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

이 결과 더 합리적인 좋은 오류 메시지가 표시됩니다. 그러나 사용자에게 제공하고 싶지 않은 추가
정보가 있습니다. 따라서 항목 9-8에서 사용한 기술을 사용하는 것은 여기선 최선의 방법은 아닙니다.
`panic!`에 대한 호출은 9장에서 논의했던 것처럼 사용 방법에 대한 문제가 아닌 아니라 프로그래밍
관련 문제에 더 적합합니다. 대신, 우리는 9장에서 배운 다른 기법으로 `Result`를 반환하는 것을
성공이나 오류를 나타낼 수 있습니다.

#### `new`에서 `panic!`을 호출하는 대신 `Result`를 반환하기. 

우리는 `Result`를 반환 값으로 선택하여 성공인 경우에는 `Config` 객체를 포함시키고 에러가 
발생한 경우에는 문제가 무엇인지 설명할 수 있게 만들 수 있습니다. `Config::new`가 `main`과 
상호작용할 시에, 우리는 `Result`를 사용하여 문제가 있다고 신호할 수 있습니다. 그리고`main`에선
`Err`의 값을 사용자들에게 보다 실용적인 방식으로 변환하여 보여줄 수 있습니다. `thread 'main'`
으로 시작하는 문자들과 `panic!`을 사용해서 보여지는 `RUST_BACKTRACE`관련 메시지 없이.

항목 12-9에서 당신이 변경해야 할 `Config::new`의 반환 값과 `Result`를 반환하기 위한
함수 본문을 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```

<span class="caption">항목 12-9: `Config::new`에서 `Result`반환</span>

우리의 `new` 함수는 이제 성공 시에는 `Config`객체가 에러 시에는 `&'static str`가 포함된
`Result`를 반환하게 됩니다. 10장의 "The Static Lifetime"에서 `&'static str'이
문자열 리터럴이라고 다뤘는데, 이게 현재 우리의 에러 타입입니다.

우리는 `new`함수의 본문에서 두 가지 변경을했습니다 : 사용자가 충분한 인수를 전달하지 않을 때
`panic!`을 호출하는 대신 `Err`값을 반환하고 `Config`를 반환할 때는 `Ok`로 포장하여
반환 합니다. 이런 변경으로 인해 함수는 새로운 타입 선언을 갖게 됩니다.


`Config::new`가 `Err`값을 반환하게 함으로써, `main`함수는 `new`함수로부터 반환된 
`Result`값을 처리하고 에러 상황에 프로세스를 더 깨끗하게 종료 할 수 있습니다.

#### `Config::new`를 호출하고 에러 처리하기 

에러 케이스를 처리하고 사용자-친화적인 메시지를 출력하기 위해서, 항목 12-10에서처럼 
`Config::new`가 리턴하는 `Result`를 처리하기 위해 `main`을 갱신해야 합니다. 그리고
우리 커맨드라인 프로그램을 `panic!`으로 0이 아닌 값을 발생시킬 때에는 종료시켜야 하므로 직접
구현해보도록 합시다. 0이 아닌 종료 값은 우리 프로그램을 호출한 프로그램에게 우리의 프로그램이 에러
상태로 종료되었음을 알리는 규칙입니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // ...snip...
```

<span class="caption">항목 12-10: new `Config`가 실패했을 때 에러 코드와 함께 종료시키기</span>

이 목록에서 우리는 이전에 다루지 않았던 메소드를 사용하고 있습니다: `unwrap_or_else`는
표준 라이브러리에 의해 `Result <T, E>`에 정의되어 있습니다. `unwrap_or_else`를
사용하면 `panic!`이 아닌 에러 처리를 직접 정의 할 수 있습니다. `Result`가 `Ok` 값이면,
이 메소드의 동작은 `unwrap`과 유사합니다 : 그것은 `Ok`로 포장한 내부 값을 반환합니다.
그러나 `Err`값이면 메소드는 *closure*의 코드를 호출합니다. *closure*는 익명의 함수로 
`unwrap_or_else`에 인수로 전달됩니다. 13장에서 클로저에 대해 더 자세히 다룰 것입니다.
여기서 알아 두어야 할 것은 `unwrap_or_else`가 `Err`의 내부 값, 이번 경우에는 항목
12-9에서 우리가 추가한 정적 문자열인 `not enough arguments`을, 수직파이프 사이에
위치하는 `err`로 인자로서 우리의 클로저로 전달한다는 겁니다. 클로저에 있는 코드는 이런 과정을
거쳐 실행 시에 `err`값을 사용할 수 있습니다.

우리는 새 `use`줄을 추가하여 `process`를 공유 라이브러리에서 import했습니다. 에러 상황에
실행될 클로저의 코드는 단 두 줄 입니다. 에러 값을 출력해주고 `process::exit`를 호출합니다. 
`process::exit`함수는 프로그래을 즉시 중단시키고 종료 상태 코드로 전달받은 값을 반환합니다.
이것은 항목 12-8에서 사용한 `panic!`기반의 처리 방식과 유사해 보이지만, 더이상 필요하지 않은 
출력을 하지 않죠. 해볼까요?

```text
$ cargo run
   Compiling greprs v0.1.0 (file:///projects/greprs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48 secs
     Running `target/debug/greprs`
Problem parsing arguments: not enough arguments
```

훌륭하네요! 이 출력은 우리 사용자들에게 훨씬 친화적입니다.

### `run` 함수 추출하기 

이제 환경 설정 파싱 리팩토링을 마무리 했습니다. 우리 프로그램의 로직으로 돌아갑시다. 우리가 
"바이너리 프로젝트에서 핵심 기능의 분리"절에서 논의한 과정에 따라, 우리는 `main`함수에 구성 
설정 또는 오류 처리와 관계 없는 남아있는 모든 로직들을 담고있는 `run`함수를 추출 할 겁니다. 
이 과정이 종료되면, `main`은 간결해져 쉽게 검증할 수 있어지고, 우리는 다른 모든 로직에 대한 
테스트를 작성할 수 있을 겁니다.

항목 12-11 추출된 `run` 함수를 보여줍니다. 현재 우리는 함수를 추출하여 *src/main.rs*에 
함수를 정의하는 작고 점진적 개선만 수행하고 있습니다. 

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    // ...snip...

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// ...snip...
```

<span class="caption">항목 12-11: 남은 프로그램 로직을 `run` 함수로 추출하기 </span>

이제 `run`함수에는 `main`에 잔존하는 파일을 읽는 것부터 나머지 모든 로직이 포함됩니다. 
`run` 함수는 `Config` 객체를 인수로 취합니다.

#### `run` 함수에서 에러 반환하기 

나머지 프로그램 로직을 `main`이 아닌`run` 함수로 분리하면, Listing 12-9의 
`Config::new`처럼 에러 처리를 향상시킬 수 있습니다. `expect`를 호출하여 프로그램을 패닉 
상태로 만드는 대신, `run`함수는 무언가가 잘못되었을 때 `Result <T, E>`를 리턴 할 
것입니다. 이러면 사용자 친화적인 방법으로 오류를 처리하는 로직을 `main`으로 통합 할 수 
있습니다. 항목 12-12는 `run`의 선언부와 본문의 변경 사항을 보여줍니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::error::Error;

// ...snip...

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

<span class="caption">항목 12-12: `run` 함수가 `Result`를 반환하게 바꾸기 </span>

우리는 여기서 세 가지 큰 변화를 만들었습니다. 먼저, `run` 함수의 리턴 타입을 `Result 
<(), Box <dyn Error >>`로 바꿨습니다. 이 함수는 이전에 유닛 타입 `()`을 반환했으며, 
우리는 `Ok`의 경우 반환할 값으로 이 타입을 유지합니다.

우리의 에러 타입으로, *특성 오브젝트* Box <Error>를 사용합니다 (그리고 상단에 `use`문으로 
`std::error::Error`를 범위 내로 임포트 해왔습니다). 우리는 특성 오브젝트들을 17장에서 
다룰 것입니다. 지금 당장은, `Box<dyn Error>`는 함수가 `Error` 특성을 구현하는 타입을 
반환한다는 것만 알면 되고, 특별히 어떤 타입이 반환될지에 대해서는 알 필요 없습니다. 이런 방식은 
다양한 에러 상황에 다른 타입의 오류 값을 반환 할 수 있는 유연성을 확보할 수 있습니다. `dyn`은 "dynamic"의 약자입니다.


우리가 만든 두 번째 변화는 우리가 9 장에서 이야기했듯이, `?`에 대한 `expect`에 대한 호출을 제거한 것입니다. 에러 시에 `panic!`을 호출하는 것보다 현재 함수에서 에러 값을 반환하며 호출자가 처리 할 수 ​​있도록 하였습니다.

셋째, 이 함수는 성공 사례에서 `Ok`값을 반환합니다. 우리는 `run` 함수의 성공 타입을 선언부에서 
`()`로 선언했습니다, 이것은 우리가 유닛 타입 값을 `Ok` 값으로 감쌀 필요가 있음을 의미합니다. 
이 `Ok (())`구문은 조금 이상하게 보일 수 있지만, `()`를 사용하는 것과 마찬가지로 이는 
사이드이펙트 없이 `run`을 호출하는 것을 나타내는 관용적인 방법입니다. 우리가 필요로 하는 값을 
반환하지 않습니다.

실행시키면, 컴파일 될텐데, 경고를 보여줍니다:

```text
warning: unused result which must be used, #[warn(unused_must_use)] on by default
  --> src/main.rs:39:5
   |
39 |     run(config);
   |     ^^^^^^^^^^^^
```

Rust는 우리 코드가 오류가 있음을 나타내는 `Result` 값을 무시한다는 것을 알려줍니다. 우리는 
에러가 있는지 아닌지를 확인하지 않고 있고, 컴파일러는 우리에게 아마도 여기에 에러 처리 코드를 
작성해야 한다는 것을 상기 시켜줄 것입니다! 당장 바로잡아 봅시다. 

Rust는 우리 코드가 오류가 있음을 나타내는 'Result'값을 무시한다는 것을 알려줍니다. 우리는 에러가 있는지 아닌지를 확인하지 않고 있고, 컴파일러는 아마도 여기에 에러 처리 코드를 가지고 있다는 것을 상기 시켜줄 것입니다! 지금 바로 잡아 보자.

#### `main`안의 `run`에서 반환되는 에러 처리하기 

우리는 항목 12-10의 `Config::new`를 사용하여 오류를 처리하는 방식과 비슷한 방법을 사용하여 오류를 검사하고 멋지게 처리합니다. 그러나 약간의 차이점이 있습니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    // ...snip...

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

우리는 `unwrap_or_else`를 호출하기 보다 `if let`을 사용하여 `run`이 `Err`값을 
반환하는지 검사하고 만약 그렇다면 `process::exit(1)`을 호출합니다. `run`은 
`Config::new`가 `Config`객체를 반환하는 것처럼 우리가 `unwrap`하기를 원하는 값을 
반환하지 않습니다. 왜냐하면 `run`은 성공하면 `()`를 반환하기 때문에, 우리는 에러가 발생한 
경우만 신경쓰면 됩니다. 그래서 우리는 `unwrap_or_else`을 통해 포장을 벗길 필요가 없죠, 
값은 무조건 `()`일테니까요.

`if let`과 `unwrap_or_else` 함수의 내용은 동일한 경우에 동일한 동작을 합니다, 오류를 
출력하고 종료하죠.

### 라이브러리 크레이트로 코드를 나누기 

지금까지 꽤 좋아 보인다! 이제 우리는 *src/main.rs* 파일을 나눠서 *src/lib.rs*에 몇 개의 코드를 넣어서 테스트 할 수 있고 작은 *src/main.rs* 파일을 갖게 될 것입니다.

*src/main.rs*에 파편으로 존재하는 다음 코드들을 새 파일로 옮겨봅시다. 
*src/lib.rs*:

- `run` 함수 정의
- 관련있는`use` 문들
- `Config`의 정의
- `Config::new` 함수와 정의

*src/lib.rs*의 내용은 항목 12-13에서 보이는 것과 같을겁니다. 

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

<span class="caption">항목 12-13: `Config`과 `run`을 *src/lib.rs*로 옮기기</span>

우리는 `Config`의 필드 및 `new` 메소드와 `run` 함수에 대해 `pub`을 자유롭게 사용했습니다. 이제 우리가 테스트 할 수있는 공개 API를 가진 라이브러리 크레이트가 생겼습니다.

#### 바이너리 크레이트에서 라이브러리 크레이트 호출하기 

이제 우리는 *src/main.rs*에 있는 바이너리 크레이트의 범위에 *src/lib.rs*로 옮긴 코드를 
`extern crate greprs`를 사용하여 가져와야 합니다. 이후 `use greprs::Config` 
행을 추가하여 `Config` 타입을 범위로 가져오고 항목 12-14와 같이 크레이트 이름으로`run` 
함수 앞에 접두사를 붙입니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

<span class="caption">항목 12-14: `greprs`크레이트를 *src/main.rs* 범위로 연결하기</span>

<!-- 이하의  https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html 
에서 직접 옮겼습니다. -->

라이브러리 크레이트를 바이너리 크레이트에 가져 오려면 `extern crate greprs`을 
사용합니다. 그런 다음 `greprs::Config`줄을 추가하여 `Config`타입을 범위로 가져오고 
`run` 함수 접두어에 크레이트 이름을 붙입니다. 이를 통해 모든 기능이 연결되어 있어야 하며 
작동해야 합니다. `cargo run`을 실행하여 모든 것이 올바르게 연결되어 있는지 확인하십시오.

아오! 빡시게 작업했네요, 우리의 미래를 우리 스스로가 성공의 방향으로 설정했습니다. 이제 에러를 
처리가 훨씬 쉬워졌고, 우리의 코드를 보다 모듈화하였습니다. 거의 모든 작업은 여기 
*src/lib.rs*에서 수행될 겁니다.

새롭게 확보한 모듈성을 통해 이전의 코드로는 하지 못했을 무언가를 쉽게 할 수 있는 이점을 확보했습니다:몇 개의 테스트를 작성해봅시다!
<!-- 이 내용은 https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html 에서 직접 옮겼습니다. -->
<!-- To bring the library crate into the binary crate, we use extern crate minigrep. Then we’ll add a use minigrep::Config line to bring the Config type into scope, and we’ll prefix the run function with our crate name. With that, all the functionality should be connected and should work. Give it a cargo run and make sure everything is wired up correctly.

Whew! That was a lot of work, but we’ve set ourselves up for success in the future. Now it’s much easier to handle errors, and we’ve made our code more modular. Almost all of our work will be done in src/lib.rs from here on out.

Let’s take advantage of this newfound modularity by doing something that would have been hard with our old code, but is easy with our new code: write some tests! -->

<!-- 업데이트된 원본:
## Refactoring to Improve Modularity and Error Handling

To improve our program, we’ll fix four problems that have to do with the
program’s structure and how it’s handling potential errors.

First, our `main` function now performs two tasks: it parses arguments and
opens files. For such a small function, this isn’t a major problem. However, if
we continue to grow our program inside `main`, the number of separate tasks the
`main` function handles will increase. As a function gains responsibilities, it
becomes more difficult to reason about, harder to test, and harder to change
without breaking one of its parts. It’s best to separate functionality so each
function is responsible for one task.

This issue also ties into the second problem: although `query` and `filename`
are configuration variables to our program, variables like `f` and `contents`
are used to perform the program’s logic. The longer `main` becomes, the more
variables we’ll need to bring into scope; the more variables we have in scope,
the harder it will be to keep track of the purpose of each. It’s best to group
the configuration variables into one structure to make their purpose clear.

The third problem is that we’ve used `expect` to print an error message when
opening the file fails, but the error message just prints `file not found`.
Opening a file can fail in a number of ways besides the file being missing: for
example, the file might exist, but we might not have permission to open it.
Right now, if we’re in that situation, we’d print the `file not found` error
message, which would give the user the wrong information!

Fourth, we use `expect` repeatedly to handle different errors, and if the user
runs our program without specifying enough arguments, they’ll get an `index out
of bounds` error from Rust that doesn’t clearly explain the problem. It would
be best if all the error-handling code were in one place so future maintainers
had only one place to consult in the code if the error-handling logic needed to
change. Having all the error-handling code in one place will also ensure that
we’re printing messages that will be meaningful to our end users.

Let’s address these four problems by refactoring our project.

### Separation of Concerns for Binary Projects

The organizational problem of allocating responsibility for multiple tasks to
the `main` function is common to many binary projects. As a result, the Rust
community has developed a process to use as a guideline for splitting the
separate concerns of a binary program when `main` starts getting large. The
process has the following steps:

* Split your program into a *main.rs* and a *lib.rs* and move your program’s
  logic to *lib.rs*.
* As long as your command line parsing logic is small, it can remain in
  *main.rs*.
* When the command line parsing logic starts getting complicated, extract it
  from *main.rs* and move it to *lib.rs*.
* The responsibilities that remain in the `main` function after this process
  should be limited to the following:

  * Calling the command line parsing logic with the argument values
  * Setting up any other configuration
  * Calling a `run` function in *lib.rs*
  * Handling the error if `run` returns an error

This pattern is about separating concerns: *main.rs* handles running the
program, and *lib.rs* handles all the logic of the task at hand. Because you
can’t test the `main` function directly, this structure lets you test all of
your program’s logic by moving it into functions in *lib.rs*. The only code
that remains in *main.rs* will be small enough to verify its correctness by
reading it. Let’s rework our program by following this process.

#### Extracting the Argument Parser

We’ll extract the functionality for parsing arguments into a function that
`main` will call to prepare for moving the command line parsing logic to
*src/lib.rs*. Listing 12-5 shows the new start of `main` that calls a new
function `parse_config`, which we’ll define in *src/main.rs* for the moment.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```

<span class="caption">Listing 12-5: Extracting a `parse_config` function from
`main`</span>

We’re still collecting the command line arguments into a vector, but instead of
assigning the argument value at index `1` to the variable `query` and the
argument value at index `2` to the variable `filename` within the `main`
function, we pass the whole vector to the `parse_config` function. The
`parse_config` function then holds the logic that determines which argument
goes in which variable and passes the values back to `main`. We still create
the `query` and `filename` variables in `main`, but `main` no longer has the
responsibility of determining how the command line arguments and variables
correspond.

This rework may seem like overkill for our small program, but we’re refactoring
in small, incremental steps. After making this change, run the program again to
verify that the argument parsing still works. It’s good to check your progress
often, to help identify the cause of problems when they occur.

#### Grouping Configuration Values

We can take another small step to improve the `parse_config` function further.
At the moment, we’re returning a tuple, but then we immediately break that
tuple into individual parts again. This is a sign that perhaps we don’t have
the right abstraction yet.

Another indicator that shows there’s room for improvement is the `config` part
of `parse_config`, which implies that the two values we return are related and
are both part of one configuration value. We’re not currently conveying this
meaning in the structure of the data other than by grouping the two values into
a tuple; we could put the two values into one struct and give each of the
struct fields a meaningful name. Doing so will make it easier for future
maintainers of this code to understand how the different values relate to each
other and what their purpose is.

> Note: Some people call this anti-pattern of using primitive values when a
> complex type would be more appropriate *primitive obsession*.

Listing 12-6 shows the addition of a struct named `Config` defined to have
fields named `query` and `filename`. We’ve also changed the `parse_config`
function to return an instance of the `Config` struct and updated `main` to use
the struct fields rather than having separate variables:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
# use std::env;
# use std::fs::File;
#
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```

<span class="caption">Listing 12-6: Refactoring `parse_config` to return an
instance of a `Config` struct</span>

The signature of `parse_config` now indicates that it returns a `Config` value.
In the body of `parse_config`, where we used to return string slices that
reference `String` values in `args`, we now define `Config` to contain owned
`String` values. The `args` variable in `main` is the owner of the argument
values and is only letting the `parse_config` function borrow them, which means
we’d violate Rust’s borrowing rules if `Config` tried to take ownership of the
values in `args`.

We could manage the `String` data in a number of different ways, but the
easiest, though somewhat inefficient, route is to call the `clone` method on
the values. This will make a full copy of the data for the `Config` instance to
own, which takes more time and memory than storing a reference to the string
data. However, cloning the data also makes our code very straightforward
because we don’t have to manage the lifetimes of the references; in this
circumstance, giving up a little performance to gain simplicity is a worthwhile
trade-off.

> ### The Trade-Offs of Using `clone`
>
> There’s a tendency among many Rustaceans to avoid using `clone` to fix
> ownership problems because of its runtime cost. In Chapter 13, you’ll learn
> how to use more efficient methods in this type of situation. But for now,
> it’s okay to copy a few strings to continue making progress because you’ll
> make these copies only once and your filename and query string are very
> small. It’s better to have a working program that’s a bit inefficient than to
> try to hyperoptimize code on your first pass. As you become more experienced
> with Rust, it’ll be easier to start with the most efficient solution, but for
> now, it’s perfectly acceptable to call `clone`.

We’ve updated `main` so it places the instance of `Config` returned by
`parse_config` into a variable named `config`, and we updated the code that
previously used the separate `query` and `filename` variables so it now uses
the fields on the `Config` struct instead.

Now our code more clearly conveys that `query` and `filename` are related and
that their purpose is to configure how the program will work. Any code that
uses these values knows to find them in the `config` instance in the fields
named for their purpose.

#### Creating a Constructor for `Config`

So far, we’ve extracted the logic responsible for parsing the command line
arguments from `main` and placed it in the `parse_config` function. Doing so
helped us to see that the `query` and `filename` values were related and that
relationship should be conveyed in our code. We then added a `Config` struct to
name the related purpose of `query` and `filename` and to be able to return the
values’ names as struct field names from the `parse_config` function.

So now that the purpose of the `parse_config` function is to create a `Config`
instance, we can change `parse_config` from a plain function to a function
named `new` that is associated with the `Config` struct. Making this change
will make the code more idiomatic. We can create instances of types in the
standard library, such as `String`, by calling `String::new`. Similarly, by
changing `parse_config` into a `new` function associated with `Config`, we’ll
be able to create instances of `Config` by calling `Config::new`. Listing 12-7
shows the changes we need to make:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
# use std::env;
#
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

# struct Config {
#     query: String,
#     filename: String,
# }
#
// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```

<span class="caption">Listing 12-7: Changing `parse_config` into
`Config::new`</span>

We’ve updated `main` where we were calling `parse_config` to instead call
`Config::new`. We’ve changed the name of `parse_config` to `new` and moved it
within an `impl` block, which associates the `new` function with `Config`. Try
compiling this code again to make sure it works.

### Fixing the Error Handling

Now we’ll work on fixing our error handling. Recall that attempting to access
the values in the `args` vector at index `1` or index `2` will cause the
program to panic if the vector contains fewer than three items. Try running the
program without any arguments; it will look like this:

```text
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1
but the index is 1', src/main.rs:29:21
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

The line `index out of bounds: the len is 1 but the index is 1` is an error
message intended for programmers. It won’t help our end users understand what
happened and what they should do instead. Let’s fix that now.

#### Improving the Error Message

In Listing 12-8, we add a check in the `new` function that will verify that the
slice is long enough before accessing index `1` and `2`. If the slice isn’t
long enough, the program panics and displays a better error message than the
`index out of bounds` message.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// --snip--
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // --snip--
```

<span class="caption">Listing 12-8: Adding a check for the number of
arguments</span>

This code is similar to the `Guess::new` function we wrote in Listing 9-9, where
we called `panic!` when the `value` argument was out of the range of valid
values. Instead of checking for a range of values here, we’re checking that the
length of `args` is at least `3` and the rest of the function can operate under
the assumption that this condition has been met. If `args` has fewer than three
items, this condition will be true, and we call the `panic!` macro to end the
program immediately.

With these extra few lines of code in `new`, let’s run the program without any
arguments again to see what the error looks like now:

```text
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep`
thread 'main' panicked at 'not enough arguments', src/main.rs:30:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This output is better: we now have a reasonable error message. However, we also
have extraneous information we don’t want to give to our users. Perhaps using
the technique we used in Listing 9-9 isn’t the best to use here: a call to
`panic!` is more appropriate for a programming problem rather than a usage
problem, as discussed in Chapter 9. Instead, we can use the other technique you
learned about in Chapter 9—returning a `Result` that indicates either success
or an error.

#### Returning a `Result` from `new` Instead of Calling `panic!`

We can instead return a `Result` value that will contain a `Config` instance in
the successful case and will describe the problem in the error case. When
`Config::new` is communicating to `main`, we can use the `Result` type to
signal there was a problem. Then we can change `main` to convert an `Err`
variant into a more practical error for our users without the surrounding text
about `thread 'main'` and `RUST_BACKTRACE` that a call to `panic!` causes.

Listing 12-9 shows the changes we need to make to the return value of
`Config::new` and the body of the function needed to return a `Result`. Note
that this won’t compile until we update `main` as well, which we’ll do in the
next listing.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```

<span class="caption">Listing 12-9: Returning a `Result` from
`Config::new`</span>

Our `new` function now returns a `Result` with a `Config` instance in the
success case and a `&'static str` in the error case. Recall from “The Static
Lifetime” section in Chapter 10 that `&'static str` is the type of string
literals, which is our error message type for now.

We’ve made two changes in the body of the `new` function: instead of calling
`panic!` when the user doesn’t pass enough arguments, we now return an `Err`
value, and we’ve wrapped the `Config` return value in an `Ok`. These changes
make the function conform to its new type signature.

Returning an `Err` value from `Config::new` allows the `main` function to
handle the `Result` value returned from the `new` function and exit the process
more cleanly in the error case.

#### Calling `Config::new` and Handling Errors

To handle the error case and print a user-friendly message, we need to update
`main` to handle the `Result` being returned by `Config::new`, as shown in
Listing 12-10. We’ll also take the responsibility of exiting the command line
tool with a nonzero error code from `panic!` and implement it by hand. A
nonzero exit status is a convention to signal to the process that called our
program that the program exited with an error state.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
```

<span class="caption">Listing 12-10: Exiting with an error code if creating a
new `Config` fails</span>

In this listing, we’ve used a method we haven’t covered before:
`unwrap_or_else`, which is defined on `Result<T, E>` by the standard library.
Using `unwrap_or_else` allows us to define some custom, non-`panic!` error
handling. If the `Result` is an `Ok` value, this method’s behavior is similar
to `unwrap`: it returns the inner value `Ok` is wrapping. However, if the value
is an `Err` value, this method calls the code in the *closure*, which is an
anonymous function we define and pass as an argument to `unwrap_or_else`. We’ll
cover closures in more detail in Chapter 13. For now, you just need to know
that `unwrap_or_else` will pass the inner value of the `Err`, which in this
case is the static string `not enough arguments` that we added in Listing 12-9,
to our closure in the argument `err` that appears between the vertical pipes.
The code in the closure can then use the `err` value when it runs.

We’ve added a new `use` line to import `process` from the standard library. The
code in the closure that will be run in the error case is only two lines: we
print the `err` value and then call `process::exit`. The `process::exit`
function will stop the program immediately and return the number that was
passed as the exit status code. This is similar to the `panic!`-based handling
we used in Listing 12-8, but we no longer get all the extra output. Let’s try
it:

```text
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48 secs
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

Great! This output is much friendlier for our users.

### Extracting Logic from `main`

Now that we’ve finished refactoring the configuration parsing, let’s turn to
the program’s logic. As we stated in “Separation of Concerns for Binary
Projects”, we’ll extract a function named `run` that will hold all the logic
currently in the `main` function that isn’t involved with setting up
configuration or handling errors. When we’re done, `main` will be concise and
easy to verify by inspection, and we’ll be able to write tests for all the
other logic.

Listing 12-11 shows the extracted `run` function. For now, we’re just making
the small, incremental improvement of extracting the function. We’re still
defining the function in *src/main.rs*.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// --snip--
```

<span class="caption">Listing 12-11: Extracting a `run` function containing the
rest of the program logic</span>

The `run` function now contains all the remaining logic from `main`, starting
from reading the file. The `run` function takes the `Config` instance as an
argument.

#### Returning Errors from the `run` Function

With the remaining program logic separated into the `run` function, we can
improve the error handling, as we did with `Config::new` in Listing 12-9.
Instead of allowing the program to panic by calling `expect`, the `run`
function will return a `Result<T, E>` when something goes wrong. This will let
us further consolidate into `main` the logic around handling errors in a
user-friendly way. Listing 12-12 shows the changes we need to make to the
signature and body of `run`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

<span class="caption">Listing 12-12: Changing the `run` function to return
`Result`</span>

We’ve made three significant changes here. First, we changed the return type of
the `run` function to `Result<(), Box<Error>>`. This function previously
returned the unit type, `()`, and we keep that as the value returned in the
`Ok` case.

For the error type, we used the *trait object* `Box<Error>` (and we’ve brought
`std::error::Error` into scope with a `use` statement at the top). We’ll cover
trait objects in Chapter 17. For now, just know that `Box<Error>` means the
function will return a type that implements the `Error` trait, but we don’t
have to specify what particular type the return value will be. This gives us
flexibility to return error values that may be of different types in different
error cases.

Second, we’ve removed the calls to `expect` in favor of `?`, as we talked about
in Chapter 9. Rather than `panic!` on an error, `?` will return the error value
from the current function for the caller to handle.

Third, the `run` function now returns an `Ok` value in the success case. We’ve
declared the `run` function’s success type as `()` in the signature, which
means we need to wrap the unit type value in the `Ok` value. This `Ok(())`
syntax might look a bit strange at first, but using `()` like this is the
idiomatic way to indicate that we’re calling `run` for its side effects only;
it doesn’t return a value we need.

When you run this code, it will compile but will display a warning:
```text
warning: unused `std::result::Result` which must be used
  -[!!!REMOVE THIS!!!]-> src/main.rs:18:5
   |
18 |     run(config);
   |     ^^^^^^^^^^^^
= note: #[warn(unused_must_use)] on by default
```

Rust tells us that our code ignored the `Result` value and the `Result` value
might indicate that an error occurred. But we’re not checking to see whether or
not there was an error, and the compiler reminds us that we probably meant to
have some error handling code here! Let’s rectify that problem now.

#### Handling Errors Returned from `run` in `main`

We’ll check for errors and handle them using a technique similar to one we used
with `Config::new` in Listing 12-10, but with a slight difference:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

We use `if let` rather than `unwrap_or_else` to check whether `run` returns an
`Err` value and call `process::exit(1)` if it does. The `run` function doesn’t
return a value that we want to `unwrap` in the same way that `Config::new`
returns the `Config` instance. Because `run` returns `()` in the success case,
we only care about detecting an error, so we don’t need `unwrap_or_else` to
return the unwrapped value because it would only be `()`.

The bodies of the `if let` and the `unwrap_or_else` functions are the same in
both cases: we print the error and exit.

### Splitting Code into a Library Crate

Our `minigrep` project is looking good so far! Now we’ll split the
*src/main.rs* file and put some code into the *src/lib.rs* file so we can test
it and have a *src/main.rs* file with fewer responsibilities.

Let’s move all the code that isn’t the `main` function from *src/main.rs* to
*src/lib.rs*:

* The `run` function definition
* The relevant `use` statements
* The definition of `Config`
* The `Config::new` function definition

The contents of *src/lib.rs* should have the signatures shown in Listing 12-13
(we’ve omitted the bodies of the functions for brevity). Note that this won’t
compile until we modify *src/main.rs* in the listing after this one.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // --snip--
}
```

<span class="caption">Listing 12-13: Moving `Config` and `run` into
*src/lib.rs*</span>

We’ve made liberal use of the `pub` keyword: on `Config`, on its fields and its
`new` method, and on the `run` function. We now have a library crate that has a
public API that we can test!

Now we need to bring the code we moved to *src/lib.rs* into the scope of the
binary crate in *src/main.rs*, as shown in Listing 12-14:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}
```

<span class="caption">Listing 12-14: Bringing the `minigrep` crate into the
scope of *src/main.rs*</span>

To bring the library crate into the binary crate, we use `extern crate
minigrep`. Then we add a `use minigrep::Config` line to bring the `Config` type
into scope, and we prefix the `run` function with our crate name. Now all the
functionality should be connected and should work. Run the program with `cargo
run` and make sure everything works correctly.

Whew! That was a lot of work, but we’ve set ourselves up for success in the
future. Now it’s much easier to handle errors, and we’ve made the code more
modular. Almost all of our work will be done in *src/lib.rs* from here on out.

Let’s take advantage of this newfound modularity by doing something that would
have been difficult with the old code but is easy with the new code: we’ll
write some tests!
-->
