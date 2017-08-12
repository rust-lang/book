## Creating Reference Cycles and Leaking Memory is Safe

Rust makes a number of guarantees that we’ve talked about, for example that
we’ll never have a null value, and data races will be disallowed at compile
time. Rust’s memory safety guarantees make it more difficult to create memory
that never gets cleaned up, which is known as a *memory leak*. Rust does not
make memory leaks *impossible*, however: preventing memory leaks is *not* one
of Rust’s guarantees. In other words, memory leaks are memory safe.

By using `Rc<T>` and `RefCell<T>`, it is possible to create cycles of
references where items refer to each other in a cycle. This is bad because the
reference count of each item in the cycle will never reach 0, and the values
will never be dropped. Let’s take a look at how that might happen and how to
prevent it.

In Listing 15-16, we’re going to use another variation of the `List` definition
from Listing 15-5. We’re going back to storing an `i32` value as the first
element in the `Cons` variant. The second element in the `Cons` variant is now
`RefCell<Rc<List>>`: instead of being able to modify the `i32` value this time,
we want to be able to modify which `List` a `Cons` variant is pointing to.
We’ve also added a `tail` method to make it convenient for us to access the
second item, if we have a `Cons` variant:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}
```

<span class="caption">Listing 15-16: A cons list definition that holds a
`RefCell` so that we can modify what a `Cons` variant is referring to</span>

Next, in Listing 15-17, we’re going to create a `List` value in the variable
`a` that initially is a list of `5, Nil`. Then we’ll create a `List` value in
the variable `b` that is a list of the value 10 and then points to the list in
`a`. Finally, we’ll modify `a` so that it points to `b` instead of `Nil`, which
will then create a cycle:

<span class="filename">Filename: src/main.rs</span>

```rust
# #[derive(Debug)]
# enum List {
#     Cons(i32, RefCell<Rc<List>>),
#     Nil,
# }
#
# impl List {
#     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
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

<span class="caption">Listing 15-17: Creating a reference cycle of two `List`
values pointing to each other</span>

We use the `tail` method to get a reference to the `RefCell` in `a`, which we
put in the variable `link`. Then we use the `borrow_mut` method on the
`RefCell` to change the value inside from an `Rc` that holds a `Nil` value to
the `Rc` in `b`. We’ve created a reference cycle that looks like Figure 15-18:

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 15-18: A reference cycle of lists `a` and `b`
pointing to each other</span>

If you uncomment the last `println!`, Rust will try and print this cycle out
with `a` pointing to `b` pointing to `a` and so forth until it overflows the
stack.

Looking at the results of the `println!` calls before the last one, we’ll see
that the reference count of both `a` and `b` are 2 after we change `a` to point
to `b`. At the end of `main`, Rust will try and drop `b` first, which will
decrease the count of the `Rc` by one. However, because `a` is still
referencing that `Rc`, its count is 1 rather than 0, so the memory the `Rc` has
on the heap won’t be dropped. It’ll just sit there with a count of one,
forever. In this specific case, the program ends right away, so it’s not a
problem, but in a more complex program that allocates lots of memory in a cycle
and holds onto it for a long time, this would be a problem. The program would
be using more memory than it needs to be, and might overwhelm the system and
cause it to run out of memory available to use.

Now, as you can see, creating reference cycles is difficult and inconvenient in
Rust. But it’s not impossible: preventing memory leaks in the form of reference
cycles is not one of the guarantees Rust makes. If you have `RefCell<T>` values
that contain `Rc<T>` values or similar nested combinations of types with
interior mutability and reference counting, be aware that you’ll have to ensure
that you don’t create cycles. In the example in Listing 15-14, the solution
would probably be to not write code that could create cycles like this, since
we do want `Cons` variants to own the list they point to.

With data structures like graphs, it’s sometimes necessary to have references
that create cycles in order to have parent nodes point to their children and
children nodes point back in the opposite direction to their parents, for
example. If one of the directions is expressing ownership and the other isn’t,
one way of being able to model the relationship of the data without creating
reference cycles and memory leaks is using `Weak<T>`. Let’s explore that next!

### Prevent Reference Cycles: Turn an `Rc<T>` into a `Weak<T>`

The Rust standard library provides `Weak<T>`, a smart pointer type for use in
situations that have cycles of references but only one direction expresses
ownership. We’ve been showing how cloning an `Rc<T>` increases the
`strong_count` of references; `Weak<T>` is a way to reference an `Rc<T>` that
does not increment the `strong_count`: instead it increments the `weak_count`
of references to an `Rc`. When an `Rc` goes out of scope, the inner value will
get dropped if the `strong_count` is 0, even if the `weak_count` is not 0. To
be able to get the value from a `Weak<T>`, we first have to upgrade it to an
`Option<Rc<T>>` by using the `upgrade` method. The result of upgrading a
`Weak<T>` will be `Some` if the `Rc` value has not been dropped yet, and `None`
if the `Rc` value has been dropped. Because `upgrade` returns an `Option`, we
know Rust will make sure we handle both the `Some` case and the `None` case and
we won’t be trying to use an invalid pointer.

Instead of the list in Listing 15-17 where each item knows only about the
next item, let’s say we want a tree where the items know about their children
items *and* their parent items.

Let’s start just with a struct named `Node` that holds its own `i32` value as
well as references to its children `Node` values:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

We want to be able to have a `Node` own its children, and we also want to be
able to have variables own each node so we can access them directly. That’s why
the items in the `Vec` are `Rc<Node>` values. We want to be able to modify what
nodes are another node’s children, so that’s why we have a `RefCell` in
`children` around the `Vec`. In Listing 15-19, let’s create one instance of
`Node` named `leaf` with the value 3 and no children, and another instance
named `branch` with the value 5 and `leaf` as one of its children:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![leaf.clone()]),
    });
}
```

<span class="caption">Listing 15-19: Creating a `leaf` node and a `branch` node
where `branch` has `leaf` as one of its children but `leaf` has no reference to
`branch`</span>

The `Node` in `leaf` now has two owners: `leaf` and `branch`, since we clone
the `Rc` in `leaf` and store that in `branch`. The `Node` in `branch` knows
it’s related to `leaf` since `branch` has a reference to `leaf` in
`branch.children`. However, `leaf` doesn’t know that it’s related to `branch`,
and we’d like `leaf` to know that `branch` is its parent.

To do that, we’re going to add a `parent` field to our `Node` struct
definition, but what should the type of `parent` be? We know it can’t contain
an `Rc<T>`, since `leaf.parent` would point to `branch` and `branch.children`
contains a pointer to `leaf`, which makes a reference cycle. Neither `leaf` nor
`branch` would get dropped since they would always refer to each other and
their reference counts would never be zero.

So instead of `Rc`, we’re going to make the type of `parent` use `Weak<T>`,
specifically a `RefCell<Weak<Node>>`:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

This way, a node will be able to refer to its parent node if it has one,
but it does not own its parent. A parent node will be dropped even if
it has child nodes referring to it, as long as it doesn’t have a parent
node as well. Now let’s update `main` to look like Listing 15-20:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![leaf.clone()]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

<span class="caption">Listing 15-20: A `leaf` node and a `branch` node where
`leaf` has a `Weak` reference to its parent, `branch`</span>

Creating the `leaf` node looks similar; since it starts out without a parent,
we create a new `Weak` reference instance. When we try to get a reference to
the parent of `leaf` by using the `upgrade` method, we’ll get a `None` value,
as shown by the first `println!` that outputs:

```text
leaf parent = None
```

Similarly, `branch` will also have a new `Weak` reference, since `branch` does
not have a parent node. We still make `leaf` be one of the children of
`branch`. Once we have a new `Node` instance in `branch`, we can modify `leaf`
to have a `Weak` reference to `branch` for its parent. We use the `borrow_mut`
method on the `RefCell` in the `parent` field of `leaf`, then we use the
`Rc::downgrade` function to create a `Weak` reference to `branch` from the `Rc`
in `branch.`

When we print out the parent of `leaf` again, this time we’ll get a `Some`
variant holding `branch`. Also notice we don’t get a cycle printed out that
eventually ends in a stack overflow like we did in Listing 15-14: the `Weak`
references are just printed as `(Weak)`:

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

The fact that we don’t get infinite output (or at least until the stack
overflows) is one way we can see that we don’t have a reference cycle in this
case. Another way we can tell is by looking at the values we get from calling
`Rc::strong_count` and `Rc::weak_count`. In Listing 15-21, let’s create a new
inner scope and move the creation of `branch` in there, so that we can see what
happens when `branch` is created and then dropped when it goes out of scope:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![leaf.clone()]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

<span class="caption">Listing 15-21: Creating `branch` in an inner scope and
examining strong and weak reference counts of `leaf` and `branch`</span>

Right after creating `leaf`, its strong count is 1 (for `leaf` itself) and its
weak count is 0. In the inner scope, after we create `branch` and associate
`leaf` and `branch`, `branch` will have a strong count of 1 (for `branch`
itself) and a weak count of 1 (for `leaf.parent` pointing to `branch` with a
`Weak<T>`). `leaf` will have a strong count of 2, since `branch` now has a
clone the `Rc` of `leaf` stored in `branch.children`. `leaf` still has a weak
count of 0.

When the inner scope ends, `branch` goes out of scope, and its strong count
decreases to 0, so its `Node` gets dropped. The weak count of 1 from
`leaf.parent` has no bearing on whether `Node` gets dropped or not, so we don’t
have a memory leak!

If we try to access the parent of `leaf` after the end of the scope, we’ll get
`None` again like we did before `leaf` had a parent. At the end of the program,
`leaf` has a strong count of 1 and a weak count of 0, since `leaf` is now the
only thing pointing to it again.

All of the logic managing the counts and whether a value should be dropped or
not was managed by `Rc` and `Weak` and their implementations of the `Drop`
trait. By specifying that the relationship from a child to its parent should be
a `Weak<T>` reference in the definition of `Node`, we’re able to have parent
nodes point to child nodes and vice versa without creating a reference cycle
and memory leaks.

## Summary

We’ve now covered how you can use different kinds of smart pointers to choose
different guarantees and tradeoffs than those Rust makes with regular
references. `Box<T>` has a known size and points to data allocated on the heap.
`Rc<T>` keeps track of the number of references to data on the heap so that
data can have multiple owners. `RefCell<T>` with its interior mutability gives
us a type that can be used where we need an immutable type, and enforces the
borrowing rules at runtime instead of at compile time.

We’ve also discussed the `Deref` and `Drop` traits that enable a lot of smart
pointers’ functionality. We explored how it’s possible to create a reference
cycle that would cause a memory leak, and how to prevent reference cycles by
using `Weak<T>`.

If this chapter has piqued your interest and you now want to implement your own
smart pointers, check out [The Nomicon] for even more useful information.

[The Nomicon]: https://doc.rust-lang.org/stable/nomicon/

Next, let’s talk about concurrency in Rust. We’ll even learn about a few new
smart pointers that can help us with it.
