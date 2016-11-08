# The Rust Programming Language

## Getting started

- [Introduction](ch01-00-introduction.md)
    - [Installation](ch01-01-installation.md)
    - [Hello, World!](ch01-02-hello-world.md)

- [Guessing Game Tutorial](ch02-00-guessing-game-tutorial.md)

- [Common Programming Concepts](ch03-00-common-programming-concepts.md)
    - [Variables and Mutability](ch03-01-variables-and-mutability.md)
    - [Data Types](ch03-02-data-types.md)
    - [How Functions Work](ch03-03-how-functions-work.md)
    - [Comments](ch03-04-comments.md)
    - [Control Flow](ch03-05-control-flow.md)

- [Understanding Ownership](ch04-00-understanding-ownership.md)
    - [What is Ownership?](ch04-01-what-is-ownership.md)
    - [References & Borrowing](ch04-02-references-and-borrowing.md)
    - [Slices](ch04-03-slices.md)

- [Structs](ch05-00-structs.md)
    - [Method Syntax](ch05-01-method-syntax.md)

- [Enums](ch06-00-enums.md)
    - [Option](ch06-01-option.md)
    - [Match](ch06-02-match.md)
    - [`if let`](ch06-03-if-let.md)

## Basic Rust Literacy

- [Modules](ch07-00-modules.md)
    - [`mod` and the Filesystem](ch07-01-mod-and-the-filesystem.md)
    - [Controlling Visibility with `pub`](ch07-02-controlling-visibility-with-pub.md)
    - [Importing Names with `use`](ch07-03-importing-names-with-use.md)

- [Fundamental Collections](ch08-00-fundamental-collections.md)
    - [Vectors](ch08-01-vectors.md)
    - [Strings](ch08-02-strings.md)
    - [Hash Maps](ch08-03-hash-maps.md)

- [Error Handling](ch09-00-error-handling.md)
    - [Unrecoverable Errors with `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
    - [Recoverable Errors with `Result`](ch09-02-recoverable-errors-with-result.md)
    - [To `panic!` or Not To `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [Generics](ch10-00-generics.md)
    - [Syntax](ch10-01-syntax.md)
    - [Traits](ch10-02-traits.md)
    - [Lifetime syntax](ch10-03-lifetime-syntax.md)

- [I/O]()
    - [`Read` & `Write`]()
    - [`std::fs`]()
    - [`std::path`]()
    - [`std::env`]()

- [Testing](chXX-00-testing.md)
    - [Unit testing](chXX-01-unit-testing.md)
    - [Integration testing](chXX-02-integration-testing.md)
    - [Documentation Tests](chXX-03-documentation-tests.md)

- [Debugging]()

## Thinking in Rust

- [Composition]()
    - [Instead of Inheritance]()
    - [Trait Objects?]()

- [Creating a Library]()
    - [Cargo]()
    - [Crates.io]()
    - [Organizing your Public API](chYY-YY-public-api.md)
    - [Documentation](chYY-YY-documentation.md)
    - [Workspaces and Multiple Related Crates](chYY-YY-workspaces.md)

- [Closures]()

- [Zero-cost Abstractions]()
    - [Iterators as a Case Study]()

- [Smart Pointers]()
    - [`Box<T>`]()
    - [`Rc<T>`]()
    - [`Cell`]()
    - [`RefCell`]()
    - [Interior Mutability]()

- [Concurrency]()
    - [Threads]()
    - [`Send` & `Sync`]()
    - [`Arc<T>`]()
    - [`Mutex<T>`]()
    - [`Channels`]()

## Advanced Topics

- [Patterns](chXX-patterns.md)

- [More Lifetimes]()

- [Unsafe Rust]()
    - [Raw Pointers]()
    - [`transmute`]()

- [Foreign Function Interface]() (?)
    - [Conditional Compilation]()
    - [Bindings to C]()
    - [Using Rust from Other Languages]()
    - [`static`]()

- [Advanced Type System Features]() (perhaps called "Advanced Traits"?)
    - [Associated Types]()
    - [Trait Objects]() (might be incorporated into the Composition chapter?)
    - [UFCS]()
    - [Coherence]()

- [Macros]()
    - [Writing Your Own Macros]()

- [Nightly Rust]() (?)
    - [Nightly Features]()
    - [How to Find Out About Nightly Features]()

- [Appendix](appendix-00.md)
    - [Keywords](appendix-01-keywords.md)
    - [Operators](appendix-02-operators.md)
    - [Derivable Traits](appendix-03-derivable-traits.md)
