## Reference Cycles Can Leak Memory

Rust’s memory safety guarantees make it difficult, but not impossible, to
accidentally create memory that is never cleaned up (known as a *memory leak*).
Preventing memory leaks entirely is not one of Rust’s guarantees in the same
way that disallowing data races at compile time is, meaning memory leaks are
memory safe in Rust. We can see that Rust allows memory leaks by using `Rc<T>`
and `RefCell<T>`: it’s possible to create references where items refer to each
other in a cycle. This creates memory leaks because the reference count of each
item in the cycle will never reach 0, and the values will never be dropped.

### Creating a Reference Cycle

Let’s look at how a reference cycle might happen and how to prevent it,
starting with the definition of the `List` enum and a `tail` method in Listing
15-25:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs}}
```

<span class="caption">Listing 15-25: A cons list definition that holds a
`RefCell<T>` so we can modify what a `Cons` variant is referring to</span>

We’re using another variation of the `List` definition from Listing 15-5. The
second element in the `Cons` variant is now `RefCell<Rc<List>>`, meaning that
instead of having the ability to modify the `i32` value as we did in Listing
15-24, we want to modify which `List` value a `Cons` variant is pointing to.
We’re also adding a `tail` method to make it convenient for us to access the
second item if we have a `Cons` variant.

In Listing 15-26, we’re adding a `main` function that uses the definitions in
Listing 15-25. This code creates a list in `a` and a list in `b` that points to
the list in `a`. Then it modifies the list in `a` to point to `b`, creating a
reference cycle. There are `println!` statements along the way to show what the
reference counts are at various points in this process.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

<span class="caption">Listing 15-26: Creating a reference cycle of two `List`
values pointing to each other</span>

We create an `Rc<List>` instance holding a `List` value in the variable `a`
with an initial list of `5, Nil`. We then create an `Rc<List>` instance
holding another `List` value in the variable `b` that contains the value 10 and
points to the list in `a`.

We modify `a` so it points to `b` instead of `Nil`, creating a cycle. We
do that by using the `tail` method to get a reference to the
`RefCell<Rc<List>>` in `a`, which we put in the variable `link`. Then we use
the `borrow_mut` method on the `RefCell<Rc<List>>` to change the value inside
from an `Rc<List>` that holds a `Nil` value to the `Rc<List>` in `b`.

When we run this code, keeping the last `println!` commented out for the
moment, we’ll get this output:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

The reference count of the `Rc<List>` instances in both `a` and `b` are 2 after
we change the list in `a` to point to `b`. At the end of `main`, Rust drops the
variable `b`, which decreases the reference count of the `Rc<List>` instance
from 2 to 1. The memory that `Rc<List>` has on the heap won’t be dropped at
this point, because its reference count is 1, not 0. Then Rust drops `a`, which
decreases the reference count of the `a` `Rc<List>` instance from 2 to 1 as
well. This instance's memory can’t be dropped either, because the other
`Rc<List>` instance still refers to it. The memory allocated to the list will
remain uncollected forever. To visualize this reference cycle, we’ve created a
diagram in Figure 15-4.

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />

<span class="caption">Figure 15-4: A reference cycle of lists `a` and `b`
pointing to each other</span>

If you uncomment the last `println!` and run the program, Rust will try to
print this cycle with `a` pointing to `b` pointing to `a` and so forth until it
overflows the stack.

In this case, right after we create the reference cycle, the program ends. The
consequences of this cycle aren’t very dire. However, if a more complex program
allocated lots of memory in a cycle and held onto it for a long time, the
program would use more memory than it needed and might overwhelm the system,
causing it to run out of available memory.

Creating reference cycles is not easily done, but it’s not impossible either.
If you have `RefCell<T>` values that contain `Rc<T>` values or similar nested
combinations of types with interior mutability and reference counting, you must
ensure that you don’t create cycles; you can’t rely on Rust to catch them.
Creating a reference cycle would be a logic bug in your program that you should
use automated tests, code reviews, and other software development practices to
minimize.

Another solution for avoiding reference cycles is reorganizing your data
structures so that some references express ownership and some references don’t.
As a result, you can have cycles made up of some ownership relationships and
some non-ownership relationships, and only the ownership relationships affect
whether or not a value can be dropped. In Listing 15-25, we always want `Cons`
variants to own their list, so reorganizing the data structure isn’t possible.
Let’s look at an example using graphs made up of parent nodes and child nodes
to see when non-ownership relationships are an appropriate way to prevent
reference cycles.

### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`

So far, we’ve demonstrated that calling `Rc::clone` increases the
`strong_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned
up if its `strong_count` is 0. You can also create a *weak reference* to the
value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a
reference to the `Rc<T>`. When you call `Rc::downgrade`, you get a smart
pointer of type `Weak<T>`. Instead of increasing the `strong_count` in the
`Rc<T>` instance by 1, calling `Rc::downgrade` increases the `weak_count` by 1.
The `Rc<T>` type uses `weak_count` to keep track of how many `Weak<T>`
references exist, similar to `strong_count`. The difference is the `weak_count`
doesn’t need to be 0 for the `Rc<T>` instance to be cleaned up.

Strong references are how you can share ownership of an `Rc<T>` instance. Weak
references don’t express an ownership relationship. They won’t cause a
reference cycle because any cycle involving some weak references will be broken
once the strong reference count of values involved is 0.

Because the value that `Weak<T>` references might have been dropped, to do
anything with the value that a `Weak<T>` is pointing to, you must make sure the
value still exists. Do this by calling the `upgrade` method on a `Weak<T>`
instance, which will return an `Option<Rc<T>>`. You’ll get a result of `Some`
if the `Rc<T>` value has not been dropped yet and a result of `None` if the
`Rc<T>` value has been dropped. Because `upgrade` returns an `Option<Rc<T>>`,
Rust will ensure that the `Some` case and the `None` case are handled, and
there won’t be an invalid pointer.

As an example, rather than using a list whose items know only about the next
item, we’ll create a tree whose items know about their children items *and*
their parent items.

#### Creating a Tree Data Structure: a `Node` with Child Nodes

To start, we’ll build a tree with nodes that know about their child nodes.
We’ll create a struct named `Node` that holds its own `i32` value as well as
references to its children `Node` values:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

We want a `Node` to own its children, and we want to share that ownership with
variables so we can access each `Node` in the tree directly. To do this, we
define the `Vec<T>` items to be values of type `Rc<Node>`. We also want to
modify which nodes are children of another node, so we have a `RefCell<T>` in
`children` around the `Vec<Rc<Node>>`.

Next, we’ll use our struct definition and create one `Node` instance named
`leaf` with the value 3 and no children, and another instance named `branch`
with the value 5 and `leaf` as one of its children, as shown in Listing 15-27:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

<span class="caption">Listing 15-27: Creating a `leaf` node with no children
and a `branch` node with `leaf` as one of its children</span>

We clone the `Rc<Node>` in `leaf` and store that in `branch`, meaning the
`Node` in `leaf` now has two owners: `leaf` and `branch`. We can get from
`branch` to `leaf` through `branch.children`, but there’s no way to get from
`leaf` to `branch`. The reason is that `leaf` has no reference to `branch` and
doesn’t know they’re related. We want `leaf` to know that `branch` is its
parent. We’ll do that next.

#### Adding a Reference from a Child to Its Parent

To make the child node aware of its parent, we need to add a `parent` field to
our `Node` struct definition. The trouble is in deciding what the type of
`parent` should be. We know it can’t contain an `Rc<T>`, because that would
create a reference cycle with `leaf.parent` pointing to `branch` and
`branch.children` pointing to `leaf`, which would cause their `strong_count`
values to never be 0.

Thinking about the relationships another way, a parent node should own its
children: if a parent node is dropped, its child nodes should be dropped as
well. However, a child should not own its parent: if we drop a child node, the
parent should still exist. This is a case for weak references!

So instead of `Rc<T>`, we’ll make the type of `parent` use `Weak<T>`,
specifically a `RefCell<Weak<Node>>`. Now our `Node` struct definition looks
like this:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

A node will be able to refer to its parent node but doesn’t own its parent.
In Listing 15-28, we update `main` to use this new definition so the `leaf`
node will have a way to refer to its parent, `branch`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

<span class="caption">Listing 15-28: A `leaf` node with a weak reference to its
parent node `branch`</span>

Creating the `leaf` node looks similar to how creating the `leaf` node looked
in Listing 15-27 with the exception of the `parent` field: `leaf` starts out
without a parent, so we create a new, empty `Weak<Node>` reference instance.

At this point, when we try to get a reference to the parent of `leaf` by using
the `upgrade` method, we get a `None` value. We see this in the output from the
first `println!` statement:

```text
leaf parent = None
```

When we create the `branch` node, it will also have a new `Weak<Node>`
reference in the `parent` field, because `branch` doesn’t have a parent node.
We still have `leaf` as one of the children of `branch`. Once we have the
`Node` instance in `branch`, we can modify `leaf` to give it a `Weak<Node>`
reference to its parent. We use the `borrow_mut` method on the
`RefCell<Weak<Node>>` in the `parent` field of `leaf`, and then we use the
`Rc::downgrade` function to create a `Weak<Node>` reference to `branch` from
the `Rc<Node>` in `branch.`

When we print the parent of `leaf` again, this time we’ll get a `Some` variant
holding `branch`: now `leaf` can access its parent! When we print `leaf`, we
also avoid the cycle that eventually ended in a stack overflow like we had in
Listing 15-26; the `Weak<Node>` references are printed as `(Weak)`:

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

The lack of infinite output indicates that this code didn’t create a reference
cycle. We can also tell this by looking at the values we get from calling
`Rc::strong_count` and `Rc::weak_count`.

#### Visualizing Changes to `strong_count` and `weak_count`

Let’s look at how the `strong_count` and `weak_count` values of the `Rc<Node>`
instances change by creating a new inner scope and moving the creation of
`branch` into that scope. By doing so, we can see what happens when `branch` is
created and then dropped when it goes out of scope. The modifications are shown
in Listing 15-29:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

<span class="caption">Listing 15-29: Creating `branch` in an inner scope and
examining strong and weak reference counts</span>

After `leaf` is created, its `Rc<Node>` has a strong count of 1 and a weak
count of 0. In the inner scope, we create `branch` and associate it with
`leaf`, at which point when we print the counts, the `Rc<Node>` in `branch`
will have a strong count of 1 and a weak count of 1 (for `leaf.parent` pointing
to `branch` with a `Weak<Node>`). When we print the counts in `leaf`, we’ll see
it will have a strong count of 2, because `branch` now has a clone of the
`Rc<Node>` of `leaf` stored in `branch.children`, but will still have a weak
count of 0.

When the inner scope ends, `branch` goes out of scope and the strong count of
the `Rc<Node>` decreases to 0, so its `Node` is dropped. The weak count of 1
from `leaf.parent` has no bearing on whether or not `Node` is dropped, so we
don’t get any memory leaks!

If we try to access the parent of `leaf` after the end of the scope, we’ll get
`None` again. At the end of the program, the `Rc<Node>` in `leaf` has a strong
count of 1 and a weak count of 0, because the variable `leaf` is now the only
reference to the `Rc<Node>` again.

All of the logic that manages the counts and value dropping is built into
`Rc<T>` and `Weak<T>` and their implementations of the `Drop` trait. By
specifying that the relationship from a child to its parent should be a
`Weak<T>` reference in the definition of `Node`, you’re able to have parent
nodes point to child nodes and vice versa without creating a reference cycle
and memory leaks.

## Summary

This chapter covered how to use smart pointers to make different guarantees and
trade-offs from those Rust makes by default with regular references. The
`Box<T>` type has a known size and points to data allocated on the heap. The
`Rc<T>` type keeps track of the number of references to data on the heap so
that data can have multiple owners. The `RefCell<T>` type with its interior
mutability gives us a type that we can use when we need an immutable type but
need to change an inner value of that type; it also enforces the borrowing
rules at runtime instead of at compile time.

Also discussed were the `Deref` and `Drop` traits, which enable a lot of the
functionality of smart pointers. We explored reference cycles that can cause
memory leaks and how to prevent them using `Weak<T>`.

If this chapter has piqued your interest and you want to implement your own
smart pointers, check out [“The Rustonomicon”][nomicon] for more useful
information.

Next, we’ll talk about concurrency in Rust. You’ll even learn about a few new
smart pointers.

[nomicon]: ../nomicon/index.html
