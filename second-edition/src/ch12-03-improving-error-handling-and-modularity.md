## 모듈성과 에러처리의 향상을 위한 리팩토링

우리 프로그램을 향상시키기 위해 네 가지 수정하고 싶은 문제가 있는데, 이들은 프로그램을 
구조화하고 발생가능한 에러를 처리하는 방식과 관련있습니다.

첫 번째, 우리 `main` 함수는 현재 두 가지 작업을 수행합니다: 인자들을 분석하고 파일을
열지요. 이런 작은 함수에서, 이건 큰 문제가 안됩니다. 하지만 우리가 계속해서 `main`함수
안에 프로그램을 작성하여 커지게 되면, `main` 함수가 처리하는 작업의 수도 늘어나게 될
겁니다. 함수가 갖게되는 책임들만큼, 근원을 파악하기도, 테스트 하기에도, 부분 별로 나누지
않고는 수정하기도 어려워 집니다. 함수는 나뉘어 하나의 작업에 대해서만 책임을 지는 것이 더
좋은 구조입니다.

이 문제는 우리의 두 번째 문제와도 관련이 있습니다: `query` 와 `filename` 은  
프로그램의 설정을 저장하는 변수이고 `f` 와 `contents` 같은 변수는 프로그램의 논리 
수행에 사용됩니다. `main`이 길어질수록 범위 내에 더 많은 변수 생깁니다. 범위 내에 
더 많은 변수가 존재할수록, 각각의 변수를 추적하기 힘들어집니다. 목적을 분명히 하기 위해 
설정 변수를 그룹화하여 하나의 구조로 결합시키는 것이 좋습니다.

세 번째 문제는 파일 열기가 실패 할 경우`expect`를 사용하여 오류 메시지를 출력해주는데,
에러 메시지가 `파일을 찾을 수 없음` 밖에 없습니다. 파일이 존재하지 않는 경우 외에도 파일
열기가 실패하는 경우들이 있습니다. 예를 들어 파일은 존재하지만 파일을 열 수있는 권한이 없을 수
있습니다. 현재는 이런 상황에도 `파일을 찾을 수 없음` 이란 오류 메시지를 출력하여 사용자에게
잘못된 조언을 해주게 됩니다.

넷째, 우리는 서로 다른 오류를 다루기 위해 `expect`를 반복적으로 사용하고 있습니다. 헌데
만약 사용자가 충분한 인수를 지정하지 않고 프로그램을 실행하면 Rust의 "index out of
bounds" 오류가 발생하는데 이는 문제를 명확하게 설명하지 않습니다. 우리가 모든 오류처리 
코드를 한 군데 모아놓으면 후에 관리자는 오류처리 로직을 변경해야 할 때 오직 이 곳의 코드만
참고하면 되니 더 좋죠. 또한, 모든 오류 처리 코드를 한 곳에 저장하면 우리가 최종 사용자에게
도움이 되는 메시지를 출력하고 있는지 확신하는데도 도움이 됩니다.

이런 문제들을 우리 프로젝트를 리팩토링하여 해결해보도록 하겠습니다.

### 바이너리 프로젝트를 위한 핵심기능(concern) 나누기

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

> Note: some people call this anti-pattern of using primitive values when a
> complex type would be more appropriate *primitive obsession*.

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
발생한 경우에는 문제가 무엇인지 설명할 수 있게 만들 수 있다. `Config::new`가 `main`과 
상호작용할 시에, 우리는 `Result`를 사용하여 문제가 있다고 신호할 수 있다. 그리고`main`에선
`Err`의 값을 사용자들에게 보다 실용적인 방식으로 변환하여 보여줄 수 있다. `thread 'main'`
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

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

<span class="caption">항목 12-12: `run` 함수가 `Result`를 반환하게 바꾸기 </span>

우리는 여기서 세 가지 큰 변화를 만들었습니다. 먼저, `run` 함수의 리턴 타입을 `Result 
<(), Box <Error >>`로 바꿨습니다. 이 함수는 이전에 유닛 타입 `()`을 반환했으며, 
우리는 `Ok`의 경우 반환할 값으로 이 타입을 유지합니다.

우리의 에러 타입으로, *특성 오브젝트* Box <Error>를 사용합니다 (그리고 상단에 `use`문으로 
`std::error::Error`를 범위 내로 임포트 해왔습니다). 우리는 특성 오브젝트들을 17장에서 
다룰 것입니다. 지금 당장은, `Box<Error>`는 함수가 `Error` 특성을 구현하는 타입을 
반환한다는 것만 알면 되고, 특별히 어떤 타입이 반환될지에 대해서는 알 필요 없습니다. 이런 방식은 
다양한 에러 상황에 다른 타입의 오류 값을 반환 할 수 있는 유연성을 확보할 수 있습니다. 


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

pub fn run(config: Config) -> Result<(), Box<Error>>{
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

라이브러리 크레이트를 바이너리 크레이트에 가져 오려면 `extern crate minigrep`을 
사용합니다. 그런 다음 `minigrep::Config`줄을 추가하여 `Config`타입을 범위로 가져오고 
`run` 함수 접두어에 크레이트 이름을 붙입니다. 이를 통해 모든 기능이 연결되어 있어야 하며 
작동해야 합니다. `cargo run`을 실행하여 모든 것이 올바르게 연결되어 있는지 확인하십시오.

아오! 빡시게 작업했네요, 우리의 미래를 우리 스스로가 성공의 방향으로 설정했습니다. 이제 에러를 
처리가 훨씬 쉬워졌고, 우리의 코드를 보다 모듈화하였습니다. 거의 모든 작업은 여기 
*src/lib.rs*에서 수행될 겁니다.

새롭게 확보한 모듈성을 통해 이전의 코드로는 하지 못했을 무언가를 쉽게 할 수 있는 이점을 확보했습니다:몇 개의 테스트를 작성해봅시다!
<!-- 이 내용은 https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html 에서 직접 옮겼습니다. -->
To bring the library crate into the binary crate, we use extern crate minigrep. Then we’ll add a use minigrep::Config line to bring the Config type into scope, and we’ll prefix the run function with our crate name. With that, all the functionality should be connected and should work. Give it a cargo run and make sure everything is wired up correctly.

Whew! That was a lot of work, but we’ve set ourselves up for success in the future. Now it’s much easier to handle errors, and we’ve made our code more modular. Almost all of our work will be done in src/lib.rs from here on out.

Let’s take advantage of this newfound modularity by doing something that would have been hard with our old code, but is easy with our new code: write some tests!