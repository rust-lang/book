## Write to `stderr` Instead of `stdout`

Right now, we're writing all of our output to the terminal with `println!`.
Most terminals provide two kinds of output: "standard out" for general
information, and "standard error" for error messages. This distinction is the
behavior that's expected of command line programs: it enables users to choose
to direct a program's successful output to a file but still print error
messages to the screen, for example. `println!` is only capable of printing to
standard out, though, so we have to use something else in order to print to
standard error.

We can verify that, the way we've written `greprs` so far, everything is being
written to standard out, including error messages that should be written to
standard error instead. We'll do that by intentionally causing an error, the
one that happens when we run the program without any arguments. We're going to
redirect standard output to a file, but not standard error. The way command
line programs are expected to work is that, because the output is an error
message, it should be shown on the screen rather than being redirected to the
file. Let's see that our program is not currently meeting this expectation by
using `>` and specifying a filename, *output.txt*, that we want to redirect
standard out to:

```text
$ cargo run > output.txt
```

<!-- why do we get an error here? Was that intentional? Does that mean it can't
print stdout to a file? -->
<!-- Yes, we're intentionally causing an error here to show that errors are
currently going to the wrong place. It's showing that `println!` only prints
to standard out, even when we're printing error messages that should go
to standard error. /Carol-->

The `>` syntax tells the shell to write the contents of standard out to
*output.txt* instead of the screen. We didn't see the error message we were
expecting printed on the screen, so that means it must have ended up in the
file. Let's see what *output.txt* contains:

```text
Problem parsing arguments: not enough arguments
```

<!-- I don't understand why we send this output to a file to then just say we
want it to the screen, won't it do that by default? And what has this got to do
with our use of println? I'm finding the motives here hard to follow -->
<!-- The point of showing this is to demonstrate that our program is NOT doing
the correct thing by default, we need to change the places we're calling
`println!` with error messages to print to standard error instead. When to use
stdout vs. stderr, and why you might want to redirect stdout but not stderr,
is something our readers will be familiar with. /Carol -->

Yup, there's our error message, which means it's being printed to standard out.
This isn't what's expected from command line programs. It's much more useful
for error messages like this to be printed to standard error, and only have
data printed to standard out from a successful run end up in the file when we
redirect standard out in this way. Let's change how error messages are printed
as shown in Listing 12-23. Because of the refactoring we did earlier in this
chapter, all of the code that prints error messages is in one place, in `main`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

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

<span class="caption">Listing 12-23: Writing error messages to `stderr` instead
of `stdout` using `writeln!`</span>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Rust does not have a convenient function like `println!` for writing to
standard error. Instead, we use the `writeln!` macro, which is like `println!`
but takes an extra argument. The first thing we pass to it is what to write to.
We can acquire a handle to standard error through the `std::io::stderr`
function. We give a mutable reference to `stderr` to `writeln!`; we need it to
be mutable so we can write to it! The second and third arguments to `writeln!`
are like the first and second arguments to `println!`: a format string and any
variables we're interpolating.

Let's try running the program again in the same way, without any arguments and
redirecting `stdout` with `>`:

```text
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Now we see our error on the screen, and `output.txt` contains nothing, which is
the behavior that's expected of command line programs.

If we run the program again with arguments that don't cause an error, but still
redirecting standard out to a file:

```text
$ cargo run to poem.txt > output.txt
```

We won't see any output to our terminal, and `output.txt` will contain our
results:

<span class="filename">Filename: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

This demonstrates that we're now using standard out for successful output and
standard error for error output as appropriate.

## Summary

In this chapter, we've recapped on some of the major concepts so far and
covered how to do common I/O operations in a Rust context. By using command
line arguments, files, environment variables, and the `writeln!` macro with
`stderr`, you're now prepared to write command line applications. By using the
concepts from previous chapters, your code will be well-organized, be able to
store data effectively in the appropriate data structures, handle errors
nicely, and be well tested.

Next, let's explore some functional-language influenced Rust features: closures
and iterators.
