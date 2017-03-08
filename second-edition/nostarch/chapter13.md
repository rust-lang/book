
[TOC]

# Functional Language features in Rust - Iterators and Closures

Rust's design has taken inspiration from a lot of previous work. One of Rust's
influences is functional programming, where functions are values that can be
used as arguments or return values to other functions, assigned to variables,
and so forth. We're going to sidestep the issue of what, exactly, functional
programming is or is not, and instead show off some features of Rust that
are similar to features in many languages referred to as functional.

More specifically, we're going to cover:

* *Closures*, a function-like construct you can store in a variable.
* *Iterators*, a way of processing series of elements.
* How to use these features to improve upon the project from the last chapter.
* The performance of these features. Spoiler alert: they're faster than you
  might think!

This is not a complete list of Rust's influence from the functional style:
pattern matching, enums, and many other features are too. But mastering
closures and iterators are an important part of writing idiomatic, fast Rust
code.

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

```rust,ignore
fn  add_one_v1   (x: i32) -> i32 { x + 1 }  // a function
let add_one_v2 = |x: i32| -> i32 { x + 1 }; // the full syntax for a closure
let add_one_v3 = |x|             { x + 1 }; // a closure eliding types
let add_one_v4 = |x|               x + 1  ; // without braces
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

Creating closures that capture values from their environment is mostly used in
the context of starting new threads. We'll show some more examples and explain
more detail about this feature of closures in Chapter 16 when we talk about
concurrency.

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

## Iterators

Iterators are a pattern in Rust that allows you to do some processing on a
sequence of items. For example, the code in Listing 13-5 adds one to each
number in a vector:

<figure>

```rust
let v1 = vec![1, 2, 3];

let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, [2, 3, 4]);
```

<figcaption>

Listing 13-5: Using an iterator, `map`, and `collect` to add one to each number
in a vector

</figcaption>
</figure>

<!-- Will add wingdings in libreoffice /Carol -->

The `iter` method on vectors allows us to produce an *iterator* from the
vector. Next, the `map` method called on the iterator allows us to process each
element: in this case, we've passed a closure to `map` that specifies for every
element `x`, add one to it. `map` is one of the most basic ways of interacting
with an iterator, as processing each element in turn is very useful! Finally,
the `collect` method consumes the iterator and puts the iterator's elements
into a new data structure. In this case, since we've said that `v2` has the
type `Vec<i32>`, `collect` will create a new vector out of the `i32` values.

Methods on iterators like `map` are sometimes called *iterator adaptors*
because they take one iterator and produce a new iterator. That is, `map`
builds on top of our previous iterator and produces another iterator by calling
the closure it's passed to create the new sequence of values.

So, to recap, this line of code does the following:

1. Creates an iterator from the vector.
2. Uses the `map` adaptor with a closure argument to add one to each element.
3. Uses the `collect` adaptor to consume the iterator and make a new vector.

That's how we end up with `[2, 3, 4]`. As you can see, closures are a very
important part of using iterators: they provide a way of customizing the
behavior of an iterator adaptor like `map`.

### Iterators are Lazy

In the previous section, you may have noticed a subtle difference in wording:
we said that `map` *adapts* an iterator, but `collect` *consumes* one. That was
intentional. By themselves, iterators won't do anything; they're lazy. That is,
if we write code like Listing 13-5 except we don't call `collect`:

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1); // without collect
```

It will compile, but it will give us a warning:

```text
warning: unused result which must be used: iterator adaptors are lazy and do
nothing unless consumed, #[warn(unused_must_use)] on by default
 --> src/main.rs:4:1
  |
4 | v1.iter().map(|x| x + 1); // without collect
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
```

We get this warning because iterator adaptors won't start actually doing the
processing on their own. They need some other method that causes the iterator
chain to evaluate. We call those *consuming adaptors*, and `collect` is one of
them.

So how do we tell which iterator methods consume the iterator or not? And what
adaptors are available? For that, let's look at the `Iterator` trait.

### The `Iterator` trait

Iterators all implement a trait named `Iterator` that is defined in the standard
library. The definition of the trait looks like this:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

There's some new syntax that we haven't covered here yet: `type Item` and
`Self::Item` are defining an *associated type* with this trait, and we'll talk
about associated types in depth in Chapter XX. For now, all you need to know is
that this code says the `Iterator` trait requires that you also define an
`Item` type, and this `Item` type is used in the return type of the `next`
method. In other words, the `Item` type will be the type of element that's
returned from the iterator.

Let's make an iterator named `Counter` that will count from `1` to `5`, using
the `Iterator` trait. First, we need to create a struct that holds the current
state of the iterator, which is one field named `count` that will hold a `u32`.
We'll also define a `new` method, which isn't strictly necessary. We want our
`Counter` to go from one to five, though, so we're always going to have it
holding a zero to start:

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

Next, we're going to implement the `Iterator` trait for our `Counter` type by
defining the body of the `next` method. The way we want our iterator to work
is to add one to the state (which is why we initialized `count` to 0, since we
want our iterator to return one first). If `count` is still less than six, we'll
return the current value, but if `count` is six or higher, our iterator will
return `None`, as shown in Listing 13-6:

<figure>

```rust
impl Iterator for Counter {
    // Our iterator will produce u32s
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // increment our count. This is why we started at zero.
        self.count += 1;

        // check to see if we've finished counting or not.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

<figcaption>

Listing 13-6: Implementing the `Iterator` trait on our `Counter` struct

</figcaption>
</figure>

<!-- I will add wingdings in libreoffice /Carol -->

The `type Item = u32` line is saying that the associated `Item` type will be
a `u32` for our iterator. Again, don't worry about associated types yet, because
we'll be covering them in Chapter XX.

The `next` method is the main interface into an iterator, and it returns an
`Option`. If the option is `Some(value)`, we have gotten another value from the
iterator. If it's `None`, iteration is finished. Inside of the `next` method,
we do whatever kind of calculation our iterator needs to do. In this case, we
add one, then check to see if we're still below six. If we are, we can return
`Some(self.count)` to produce the next value. If we're at six or more,
iteration is over, so we return `None`.

The iterator trait specifies that when an iterator returns `None`, that
indicates iteration is finished. The trait does not mandate anything about the
behavior an iterator must have if the `next` method is called again after
having returned one `None` value. In this case, every time we call `next` after
getting the first `None` value will still return `None`, but the internal
`count` field will continue to be incremented by one each time. If we call
`next` as many times as the maximum value a `u32` value can hold, `count` will
overflow (which will `panic!` in debug mode and wrap in release mode). Other
iterator implementations choose to start iterating again. If you need to be
sure to have an iterator that will always return `None` on subsequent calls to
the `next` method after the first `None` value is returned, you can use the
`fuse` method to create an iterator with that characteristic out of any other
iterator.

Once we've implemented the `Iterator` trait, we have an iterator! We can use
the iterator functionality that our `Counter` struct now has by calling the
`next` method on it repeatedly:

```rust,ignore
let mut counter = Counter::new();

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);
```

This will print `Some(1)` through `Some(5)` and then `None`, each on their own
line.

### All Sorts of `Iterator` Adaptors

In Listing 13-5, we had iterators and we called methods like `map` and
`collect` on them. In Listing 13-6, however, we only implemented the `next`
method on our `Counter`. How do we get methods like `map` and `collect` on our
`Counter`?

Well, when we told you about the definition of `Iterator`, we committed a small
lie of omission. The `Iterator` trait has a number of other useful methods
defined on it that come with default implementations that call the `next`
method. Since `next` is the only method of the `Iterator` trait that does not
have a default implementation, once you've done that, you get all of the other
`Iterator` adaptors for free. There are a lot of them!

For example, if for some reason we wanted to take the first five values that
an instance of `Counter` produces, pair those values with values produced by
another `Counter` instance after skipping the first value that instance
produces, multiply each pair together, keep only those results that are
divisible by three, and add all the resulting values together, we could do:

```rust,ignore
let sum: u32 = Counter::new().take(5)
                             .zip(Counter::new().skip(1))
                             .map(|(a, b)| a * b)
                             .filter(|x| x % 3 == 0)
                             .sum();
assert_eq!(48, sum);
```

All of these method calls are possible because we implemented the `Iterator`
trait by specifying how the `next` method works. Use the standard library
documentation to find more useful methods that will come in handy when you're
working with iterators.

## Improving our I/O Project

In our I/O project implementing `grep` in the last chapter, there are some
places where the code could be made clearer and more concise using iterators.
Let's take a look at how iterators can improve our implementation of the
`Config::new` function and the `grep` function.

### Removing a `clone` by Using an Iterator

Back in listing 12-8, we had this code that took a slice of `String` values and
created an instance of the `Config` struct by checking for the right number of
arguments, indexing into the slice, and cloning the values so that the `Config`
struct could own those values:

```rust,ignore
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let search = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            search: search,
            filename: filename,
        })
    }
}
```

At the time, we said not to worry about the `clone` calls here, and that we
could remove them in the future. Well, that time is now! So, why do we need
`clone` here? The issue is that we have a slice with `String` elements in the
parameter `args`, and the `new` function does not own `args`. In order to be
able to return ownership of a `Config` instance, we need to clone the values
that we put in the `search` and `filename` fields of `Config`, so that the
`Config` instance can own its values.

Now that we know more about iterators, we can change the `new` function to
instead take ownership of an iterator as its argument. We'll use the iterator
functionality instead of having to check the length of the slice and index into
specific locations. Since we've taken ownership of the iterator, and we won't be
using indexing operations that borrow anymore, we can move the `String` values
from the iterator into `Config` instead of calling `clone` and making a new
allocation.

First, let's take `main` as it was in Listing 12-6, and change it to pass the
return value of `env::args` to `Config::new`, instead of calling `collect` and
passing a slice:

```rust,ignore
fn main() {
    let config = Config::new(env::args());
    // ...snip...
```

<!-- Will add ghosting in libreoffice /Carol -->

If we look in the standard library documentation for the `env::args` function,
we'll see that its return type is `std::env::Args`. So next we'll update the
signature of the `Config::new` function so that the parameter `args` has the
type `std::env::Args` instead of `&[String]`:


```rust,ignore
impl Config {
    fn new(args: std::env::Args) -> Result<Config, &'static str> {
        // ...snip...
```

<!-- Will add ghosting in libreoffice /Carol -->

Next, we'll fix the body of `Config::new`. As we can also see in the standard
library documentation, `std::env::Args` implements the `Iterator` trait, so we
know we can call the `next` method on it! Here's the new code:

```rust
impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    	args.next();

        let search = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a search string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config {
            search: search,
            filename: filename,
        })
    }
}
```

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Remember that the first value in the return value of `env::args` is the name of
the program. We want to ignore that, so first we'll call `next` and not do
anything with the return value. The second time we call `next` should be the
value we want to put in the `search` field of `Config`. We use a `match` to
extract the value if `next` returns a `Some`, and we return early with an `Err`
value if there weren't enough arguments (which would cause this call to `next`
to return `None`).

We do the same thing for the `filename` value. It's slightly unfortunate that
the `match` expressions for `search` and `filename` are so similar. It would be
nice if we could use `?` on the `Option` returned from `next`, but `?` only
works with `Result` values currently. Even if we could use `?` on `Option` like
we can on `Result`, the value we would get would be borrowed, and we want to
move the `String` from the iterator into `Config`.

### Making Code Clearer with Iterator Adaptors

The other bit of code where we could take advantage of iterators was in the
`grep` function as implemented in Listing 12-15:

<!-- We hadn't had a listing number for this code sample when we submitted
chapter 12; we'll fix the listing numbers in that chapter after you've
reviewed it. /Carol -->

```rust
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(search) {
            results.push(line);
        }
    }

    results
}
```

We can write this code in a much shorter way, and avoiding having to have a
mutable intermediate `results` vector, by using iterator adaptor methods like
this instead:

```rust
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(search))
        .collect()
}
```

Here, we use the `filter` adaptor to only keep the lines that
`line.contains(search)` returns true for. We then collect them up into another
vector with `collect`. Much simpler!

We can use the same technique in the `grep_case_insensitive` function that we
defined in Listing 12-16 as follows:

<!-- Similarly, the code snippet that will be 12-16 didn't have a listing
number when we sent you chapter 12, we will fix it. /Carol -->

```rust
fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| {
            line.to_lowercase().contains(&search)
        }).collect()
}
```

Not too bad! So which style should you choose? Most Rust programmers prefer to
use the iterator style. It's a bit tougher to understand at first, but once you
gain an intuition for what the various iterator adaptors do, this is much
easier to understand. Instead of fiddling with the various bits of looping
and building a new vector, the code focuses on the high-level objective of the
loop, abstracting some of the commonplace code so that it's easier to see the
concepts that are unique to this usage of the code, like the condition on which
the code is filtering each element in the iterator.

But are they truly equivalent? Surely the more low-level loop will be faster.
Let's talk about performance.

## Performance

Which version of our `grep` functions is faster: the version with an explicit
`for` loop or the version with iterators? We ran a benchmark by loading the
entire contents of "The Adventures of Sherlock Holmes" by Sir Arthur Conan
Doyle into a `String` and looking for the word "the" in the contents. Here were
the results of the benchmark on the version of grep using the `for` loop and the
version using iterators:

```text
test bench_grep_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_grep_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

The iterator version ended up slightly faster! We're not going to go through
the benchmark code here, as the point is not to prove that they're exactly
equivalent, but to get a general sense of how these two implementations
compare. For a *real* benchmark, you'd want to check various texts of various
sizes, different words, words of different lengths, and all kinds of other
variations. The point is this: iterators, while a high-level abstraction, get
compiled down to roughly the same code as if you'd written the lower-level code
yourself. Iterators are one of Rust's *zero-cost abstractions*, by which we mean
using the abstraction imposes no additional runtime overhead in the same way
that Bjarne Stroustrup, the original designer and implementer of C++, defines
*zero-overhead*:

> In general, C++ implementations obey the zero-overhead principle: What you
> don’t use, you don’t pay for. And further: What you do use, you couldn’t hand
> code any better.
>
> - Bjarne Stroustrup "Foundations of C++"

As another example, here is some code taken from an audio decoder. This code
uses an iterator chain to do some math on three variables in scope: a `buffer`
slice of data, an array of 12 `coefficients`, and an amount by which to shift
data in `qlp_shift`. We've declared the variables within this example but not
given them any values; while this code doesn't have much meaning outside of its
context, it's still a concise, real-world example of how Rust translates
high-level ideas to low-level code:

```rust,ignore
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

In order to calculate the value of `prediction`, this code iterates through
each of the 12 values in `coefficients`, uses the `zip` method to pair the
coefficient values with the previous 12 values in `buffer`. Then for each pair,
multiply the values together, sum all the results, and shift the bits in the
sum `qlp_shift` bits to the right

Calculations in applications like audio decoders often prioritize performance
most highly. Here, we're creating an iterator, using two adaptors, then
consuming the value. What assembly code would this Rust code compile to? Well,
as of this writing, it compiles down to the same assembly you'd write by hand.
There's no loop at all corresponding to the iteration over the values in
`coefficients`: Rust knows that there are twelve iterations, so it "unrolls"
the loop. All of the coefficients get stored in registers (which means
accessing the values is very fast). There are no bounds checks on the array
access. It's extremely efficient.

Now that you know this, go use iterators and closures without fear! They make
code feel higher-level, but don't impose a runtime performance penalty for
doing so.

## Summary

Closures and iterators are Rust features inspired by functional programming
language ideas. They contribute to Rust's ability to clearly express high-level
ideas. The implementations of closures and iterators, as well as other zero-cost
abstractions in Rust, are such that runtime performance is not affected.

Now that we've improved the expressiveness of our I/O project, let's look at
some more features of `cargo` that would help us get ready to share the project
with the world.
