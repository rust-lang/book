## Improving our I/O Project

We can improve our implementation of the I/O project in Chapter 12 by using
iterators to make places in the code clearer and more concise. Let’s take a
look at how iterators can improve our implementation of both the `Config::new`
function and the `search` function.

### Removing a `clone` Using an Iterator

In Listing 12-6, we added code that took a slice of `String` values and created
an instance of the `Config` struct by indexing into the slice and cloning the
values so that the `Config` struct could own those values. We’ve reproduced the
implementation of the `Config::new` function as it was at the end of Chapter 12
in Listing 13-24:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

<span class="caption">Listing 13-24: Reproduction of the `Config::new` function
from the end of Chapter 12</span>

<!--Is this why we didn't want to use clone calls, they were inefficient, or
was it that stacking clone calls can become confusing/is bad practice? -->
<!-- Yep, it's for performance reasons /Carol -->

At the time, we said not to worry about the inefficient `clone` calls here
because we would remove them in the future. Well, that time is now!

The reason we needed `clone` here in the first place is that we have a slice
with `String` elements in the parameter `args`, but the `new` function does not
own `args`. In order to be able to return ownership of a `Config` instance, we
need to clone the values that we put in the `query` and `filename` fields of
`Config`, so that the `Config` instance can own its values.

With our new knowledge about iterators, we can change the `new` function to
take ownership of an iterator as its argument instead of borrowing a slice.
We’ll use the iterator functionality instead of the code we had that checks the
length of the slice and indexes into specific locations. This will clear up
what the `Config::new` function is doing since the iterator will take care of
accessing the values.

<!-- use the iterator functionality to what? How will iterating allow us to do
the same thing, can you briefly lay that out? -->
<!-- It's mostly for clarity and using a good abstraction, I've tried fixing
/Carol -->

Once `Config::new` taking ownership of the iterator and not using indexing
operations that borrow, we can move the `String` values from the iterator into
`Config` rather than calling `clone` and making a new allocation.

<!-- below: which file are we in, can you specify here? -->
<!-- done /Carol -->

#### Using the Iterator Returned by `env::args` Directly

In your I/O project’s *src/main.rs*, let’s change the start of the `main`
function from this code that we had at the end of Chapter 12:

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // ...snip...
}
```

To the code in Listing 13-25:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // ...snip...
}
```

<span class="caption">Listing 13-25: Passing the return value of `env::args` to
`Config::new`</span>

<!-- I think, if we're going to be building this up bit by bit, it might be
worth adding listing numbers and file names to each, can you add those? Don't
worry about being accurate with the numbers, we can update them more easily
later -->
<!-- That's nice of you to offer, but since we're maintaining an online version
that we're keeping in sync with each round of edits, we need to keep the
listing numbers making sense as well. We'll just take care of them. /Carol -->

The `env::args` function returns an iterator! Rather than collecting the
iterator values into a vector and then passing a slice to `Config::new`, now
we’re passing ownership of the iterator returned from `env::args` to
`Config::new` directly.

Next, we need to update the definition of `Config::new`. In your I/O project’s
*src/lib.rs*, let’s change the signature of `Config::new` to look like Listing
13-26:

<!-- can you give the filename here too? -->
<!-- done /Carol -->

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Config {
    pub fn new(args: std::env::Args) -> Result<Config, &'static str> {
        // ...snip...
```

<span class="caption">Listing 13-26: Updating the signature of `Config::new` to
expect an iterator</span>

The standard library documentation for the `env::args` function shows that the
type of the iterator it returns is `std::env::Args`. We’ve updated the
signature of the `Config::new` function so that the parameter `args` has the
type `std::env::Args` instead of `&[String]`.

#### Using `Iterator` Trait Methods Instead of Indexing

Next, we’ll fix the body of `Config::new`. The standard library documentation
also mentions that `std::env::Args` implements the `Iterator` trait, so we know
we can call the `next` method on it! Listing 13-27 has updated the code
from Listing 12-23 to use the `next` method:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::env;
#
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }
#
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    	args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query, filename, case_sensitive
        })
    }
}
```

<span class="caption">Listing 13-27: Changing the body of `Config::new` to use
iterator methods</span>

<!-- is this the *full* new lib.rs code? Worth noting for ghosting purposes -->
<!-- No, this is just the `Config::new` function, which I thought would be
clear by saying "Next, we'll fix the body of `Config::new`.", can you elaborate
on why that's not clear enough? I would expect programmers to be able to
understand where a function starts and ends. /Carol -->

Remember that the first value in the return value of `env::args` is the name of
the program. We want to ignore that and get to the next value, so first we call
`next` and do nothing with the return value. Second, we call `next` on the
value we want to put in the `query` field of `Config`. If `next` returns a
`Some`, we use a `match` to extract the value. If it returns `None`, it means
not enough arguments were given and we return early with an `Err` value. We do
the same thing for the `filename` value.

<!-- Hm, if ? would not work anyway, I'm not clear on why we mention, why it's
a shame we cant use it on Option? -->
<!-- We've taken this out, it's something that a portion of the readers might
be wondering and something that Rust might let you do someday, but yeah, it's
probably just distracting to most people /Carol -->

### Making Code Clearer with Iterator Adaptors

The other place in our I/O project we could take advantage of iterators is in
the `search` function, reproduced here in Listing 13-28 as it was at the end of
Chapter 12:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

<span class="caption">Listing 13-28: The implementation of the `search`
function from Chapter 12</span>

We can write this code in a much shorter way by using iterator adaptor methods
instead. This also lets us avoid having a mutable intermediate `results`
vector. The functional programming style prefers to minimize the amount of
mutable state to make code clearer. Removing the mutable state might make it
easier for us to make a future enhancement to make searching happen in
parallel, since we wouldn’t have to manage concurrent access to the `results`
vector. Listing 13-29 shows this change:

<!-- Remind us why we want to avoid the mutable results vector? -->
<!-- done /Carol -->

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

<span class="caption">Listing 13-29: Using iterator adaptor methods in the
implementation of the `search` function</span>

Recall that the purpose of the `search` function is to return all lines in
`contents` that contain the `query`. Similarly to the `filter` example in
Listing 13-19, we can use the `filter` adaptor to keep only the lines that
`line.contains(query)` returns true for. We then collect the matching lines up
into another vector with `collect`. Much simpler! Feel free to make the same
change to use iterator methods in the `search_case_insensitive` function as
well.

<!-- what is that, here, only lines that contain a matching string? A bit more
context would help out, we probably can't rely on readers remembering all the
details I'm afraid -->
<!-- done /Carol -->

The next logical question is which style you should choose in your own code:
the original implementation in Listing 13-28, or the version using iterators in
Listing 13-29. Most Rust programmers prefer to use the iterator style. It’s a
bit tougher to get the hang of at first, but once you get a feel for the
various iterator adaptors and what they do, iterators can be easier to
understand. Instead of fiddling with the various bits of looping and building
new vectors, the code focuses on the high-level objective of the loop. This
abstracts away some of the commonplace code so that it’s easier to see the
concepts that are unique to this code, like the filtering condition each
element in the iterator must pass.

But are the two implementations truly equivalent? The intuitive assumption
might be that the more low-level loop will be faster. Let’s talk about
performance.
