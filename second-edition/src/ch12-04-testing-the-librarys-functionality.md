## 테스트 주도 개발로 라이브러리의 기능 개발하기

*src/lib.rs*으로 로직을 추출하고 *src/main.rs*에 인수 수집 및 에러 처리를 남겨 두었으므로 우리의 핵심 기능 코드에 대한 테스트를 작성하는 것이 훨씬 쉬워졌습니다. 커맨드라인에서 바이너리를 실행할 필요없이 다양한 인수를 사용하여 함수를 직접 호출하고 반환 값을 확인할 수 있습니다. 자신이 만든 `Config::new`와 `run`함수의 기능에 대해 몇 가지 테스트를 작성하면서 자유도를 느껴보세요. 

이 섹션에서는 TDD(Test Driven Development) 프로세스에 따라 `minigrep`에 검색 로직을 추가합니다. 해당 소프트웨어 개발 기법은 다음의 단계를 따릅니다:

1. 실패할 테스트를 작성하고, 의도한 대로 실패하는지 실행해보세요. 
2. 새 테스트를 통과하기 충분할 정도로 코드를 작성하거나 수정하세요. 
3. 추가하거나 수정하는 정도의 리팩토링을 해보고, 여전히 테스트를 통과하는지 확인해보세요. 
4. 1단계로 반복!

이것은 소프트웨어를 작성하는 여러 가지 방법 중 하나지만 TDD는 코드 설계를 좋은 상태로 유지시켜 줍니다. 코드를 작성하기 전에 테스트를 작성하고 테스트를 통과시키면 높은 테스트 범위를 유지하는데 도움이 됩니다. 테스트 패스를 작성하는 코드를 작성하기 전에 테스트를 작성하면 프로세스 전체에서 높은 테스트 적용 범위를 유지하는 데 도움이 됩니다.

우리는 실제로 파일 내용에서 쿼리 문자열을 검색하고 쿼리와 일치하는 줄의 목록을 생성하는 기능의 구현을 테스트 주도로 개발해 볼 겁니다. 이 기능을 `search`라는 함수에 추가 할 것입니다.

### 실패 테스트 작성하기

더 이상 필요하지 않으므로 프로그램의 동작을 확인하는 데 사용했던 *src/lib.rs* 및 *src/main.rs *에서 `println!`문을 제거해 봅시다. 그런 다음 *src/lib.rs*에 11 장에서 했던 것처럼 test 함수가 있는 `test` 모듈을 추가 할 것입니다. test 함수는 `search` 함수에 필요한 동작을 지정합니다. 쿼리와 텍스트를 가져 와서 쿼리를 검색하고 쿼리를 포함하는 텍스트의 줄만 반환합니다. 항목 12-15는 아직 컴파일되지 않는 이 테스트를 보여줍니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
# fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
```

<span class="caption">Listing 12-15: Creating a failing test for the `search`
function we wish we had</span>

이 테스트는 “duct.”라는 문자열을 검색합니다. 우리가 검색하는 텍스트는 세 줄로, 한 줄은 “duct.”를 포함합니다. 우리는 `search` 함수에서 반환하는 값이 우리가 예상한 줄이어야 한다고 단정했습니다(assert).

테스트가 컴파일되지 않기 때문에 우리는 이 테스트를 실행할 수 없으며 `search` 함수가 아직 존재하지 않습니다! 이제 우리는 항목 12-16에서 보듯이 항상 빈 벡터를 반환하는 `search` 함수의 정의를 추가하여 컴파일과 실행하기에 충분한 코드를 추가 할 것입니다. 빈 벡터가 `"safe, fast, productive."`줄을 포함하는 벡터와 일치하지 않기 때문에 테스트는 컴파일되지만 실패해야 합니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

<span class="caption">항목 12-16: 우리 테스트를 컴파일 하기 위해 필요한 `search` 정의.
</span>

`search`의 선언부에는 필요한 명시적인 라이프타임 `'a`가 `contents` 인자, 그리고 반환 값과 함께 사용됩니다. 10 장에서 인자의 라이프타임으로 라이프타임 값이 매개변수로 명시된 경우 반환되는 값의 라이프타임도 연결된다고 했던 점을 상기하십시오. 이 경우 반환된 벡터는 인자로 받은 `contents`를 참조하는 문자열 조각들이 포함되어 있어야 합니다. (`query` 인자가 아니라)

다른 말로 하자면, `search`함수로 반환되는 데이터는 `search`함수로 전달된 `contents`인자만큼 오래 유지될 것이라고 Rust에게 말해주는 겁니다. 이것이 중요합니다! 조각들에 *의해* 참조되는 데이터는 참조가 유효한 동안 유효해야 하기 때문이죠; 만일 컴파일러가 우리가 만든 문자열 조각이 `contents`에서가 아니라 `query`에서 만들었다고 추측하면 그에 대한 안전성 검사가 제대로 수행되지 않을 겁니다. 

만약 우리가 라이프타임 어노테이션을 깜빡하고  이 함수를 컴파일하려고 시도하면, 이런 에러를 얻게 될겁니다:

```text
error[E0106]: missing lifetime specifier
 --> src/lib.rs:5:51
  |
5 | pub fn search(query: &str, contents: &str) -> Vec<&str> {
  |                                                   ^ expected lifetime
parameter
  |
  = help: this function's return type contains a borrowed value, but the
  signature does not say whether it is borrowed from `query` or `contents`
```

Rust는 두 인자 중에 우리가 필요한 쪽이 어느건지 알 수 없기 때문에, 우리가 알려줘야 합니다. `contents`가 우리의 문자들을 모두 가지고 있고 우리가 원하는 것은 그 중 일치하는 부분이기 때문에, `contents`가 라이프타임 문법을 사용하여 반환 값과 연결되어야 한다는걸 압니다. 

다른 프로그래밍 언어는 인자와 반환 값을 선언부에서 연결시키라고 요구하지 않으니, 아마 이게 낯설거고, 전체적으로 좀더 쉬울겁니다. 아마 여러분은 이 예제와 10장에서 다룬 “Validating References with Lifetimes” 장의 내용을 비교하고 싶을지도 모르겠습니다. 

이제 테스트를 실행해봅시다:


```text
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
--warnings--
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running target/debug/deps/minigrep-abcabcabc

running 1 test
test test::one_result ... FAILED

failures:

---- test::one_result stdout ----
        thread 'test::one_result' panicked at 'assertion failed: `(left ==
right)`
left: `["safe, fast, productive."]`,
right: `[]`)', src/lib.rs:48:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```

훌륭하게, 우리가 예상했던 예상대로 테스트가 실패했습니다. 테스트를 통과하게 만들어봅시다!

### 테스트를 통과하는 코드 작성

현재는, 우리가 늘 빈 벡터를 반환하니까 테스트가 실패하게 됩니다. 이를 수정하고 `search`를 구현하기 위해, 우리의 프로그램은 다음 단계를 따를 필요가 있습니다. 

* contents의 각 줄에 대한 반복작업
* 해당 줄에 우리의 쿼리 문자열이 포함되어 있는지 검사 
* 그렇다면, 우리가 반환할 값 목록에 추가
* 그렇지 않다면, 통과 
* 일치하는 결과 목록을 반환 

각 단계를 밟아가기 위해, 줄들에 대한 반복작업부터 시작합시다!

#### `lines` 메소드를 사용하여 줄들에 대한 반복 작업 

Rust는 문자열의 줄-단위로 반복 작업을 할 수 있는 유용한 메소드가 있는데, 편리하게 이름이 `lines`이고, 항목 12-17처럼 보여주는 것처럼 동작합니다. 아직 컴파일되지 않는다는 점에 유의하세요:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

<span class="caption">항목 12-17: `contents`의 각 줄마다 반복작업
</span>

`lines` 메소드는 반복자를 리턴합니다. 우리는 13장에서 반복자에 대해서 다루게 될 겁니다만, 항목 3-4에서 반복자를 사용하는 방법을 봤었다는걸 상기시켜 드립니다. 항목 3-4에서는 반복자와 함께 for반복문을 사용하여 컬렉션의 각 항목에 대해 임의의 코드를 수행했었습니다. 

#### Query로 각 줄을 검색하기 

다음으로 현재 줄에 쿼리 문자열이 포함되어 있는지 확인합니다. 다행스럽게도 문자열에는 유용한 'contains'라는 메소드가 있습니다. 항목 12-18과 같이 `search` 함수에서 `contains` 메소드에 대한 호출을 추가하십시오. 이 코드는 여전히 컴파일되지 않으니 주의하세요.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
        }
    }
}
```

<span class="caption">항목 12-18: 어느 줄이 `query` 문자열을 포함하고 있는지 보기 위한 기능 추가
</span>

#### 일치하는 줄 보관하기

또한 쿼리 문자열이 포함된 줄을 저장할 방법이 필요합니다. 이를 위해 우리는 `for`반복문 전에 가변 벡터를 만들고 `push` 메소드를 호출하여 벡터에 `line`을 저장합니다. 항목 12-19처럼 `for`반복문이 끝난 다음에 벡터를 반환합니다.

<span class="filename">Filename: src/lib.rs</span>

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

<span class="caption">항목 12-19: 일치하는 라인들을 저장하여 반환할 수 있게 만들기.</span>

이제 `search`함수는 `query`를 포함하는 줄들만 반환하게 되었으니 우리의 테스트는 통과되야 할 겁니다. 테스트를 실행해 봅시다:

```text
$ cargo test
--snip--
running 1 test
test test::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

우리 테스트가 통과되었으니, 제대로 동작한다는 것을 알게 되죠!

이 시점에서, 우리는 동일한 기능을 유지하기 위해 테스트를 통과시키면서 search 함수를 리팩토링할 기회를 고려해 볼 수 있게 됐습니다. search 함수가 많이 나쁘지는 않지만, 반복자의 기능들이 주는 유용함을 충분히 활용하지 못하고 있습니다. 우리는 13장에서 이 예제로 돌아와 반복자에 대해서 자세히 알아보고 어떻게 개선할 수 있는지 알아볼 겁니다. 

#### `run`함수에서 `search`함수를 사용하기 
Using the `search` Function in the `run` Function

이제 `search` 함수는 실행되고 테스트 되었지만, 우리의 `run`함수에서 `search`를 호출하게 해야 합니다. 우리는 `config.query` 값과 `run`으로 파일에서 읽어온 `contents`를 `search`함수에 전달해야 합니다. 그 이후 `run`은 `search`로부터 반환된 각 줄을 출력합니다:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
```

우리는 아직 `search`에서 `for`반복문을 사용해 각 줄을 반환하고 출력하고 있습니다. 

이제 우리의 프로그램 전체가 동작하는 것 같습니다! 확신하기 위해, 첫째로 “frog” 단어로 Emily Dickinson의 시에서 정확히 한 줄이 반환되야 합니다: 

```text
$ cargo run frog poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38 secs
     Running `target/debug/minigrep frog poem.txt`
How public, like a frog
```

좋군요! 다음으 여러 줄에 일치할 “body” 같은 단어를 해봅시다:

```text
$ cargo run body poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep body poem.txt`
I’m nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

그리고 마지막으로, 시의 어디서도 찾을 수 없는 단어 “monomorphization” 같은걸 검색하면 어떤 줄도 찾을 수 없다는걸 확인해봅시다.

```text
$ cargo run monomorphization poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep monomorphization poem.txt`
```

훌륭해! 우리는 어플리케이션의 구조화를 어떻게 수행하는지에 대해 많은 것을 배우며 고전적인 도구를 우리 자체 미니 버전으로 만들어봤습니다. 또한 우리는 파일의 입력, 출력, 라이프타임, 테스팅과 커맨드라인 파싱에 대해서도 좀 알게 되었네요.

이 프로젝트를 완벽하게 하기 위해, 환경 변수를 다루고 표준 에러를 출력하는 방법을 간단히 시연하려고 하는데, 모두 커맨드라인 프로그램을 작성하는데 유용할 겁니다. 

