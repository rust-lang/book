# لغة البرمجة Rust (الترجمة العربية)

# The Rust Programming Language (Arabic Translation)

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)

هذا المستودع يحتوي على الترجمة العربية لكتاب "لغة البرمجة Rust".

This repository contains the Arabic translation of "The Rust Programming Language" book.

## النسخة الأصلية / Original Version

النسخة الأصلية الإنجليزية متاحة على: [rust-lang/book](https://github.com/rust-lang/book)

The original English version is available at: [rust-lang/book](https://github.com/rust-lang/book)

يمكنك قراءة الكتاب الأصلي مجاناً عبر الإنترنت في النسخ [المستقرة][stable]، [التجريبية][beta]، أو [الليلية][nightly] من Rust.

You can read the original book for free online in the latest [stable], [beta], or [nightly] Rust releases.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

[The original book is also available in dead-tree form from No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

## قائمة الترجمات / Translations List

هذا المشروع مدرج الآن في [قائمة الترجمات الرسمية](https://doc.rust-lang.org/book/appendix-06-translation.html) في ملحق الكتاب.

This project is now listed in the [official translations list](https://doc.rust-lang.org/book/appendix-06-translation.html) in the book's appendix.

## Requirements

Building the book requires [mdBook], ideally the same version that
rust-lang/rust uses in [this file][rust-mdbook]. To get it:

[mdBook]: https://github.com/rust-lang/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/HEAD/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --locked --version <version_num>
```

The book also uses two mdbook plugins which are part of this repository. If you
do not install them, you will see warnings when building and the output will not
look right, but you _will_ still be able to build the book. To use the plugins,
you should run:

```bash
$ cargo install --locked --path packages/mdbook-trpl --force
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
$ cd packages/trpl
$ mdbook test --library-path packages/trpl/target/debug/deps
```

## المساهمة / Contributing

نرحب بمساعدتك في تحسين الترجمة العربية! إذا وجدت أخطاء في الترجمة أو لديك اقتراحات لتحسينها، يرجى فتح issue أو إرسال pull request.

We'd love your help improving the Arabic translation! If you find translation errors or have suggestions for improvements, please open an issue or submit a pull request.

للمساهمة في النسخة الأصلية الإنجليزية، يرجى زيارة [CONTRIBUTING.md][contrib] في المستودع الأصلي.

For contributing to the original English version, please see [CONTRIBUTING.md][contrib] in the original repository.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

### الترجمات الأخرى / Other Translations

للاطلاع على ترجمات أخرى لكتاب Rust، راجع [قائمة الترجمات][translations] في الملحق و من الكتاب.

For other translations of the Rust book, see the [Translations list][translations] in Appendix F of the book.

[translations]: https://doc.rust-lang.org/book/appendix-06-translation.html

## Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script available in the `ci` directory. It needs a dictionary of valid words,
which is provided in `ci/dictionary.txt`. If the script produces a false
positive (say, you used the word `BTreeMap` which the script considers invalid),
you need to add this word to `ci/dictionary.txt` (keep the sorted order for
consistency).
