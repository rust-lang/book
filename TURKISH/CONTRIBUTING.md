# Contributing to translate

## Guidelines

### Translation terms

We need to coordinate us with the main English terms translations.

In this purpose, please refer to the file `/FRENCH/src/translation-terms.md`
when you need to translate a technical term.

*(PS : see the next process `Add a translation term` on this same page)*

### Translation of Rust code

In rust code, we should translate :

- comment
- string text
- variable name
- struct name (which is custom, that means it not come from *standard library*
  or *external crate*)
- enum name (which is custom, that means it not come from *standard library*
  or *external crate*)

All the standard code (and terminal outputs) should stay in English.

### Files

The name of all the files should not be translated, so just keep them unchanged,
in English.

Please limit each line of Markdown file to 80 characters (including spaces). You
can write your file as you want, but it would be nice to use a tool like
[https://www.dcode.fr/text-splitter](https://www.dcode.fr/text-splitter) on your
translated paragraphs before committing.

### Punctuation

Please use a [non-breaking space](https://en.wikipedia.org/wiki/Non-breaking_space)
instead of space on punctuation who need this space before (like `:`, `!`, `?`,
...).

## Tools

We recommend to use the following tools :

- [Microsoft Visual Studio Code](https://code.visualstudio.com/) for writing
  code, and improve it with following extensions :
    - [`davidanson.vscode-markdownlint`](https://marketplace.visualstudio.com/items?itemName=davidanson.vscode-markdownlint)
    - [`moshfeu.compare-folders`](https://marketplace.visualstudio.com/items?itemName=moshfeu.compare-folders)
    - [`rust-lang.rust`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- [Nadaclair BonPatron.com](https://bonpatron.com/) for proofreading
- [Deepl](https://www.deepl.com/translator) for translations

## Processes

### Translate flow

*NB : the following term `main translate repository` refers to
<https://github.com/Jimskapt/rust-book-fr>*

01. Open or edit an GitHub issue in the *main translate repository* to report to
    other that you are working on `ch00-00-introduction`.
      - if someone has reported to work on the same page you want to translate :
        - if its works is *active* : you should work on another page.
        - else : you can fork his/her repo in the following step 02 (instead of
          the *main translate repository*), and please mention this in the
          existing issue.
      - else : just follow the next instructions.
02. Fork the *main translate repository* in GitHub (on your account)
03. `git clone https://github.com/<your_fork>.git`
    (copy your fork on your hard-disk). You should be on the `french-release`
    branch by default.
04. `cd <your_fork>` (go inside your fork folder your previously downloaded)
05. `git checkout -b <your_new_working_branch_name>` (create your new working
    branch)
06. Copy the file you want to translate : `/src/ch00-00-introduction.md` into
    `/FRENCH/src/ch00-00-introduction.md`. Tip : you can use a tool like
    [Castor Whispers](https://github.com/Jimskapt/castor_whispers) in order to
    copy and mass-comment each paragraphs of this file.
07. Add the text for the link to this file in the `/FRENCH/src/SUMMARY.md`.
08. Comment each English paragraphs of this file. The goal is to keep an
    *invisible* version of the English book, in order to easily reflect the
    changes of the English source (english-book) when they occur later, and to
    easily translate them.
09. Write each of your translated paragraph under each commented English
    paragraph.
      - Please quickly read following `Guidelines` currently on this page.
      - A little tip : the [deepl.com](https://www.deepl.com/) translator.
10. (optional) Limit each line of your translation to 80 characters, thank to a
    tool like
    [https://www.dcode.fr/text-splitter](https://www.dcode.fr/text-splitter).
11. (optional) `cd FRENCH && mdbook build && cd ..` (build the book in
    `/FRENCH/book`). Open its index.html file in your browser, and check its
    correctness. It also should help you for next task.
12. (optional) self-proofreading your work thank to services like
    [bonpatron.fr](https://bonpatron.com).
13. `git add -A && git commit -m "<Description of your work>"` (committing your
    work)
14. (optional) `git rebase -i HEAD~<the number of commits you need to merge>`
    (squash all your commits into one commit)
15. `git push origin` (pushing your work on your fork)
16. In GitHub, create a new pull request from your fork to the main translation
    repository, in order to mark your work ready for a proofreading.
17. After someone proofreading it (and eventually some edits), it would be
    merged on `french-release` branch.

### Update your fork with another fork

01. `git remote add english-book https://github.com/rust-lang/book.git`
    (Add source of the *English main repository*)
02. `git fetch english-book` (fetching the latest changes from the *English main
    repository*)
03. `git merge english-book/master` (merging latest changes from *English main
    repository* on current branch)

It is also the same to update your fork with the main translate repository.

### Add a translation term

*(PS : see previous Guideline `Translation terms` on this same page)*

01. Check if you are one your working branch, or create it (see `Translate flow`
    process)
02. Edit the `/FRENCH/src/translation-terms.md` file with your new technical
    term translation. Write it in singular and if necessary, specify the gender
    of the translation in `Remarques` column.

### Translate figures

Let's suppose you want to translate Figure 42-69.
You need to have the `dot` installed, for instance after typing
`sudo apt install graphviz`.

01. Copy the DOT figure you want to translate (for instance, `trpl42-69.dot`)
    from `/dot/` to `/FRENCH/dot/`.
02. Edit `/FRENCH/dot/trpl42-69.dot` and translate the text into French.
    You should not translate the names and values of attributes.
03. Run `dot FRENCH/dot/trpl42-69.dot -Tsvg > FRENCH/src/img/trpl42-69.svg`
04. Edit the new file `FRENCH/src/img/trpl42-69.svg`:
    - Within the `<svg>` tag, remove the `width` and `height` attributes, and
      set the `viewBox` attribute to `0.00 0.00 1000.00 1000.00` or other values
      that don't cut off the image.
    - Replace every instance of `font-family="Times,serif"` with
      `font-family="Times,Liberation Serif,serif"`.
