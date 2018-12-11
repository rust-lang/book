# Appendix E - Editions

Way back in Chapter 1, we saw that `cargo new` adds a bit of metadata to your
*Cargo.toml* about an `edition`. This appendix talks about what that means!

The Rust language and compiler have a six-week release cycle. This means users
get a constant stream of new features. Other programming languages release
larger changes less often; Rust chooses to release smaller updates more
frequently. After a while, all of those tiny changes add up. But from release
to release, it can be hard to look back and say “Wow, between Rust 1.10 and
Rust 1.31, Rust has changed a lot!”

Every two or three years, the Rust team produces a new *edition* of Rust.
Each edition brings together the features that have landed into a clear
package with fully updated documentation and tooling. New editions ship
as part of the usual six-week release process.

This serves different purposes for different people:

* For active Rust users, it brings together incremental changes into an
  easy-to-understand package.
* For non-users, it signals that some major advancements have landed, which
  might make Rust worth another look.
* For those developing Rust itself, it provides a rallying point for the
  project as a whole.

At the time of writing, there are two editions: Rust 2015 and Rust 2018.
This book is written using Rust 2018 edition idioms.

The `edition` key in *Cargo.toml* indicates which edition your code should be
compiled under. If the key does not exist, it defaults to `2015` for backwards
compatibility reasons.

Each project can choose to opt in to an edition other than the default 2015
edition. By doing so, editions can contain incompatible changes, such as adding
a new keyword that might conflict with identifiers in code or turning warnings
into errors. But unless you opt in to those changes, your code will continue to
compile even as you upgrade the version of the Rust compiler that you use. All
Rust compiler versions support any edition that existed prior to that
compiler’s release, and they can link crates of any supported editions
together. Edition changes only affect the way the compiler initially parses
code. Therefore, if you’re using Rust 2015 and one of your dependencies uses
Rust 2018, your project will compile and be able to use that dependency. The
opposite situation, where your project uses Rust 2018 and a dependency uses
Rust 2015, works as well.

To be clear: most features will be available on all editions. Developers using
any edition of Rust will continue to see improvements as new stable releases
are made. In some cases, however, mainly when new keywords are added, there may
be new features that are only available in later editions. You only need to
switch editions if you want to take advantage of such features.

For more details, the [Edition
Guide](https://rust-lang-nursery.github.io/edition-guide/) is a complete
book about editions, including how to automatically upgrade your code to
a new edition via `cargo fix`.
