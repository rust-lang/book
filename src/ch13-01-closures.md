## Closures

Rust gives you the ability to define *closures*, which are similar to
functions. Instead of starting with a technical definition, let's see what
closures look like, syntactically, and then we'll return to defining what they
are. Listing 13-1 shows a small closure whose definition is assigned to the
variable `add_one`, which we can then use to call the closure:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let add_one = |x| x + 1;

    let five = add_one(4);

    assert_eq!(5, five);
}
```

<figcaption>

Listing 13-1: A closure that takes one parameter and adds one to it, assigned to
the variable `add_one`

</figcaption>
</figure>

The closure definition, on the first line, shows that the closure takes one
parameter named `x`. Parameters to closures go in between vertical pipes (`|`).

This is a minimal closure with only one expression as its body. Listing 13-2 has
a closure with a bit more complexity:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let calculate = |a, b| {
        let mut result = a * 2;

        result += b;

        result
    };

    assert_eq!(7, calculate(2, 3)); // 2 * 2 + 3 == 7
    assert_eq!(13, calculate(4, 5)); // 4 * 2 + 5 == 13
}
```

<figcaption>

Listing 13-2: A closure with two parameters and multiple expressions in its body

</figcaption>
</figure>

We can use curly brackets to define a closure body with more than one
expression.

You'll notice a few things about closures that are different from functions
defined with the `fn` keyword. The first difference is that we did not need to
annotate the types of the parameters the closure takes or the value it returns.
We can choose to add type annotations if we want; Listing 13-3 shows the
closure from Listing 13-1 with annotations for the parameter's and return
value's types:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let add_one = |x: i32| -> i32 { x + 1 };

    assert_eq!(2, add_one(1));
}
```

<figcaption>

Listing 13-3: A closure definition with optional parameter and return value
type annotations

</figcaption>
</figure>

The syntax of closures and functions looks more similar with type annotations.
Let's compare the different ways we can specify closures with the syntax for
defining a function more directly. We've added some spaces here to line up the
relevant parts:

```rust
fn  add_one_v1   (x: i32) -> i32 { x + 1 }  // a function
let add_one_v2 = |x: i32| -> i32 { x + 1 }; // the full syntax for a closure
let add_one_v3 = |x: i32|        { x + 1 }; // a closure eliding types
let add_one_v4 = |x: i32|          x + 1  ; // without braces
```

The reason type annotations are not required for defining a closure but are
required for defining a function is that functions are part of an explicit
interface exposed to your users, so defining this interface rigidly is
important for ensuring that everyone agrees on what types of values a function
uses and returns. Closures aren't used in an exposed interface like this,
though: they're stored in bindings and called directly. Being forced to
annotate the types would be a significant ergonomic loss for little advantage.

Closure definitions do have one type inferred for each of their parameters and
for their return value. For instance, if we call the closure without type
annotations from Listing 13-1 using an `i8`, we'll get an error if we then try
to call the same closure with an `i32`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
let add_one = |x| x + 1;

let five = add_one(4i8);
assert_eq!(5i8, five);

let three = add_one(2i32);
```

The compiler gives us this error:

```text
error[E0308]: mismatched types
 -->
  |
7 | let three = add_one(2i32);
  |                     ^^^^ expected i8, found i32
```

Since closures' types can be inferred reliably since they're called directly,
it would be tedious if we were required to annotate their types.

Another reason to have a different syntax from functions for closures is that
they have different behavior than functions: closures possess an *environment*.

### Closures Can Reference Their Environment

We've learned that functions can only use variables that are in scope, either
by being `const` or being declared as parameters. Closures can do more: they're
allowed to access variables from their enclosing scope. Listing 13-4 has an
example of a closure in the variable `equal_to_x` that uses the variable `x`
from the closure's surrounding environment:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

<figcaption>

Listing 13-4: Example of a closure that refers to a variable in its enclosing
scope

</figcaption>
</figure>

Here, even though `x` is not one of the parameters of `equal_to_x`, the
`equal_to_x` closure is allowed to use `x`, since `x` is a variable defined in
the scope that `equal_to_x` is defined. We aren't allowed to do the same thing
that Listing 13-4 does with functions; let's see what happens if we try:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}
```

We get an error:

```text
error[E0434]: can't capture dynamic environment in a fn item; use the || { ... }
closure form instead
 -->
  |
4 |     fn equal_to_x(z: i32) -> bool { z == x }
  |                                          ^
```

The compiler even reminds us that this only works with closures!

### Closures, Ownership, and Borrowing

The property of being allowed to use variables from the surrounding scope is
also subject to all of the usual rules around ownership and borrowing. Since
closures attempt to infer the types of their parameters, they also infer how
those parameters are borrowed. Closures make that inference by looking at how
they are used. Consider the example in Listing 13-5 that has functions that
borrow immutably, borrow mutably, and move their parameters, then closures that
reference values from their environment and call each of the functions. We'll
see how this affects inference of when a value is borrowed:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
struct Foo;

fn borrows(f: &Foo) {
    println!("Took {:?} by reference.", f);
}

fn borrows_mut(f: &mut Foo) {
    println!("Took {:?} by mutable reference.", f);
}

fn moves(f: Foo) {
    println!("Took ownership of {:?}.", f);
}

fn main() {
    let f1 = Foo;
    let closure_that_borrows = |x| borrows(x);
    closure_that_borrows(&f1);

    let mut f2 = Foo;
    let closure_that_borrows_mut = |y| borrows_mut(y);
    closure_that_borrows_mut(&mut f2);

    let f3 = Foo;
    let closure_that_moves = |z| moves(z);
    closure_that_moves(f3);
}
```

<figcaption>

Listing 13-5: Closures that borrow, borrow mutably, and take ownership of their
parameters, which is inferred from how the closure body uses the parameters

</figcaption>
</figure>

Here, Rust is able to look at how we use the parameters of each closure inside
their bodies. If the closure passes its parameter it to a function that takes
`&Foo`, then the type of the parameter must be `&Foo`. If it passes the
parameter to a function that takes `&mut Foo`, then the type of parameter must
be `&mut Foo`, and so on. If we try to use `f3` after the call to
`closure_that_moves` in the last line of `main`, we'll get a compiler error
since ownership of `f3` was transferred to `closure_that_moves`, which
transferred ownership to the function `moves`.

### Overriding Inferred Borrowing with the `move` Keyword

Rust will allow you to override the borrowing inference by using the `move`
keyword. This will cause all of the closure's parameters to be taken by
ownership, instead of whatever they were inferred as. Consider this example:

```rust
let mut num = 4;

{
    let mut add_num = |x| num += x;

    add_num(6);
}

assert_eq!(10, num);
```

In this case, the `add_num` closure took a mutable reference to `num`, then
when we called `add_num`, it mutated the underlying value. In the last line,
`num` contains 10, as we'd expect. We also needed to declare `add_num` itself
as `mut` too, because we're mutating its environment.

If we change the definition of `add_num` to a `move` closure, the behavior is
different:

```rust
let mut num = 4;

{
    let mut add_num = move |x| num += x;

    add_num(6);
}

assert_eq!(4, num);
```

In the last line, `num` now contains 4: `add_num` took ownership of a copy of
`num`, rather than mutably borrowing `num`.

One of the most common places you'll see the `move` keyword used is with
threads, since it's important that one thread is no longer allowed to use a
value once the value has been transferred to another thread through a closure
in order to prevent data races. We'll talk more about that in Chapter XX.

### Closures and Lifetimes

Remember Listing 10-8 from the Lifetime Syntax section of Chapter 10? It looked
like this:

```rust,ignore
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

This example doesn't compile since `x` doesn't have a long enough lifetime.
Because closures may borrow variables from their enclosing scope, we can
construct a similar example with a closure that borrows `x` and tries to return
that borrowed value. The code in Listing 13-6 also won't compile:

<figure>

```rust,ignore
{
    let closure;

    {
        let x = 4;

        closure = || x ; // A closure that takes no arguments and returns x.
    }
}
```

<figcaption>

Listing 13-6: A closure that tries to return a borrowed value that does not live
long enough

</figcaption>
</figure>

We get an error because `x` does not live long enough:

```text
error: `x` does not live long enough
  -->
   |
8  |         closure = || x ; // A closure that takes no arguments and returns x.
   |                   -- ^ does not live long enough
   |                   |
   |                   capture occurs here
9  |     }
   |     - borrowed value only lives until here
10 | }
   | - borrowed value needs to live until here
```

To fix the error in the code in Listing 13-6, we can use the `move` keyword
from the last section to make the closure take ownership of `x`. Because `x` is
a number, it is a `Copy` type and therefore will be copied into the closure.
The code in Listing 13-7 will compile:

<figure>

```rust
{
    let closure;

    {
        let mut x = 4;

        closure = move || x ; // A closure that takes no arguments and returns x.

        x = 5;

        assert_eq!(closure(), 4);
    }
}
```

<figcaption>

Listing 13-7: Moving a value into the closure to fix the lifetime error

</figcaption>
</figure>

Even though we modified `x` between the closure definition and `assert_eq!`,
since `closure` now has its own version, the changes to `x` won't change the
version of `x` that's in the closure.

Rust doesn't provide a way to say that some values a closure uses should be
borrowed and some should be moved; it's either all by inference or all moved by
adding the `move` keyword. However, we can accomplish the goal of borrowing
some values and taking ownership of others by combining `move` with some extra
bindings. Consider this example where we want to borrow `s1` but take ownership
of `s2`:

```rust
let s1 = String::from("hello");
let s2 = String::from("goodbye");

let r = &s1;

let calculation = move || {
    r;
    s2;
};

println!("Can still use s1 here but not s2: {}", s1);
```

We've declared `calculation` to `move` all the values it references. Before
defining `calculation`, we declare a new variable `r` that borrows `s1`. Then
in the body of the `calculation` closure, we use `r` instead of using `s1`
directly. The closure takes ownership of `r`, but `r` is a reference, so the
closure hasn't taken ownership of `s1` even though `calculation` uses `move`.

### Closures as Function Parameters Using the `Fn` Traits

While we can bind closures to variables, that's not the most useful thing we
can do with them. We can also define functions that have closures as parameters
by using the `Fn` traits. Here's an example of a function named `call_with_one`
whose signature has a closure as a parameter:

```rust
fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32 {

    some_closure(1)
}

let answer = call_with_one(|x| x + 2);

assert_eq!(3, answer);
```

We pass the closure `|x| x + 2`, to `call_with_one`, and `call_with_one` calls
that closure with `1` as an argument. The return value of the call to
`some_closure` is then returned from `call_with_one`.

The signature of `call_with_one` is using the `where` syntax discussed in the
Traits section of Chapter 10. The `some_closure` parameter has the generic type
`F`, which in the `where` clause is defined as having the trait bounds
`Fn(i32) -> i32`. The `Fn` trait represents a closure, and we can add types to
the `Fn` trait to represent a specific type of closure. In this case, our
closure has a parameter of type `i32` and returns an `i32`, so the generic bound
we specify is `Fn(i32) -> i32`.

Specifying a function signature that contains a closure requires the use of
generics and trait bounds. Each closure has a unique type, so we can't write
the type of a closure directly, we have to use generics.

`Fn` isn't the only trait bound available for specifying closures, however.
There are three: `Fn`, `FnMut`, and `FnOnce`. This continues the patterns of
threes we've seen elsewhere in Rust: borrowing, borrowing mutably, and
ownership. Using `Fn` specifies that the closure used may only borrow values in
its environment. To specify a closure that mutates the environment, use
`FnMut`, and if the closure takes ownership of the environment, `FnOnce`. Most
of the time, you can start with `Fn`, and the compiler will tell you if you
need `FnMut` or `FnOnce` based on what happens when the function calls the
closure.

To illustrate a situation where it's useful for a function to have a parameter
that's a closure, let's move on to our next topic: iterators.
