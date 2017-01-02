# Functional Language features in Rust - Iterators and Closures

## Closures

### What is a closure

In programming languages, a closure is a lot like a function. Like a function, closures contain code
that is executed when the closure is called. The main difference, besides syntax, between closures
and functions is that closures have *capture*. What this means is that a closure can use variables
in its surrounding scope. Consider the following code:

```rust,ignore
fn main() {
  let x = 4;
  fn is_x(z: i32) -> bool {
    z == x
  }
  
  let y = 4;
  is_x(y);
}
```

Here, the function `is_x` is trying to use the x from `main`'s scope. This doesn't work, however,
giving the error

```text
error: can't capture dynamic environment in a fn item; use the || { ... } closure form instead [E0434]
z == x
     ^
```

This error message is saying that functions can't *capture* -- only closures can. So, let's try
making the `is_x` function into a closure:
```rust
fn main() {
  let x = 4;
  let is_x = |z: i32| -> bool {
    z == x
  };

  let y = 4;
  is_x(y);
}
```

We can see here that the syntax for defining a closure is `|input arguments| -> output { code }`.
This is very similar to a function definition with some major differences. The name is not a part of
the closure -- by default, closures are unnamed. We also use vertical bars (`|`) instead of
parentheses to define the arguments to the closure, and specifying the types of the inputs and
outputs is optional if Rust can figure them out. Finally, closures are expressions rather than
statements. You can see here that we could assign the closure to a variable, and needed to terminate
the line with a semicolon. Frequently, closures are defined and passed into functions without ever
giving them names.

This closure is capturing the x variable. But how is it doing that? What if we define a closure in a
function and return it? What if we change x between where the closure is defined and where it's
executed?

By default, closures capture the variables by reference. This means that closures cannot outlive
variables that they capture. If we try to compile the following code:
```rust,ignore
fn main() {
  let closure;
  {
    let x = 4;
    closure = ||{ x }; // Closure that takes no arguments and returns x
  }
}
```
we get an error because `x` does not live long enough.

We can make closures move (or copy, for types declared Copy) their values in by using the `move`
keyword:
```rust
fn main() {
  let closure;
  {
    let x = 4;
    closure = move ||{ x }; // Closure that takes no arguments and returns x
  }
}
```

### `Fn` traits

It's clear that closurse more than functions, because closures have to keep track of what variables
they've captured. Closures are essentially functions that come with a struct that includes their
captured variables (or references). This means that every closure is a different type. If we want a
function to take a closure as an argument, we can't simply talk about a "closure" type, because each
one is a different type. The way that Rust manages this is with traits. There are three traits that
can be automatically applied to each closure, depending on what the closure is doing: `FnOnce`,
`FnMut`, and `Fn`. These traits derive from each other, so if a type is `FnMut`, it must also be
`FnOnce`, and if a type is `Fn`, it must be both `FnMut` and `FnOnce`. Closures automatically derive
from the ones that are appropriate, and you cannot currently derive them for your custom types.

If you want to write a function such as `map` that takes in a function, you should almost always
take in one of the `Fn` traits. The `FnOnce` trait defines a function `call_once` that consumes
`self`. It's the most general option, and if your function will only call the given function once,
it should take in an `FnOnce`. `FnMut` is the next most general, since its `call_mut` function takes
`self` by mutable reference, so you need a mutable reference to the closure to call it. `Fn` is the
most specific of these, but you only need a immutable reference to call a function that is `Fn`.

All functions and closures implement `FnOnce`. If a closure takes its variables by reference rather
than by move, then it also implements `FnMut`. If the closure modifies none of its captures, then it
also implements `Fn`. All functions also implement `Fn` because they don't capture at all.

## Iterators

Iterators are types that implement the `Iterator` trait. Iterators are designed to return a sequence
of values by repetedly calling their `next()` method. Often, these values come from a data structure
such as `Vec`. Iterators are powerful, however, because they can be used for more than that. For
example, you can have an infinite iterator -- one whose `next()` method never returns `None`. There
are also functions on iterators like `map`, which applies a function to each value in the iterators
as they're requested. `map` has low memory usage because it only applies the function as elements
are requested, so the whole sequence does not need to build up.

### Iterator & for loop

`for` loops are usually said to "iterate" over things, so it makes perfect sense that in Rust, for
loops use iterators. Specifically, you can say `for x in y` if `y` implements the `IntoIterator`
trait with the `into_iter` method. This method consumes `self` and (for data structures) returns an
iterator that returns the values in the data structure.

Common `IntoIterator` types are data structures and ranges such as `0..10`. Thus, when you say `for
i in 0..10`, you are creating a `Range` that knows its start and end, then calling `into_iter` on it
to convert it to an iterator, and then repeatedly calling `next()` on that iterator to get the
numbers 0, 1, ..., 9.

### Iterators are Lazy

An important and very useful fact about iterators is that they are _lazy_. This means that, in
general, iterators don't look at the things they're iterating over until they are about to return
it. This is very useful when your iterator will return a large (or perhaps even infinite!) sequence.
Another consequence of this is that you must be careful when using the `map` function with a closure
that mutates things. The closure will not be executed on any elements until the resulting iterator
is consumed.

Iterator adapters are iterators that are based off of other iterators. A simple iterator adaptor in
the standard library is the [take](https://doc.rust-lang.org/std/iter/struct.Take.html) adaptor.
This iterator adaptor contains a counter initialized with a user-defined number and another
iterator. When its `next()` method is called, it decrements the count, and if it's 0, it returns
`None`. Otherwise, it returns `next()` of its contained iterator. In this way, we _adapt_ the inner
iterator to only return the first `n` elements rather than everything.

### Implementing the Iterator trait

Talk about using Associated Types here, foreshadow to advanced type systems
chapter about why this is a different thing than normal

## ??? How does this improve the I/O project from Chapter 12

Does this get woven into the above sections?

## Summary: Performance

### Iterators compile down to ASM == for loop

Most complicated chain of iterator functions that compile down to the same ASM as a for loop

### Representation: Closures are a Struct

Closures don't have any further performance penalty over regular fn calls

