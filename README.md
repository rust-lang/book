# The Rust Programming Language

[![Build Status](https://travis-ci.org/rust-lang/book.svg?branch=master)](https://travis-ci.org/rust-lang/book)

This is the next iteration of “The Rust Programming Language”, which is
currently located [in the main Rust repository][src]. If you want to read the
book, you should check it out there or [on the web][prod].

[src]: https://github.com/rust-lang/rust/tree/master/src/doc/book
[prod]: https://doc.rust-lang.org/book/

If you would like to see this version rendered, it’s [on GitHub pages][html].

[html]: http://rust-lang.github.io/book/

## Requirements

Building the book requires [mdBook] >= v0.0.13. To get it:

[mdBook]: https://github.com/azerupi/mdBook

```bash
$ cargo install mdbook
```

## Building

To build the book, type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

_Firefox:_
```bash
$ firefox book/index.html           # Linux
$ open -a "Firefox" book/index.html # OS X
```

_Chrome:_
```bash
$ google-chrome book/index.html           # Linux
$ open -a "Google Chrome" book/index.html # OS X
```

To run the tests:

```bash
$ mdbook test
```

## Contributing

I’m not going to be accepting major changes at first, but pull requests to fix
typos and such are welcome. Please file any issues for any bugs you find.

This repository is under the same license as Rust itself, MIT/Apache2.

There are a number of labels on Issues:

* `Enhancement` issues are a request for an improvement of some kind.
* `Bug` issues indicate problems in the text.
* `Needs Backport` will be used when we are further along. At some point, we
  will import the text into their review system, and so changes made here will
  need to be upstreamed. This will track those.

## No Starch

As the book will be published by No Starch, we first iterate here, then ship the
text off to No Starch. Then they do editing, and we fold it back in.

As such, there’s a directory, `nostarch`, which corresponds to the text in No
Starch’s system.
