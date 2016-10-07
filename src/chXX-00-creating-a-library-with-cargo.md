# Creating a Library with Cargo

We've used Cargo many times in this book already. `cargo build` and `cargo run`
are core to building and running our Rust projects. But Cargo is a fully
featured tool, and it has a lot to offer. In this chapter, we'll dig more into
the features of Cargo.

## Why does Cargo exist?

If we already have `rustc`, why do we need Cargo? While `rustc` knows how to
compile Rust code, real projects have more complexity. This separation means
that we can let `rustc` stay focused on doing one thing, and doing it well:
compiling Rust code.

Cargo, on the other hand, allows Rust projects to declare their various
dependencies and ensure that we’ll always get a repeatable build.

To accomplish this goal, Cargo does four things:

* Introduces two metadata files with various bits of package information.
* Fetches and builds our package’s dependencies.
* Invokes `rustc` or another build tool with the correct parameters to build our package.
* Introduces conventions to make working with Rust packages easier.

Cargo's job is to manage all of this additional complexity around building
real-world projects. As such, Cargo has many features. We won't cover them all
here, but hit the big points. Cargo's documentation contains even more options.

## Convention over configuration

A core principle of Cargo is "convention over configuration." What this means
is that Cargo chooses certain defaults for your package automatically. Even
though Cargo has conventions, it still allows you to configure things if you
need something other than the default for some reason.

For example, for a library package, the crate root is conventionally located
at `src/lib.rs`. If we wanted to, we could configure Cargo to choose another
file at a different location, but by default, that's where it goes.

This principle means that most Rust projects work in a similar way, which helps
when you get started with a new codebase. Once you learn Cargo's conventions,
you can get going very quickly on a new codebase.

## Package layout

### Summary

A quick summary:

```shell
.
├── Cargo.lock
├── Cargo.toml
├── benches
│   └── large-input.rs
├── examples
│   └── simple.rs
├── src
│   ├── bin
│   │   └── another_executable.rs
│   ├── lib.rs
│   └── main.rs
├── target
└── tests
    └── some-integration-tests.rs
```

## `Cargo.toml` and `Cargo.lock`

`Cargo.toml` and `Cargo.lock` serve two different purposes. Before we talk
about them, here’s a summary:

* `Cargo.toml` is about describing your dependencies in a broad sense, and is written by you.
* `Cargo.lock` contains exact information about your dependencies. It is maintained by Cargo and should not be manually edited.

If you’re building a library that other projects will depend on, put
`Cargo.lock` in your `.gitignore`. If you’re building an executable like a
command-line tool or an application, check `Cargo.lock` into `git`. If you're
curious about why that is, see ["Why do binaries have `Cargo.lock` in version
control, but not libraries?" in the
FAQ](faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries).

Let’s dig in a little bit more.

`Cargo.toml` is a **manifest** file in which we can specify a bunch of
different metadata about our project. For example, we can say that we depend
on another project:

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git" }
```

This project has a single dependency, on the `rand` library. We’ve stated in
this case that we’re relying on a particular Git repository that lives on
GitHub. Since we haven’t specified any other information, Cargo assumes that
we intend to use the latest commit on the `master` branch to build our project.

Sound good? Well, there’s one problem: If you build this project today, and
then you send a copy to me, and I build this project tomorrow, something bad
could happen. There could be more commits to `rand` in the meantime, and my
build would include new commits while yours would not. Therefore, we would
get different builds. This would be bad because we want reproducible builds.

We could fix this problem by putting a `rev` line in our `Cargo.toml`:

```toml
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git", rev = "9f35b8e" }
```

Now our builds will be the same. But there’s a big drawback: now we have to
manually think about SHA-1s every time we want to update our library. This is
both tedious and error prone.

Enter the `Cargo.lock`. Because of its existence, we don’t need to manually
keep track of the exact revisions: Cargo will do it for us. When we have a
manifest like this:

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git" }
```

Cargo will take the latest commit and write that information out into our
`Cargo.lock` when we build for the first time. That file will look like this:

```toml
[root]
name = "hello_world"
version = "0.1.0"
dependencies = [
 "rand 0.1.0 (git+https://github.com/rust-lang-nursery/rand.git#9f35b8e439eeedd60b9414c58f389bdc6a3284f9)",
]

[[package]]
name = "rand"
version = "0.1.0"
source = "git+https://github.com/rust-lang-nursery/rand.git#9f35b8e439eeedd60b9414c58f389bdc6a3284f9"

```

You can see that there’s a lot more information here, including the exact
revision we used to build. Now when you give your project to someone else,
they’ll use the exact same SHA, even though we didn’t specify it in our
`Cargo.toml`.

When we’re ready to opt in to a new version of the library, Cargo can
re-calculate the dependencies and update things for us:

```shell
$ cargo update           # updates all dependencies
$ cargo update -p rand  # updates just “rand”
```

This will write out a new `Cargo.lock` with the new version information. Note
that the argument to `cargo update` is actually a
[Package ID Specification](pkgid-spec.html) and `rand` is just a short
specification.

## Working on an existing Cargo package

## `cargo install`

## Extending Cargo with subcommands
