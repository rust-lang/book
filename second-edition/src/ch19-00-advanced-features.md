<!-- This is a long chapter! I was trying to consider whether to split it, and
if so where --- the only solution I could come up with was to split it into the
five main subjects: Unsafe, Lifetimes, Traits, Types, and Functions and
Closures. However, I'm not convinced that's ideal, so I thought we might
include a ToC at the top of this chapter in print so the reader can use it as a
reference when they come across something they can't figure out. What do you
think? -->
<!-- A ToC to make this chapter more easily used as a reference sounds okay,
would it be redundant with the ToC at the beginning of the whole book though?
Or would this ToC be more detailed than the beginning of the book? Would it
just be adding page numbers to the bullet points after the first paragraph?
We're curious about implementation :) /Carol -->

# Advanced Features

We’ve come a long way! By now, you’ve learned 99% of the things you’ll need to
know when writing Rust. Before we do one more project in Chapter 20, let’s talk
about a few things you may run into that last 1% of the time. Feel free to use
this chapter as a reference for when you run into something unknown in the
wild; the features you’ll learn to use here are useful in very specific
situations. We don’t want to leave these features out, but you won’t find
yourself reaching for them often.

In this chapter, we’re going to cover:

* Unsafe Rust: for when you need to opt out of some of Rust’s guarantees and
  make yourself responsible for upholding the guarantees instead
* Advanced Lifetimes: syntax for complex lifetime situations
* Advanced Traits: Associated Types, default type parameters, fully qualified
  syntax, supertraits, and the newtype pattern in relation to traits
* Advanced Types: some more about the newtype pattern, type aliases, the
  “never” type, and dynamically sized types
* Advanced Functions and Closures: function pointers and returning closures

It’s a panoply of Rust features with something for everyone! Let’s dive in!
