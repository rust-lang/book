## Improving our I/O Project

In our I/O project implementing `grep` in the last chapter, there are some
places where the code could be made clearer and more concise with iterators.
Let's take a look at how iterators can improve our implementation of the
`Config::new` function and the `search` function.

### Removing a `clone` by Using an Iterator

Back in Listing 12-8, we had this code that took a slice of `String` values and
created an instance of the `Config` struct by checking for the right number of
arguments, indexing into the slice, and cloning the values so that the `Config`
struct could own those values:

```rust,ignore
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query, filename
        })
    }
}
```

At the time, we said not to worry about the `clone` calls here, and that we
could remove them in the future. Well, that time is now! So, why do we need
`clone` here? The issue is that we have a slice with `String` elements in the
parameter `args`, and the `new` function does not own `args`. In order to be
able to return ownership of a `Config` instance, we need to clone the values
that we put in the `query` and `filename` fields of `Config`, so that the
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
# struct Config {
#     query: String,
#     filename: String,
# }
#
impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    	args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config {
            query, filename
        })
    }
}
```

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Remember that the first value in the return value of `env::args` is the name of
the program. We want to ignore that, so first we'll call `next` and not do
anything with the return value. The second time we call `next` should be the
value we want to put in the `query` field of `Config`. We use a `match` to
extract the value if `next` returns a `Some`, and we return early with an `Err`
value if there weren't enough arguments (which would cause this call to `next`
to return `None`).

We do the same thing for the `filename` value. It's slightly unfortunate that
the `match` expressions for `query` and `filename` are so similar. It would be
nice if we could use `?` on the `Option` returned from `next`, but `?` only
works with `Result` values currently. Even if we could use `?` on `Option` like
we can on `Result`, the value we would get would be borrowed, and we want to
move the `String` from the iterator into `Config`.

### Making Code Clearer with Iterator Adaptors

The other bit of code where we could take advantage of iterators was in the
`search` function as implemented in Listing 12-15:

<!-- We hadn't had a listing number for this code sample when we submitted
chapter 12; we'll fix the listing numbers in that chapter after you've
reviewed it. /Carol -->

```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

We can write this code in a much shorter way, and avoiding needing a mutable
intermediate `results` vector, by using iterator adaptor methods like this
instead:

```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

Here, we use the `filter` adaptor to only keep the lines that
`line.contains(query)` returns true for. We then collect them up into another
vector with `collect`. Much simpler!

We can use the same technique in the `search_case_insensitive` function that we
defined in Listing 12-16 as follows:

<!-- Similarly, the code snippet that will be 12-16 didn't have a listing
number when we sent you chapter 12, we will fix it. /Carol -->

```rust
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| {
            line.to_lowercase().contains(&query)
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
