# Fearless Concurrency

Handling concurrent programming safely and efficiently is another of Rust’s
major goals. *Concurrent programming*, where different parts of a program
execute independently, and *parallel programming*, where different parts of a
program are executing at the same time, are becoming increasingly important as
more computers have multiple processors to take advantage of. Historically,
programming in these contexts has been difficult and error prone: Rust hopes to
change that.

Initially, the Rust team thought that ensuring memory safety and preventing
concurrency problems were two separate challenges to be solved with different
methods. Over time, they discovered that the ownership and type systems are a
powerful set of tools to help in dealing with both memory safety *and*
concurrency problems! By leveraging ownership and type checking, many
concurrency errors are *compile time* errors in Rust, rather than runtime
errors. Rather than spending lots of time trying to reproduce the exact
circumstances under which a runtime concurrency bug occurs, incorrect code will
refuse to compile with an error explaining the problem. This lets you fix your
code while you’re working on it, rather than potentially after it’s been
shipped to production. We’ve nicknamed this aspect of Rust *fearless
concurrency*. Fearless concurrency allows you to write code that’s free of
subtle bugs and is easy to refactor without introducing new bugs.

<!-- Can you say explicitly why making concurrency issues compile time errors
rather than runtime errors is an advantage? -->
<!-- I feel like we've explained this a few times now, but I suppose since the
advantage should be greater in concurrent code it's worth saying again /Carol
-->

> Note: we’ll be referring to many of the problems here as *concurrent* rather
> than being more precise by saying *concurrent and/or parallel*, for
> simplicity’s sake. If this were a book specifically about concurrency and/or
> parallelism, we’d be sure to be more specific. For this chapter, please
> mentally substitute *concurrent and/or parallel* whenever we say *concurrent*.

<!-- I'm not sure what you mean about languages being strongly opinionated over
these issues and what kind of strategy that is, below, can you be more
specific? -->
<!-- I've added an example and elaborated on the strategy we're talking about
here. /Carol -->

Many languages are strongly opinionated about the solutions they offer for
dealing with concurrent problems. For example, Erlang has elegant functionality
for message passing concurrency, but only obscure ways to share state between
threads. Only supporting a subset of possible solutions is a reasonable
strategy for higher-level languages to take, because a higher-level language
promises benefits from giving up some control in order to gain abstractions.
However, lower-level languages are expected to provide the solution with the
best performance in any given situation, and have fewer abstractions over the
hardware. Rust, therefore, gives us a variety of tools for modeling your
problems in whatever way is appropriate for your situation and requirements.

Here’s what we’ll cover in this chapter:

* How to create threads to run multiple pieces of code at the same time
* *Message passing* concurrency, where channels are used to send messages
  between threads.
* *Shared state* concurrency, where multiple threads have access to some piece
  of data.
* The `Sync` and `Send` traits, which extend Rust’s concurrency guarantees to
  user-defined types as well as types provided by the standard library.
