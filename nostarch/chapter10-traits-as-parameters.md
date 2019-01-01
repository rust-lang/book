Please replace the "Trait Bounds" section on page 182-183 with this text, which should then be followed by the existing "Fixing the largest Function with Trait Bounds" section.

---

### Traits as Parameters

Now that you know how to define traits and implement those traits on types, we
can explore how to use traits to define functions that accept many different
types.

For example, in Listing 10-13, we implemented the `Summary` trait on the types
`NewsArticle` and `Tweet`. We can define a function `notify` that calls the
`summarize` method on its parameter `item`, which is of some type that
implements the `Summary` trait. To do this, we can use the `impl Trait` syntax,
like this:

```
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Instead of a concrete type for the `item` parameter, we specify the `impl`
keyword and the trait that the type passed as an argument must implement. In
the body of `notify`, we can call any methods on `item` that come from the
`Summary` trait, like `summarize`. We can call `notify` and pass in any
instance of `NewsArticle` or `Tweet`. Code that calls the function with any
other type, like a `String` or an `i32`, won’t compile, because those types
don’t implement `Summary`.

#### Trait Bound Syntax

The `impl Trait` syntax works for straightforward cases, but is syntax sugar
for a longer form. The longer syntax is called a *trait bound*, and it looks
like this:

```
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

This is equivalent to the example above, but is a bit more verbose. We place
trait bounds with the declaration of the generic type parameter, after a
colon and inside angle brackets.

The `impl Trait` syntax is convenient and makes for more concise code in
straightforward cases. The trait bound syntax is able to express more
complexity in other cases. For example, to have two parameters that implement
`Summary`, the `impl Trait` syntax would look like this:

```
pub fn notify(item1: impl Summary, item2: impl Summary) {
```

Defining this function using `impl Trait` would be appropriate if `item1` and
`item2` were allowed to have different types (as long as both types implement
`Summary`). If you wanted to force both parameters to have the exact same type,
that is only possible to express with a trait bound:

```
pub fn notify<T: Summary>(item1: T, item2: T) {
```

The generic type `T` specified as the type of the `item1` and `item2`
parameters constrains the function such that the concrete type of the value
passed as an argument for `item1` and `item2` must be the same.

#### Specifying Multiple Trait Bounds with the `+` Syntax

If `notify` needed to use display formatting on `item` as well as the
`summarize` method, then the `notify` definition specifies that `item` must
implement two traits: `Display` and `Summary`. This can be done using the `+`
syntax:

```
pub fn notify(item: impl Summary + Display) {
```

The `+` syntax is also valid with trait bounds on generic types:

```
pub fn notify<T: Summary + Display>(item: T) {
```

With the two trait bounds specified, the body of `notify` can call `summarize`
and use `{}` to format `item`.

#### Clearer Trait Bounds with `where` Clauses

There are downsides to using too many trait bounds. Each generic has its own
trait bounds, so functions with multiple generic type parameters can have lots
of trait bound information between a function’s name and its parameter list,
making the function signature hard to read. For this reason, Rust has alternate
syntax for specifying trait bounds inside a `where` clause after the function
signature. So instead of writing this:

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

we can use a `where` clause, like this:

```
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

This function’s signature is less cluttered in that the function name,
parameter list, and return type are close together, similar to a function
without lots of trait bounds.

### Returning Types that Implement Traits

We can use the `impl Trait` syntax in return position as well, to return
a value of some type that implements a trait:

```
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

The `impl Summary` return type means that the `returns_summarizable` function
returns some type that implements the `Summary` trait, but doesn't specify the
concrete type. In this case, `returns_summarizable` returns a `Tweet`, but the
code calling this function doesn’t know that.

Returning a type that is only specified by the trait it implements is
especially useful in the context of closures and iterators, which we'll be
covering in Chapter 13. Closures and iterators create types that only the
compiler knows or types that are very long to specify. `impl Trait` lets you
concisely specify that a function returns some type that implements the
`Iterator` trait without needing to write out a really long type.

However, using `impl Trait` is only allowed if you have a single type that
you’re returning. For example, this code returning either a `NewsArticle` or a
`Tweet` with the return type specified as `impl Summary` would *not* work:

```
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
```

Returning either a `NewsArticle` or a `Tweet` isn't allowed due to restrictions
around how `impl Trait` is implemented. We'll cover how to write a function
with this behavior in the “Using Trait Objects That Allow for Values of
Different Types” section on page XX [chapter 17].
