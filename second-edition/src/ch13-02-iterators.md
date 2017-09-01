## Processing a Series of Items with Iterators

<!-- From reading on, it seems like an iterator is useless without the methods
we use it with --- I think this is an important point to make early, I did find
it difficult to know what an iterator actually was throughout, and how it
depends on these other methods. Can you add something to this effect? -->
<!-- Reiterating the need for a clear definition of an iterator here--it seems
like an item that's used in iteration rather than something that performs the
process of iteration itself, is that right? Like a counter passed from element
to element? Can we define this at the begin of the iterator section? -->
<!-- I've added an explanation along these lines, does this help? /Carol -->

The iterator pattern allows you to perform some task on a sequence of items in
turn. An *iterator* is responsible for the logic around iterating over each item
in the sequence and determining when the sequence has finished. When we use
iterators, we don’t have to reimplement that logic ourselves.

In Rust, iterators are *lazy*, which means they have no effect until we call
methods on them that consume the iterator to use it up. For example, the code
in Listing 13-13 creates an iterator over the items in the vector `v1` by
calling the `iter` method defined on `Vec`. This code by itself doesn’t do
anything useful:

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

<span class="caption">Listing 13-13: Creating an iterator</span>

After creating an iterator, we can choose to use it in a variety of ways. In
Listing 3-6, we actually used iterators with `for` loops to execute some code
on each item, though we glossed over what the call to `iter` did until now. The
example in Listing 13-14 separates the creation of the iterator from the use of
the iterator in the `for` loop. The iterator is stored in the `v1_iter`
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
the variable value in a loop until its value gets up to the total number of
items in the vector. Iterators take care of all of that logic for us, which
cuts down on the repetitive code we would have to write and potentially mess up.
In addition, the way iterators are implemented gives us more flexibility to
use the same logic with many different kinds of sequences, not just data
structures that we can index into like vectors. Let’s see how iterators do that.

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
to know is that this code says implementing `Iterator` trait requires that you
also define an `Item` type, and this `Item` type is used in the return type of
the `next` method. In other words, the `Item` type will be the type of element
that’s returned from the iterator.

The `next` method is the only method that the `Iterator` trait requires
implementors of the trait to define. `next` returns one item of the iterator
at a time wrapped in `Some`, and when iteration is over, it returns `None`.
We can call the `next` method on iterators directly if we’d like; Listing 13-15
has a test that demonstrates the values we’d get on repeated calls to `next`
on the iterator created from the vector:

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
iterator changes the iterator’s state that keeps track of where it is in the
sequence. Put another way, this code *consumes*, or uses up, the iterator. Each
call to `next` eats up an item from the iterator. We didn’t need to make
`v1_iter` mutable when we used a `for` loop because the `for` loop took
ownership of `v1_iter` and made `v1_iter` mutable behind the scenes.

Also note that the values we get from the calls to `next` are immutable
references to the values in the vector. The `iter` method produces an iterator
over immutable references. If we wanted to create an iterator that takes
ownership of `v1` and returns owned values, we can call `into_iter` instead of
`iter`. Similarly, if we want to iterate over mutable references, we can call
`iter_mut` instead of `iter`.

### Methods in the `Iterator` Trait that Consume the Iterator

<!-- Can you explain what it is you mean by "consumes" an iterator here? It
doesn't look like we do in this section, I think that's important to lay that
out clearly -->
<!-- This next paragraph doesn't give much away to me I'm afraid, not being
clear what we mean by *consume* at this point. Is a consuming adaptor like a
catalyst? -->
<!-- I hope this section addresses these comments you had /Carol -->

The `Iterator` trait has a number of different methods with default
implementations provided for us by the standard library; you can find out all
about these methods by looking in the standard library API documentation for
the `Iterator` trait. Some of these methods call the `next` method in their
definition, which is why we’re required to implement the `next` method when
implementing the `Iterator` trait.

<!-- Is there somewhere they can learn about all the methods and what they do,
how to use them? This seems like a good sample example, and if we can broaden
it out that would be really helpful -->
<!-- I've moved this comment here since you made this comment on the last
version of this chapter right after a spot where we mentioned looking at the
standard library API documentation for the iterator trait, like we're now doing
in the above paragraph. That's where the reader should go to learn about
all the methods and what they do and how to use them. Can you elaborate on why
that wasn't clear in the previous version of the chapter? Is there a reason why
the standard library API documentation didn't sound like that place to go?
/Carol -->

The methods that call the `next` method are called *consuming adaptors*, since
calling them uses up the iterator. An example of a consuming adaptor is the
`sum` method. This method takes ownership of the iterator and iterates through
the items by repeatedly calling `next`, thus consuming the iterator. As it
iterates through each item, it adds each item to a running total and returns
the total when iteration has completed. Listing 13-16 has a test illustrating a
use of the `sum` method:

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

Another kind of method defined on the `Iterator` trait are methods that produce
other iterators. These methods are called *iterator adaptors* and allow us to
change iterators into different kind of iterators. We can chain multiple calls
to iterator adaptors. Because all iterators are lazy, however, we have to
call one of the consuming adaptor methods in order to get results from calls
to iterator adaptors. Listing 13-17 shows an example of calling the iterator
adaptor method `map`, which takes a closure that `map` will call on each
item in order to produce a new iterator in which each item from the vector has
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
lazy, and we probably meant to consume the iterator here.

In order to fix this warning and consume the iterator to get a useful result,
we’re going to use the `collect` method, which we saw briefly in Chapter 12.
This method consumes the iterator and collects the resulting values into a
data structure. In Listing 13-18, we’re going to collect the results of
iterating over the iterator returned from the call to `map` into a vector that
will contain each item from the original vector incremented by 1:

<span class="filename">Filename: src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

<span class="caption">Listing 13-18: Calling the `map` method to create a new
iterator, then calling the `collect` method to consume the new iterator and
create a vector</span>

Because `map` takes a closure, we can specify any operation that we want to
perform on each item that we iterate over. This is a great example of how using
closures lets us customize some behavior while reusing the iteration behavior
that the `Iterator` trait provides.

<!-- I'm not clear from this last sentence which part is iterating through each
element, iter or map? What is map actually doing?-->
<!--Ah, I'm afraid I completely failed to follow this. What is the second
iterator for? I'm still not clear on what map does, can you expand on this? It
seems crucial to using iterators. Map applies the iterator to each element,
which applies the closure?

Also, to generalize this discussion a bit, would you ever use iter without map?
-->
<!-- I hope this new breakdown/rearranging has cleared up these comments you
had on the last version of this chapter about the difference between
iter and map. I hope the added examples where we've used iter without map have
cleared up the last question. /Carol -->

### Using Closures that Capture their Environment with Iterators

Now that we’ve introduced iterators, we can demonstrate a common use of
closures that capture their environment by using the `filter` iterator adapter.
The `filter` method on an iterator takes a closure that takes each item from
the iterator and returns a boolean. If the closure returns `true`, the value
will be included in the iterator produced by `filter`. If the closure returns
`false`, the value won’t be included in the resulting iterator. Listing 13-19
demonstrates using `filter` with a closure that captures the `shoe_size`
variable from its environment in order to iterate over a collection of `Shoe`
struct instances in order to return only shoes that are the specified size:

<span class="filename">Filename: src/lib.rs</span>

```rust,test_harness
#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
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

<!-- Will add wingdings in libreoffice /Carol -->

The `shoes_in_my_size` function takes ownership of a vector of shoes and a shoe
size as parameters. It returns a vector containing only shoes of the specified
size. In the body of `shoes_in_my_size`, we call `into_iter` to create an
iterator that takes ownership of the vector. Then we call `filter` to adapt
that iterator into a new iterator that only contains elements for which the
closure returns `true`. The closure we’ve specified captures the `shoe_size`
parameter from the environment and uses the value to compare with each shoe’s
size to only keep shoes that are of the size specified. Finally, calling
`collect` gathers the values returned by the adapted iterator into a vector
that the function returns.

The test shows that when we call `shoes_in_my_size`, we only get back shoes
that have the same size as the value we specified.

### Implementing the `Iterator` Trait to Create Our Own Iterators

<!-- So it seems like we are creating a program with an iterator inside, is
that right? I assumed the whole thing we were making was an iterator at first,
which lead to a few confusions, can you lay it out up front? -->
<!-- I'm not sure what you mean here, can you elaborate on what the distinction
is to you between "a program with an iterator inside" and "whole thing we were
making was an iterator"? I don't understand what you mean by these terms so I'm
not sure how to clear this up. /Carol -->

We’ve shown that we can create an iterator by calling `iter`, `into_iter`, or
`iter_mut` on a vector. We can also create iterators from the other collection
types in the standard library, such as hash map. Additionally, we can implement
the `Iterator` trait in order to create iterators that do anything we want.
As previously mentioned, the only method we’re required to provide a definition
for is the `next` method. Once we’ve done that, we can use all the other
methods that have default implementations provided by the `Iterator` trait on
our iterator!

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

The iterator we’re going to create is one that will only ever count from 1
to 5. First, we’ll create a struct to hold on to some values, and then we’ll
make this struct into an iterator by implementing the `Iterator` trait and use
the values in that implementation.

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

<!-- Could you add a filename here? I think that will help the reader keep
track of what they're working on. Can you also just sum up in a line what this
code has accomplished so far? I moved this down from above the code, if this
will do? -->
<!-- Done /Carol -->

The `Counter` struct has one field named `count`. This field holds a `u32`
value that will keep track of where we are in the process of iterating from 1
to 5. The `count` field is private since we want the implementation of
`Counter` to manage its value. The `new` function enforces the behavior we want
of always starting new instances with a value of 0 in the `count` field.

<!-- Why define the new method, if it isn't necessary? Or is that what this
next line is telling us? -->
<!-- So does this code just initialize it with 0? Is that jat { count: 0 }
does?-->
<!-- I've rearranged to make this clearer /Carol -->

Next, we’re going to implement the `Iterator` trait for our `Counter` type by
defining the body of the `next` method to specify what we want to happen when
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

<!-- I will add wingdings in libreoffice /Carol -->

We set the associated `Item` type for our iterator to `u32`, meaning the
iterator will return `u32` values. Again, don’t worry about associated types
yet, we’ll be covering them in Chapter 19. We want our iterator to add one to
the current state, which is why we initialized `count` to 0: we want our
iterator to return one first. If the value of `count` is less than six, `next`
will return the current value wrapped in `Some`, but if `count` is six or
higher, our iterator will return `None`.

#### Using Our `Counter` Iterator’s `next` Method

Once we’ve implemented the `Iterator` trait, we have an iterator! Listing 13-22
shows a test demonstrating that we can use the iterator functionality our
`Counter` struct now has by calling the `next` method on it directly, just like
we did with the iterator created from a vector in Listing 13-15:

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
want this iterator to have of returning the values from 1 to 5.

<!-- So if I have this right, the first line creates a new Counter called
counter, and the rest of them merely call counter with next, store it in x, and
then print x? And we have to do that 5 times to get the 1-5 count? Phew, could
you wrap that up if indeed it is correct!) and sum up here? -->
<!-- I decided to change this into a test rather than printing out values, and
I added some summary text about what the test is doing. Is this clearer? /Carol
-->

#### Using Other `Iterator` Trait Methods on Our Iterator

Because we implemented the `Iterator` trait by defining the `next` method, we
can now use any `Iterator` trait method’s default implementations that the
standard library has defined, since they all use the `next` method’s
functionality.

<!-- So we can't just use these methods anyway? It seems like we did earlier,
but here we have to use next first, before we cam access these methods? -->
<!-- No, we don't have to *use* `next` before we can use the other methods, we
have to *define* `next` before we can use the other methods. I hope the various
rewordings and reworkings have made this clearer by this point. /Carol -->

<!-- below: once you've done what, defined a default implementation? Only then
can you use other adapters, is that what we're saying? And I'm still not clear
on what an adapter does/means, as opposed to a method, or consumer, at this
point. -->
<!-- I hope this comment has been cleared up too /Carol -->

For example, if for some reason we wanted to take the values that an instance
of `Counter` produces, pair those values with values produced by another
`Counter` instance after skipping the first value that instance produces,
multiply each pair together, keep only those results that are divisible by
three, and add all the resulting values together, we could do so as shown in
the test in Listing 13-23:

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

All of these method calls are possible because we implemented the `Iterator`
trait by specifying how the `next` method works and the standard library
provides default implementations for other methods that call `next`.
