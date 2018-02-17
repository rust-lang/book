## Deref 트레잇을 가지고 스마트 포인터를 평범한 참조자와 같이 취급하기

Deref 트레잇을 구현하는 것은 우리가 (곱하기 혹은 글롭 연산자와는 반대측에 있는)
*역참조 연산자 (dereference operator)* `*` 의 동작을 커스터마이징 하는
것을 허용합니다. 스마트 포인터가 평범한 참조자처럼 취급될 수 있는 방식으로 Deref를
구현함으로써, 우리는 참조자에 대해 작동하는 코드를 작성하고 이 코드를 또한 스마트
포인터에도 사용할 수 있습니다.

먼저 `*`가 보통의 참조자와 어떤식으로 동작하는지를 살펴보고, 그런 다음 `Box<T>`와
비슷한 우리만의 타입을 정의하는 시도를 해서 왜 `*`가 우리의 새로 정의된 타입에서는
참조자처럼 작동하지 않는지를 봅시다. 우리는 `Defer` 트레잇을 구현하는 것이 어떻게
스마트 포인터가 참조자와 유사한 방식으로 동작하는 것을 가능하게 해주는지를 탐구할
것입니다. 그런 뒤 러스트의 *역참조 강제 (deref corecion)* 기능과 이 기능이 어떻게
참조자 혹은 스마트 포인터와 함께 동작하도록 하는지 살펴보겠습니다.

### `*`와 함께 포인터를 따라가서 값을 얻기

보통의 참조자는 포인터 타입이며, 포인터를 생각하는 한가지 방법은 다른 어딘가에
저장된 값을 가리키는 화살표로서 생각하는 것입니다. Listing 15-6에서는 `i32`
값의 참조자를 생성하고는 참조자를 따라가서 값을 얻기 위해 역참조 연산자를
사용합니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<span class="caption">Listing 15-6: 역참조 연산자를 사용하여 `i32` 값에 대한
참조자를 따라가기</span>

변수 `x`는 `i32` 값을 가지고 있습니다. `y`에는 `x`의 참조자를 설정했습니다.
우리는 `x`가 `5`와 동일함을 단언할 수 있습니다. 하지만, 만일 `y` 안의 값에
대한 단언을 만들고 싶다면, 참조자를 따라가서 이 참조자가 가리키고 있는 값을
얻기 위해 `*y`를 사용해야 합니다 (그래서 *역참조*라 합니다). 일단 `y`를
역참조하면, `5`와 비교 가능한 `y`가 가리키고 있는 정수값에 접근하게
됩니다.

대신 `assert_eq!(5, y);`라고 작성하길 시도했다면, 아래와 같은 컴파일 에러를
얻을 것입니다:

```text
error[E0277]: the trait bound `{integer}: std::cmp::PartialEq<&{integer}>` is
not satisfied
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^^ can't compare `{integer}` with `&{integer}`
  |
  = help: the trait `std::cmp::PartialEq<&{integer}>` is not implemented for
  `{integer}`
```

숫자와 숫자에 대한 참조자를 비교하는 것은 허용되지 않는데 그 이유는 이들이 서로
다른 타입이기 때문입니다. `*`를 사용하여 해당 잠조자를 따라가서 그것이 가리키고 있는
값을 얻어야 합니다.

### `Box<T>`를 참조자처럼 사용하기

Listing 15-7에서 보는 바와 같이, Listing 15-6의 코드는 참조자 대신
`Box<T>`를 이용하여 재작성될 수 있으며, 역참조 연산자는 동일한 방식으로
작동될 것입니다:


<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<span class="caption">Listing 15-7: `Box<i32>` 상에 역참조 연산자
사용하기</span>

Listing 15-7와 Listing 15-6 사이의 차이점은 오직 `x`의 값을 가리키는
참조자보다는 `x`를 가리키는 박스의 인스턴스로 `y`를 설정했다는 것입니다.
마지막 단언문에서, 우리는 `y`가 참조자일때 했던 것과 동일한 방식으로 박스
포인터 앞에 역참조 연산자를 사용할 수 있습니다. 다음으로, 우리만의 박스 타입을
정의함으로써 `Box<T>`가 우리에게 역참조 연산자를 사용 가능하게끔 해주는
특별함이 무엇인지 탐구해 보겠습니다.

### Defining Our Own Smart Pointer

Let’s build a smart pointer similar to the `Box<T>` type provided by the
standard library to experience how smart pointers behave differently to
references by default. Then we’ll look at how to add the ability to use the
dereference operator.

The `Box<T>` type is ultimately defined as a tuple struct with one element, so
Listing 15-8 defines a `MyBox<T>` type in the same way. We’ll also define a
`new` function to match the `new` function defined on `Box<T>`:

<span class="filename">Filename: src/main.rs</span>

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

<span class="caption">Listing 15-8: Defining a `MyBox<T>` type</span>

We define a struct named `MyBox` and declare a generic parameter `T`, because
we want our type to hold values of any type. The `MyBox` type is a tuple struct
with one element of type `T`. The `MyBox::new` function takes one parameter of
type `T` and returns a `MyBox` instance that holds the value passed in.

Let’s try adding the `main` function in Listing 15-7 to Listing 15-8 and
changing it to use the `MyBox<T>` type we’ve defined instead of `Box<T>`. The
code in Listing 15-9 won’t compile because Rust doesn’t know how to dereference
`MyBox`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<span class="caption">Listing 15-9: Attempting to use `MyBox<T>` in the same
way we used references and `Box<T>`</span>

Here’s the resulting compilation error:

```text
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^
```

Our `MyBox<T>` type can’t be dereferenced because we haven’t implemented that
ability on our type. To enable dereferencing with the `*` operator, we
implement the `Deref` trait.

### Treating a Type Like a Reference by Implementing the `Deref` Trait

As discussed in Chapter 10, to implement a trait, we need to provide
implementations for the trait’s required methods. The `Deref` trait, provided
by the standard library, requires us to implement one method named `deref` that
borrows `self` and returns a reference to the inner data. Listing 15-10
contains an implementation of `Deref` to add to the definition of `MyBox`:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::ops::Deref;

# struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

<span class="caption">Listing 15-10: Implementing `Deref` on `MyBox<T>`</span>

The `type Target = T;` syntax defines an associated type for the `Deref` trait
to use. Associated types are a slightly different way of declaring a generic
parameter, but you don’t need to worry about them for now; we’ll cover them in
more detail in Chapter 19.

We fill in the body of the `deref` method with `&self.0` so `deref` returns a
reference to the value we want to access with the `*` operator. The `main`
function in Listing 15-9 that calls `*` on the `MyBox<T>` value now compiles
and the assertions pass!

Without the `Deref` trait, the compiler can only dereference `&` references.
The `deref` method gives the compiler the ability to take a value of any type
that implements `Deref` and call the `deref` method to get a `&` reference that
it knows how to dereference.

When we entered `*y` in Listing 15-9, behind the scenes Rust actually ran this
code:

```rust,ignore
*(y.deref())
```

Rust substitutes the `*` operator with a call to the `deref` method and then a
plain dereference so as programmers we don’t have to think about whether or not
we need to call the `deref` method. This Rust feature lets us write code that
functions identically whether we have a regular reference or a type that
implements `Deref`.

The reason the `deref` method returns a reference to a value and that the plain
dereference outside the parentheses in `*(y.deref())` is still necessary is due
to the ownership system. If the `deref` method returned the value directly
instead of a reference to the value, the value would be moved out of `self`. We
don’t want to take ownership of the inner value inside `MyBox<T>` in this case
and in most cases where we use the dereference operator.

Note that the `*` is replaced with a call to the `deref` method and then a call
to `*` just once, each time we type a `*` in our code. Because the substitution
of `*` does not recurse infinitely, we end up with data of type `i32`, which
matches the `5` in `assert_eq!` in Listing 15-9.

### Implicit Deref Coercions with Functions and Methods

*Deref coercion* is a convenience that Rust performs on arguments to functions
and methods. Deref coercion converts a reference to a type that implements
`Deref` into a reference to a type that `Deref` can convert the original type
into. Deref coercion happens automatically when we pass a reference to a
particular type’s value as an argument to a function or method that doesn’t
match the parameter type in the function or method definition. A sequence of
calls to the `deref` method converts the type we provided into the type the
parameter needs.

Deref coercion was added to Rust so that programmers writing function and
method calls don’t need to add as many explicit references and dereferences
with `&` and `*`. The deref coercion feature also lets us write more code that
can work for either references or smart pointers.

To see deref coercion in action, let’s use the `MyBox<T>` type we defined in
Listing 15-8 as well as the implementation of `Deref` that we added in Listing
15-10. Listing 15-11 shows the definition of a function that has a string slice
parameter:

<span class="filename">Filename: src/main.rs</span>

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
```

<span class="caption">Listing 15-11: A `hello` function that has the parameter
`name` of type `&str`</span>

We can call the `hello` function with a string slice as an argument, such as
`hello("Rust");` for example. Deref coercion makes it possible to call `hello`
with a reference to a value of type `MyBox<String>`, as shown in Listing 15-12:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::ops::Deref;
#
# struct MyBox<T>(T);
#
# impl<T> MyBox<T> {
#     fn new(x: T) -> MyBox<T> {
#         MyBox(x)
#     }
# }
#
# impl<T> Deref for MyBox<T> {
#     type Target = T;
#
#     fn deref(&self) -> &T {
#         &self.0
#     }
# }
#
# fn hello(name: &str) {
#     println!("Hello, {}!", name);
# }
#
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

<span class="caption">Listing 15-12: Calling `hello` with a reference to a
`MyBox<String>` value, which works because of deref coercion</span>

Here we’re calling the `hello` function with the argument `&m`, which is a
reference to a `MyBox<String>` value. Because we implemented the `Deref` trait
on `MyBox<T>` in Listing 15-10, Rust can turn `&MyBox<String>` into `&String`
by calling `deref`. The standard library provides an implementation of `Deref`
on `String` that returns a string slice, which is in the API documentation for
`Deref`. Rust calls `deref` again to turn the `&String` into `&str`, which
matches the `hello` function’s definition.

If Rust didn’t implement deref coercion, we would have to write the code in
Listing 15-13 instead of the code in Listing 15-12 to call `hello` with a value
of type `&MyBox<String>`:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::ops::Deref;
#
# struct MyBox<T>(T);
#
# impl<T> MyBox<T> {
#     fn new(x: T) -> MyBox<T> {
#         MyBox(x)
#     }
# }
#
# impl<T> Deref for MyBox<T> {
#     type Target = T;
#
#     fn deref(&self) -> &T {
#         &self.0
#     }
# }
#
# fn hello(name: &str) {
#     println!("Hello, {}!", name);
# }
#
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

<span class="caption">Listing 15-13: The code we would have to write if Rust
didn’t have deref coercion</span>

The `(*m)` dereferences the `MyBox<String>` into a `String`. Then the `&` and
`[..]` take a string slice of the `String` that is equal to the whole string to
match the signature of `hello`. The code without deref coercions is harder to
read, write, and understand with all of these symbols involved. Deref coercion
allows Rust to handle these conversions for us automatically.

When the `Deref` trait is defined for the types involved, Rust will analyze the
types and use `Deref::deref` as many times as necessary to get a reference to
match the parameter’s type. The number of times that `Deref::deref` needs to be
inserted is resolved at compile time, so there is no runtime penalty for taking
advantage of deref coercion!

### How Deref Coercion Interacts with Mutability

Similar to how we use the `Deref` trait to override `*` on immutable
references, Rust provides a `DerefMut` trait for overriding `*` on mutable
references.

Rust does deref coercion when it finds types and trait implementations in three
cases:

* From `&T` to `&U` when `T: Deref<Target=U>`
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
* From `&mut T` to `&U` when `T: Deref<Target=U>`

The first two cases are the same except for mutability. The first case states
that if you have a `&T`, and `T` implements `Deref` to some type `U`, you can
get a `&U` transparently. The second case states that the same deref coercion
happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an
immutable one. But the reverse is *not* possible: immutable references will
never coerce to mutable references. Because of the borrowing rules, if you have
a mutable reference, that mutable reference must be the only reference to that
data (otherwise, the program wouldn’t compile). Converting one mutable
reference to one immutable reference will never break the borrowing rules.
Converting an immutable reference to a mutable reference would require that
there is only one immutable reference to that data, and the borrowing rules
don’t guarantee that. Therefore, Rust can’t make the assumption that converting
an immutable reference to a mutable reference is possible.
