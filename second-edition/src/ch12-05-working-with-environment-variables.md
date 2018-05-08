## 환경 변수들을 활용하기

우리는 추가 기능을 구현하여 `minigrep`을 향상시키려고 합니다. 대소문자를 구분하여 검색할지를 선택할 수 있는 기능인데,
사용자가 환경 변수를 사용하여 키고 끌 수 있게 할 수 있도록 하려 합니다. 우리는 해당 기능을 명령줄 옵션으로 구현하고 사용자가 
원할때마다 해당 옵션을 기입하게 만들 수도 있지만, 대신 환경 변수를 사용하게 할 수도 있습니다. 이를 통해 사용자가 한번 환경변수를
설정하는 것을 통해 현재 터미널 세션에서 하는 모든 검색이 대소문자를 구분하게 만듭니다.

### 대소문자를 구분하는 `search` 함수의 실패 케이스 작성하기 

우리는 새로운 `search_case_insensitive` 함수를 추가하고, 환경 변수가 적용되어 있으면 호출하고자 합니다. 우리는 TDD
절차를 따르고자 하니, 우리는 먼저 실패 테스트를 작성해야 합니다.  우리는 새 테스트를 새 `search_case_insensitive`를
위해 작성하고 예전에 작성한 테스트 `one_result`를 `case_sensitive`로 이름을 바꿔 두 테스트 간의 차이점을 명확하게 
합니다. 항목 12-20에서는 이를 보여줍니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

<span class="caption">항목 12-20: 새로운 실패 테스트를 우리가 추가할 대소문자 구문 함수를 위해 추가</span>

우리가 예전 테스트의 `contents`도 바꿨음을 주의하세요. 우리는 `“Duct tape”`라는 대문자 D로 시작되는 새로운 
문자를 추가해 대소문자 구분 시에 쿼리 “duct”으로는 검색되지 않도록 하였습니다. 이러한 방식으로 이전 테스트를 변경하면 
이미 구현한 대소문자 구분 검색 기능을 실수로 손상시키지 않게됩니다. 이 테스트는 지금 통과해야하며 우리가 작업을 마친 이후에도
대소문자를 구분하지 않는 검색 시에 통과되어야 합니다.

대소문자를 구분하지 않는 검색을 위해 새로 추가된 테스트는 “rUsT”를 쿼리로 사용합니다. 우리가 추가할 함수 
`search_case_insensitive`는 “rUsT”가 대문자 R이 포함된 “Rust:”에 그리고 “Trust me.”처럼 쿼리와 
다른 경우에도 일치될 겁니다. 이건 우리가 만든 `search_case_insensitive` 함수의 실패 테스트이고, 우리가 아직 
함수를 추가하지 않았기 때문에 컴파일은 실패할 겁니다. 우리는 search` 함수를 추가할 때와 비슷한 방식으로 빈 벡터를 반환하는 
뼈대를 자유롭게 추가하면 됩니다. 항목 12-16에서 테스트의 컴파일과 실패를 볼 수 있습니다.

 
### `search_case_insensitive` 함수 구현하기

항목 12-21에서 보여주는 `search_case_insensitive`는 `search` 함수와 거의 같습니다. 유일하게 다른 점은
`query`와 각 `line`을 소문자로 만들어 인자의 대소문자 여부와 무관하게 동일한 문자가 각 라인에 존재하는지 검사할 수 
있게 만든겁니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

<span class="caption">항목 12-21: `search_case_insensitive` 함수를 정의해 query와 line을 
query와 line을 비교하기 전에 소문자로 변경.</span>

첫 째, 소문자화 한 `query` 문자열을 동일한 이름을 가진 그림자 변수에 보관합니다. `to_lowercase`를 쿼리에서 호출하면
사용자의 쿼리가 “rust”, “RUST”, “Rust”, 혹은 “rUsT”인지 구분할 필요가 없어지고, 우리는 사용자 쿼리가 “rust” 
로 간주하고 대소문자 구문을 하지 않을 겁니다.

`to_lowercase` 호출은 기존 데이터를 참조하는 것이 아니라 새로운 데이터를 생성기 때문에 `query`는 문자열 슬라이스가 아닌
`String`입니다. 예로 들었던 쿼리 “rUsT” 문자열 slice에는 우리가 사용할 “u” 또는 “t” 소문자가 없으므로 “rust”가 
포함 된 새 `String`을 할당해야 합니다. 우리가 `contains` 메소드에 인자로 `query`를 전달할 때 `contains`의 
선언이 문자열 slice를 인자로 받게 정의되어 있으니 앰퍼샌드(&)를 추가해야합니다.

다음으로, 우리는 각 `line`에 모두 소문자로 이뤄진 `query`가 존재하는지 검사하기 전에 `to_lowercase`를 호출합니다.
이제 `line`과 `query`를 모두 소문자로 변경했으니, 대소문자 구분없이 매치되는 문자열을 검색할 수 있습니다. 

해당 구현이 테스트들을 통과하는지 한번 보시죠.

```text
running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

시원하게 통과했습니다. 이제 `run` 함수에서 신상 `search_case_insensitive`를 호출해보자구요. 먼저 
`Config` 구조체에 검색을 시에 대소문자를 구분할지 설정 옵션을 추가부터 하구요. 근데 이 필드를 추가하면 컴파일러가
필드 값을 초기화 하지 않았다고 에러를 내게 되요.

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

우리는 불린 값을 갖는 `case_sensitive`를 추가했어요. 다음으로, 우리는 `run` 함수를 실행해서
`case_sensitive` 필드의 값을 확인한 뒤에 `search` 함수와 `search_case_insensitive`
함수 중에 어느 쪽을 호출 할 것인지 결정하면 되요, 항목 12-22처럼 말이죠. 아직도 컴파일은 안되욧!

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::error::Error;
# use std::fs::File;
# use std::io::prelude::*;
#
# fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
# fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }
#
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

<span class="caption">항목 12-22: `config.case_sensitive`의 값을 기준으로  
`search` 혹은 `search_case_insensitive`이 호출됩니다. </span>

마지막으로, 우리는 환경 변수를 검사해야 해요. 환경 변수를 다루기 위한 함수들은 `env`모듈이 있는 표준 라이브러리에
있어요, 그래서 우리는 `use std::env;`을 *src/lib.rs*의 최상단에 추가해서 현재 범위로 끌어오려고 해요. 
그러면 우리는 `env`에 있는 `var`메소드를 사용하여 `CASE_INSENSITIVE`란 이름의 환경변수를 검사할 수 있죠.
항목 12-23에서 보이듯 말이에요.

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::env;
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }

// --snip--

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

<span class="caption">항목 12-23: `CASE_INSENSITIVE`란 이름의 환경변수 검사하기</span>

여기서 우리는 `case_sensitive`라는 새 변수를 만들어요. 그의 값을 설정하려고, `env::var` 함수를 호출하고 
`CASE_INSENSITIVE`란 환경변수의 이름을 전달하죠. `env::var` 메소드는 `Result`를 반환하는데, 만약
환경변수가 설정된 상태라면 환경 변수의 값을 포함한 성공한 `Ok` 변형체가, 만약 설정되지 않았다면 `Err` 변형체를
반환하게 됩니다. 

우리는 `Result`의 `is_err` 메소드를 에러이며 설정되지 않은 상태라서 대소문자를 구분하는 검색을 *해야하는지*
확인하고자 사용합니다. 만약 `CASE_INSENSITIVE` 환경 변수에 뭐라도 설정이 되었으면, `is_err`는 
false를 반환하고 대소문자 구분 검색을 수행하게 될겁니다. 우리는 환경변수의 *내용*은 신경쓰지 않고, 그저 그게 설정이
되어있는지만을, `is_err`로 검사하며 `unwrap`, `expect`나 `Result`에 존재하는 다른 메소드는 
사용하지 않았어요.

항목 12-22에서 구현했던 것처럼 `case_sensitive` 변수의 값을 Config 인스턴스에 전달하여 `run` 
함수가 해당 값을 읽고 `search_case_insensitive` 또는 `search` 를 호출할지 여부를 결정할 수 
있도록 합니다.

이제 돌려보죠! 처음에는 프로그램을 환경변수 설정없이 “to” 쿼리와 함께 실행하면, 소문자 “to” 를 포함하는 
모든 줄이 일치되게 됩니다. 

```text
$ cargo run to poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

잘 동작하고 있네요! 이제, 프로그램을 `CASE_INSENSITIVE`를 `1`로 설정하지만 쿼리는 동일한 “to”로 
실행해볼까요. 

PowerShell을 사용하는 경우 환경 변수를 설정하고 둘로 나눈 명령으로 프로그램을 실행해야합니다.
```text
$ $env:CASE_INSENSITIVE=1
$ cargo run to poem.txt
```

대소문자 “to” 가 포함된 줄을 가져와야 합니다.
```text
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

훌륭하게, “To”가 포함 된 줄도 있습니다! 우리의 `minigrep` 프로그램은 이제 환경변수를 통해 대소문자를 구분하지 
않고 검색 할 수 있습니다. 이제 커맨드라인 인수나 환경변수를 사용하여 설정 옵션을 관리하는 방법을 알게 되었네요!

일부 프로그램은 동일 설정에 대해 인수, *그리고* 환경변수를 모두 허용합니다. 이 경우 프로그램은 둘 중 하나의 우선 
순위를 결정합니다. 또다른 독자 연습의 일환으로, 커맨드라인 인수와 환경변수를 통해 대소문자 구분을 제어 해보세요. 
프로그램이 하나는 대소문자를 구분하고 다른 하나는 구분하지 않도록 설정되어 실행된다면 커맨드라인 인자와 환경변수 중에 
어느쪽에 우선순위를 둘지 결정해보세요. 


`std::env` 모듈에는 환경 변수를 다루는 데 유용한 여러 가지 기능이 있으니 
사용 가능한 내용을 보려면 문서를 확인하세요.

<!-- 업데이트된 원본:
## Working with Environment Variables

We’ll improve `minigrep` by adding an extra feature: an option for
case-insensitive searching that the user can turn on via an environment
variable. We could make this feature a command line option and require that
users enter it each time they want it to apply, but instead we’ll use an
environment variable. Doing so allows our users to set the environment variable
once and have all their searches be case insensitive in that terminal session.

### Writing a Failing Test for the Case-Insensitive `search` Function

We want to add a new `search_case_insensitive` function that we’ll call when
the environment variable is on. We’ll continue to follow the TDD process, so
the first step is again to write a failing test. We’ll add a new test for the
new `search_case_insensitive` function and rename our old test from
`one_result` to `case_sensitive` to clarify the differences between the two
tests, as shown in Listing 12-20:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

<span class="caption">Listing 12-20: Adding a new failing test for the
case-insensitive function we’re about to add</span>

Note that we’ve edited the old test’s `contents` too. We’ve added a new line
with the text `"Duct tape."` using a capital D that shouldn’t match the query
“duct” when we’re searching in a case-sensitive manner. Changing the old test
in this way helps ensure that we don’t accidentally break the case-sensitive
search functionality that we’ve already implemented. This test should pass now
and should continue to pass as we work on the case-insensitive search.

The new test for the case-*insensitive* search uses `"rUsT"` as its query. In
the `search_case_insensitive` function we’re about to add, the query `"rUsT"`
should match the line containing `"Rust:"` with a capital R and match the line
`"Trust me."` even though both have different casing than the query. This is
our failing test, and it will fail to compile because we haven’t yet defined
the `search_case_insensitive` function. Feel free to add a skeleton
implementation that always returns an empty vector, similar to the way we did
for the `search` function in Listing 12-16 to see the test compile and fail.

### Implementing the `search_case_insensitive` Function

The `search_case_insensitive` function, shown in Listing 12-21, will be almost
the same as the `search` function. The only difference is that we’ll lowercase
the `query` and each `line` so whatever the case of the input arguments,
they’ll be the same case when we check whether the line contains the query.

<span class="filename">Filename: src/lib.rs</span>

```rust
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

<span class="caption">Listing 12-21: Defining the `search_case_insensitive`
function to lowercase the query and the line before comparing them</span>

First, we lowercase the `query` string and store it in a shadowed variable with
the same name. Calling `to_lowercase` on the query is necessary so no matter
whether the user’s query is `"rust"`, `"RUST"`, `"Rust:"`, or `"rUsT"`, we’ll
treat the query as if it were `"rust"` and be insensitive to the case.

Note that `query` is now a `String` rather than a string slice, because calling
`to_lowercase` creates new data rather than referencing existing data. Say the
query is `"rUsT"`, as an example: that string slice doesn’t contain a lowercase
`u` or `t` for us to use, so we have to allocate a new `String` containing
`"rust"`. When we pass `query` as an argument to the `contains` method now, we
need to add an ampersand because the signature of `contains` is defined to take
a string slice.

Next, we add a call to `to_lowercase` on each `line` before we check whether it
contains `query` to lowercase all characters. Now that we’ve converted `line`
and `query` to lowercase, we’ll find matches no matter what the case of the
query is.

Let’s see if this implementation passes the tests:

```text
running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Great! They passed. Now, let’s call the new `search_case_insensitive` function
from the `run` function. First, we’ll add a configuration option to the
`Config` struct to switch between case-sensitive and case-insensitive search.
Adding this field will cause compiler errors since we aren’t initializing this
field anywhere yet:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

Note that we added the `case_sensitive` field that holds a Boolean. Next, we
need the `run` function to check the `case_sensitive` field’s value and use
that to decide whether to call the `search` function or the
`search_case_insensitive` function, as shown in Listing 12-22. Note this still
won’t compile yet:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::error::Error;
# use std::fs::File;
# use std::io::prelude::*;
#
# fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
# fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }
#
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

<span class="caption">Listing 12-22: Calling either `search` or
`search_case_insensitive` based on the value in `config.case_sensitive`</span>

Finally, we need to check for the environment variable. The functions for
working with environment variables are in the `env` module in the standard
library, so we want to bring that module into scope with a `use std::env;` line
at the top of *src/lib.rs*. Then we’ll use the `var` method from the `env`
module to check for an environment variable named `CASE_INSENSITIVE`, as shown
in Listing 12-23:

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::env;
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }

// --snip--

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

<span class="caption">Listing 12-23: Checking for an environment variable named
`CASE_INSENSITIVE`</span>

Here, we create a new variable `case_sensitive`. To set its value, we call the
`env::var` function and pass it the name of the `CASE_INSENSITIVE` environment
variable. The `env::var` method returns a `Result` that will be the successful
`Ok` variant that contains the value of the environment variable if the
environment variable is set. It will return the `Err` variant if the
environment variable is not set.

We’re using the `is_err` method on the `Result` to check whether it’s an error
and therefore unset, which means it *should* do a case-sensitive search. If the
`CASE_INSENSITIVE` environment variable is set to anything, `is_err` will
return false and the program will perform a case-insensitive search. We don’t
care about the *value* of the environment variable, just whether it’s set or
unset, so we’re checking `is_err` rather than using `unwrap`, `expect`, or any
of the other methods we’ve seen on `Result`.

We pass the value in the `case_sensitive` variable to the `Config` instance so
the `run` function can read that value and decide whether to call `search` or
`search_case_insensitive`, as we implemented in Listing 12-22.

Let’s give it a try! First, we’ll run our program without the environment
variable set and with the query `to`, which should match any line that contains
the word “to” in all lowercase:

```text
$ cargo run to poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

Looks like that still works! Now, let’s run the program with `CASE_INSENSITIVE`
set to `1` but with the same query `to`.

If you’re using PowerShell, you will need to set the environment variable and
run the program in two commands rather than one:

```text
$ $env:CASE_INSENSITIVE=1
$ cargo run to poem.txt
```

We should get lines that contain “to” that might have uppercase letters:

```text
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Excellent, we also got lines containing “To”! Our `minigrep` program can now do
case-insensitive searching controlled by an environment variable. Now you know
how to manage options set using either command line arguments or environment
variables.

Some programs allow arguments *and* environment variables for the same
configuration. In those cases, the programs decide that one or the other takes
precedence. For another exercise on your own, try controlling case
insensitivity through either a command line argument or an environment
variable. Decide whether the command line argument or the environment variable
should take precedence if the program is run with one set to case sensitive and
one set to case insensitive.

The `std::env` module contains many more useful features for dealing with
environment variables: check out its documentation to see what is available.
-->
