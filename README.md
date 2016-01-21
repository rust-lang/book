# The Rust Programming Language

This is the next iteration of “The Rust Programming Language”, which is
currently located [in the main Rust repository][src]. If you want to read the
book, you should check it out there or [on the web][prod].

[src]: https://github.com/rust-lang/rust/tree/master/src/doc/book
[prod]: https://doc.rust-lang.org/book/

If you would like to see this version rendered, it’s [on GitHub pages][html].

[html]: http://rust-lang.github.io/book/

## Requirements

Building the book requires [mdBook]. To get it:

[mdBook]: https://github.com/azerupi/mdBook

```bash
$ cargo install mdbook
```

## Building

To build the book, type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser:

```bash
$ firefox book/index.html
```

To run the tests:

```bash
$ mdbook test
```

## Contributing

I’m not going to be accepting major changes at first, but pull requests to fix
typos and such are welcome. Please file any issues for any bugs you find, and
utilize the issue tagged with `Discussion` to raise any larger questions /
feedback.

This repository is under the same license as Rust itself, MIT/Apache2.

There are a number of labels on Issues:

* `Discussion` issues are for discussing the chapters. There’s an issue per
  section.
* `Bug` issues indicate problems in the text.
* `Needs Backport` will be used when we are further along. At some point, we
  will import the text into their review system, and so changes made here will
  need to be upstreamed. This will track those.

Finally, there’s the `S-` labels, which are for various ‘status’es:

* `S-initial`: Steve has not done any work here yet.
* `S-rough-draft`: Steve has worked up a rough draft of what this section will
  look like.
* `S-under-review`: Aaron and Steve are in the process of reviewing this
  section.
* `S-done`: imported into No Starch’s system. There may still be changes based
  on their feedback, even after a section is marked `S-done`.

## No Starch

As the book will be published by No Starch, we first iterate here, then ship the
text off to No Starch. Then they do editing, and we fold it back in.

As such, there’s a directory, `nostarch`, which corresponds to the `S-done`,
and its current status in No Starch’s system.
