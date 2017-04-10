## Writing to `stderr` instead of `stdout`

In our program, we have been careful to use `eprintln!` for errors and
`println!` for the matching lines. This is because, as we mentioned in Chapter
9.3, command-line programs have two kinds of output: "standard output" is for
the "normal output" of the program, and "standard error" is for error and
progress messages. You may be wondering why this is an important distinction
to make, so we're going to step up a level and illustrate how it's useful.

If you run our program with no command line arguments, it prints an error:

```text
$ cargo run
Problem parsing arguments: not enough arguments
```

That error is printed with `eprintln!`, so it goes to standard error, so it is
_not_ redirected by the shell's `>` operator (which sends standard _output_ to
a file):

```text
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

We still see the error message in the terminal, and `output.txt` will be
empty. If we had used `println!` for the error messages, they would have been
redirected into `output.txt`.  We wouldn't have seen them, and `output.txt`
would have unexpected junk in it.

If we try this again with arguments that work:

```text
$ cargo run to poem.txt > output.txt
```

we'll see no output to our terminal, but `output.txt` will contain our
results:

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
