## 고급 타입

러스트의 타입 시스템은 이 책에서 언급은 했지만 아직 논의하지는 않았던 몇가지
기능들을 가지고 있습니다. 우리는 대개 왜 뉴타입이 타입으로서 유용한지를 시험함으로서
뉴타입에 대해 논하는 것으로 시작할 것입니다. 그 다음 뉴타입과 비슷안 기능이지만
약간 다른 의미를 가지고 있는 타입 별칭(type alias)으로 넘어가겠습니다. 또한 `!`
타입과 동적인 크기의 (dynamically sized) 타입에 대해 논할 것입니다.

> 노트: 다음 절은 여러분이 이전 절 “외부 타입에 대해 외부 트레잇을 구현하기
> 위한 뉴타입 패턴”을 읽었음을 가정합니다.

### 타입 안전성과 추상화를 위한 뉴타입 패턴 사용하기

뉴타입 패턴은 우리가 지금까지 논했던 것 이상으로 다른 작업에 대해서도 유용한데,
여기에는 어떤 값이 혼동되지 않도록 정적으로 강제하는 것과 어떤 값의 단위
표시로서의 기능을 포함합니다. 여러분은 Listing 19-23에서 단위를 나타내기
위해 뉴타입을 사용하는 예제를 봤습니다: `u32` 값을 뉴타입으로 감싼
`Millimeters`와 `Meters` 구조체를 상기하세요. 만일 우리가 `Millimeters`
타입의 파라미터를 가지고 함수를 작성했다면, 의도치않게 그 함수에 `Meters`
타입의 값이나 그냥 `u32` 값을 넣어서 호출 시도를 하는 프로그램의 컴파일을
하지 못하게 됩니다.

뉴타입 패턴의 또다른 사용례는 어떤 타입의 몇몇 자세한 구현 사항을 추상화
하는 것입니다: 예를 들어 우리가 가능한 기능을 제약하기 위해 뉴타입을 직접
사용했다면 뉴타입은 내부의 비공개 타입이 가진 API와 다른 공개 API를 노출할
수 있습니다.

뉴타입은 또한 내부 구현사항을 숨길 수 있습니다. 예을 들어, 우리는 사람의
ID와 그의 이름을 저장하는 `HashMap<i32, String>`을 감싸는 `People`
타입을 제공할 수 있습니다. `People`을 사용하는 코드는 오직 우리가 제공하는
공개 API만을 통해 상호작용할 것이며, 여기에는 `People` 컬렉션에 이름
문자열을 추가하는 메소드 같은게 있겠지요; 이 코드에서는 우리가 내부적으로
이름에 대해 `i32` ID를 할당한다는 점을 알 필요가 없을 것입니다. 뉴타입
패턴은 캡술화를 하여 자세한 구현 사항을 숨기기 위한 가벼운 방식으로, 캡술화에
대한 것은 17장의 “자세한 구현사항을 숨기는 캡슐화” 절에서 다루었습니다.

### 타입 별칭은 타입의 동의어를 만듭니다

뉴타입 패턴에 덧붙여서, 러스트는 존재하는 타입에게 다른 이름을 부여하기 위한 *타입 별칭
(type alias)* 선언 기능을 제공합니다. 이를 위해서는 `type` 키워드를 사용합니다.
예를 들어, 우리는 아래와 같이 `i32`에 대한 별칭 `Kilometers`를 생성할 수 있습니다:

```rust
type Kilometers = i32;
```

이제 별칭인 `Kilometers`는 `i32`와 *동의어*입니다; 우리가 Listing 19-23에서
만들었던 `Millimeters` 및 `Meters`와는 달리, `Kilometers`는 분리된, 새로운
타입이 아닙니다. `Kilometers` 타입의 값은 `i32` 타입의 갑과 동일한 것으로
취급될 것입니다:

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

`Kilometers`와 `i32`가 동일한 타입이기 때문에, 우리는 두 타입의 값을
더할 수 있고 `i32` 파라미터를 갖는 함수에게 `Kilometers` 값을 넘길 수
있습니다. 그러나, 이 방법을 사용하면 우리는 앞서 논의했던 뉴타입 패턴이
제공하는 타입 검사의 이점을 얻지 못합니다.

타입 동의어의 주요 사용 사례는 반복 줄이기 입니다. 예를 들어, 우리는
아래와 같이 길다란 타입을 가질지도 모릅니다:

```rust,ignore
Box<Fn() + Send + 'static>
```

이러한 길다란 타입을 함수 시그니처 혹은 타입 명시로 코드의 모든 곳에
작성하는 것은 성가시고 에러를 내기도 쉽습니다. Listing 19-32와 같은
코드로 가득한 프로젝트가 있다고 상상해보세요.

```rust
let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));

fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<Fn() + Send + 'static> {
    // --snip--
#     Box::new(|| ())
}
```

<span class="caption">Listing 19-32: 수많은 곳에 긴 타입을 사용하기</span>

타입 별칭은 반복을 줄임으로서 이 코드의 관리를 더 잘되게끔 만들어줍니다.
Listing 19-33에서 우리는 이 장황한 타입에 대해 `Thunk`라는 이름의 별칭을
도입해서 이 타입이 사용되는 모든 부분을 짧은 별칭인 `Thunk`로 대체할 수 있습니다.

```rust
type Thunk = Box<Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
#     Box::new(|| ())
}
```

<span class="caption">Listing 19-33: 반복을 줄이기 위해 타입 별칭 `Thunk`을
도입하기</span>

이 코드가 훨씬 읽고 쓰기 쉽습니다! 타입 별칭을 위한 의미잆는 이름을 고르는
것은 또한 여러분의 의도를 전달하는 데에 도움을 줄 수 있습니다 (*thunk*는
이후에 실행될 코드를 위한 단어로, 저징되는 클로저를 위한 적절한
이름입니다.)

타입 별칭은 또한 `Result<T, E>`타입의 반복을 줄이기 위해 흔하게 사용됩니다.
표준 라이브러리의 `std::io` 모듈을 고려해 보세요. I/O 연산들은 작동에
실패하는 상황을 다루기 위해서 자주 `Result<T, E>`을 반환합니다.
이 라이브러리는 모든 가능한 I/O 에러를 표현하는 `std::io::Error`
구조체를 가지고 있습니다. `std::io` 내의 많은 함수들이 `E`가
`std::io::Error`인 `Result<T, E>`을 반환합니다. `Write` 트레잇의
아래 함수들 같이 말이죠:

```rust
use std::io::Error;
use std::fmt;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

`Result<..., Error>`이 너무 많이 반복됩니다. 그렇기 때문에, `std::io`는
이 타입의 별칭 선언을 갖고 있습니다:

```rust,ignore
type Result<T> = Result<T, std::io::Error>;
```

이 선언이 `std::io` 모듈 내에 있으므로, 우리는 완전 정규화된 별칭
`std::io::Result<T>`을 사용할 수 있습니다; 이는 `E`가 `std::io::Error`로
채워진 `Result<T, E>`입니다. `Write` 트레잇 함수 시그니처는 결국 아래와 같이
보이게 됩니다:

```rust,ignore
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
}
```

이 타입 별칭은 두 가지 방식으로 도움을 줍니다; 코드를 작성하기 더 편하게 해주고
*그러면서도* 모든 `std::io`에 걸쳐 일관된 인터페이스를 제공합니다. 이것이 별칭이기
때문에, 이것은 그저 또다른 `Result<T, E>`일 뿐이고, 이는 우리가 `Result<T, E>`을
가지고 쓸 수 있는 어떠한 메소드는 물론, `?`같은 특별 문법도 사용할 수 있음을 의미합니다.

### The `!` Never Type that Never Returns

Rust has a special type named `!` that’s known in type theory lingo as the
*empty type* because it has no values. We prefer to call it the *never type*
because it stands in the place of the return type when a function will never
return. Here is an example:

```rust,ignore
fn bar() -> ! {
    // --snip--
}
```

This code is read as “the function `bar` returns never.” Functions that return
never are called *diverging functions*. We can’t create values of the type `!`
so `bar` can never possibly return.

But what use is a type you can never create values for? Recall the code from
Listing 2-5; we’ve reproduced it here in Listing 19-34.

```rust
# let guess = "3";
# loop {
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
# break;
# }
```

<span class="caption">Listing 19-34: A `match` with an arm that ends in
`continue`</span>

At the time, we skipped over some details in this code. In Chapter 6 in “The
`match` Control Flow Operator” section, we discussed that `match` arms must all
return the same type. So, for example, the following code doesn’t work:

```rust,ignore
let guess = match guess.trim().parse() {
    Ok(_) => 5,
    Err(_) => "hello",
}
```

The type of `guess` in this code would have to be an integer *and* a string,
and Rust requires that `guess` can only have one type. So what does `continue`
return? How were we allowed to return a `u32` from one arm and have another arm
that ends with `continue` in Listing 19-34?

As you might have guessed, `continue` has a `!` value. That is, when Rust
computes the type of `guess`, it looks at both match arms, the former with a
value of `u32` and the latter with a `!` value. Because `!` can never have a
value, Rust decides that the type of `guess` is `u32`.

The formal way of describing this behavior is that expressions of type `!` can
be coerced into any other type. We’re allowed to end this `match` arm with
`continue` because `continue` doesn’t return a value; instead, it moves control
back to the top of the loop, so in the `Err` case, we never assign a value to
`guess`.

The never type is useful with the `panic!` macro as well. Remember the `unwrap`
function that we call on `Option<T>` values to produce a value or panic? Here
is its definition:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

In this code, the same thing happens as in the `match` in Listing 19-34: Rust
sees that `val` has the type `T` and `panic!` has the type `!` so the result of
the overall `match` expression is `T`. This code works because `panic!` doesn’t
produce a value; it ends the program. In the `None` case, we won’t be returning
a value from `unwrap`, so this code is valid.

One final expression that has the type `!` is a `loop`:

```rust,ignore
print!("forever ");

loop {
    print!("and ever ");
}
```

Here, the loop never ends, so `!` is the value of the expression. However, this
wouldn’t be true if we included a `break`, because the loop would terminate
when it got to the `break`.

### Dynamically Sized Types and `Sized`

Due to Rust’s need to know certain details, such as how much space to allocate
for a value of a particular type, there is a corner of its type system that can
be confusing: the concept of *dynamically sized types*. Sometimes referred to
as *DSTs* or *unsized types*, these types let us write code using values whose
size we can only know at runtime.

Let’s dig into the details of a dynamically sized type called `str`, which
we’ve been using throughout the book. That’s right, not `&str`, but `str` on
its own, is a DST. We can’t know how long the string is until runtime, meaning
we can’t create a variable of type `str`, nor can we take an argument of type
`str`. Consider the following code, which does not work:

```rust,ignore
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

Rust needs to know how much memory to allocate for any value of a particular
type, and all values of a type must use the same amount of memory. If Rust
allowed us to write this code, these two `str` values would need to take up the
same amount of space. But they have different lengths: `s1` needs 12 bytes of
storage and `s2` needs 15. This is why it’s not possible to create a variable
holding a dynamically sized type.

So what do we do? In this case, you already know the answer: we make the types
of `s1` and `s2` a `&str` rather than a `str`. Recall that in the “String
Slices” section of Chapter 4 we said the slice data structure stores the
starting position and the length of the slice.

So although a `&T` is a single value that stores the memory address of where
the `T` is located, a `&str` is *two* values: the address of the `str` and its
length. As such, we can know the size of a `&str` value at compile time: it’s
two times the size of a `usize` in length. That is, we always know the size of
a `&str`, no matter how long the string it refers to is. In general, this is
the way in which dynamically sized types are used in Rust: they have an extra
bit of metadata that stores the size of the dynamic information. The golden
rule of dynamically sized types is that we must always put values of
dynamically sized types behind a pointer of some kind.

We can combine `str` with all kinds of pointers: for example, `Box<str>` or
`Rc<str>`. In fact, you’ve seen this before but with a different dynamically
sized type: traits. Every trait is a dynamically sized type we can refer to by
using the name of the trait. In Chapter 17 in the “Using Trait Objects that
Allow for Values of Different Types” section, we mentioned that to use traits
as trait objects, we must put them behind a pointer, such as `&Trait` or
`Box<Trait>` (`Rc<Trait>` would work too).

To work with DSTs, Rust has a particular trait called the `Sized` trait to
determine whether or not a type’s size is known at compile time. This trait is
automatically implemented for everything whose size is known at compile time.
In addition, Rust implicitly adds a bound on `Sized` to every generic function.
That is, a generic function definition like this:

```rust,ignore
fn generic<T>(t: T) {
    // --snip--
}
```

is actually treated as though we had written this:

```rust,ignore
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

By default, generic functions will only work on types that have a known size at
compile time. However, you can use the following special syntax to relax this
restriction:

```rust,ignore
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

A trait bound on `?Sized` is the opposite of a trait bound on `Sized`: we would
read this as “`T` may or may not be `Sized`.” This syntax is only available for
`Sized`, not any other traits.

Also note that we switched the type of the `t` parameter from `T` to `&T`.
Because the type might not be `Sized`, we need to use it behind some kind of
pointer. In this case, we’ve chosen a reference.

Next, we’ll talk about functions and closures!
