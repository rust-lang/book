# Style Guide

## Prose

* Prefer title case for chapter/section headings, ex: `## Generating a Secret
  Number` rather than `## Generating a secret number`.
* Prefer italics over single quotes when calling out a term, ex: `is an
  *associated function* of` rather than `is an ‘associated function’ of`.
* When talking about a method in prose, DO NOT include the parentheses, ex:
  `read_line` rather than `read_line()`.
* Hard wrap at 80 chars
* Prefer not mixing code and not-code in one word, ex: ``Remember when we wrote
  `use std::io`?`` rather than ``Remember when we `use`d `std::io`?``

## Code

* Add the file name before markdown blocks to make it clear which file we're
  talking about, when applicable.
* When making changes to code, make it clear which parts of the code changed
  and which stayed the same... not sure how to do this yet
* Split up long lines as appropriate to keep them under 80 chars if possible
* Use `bash` syntax highlighting for command line output code blocks

## Links

Once all the scripts are done:

* If a link shouldn't be printed, mark it to be ignored
  * This includes all "Chapter XX" intra-book links, which *should* be links
    for the HTML version
* Make intra-book links and stdlib API doc links relative so they work whether
  the book is read offline or on docs.rust-lang.org
* Use markdown links and keep in mind that they will be changed into `text at
  *url*` in print, so word them in a way that it reads well in that format
