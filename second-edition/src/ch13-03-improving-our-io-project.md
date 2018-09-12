## I/O 프로젝트 개선하기

반복자에 대한 새로운 지식을 사용하여 12장의 I/O 프로젝트의 코드들을 더 깔끔하고
간결하게 개선할 수 있습니다. 반복자를 사용하여 어떻게 `Config::new` 함수와
`search` 함수의 구현을 개선할 수 있는지 살펴봅시다.


### 반복자를 사용하여 `clone` 제거하기

리스트 12-6 에서, `String` 값의 슬라이스를 받고 슬라이스를 인덱싱하고 복사
함으로써 `Config` 구조체의 인스턴스를 생성하였고, `Config` 구조체가 이 값들을
소유하도록 했습니다. 리스트 13-24 에서는 리스트 12-23 에 있던 것 처럼
`Config::new` 함수의 구현을 다시 재현 했습니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

<span class="caption">리스트 13-24: 리스트 12-23 의 `Config::new` 함수 재현
</span>

그 당시, 비효율적인 `clone` 호출에 대해 걱정하지 말라고 얘기 했으며 미래에
없앨 것이라고 했습니다. 자, 그때가 되었습니다!

`String` 요소들의 슬라이스를 `args` 파라미터로 받았지만 `new` 함수는 `args` 를
소유하지 않기 때문에 `clone` 이 필요했습니다. `Config` 인스턴스의 소유권을
반환하기 위해, `Config` 의 `query` 와 `filename` 필드로 값을 복제 함으로써
`Config` 인스턴스는 그 값들을 소유할 수 있습니다.

반복자에 대한 새로운 지식으로, 인자로써 슬라이스를 빌리는 대신 반복자의 소유권을
갖도록 `new` 함수를 변경할 수 있습니다. 슬라이스의 길이를 체크하고 특정 위치로
인덱싱을 하는 코드 대신 반복자의 기능을 사용할 것 입니다. 이것은 반복자가 값에
접근 할 것이기 때문에 `Config::new` 함수가 무엇을 하는지를 명확하게 해줄 것
입니다.

`Config::new` 가 반복자의 소유권을 갖고 빌린 값에 대한 인뎅싱을 사용하지 않게
된다면, `clone` 을 호출하고 새로운 할당을 만드는 대신 `String` 값들을 반복자에서
`Config` 로 이동할 수 있습니다.


#### 반환된 반복자를 직접 사용하기

I/O 프로젝트의 *src/main.rs* 파일을 열어보면, 아래와 같을 것 입니다:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

우리는 리스트 12-24 에 있는 `main` 함수의 시작점을 리스트 13-25 에 있는 코드로
바꿀 것 입니다. 이것은 `Config::new` 도 업데이트 해야 컴파일 됩니다. 

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

<span class="caption">리스트 13-25: `Config::new` 로 `env::args` 의 반환값
넘기기</span>

`env::args` 함수는 반복자를 반환 합니다! 반복자의 값들을 벡터로 모아서
`Config::new` 에 슬라이스를 넘기는 대신, `env::args` 에서 반환된 반복자의 소유권
을 `Config::new` 로 직접 전달 합니다.

그 다음, `Config::new` 정의를 업데이트 할 필요가 있습니다. I/O 프로젝트의
*src/lib.rs* 파일에서, 리스트 13-26 처럼 `Config::new` 의 시그니처를 변경
합시다. 함수 본문을 업데이트 해야 하기 때문이 아직 컴파일 되지 않습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // --snip--
```

<span class="caption">리스트 13-26: 반복자를 받도록 `Config::new` 의 시그니처
업데이트 하기</span>

`env::args` 함수에 대한 표준 라이브러리 문서에는 반환하는 반복자의 타입이
`std::env::Args` 라고 명시되어 있습니다. `Config::new` 함수의 시그니처를 업데이
트 해서 `args` 파리미터가 `&[String]` 대신 `std::env::Args` 타입을 갖도록
했습니다. `args` 의 소유권을 갖고 그것을 순회하면서 `args` 를 변경할 것이기
때문에, 변경 가능하도록 하기 위해 `args` 파라미터의 명세에 `mut` 키워드를 추가
할 수 있습니다.

#### 인덱싱 대신 `Iterator` 트레잇 메서드 사용하기

다음으로, `Config::new` 의 본문을 수정 할 것입니다. 표준 라이브러리 문서에는
`std::env::Args` 이 `Iterator` 트레잇을 구현하고 있다는 것 역시 언급하고 있으
므로, `next` 메서드를 호출 할 수 있다는 것을 알 수 있습니다! 리스트 13-27 은
리스트 12-23 의 코드에서 `next` 메서드를 사용하도록 변경 합니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
# fn main() {}
# use std::env;
#
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }
#
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

<span class="caption">리스트 13-27: 반복자 메서드들을 사용하도록 `Config::new`
의 본문 변경하기</span>

`env::args` 반환값의 첫번째 값은 프로그램 이름이라는 것을 명심하세요.
우리는 첫번째 값을 무시하고 그 다음 값을 얻기 위해 우선 `next` 를 호출한
다음, 그 반환값으로 아무것도 하지 않았습니다. 두번째로, `Config` 의 `query` 에
원하는 값을 넣기 위해 `next` 를 호출 했습니다. `next` 가 `Some` 을 반환하면,
값을 추출하기 위해 `match` 를 사용 합니다. 만약 `None` 을 반환하면, 이것은
충분한 인자가 넘어오지 않았다는 것을 의미하고, `Err` 값과 함께 조기 반환을
합니다. `filename` 값도 동일하게 처리 합니다.

### 반복자 어댑터로 더 간결한 코드 만들기

I/O 프로젝트의 `search` 함수에도 반복자의 장점을 활용할 수 있습니다.
리스트 12-19 의 코드가 리스트 13-28 에 재현되어 있습니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

<span class="caption">리스트 13-28: 리스트 12-19 의 `search` 함수 구현</span>

우리는 반복자 어댑터 메서드를 사용해서 이 코드를 더 간결한 방식으로 작성할 수
있습니다. 이렇게 함으로써 `results` 벡터가 변경 가능한 중간 상태를 갖는 것을
피할 수 있습니다. 함수형 프로그래밍 스타일은 더 깔끔한 코드를 만들기 위해
변경 가능한 상태의 양을 최소화 하는 것을 선호 합니다. 가변 상태를 제거하면
`results` 벡터에 대한 동시 접근을 관리 할 필요가 없기 때문에, 추후에 검색을
병렬로 수행하는 것과 같은 향상이 가능해 집니다. 리스트 13-29 는 이 변경을
보여줍니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

<span class="caption">리스트 13-29: `search` 함수 구현에서 반복자 어댑터 메서드
사용하기</span>

`search` 함수의 목적은 `query` 를 포함하는 `contents` 의 모든 줄을 반환하는 것임
을 기억하세요. 리스트 13-19 의 `filter` 예제와 유사하게, 이 코드는
`line.contains(query)` 이 `true` 를 반환하는 줄들만 유지하기 위해 `filter` 어댑
터를 사용 합니다. 그러고나서 `collect` 를 통해서 일치하는 줄들을 모아 새로운
벡터로 만듭니다. 훨씬 단순합니다! `search_case_insensitive` 도 역시 반복자
메서드들을 사용하도록 같은 변경을 자유롭게 만들어 보세요.

다음 논리적 질문은 당신의 코드에서 어떤 스타일을 선택하는 것이 좋은지와 그 이유
입니다: 리스트 13-28 의 최초 구현 혹은 리스트 13-29 의 반복자를 사용하는 버전.
대부분의 러스트 프로그래머는 반복자 스타일을 선호 합니다. 처음 사용하기는
다소 어렵습니다만, 다양한 반복자 어댑터와 어떤 일을 하는지에 대해 한번 감이
온다면, 반복자들은 이해하기 쉬워질 것 입니다. 루핑과 새로운 벡터 생성과 같은
다양한 작업을 수행하는 대신, 코드는 루프의 고차원적 목표에 집중 합니다.
이것은 아주 흔한 코드의 일부를 추상화해서 제거함으로써 반복자의 각 요소가 반드시
통과 해야하는 필터링 조건과 같이 이 코드에 유일한 개념을 더 쉽게 볼 수 있도록
합니다.

그러나 두 구현은 정말 동일 할까요? 직관적으로 저수준의 루프가 더 빠르다고 가정할
수도 있습니다. 그럼 성능에 대해서 얘기해 봅시다.
