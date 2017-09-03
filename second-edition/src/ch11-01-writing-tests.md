## 테스트를 작성하는 방법

테스트는 테스트 아닌 코드가 프로그램 내에서 기대했던 대로 기능을 하는지 검증하는 러스트
함수입니다. 테스트 함수의 본체는 통상적으로 셋업, 우리가 테스트하고 싶은 코드의 실행,
그런 다음 그 결과가 우리 예상대로인지 확고히 하는(assert) 단계를 담고 있습니다. 테스트
작성을 위해 러스트가 특별히 제공하는 기능들을 살펴봅시다: `test` 속성, 몇 가지 매크로,
그리고 `should_panic` 속성들을 말이죠.

### 테스트 함수의 해부

가장 단순하게 말하면, 러스트 내의 테스트란 `test` 속성(attribute)이 주석으로 달려진
(annotated) 함수입니다. 속성은 러스트 코드 조각에 대한 메타데이터입니다: 5장에서
우리가 구조체와 함께 사용했던 `derive` 속성이 한가지 예입니다. 함수를 테스트 함수로
만들기 위해서는, `fn` 전 라인에 `#[test]`를 추가합니다. `cargo test` 커맨드를
사용하여 테스트를 실행시키면, 러스트는 `test` 속성이 달려있는 함수들을 실행하고
각 테스트 함수가 성공 혹은 실패했는지를 보고하는 테스트 실행용 바이너리를 빌드할
것입니다.

<!-- is it annotated with `test` by the user, or only automatically? I think
it's the latter, and has edited with a more active tone to make that clear, but
please change if I'm wrong -->
<!-- What do you mean by "only automatically"? The reader should be typing in
`#[test] on their own when they add new test functions; there's nothing special
about that text. I'm not sure what part of this chapter implied "only
automatically", can you point out where that's happening if we haven't taken
care of it? /Carol -->

7장에서 여러분이 카고를 통해 새로운 라이브러리 프로젝트를 만들었을 때, 테스트 함수를
갖고 있는 테스트 모듈이 자동으로 생성되는 것을 보았습니다. 이는 우리의 테스트를 작성하기
시작하도록 돕기 위한 것인데, 이렇게 하면 우리가 새로운 프로젝트를 시작할 때마다
매번 테스트 함수를 위한 추가적인 구조 및 문법을 찾아보지 않아도 되기 때문입니다.
하지만, 우리는 원하는만큼 추가적인 테스트 함수들과 테스트 모듈들을 추가할 수 있습니다!

우리는 실제 코드를 테스팅하지는 않으면서 자동으로 만들어진 템플릿 테스트를 가지고
실험하는 식으로 테스트가 어떻게 동작하는지를 몇가지 관점에서 탐구할 것입니다.
그리고나서 우리가 작성한 몇몇 코드를 호출하고 동작이 정확한지를 확고히하는
실제의 테스트를 작성해 볼 것입니다.

`adder`라고 하는 새로운 라이브러리 프로젝트를 만듭시다:

```text
$ cargo new adder
     Created library `adder` project
$ cd adder
```

여러분의 adder 라이브러리 내에 있는 `src/lib.rs` 파일의 내용물은 아래와 같아야
합니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

<span class="caption">Listing 11-1: `cargo new`를 이용하여 자동으로 생성된
테스트 모듈과 함수 </span>

지금은 제일 위의 두 줄은 무시하고 함수가 어떻게 작동하는지 알아보는데 집중합시다.
`fn` 라인 전의 `#[test]` 어노테이션을 주목하세요: 이 속성이 바로 이것이 테스트 함수임을
나타내므로, 테스트 실행기는 이 함수를 테스트로 다루어야 한다는 것을 알게 됩니다.
또한 우리는 `tests` 모듈 내에 일반적인 시나리오를 셋업하거나 일반적인 연산을 수행하는
것을 돕기 위한 테스트 아닌 함수를 넣을 수 있으므로, 어떤 함수가 테스트 함수인지
`#[test]`를 가지고 표시할 필요가 있습니다.

이 함수는 현재 본체가 없는데, 이는 테스트에 실패할 코드가 없다는 의미입니다;
빈 테스트는 통과하는 테스트입니다! 이걸 실행하여 이 테스트가 통과하는지 봅시다.

`cargo test` 커맨드는 Listing 11-2에서 보는 바와 같이 우리 프로젝트에 있는
모든 테스트를 실행합니다:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

<span class="caption">Listing 11-2: 자동으로 생성된 테스트를 실행한 결과 </span>

카고는 우리의 테스트를 컴파일하고 실행했습니다. `Compiling`, `Finished`, 그리고
`Running` 라인 이후에, `running 1 test` 라인을 보게 됩니다. 그 다음 라인에는
생성된 테스트 함수의 이름인 `it_works`가 나타나고, 테스트의 실행 결과 `ok`가
나타납니다. 그리고나서 테스트 실행의 전체 요약을 보게 됩니다:
`test result: ok.`는 모든 테스트가 통과했다는 뜻입니다. `1 passed; 0 failed`는
통과하거나 실패한 테스트의 개수를 추가적으로 보여줍니다.

우리가 무시하라고 표시한 테스트가 없으므로, 요약문에 `0 ignored`라고 표시됩니다.
테스트를 실행하는 다른 방법에 대하여 다루는 다음 절에서 테스트를 무시하는 것에 대해
다룰 것입니다. `0 measured` 통계는 성능을 측정하는 벤치마크 테스트를 위한 것입니다.
벤치마크 테스트는 이 글이 쓰여진 시점에서는 오직 나이틀리(nightly) 러스트에서만 사용
가능합니다. 나이틀리 러스트에 대한 정보는 부록 D에 나와있습니다.

`Doc-tests adder`로 시작하는 테스트 출력의 다음 부분은 문서 테스트의 결과를 보여주기
위한 것입니다. 아직 어떠한 문서 테스트도 없긴 하지만, 러스트는 우리의 API 문서 내에
나타난 어떠한 코드 예제라도 컴파일 할 수 있습니다. 이 기능은 우리의 문서와 코드가
동기화를 유지하도록 돕습니다! 우리는 14장의 "문서 주석"절 에서 문서 테스트를 작성하는
방법에 대해 이야기할 것입니다. 지금은 `Doc-tests` 출력을 무시할 것입니다.

<!-- I might suggest changing the name of the function, could be misconstrued
as part of the test output! -->
<!-- `it_works` is always the name that `cargo new` generates for the first
test function, though. We wanted to show the reader what happens when you run
the tests immediately after generating a new project; they pass without you
needing to change anything. I've added a bit to walk through changing the
function name and seeing how the output changes; I hope that's sufficient.
/Carol -->

우리의 테스트의 이름을 변경하고 테스트 출력이 어떻게 변하는지를 살펴봅시다.
다음과 같이 `it_works` 함수의 이름을 `exploration`으로 변경하세요:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
    }
}
```

그리고 `cargo test`를 다시 실행시킵니다. 이제 출력 부분에서 `it_works` 대신
`exploration`을 볼 수 있을 것입니다:

```text
running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

다른 테스트를 추가해봅시다. 하지만 이번에는 실패하는 테스트를 만들 것입니다! 테스트
함수 내의 무언가가 패닉을 일으키면 테스트는 실패합니다. 9장에서 패닉을 유발하는
가장 단순한 방법에 대해 이야기했었습니다: 바로 `panic!` 매크로를 호출하는 것이죠!
새로운 테스트를 입력하여 여러분의 `src/lib.rs`가 Listing 11-3과 같은 모양이
되게 해보세요:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

<span class="caption">Listing 11-3: 두번째 테스트 추가; `panic!` 매크로를
호출하기 떄문에 실패할 테스트 </span>

그리고 `cargo test`를 이용하여 다시 한번 테스트를 실행시키세요. 결과 출력은
Listing 11-4와 같이 나올 것인데, 이는 `exploration` 테스트는 통과하고 `another`는
실패했음을 보여줍니다:

```text
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
	thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured

error: test failed
```

<span class="caption">Listing 11-4: 한 테스트는 통과하고 다른 한 테스트는
실패할 때의 테스트 결과 </span>

`test tests::another` 라인은 `ok` 대신 `FAILED`이라 말해줍니다. 우리는 개별 결과
부분과 요약 부분 사이에 새로운 두개의 섹션을 보게 됩니다: 첫번째 섹션은 테스트 실패에
대한 구체적인 이유를 표시합니다. 이 경우, `another`는 `panicked at 'Make this test fail'`
때문에 실패했는데, 이는 *src/lib.rs*의 9번 라인에서 발생했습니다. 다음 섹션은
실패한 모든 테스트의 이름만 목록화한 것인데, 이는 테스트들이 많이 있고 구체적인 테스트
실패 출력이 많을 때 유용합니다. 실패하는 테스트의 이름은 이를 더 쉽게 디버깅하기 위해서
해당 테스트만을 실행시키는데 사용될 수 있습니다; 다음 절에서 테스트를 실행시키는 방법에
대한 더 많은 내용을 이야기할 것입니다.

마지막으로, 요약 라인입니다: 전체적으로, 우리의 테스트 결과는 `FAILED`입니다.
우리는 1개의 테스트에 통과했고 1개의 테스트에 실패했습니다.

이제 서로 다른 시나리오에서 테스트 결과가 어떻게 보이는지를 알았으니,
`panic!` 외에 테스트 내에서 유용하게 쓰일 수 있는 몇가지 매크로를 봅시다.

### `assert!` 매크로를 이용하여 결과 확인하기

The `assert!` macro, provided by the standard library, is useful when you want
to ensure that some condition in a test evaluates to `true`. We give the
`assert!` macro an argument that evaluates to a boolean. If the value is `true`,
`assert!` does nothing and the test passes. If the value is `false`, `assert!`
calls the `panic!` macro, which causes the test to fail. This is one macro that
helps us check that our code is functioning in the way we intend.

<!-- what kind of thing can be passed as an argument? Presumably when we use it
for real we won't pass it `true` or `false` as an argument, but some condition
that will evaluate to true or false? In which case, should below be phrased "If
the argument evaluates to true" and an explanation of that? Or maybe even a
working example would be better, this could be misleading -->
<!-- We were trying to really break it down, to show just how the `assert!`
macro works and what it looks like for it to pass or fail, before we got into
calling actual code. We've changed this section to move a bit faster and just
write actual tests instead. /Carol -->

Remember all the way back in Chapter 5, Listing 5-9, where we had a `Rectangle`
struct and a `can_hold` method, repeated here in Listing 11-5. Let's put this
code in *src/lib.rs* instead of *src/main.rs* and write some tests for it using
the `assert!` macro.

<!-- Listing 5-9 wasn't marked as such; I'll fix it the next time I get Chapter
5 for editing. /Carol -->

<span class="filename">Filename: src/lib.rs</span>

```rust
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
```

<span class="caption">Listing 11-5: The `Rectangle` struct and its `can_hold`
method from Chapter 5 </span>

The `can_hold` method returns a boolean, which means it's a perfect use case
for the `assert!` macro. In Listing 11-6, let's write a test that exercises the
`can_hold` method by creating a `Rectangle` instance that has a length of 8 and
a width of 7, and asserting that it can hold another `Rectangle` instance that
has a length of 5 and a width of 1:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
}
```

<span class="caption">Listing 11-6: A test for `can_hold` that checks that a
larger rectangle indeed holds a smaller rectangle </span>

Note that we've added a new line inside the `tests` module: `use super::*;`.
The `tests` module is a regular module that follows the usual visibility rules
we covered in Chapter 7. Because we're in an inner module, we need to bring the
code under test in the outer module into the scope of the inner module. We've
chosen to use a glob here so that anything we define in the outer module is
available to this `tests` module.

We've named our test `larger_can_hold_smaller`, and we've created the two
`Rectangle` instances that we need. Then we called the `assert!` macro and
passed it the result of calling `larger.can_hold(&smaller)`. This expression is
supposed to return `true`, so our test should pass. Let's find out!

```text
running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

It does pass! Let's add another test, this time asserting that a smaller
rectangle cannot hold a larger rectangle:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}
```

Because the correct result of the `can_hold` function in this case is `false`,
we need to negate that result before we pass it to the `assert!` macro. This
way, our test will pass if `can_hold` returns `false`:

```text
running 2 tests
test tests::smaller_can_not_hold_larger ... ok
test tests::larger_can_hold_smaller ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Two passing tests! Now let's see what happens to our test results if we
introduce a bug in our code. Let's change the implementation of the `can_hold`
method to have a less-than sign when it compares the lengths where it's
supposed to have a greater-than sign:

```rust
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length < other.length && self.width > other.width
    }
}
```

Running the tests now produces:

```text
running 2 tests
test tests::smaller_can_not_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED

failures:

---- tests::larger_can_hold_smaller stdout ----
	thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed:
    larger.can_hold(&smaller)', src/lib.rs:22
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured
```

Our tests caught the bug! Since `larger.length` is 8 and `smaller.length` is 5,
the comparison of the lengths in `can_hold` now returns `false` since 8 is not
less than 5.

### Testing Equality with the `assert_eq!` and `assert_ne!` Macros

A common way to test functionality is to take the result of the code under test
and the value we expect the code to return and check that they're equal. We
could do this using the `assert!` macro and passing it an expression using the
`==` operator. However, this is such a common test that the standard library
provides a pair of macros to perform this test more conveniently: `assert_eq!`
and `assert_ne!`. These macros compare two arguments for equality or
inequality, respectively. They'll also print out the two values if the
assertion fails, so that it's easier to see *why* the test failed, while the
`assert!` macro only tells us that it got a `false` value for the `==`
expression, not the values that lead to the `false` value.

In Listing 11-7, let's write a function named `add_two` that adds two to its
parameter and returns the result. Then let's test this function using the
`assert_eq!` macro:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

<span class="caption">Listing 11-7: Testing the function `add_two` using the
`assert_eq!` macro </span>

Let's check that it passes!

```text
running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

The first argument we gave to the `assert_eq!` macro, 4, is equal to the result
of calling `add_two(2)`. We see a line for this test that says `test
tests::it_adds_two ... ok`, and the `ok` text indicates that our test passed!

Let's introduce a bug into our code to see what it looks like when a test that
uses `assert_eq!` fails. Change the implementation of the `add_two` function to
instead add 3:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

And run the tests again:

```text
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
	thread 'tests::it_adds_two' panicked at 'assertion failed: `(left ==
    right)` (left: `4`, right: `5`)', src/lib.rs:11
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

Our test caught the bug! The `it_adds_two` test failed with the message ``
assertion failed: `(left == right)` (left: `4`, right: `5`) ``. This message is
useful and helps us get started debugging: it says the `left` argument to
`assert_eq!` was 4, but the `right` argument, where we had `add_two(2)`, was 5.

Note that in some languages and test frameworks, the parameters to the
functions that assert two values are equal are called `expected` and `actual`
and the order in which we specify the arguments matters. However, in Rust,
they're called `left` and `right` instead, and the order in which we specify
the value we expect and the value that the code under test produces doesn't
matter. We could have written the assertion in this test as
`assert_eq!(add_two(2), 4)`, which would result in a failure message that says
`` assertion failed: `(left == right)` (left: `5`, right: `4`) ``.

The `assert_ne!` macro will pass if the two values we give to it are not equal
and fail if they are equal. This macro is most useful for cases when we're not
sure exactly what a value *will* be, but we know what the value definitely
*won't* be, if our code is functioning as we intend. For example, if we have a
function that is guaranteed to change its input in some way, but the way in
which the input is changed depends on the day of the week that we run our
tests, the best thing to assert might be that the output of the function is not
equal to the input.

Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators
`==` and `!=`, respectively. When the assertions fail, these macros print their
arguments using debug formatting, which means the values being compared must
implement the `PartialEq` and `Debug` traits. All of the primitive types and
most of the standard library types implement these traits. For structs and
enums that you define, you'll need to implement `PartialEq` in order to be able
to assert that values of those types are equal or not equal. You'll need to
implement `Debug` in order to be able to print out the values in the case that
the assertion fails. Because both of these traits are derivable traits, as we
mentioned in Chapter 5, this is usually as straightforward as adding the
`#[derive(PartialEq, Debug)]` annotation to your struct or enum definition. See
Appendix C for more details about these and other derivable traits.

### Custom Failure Messages

We can also add a custom message to be printed with the failure message as
optional arguments to `assert!`, `assert_eq!`, and `assert_ne!`. Any arguments
specified after the one required argument to `assert!` or the two required
arguments to `assert_eq!` and `assert_ne!` are passed along to the `format!`
macro that we talked about in Chapter 8, so you can pass a format string that
contains `{}` placeholders and values to go in the placeholders. Custom
messages are useful in order to document what an assertion means, so that when
the test fails, we have a better idea of what the problem is with the code.

For example, let's say we have a function that greets people by name, and we
want to test that the name we pass into the function appears in the output:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

The requirements for this program haven't been agreed upon yet, and we're
pretty sure the `Hello` text at the beginning of the greeting will change. We
decided we don't want to have to update the test for the name when that
happens, so instead of checking for exact equality to the value returned from
the `greeting` function, we're just going to assert that the output contains
the text of the input parameter.

Let's introduce a bug into this code to see what this test failure looks like,
by changing `greeting` to not include `name`:

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```

Running this test produces:

```text
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
	thread 'tests::greeting_contains_name' panicked at 'assertion failed:
    result.contains("Carol")', src/lib.rs:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::greeting_contains_name
```

This just tells us that the assertion failed and which line the assertion is
on. A more useful failure message in this case would print the value we did get
from the `greeting` function. Let's change the test function to have a custom
failure message made from a format string with a placeholder filled in with the
actual value we got from the `greeting` function:

```rust,ignore
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}
```

Now if we run the test again, we'll get a much more informative error message:

```text
---- tests::greeting_contains_name stdout ----
	thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain
    name, value was `Hello`', src/lib.rs:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

We can see the value we actually got in the test output, which would help us
debug what happened instead of what we were expecting to happen.

### Checking for Panics with `should_panic`

In addition to checking that our code returns the correct values we expect,
it's also important to check that our code handles error conditions as we
expect. For example, consider the `Guess` type that we created in Chapter 9 in
Listing 9-8. Other code that uses `Guess` is depending on the guarantee that
`Guess` instances will only contain values between 1 and 100. We can write a
test that ensures that attempting to create a `Guess` instance with a value
outside that range panics.

We can do this by adding another attribute, `should_panic`, to our test
function. This attribute makes a test pass if the code inside the function
panics, and the test will fail if the code inside the function does not panic.

Listing 11-8 shows how we'd write a test that checks the error conditions of
`Guess::new` happen when we expect:

<span class="filename">Filename: src/lib.rs</span>

```rust
struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

<span class="caption">Listing 11-8: Testing that a condition will cause a
`panic!` </span>

The `#[should_panic]` attribute goes after the `#[test]` attribute and before
the test function it applies to. Let's see what it looks like when this test
passes:

```text
running 1 test
test tests::greater_than_100 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Looks good! Now let's introduce a bug in our code, by removing the condition
that the `new` function will panic if the value is greater than 100:

```rust
# struct Guess {
#     value: u32,
# }
#
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1  {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}
```

If we run the test from Listing 11-8, it will fail:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

We don't get a very helpful message in this case, but once we look at the test
function, we can see that it's annotated with `#[should_panic]`. The failure we
got means that the code in the function, `Guess::new(200)`, did not cause a
panic.

`should_panic` tests can be imprecise, however, because they only tell us that
the code has caused some panic. A `should_panic` test would pass even if the
test panics for a different reason than the one we were expecting to happen. To
make `should_panic` tests more precise, we can add an optional `expected`
parameter to the `should_panic` attribute. The test harness will make sure that
the failure message contains the provided text. For example, consider the
modified code for `Guess` in Listing 11-9 where the `new` function panics with
different messages depending on whether the value was too small or too large:

<span class="filename">Filename: src/lib.rs</span>

```rust
struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

<span class="caption">Listing 11-9: Testing that a condition will cause a
`panic!` with a particular panic message </span>

This test will pass, because the value we put in the `expected` parameter of
the `should_panic` attribute is a substring of the message that the
`Guess::new` function panics with. We could have specified the whole panic
message that we expect, which in this case would be `Guess value must be less
than or equal to 100, got 200.` It depends on how much of the panic message is
unique or dynamic and how precise you want your test to be. In this case, a
substring of the panic message is enough to ensure that the code in the
function that gets run is the `else if value > 100` case.

To see what happens when a `should_panic` test with an `expected` message
fails, let's again introduce a bug into our code by swapping the bodies of the
`if value < 1` and the `else if value > 100` blocks:

```rust,ignore
if value < 1 {
    panic!("Guess value must be less than or equal to 100, got {}.", value);
} else if value > 100 {
    panic!("Guess value must be greater than or equal to 1, got {}.", value);
}
```

This time when we run the `should_panic` test, it will fail:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
	thread 'tests::greater_than_100' panicked at 'Guess value must be greater
    than or equal to 1, got 200.', src/lib.rs:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
note: Panic did not include expected string 'Guess value must be less than or
equal to 100'

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

The failure message indicates that this test did indeed panic as we expected,
but the panic message `did not include expected string 'Guess value must be
less than or equal to 100'`. We can see the panic message that we did get,
which in this case was `Guess value must be greater than or equal to 1, got
200.` We could then start figuring out where our bug was!

Now that we've gone over ways to write tests, let's look at what is happening
when we run our tests and talk about the different options we can use with
`cargo test`.
