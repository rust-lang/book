## Writing Error Messages to Standard Error Instead of Standard Output

At the moment we’re writing all of our output to the terminal with the
`println!` function. Most terminals provide two kinds of output: *standard
output* for general information (sometimes abbreviated as `stdout` in code),
and *standard error* for error messages (`stderr`). This distinction enables
users to choose to direct the successful output of a program to a
file but still print error messages to the screen.

The `println!` function is only capable of printing to standard output, though,
so we have to use something else in order to print to standard error.

### Checking Where Errors are Written to

First, let’s observe how all content printed by `minigrep` is currently being
written to standard output, including error messages that we want to write to
standard error instead. We’ll do that by redirecting the standard output stream
to a file while we also intentionally cause an error. We won’t redirect the
standard error stream, so any content sent to standard error will continue to
display on the screen. Command line programs are expected to send error
messages to the standard error stream so that we can still see error messages
on the screen even if we choose to redirect the standard output stream to a
file. Our program is not currently well-behaved; we’re about to see that it
saves the error message output to the file instead!

The way to demonstrate this behavior is by running the program with `>` and the
filename, *output.txt*, that we want to redirect the standard output stream to.
We’re not going to pass any arguments, which should cause an error:

```text
$ cargo run > output.txt
```

The `>` syntax tells the shell to write the contents of standard output to
*output.txt* instead of the screen. We didn’t see the error message we were
expecting printed on the screen, so that means it must have ended up in the
file. Let’s see what *output.txt* contains:

```text
Problem parsing arguments: not enough arguments
```

Yup, our error message is being printed to standard output. It’s much more
useful for error messages like this to be printed to standard error, and have
only data from a successful run end up in the file when we redirect standard
output in this way. We’ll change that.

### Printing Errors to Standard Error

Let’s change how error messages are printed using the code in Listing 12-24.
Because of the refactoring we did earlier in this chapter, all the code that
prints error messages is in one function, in `main`. The standard library
provides the `eprintln!` macro that prints to the standard error stream, so
let’s change the two places we were calling `println!` to print errors so that
these spots use `eprintln!` instead:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
```

<span class="caption">Listing 12-24: Writing error messages to standard error
instead of standard output using `eprintln!`</span>

After changing `println!` to `eprintln!`, let’s try running the program again
in the same way, without any arguments and redirecting standard output with `>`:

```text
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Now we see our error on the screen and `output.txt` contains nothing, which is
the behavior expected of command line programs.

If we run the program again with arguments that don’t cause an error, but still
redirect standard output to a file:

```text
$ cargo run to poem.txt > output.txt
```

We won’t see any output to our terminal, and `output.txt` will contain our
results:

<span class="filename">Filename: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

This demonstrates that we’re now using standard output for successful output and
standard error for error output as appropriate.

## Summary

In this chapter, we’ve recapped on some of the major concepts so far and
covered how to do common I/O operations in a Rust context. By using command
line arguments, files, environment variables, and the `eprintln!` macro for
printing errors, you’re now prepared to write command line applications. By
using the concepts from previous chapters, your code will be well-organized, be
able to store data effectively in the appropriate data structures, handle
errors nicely, and be well tested.

Next, let’s explore some functional-language influenced Rust features: closures
and iterators.
