## Comparing Performance: Loops versus Iterators

To determine which to use, we need to know which version of our `search`
functions is faster: the version with an explicit `for` loop or the version
with iterators.

We ran a benchmark by loading the entire contents of “The Adventures of
Sherlock Holmes” by Sir Arthur Conan Doyle into a `String` and looking for the
word “the” in the contents. Here were the results of the benchmark on the
version of `search` using the `for` loop and the version using iterators:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

The iterator version ended up slightly faster! We’re not going to go through
the benchmark code here, as the point is not to prove that they’re exactly
equivalent, but to get a general sense of how these two implementations compare
performance-wise.

For a more comprehensive benchmark, you’d want to check various texts of
various sizes, different words, words of different lengths, and all kinds of
other variations. The point is this: iterators, while a high-level abstraction,
get compiled down to roughly the same code as if you’d written the lower-level
code yourself. Iterators are one of Rust’s *zero-cost* *abstractions*, by which
we mean using the abstraction imposes no additional runtime overhead, in the
same way that Bjarne Stroustrup, the original designer and implementor of C++,
defines *zero-overhead*:

> In general, C++ implementations obey the zero-overhead principle: What you
> don’t use, you don’t pay for. And further: What you do use, you couldn’t hand
> code any better.
>
> - Bjarne Stroustrup “Foundations of C++”

As another example, here is some code taken from an audio decoder. The decoding
algorithm uses the linear prediction mathematical operation to estimate future
values based on a linear function of the previous samples.

This code uses an iterator chain to do some math on three variables in scope: a
`buffer` slice of data, an array of 12 `coefficients`, and an amount by which
to shift data in `qlp_shift`. We’ve declared the variables within this example
but not given them any values; while this code doesn’t have much meaning
outside of its context, it’s still a concise, real-world example of how Rust
translates high-level ideas to low-level code:

```rust,ignore
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

In order to calculate the value of `prediction`, this code iterates through
each of the 12 values in `coefficients` and uses the `zip` method to pair the
coefficient values with the previous 12 values in `buffer`. Then, for each
pair, we multiply the values together, sum all the results, and shift the bits
in the sum `qlp_shift` bits to the right.

Calculations in applications like audio decoders often prioritize performance
most highly. Here, we’re creating an iterator, using two adaptors, then
consuming the value. What assembly code would this Rust code compile to? Well,
as of this writing, it compiles down to the same assembly you’d write by hand.
There’s no loop at all corresponding to the iteration over the values in
`coefficients`: Rust knows that there are twelve iterations, so it “unrolls”
the loop. *Unrolling* is an optimization that removes the overhead of the loop
controlling code and instead generates repetitive code for each iteration of
the loop.

All of the coefficients get stored in registers, which means it’s very fast to
access the values. There are no bounds checks on the array access at runtime.
All these optimizations Rust is able to apply make the resulting code extremely
efficient.

Now that you know this, go use iterators and closures without fear! They make
code feel higher-level, but don’t impose a runtime performance penalty for
doing so.

## Summary

Closures and iterators are Rust features inspired by functional programming
language ideas. They contribute to Rust’s ability to clearly express high-level
ideas, at low level performance. The implementations of closures and iterators
are such that runtime performance is not affected. This is part of Rust’s goal
to strive to provide zero-cost abstractions.

Now that we’ve improved the expressiveness of our I/O project, let’s look at
some more features of `cargo` that would help us get ready to share the project
with the world.
