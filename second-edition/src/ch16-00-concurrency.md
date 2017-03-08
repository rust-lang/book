# Fearless Concurrency

While Rust has a strong focus on memory safety, it's more than only that.
There's one problem in particular that's becoming more and more important, yet
many languages are ill-equipped to deal with: concurrency and parallelism.
Rust's goals have always included making concurrent and/or parallel code
easier.

Originally, we thought that these were two separate problems, to be solved with
different methods, but over time, we discovered that ownership provides a
powerful set of tools for dealing with these problems too. In short, many
concurrency and/or parallelism errors are *compile time* errors in Rust,
rather than runtime errors. We've nicknamed this aspect of Rust 'fearless
concurrency', as it not only allows you to have confidence that your code
is free of subtle bugs, but also lets you refactor this kind of code easily,
without worrying about introducing new bugs.

> Side note: given that the slogan is 'fearless concurrency', we'll be
> referring to many of the problems here as 'concurrent' ones rather than
> 'concurrent and/or parallel' ones, for simplicity's sake. These two kinds
> of problems are similar, but technically different. If this were a book
> specifically about concurrency and/or parallelism, we'd be sure to be
> more specific, but for this chapter, we're going to fudge it. Sorry about
> that, pedants!

Many languages that give you tools to deal with concurrent problems are
strongly opinionated about them. That's a very reasonable strategy, but
lower-level languages don't have that luxury. Rust, therefore, gives us a
variety of tools for modeling our problems in whatever way we feel is
appropriate.

Here's what we'll cover in this chapter:

* *Message passing* concurrency, where channels are used to send messages
  between threads.
* *Shared state* concurrency, where multiple threads have access to some piece
  of data.
* The `Sync` and `Send` traits, which allow Rust's concurrency guarantees to be
  *extensible*; they aren't limited to the built-in types.
