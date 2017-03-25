# Contributing

## First Edition

We accept pull requests for the first edition,
but prefer small tweaks over large changes,
because we would like effort be spent on improving the second edition.

## Second Edition

We are working with No Starch Press to
[bring the book to print](#publication-stages).

### Corrections and Modifications

Until a chapter goes to layout with No Starch, we would love issues 
and pull requests to the Markdown files in the [second-edition/src] directory.
At that point, we will likely only be accepting changes that correct
factual errors or major problems and not, for example, minor wording changes.
You can check which chapters have gone to layout and are frozen, on the
[project page][project] by scrolling all the way to the right to find the column
titled **Frozen**.

[second-edition/src]: https://github.com/rust-lang/book/tree/master/second-edition/src

### New Content and Ideas

We would love your ideas.  Please open issues with ideas for what,
specifically, you'd like to see covered!

### Review

Our [open pull requests][pulls] are new chapters or edits that we're currently
working on. We would love if you would read through those and make comments for
any suggestions or corrections!

[pulls]: https://github.com/rust-lang/book/pulls

### Techical Help

If you're looking for ways to help that don't involve reading or writing, check
out the [open issues with the E-help-wanted label][help-wanted]. These might be
Rust code, frontend code, or shell scripts that would help us be more efficient
or enhance the book in some way!

[help-wanted]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3AE-help-wanted

### Post-Publication
After the book goes to print, here are our plans:

* The online version should stay close-ish to the printed version, for
  example, you should be able to look at listing 10-3 in the book and find
  listing 10-3 in the online version and copy-paste the code if you want to
  play with it. Major changes to correct errors should get documented in
  errata.
* There are multiple efforts starting to translate the online book into
  other languages. It would help the translations stay in sync if we're not
  constantly changing the text.
* Someday there might be a third edition, once there are enough large, new
  features in Rust to warrant such a thing. We don't have any schedule in mind
  for that though, so if we were to leave PRs around unmerged, they might be
  open indefinitely.

### Publication Stages
Each chapter goes through a number of stages which are [tracked on GitHub][project]:

[project]: https://github.com/rust-lang/book/projects/1

* We write and edit a chapter's initial content
* No Starch provides a round of edits and questions
* We revise, clarify, and check those edits
* A Technical Reviewer checks for the accuracy of technical details
* No Starch copyedits the chapter for spelling, grammar, wording, consistency
* We revise, clarify, and check the copyedits
* The chapter goes to layout, at which point only minor changes should be made

## License

This repository is under the same license as Rust itself, MIT/Apache2.