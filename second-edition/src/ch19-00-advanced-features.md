# Advanced Features

We've come a long way! By now, we've learned 99% of the things you'll need to
know when writing Rust. Before we do one more project in Chapter 20, let's talk
about a few things that you may run into that last 1% of the time. Feel free to
skip this chapter and come back to it once you run into these things in the
wild; the features we'll learn to use here are useful in very specific
situations. We don't want to leave these features out, but you won't find
yourself reaching for them often.

In this chapter, we're going to cover:

* Unsafe Rust: for when you need to opt out of some of Rust's guarantees and
  tell the compiler that you will be responsible for upholding the guarantees
  instead
* Advanced Lifetimes: Additional lifetime syntax for complex situations
* Advanced Traits: Associated Types, coherence, and disambiguation
