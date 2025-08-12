![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)
# The Rust Programming Language
## Abstract
This repository contains the source of "The Rust Programming Language" book.

The book is available in various forms and releases:
- [Printed form, **stable** release, from No Starch Press][printed]
- [Digital form, **stable** release, from No Starch Press][stable]
- [Digital form, **beta** release, from No Starch Press][beta]
- [Digital form, **nightly** release, from No Starch Press][nightly]

> [!WARNING]
Be aware that issues in those versions may have been fixed in this repository already, as those
releases are updated less frequently.

[printed]: https;//nostarch.com/rust-programming-language-2nd-edition 
[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

See the [releases] to download just the code of all the code listings that appear in the book.

[releases]: https://github.com/rust-lang/book/releases

## Prerequisites

Building the book requires [mdBook], ideally the same version that
rust-lang/rust uses in [this file][rust-mdbook]. To get it:

[mdBook]: https://github.com/rust-lang/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```sh
cargo install mdbook --locked --version <version_num>
```

The book also uses two mdbook plugins which are part of this repository. If you
do not install them, you will see warnings when building and the output will not
look right, but you _will_ still be able to build the book. To use the plugins,
you should run:

```sh
cargo install --locked --path packages/mdbook-trpl --force
```

## Building

To build the book, type:

```sh
mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

### Firefox:

_Linux:_

```sh
firefox book/index.html                       
```
_macOS:_

```sh
open -a "Firefox" book/index.html             
```

_Windows | Powershell:_

```sh
Start-Process "firefox.exe" .\book\index.html 
```

_Windows | CMD:_

```sh
start firefox.exe .\book\index.html           
```

### Chrome:

_Linux:_

```sh
google-chrome book/index.html                       
```
_macOS:_

```sh
open -a "Google Chrome" book/index.html             
```

_Windows | Powershell:_

```sh
Start-Process "chrome.exe" .\book\index.html 
```

_Windows | CMD:_

```sh
start chrome.exe .\book\index.html           
```

## TESTING

```sh
cd packages/trpl
mdbook test --library-path packages/trpl/target/debug/deps
```

## Contributing

Please see [CONTRIBUTING.md][contrib] 

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Considering the book is [printed], and parity between available forms is desired, it may take a while for your issue or PR to be addressed.

A larger revision is currently underway to coincide with [Rust Editions](https://doc.rust-lang.org/edition-guide/). Between those larger
revisions, only error-correcting will take place.

If the submitted issue or pull request isn't strictly fixing an error, it might sit until the next large revision: expect on the order of months or years. Thank you
for your patience!

### Translations

See the [Translations] label to join in efforts that are currently in progress. Open a new issue to start working on
a new language! We're waiting on [mdbook support] for multiple languages before we merge any in, but feel free to start!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang/mdBook/issues/5

## Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script available in the `ci` directory. It needs a dictionary of valid words,
which is provided in `ci/dictionary.txt`. If the script produces a false
positive (say, you used the word `BTreeMap` which the script considers invalid),
you need to add this word to `ci/dictionary.txt` (keep the sorted order for
consistency).
