## 데이터 타입

러스트의 모든 값은 특정한 *데이터 타입*을 가지고 있고, 데이터 타입은 러스트에게 어떤 종류의 데이터가 지정될지 알려 어떻게 데이터를 처리할지 알려줍니다. 우리는 스칼라  타입(단 하나의 값만을 저장할 수 있음)과 컴파운드 타입(두 개 이상의 값을 저장할 수 있음)이라는 데이터 타입의 부분집합에 대해 살펴불 것입니다.

러스트가 *정적 타입 언어*이기 때문에, 모든 변수의 타입을 컴파일 시에 알아야 한다는 점을 알아두세요. 컴파일러는 일반적으로 그 값을 기반으로 무슨 타입을 사용하고 싶은지 추론할 수 있습니다. 2장의 ["Comparing the Guess to the Secret Number"]
[comparing-the-guess-to-the-secret-number]<!-- ignore -->>에서와 같이 `parse`로 `String`을 수 타입으로 변환할 때는 여러 타입이 가능하기 때문에, 다음과 같이 타입 어노테이션을 추가해야 합니다.

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

만약에 타입 어노테이션을 추가하지 않는다면, 우리가 사용하고 싶은 타입의 종류를 알아야 한다는 내용의 에러를 다음과 같이 보여줄 것입니다.

```text
에러[E0282]: 타입 어노테이션이 필요합니다
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^
  |         |
  |         `_`에 대한 타입 추론이 불가능합니다
  |         `guess`에게 타입을 주는 것을 고려해보세요
```

다른 데이터 타입에게는 다른 타입 어노테이션이 보일 것입니다.

### 스칼라 타입

스칼라 타입은 한개의 값을 표현합니다. 러스트는 네 개의 주된 스칼라 타입을 가지고 있는데, 정수, 부동(떠다니는) 소수점수, 부울 대수, 그리고 문자입니다. 아마 다른 프로그래밍 언어에서도 이들을 볼 수 있을 겁니다. 러스트에서는 어떻게 작동하는지 알아봅시다.

#### 정수 타입

*정수*는 분모가 1인 수입니다. 우리는 2장에서 한 가지 정수 타입인 `u32`타입을 사용했습니다. 이 타입 선언은 저장될 값이 부호 없는 정수여야 하고(부호 있는 정수 타입은 `u` 대신 `i`로 시작합니다) 32비트의 공간을 차지한다는 것을 의미합니다. 표 3-1은 러스트에 자체 내장된 정수 타입을 보여줍니다. 부호 있음 열과 부호 없음 열 각각의 변종들은(예를 들면 `i16`) 정수 타입을 선언하는데 쓰일 수 있습니다.

<span class="caption">표 3-1: 러스트의 정수 타입</span>

|  길이   |부호 있음 | 부호 없음 |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

각각의 변종들은 부호가 있거나 없고 명시적인 크기를 가지고 있습니다. *부호 있음*과 *부호 없음*은 음수가 되는 것이 가능하다와 불가능하다로 바꿔 부를 수 있고, 숫자에 부호가 필요한지와(부호 있음) 항상 양수만 될 것이기에 부호가 필요없으므로 부호 없이 표현할 수 있습니다(부호 없음). 종이에 숫자를 쓰는 것과 비슷한데, 부호가 중요할 때는 + 부호나 - 부호를 사용합니다. 하지만, 양수라고 추측해도 괜찮을 때에는, 부호를 사용하지 않습니다. 부호 있는 수는 [2의 보수](https://en.wikipedia.org/wiki/Two%27s_complement)를 사용하여 저장됩니다.

각각의 부호 있는 변종들은 -(2<sup>n - 1</sup>)에서 2<sup>n - 1</sup> - 1까지 저장할 수 있습니다. *n*은 변종이 사용하는 비트 수입니다. 따라서 `i8`은 -(2<sup>7</sup>)에서 2<sup>7</sup> - 1까지, 즉 -128부터 127까지의 수를 저장할 수 있습니다. 부호 없는 변종들은 0에서 2<sup>n</sup> - 1까지 저장할 수 있으며, 따라서 `u8`은 0에서 2<sup>8</sup> - 1까지 저장할 수 있고, 이는 0에서 255까지와 같습니다.

추가적으로, `isize`와 `usize`타입은 프로그램이 작동하는 종류에 따라 다릅니다. 64-비트 아키텍쳐라면 64비트, 32-비트 운영체제라면 32비트가 됩니다.

정수 리터럴은 표 3-2와 같이 어떤 형태로도 쓸 수 있습니다. 바이트 리터럴을 제외한 다른 모든 수 리터럴은 `57u8`과 같이 타입 접미사를 사용할 수 있고, `1_000`과 같이 `_`를 시각적 분리자로 사용할 수 있습니다.

<span class="caption">표 3-2: 러스트의 정수 리터럴</span>

|     수 리터럴     |     예시      |
|------------------|---------------|
| 10진수           | `98_222`      |
| 16진수           | `0xff`        |
| 8진수            | `0o77`        |
| 2진수            | `0b1111_0000` |
| 바이트 (`u8` only) | `b'A'`        |

그럼 어떤 타입의 정수를 써야 할까요? 확신이 서지 않는다면, 러스트의 기본값이 일반적으로 좋은 선택이고, 정수 타입의 기본값은 `i32`입니다. 이 타입이 64-비트 체제에서도 일반적으로 가장 빠릅니다. `isize`나 `usize`를 사용하는 주된 상황은 어떤 종류의 컬렉션을 색인할 때입니다.

> ##### Integer Overflow
>
> Let’s say you have a variable of type `u8` that can hold values between 0 and 255.
> If you try to change the variable to a value outside of that range, such
> as 256, *integer overflow* will occur. Rust has some interesting rules
> involving this behavior. When you’re compiling in debug mode, Rust includes
> checks for integer overflow that cause your program to *panic* at runtime if
> this behavior occurs. Rust uses the term panicking when a program exits with
> an error; we’ll discuss panics in more depth in the [“Unrecoverable Errors
> with `panic!`”][unrecoverable-errors-with-panic] section in Chapter 9.
>
> When you’re compiling in release mode with the `--release` flag, Rust does
> *not* include checks for integer overflow that cause panics. Instead, if
> overflow occurs, Rust performs *two’s complement wrapping*. In short, values
> greater than the maximum value the type can hold “wrap around” to the minimum
> of the values the type can hold. In the case of a `u8`, 256 becomes 0, 257
> becomes 1, and so on. The program won’t panic, but the variable will have a
> value that probably isn’t what you were expecting it to have. Relying on
> integer overflow’s wrapping behavior is considered an error. If you want to
> wrap explicitly, you can use the standard library type [`Wrapping`][wrapping].

#### Floating-Point Types

Rust also has two primitive types for *floating-point numbers*, which are
numbers with decimal points. Rust’s floating-point types are `f32` and `f64`,
which are 32 bits and 64 bits in size, respectively. The default type is `f64`
because on modern CPUs it’s roughly the same speed as `f32` but is capable of
more precision.

Here’s an example that shows floating-point numbers in action:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The
`f32` type is a single-precision float, and `f64` has double precision.

#### Numeric Operations

Rust supports the basic mathematical operations you’d expect for all of the
number types: addition, subtraction, multiplication, division, and remainder.
The following code shows how you’d use each one in a `let` statement:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

Each expression in these statements uses a mathematical operator and evaluates
to a single value, which is then bound to a variable. Appendix B contains a
list of all operators that Rust provides.

#### The Boolean Type

As in most other programming languages, a Boolean type in Rust has two possible
values: `true` and `false`. Booleans are one byte in size. The Boolean type in
Rust is specified using `bool`. For example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

The main way to use Boolean values is through conditionals, such as an `if`
expression. We’ll cover how `if` expressions work in Rust in the [“Control
Flow”][control-flow]<!-- ignore --> section.

#### The Character Type

So far we’ve worked only with numbers, but Rust supports letters too. Rust’s
`char` type is the language’s most primitive alphabetic type, and the following
code shows one way to use it. (Note that `char` literals are specified with
single quotes, as opposed to string literals, which use double quotes.)

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value,
which means it can represent a lot more than just ASCII. Accented letters;
Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all
valid `char` values in Rust. Unicode Scalar Values range from `U+0000` to
`U+D7FF` and `U+E000` to `U+10FFFF` inclusive. However, a “character” isn’t
really a concept in Unicode, so your human intuition for what a “character” is
may not match up with what a `char` is in Rust. We’ll discuss this topic in
detail in [“Storing UTF-8 Encoded Text with Strings”][strings]<!-- ignore -->
in Chapter 8.

### Compound Types

*Compound types* can group multiple values into one type. Rust has two
primitive compound types: tuples and arrays.

#### The Tuple Type

A tuple is a general way of grouping together some number of other values
with a variety of types into one compound type. Tuples have a fixed length:
once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside
parentheses. Each position in the tuple has a type, and the types of the
different values in the tuple don’t have to be the same. We’ve added optional
type annotations in this example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds to the entire tuple, because a tuple is considered a
single compound element. To get the individual values out of a tuple, we can
use pattern matching to destructure a tuple value, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

This program first creates a tuple and binds it to the variable `tup`. It then
uses a pattern with `let` to take `tup` and turn it into three separate
variables, `x`, `y`, and `z`. This is called *destructuring*, because it breaks
the single tuple into three parts. Finally, the program prints the value of
`y`, which is `6.4`.

In addition to destructuring through pattern matching, we can access a tuple
element directly by using a period (`.`) followed by the index of the value we
want to access. For example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

This program creates a tuple, `x`, and then makes new variables for each
element by using their index. As with most programming languages, the first
index in a tuple is 0.

#### The Array Type

Another way to have a collection of multiple values is with an *array*. Unlike
a tuple, every element of an array must have the same type. Arrays in Rust are
different from arrays in some other languages because arrays in Rust have a
fixed length, like tuples.

In Rust, the values going into an array are written as a comma-separated list
inside square brackets:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather than
the heap (we will discuss the stack and the heap more in Chapter 4) or when
you want to ensure you always have a fixed number of elements. An array isn’t
as flexible as the vector type, though. A vector is a similar collection type
provided by the standard library that *is* allowed to grow or shrink in size.
If you’re unsure whether to use an array or a vector, you should probably use a
vector. Chapter 8 discusses vectors in more detail.

An example of when you might want to use an array rather than a vector is in a
program that needs to know the names of the months of the year. It’s very
unlikely that such a program will need to add or remove months, so you can use
an array because you know it will always contain 12 items:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

You would write an array’s type by using square brackets, and within the
brackets include the type of each element, a semicolon, and then the number of
elements in the array, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, `i32` is the type of each element. After the semicolon, the number `5`
indicates the element contains five items.

Writing an array’s type this way looks similar to an alternative syntax for
initializing an array: if you want to create an array that contains the same
value for each element, you can specify the initial value, followed by a
semicolon, and then the length of the array in square brackets, as shown here:

```rust
let a = [3; 5];
```

The array named `a` will contain `5` elements that will all be set to the value
`3` initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a
more concise way.

##### Accessing Array Elements

An array is a single chunk of memory allocated on the stack. You can access
elements of an array using indexing, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, the variable named `first` will get the value `1`, because
that is the value at index `[0]` in the array. The variable named `second` will
get the value `2` from index `[1]` in the array.

##### Invalid Array Element Access

What happens if you try to access an element of an array that is past the end
of the array? Say you change the example to the following code, which will
compile but exit with an error when it runs:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,panics
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

Running this code using `cargo run` produces the following result:

```text
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/arrays`
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is
 10', src/main.rs:5:19
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

The compilation didn’t produce any errors, but the program resulted in a
*runtime* error and didn’t exit successfully. When you attempt to access an
element using indexing, Rust will check that the index you’ve specified is less
than the array length. If the index is greater than or equal to the array
length, Rust will panic.

This is the first example of Rust’s safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects you against this
kind of error by immediately exiting instead of allowing the memory access and
continuing. Chapter 9 discusses more of Rust’s error handling.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[wrapping]: ../std/num/struct.Wrapping.html
