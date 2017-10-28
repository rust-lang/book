## Processing a Series of Items with Iterators

The iterator pattern allows you to perform some task on a sequence of items in
turn. An *iterator* is responsible for the logic of iterating over each item
and determining when the sequence has finished. When we use iterators, we don’t
have to reimplement that logic ourselves.

In Rust, iterators are *lazy*, meaning they have no effect until we call
methods that consume the iterator to use it up. For example, the code in
Listing 13-13 creates an iterator over the items in the vector `v1` by calling
the `iter` method defined on `Vec`. This code by itself doesn’t do anything
useful:

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

<span class="caption">Listing 13-13: Creating an iterator</span>

Once we’ve created an iterator, we can choose to use it in a variety of ways.
In Listing 3-6 from Chapter 3, we actually used iterators with `for` loops to
execute some code on each item, though we glossed over what the call to `iter`
did until now.

The example in Listing 13-14 separates the creation of the iterator from the
use of the iterator in the `for` loop. The iterator is stored in the `v1_iter`
variable, and no iteration takes place at that time. Once the `for` loop is
called using the iterator in `v1_iter`, then each element in the iterator is
used in one iteration of the loop, which prints out each value:

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

<span class="caption">Listing 13-14: Making use of an iterator in a `for`
loop</span>

In languages that don’t have iterators provided by their standard libraries, we
would likely write this same functionality by starting a variable at index 0,
using that variable to index into the vector to get a value, and incrementing
the variable value in a loop until it gets to the total number of items in the
vector.

Iterators take care of all of that logic for us, cutting down on repetitive
code we could potentially mess up. Iterators give us more flexibility to use
the same logic with many different kinds of sequences, not just data structures
we can index into like vectors. Let’s see how iterators do that.

### The `Iterator` trait and the `next` method

Iterators all implement a trait named `Iterator` that is defined in the
standard library. The definition of the trait looks like this:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

You’ll notice some new syntax that we haven’t covered yet: `type Item` and
`Self::Item`, which are defining an *associated type* with this trait. We’ll
talk about associated types in depth in Chapter 19, but for now, all you need
to know is that this code says implementing the `Iterator` trait requires that
you also define an `Item` type, and this `Item` type is used in the return type
of the `next` method. In other words, the `Item` type will be the type returned
from the iterator.

The `Iterator` trait only requires implementors to define one method: the
`next` method, which returns one item of the iterator at a time wrapped in
`Some` and, when iteration is over, it returns `None`.

We can call the `next` method on iterators directly; Listing 13-15 demonstrates
what values are returned from repeated calls to `next` on the iterator created
from the vector:

<span class="filename">Filename: src/lib.rs</span>

```rust,test_harness
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

<span class="caption">Listing 13-15: Calling the `next` method on an
iterator</span>

Note that we needed to make `v1_iter` mutable: calling the `next` method on an
iterator changes state that keeps track of where it is in the sequence. Put
another way, this code *consumes*, or uses up, the iterator. Each call to
`next` eats up an item from the iterator. We didn’t need to make `v1_iter`
mutable when we used a `for` loop because the loop took ownership of `v1_iter`
and made it mutable behind the scenes.

Also note that the values we get from the calls to `next` are immutable
references to the values in the vector. The `iter` method produces an iterator
over immutable references. If we want to create an iterator that takes
ownership of `v1` and returns owned values, we can call `into_iter` instead of
`iter`. Similarly, if we want to iterate over mutable references, we can call
`iter_mut` instead of `iter`.

### Methods in the `Iterator` Trait that Consume the Iterator

The `Iterator` trait has a number of different methods with default
implementations provided for us by the standard library; you can find out all
about these methods by looking in the standard library API documentation for
the `Iterator` trait. Some of these methods call the `next` method in their
definition, which is why we’re required to implement the `next` method when
implementing the `Iterator` trait.

Methods that call `next` are called *consuming adaptors*, because calling them
uses up the iterator. One example is the `sum` method, which takes ownership of
the iterator and iterates through the items by repeatedly calling `next`, thus
consuming the iterator. As it iterates through, it adds each item to a running
total and returns the total when iteration is complete. Listing 13-16 has a
test illustrating a use of the `sum` method:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

<span class="caption">Listing 13-16: Calling the `sum` method to get the total
of all items in the iterator</span>

We aren’t allowed to use `v1_iter` after the call to `sum` since `sum` takes
ownership of the iterator we call it on.

### Methods in the `Iterator` Trait that Produce Other Iterators

Other methods defined on the `Iterator` trait, known as *iterator adaptors*,
allow us to change iterators into different kind of iterators. We can chain
multiple calls to iterator adaptors to perform complex actions in a readable
way. Because all iterators are lazy, however, we have to call one of the
consuming adaptor methods in order to get results from calls to iterator
adaptors.

Listing 13-17 shows an example of calling the iterator adaptor method `map`
which takes a closure to call on each item in order to produce a new iterator.
The closure here creates a new iterator in which each item from the vector has
been incremented by 1. This code produces a warning, though:

<span class="filename">Filename: src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

<span class="caption">Listing 13-17: Calling the iterator adapter `map` to
create a new iterator</span>

The warning we get is:

```text
warning: unused result which must be used: iterator adaptors are lazy and do
nothing unless consumed
 --> src/main.rs:4:1
  |
4 | v1.iter().map(|x| x + 1);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default
```

The code in Listing 13-17 isn’t actually doing anything; the closure we’ve
specified never gets called. The warning reminds us why: iterator adaptors are
lazy, and we need to consume the iterator here.

To fix this and consume the iterator, we’re going to use the `collect` method,
which we saw briefly in Chapter 12. This method consumes the iterator and
collects the resulting values into a collection data type.

In Listing 13-18, we collect the results of iterating over the iterator that’s
returned from the call to `map` into a vector. This vector will end up
containing each item from the original vector incremented by 1:

<span class="filename">Filename: src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

<span class="caption">Listing 13-18: Calling the `map` method to create a new
iterator, then calling the `collect` method to consume the new iterator and
create a vector</span>

Because `map` takes a closure, we can specify any operation we want to perform
on each item. This is a great example of how closures let us customize some
behavior while reusing the iteration behavior that the `Iterator` trait
provides.

### Using Closures that Capture their Environment with Iterators

Now that we’ve introduced iterators, we can demonstrate a common use of
closures that capture their environment by using the `filter` iterator adapter.
The `filter` method on an iterator takes a closure that takes each item from
the iterator and returns a boolean. If the closure returns `true`, the value
will be included in the iterator produced by `filter`. If the closure returns
`false`, the value won’t be included in the resulting iterator.

In Listing 13-19 we use `filter` with a closure that captures the `shoe_size`
variable from its environment, in order to iterate over a collection of `Shoe`
struct instances. It will return only shoes that are the specified size:

<span class="filename">Filename: src/lib.rs</span>

```rust,test_harness
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
```

<span class="caption">Listing 13-19: Using the `filter` method with a closure
that captures `shoe_size`</span>

The `shoes_in_my_size` function takes ownership of a vector of shoes and a shoe
size as parameters. It returns a vector containing only shoes of the specified
size.

In the body of `shoes_in_my_size`, we call `into_iter` to create an iterator
that takes ownership of the vector. Then we call `filter` to adapt that
iterator into a new iterator that only contains elements for which the closure
returns `true`.

The closure captures the `shoe_size` parameter from the environment and
compares the value with each shoe’s size, keeping only shoes of the size
specified. Finally, calling `collect` gathers the values returned by the
adapted iterator into a vector that’s returned by the function.

The test shows that when we call `shoes_in_my_size`, we only get back shoes
that have the same size as the value we specified.

### Implementing the `Iterator` Trait to Create Our Own Iterators

We’ve shown that we can create an iterator by calling `iter`, `into_iter`, or
`iter_mut` on a vector. We can create iterators from the other collection types
in the standard library, such as hash map. We can also create iterators that do
anything we want by implementing the `Iterator` trait on our own types. As
previously mentioned, the only method we’re required to provide a definition
for is the `next` method. Once we’ve done that, we can use all other methods
that have default implementations provided by the `Iterator` trait!

To demonstrate, let’s create an iterator that will only ever count from 1 to 5.
First, we’ll create a struct to hold some values, and then we’ll make this
struct into an iterator by implementing the `Iterator` trait and use the values
in that implementation.

Listing 13-20 has the definition of the `Counter` struct and an associated
`new` function to create instances of `Counter`:

<span class="filename">Filename: src/lib.rs</span>

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

<span class="caption">Listing 13-20: Defining the `Counter` struct and a `new`
function that creates instances of `Counter` with an initial value of 0 for
`count`</span>

The `Counter` struct has one field named `count`. This holds a `u32` value that
will keep track of where we are in the process of iterating from 1 to 5. The
`count` field is private since we want the implementation of `Counter` to
manage its value. The `new` function enforces the behavior of always starting
new instances with a value of 0 in the `count` field.

Next, we’re going to implement the `Iterator` trait for our `Counter` type by
defining the body of the `next` method, to specify what we want to happen when
this iterator is used, as shown in Listing 13-21:

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

<span class="caption">Listing 13-21: Implementing the `Iterator` trait on our
`Counter` struct</span>

We set the associated `Item` type for our iterator to `u32`, meaning the
iterator will return `u32` values. Again, don’t worry about associated types
yet, we’ll be covering them in Chapter 19.

We want our iterator to add one to the current state, so we initialized `count`
to 0 so it would return one first. If the value of `count` is less than six,
`next` will return the current value wrapped in `Some`, but if `count` is six
or higher, our iterator will return `None`.

#### Using Our `Counter` Iterator’s `next` Method

Once we’ve implemented the `Iterator` trait, we have an iterator! Listing 13-22
shows a test demonstrating that we can use the iterator functionality of our
`Counter` struct by calling the `next` method on it directly, just like we did
with the iterator created from a vector in Listing 13-15:

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
# impl Iterator for Counter {
#     type Item = u32;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         self.count += 1;
#
#         if self.count < 6 {
#             Some(self.count)
#         } else {
#             None
#         }
#     }
# }
#
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

<span class="caption">Listing 13-22: Testing the functionality of the `next`
method implementation</span>

This test creates a new `Counter` instance in the `counter` variable and then
calls `next` repeatedly, verifying that we have implemented the behavior we
want this iterator to have: returning the values from 1 to 5.

#### Using Other `Iterator` Trait Methods on Our Iterator

Because we implemented the `Iterator` trait by defining the `next` method, we
can now use any `Iterator` trait method’s default implementations as defined in
the standard library, since they all use the `next` method’s functionality.

For example, if for some reason we wanted to take the values produced by an
instance of `Counter`, pair them with values produced by another `Counter`
instance after skipping the first value, multiply each pair together, keep only
those results that are divisible by three, and add all the resulting values
together, we could do so as shown in the test in Listing 13-23:

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
# impl Counter {
#     fn new() -> Counter {
#         Counter { count: 0 }
#     }
# }
#
# impl Iterator for Counter {
#     // Our iterator will produce u32s
#     type Item = u32;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         // increment our count. This is why we started at zero.
#         self.count += 1;
#
#         // check to see if we've finished counting or not.
#         if self.count < 6 {
#             Some(self.count)
#         } else {
#             None
#         }
#     }
# }
#
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
```

<span class="caption">Listing 13-23: Using a variety of `Iterator` trait
methods on our `Counter` iterator</span>

Note that `zip` produces only four pairs; the theoretical fifth pair `(5,
None)` is never produced because `zip` returns `None` when either of its input
iterators return `None`.

All of these method calls are possible because we specified how the `next`
method works, and the standard library provides default implementations for
other methods that call `next`.
