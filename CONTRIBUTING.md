# Contributing

We'd love your help! Thanks for caring about the book.

## Where to Edit

All edits should be made in the `src` directory.

The `nostarch` directory contains snapshots for sending edits to the publishers
of the print version. The snapshot files reflect what has been sent or not, so
they only get updated when edits are sent to No Starch. **Do not submit pull
requests changing files in the `nostarch` directory, they will be closed.**

## Checking for Fixes

The book rides the Rust release trains. Therefore, if you see a problem on
https://doc.rust-lang.org/stable/book, it may already be fixed on the `main`
branch in this repo, but the fix hasn't gone through nightly -> beta -> stable
yet. Please check the `main` branch in this repo before reporting an issue.

Looking at the history for a particular file can also give more information on
how or whether an issue has been fixed or not if you're trying to figure that
out.

Please also search open and closed issues and open and closed PRs before
reporting a new issue or opening a new PR.

## Licensing

This repository is under the same license as Rust itself, MIT/Apache2. You
can find the full text of each license in the `LICENSE-*` files in this
repository.

## Code of Conduct

The Rust project has [a code of conduct](http://rust-lang.org/policies/code-of-conduct)
that governs all sub-projects, including this one. Please respect it!

## Expectations

Because the book is [printed](https://nostarch.com/rust), and because we want
to keep the online version of the book close to the print version when
possible, it may take longer than you're used to for us to address your issue
or pull request.

So far, we've been doing a larger revision to coincide with [Rust
Editions](https://doc.rust-lang.org/edition-guide/). Between those larger
revisions, we will only be correcting errors. If your issue or pull request
isn't strictly fixing an error, it might sit until the next time that we're
working on a large revision: expect on the order of months or years. Thank you
for your patience!

## Help wanted

If you're looking for ways to help that don't involve large amounts of
reading or writing, check out the [open issues with the E-help-wanted
label][help-wanted]. These might be small fixes to the text, Rust code,
frontend code, or shell scripts that would help us be more efficient or
enhance the book in some way!

[help-wanted]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3AE-help-wanted

## Translations

We'd love help translating the book! See the [Translations] label to join in
efforts that are currently in progress. Open a new issue to start working on
a new language! We're waiting on [mdbook support] for multiple languages
before we merge any in, but feel free to start!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang-nursery/mdBook/issues/5
