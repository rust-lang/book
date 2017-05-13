# Contributing

## First edition

The first edition of the book is no longer actively being worked on, since
we're concentrating our efforts on bringing the second edition to print. We
will accept pull requests for small tweaks to the first edition; any larger
work should be spent improving the second edition. Issues will likely be closed
unless they are also issues in the second edition.

## Second edition

We're currently working with No Starch Press to bring the second edition of the
book to print. Each chapter goes through [a number of stages][project]:

[project]: https://github.com/rust-lang/book/projects/1

* We write and edit a chapter's initial content
* No Starch provides a round of edits and questions
* We revise, clarify, and check those edits
* A Technical Reviewer checks for the accuracy of technical details
* No Starch copyedits the chapter for spelling, grammar, wording, consistency
* We revise, clarify, and check the copyedits
* The chapter goes to layout, at which point only minor changes should be made

### Documenting newly stabilized features

New features added to Rust will be documented in the ["Newest Features"
Appendix][new] per [RFC 1636][rfc]. We'd love pull requests adding new
sections! These sections may be incorporated into the book at some point, but
we have no timeline for doing so; see the Post-publication section below for
more details.

[new]: https://github.com/rust-lang/book/blob/master/second-edition/src/appendix-07-newest-features.md
[rfc]: https://github.com/rust-lang/rfcs/pull/1636#issuecomment-247325313

### Corrections and Modifications

We would love issues and pull requests to the Markdown files in the src
directory, up until the chapter goes to layout with No Starch. At that point,
we will likely only be accepting changes that correct factual errors or major
problems and not, for example, minor wording changes.

You can check which chapters have gone to layout and are frozen on the [project
page][project] by scrolling all the way to the right to find the column titled
**Frozen**.

### Review

Our [open pull requests][pulls] are new chapters or edits that we're currently
working on. We would love if you would read through those and make comments for
any suggestions or corrections!

[pulls]: https://github.com/rust-lang/book/pulls

### Translations

We'd especially love help translating the second edition of the book! See the
[Translations] label to join in efforts that are currently in progress. Open
a new issue to start working on a new language! We're waiting on [mdbook
support] for multiple languages before we merge any in, but feel free to
start! The chapters in [the frozen column] of the project won't see major
changes, so if you start with those, you won't have to redo work :)

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/azerupi/mdBook/issues/5
[the frozen column]: https://github.com/rust-lang/book/projects/1

### Help wanted

If you're looking for ways to help that don't involve large amounts of reading
or writing, check out the [open issues with the E-help-wanted
label][help-wanted]. These might be small fixes to the text Rust code, frontend
code, or shell scripts that would help us be more efficient or enhance the book
in some way!

[help-wanted]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3AE-help-wanted

### Post-publication

After the second edition of the book goes to print, here are our intentions for
changes:

* The online version should stay fairly close to the printed version. For
  example, you should be able to look at listing 10-3 in the book and find
  listing 10-3 in the online version and copy-paste the code if you want to
  play with it. Major changes to correct errors should get documented in
  errata.
* There are multiple efforts starting to translate the online book into
  other languages. It would help the translations stay in sync if we're not
  constantly changing the text.
* Someday there might be a third edition, once there are enough large, new
  features in Rust to warrant such a thing. We don't have any schedule in mind
  for that though, nor have we decided if it would be modifications to the
  second edition or a ground-up rewrite. Until we have plans for that, we won't
  be accepting pull requests that aren't fixing errors, for example, changing
  the way something is worded.

This repository is under the same license as Rust itself, MIT/Apache2.
