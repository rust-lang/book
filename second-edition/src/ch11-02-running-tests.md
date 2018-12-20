## 테스트의 실행 방식 제어하기

`cargo run`이 여러분의 코드를 컴파일하고 난 뒤 그 결과인 바이너리를 실행하는 것과 마찬가지로,
`cargo test`는 여러분의 코드를 테스트 모드에서 컴파일하고 결과로 발생한 테스트 바이너리를 실행합니다.
여러분은 커맨드 라인 옵션을 지정하여 `cargo test`의 기본 동작을 변경할 수 있습니다.
예를 들어, `cargo test`를 통해 생성된 바이너리의 기본 동작은 모든 테스트를 병렬적으로 수행하고
테스트가 실행되는 동안 생성된 결과를 캡처하는 것으로, 테스트 결과와 연관된 출력을 읽기 쉽도록 화면에
표시되는 것을 막아버립니다.

어떤 커맨드 라인 옵션은 `cargo test`에 입력되고 어떤 옵션은 결과 테스트 바이너리에 입력됩니다.
이 두 가지 타입의 인자를 구분하기 위해서, `cargo test`에 주어질 인자를 먼저
나열하고, 그다음 구분자(separator)로 `--`를 넣고, 그 뒤 테스트 바이너리에 입력될 인자를
나열합니다. `cargo test --help`를 실행하는 것은 `cargo test`에서 사용할 수 있는 옵션을
표시하고, `cargo test -- --help`를 실행하는 것은 구분자 `--` 이후에 나올 수 있는
옵션을 표시합니다.

### 테스트를 병렬 혹은 연속으로 실행하기

여러 개의 테스트를 실행할 때는, 기본적으로 스레드를 이용하여 병렬적으로 수행됩니다. 이는 테스트가
더 빠르게 실행되어 끝낼 수 있다는 의미이므로, 우리의 코드가 잘 동작하는지 혹은 그렇지 않은지에
대한 피드백을 더 빨리 얻을 수 있습니다. 테스트가 동시에 실행되므로, 여러분의 테스트가 서로
다른 테스트 혹은 공유 상태 값에 의존하지 않는지 주의해야 하는데, 이는 이를테면 현재 작업 디렉토리나
환경 변수와 같은 공유 환경 값을 포함합니다.

예를 들면, 여러분이 작성한 테스트 각각이 *test-output.txt*라는 파일을 디스크에 만들고 이 파일에
어떤 데이터를 쓰는 코드를 실행한다고 가정해봅시다. 그런 다음 각 테스트는 그 파일로부터 데이터를 읽고,
이 파일이 특정한 값을 담고 있는지 단언하는데, 이 값들은 테스트마다 다릅니다. 모든 테스트들이 동시에
실행되기 때문에, 어떤 테스트가 파일을 쓰고 읽는 동안 다른 테스트가 파일을 덮어쓸지도 모릅니다. 두 번째
테스트는 실패할 것인데, 이는 코드가 정확히 않아서가 아니라 테스트들이 병렬적으로 실행하는 동안 서로에게
간섭을 일으켰기 때문입니다. 한 가지 해결책은 각 테스트가 서로 다른 파일을 쓰도록 확실히 하는 것일 겁니다;
또 다른 해결책은 테스트를 한 번에 하나씩만 실행하는 것입니다.

만일 여러분이 테스트들을 병렬적으로 실행하고 싶지 않을 경우, 혹은 여러분이 사용되는 스레드의 개수에
대한 더 정밀한 제어를 하고 싶을 경우, 여러분은 `--test-threads` 플리그와 테스트 바이너리에서
사용하고 싶은 스레드 개수를 넘길 수 있습니다. 다음 예제를 봅시다:

```text
$ cargo test -- --test-threads=1
```

여기서는 테스트 스레드의 개수에 1을 지정했는데, 이는 프로그램이 어떠한 병렬 처리도 사용하지 않음을
얘기해줍니다. 테스트를 하나의 스레드에서 실행하는 것은 병렬로 수행하는 것에 비해 시간이 더 오래
걸리겠지만, 테스트들이 어떤 상태를 공유할 경우 서로가 간섭할 가능성이 없어질 것입니다.

### 함수 결과 보여주기

기본적으로 어떤 테스트가 통과하면, 러스트의 테스트 라이브러리는 표준 출력(standard output)으로
출력되는 어떤 것이든 캡처합니다. 예를 들면, 우리가 테스트 내에서 `println!`을 호출하고 이 테스트가
통과하면, `println!` 출력을 터미널에서 볼 수 없습니다: 우리는 오직 그 테스트가 통과되었다고 표시된
라인만 볼 뿐입니다. 만일 테스트가 실패하면, 실패 메세지 아래에 표준 출력으로 출력되었던 어떤 것이든
보게 될 것입니다.

예를 들어, Listing 11-10은 파라미터의 값을 출력한 뒤 10을 반환하는 바보 같은 함수를 보여주고
있습니다. 그리고 통과하는 테스트와 실패하는 테스트를 갖추고 있습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

<span class="caption">Listing 11-10: `println!`을 호출하는 함수를 위한 테스트
</span>

`cargo test`를 이용하여 이 테스트를 실행했을 때 보게 될 출력은 다음과 같습니다:

```text
running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
        I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

`I got the value 4`라는 메세지를 어디에서도 볼 수 없는데, 이는 성공하는 테스트가 실행시키는
출력이라는 점을 주목하세요. 이 출력 메세지는 캡처되었습니다. 실패한 테스트로부터 얻어진 출력 메세지인
`I got the value 8`은 테스트 정리 출력 부분에 나타나는데, 이는 테스트 실패 원인 또한 함께 보여줍니다.

만일 성공하는 테스트에 대한 출력 값 또한 볼 수 있기를 원한다면, `--nocapture` 플래그를 이용하여
출력 캡처 동작을 비활성화시킬 수 있습니다:

```text
$ cargo test -- --nocapture
```

Listing 11-10의 테스트를 `--nocapture` 플래그와 함께 실행시키면 다음과 같이 나옵니다:

```text
running 2 tests
I got the value 4
I got the value 8
test tests::this_test_will_pass ... ok
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
test tests::this_test_will_fail ... FAILED

failures:

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

테스트에서의 출력과 테스트 결과 출력이 분리된 점을 주목하세요; 이는 우리가 이전 절에서 다룬 내용처럼
테스트가 병렬적으로 수행되기 때문입니다. `--test-threads=1` 옵션과 `--nocapture` 기능을
동시에 시도하고 출력이 어떻게 바뀌는지를 확인해 보세요!

### 이름으로 테스트의 일부분만 실행하기

가끔, 모든 테스트 셋을 실행하는 것은 시간이 오래 걸릴 수 있습니다. 만일 여러분이 특정 영역의 코드에
대해서 작업하고 있다면, 그 코드와 연관된 테스트만 실행시키고 싶어 할 수도 있습니다. 여러분은
`cargo test`에 여러분이 실행시키고 싶어 하는 테스트(들)의 이름들을 인자로 넘김으로써 어떤
테스트들을 실행시킬지 고를 수 있습니다.

테스트의 일부분만을 실행시키는 법을 보여드리기 위해서, Listing 11-11에서 보시는 바와 같이
`add_two` 함수를 위한 세 개의 테스트를 만들어서 하나만 골라 실행해보겠습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

<span class="caption">Listing 11-11: 여러 이름으로 된 세 가지 테스트</span>

만일 테스트를 어떠한 인자 없이 실행시키면, 전에 본 것과 같이 모든 테스트가 병렬적으로 수행될 것입니다:

```text
running 3 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

#### 단일 테스트 실행하기

단 하나의 테스트만 실행시키기 위해 `cargo test`에 그 테스트 함수의 이름을 넘길 수 있습니다:

```text
$ cargo test one_hundred
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

`one_hundred`라는 이름의 테스트만 실행되었습니다; 다른 두 개의 테스트는 이 이름에 맞지 않습니다.
테스트 출력은 정리 라인의 끝에 `2 filtered out`이라고 표시함으로써 이 커맨드로 지정한 것보다
많은 테스트를 가지고 있음을 우리에게 알려줍니다.

이 방법으로는 여러 테스트의 이름들을 특정할 수는 없고, `cargo test`에 주어진 제일 첫 번째 값만
이용될 것입니다.

#### 여러 개의 테스트를 실행시키기 위한 필터링

우리는 테스트 이름의 일부분을 특정할 수 있고, 해당 값과 일치하는 이름의 테스트가 실행될 것입니다.
예를 들면, 우리의 테스트 이름들 중에서 두 개가 `add`를 포함하므로, `cargo test add`라고
실행하여 이 두 개의 테스트를 실행시킬 수 있습니다:

```text
$ cargo test add
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

이는 `add`가 이름에 포함된 모든 테스트를 실행시켰고 `one_hundred`라는 이름의 테스트를 걸러냈습니다.
또한 테스트가 있는 모듈이 테스트의 이름의 일부가 되어 있으므로, 모듈의 이름으로 필터링하여 그 모듈
내의 모든 테스트를 실행시킬 수 있다는 점도 주목하세요.

### 특별한 요청이 없는 한 몇몇 테스트들 무시하기

이따금씩 몇몇 특정 테스트들은 실행하는데 너무나 시간이 많이 소모될 수 있어서, 여러분은 `cargo test`의
실행 시 이 테스트들을 배제하고 싶어 할지도 모릅니다. 여러분이 실행시키고자 하는 모든 테스트들을 인자로서
열거하는 것 대신, 다음과 같이 시간이 많이 걸리는 테스트들에 `ignore` 속성을 어노테이션하여 이들을
배제시킬 수 있습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

배제시키고 싶은 테스트에 대하여 `#[test]` 다음 줄에 `#[ignore]`를 추가하였습니다. 이제
우리의 테스트들을 실행시키면, `it_works`가 실행되는 것은 보이지만, `expensive-test`는
실행되지 않는 것을 볼 수 있습니다:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 2 tests
test expensive_test ... ignored 
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
```

`expensive_test`는 `ignored` 리스트에 포함되었습니다. 만일 무시된 테스트들만 실행시키고
싶다면, `cargo test -- --ignored`라고 실행함으로써 이를 요청할 수 있습니다.

```text
$ cargo test -- --ignored
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

어떠한 테스트를 실행시킬지를 제어함으로써, 여러분은 `cargo test`의 결과가 빠르게 나오도록 확실히
할 수 있습니다. `ignored` 테스트들의 결과를 확인하기에 타당한 시점에 있고 해당 결과를 기다릴
시간을 가지고 있을 때, 여러분은 대신 `cargo test -- --ignored`를 실행시킬 수 있습니다.