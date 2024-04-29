# Advanced Features

By now, you’ve learned the most commonly used parts of the Rust programming
language. Before we do one more project in Chapter 20, we’ll look at a few
aspects of the language you might run into every once in a while, but may not
use every day. You can use this chapter as a reference for when you encounter
any unknowns. The features covered here are useful in very specific situations.
Although you might not reach for them often, we want to make sure you have a
grasp of all the features Rust has to offer.

In this chapter, we’ll cover:

* Unsafe Rust: how to opt out of some of Rust’s guarantees and take
  responsibility for manually upholding those guarantees
* Advanced traits: associated types, default type parameters, fully qualified
  syntax, supertraits, and the newtype pattern in relation to traits
* Advanced types: more about the newtype pattern, type aliases, the never type,
  and dynamically sized types
* Advanced functions and closures: function pointers and returning closures
* Macros: ways to define code that defines more code at compile time

It’s a panoply of Rust features with something for everyone! Let’s dive in!
