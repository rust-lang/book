% Crates and Modules

<small>There is a new edition of the book and this is an old link.</small>

> Rust has a module system that enables the reuse of code in an organized fashion.
> A module is a namespace that contains definitions of functions or types, and you can choose whether those definitions are visible outside their module (public) or not (private).
>
> A crate is a project that other people can pull into their projects as a dependency.

```rust
mod network {
    fn connect() {
    }
}
```

---

Here are the relevant sections in the new and old books:

* **[in the current edition: Ch 7.01 — `mod` and the Filesystem][2]**
* [in the current edition: Ch 14.02 — Publishing a Crate to Crates.io][3]
* <small>[In the first edition: Ch 3.25 — Crates and Modules][1]</small>


[1]: https://doc.rust-lang.org/1.30.0/book/first-edition/crates-and-modules.html
[2]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[3]: ch14-02-publishing-to-crates-io.html
