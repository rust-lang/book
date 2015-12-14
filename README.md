# The Rust Programming Language

This is the next iteration of “The Rust Programming Language”, which is
currently located [in the main Rust repository][src]. If you want to read the
book, you should check it out there or [on the web][prod].

[src]: https://github.com/rust-lang/rust/tree/master/src/doc/book
[prod]: https://doc.rust-lang.org/book/

If you would like to see this verison rendered, it’s [on GitHub pages][html].

[html]: http://rust-lang.github.io/book/

## Requirements

Building the book requires [mdBook]. To get it:

[mdBook]: https://github.com/azerupi/mdBook

```bash
$ cargo install mdbook
```

## Building

To build the book, type:

```bash
$ make
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser:

```bash
$ firefox book/index.html
```

To run the tests:

```bash
$ make test
```

To do both:

```bash
$ make all
```

## Contributing

I’m not going to be accepting major changes at first, but pull requests to fix
typos and such are welcome. Please file any issues for discussion of larger
changes.

This repository is under the same license as Rust itself, MIT/Apache2.
