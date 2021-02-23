## Processing a Series of Items with Iterators

The iterator pattern allows you to perform some task on a sequence of items in
turn. An iterator is responsible for the logic of iterating over each item and
determining when the sequence has finished. When you use iterators, you don’t
have to reimplement that logic yourself.

In Rust, iterators are *lazy*, meaning they have no effect until you call
methods that consume the iterator to use it up. For example, the code in
Listing 13-13 creates an iterator over the items in the vector `v1` by calling
the `iter` method defined on `Vec<T>`. This code by itself doesn’t do anything
useful.

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/main.rs:here}}
```

<span class="caption">Listing 13-13: Creating an iterator</span>

Once we’ve created an iterator, we can use it in a variety of ways. In Listing
3-5 in Chapter 3, we used iterators with `for` loops to execute some code on
each item, although we glossed over what the call to `iter` did until now.

The example in Listing 13-14 separates the creation of the iterator from the
use of the iterator in the `for` loop. The iterator is stored in the `v1_iter`
variable, and no iteration takes place at that time. When the `for` loop is
called using the iterator in `v1_iter`, each element in the iterator is used in
one iteration of the loop, which prints out each value.

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

<span class="caption">Listing 13-14: Using an iterator in a `for` loop</span>

In languages that don’t have iterators provided by their standard libraries,
you would likely write this same functionality by starting a variable at index
0, using that variable to index into the vector to get a value, and
incrementing the variable value in a loop until it reached the total number of
items in the vector.

Iterators handle all that logic for you, cutting down on repetitive code you
could potentially mess up. Iterators give you more flexibility to use the same
logic with many different kinds of sequences, not just data structures you can
index into, like vectors. Let’s examine how iterators do that.

### The `Iterator` Trait and the `next` Method

All iterators implement a trait named `Iterator` that is defined in the
standard library. The definition of the trait looks like this:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

Notice this definition uses some new syntax: `type Item` and `Self::Item`,
which are defining an *associated type* with this trait. We’ll talk about
associated types in depth in Chapter 19. For now, all you need to know is that
this code says implementing the `Iterator` trait requires that you also define
an `Item` type, and this `Item` type is used in the return type of the `next`
method. In other words, the `Item` type will be the type returned from the
iterator.

The `Iterator` trait only requires implementors to define one method: the
`next` method, which returns one item of the iterator at a time wrapped in
`Some` and, when iteration is over, returns `None`.

We can call the `next` method on iterators directly; Listing 13-15 demonstrates
what values are returned from repeated calls to `next` on the iterator created
from the vector.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/lib.rs:here}}
```

<span class="caption">Listing 13-15: Calling the `next` method on an
iterator</span>

Note that we needed to make `v1_iter` mutable: calling the `next` method on an
iterator changes internal state that the iterator uses to keep track of where
it is in the sequence. In other words, this code *consumes*, or uses up, the
iterator. Each call to `next` eats up an item from the iterator. We didn’t need
to make `v1_iter` mutable when we used a `for` loop because the loop took
ownership of `v1_iter` and made it mutable behind the scenes.

Also note that the values we get from the calls to `next` are immutable
references to the values in the vector. The `iter` method produces an iterator
over immutable references. If we want to create an iterator that takes
ownership of `v1` and returns owned values, we can call `into_iter` instead of
`iter`. Similarly, if we want to iterate over mutable references, we can call
`iter_mut` instead of `iter`.

### Methods that Consume the Iterator

The `Iterator` trait has a number of different methods with default
implementations provided by the standard library; you can find out about these
methods by looking in the standard library API documentation for the `Iterator`
trait. Some of these methods call the `next` method in their definition, which
is why you’re required to implement the `next` method when implementing the
`Iterator` trait.

Methods that call `next` are called *consuming adaptors*, because calling them
uses up the iterator. One example is the `sum` method, which takes ownership of
the iterator and iterates through the items by repeatedly calling `next`, thus
consuming the iterator. As it iterates through, it adds each item to a running
total and returns the total when iteration is complete. Listing 13-16 has a
test illustrating a use of the `sum` method:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs:here}}
```

<span class="caption">Listing 13-16: Calling the `sum` method to get the total
of all items in the iterator</span>

We aren’t allowed to use `v1_iter` after the call to `sum` because `sum` takes
ownership of the iterator we call it on.

### Methods that Produce Other Iterators

Other methods defined on the `Iterator` trait, known as *iterator adaptors*,
allow you to change iterators into different kinds of iterators. You can chain
multiple calls to iterator adaptors to perform complex actions in a readable
way. But because all iterators are lazy, you have to call one of the consuming
adaptor methods to get results from calls to iterator adaptors.

Listing 13-17 shows an example of calling the iterator adaptor method `map`,
which takes a closure to call on each item to produce a new iterator. The
closure here creates a new iterator in which each item from the vector has been
incremented by 1. However, this code produces a warning:

<span class="filename">Filename: src/main.rs</span>

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-17/src/main.rs:here}}
```

<span class="caption">Listing 13-17: Calling the iterator adaptor `map` to
create a new iterator</span>

The warning we get is this:

```console
{{#include ../listings/ch13-functional-features/listing-13-17/output.txt}}
```

The code in Listing 13-17 doesn’t do anything; the closure we’ve specified
never gets called. The warning reminds us why: iterator adaptors are lazy, and
we need to consume the iterator here.

To fix this and consume the iterator, we’ll use the `collect` method, which we
used in Chapter 12 with `env::args` in Listing 12-1. This method consumes the
iterator and collects the resulting values into a collection data type.

In Listing 13-18, we collect the results of iterating over the iterator that’s
returned from the call to `map` into a vector. This vector will end up
containing each item from the original vector incremented by 1.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

<span class="caption">Listing 13-18: Calling the `map` method to create a new
iterator and then calling the `collect` method to consume the new iterator and
create a vector</span>

Because `map` takes a closure, we can specify any operation we want to perform
on each item. This is a great example of how closures let you customize some
behavior while reusing the iteration behavior that the `Iterator` trait
provides.

### Using Closures that Capture Their Environment

Now that we’ve introduced iterators, we can demonstrate a common use of
closures that capture their environment by using the `filter` iterator adaptor.
The `filter` method on an iterator takes a closure that takes each item from
the iterator and returns a Boolean. If the closure returns `true`, the value
will be included in the iterator produced by `filter`. If the closure returns
`false`, the value won’t be included in the resulting iterator.

In Listing 13-19, we use `filter` with a closure that captures the `shoe_size`
variable from its environment to iterate over a collection of `Shoe` struct
instances. It will return only shoes that are the specified size.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/lib.rs}}
```

<span class="caption">Listing 13-19: Using the `filter` method with a closure
that captures `shoe_size`</span>

The `shoes_in_size` function takes ownership of a vector of shoes and a shoe
size as parameters. It returns a vector containing only shoes of the specified
size.

In the body of `shoes_in_size`, we call `into_iter` to create an iterator
that takes ownership of the vector. Then we call `filter` to adapt that
iterator into a new iterator that only contains elements for which the closure
returns `true`.

The closure captures the `shoe_size` parameter from the environment and
compares the value with each shoe’s size, keeping only shoes of the size
specified. Finally, calling `collect` gathers the values returned by the
adapted iterator into a vector that’s returned by the function.

The test shows that when we call `shoes_in_size`, we get back only shoes
that have the same size as the value we specified.

### Creating Our Own Iterators with the `Iterator` Trait

We’ve shown that you can create an iterator by calling `iter`, `into_iter`, or
`iter_mut` on a vector. You can create iterators from the other collection
types in the standard library, such as hash map. You can also create iterators
that do anything you want by implementing the `Iterator` trait on your own
types. As previously mentioned, the only method you’re required to provide a
definition for is the `next` method. Once you’ve done that, you can use all
other methods that have default implementations provided by the `Iterator`
trait!

To demonstrate, let’s create an iterator that will only ever count from 1 to 5.
First, we’ll create a struct to hold some values. Then we’ll make this struct
into an iterator by implementing the `Iterator` trait and using the values in
that implementation.

Listing 13-20 has the definition of the `Counter` struct and an associated
`new` function to create instances of `Counter`:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/lib.rs}}
```

<span class="caption">Listing 13-20: Defining the `Counter` struct and a `new`
function that creates instances of `Counter` with an initial value of 0 for
`count`</span>

The `Counter` struct has one field named `count`. This field holds a `u32`
value that will keep track of where we are in the process of iterating from 1
to 5. The `count` field is private because we want the implementation of
`Counter` to manage its value. The `new` function enforces the behavior of
always starting new instances with a value of 0 in the `count` field.

Next, we’ll implement the `Iterator` trait for our `Counter` type by defining
the body of the `next` method to specify what we want to happen when this
iterator is used, as shown in Listing 13-21:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-21/src/lib.rs:here}}
```

<span class="caption">Listing 13-21: Implementing the `Iterator` trait on our
`Counter` struct</span>

We set the associated `Item` type for our iterator to `u32`, meaning the
iterator will return `u32` values. Again, don’t worry about associated types
yet, we’ll cover them in Chapter 19.

We want our iterator to add 1 to the current state, so we initialized `count`
to 0 so it would return 1 first. If the value of `count` is less than 5, `next`
will increment `count` and return the current value wrapped in `Some`. Once
`count` is 5, our iterator will stop incrementing `count` and always return
`None`.

#### Using Our `Counter` Iterator’s `next` Method

Once we’ve implemented the `Iterator` trait, we have an iterator! Listing 13-22
shows a test demonstrating that we can use the iterator functionality of our
`Counter` struct by calling the `next` method on it directly, just as we did
with the iterator created from a vector in Listing 13-15.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

<span class="caption">Listing 13-22: Testing the functionality of the `next`
method implementation</span>

This test creates a new `Counter` instance in the `counter` variable and then
calls `next` repeatedly, verifying that we have implemented the behavior we
want this iterator to have: returning the values from 1 to 5.

#### Using Other `Iterator` Trait Methods

We implemented the `Iterator` trait by defining the `next` method, so we
can now use any `Iterator` trait method’s default implementations as defined in
the standard library, because they all use the `next` method’s functionality.

For example, if for some reason we wanted to take the values produced by an
instance of `Counter`, pair them with values produced by another `Counter`
instance after skipping the first value, multiply each pair together, keep only
those results that are divisible by 3, and add all the resulting values
together, we could do so, as shown in the test in Listing 13-23:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-23/src/lib.rs:here}}
```

<span class="caption">Listing 13-23: Using a variety of `Iterator` trait
methods on our `Counter` iterator</span>

Note that `zip` produces only four pairs; the theoretical fifth pair `(5,
None)` is never produced because `zip` returns `None` when either of its input
iterators return `None`.

All of these method calls are possible because we specified how the `next`
method works, and the standard library provides default implementations for
other methods that call `next`.
