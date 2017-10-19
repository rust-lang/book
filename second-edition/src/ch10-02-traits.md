## Traits: Defining Shared Behavior

Traits allow us to use another kind of abstraction: they let us abstract over
behavior that types can have in common. A *trait* tells the Rust compiler about
functionality a particular type has and might share with other types. In
situations where we use generic type parameters, we can use *trait bounds* to
specify, at compile time, that the generic type may be any type that implements
a trait and therefore has the behavior we want to use in that situation.

> Note: *Traits* are similar to a feature often called ‘interfaces’ in other
> languages, though with some differences.

### Defining a Trait

The behavior of a type consists of the methods we can call on that type.
Different types share the same behavior if we can call the same methods on all
of those types. Trait definitions are a way to group method signatures together
in order to define a set of behaviors necessary to accomplish some purpose.

For example, say we have multiple structs that hold various kinds and amounts
of text: a `NewsArticle` struct that holds a news story filed in a particular
place in the world, and a `Tweet` that can have at most 140 characters in its
content along with metadata like whether it was a retweet or a reply to another
tweet.

We want to make a media aggregator library that can display summaries of data
that might be stored in a `NewsArticle` or `Tweet` instance. The behavior we
need each struct to have is that it’s able to be summarized, and that we can
ask for that summary by calling a `summary` method on an instance. Listing
10-12 shows the definition of a `Summarizable` trait that expresses this
concept:

<span class="filename">Filename: lib.rs</span>

```rust
pub trait Summarizable {
    fn summary(&self) -> String;
}
```

<span class="caption">Listing 10-12: Definition of a `Summarizable` trait that
consists of the behavior provided by a `summary` method</span>

We declare a trait with the `trait` keyword, then the trait’s name, in this
case `Summarizable`. Inside curly brackets we declare the method signatures
that describe the behaviors that types that implement this trait will need to
have, in this case `fn summary(&self) -> String`. After the method signature,
instead of providing an implementation within curly brackets, we put a
semicolon. Each type that implements this trait must then provide its own
custom behavior for the body of the method, but the compiler will enforce that
any type that has the `Summarizable` trait will have the method `summary`
defined for it with this signature exactly.

A trait can have multiple methods in its body, with the method signatures
listed one per line and each line ending in a semicolon.

### Implementing a Trait on a Type

Now that we’ve defined the `Summarizable` trait, we can implement it on the
types in our media aggregator that we want to have this behavior. Listing 10-13
shows an implementation of the `Summarizable` trait on the `NewsArticle` struct
that uses the headline, the author, and the location to create the return value
of `summary`. For the `Tweet` struct, we’ve chosen to define `summary` as the
username followed by the whole text of the tweet, assuming that tweet content
is already limited to 140 characters.

<span class="filename">Filename: lib.rs</span>

```rust
# pub trait Summarizable {
#     fn summary(&self) -> String;
# }
#
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

<span class="caption">Listing 10-13: Implementing the `Summarizable` trait on
the `NewsArticle` and `Tweet` types</span>

Implementing a trait on a type is similar to implementing methods that aren’t
related to a trait. The difference is after `impl`, we put the trait name that
we want to implement, then say `for` and the name of the type that we want to
implement the trait for. Within the `impl` block, we put the method signatures
that the trait definition has defined, but instead of putting a semicolon after
each signature, we put curly brackets and fill in the method body with the
specific behavior that we want the methods of the trait to have for the
particular type.

Once we’ve implemented the trait, we can call the methods on instances of
`NewsArticle` and `Tweet` in the same manner that we call methods that aren’t
part of a trait:

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summary());
```

This will print `1 new tweet: horse_ebooks: of course, as you probably already
know, people`.

Note that because we’ve defined the `Summarizable` trait and the `NewsArticle`
and `Tweet` types all in the same `lib.rs` in Listing 10-13, they’re all in the
same scope. If this `lib.rs` is for a crate we’ve called `aggregator`, and
someone else wants to use our crate’s functionality plus implement the
`Summarizable` trait on their `WeatherForecast` struct, their code would need
to import the `Summarizable` trait into their scope first before they could
implement it, like in Listing 10-14:

<span class="filename">Filename: lib.rs</span>

```rust,ignore
extern crate aggregator;

use aggregator::Summarizable;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }
}
```

<span class="caption">Listing 10-14: Bringing the `Summarizable` trait from our
`aggregator` crate into scope in another crate</span>

This code also assumes `Summarizable` is a public trait, which it is because we
put the `pub` keyword before `trait` in Listing 10-12.

One restriction to note with trait implementations: we may implement a trait on
a type as long as either the trait or the type are local to our crate. In other
words, we aren’t allowed to implement external traits on external types. We
can’t implement the `Display` trait on `Vec`, for example, since both `Display`
and `Vec` are defined in the standard library. We are allowed to implement
standard library traits like `Display` on a custom type like `Tweet` as part of
our `aggregator` crate functionality. We could also implement `Summarizable` on
`Vec` in our `aggregator` crate, since we’ve defined `Summarizable` there. This
restriction is part of what’s called the *orphan rule*, which you can look up
if you’re interested in type theory. Briefly, it’s called the orphan rule
because the parent type is not present. Without this rule, two crates could
implement the same trait for the same type, and the two implementations would
conflict: Rust wouldn’t know which implementation to use. Because Rust enforces
the orphan rule, other people’s code can’t break your code and vice versa.

### Default Implementations

Sometimes it’s useful to have default behavior for some or all of the methods
in a trait, instead of making every implementation on every type define custom
behavior. When we implement the trait on a particular type, we can choose to
keep or override each method’s default behavior.

Listing 10-15 shows how we could have chosen to specify a default string for
the `summary` method of the `Summarize` trait instead of choosing to only
define the method signature like we did in Listing 10-12:

<span class="filename">Filename: lib.rs</span>

```rust
pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}
```

<span class="caption">Listing 10-15: Definition of a `Summarizable` trait with
a default implementation of the `summary` method</span>

If we wanted to use this default implementation to summarize instances of
`NewsArticle` instead of defining a custom implementation like we did in
Listing 10-13, we would specify an empty `impl` block:

```rust,ignore
impl Summarizable for NewsArticle {}
```

Even though we’re no longer choosing to define the `summary` method on
`NewsArticle` directly, since the `summary` method has a default implementation
and we specified that `NewsArticle` implements the `Summarizable` trait, we can
still call the `summary` method on an instance of `NewsArticle`:

```rust,ignore
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
};

println!("New article available! {}", article.summary());
```

This code prints `New article available! (Read more...)`.

Changing the `Summarizable` trait to have a default implementation for
`summary` does not require us to change anything about the implementations of
`Summarizable` on `Tweet` in Listing 10-13 or `WeatherForecast` in Listing
10-14: the syntax for overriding a default implementation is exactly the same
as the syntax for implementing a trait method that doesn’t have a default
implementation.

Default implementations are allowed to call the other methods in the same
trait, even if those other methods don’t have a default implementation. In this
way, a trait can provide a lot of useful functionality and only require
implementors to specify a small part of it. We could choose to have the
`Summarizable` trait also have an `author_summary` method whose implementation
is required, then a `summary` method that has a default implementation that
calls the `author_summary` method:

```rust
pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}
```

In order to use this version of `Summarizable`, we’re only required to define
`author_summary` when we implement the trait on a type:

```rust,ignore
impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}
```

Once we define `author_summary`, we can call `summary` on instances of the
`Tweet` struct, and the default implementation of `summary` will call the
definition of `author_summary` that we’ve provided.

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summary());
```

This will print `1 new tweet: (Read more from @horse_ebooks...)`.

Note that it is not possible to call the default implementation from an
overriding implementation.

### Trait Bounds

Now that we’ve defined traits and implemented those traits on types, we can use
traits with generic type parameters. We can constrain generic types so that
rather than being any type, the compiler will ensure that the type will be
limited to those types that implement a particular trait and thus have the
behavior that we need the types to have. This is called specifying *trait
bounds* on a generic type.

For example, in Listing 10-13, we implemented the `Summarizable` trait on the
types `NewsArticle` and `Tweet`. We can define a function `notify` that calls
the `summary` method on its parameter `item`, which is of the generic type `T`.
To be able to call `summary` on `item` without getting an error, we can use
trait bounds on `T` to specify that `item` must be of a type that implements
the `Summarizable` trait:

```rust,ignore
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}
```

Trait bounds go with the declaration of the generic type parameter, after a
colon and within the angle brackets. Because of the trait bound on `T`, we can
call `notify` and pass in any instance of `NewsArticle` or `Tweet`. The
external code from Listing 10-14 that’s using our `aggregator` crate can call
our `notify` function and pass in an instance of `WeatherForecast`, since
`Summarizable` is implemented for `WeatherForecast` as well. Code that calls
`notify` with any other type, like a `String` or an `i32`, won’t compile, since
those types do not implement `Summarizable`.

We can specify multiple trait bounds on a generic type by using `+`. If we
needed to be able to use display formatting on the type `T` in a function as
well as the `summary` method, we can use the trait bounds `T: Summarizable +
Display`. This means `T` can be any type that implements both `Summarizable`
and `Display`.

For functions that have multiple generic type parameters, each generic has its
own trait bounds. Specifying lots of trait bound information in the angle
brackets between a function’s name and its parameter list can get hard to read,
so there’s an alternate syntax for specifying trait bounds that lets us move
them to a `where` clause after the function signature. So instead of:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

We can write this instead with a `where` clause:

```rust,ignore
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

This is less cluttered and makes this function’s signature look more similar to
a function without lots of trait bounds, in that the function name, parameter
list, and return type are close together.

### Fixing the `largest` Function with Trait Bounds

So any time you want to use behavior defined by a trait on a generic, you need
to specify that trait in the generic type parameter’s type bounds. We can now
fix the definition of the `largest` function that uses a generic type parameter
from Listing 10-5! When we set that code aside, we were getting this error:

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
  |
5 |         if item > largest {
  |            ^^^^
  |
note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

In the body of `largest` we wanted to be able to compare two values of type `T`
using the greater-than operator. That operator is defined as a default method
on the standard library trait `std::cmp::PartialOrd`. So in order to be able to
use the greater-than operator, we need to specify `PartialOrd` in the trait
bounds for `T` so that the `largest` function will work on slices of any type
that can be compared. We don’t need to bring `PartialOrd` into scope because
it’s in the prelude.

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

If we try to compile this, we’ll get different errors:

```text
error[E0508]: cannot move out of type `[T]`, a non-copy array
 --> src/main.rs:4:23
  |
4 |     let mut largest = list[0];
  |         -----------   ^^^^^^^ cannot move out of here
  |         |
  |         hint: to prevent move, use `ref largest` or `ref mut largest`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:6:9
  |
6 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
```

The key to this error is `cannot move out of type [T], a non-copy array`.
With our non-generic versions of the `largest` function, we were only trying to
find the largest `i32` or `char`. As we discussed in Chapter 4, types like
`i32` and `char` that have a known size can be stored on the stack, so they
implement the `Copy` trait. When we changed the `largest` function to be
generic, it’s now possible that the `list` parameter could have types in it
that don’t implement the `Copy` trait, which means we wouldn’t be able to move
the value out of `list[0]` and into the `largest` variable.

If we only want to be able to call this code with types that are `Copy`, we can
add `Copy` to the trait bounds of `T`! Listing 10-16 shows the complete code of
a generic `largest` function that will compile as long as the types of the
values in the slice that we pass into `largest` implement both the `PartialOrd`
and `Copy` traits, like `i32` and `char`:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::cmp::PartialOrd;

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

<span class="caption">Listing 10-16: A working definition of the `largest`
function that works on any generic type that implements the `PartialOrd` and
`Copy` traits</span>

If we don’t want to restrict our `largest` function to only types that
implement the `Copy` trait, we could specify that `T` has the trait bound
`Clone` instead of `Copy` and clone each value in the slice when we want the
`largest` function to have ownership. Using the `clone` function means we’re
potentially making more heap allocations, though, and heap allocations can be
slow if we’re working with large amounts of data. Another way we could
implement `largest` is for the function to return a reference to a `T` value in
the slice. If we change the return type to be `&T` instead of `T` and change
the body of the function to return a reference, we wouldn’t need either the
`Clone` or `Copy` trait bounds and we wouldn’t be doing any heap allocations.
Try implementing these alternate solutions on your own!

### Using Trait Bounds to Conditionally Implement Methods

By using a trait bound with an `impl` block that uses generic type parameters,
we can conditionally implement methods only for types that implement the
specified traits. For example, the type `Pair<T>` in listing 10-17 always
implements the `new` method, but `Pair<T>` only implements the `cmp_display` if
its inner type `T` implements the `PartialOrd` trait that enables comparison
and the `Display` trait that enables printing:

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

<span class="caption">Listing 10-17: Conditionally implement methods on a
generic type depending on trait bounds</span>

We can also conditionally implement a trait for any type that implements a
trait. Implementations of a trait on any type that satisfies the trait bounds
are called *blanket implementations*, and are extensively used in the Rust
standard library. For example, the standard library implements the `ToString`
trait on any type that implements the `Display` trait. This `impl` block looks
similar to this code:

```rust,ignore
impl<T: Display> ToString for T {
    // ...snip...
}
```

Because the standard library has this blanket implementation, we can call the
`to_string` method defined by the `ToString` type on any type that implements
the `Display` trait. For example, we can turn integers into their corresponding
`String` values like this since integers implement `Display`:

```rust
let s = 3.to_string();
```

Blanket implementations appear in the documentation for the trait in the
“Implementors” section.

Traits and trait bounds let us write code that uses generic type parameters in
order to reduce duplication, but still specify to the compiler exactly what
behavior our code needs the generic type to have. Because we’ve given the trait
bound information to the compiler, it can check that all the concrete types
used with our code provide the right behavior. In dynamically typed languages,
if we tried to call a method on a type that the type didn’t implement, we’d get
an error at runtime. Rust moves these errors to compile time so that we’re
forced to fix the problems before our code is even able to run. Additionally,
we don’t have to write code that checks for behavior at runtime since we’ve
already checked at compile time, which improves performance compared to other
languages without having to give up the flexibility of generics.

There’s another kind of generics that we’ve been using without even realizing
it called *lifetimes*. Rather than helping us ensure that a type has the
behavior we need it to have, lifetimes help us ensure that references are valid
as long as we need them to be. Let’s learn how lifetimes do that.
