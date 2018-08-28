# The Rust Programming Language

[![Build Status](https://travis-ci.org/rust-lang/book.svg?branch=master)](https://travis-ci.org/rust-lang/book)

This repository contains the source of all editions of "the Rust Programming
Language".

The second edition will also be available in dead-tree form by No Starch
Press, available around June 2018. Check [the No Starch Page][nostarch] for
the latest information on the release date and how to order.

[nostarch]: https://nostarch.com/rust

You can read all editions of the book for free online! Please see the book as
shipped with the latest [stable], [beta], or [nightly] Rust releases. Be
aware that issues in those versions may have been fixed in this repository
already, as those releases are updated less frequently.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

## Requirements

Building the book requires [mdBook], ideally the same version that
[rust-lang/rust uses in this file][rust-mdbook]. To get it:

[mdBook]: https://github.com/azerupi/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --vers [version-num]
```

## Building

To build the book, first `cd` into the directory of the edition of the
book you'd like to build. For example, the `first-edition` or
`second-edition` directory. Then type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

To run the tests:

```bash
$ mdbook test
```

## Contributing

We'd love your help! Please see [CONTRIBUTING.md][contrib] to learn about the
kinds of contributions we're looking for.

### 2018 Edition

The "2018" Edition is in the process of being updated with the language changes 
that will be available with the 2018 Edition of the Rust language. All new 
contributions should be to this edition.

### Second Edition

No Starch Press has brought the second edition to print. Pull requests fixing
factual errors will be accepted and documented as errata; pull requests changing
wording or other small corrections should be made against the 2018 edition instead.

### First Edition

The first edition is frozen, and is not accepting any changes at this time.


[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md

### Translations

We'd especially love help translating the second edition or 2018 edition of the book! See the
[Translations] label to join in efforts that are currently in progress. Open
a new issue to start working on a new language! We're waiting on [mdbook
support] for multiple languages before we merge any in, but feel free to
start! The second edition is frozen and won't see major
changes, so if you start with that, you won't have to redo work :)

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/azerupi/mdBook/issues/5

## No Starch

As the second edition of the book will be published by No Starch, we first
iterate here, then ship the text off to No Starch. Then they do editing, and we
fold it back in.

As such, there’s a directory, *nostarch*, which corresponds to the text in No
Starch’s system.

When we've started working with No Starch in a word doc, we will also check
those into the repo in the *nostarch/odt* directory. To extract the text from
the word doc as markdown in order to backport changes to the online book:

1. Open the doc file in LibreOffice
1. Accept all tracked changes
1. Save as Microsoft Word 2007-2013 XML (.docx) in the *tmp* directory
1. Run `./doc-to-md.sh`
1. Inspect changes made to the markdown file in the *nostarch* directory and
   copy the changes to the *src* directory as appropriate.

## Graphviz dot

We're using [Graphviz](http://graphviz.org/) for some of the diagrams in the
book. The source for those files live in the `dot` directory. To turn a `dot`
file, for example, `dot/trpl04-01.dot` into an `svg`, run:

```bash
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
