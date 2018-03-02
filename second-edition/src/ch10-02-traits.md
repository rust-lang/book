## Traits: Defining Shared Behavior

A *trait* tells the Rust compiler about functionality a particular type has and
can share with other types. We can use traits to define shared behavior in an
abstract way. We can use trait bounds to state that a generic can be any type
that has certain behavior.

> Note: *Traits* are similar to a feature often called ‘interfaces’ in other
> languages, though with some differences.

### Defining a Trait

A type’s behavior consists of the methods we can call on that type. Different
types share the same behavior if we can call the same methods on all of those
types. Trait definitions are a way to group method signatures together in order
to define a set of behaviors necessary to accomplish some purpose.

For example, let’s say we have multiple structs that hold various kinds and
amounts of text: a `NewsArticle` struct that holds a news story filed in a
particular location, and a `Tweet` that can have at most 280 characters along
with metadata that indicates whether it was a new tweet, a retweet, or a reply
to another tweet.

We want to make a media aggregator library that can display summaries of data
that might be stored in a `NewsArticle` or `Tweet` instance. To do this, we
need a summary from each type, and we need to be able to ask for that summary
by calling a `summarize` method on an instance. Listing 10-12 shows the
definition of a `Summary` trait that expresses this behavior:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

<span class="caption">Listing 10-12: Definition of a `Summary` trait that
consists of the behavior provided by a `summarize` method</span>

Here, we declare a trait with the `trait` keyword, and then the trait’s name,
which is `Summary` in this case. Inside the curly brackets we declare the
method signatures that describe the behaviors of the types that implement this
trait, which in this case is `fn summarize(&self) -> String`.

After the method signature, instead of providing an implementation within curly
brackets, we put a semicolon. Each type implementing this trait must provide
its own custom behavior for the body of the method, but the compiler will
enforce that any type that has the `Summary` trait will have the method
`summarize` defined with this signature exactly.

A trait can have multiple methods in its body, with the method signatures
listed one per line and each line ending in a semicolon.

### Implementing a Trait on a Type

Now that we’ve defined our desired behavior using the `Summary` trait, we can
implement it on the types in our media aggregator. Listing 10-13 shows an
implementation of the `Summary` trait on the `NewsArticle` struct that uses the
headline, the author, and the location to create the return value of
`summarize`. For the `Tweet` struct, we define `summarize` as the username
followed by the whole text of the tweet, assuming that tweet content is already
limited to 280 characters.

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Summary {
#     fn summarize(&self) -> String;
# }
#
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

<span class="caption">Listing 10-13: Implementing the `Summary` trait on the
`NewsArticle` and `Tweet` types</span>

Implementing a trait on a type is similar to implementing regular methods. The
difference is that after `impl`, we put the trait name that we want to
implement, then use the `for` keyword, then specify the name of the type we
want to implement the trait for. Within the `impl` block, we put the method
signatures that the trait definition has defined, but instead of adding a
semicolon after each signature, we put curly brackets and fill in the method
body with the specific behavior that we want the methods of the trait to have
for the particular type.

After implementing the trait, we can call the methods on instances of
`NewsArticle` and `Tweet` in the same manner that we call regular methods, like
this:

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

This prints: `1 new tweet: horse_ebooks: of course, as you probably already
know, people`.

Note that because we defined the `Summary` trait and the `NewsArticle` and
`Tweet` types all in the same *lib.rs* in Listing 10-13, they’re all in the
same scope. If this *lib.rs* is for a crate we’ve called `aggregator`, and
someone else wants to use our crate’s functionality to implement the `Summary`
trait on a struct defined within their library’s scope, they would need to
import the trait into their scope first. They would do so by specifying `use
aggregator::Summary;` which then enables them to implement `Summary` for their
type. `Summary` would also need to be a public trait for another crate to
implement it, which it is because we put the `pub` keyword before `trait` in
Listing 10-12.

One restriction to note with trait implementations is that we can implement a
trait on a type only if either the trait or the type is local to your crate.
For example, we can implement standard library traits like `Display` on a
custom type like `Tweet` as part of our `aggregator` crate functionality
because the type `Tweet` is local to our `aggregator` crate. We can also
implement `Summary` on `Vec<T>` in our `aggregator` crate, because the
trait `Summary` is local to our `aggregator` crate.

What we can’t do is implement external traits on external types. We can’t
implement the `Display` trait on `Vec<T>` within our `aggregator` crate, for
example, because both `Display` and `Vec<T>` are defined in the standard
library and aren’t local to our `aggregator` crate. This restriction is part of
a property of programs called *coherence*, and more specifically the *orphan
rule*, so named because the parent type is not present. This rule ensures that
other people’s code can’t break your code and vice versa. Without it, two
crates could implement the same trait for the same type, and Rust wouldn’t know
which implementation to use.

### Default Implementations

Sometimes it’s useful to have default behavior for some or all of the methods
in a trait, instead of requiring implementations for all methods on every type.
Then, as we implement the trait on a particular type, we can choose to keep or
override each method’s default behavior.

Listing 10-14 shows how to specify a default string for the `summarize` method
of the `Summary` trait instead of only defining the method signature like we
did in Listing 10-12:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

<span class="caption">Listing 10-14: Definition of a `Summary` trait with a
default implementation of the `summarize` method</span>

To use a default implementation to summarize instances of `NewsArticle` instead
of defining a custom implementation, we specify an empty `impl` block with
`impl Summary for NewsArticle {}`.

Even though we’re no longer choosing to define the `summarize` method on
`NewsArticle` directly, we’ve provided a default implementation and specified
that `NewsArticle` implements the `Summary` trait, so we can still call the
`summarize` method on an instance of `NewsArticle`, like this:

```rust,ignore
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
};

println!("New article available! {}", article.summarize());
```

This code prints `New article available! (Read more...)`.

Creating a default implementation for `summarize` does not require us to change
anything about the implementation of `Summary` on `Tweet` in Listing 10-13
because the syntax for overriding a default implementation is exactly the same
as the syntax for implementing a trait method that doesn’t have a default
implementation.

Default implementations can call other methods in the same trait, even if those
other methods don’t have a default implementation. In this way, a trait can
provide a lot of useful functionality and only require implementors to specify
a small part of it. For example, we could define the `Summary` trait to have a
`summarize_author` method whose implementation is required, then a `summarize`
method that has a default implementation that calls the `summarize_author`
method:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

To use this version of `Summary`, we only need to define `summarize_author`
when we implement the trait on a type:

```rust,ignore
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

Once we define `summarize_author`, we can call `summarize` on instances of the
`Tweet` struct, and the default implementation of `summarize` will call the
definition of `summarize_author` that we’ve provided. Because we’ve implemented
`summarize_author`, the `Summary` trait has given us the behavior of the
`summarize` method without requiring us to write any more code.

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

This prints `1 new tweet: (Read more from @horse_ebooks...)`.

Note that it is not possible to call the default implementation from an
overriding implementation of that same method.

### Trait Bounds

Now that you’ve learned how to define traits and implement those traits on
types, we can cover how to use traits with generic type parameters. We can use
*trait bounds* to constrain generic types to ensure the type will be limited to
those that implement a particular trait and behavior.

For example, in Listing 10-13, we implemented the `Summary` trait on the types
`NewsArticle` and `Tweet`. We can define a function `notify` that calls the
`summarize` method on its parameter `item`, which is of the generic type `T`.
To be able to call `summarize` on `item` without getting an error that the
generic type `T` doesn’t implement the method `summarize`, we can use trait
bounds on `T` to specify that `item` must be of a type that implements the
`Summary` trait:

```rust,ignore
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

We place trait bounds with the declaration of the generic type parameter, after
a colon and inside the angle brackets. Because of the trait bound on `T`, we
can call `notify` and pass in any instance of `NewsArticle` or `Tweet`. Code
that calls the function with any other type, like a `String` or an `i32`, won’t
compile, because those don’t implement `Summary`.

We can specify multiple trait bounds on a generic type using the `+` syntax.
For example, to use display formatting on the type `T` in a function as well as
the `summarize` method, we can use `T: Summary + Display` to say `T` can be any
type that implements both `Summary` and `Display`.

There are downsides to using too many trait bounds, however. Each generic has
its own trait bounds, so functions with multiple generic type parameters can
have lots of trait bound information between a function’s name and its
parameter list, making the function signature hard to read. For this reason,
Rust has alternate syntax for specifying trait bounds inside a `where` clause
after the function signature. So instead of writing this:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

We can use a `where` clause, like this:

```rust,ignore
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

This function’s signature is less cluttered in that the function name,
parameter list, and return type are close together, similar to a function
without lots of trait bounds.

### Fixing the `largest` Function with Trait Bounds

Now that we’ve covered how to specify the behavior you want to use using the
generic type parameter’s bounds, we can return to Listing 10-5 to fix the
definition of the `largest` function that uses a generic type parameter! Last
time we were trying out that code, we were getting this error:

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:12
  |
5 |         if item > largest {
  |            ^^^^^^^^^^^^^^
  |
  = note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

In the body of `largest` we wanted to compare two values of type `T` using the
greater-than operator. Because that operator is defined as a default method on
the standard library trait `std::cmp::PartialOrd`, we need to specify
`PartialOrd` in the trait bounds for `T` so that the `largest` function can
work on slices of any type that can be compared. We don’t need to bring
`PartialOrd` into scope because it’s in the prelude. Change the signature of
`largest` to look like this:

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

This time, when we compile the code, we’ll get a different set of errors:

```text
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       help: consider using a reference instead: `&list[0]`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:4:9
  |
4 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
```

The key line to note about this error is `cannot move out of type [T], a
non-copy slice`. With our non-generic versions of the `largest` function, we
were only trying to find the largest `i32` or `char`. As we discussed in the
“Stack-Only Data: Copy” section in Chapter 4, types like `i32` and `char` that
have a known size can be stored on the stack, so they implement the `Copy`
trait. But when we made the `largest` function generic, it became possible that
the `list` parameter could have types in it that don’t implement the `Copy`
trait, which would mean we wouldn’t be able to move the value out of `list[0]`
and into the `largest` variable, resulting in this error.

To call this code with only those types that implement the trait `Copy`, we can
add `Copy` to the trait bounds of `T`! Listing 10-15 shows the complete code of
a generic `largest` function that will compile as long as the types of the
values in the slice that we pass into the function implement both the
`PartialOrd` and `Copy` traits, like `i32` and `char` do:

<span class="filename">Filename: src/main.rs</span>

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

<span class="caption">Listing 10-15: A working definition of the `largest`
function that works on any generic type that implements the `PartialOrd` and
`Copy` traits</span>

If we don’t want to restrict our `largest` function to the types that implement
the `Copy` trait, we could specify that `T` has the trait bound `Clone` instead
of `Copy` and clone each value in the slice when we want the `largest` function
to have ownership. Using the `clone` function means we’re potentially making
more heap allocations, though, in the case of types that own heap data like
`String`, and heap allocations can be slow if we’re working with large amounts
of data.

Another way we could implement `largest` is for the function to return a
reference to a `T` value in the slice. If we change the return type to `&T`
instead of `T`, thereby changing the body of the function to return a
reference, we wouldn’t need either the `Clone` or `Copy` trait bounds and we
could avoid heap allocations altogether. Try implementing these alternate
solutions on your own!

### Using Trait Bounds to Conditionally Implement Methods

By using a trait bound with an `impl` block that uses generic type parameters,
we can conditionally implement methods only for types that implement the
specified traits. For example, the type `Pair<T>` in Listing 10-16 always
implements the `new` method, but `Pair<T>` only implements the `cmp_display`
method if its inner type `T` implements the `PartialOrd` trait that enables
comparison and the `Display` trait that enables printing:

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

<span class="caption">Listing 10-16: Conditionally implement methods on a
generic type depending on trait bounds</span>

We can also conditionally implement a trait for any type that implements a
trait. Implementations of a trait on any type that satisfies the trait bounds
are called *blanket implementations*, and are extensively used in the Rust
standard library. For example, the standard library implements the `ToString`
trait on any type that implements the `Display` trait. The `impl` block in the
standard library looks similar to this code:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Because the standard library has this blanket implementation, we can call the
`to_string` method defined by the `ToString` trait on any type that implements
the `Display` trait. For example, we can turn integers into their corresponding
`String` values like this because integers implement `Display`:

```rust
let s = 3.to_string();
```

Blanket implementations appear in the documentation for the trait in the
“Implementors” section.

Traits and trait bounds let us write code that uses generic type parameters to
reduce duplication, but also specify to the compiler that we want the generic
type to have particular behavior. The compiler can then use the trait bound
information to check that all the concrete types used with our code provide the
right behavior. Unlike in dynamically typed languages, where we’d get an error
at runtime if we tried to call a method on a type that the type didn’t
implement, Rust moves these errors to compile time so that we’re forced to fix
the problems before our code is even able to run. Additionally, we don’t have
to write code that checks for behavior at runtime because we’ve already checked
at compile time, which improves performance without having to give up the
flexibility of generics.

There’s another kind of generic that we’ve already been using called
*lifetimes*. Rather than ensuring that a type has the behavior we want,
lifetimes ensure that references are valid as long as we need them to be. Let’s
learn how lifetimes do that.
