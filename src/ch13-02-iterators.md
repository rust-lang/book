## Iterators

Iterators are a pattern in Rust that allows you to do some processing on a
sequence of items. For example, the code in Listing 13-5 adds one to each
number in a vector:

<figure>

```rust
let v1 = vec![1, 2, 3];

let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, [2, 3, 4]);
```

<figcaption>

Listing 13-5: Using an iterator, `map`, and `collect` to add one to each number
in a vector

</figcaption>
</figure>

<!-- Will add wingdings in libreoffice /Carol -->

The `iter` method on vectors allows us to produce an *iterator* from the
vector. Next, the `map` method called on the iterator allows us to process each
element: in this case, we've passed a closure to `map` that specifies for every
element `x`, add one to it. `map` is one of the most basic ways of interacting
with an iterator, as processing each element in turn is very useful! Finally,
the `collect` method consumes the iterator and puts the iterator's elements
into a new data structure. In this case, since we've said that `v2` has the
type `Vec<i32>`, `collect` will create a new vector out of the `i32` values.

Methods on iterators like `map` are sometimes called *iterator adaptors*
because they take one iterator and produce a new iterator. That is, `map`
builds on top of our previous iterator and produces another iterator by calling
the closure it's passed to create the new sequence of values.

So, to recap, this line of code does the following:

1. Creates an iterator from the vector.
2. Uses the `map` adaptor with a closure argument to add one to each element.
3. Uses the `collect` adaptor to consume the iterator and make a new vector.

That's how we end up with `[2, 3, 4]`. As you can see, closures are a very
important part of using iterators: they provide a way of customizing the
behavior of an iterator adaptor like `map`.

### Iterators are Lazy

In the previous section, you may have noticed a subtle difference in wording:
we said that `map` *adapts* an iterator, but `collect` *consumes* one. That was
intentional. By themselves, iterators won't do anything; they're lazy. That is,
if we write code like Listing 13-5 except we don't call `collect`:

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1); // without collect
```

It will compile, but it will give us a warning:

```text
warning: unused result which must be used: iterator adaptors are lazy and do
nothing unless consumed, #[warn(unused_must_use)] on by default
 --> src/main.rs:4:1
  |
4 | v1.iter().map(|x| x + 1); // without collect
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
```

We get this warning because iterator adaptors won't start actually doing the
processing on their own. They need some other method that causes the iterator
chain to evaluate. We call those *consuming adaptors*, and `collect` is one of
them.

So how do we tell which iterator methods consume the iterator or not? And what
adaptors are available? For that, let's look at the `Iterator` trait.

### The `Iterator` trait

Iterators all implement a trait named `Iterator` that is defined in the standard
library. The definition of the trait looks like this:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

There's some new syntax that we haven't covered here yet: `type Item` and
`Self::Item` are defining an *associated type* with this trait, and we'll talk
about associated types in depth in Chapter XX. For now, all you need to know is
that this code says the `Iterator` trait requires that you also define an
`Item` type, and this `Item` type is used in the return type of the `next`
method. In other words, the `Item` type will be the type of element that's
returned from the iterator.

Let's make an iterator named `Counter` that will count from `1` to `5`, using
the `Iterator` trait. First, we need to create a struct that holds the current
state of the iterator, which is one field named `count` that will hold a `u32`.
We'll also define a `new` method, which isn't strictly necessary. We want our
`Counter` to go from one to five, though, so we're always going to have it
holding a zero to start:

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

Next, we're going to implement the `Iterator` trait for our `Counter` type by
defining the body of the `next` method. The way we want our iterator to work
is to add one to the state (which is why we initialized `count` to 0, since we
want our iterator to return one first). If `count` is still less than six, we'll
return the current value, but if `count` is six or higher, our iterator will
return `None`, as shown in Listing 13-6:

<figure>

```rust
# struct Counter {
#     count: u32,
# }
#
impl Iterator for Counter {
    // Our iterator will produce u32s
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // increment our count. This is why we started at zero.
        self.count += 1;

        // check to see if we've finished counting or not.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

<figcaption>

Listing 13-6: Implementing the `Iterator` trait on our `Counter` struct

</figcaption>
</figure>

<!-- I will add wingdings in libreoffice /Carol -->

The `type Item = u32` line is saying that the associated `Item` type will be
a `u32` for our iterator. Again, don't worry about associated types yet, because
we'll be covering them in Chapter XX.

The `next` method is the main interface into an iterator, and it returns an
`Option`. If the option is `Some(value)`, we have gotten another value from the
iterator. If it's `None`, iteration is finished. Inside of the `next` method,
we do whatever kind of calculation our iterator needs to do. In this case, we
add one, then check to see if we're still below six. If we are, we can return
`Some(self.count)` to produce the next value. If we're at six or more,
iteration is over, so we return `None`.

The iterator trait specifies that when an iterator returns `None`, that
indicates iteration is finished. The trait does not mandate anything about the
behavior an iterator must have if the `next` method is called again after
having returned one `None` value. In this case, every time we call `next` after
getting the first `None` value will still return `None`, but the internal
`count` field will continue to be incremented by one each time. If we call
`next` as many times as the maximum value a `u32` value can hold, `count` will
overflow (which will `panic!` in debug mode and wrap in release mode). Other
iterator implementations choose to start iterating again. If you need to be
sure to have an iterator that will always return `None` on subsequent calls to
the `next` method after the first `None` value is returned, you can use the
`fuse` method to create an iterator with that characteristic out of any other
iterator.

Once we've implemented the `Iterator` trait, we have an iterator! We can use
the iterator functionality that our `Counter` struct now has by calling the
`next` method on it repeatedly:

```rust,ignore
let mut counter = Counter::new();

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);
```

This will print `Some(1)` through `Some(5)` and then `None`, each on their own
line.

### All Sorts of `Iterator` Adaptors

In Listing 13-5, we had iterators and we called methods like `map` and
`collect` on them. In Listing 13-6, however, we only implemented the `next`
method on our `Counter`. How do we get methods like `map` and `collect` on our
`Counter`?

Well, when we told you about the definition of `Iterator`, we committed a small
lie of omission. The `Iterator` trait has a number of other useful methods
defined on it that come with default implementations that call the `next`
method. Since `next` is the only method of the `Iterator` trait that does not
have a default implementation, once you've done that, you get all of the other
`Iterator` adaptors for free. There are a lot of them!

For example, if for some reason we wanted to take the first five values that
an instance of `Counter` produces, pair those values with values produced by
another `Counter` instance after skipping the first value that instance
produces, multiply each pair together, keep only those results that are
divisible by three, and add all the resulting values together, we could do:

```rust,ignore
let sum: u32 = Counter::new().take(5)
                             .zip(Counter::new().skip(1))
                             .map(|(a, b)| a * b)
                             .filter(|x| x % 3 == 0)
                             .sum();
assert_eq!(48, sum);
```

All of these method calls are possible because we implemented the `Iterator`
trait by specifying how the `next` method works. Use the standard library
documentation to find more useful methods that will come in handy when you're
working with iterators.
