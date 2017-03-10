# The Rust Programming Language

[![Build Status](https://travis-ci.org/rust-lang/book.svg?branch=master)](https://travis-ci.org/rust-lang/book)

To read this book online, visit [rust-lang.github.io/book/][html].

[html]: http://rust-lang.github.io/book/

This is the next iteration of “The Rust Programming Language” ([source][src],
[read online][prod]).

[src]: https://github.com/rust-lang/rust/tree/master/src/doc/book
[prod]: https://doc.rust-lang.org/book/

## Requirements

Building the book requires [mdBook] >= v0.0.13. To get it:

[mdBook]: https://github.com/azerupi/mdBook

```
$ cargo install mdbook
```

## Building

To build the book, first `cd` into either the `first-edition` or `second-edition` directory
depending on which edition of the book you would like to build. Then type:

```
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

_Firefox:_
```
$ firefox book/index.html           # Linux
$ open -a "Firefox" book/index.html # OS X
```

_Chrome:_
```
$ google-chrome book/index.html           # Linux
$ open -a "Google Chrome" book/index.html # OS X
```

To run the tests:

```
$ mdbook test
```

## Contributing

We'd love your help! Please see [CONTRIBUTING.md][contrib].

[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md

### Translations 

We'd especially love help translating! See the [Translations][] label to join
in efforts that are currently in progress. Open a new issue to start 
working on a new language! We're waiting on [mdbook support][] for multiple
languages before we merge any in, but feel free to start! The chapters in 
[the frozen column][] of the project won't see major changes, so if you start
with those, you won't have to redo work :)

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/azerupi/mdBook/issues/5
[the frozen column]: https://github.com/rust-lang/book/projects/1

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

## Graphviz dot

This is mostly for Carol's reference because she keeps having to look it up.

We're using [Graphviz](http://graphviz.org/) for some of the diagrams in the
book. The source for those files live in the `dot` directory. To turn a `dot`
file, for example, `dot/trpl04-01.dot` into an `svg`, run:

```
$ dot dot/trpl04-01.dot -Tsvg > src/img/trpl04-01.svg
```

In the generated SVG, remove the width and the height attributes from the `svg`
element and set the `viewBox` attribute to `0.00 0.00 1000.00 1000.00` or other
values that don't cut off the image.

## Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script. It needs a dictionary of valid words, which is provided in
`dictionary.txt`. If the script produces a false positive (say, you used word
`BTreeMap` which the script considers invalid), you need to add this word to
`dictionary.txt` (keep the sorted order for consistency).

