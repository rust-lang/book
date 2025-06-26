## Comparing Performance: Loops vs. Iterators

To determine whether to use loops or iterators, you need to know which
implementation is faster: the version of the `search` function with an explicit
`for` loop or the version with iterators.

We ran a benchmark by loading the entire contents of _The Adventures of
Sherlock Holmes_ by Sir Arthur Conan Doyle into a `String` and looking for the
word _the_ in the contents. Here are the results of the benchmark on the
version of `search` using the `for` loop and the version using iterators:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

The two implementations have similar performance! We won’t explain the
benchmark code here because the point is not to prove that the two versions
are equivalent but to get a general sense of how these two implementations
compare performance-wise.

For a more comprehensive benchmark, you should check using various texts of
various sizes as the `contents`, different words and words of different lengths
as the `query`, and all kinds of other variations. The point is this:
iterators, although a high-level abstraction, get compiled down to roughly the
same code as if you’d written the lower-level code yourself. Iterators are one
of Rust’s _zero-cost abstractions_, by which we mean that using the abstraction
imposes no additional runtime overhead. This is analogous to how Bjarne
Stroustrup, the original designer and implementor of C++, defines
_zero-overhead_ in “Foundations of C++” (2012):

> In general, C++ implementations obey the zero-overhead principle: What you
> don’t use, you don’t pay for. And further: What you do use, you couldn’t hand
> code any better.

In many cases, Rust code using iterators compiles to the same assembly you’d
write by hand. Optimizations such as loop unrolling and eliminating bounds
checking on array access apply and make the resultant code extremely efficient.
Now that you know this, you can use iterators and closures without fear! They
make code seem like it’s higher level but don’t impose a runtime performance
penalty for doing so.

## Summary

Closures and iterators are Rust features inspired by functional programming
language ideas. They contribute to Rust’s capability to clearly express
high-level ideas at low-level performance. The implementations of closures and
iterators are such that runtime performance is not affected. This is part of
Rust’s goal to strive to provide zero-cost abstractions.

Now that we’ve improved the expressiveness of our I/O project, let’s look at
some more features of `cargo` that will help us share the project with the
world.
