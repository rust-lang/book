## Write to `stderr` Instead of `stdout`

Right now, we're writing all of our output to the terminal with `println!`.
This works, but most terminals provide two kinds of output: "standard out" is
used for most information, but "standard error" is used for error messages. This
makes it easier to do things like "Print error messages to my terminal, but
write other output to a file."

We can see that our program is only capable of printing to `stdout` by
redirecting it to a file using `>` on the command line, and running our program
without any arguments, which causes an error:

```text
$ cargo run > output.txt
```

The `>` syntax tells the shell to write the contents of standard out to
*output.txt* instead of the screen. However, if we open *output.txt* after
running we'll see our error message:

```text
Problem parsing arguments: not enough arguments
```

We'd like this to be printed to the screen instead, and only have the output
from a successful run end up in the file if we run our program this way. Let's
change how error messages are printed as shown in Listing 12-17:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::Config;

fn main() {
    let mut stderr = std::io::stderr();
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        writeln!(
            &mut stderr,
            "Problem parsing arguments: {}",
            err
        ).expect("Could not write to stderr");

        process::exit(1);
    });

    if let Err(e) = greprs::run(config) {

        writeln!(
            &mut stderr,
            "Application error: {}",
            e
        ).expect("Could not write to stderr");

        process::exit(1);
    }
}
```

<figcaption>

Listing 12-17: Writing error messages to `stderr` instead of `stdout`

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Rust does not have a convenient function like `println!` for writing to
standard error. Instead, we use the `writeln!` macro, which is sort of like
`println!`, but it takes an extra argument. The first thing we pass to it is
what to write to. We can acquire a handle to standard error through the
`std::io::stderr` function. We give a mutable reference to `stderr` to
`writeln!`; we need it to be mutable so we can write to it! The second and
third arguments to `writeln!` are like the first and second arguments to
`println!`: a format string and any variables we're interpolating.

Let's try running the program again in the same way, without any arguments and
redirecting `stdout` with `>`:

```text
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Now we see our error on the screen, but `output.txt` contains nothing. If we
try it again with arguments that work:

```text
$ cargo run to poem.txt > output.txt
```

We'll see no output to our terminal, but `output.txt` will contain
our results:

<span class="filename">Filename: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

## Summary

In this chapter, we've covered how to do common I/O operations in a Rust
context. By using command line arguments, files, environment variables, and the
ability to write to `stderr`, you're now prepared to write command line
applications. By using the concepts from previous chapters, your code will be
well-organized, be able to store data effectively in the appropriate data
structures, handle errors nicely, and be well tested. We also saw a real-world
scenario where lifetime annotations are needed to ensure references are
always valid.

Next, let's explore how to make use of some features of Rust that were
influenced by functional languages: closures and iterators.
