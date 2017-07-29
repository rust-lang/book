# Fearless Concurrency

Ensuring memory safety isn’t Rust’s only goal: being a language that is better
equipped to handle concurrent and parallel programming has always been another
major goal of Rust. *Concurrent programming*, where different parts of a
program execute independently, and *parallel programming*, where different
parts of a program are executing at the same time, are becoming more important
as more computers have multiple processors for our programs to take advantage
of. Historically, programming in these contexts has been difficult and error
prone: Rust hopes to change that.

Originally, we thought that memory safety and preventing concurrency problems
were two separate challenges to be solved with different methods. However, over
time, we discovered that ownership and the type system are a powerful set of
tools that help in dealing with both memory safety *and* concurrency problems!
By leveraging ownership and type checking, many concurrency errors are *compile
time* errors in Rust, rather than runtime errors. We’ve nicknamed this aspect
of Rust *fearless concurrency*. Fearless concurrency means Rust not only allows
you to have confidence that your code is free of subtle bugs, but also lets you
refactor this kind of code easily without worrying about introducing new bugs.

> Note: given that Rust’s slogan is *fearless concurrency*, we’ll be referring
> to many of the problems here as *concurrent* rather than being more precise
> by saying *concurrent and/or parallel*, for simplicity’s sake. If this were a
> book specifically about concurrency and/or parallelism, we’d be sure to be
> more specific. For this chapter, please mentally substitute
> *concurrent and/or parallel* whenever we say *concurrent*.

Many languages are strongly opinionated about the solutions they offer you to
deal with concurrent problems. That’s a very reasonable strategy, especially
for higher-level languages, but lower-level languages don’t have that luxury.
Lower-level languages are expected to enable whichever solution would provide
the best performance in a given situation, and they have fewer abstractions
over the hardware. Rust, therefore, gives us a variety of tools for modeling
our problems in whatever way is appropriate for our situation and requirements.

Here’s what we’ll cover in this chapter:

* How to create threads to run multiple pieces of code at the same time
* *Message passing* concurrency, where channels are used to send messages
  between threads.
* *Shared state* concurrency, where multiple threads have access to some piece
  of data.
* The `Sync` and `Send` traits, which allow Rust’s concurrency guarantees to be
  extended to user-defined types as well as types provided by the standard
  library.
