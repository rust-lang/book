# The Rust Programming Language

[![Build Status](https://travis-ci.org/rust-lang/book.svg?branch=master)](https://travis-ci.org/rust-lang/book)

This repo contains the translation of “The Rust Programming Language” - second edition.

La seconda edizione è una riscrittura del testo e sarà stampata da NoStarch Press, disponibile verso Ottobre 2017.

## Requisiti

Per costruire questo libro ti serve [mdBook] >= v0.0.13. Per installarlo:

[mdBook]: https://github.com/azerupi/mdBook

```bash
$ cargo install mdbook
```

## Building

Per costruire il libro entra nella directory della seconda edizione:
 
```bash
$ cd second-edition
``` 
Poi:

```bash
$ mdbook build
```

Il risultato sarà nella subdirectory `book`.
Per leggerlo vai nel tuo browser:

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

Per eseguire i test:

```bash
$ mdbook test
```

## Contribuire

Apprezzeremmo molto il tuo aiuto! Leggi [CONTRIBUTING.md][contrib] per sapere i generi di contributo che stiamo cercando.

[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md

### Traduzioni

Ci piacerebbe soprattutto aiutare a tradurre la seconda edizione del libro! Guarda le etichette
[Translations] per partecipare agli sforzi attualmente in corso. 

Apri una nuova segnalazione per iniziare a lavorare su una nuova lingua! Stiamo aspettando il [supporto di mdbook] per linguaggi multipli prima di unirli, ma sentiti libero di iniziare! I capitoli in [the frozen column] non subiranno più grossi cambiamenti, quindi se inizi da quelli, non dovrai rifare il lavoro :)

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[supporto di mdbook]: https://github.com/azerupi/mdBook/issues/5
[the frozen column]: https://github.com/rust-lang/book/projects/1

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

This is mostly for Carol's reference because she keeps having to look it up.

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
