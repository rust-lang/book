## 참조자(References)와 빌림(Borrowing)

앞 절의 마지막에 등장한 튜플을 이용하는 이슈는 `String`을 호출하는 함수 쪽으로
반환함으로써 `calculate_length`를 호출한 이후에도 여전히 `String`을 이용할 수 있도록
하는 것인데, 그 이유는 `String`이 `calculate_length` 안쪽으로 이동되었기 때문입니다.

여기 값의 소유권을 넘기는 대신 개체에 대한 *참조자*(*reference*)를 인자로 사용하는
`calculate_length` 함수를 정의하고 이용하는 방법이 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

첫번째로, 변수 선언부와 함수 반환값에 있던 튜플 코드가 모두 없어진 것에 주목하세요.
두번째로, `calculaate_length` 함수에 `&s1`를 넘기고, 함수의 정의 부분에는
`String`이 아니라 `&String`을 이용했다는 점을 기억하세요.

이 엠퍼센드(&) 기호가 *참조자*이며, 이는 여러분이 어떤 값을 소유권을 넘기지 않고
참조할수 있도록 해줍니다. Figure 4-8은 이에 대한 다이어그램입니다.

<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />

<span class="caption">Figure 4-8: `String s1`을 가리키고 있는 `&String s`</span>

함수 호출 부분을 좀더 자세히 봅시다:

```rust
# fn calculate_length(s: &String) -> usize {
#     s.len()
# }
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

`&s1` 문법은 우리가 `s1`의 값을 *참조*하지만 소유하지는 않는 참조자를 생성하도록
해줍니다. 소유권을 갖고 있지는 않기 때문에, 이 참조자가 가리키는 값은 참조자가 스코프
밖으로 벗어났을 때도 메모리가 반납되지 않을 것입니다.

비슷한 이치로, 함수 시그니처도 `&`를 사용하여 인자 `s`의 타입이 참조자라는 것을 나타내고
있습니다. 설명을 위한 주석을 달아봅시다:

```rust
fn calculate_length(s: &String) -> usize { // s는 String의 참조자입니다
    s.len()
} // 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기
  // 때문에, 아무런 일도 발생하지 않습니다.
```

The scope in which the variable `s` is valid is the same as any function
parameter's scope, but we don’t drop what the reference points to when it goes
out of scope because we don’t have ownership. Functions that have references as
parameters instead of the actual values mean we won’t need to return the values
in order to give back ownership, since we never had ownership.

We call having references as function parameters *borrowing*. As in real life,
if a person owns something, you can borrow it from them. When you’re done, you
have to give it back.

So what happens if we try to modify something we’re borrowing? Try the code in
Listing 4-9. Spoiler alert: it doesn’t work!

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

<span class="caption">Listing 4-9: Attempting to modify a borrowed value</span>

Here’s the error:

```text
error: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^
```

Just as variables are immutable by default, so are references. We’re not
allowed to modify something we have a reference to.

### Mutable References

We can fix the error in the code from Listing 4-9 with just a small tweak:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First, we had to change `s` to be `mut`. Then we had to create a mutable
reference with `&mut s` and accept a mutable reference with `some_string: &mut
String`.

But mutable references have one big restriction: you can only have one mutable
reference to a particular piece of data in a particular scope. This code will
fail:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

Here’s the error:

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> borrow_twice.rs:5:19
  |
4 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
5 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
6 | }
  | - first borrow ends here
```

This restriction allows for mutation but in a very controlled fashion. It’s
something that new Rustaceans struggle with, because most languages let you
mutate whenever you’d like. The benefit of having this restriction is that Rust
can prevent data races at compile time.

A *data race* is a particular type of race condition in which these three
behaviors occur:

1. Two or more pointers access the same data at the same time.
1. At least one of the pointers is being used to write to the data.
1. There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix
when you’re trying to track them down at runtime; Rust prevents this problem
from happening because it won’t even compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for
multiple mutable references, just not *simultaneous* ones:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

A similar rule exists for combining mutable and immutable references. This code
results in an error:

```rust,ignore
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
```

Here’s the error:

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as
immutable
 --> borrow_thrice.rs:6:19
  |
4 |     let r1 = &s; // no problem
  |               - immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |                   ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

Whew! We *also* cannot have a mutable reference while we have an immutable one.
Users of an immutable reference don’t expect the values to suddenly change out
from under them! However, multiple immutable references are okay because no one
who is just reading the data has the ability to affect anyone else’s reading of
the data.

Even though these errors may be frustrating at times, remember that it’s the
Rust compiler pointing out a potential bug early (at compile time rather than
at runtime) and showing you exactly where the problem is instead of you having
to track down why sometimes your data isn’t what you thought it should be.

### Dangling References

In languages with pointers, it’s easy to erroneously create a *dangling
pointer*, a pointer that references a location in memory that may have been
given to someone else, by freeing some memory while preserving a pointer to
that memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling references: if we have a reference to some data, the compiler
will ensure that the data will not go out of scope before the reference to the
data does.

Let’s try to create a dangling reference:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Here’s the error:

```text
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^^^^^^^
  |
  = help: this function's return type contains a borrowed value, but there is no
    value for it to be borrowed from
  = help: consider giving it a 'static lifetime

error: aborting due to previous error
```

This error message refers to a feature we haven’t covered yet: *lifetimes*.
We’ll discuss lifetimes in detail in Chapter 10. But, if you disregard the
parts about lifetimes, the message does contain the key to why this code is a
problem:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

Let’s take a closer look at exactly what’s happening at each stage of our
`dangle` code:

```rust,ignore
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished,
`s` will be deallocated. But we tried to return a reference to it. That means
this reference would be pointing to an invalid `String`! That’s no good. Rust
won’t let us do this.

The solution here is to return the `String` directly:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

This works without any problems. Ownership is moved out, and nothing is
deallocated.

### The Rules of References

Let’s recap what we’ve discussed about references:

1. At any given time, you can have *either* but not both of:
  * One mutable reference.
  * Any number of immutable references.
2. References must always be valid.

Next, we’ll look at a different kind of reference: slices.
