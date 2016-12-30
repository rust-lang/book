## Accepting Command Line Arguments

Our first task is to have `greprs` accept its two command line arguments. There
are some existing libraries on crates.io that can help us do this, but since
we're learning, we'll implement this ourselves.

We'll need to call a function provided in Rust's standard library:
`std::env::args`. This function returns an *iterator* of the command line
arguments that were given to our program. We haven't discussed iterators yet;
Chapter 13 will cover them fully. For our purposes, though, we don't need to
understand much about how they work in order to use them. We only need to
understand two things:

1. Iterators produce a series of values.
2. We can call the `collect` function on an iterator to turn it into a vector
   containing all of the elements the iterator produces.

Let's give it a try as shown in Listing 12-1:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

<figcaption>

Listing 12-1: Collect the command line arguments into a vector and print them out

</figcaption>
</figure>

<!-- Will add wingdings in libreoffice /Carol -->

First, we have a `use` statement to bring the `std::env` module into scope.
When using a function that's nested in more than one level of module, like
`std::env::args` is, it's conventional to use `use` to bring the parent module
into scope, rather than the function itself. `env::args` is less ambiguous than
a lone `args`. Also, if we end up using more than one function in `std::env`,
we only need a single `use`.

On the first line of `main`, we call `env::args`, and immediately use `collect`
to create a vector out of it. We're also explicitly annotating the type of
`args` here: `collect` can be used to create many kinds of collections. Rust
won't be able to infer what kind of type we want, so the annotation is
required. We very rarely need to annotate types in Rust, but `collect` is one
function where you often need to.

Finally, we print out the vector with the debug formatter, `:?`. Let's try
running our code with no arguments, and then with two arguments:

```text
$ cargo run
["target/debug/greprs"]

$ cargo run needle haystack
...snip...
["target/debug/greprs", "needle", "haystack"]
```

You'll notice one interesting thing: the name of the binary is the first
argument. The reasons for this are out of the scope of this chapter, but it's
something we'll have to remember to account for.

Now that we have a way to access all of the arguments, let's find the ones we
care about and save them in variables as shown in Listing 12-2:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let search = &args[1];
    let filename = &args[2];

    println!("Searching for {}", search);
    println!("In file {}", filename);
}
```

<figcaption>

Listing 12-2: Create variables to hold the search argument and filename argument

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Remember, the program's name is the first argument, so we don't need `args[0]`.
We've decided that the first argument will be the string we're searching for,
so we put a reference to the first argument in the variable `search`. The
second argument will be the filename, so we put a reference to the second
argument in the variable `filename`. Let's try running this program again:

```text
$ cargo run test sample.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe test sample.txt`
Searching for test
In file sample.txt
```

Great! There's one problem, though. Let's try giving it no arguments:

```text
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe`
thread 'main' panicked at 'index out of bounds: the len is 1
but the index is 1', ../src/libcollections\vec.rs:1307
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Because our vector only has one element, the program's name, but we tried to
access the second element, our program panics with a message about the
out-of-bound access. While this error message is _accurate_, it's not
meaningful to users of our program at all. We could fix this problem right now,
but let's push forward: we'll improve this situation before we're finished.
