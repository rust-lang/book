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

We'd love your help! Please see [CONTRIBUTING.md][contrib].

[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md

## No Starch

As the book will be published by No Starch, we first iterate here, then ship the
text off to No Starch. Then they do editing, and we fold it back in.

As such, there’s a directory, *nostarch*, which corresponds to the text in No
Starch’s system.

When we've started working with No Starch in a word doc, we will also check
those into the repo in the *nostarch/odt* directory. To extract the text from
the word doc as markdown in order to backport changes to the online book:

1. Open the doc file in LibreOffice
1. Accept all tracked changes
1. Save as Microsoft Word 2007-2013 XML (.docx) in the *tmp* directory
1. Run `./doc-to-md.sh`
1. Inspect changes made to the markdown file in the *nostarch* directory and copy the changes to the *src* directory as appropriate.
