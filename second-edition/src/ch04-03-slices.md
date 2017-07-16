## 슬라이스(Slices)

소유권을 갖지 않는 또다른 데이터 타입은 *슬라이스*입니다. 슬라이스는 여러분이 콜렉션(collection)
전체가 아닌 콜렉션의 연속된 구성요소들의 나열을 참조할 수 있게 합니다.

여기 작은 프로그래밍 문제가 있습니다: 스트링을 입력 받아 그 스트링에서 찾은 첫번째 단어를 반환하는
함수를 작성하세요. 만일 함수가 공백문자를 찾지 못한다면, 이는 전체 스트링이 한 단어라는 의미이고,
이때는 전체 스트링이 반환되어야 합니다.

이 함수의 시그니처에 대해 생각해봅시다:

```rust,ignore
fn first_word(s: &String) -> ?
```

이 함수 `first_word`는 `&String`을 파라미터로 갖습니다. 우리는 소유권을 원하지 않으므로, 이는
괜찮습니다. 하지만 뭘 반환해야할까요? 우리는 스트링의 *일부*에 대해 표현할 방법이 없습니다. 하지만
단어의 끝부분의 인덱스를 반환할 수는 있겠습니다. Listing 4-10의 코드를 시도해봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

<span class="caption">Listing 4-10: `String` 파라미터의 바이트 인덱스 값을 반환하는
`first_word` 함수</span>

이 코드를 쪼개서 봅시다. 입력된 `String`를 원소 단위 보면서 그 값이 공백인지 확인할 필요가 있기
때문에, `String`은 `as_bytes` 메소드를 이용하여 바이트 배열로 변환됩니다:

```rust,ignore
let bytes = s.as_bytes();
```

다음으로, `iter` 메소드를 이용하여 바이트 배열의 반복자(iterator)를 생성합니다:

```rust,ignore
for (i, &item) in bytes.iter().enumerate() {
```

반복자에 대한 것은 13장에서 더 자세히 다루겠습니다. 지금은 `iter`가 콜렉션의 각 원소를 반환하는 함수이며,
`enumerate`은 `iter`의 결과값을 직접 반환하는 대신 이를 감싸서 튜플의 일부로 만들어 반환한다는 정도만
알아두세요. 반환된 튜플의 첫번째 원소는 인덱스이며, 두번째 원소는 원소에 대한 참조값입니다. 이는 우리 스스로
인덱스를 계산하는 것보다 살짝 더 편리합니다.

`enumerate` 메소드가 튜플을 반환하기 때문에, 우리는 러스트의 다른 모든 부분에서 그러하듯이 이 튜플을
해체하기 위해 패턴을 이용할 수 있습니다. 따라서 `for` 루프 내에서, `i`는 튜플 내의 인덱스에 대응하고
`&item`은 튜플 내의 한 바이트에 대응하는 패턴을 특정하는 것입니다. `.iter().enumerate()`의
원소에 대한 참조자를 갖는 것이므로, `&`을 패턴 내에 사용했습니다.

우리는 바이트 리터럴 문법을 이용하여 공백 문자를 나타내는 바이트를 찾습니다. 공백 문자를 찾았다면,
이 위치를 반환합니다. 그렇지 않으면 `s.len()`을 통해 스트링의 길이값을 반환합니다:

```rust,ignore
    if item == b' ' {
        return i;
    }
}
s.len()
```

이제 우리에게 스트링의 첫번째 단어의 끝부분의 인덱스를 찾아낼 방법이 생겼습니다. 소유권이 포함된
`usize`를 반환하고 있지만, 이는 `&string`의 내용물 내에서만 의미가 있습니다. 바꿔 말하면,
이것이 `String`로부터 분리되어 있는 숫자이기 때문에, 이것이 나중에도 여전히 유효한지를 보장할
길이 없습니다. Listing 4-10의 `first_word` 함수를 사용하는 Listing 4-11의 프로그램을
보시죠:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn first_word(s: &String) -> usize {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return i;
#         }
#     }
#
#     s.len()
# }
#
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word는 5를 갖게 될 것입니다.

    s.clear(); // 이 코드는 String을 비워서 ""로 만들게 됩니다.

    // word는 여기서 여전히 5를 갖고 있지만, 5라는 값을 의미있게 쓸 수 있는 스트링은 이제 없습니다.
    // word는 이제 완전 유효하지 않습니다!
}
```

<span class="caption">Listing 4-11: `first_word` 함수를 호출하여 결과를 저장한 뒤
`String`의 내용물을 바꾸기</span>

이 프로그램은 아무런 오류 없이 컴파일되고, `s.clear()`을 호출한 뒤 `word`를 사용한다 해도
역시 컴파일될 것입니다. `word`는 `s`의 상태와 전혀 연결되어 있지 않으므로, `word`는 여전히 값
`5`를 담고 있습니다. 우리는 첫번째 단어를 추출하고자 하기 위해 `s`와 값 `5`를 사용할 수 있지만,
`word에 `5`를 저장한 뒤 `s`의 내용물이 변경되었기 때문에 이러한 사용은 버그가 될 것입니다.

Having to worry about the index in `word` getting out of sync with the data in
`s` is tedious and error prone! Managing these indices is even more brittle if
we write a `second_word` function. Its signature would have to look like this:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Now we’re tracking a start *and* an ending index, and we have even more values
that were calculated from data in a particular state but aren’t tied to that
state at all. We now have three unrelated variables floating around that need
to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

### String Slices

A *string slice* is a reference to part of a `String`, and looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

This is similar to taking a reference to the whole `String` but with the extra
`[0..5]` bit. Rather than a reference to the entire `String`, it’s a reference
to a portion of the `String`. The `start..end` syntax is a range that begins at
`start` and continues up to, but not including, `end`.

We can create slices using a range within brackets by specifying
`[starting_index..ending_index]`, where `starting_index` is the first position
included in the slice and `ending_index` is one more than the last position
included in the slice. Internally, the slice data structure stores the starting
position and the length of the slice, which corresponds to `ending_index` minus
`starting_index`. So in the case of `let world = &s[6..11];`, `world` would be
a slice that contains a pointer to the 6th byte of `s` and a length value of 5.

Figure 4-12 shows this in a diagram.

<img alt="world containing a pointer to the 6th byte of String s and a length 5" src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-12: String slice referring to part of a
`String`</span>

With Rust’s `..` range syntax, if you want to start at the first index (zero),
you can drop the value before the two periods. In other words, these are equal:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

By the same token, if your slice includes the last byte of the `String`, you
can drop the trailing number. That means these are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

You can also drop both values to take a slice of the entire string. So these
are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

With all this information in mind, let’s rewrite `first_word` to return a
slice. The type that signifies “string slice” is written as `&str`:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

We get the index for the end of the word in the same way as we did in Listing
4-10, by looking for the first occurrence of a space. When we find a space, we
return a string slice using the start of the string and the index of the space
as the starting and ending indices.

Now when we call `first_word`, we get back a single value that is tied to the
underlying data. The value is made up of a reference to the starting point of
the slice and the number of elements in the slice.

Returning a slice would also work for a `second_word` function:

```rust,ignore
fn second_word(s: &String) -> &str {
```

We now have a straightforward API that’s much harder to mess up, since the
compiler will ensure the references into the `String` remain valid. Remember
the bug in the program in Listing 4-11, when we got the index to the end of the
first word but then cleared the string so our index was invalid? That code was
logically incorrect but didn’t show any immediate errors. The problems would
show up later if we kept trying to use the first word index with an emptied
string. Slices make this bug impossible and let us know we have a problem with
our code much sooner. Using the slice version of `first_word` will throw a
compile time error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

Here’s the compiler error:

```text
17:6 error: cannot borrow `s` as mutable because it is also borrowed as
            immutable [E0502]
    s.clear(); // Error!
    ^
15:29 note: previous borrow of `s` occurs here; the immutable borrow prevents
            subsequent moves or mutable borrows of `s` until the borrow ends
    let word = first_word(&s);
                           ^
18:2 note: previous borrow ends here
fn main() {

}
^
```

Recall from the borrowing rules that if we have an immutable reference to
something, we cannot also take a mutable reference. Because `clear` needs to
truncate the `String`, it tries to take a mutable reference, which fails. Not
only has Rust made our API easier to use, but it has also eliminated an entire
class of errors at compile time!

#### String Literals Are Slices

Recall that we talked about string literals being stored inside the binary. Now
that we know about slices, we can properly understand string literals:

```rust
let s = "Hello, world!";
```

The type of `s` here is `&str`: it’s a slice pointing to that specific point of
the binary. This is also why string literals are immutable; `&str` is an
immutable reference.

#### String Slices as Parameters

Knowing that you can take slices of literals and `String`s leads us to one more
improvement on `first_word`, and that’s its signature:

```rust,ignore
fn first_word(s: &String) -> &str {
```

A more experienced Rustacean would write the following line instead because it
allows us to use the same function on both `String`s and `&str`s:

```rust,ignore
fn first_word(s: &str) -> &str {
```

If we have a string slice, we can pass that directly. If we have a `String`, we
can pass a slice of the entire `String`. Defining a function to take a string
slice instead of a reference to a String makes our API more general and useful
without losing any functionality:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn first_word(s: &str) -> &str {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return &s[0..i];
#         }
#     }
#
#     &s[..]
# }
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### Other Slices

String slices, as you might imagine, are specific to strings. But there’s a
more general slice type, too. Consider this array:

```rust
let a = [1, 2, 3, 4, 5];
```

Just like we might want to refer to a part of a string, we might want to refer
to part of an array and would do so like this:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

This slice has the type `&[i32]`. It works the same way as string slices do, by
storing a reference to the first element and a length. You’ll use this kind of
slice for all sorts of other collections. We’ll discuss these collections in
detail when we talk about vectors in Chapter 8.

## Summary

The concepts of ownership, borrowing, and slices are what ensure memory safety
in Rust programs at compile time. The Rust language gives you control over your
memory usage like other systems programming languages, but having the owner of
data automatically clean up that data when the owner goes out of scope means
you don’t have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we’ll talk about
these concepts further throughout the rest of the book. Let’s move on to the
next chapter and look at grouping pieces of data together in a `struct`.
