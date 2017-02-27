## Creating Reference Cycles and Leaking Memory is Safe

Rust makes a number of guarantees that we've talked about, for example that
we'll never have a null value, and data races will be disallowed at compile
time. Rust's memory safety guarantees make it more difficult to create memory
that never gets cleaned up, which is known as a *memory leak*. Rust does not
make memory leaks *impossible*, however, preventing memory leaks is *not* one
of Rust's guarantees. In other words, memory leaks are memory safe.

By using `Rc<T>` and `RefCell<T>`, it is possible to create cycles of
references where items refer to each other in a cycle. This is bad because the
reference count of each item in the cycle will never reach 0, and the values
will never be dropped. Let's take a look at how that might happen and how to
prevent it.

In Listing 15-13, we're going to continue building on the `List<T>` definition
from Listing 15-12. We've added a `RefCell` to the `Cons` variant's definition
so that we can modify a `Cons` instance after we've created it. We've also
added a `tail` method to make it convenient for us to access the second item,
if we have a `Cons` variant:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

impl<T> List<T> {
    fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}
```

<figcaption>

Listing 15-13: A cons list definition that holds a `RefCell` so that we can
modify what a `Cons` variant is referring to

</figcaption>
</figure>

TODO: insert diagram showing the "shape" of the `Cons` variant?

Next, in Listing 15-14, we're going to create a `List<T>` value in the variable
`a` that initially is a list of `5, Nil`. Then we'll create a `List<T>` value
in the variable `b` that is a list of the value 10 and then points to the list
in `a`. Finally, we'll modify `a` so that it points to `b` instead of `Nil`,
which will then create a cycle:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
# #[derive(Debug)]
# enum List<T> {
#     Cons(T, RefCell<Rc<List<T>>>),
#     Nil,
# }
#
# impl<T> List<T> {
#     fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
#         match *self {
#             Cons(_, ref item) => Some(item),
#             Nil => None,
#         }
#     }
# }
#
use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(a.clone())));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = b.clone();
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle; it will
    // overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

<figcaption>

Listing 15-14: Creating a reference cycle of two `List<T>` values pointing to
each other

</figcaption>
</figure>

We use the `tail` method to get a reference to the `RefCell` in `a`, which we
put in the variable `link`. Then we use the `borrow_mut` method on the
`RefCell` to change the value inside from an `Rc` that holds a `Nil` value to
the `Rc` in `b`. We've created a reference cycle that looks like:

TODO: insert diagram showing b and a pointing to each other

If you uncomment the last `println!`, Rust will try and print this cycle out
with `a` pointing to `b` pointing to `a` and so forth until it overflows the
stack.

Looking at the results of the `println!` calls before the last one, we'll see
that the reference count of both `a` and `b` are 2 after we change `a` to point
to `b`. At the end of `main`, Rust will try and drop `b` first, which will
decrease the count of the `Rc` by one. However, because `a` is still
referencing that `Rc`, its count is 1 rather than 0, so the memory the `Rc` has
on the heap won't be dropped. It'll just sit there with a count of one,
forever. In this specific case, the program ends right away, so it's not a
problem, but in a more complex program that allocates lots of memory in a cycle
and holds onto it for a long time, this would be a problem. The program would
be using more memory than it needs to be, and might overwhelm the system and
cause it to run out of memory available to use.

Now, as you can see, creating reference cycles is difficult and inconvenient in
Rust. To be honest, your authors had to look up previous discussions of an
example to get this right. But it's not impossible: preventing memory leaks in
the form of reference cycles is not one of the guarantees Rust makes. If you
have `RefCell<T>` values that contain `Rc<T>` values or similar nested
combinations of types with interior mutability and reference counting, be aware
that you'll have to ensure that you don't create cycles on your. One way of
doing that is using `Weak<T>`.

### Prevent Reference Cycles: Turn an `Rc<T>` into a `Weak<T>`

To help with this problem, the Rust standard library contains another smart
pointer type: `Weak<T>`. We've been showing how cloning an `Rc<T>` increases
the `strong_count` of references; `Weak<T>` is a way to reference an `Rc<T>`
that does not increment the `strong_count`: instead it increments the
`weak_count` of references to an `Rc`. When an `Rc` goes out of scope, the
inner value will get dropped if the `strong_count` is 0, even if the
`weak_count` is not 0. When we attempt to use a `Weak<T>` reference, we'll get
an `Option<T>` that will be `Some` if the `Rc` value has not been dropped yet,
and `None` if the `Rc` value has been dropped.

So in order to make it possible to create lists that point to each other but
not create reference cycles, in Listing 15-15, we're going to change our
definition of `List<T>` again to hold a `Weak<T>` instead of an `Rc<T>`.

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, RefCell<Weak<List<T>>>),
    Nil,
}

impl<T> List<T> {
    fn tail(&self) -> Option<Rc<List<T>>> {
        match *self {
            Cons(_, ref n) => (&*n.borrow()).upgrade(),
            Nil => None,
        }
    }
}
```

<figcaption>

Listing 15-15: Modifying `List<T>` to have `Weak<T>` references instead of
`Rc<T>`

</figcaption>
</figure>

We've also modified the `tail` method: not only does it return `None` when
`self` is `Nil` and doesn't have a `tail`, it now also returns `None` if the
value that the `Weak<T>` references has been dropped. The `upgrade` method on a
`Weak<T>` value returns `Some` containing an `Rc` if the value has not yet been
dropped and `None` if the value has been dropped.

TODO: is this bad software design, collapsing two cases that return `None` like this???

To create `Weak<T>` values, we call the `Rc::downgrade` associated function,
which takes an `&Rc<T>` as an argument, and gives a `Weak<T>` back.

Listing 15-16 shows a `main` method where we're trying to create `a` and `b`
lists that point to each other, similarly to what we did in Listing 15-14, but
this time we won't have a reference cycle and the values will be dropped when
they go out of scope at the end of `main`:

<figure>

```rust
# #[derive(Debug)]
# enum List<T> {
#     Cons(T, RefCell<Weak<List<T>>>),
#     Nil,
# }
#
# impl<T> List<T> {
#     fn tail(&self) -> Option<Rc<List<T>>> {
#         match *self {
#             Cons(_, ref n) => (&*n.borrow()).upgrade(),
#             Nil => None,
#         }
#     }
# }
#
use List::{Cons, Nil};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let nil = Rc::new(Nil);

    let a = Rc::new(Cons(5, RefCell::new(Rc::downgrade(&nil))));

    println!("a.tail() = {:?}", a.tail());

    {
        let b = Rc::new(Cons(10, RefCell::new(Rc::downgrade(&a))));

        if let Cons(_, ref link) = *a {
            *link.borrow_mut() = Rc::downgrade(&b);
        }

        println!("a.tail() = {:?}", a.tail());
        println!("b.tail() = {:?}", b.tail());
    }

    println!("a.tail() = {:?}", a.tail());
}
```

<figcaption>

Listing 15-16: Creating `List<T>` values using weak references

</figcaption>
</figure>

First, we create a variable for the `Rc<T>` that holds the `Nil` value, so that
it's clearer to see that we call `Rc::downgrade` and pass a reference to `nil`
when we create `a`. At that point, we print out the value of `a.tail()` and we
can see that it's `Some(Nil)`.

We've added an inner scope in order to demonstrate what happens when `b` goes
out of scope. In the inner scope, we create `b` that has a weak reference to
`a`, and then we modify `a` to have a weak reference to `b` instead of to
`nil`. At the end of the inner scope, we can see that `a` and `b` are pointing
to each other, but through weak references.

At the end of the inner scope, `b` goes out of scope. The value that the
`Rc<T>` in `b` holds gets dropped, even though `a` still references it, because
the reference in `a` is a weak reference that doesn't count when Rust decides
whether the value in `b` should be dropped or not. At the end of `main` when we
print out `a.tail()` again, we can see that we now get a `None` value since the
value that the weak reference in `a` was pointing to has been dropped. Success!
We've broken the cycle.

Add some `println!` statements at various places that display the values that
`Rc::strong_count` and `Rc::weak_count` return for `a` and `b` at different
points in this example to see which parts of the code increase or decrease the
strong or weak counts.

If you're doing complex things with `Rc<T>`s, you should investigate if it's
possible for you to have a cycle, and insert a `Weak<T>` so that you don't
create cycles and leak memory. The specifics depend on exactly what you're
doing, but luckily, this isn't a Rust-specific idea; all of this translates
over to other reference counting libraries in other languages, so doing some
reading about it should help you.

## Summary

We've now covered how you can use different kinds of smart pointers to choose
different guarantees and tradeoffs than those Rust makes with regular
references. `Box<T>` has a known size and points to data allocated on the heap.
`Rc<T>` keeps track of the number of references to data on the heap so that
data can have multiple owners. `RefCell<T>` with its interior mutability gives
us a type that can be used where we need an immutable type, and enforces the
borrowing rules at runtime instead of at compile time.

We've also discussed the `Deref` and `Drop` traits that enable a lot of smart
pointers' functionality. We explored how it's possible to create a reference
cycle that would cause a memory leak, and how to prevent reference cycles by
using `Weak<T>`.

If this chapter has piqued your interest and you now want to implement your own
smart pointers, check out [The Nomicon] for even more useful information.

[The Nomicon]: https://doc.rust-lang.org/stable/nomicon/vec.html

Next, let's talk about concurrency in Rust. We'll even learn about a few new
smart pointers that can help us with it.
