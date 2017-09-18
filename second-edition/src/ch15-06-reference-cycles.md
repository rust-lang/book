## Reference Cycles Can Leak Memory

Rust's memory safety guarantees make it *difficult* to accidentally create
memory that's never cleaned up, known as a *memory leak*, but not impossible.
Entirely preventing memory leaks is not one of Rust's guarantees in the same
way that disallowing data races at compile time is, meaning memory leaks are
memory safe in Rust. We can see this with `Rc<T>` and `RefCell<T>`: it's
possible to create references where items refer to each other in a cycle. This
creates memory leaks because the reference count of each item in the cycle will
never reach 0, and the values will never be dropped.

### Creating a Reference Cycle

Let's take a look at how a reference cycle might happen and how to prevent it,
starting with the definition of the `List` enum and a `tail` method in Listing
15-20:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

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

<span class="caption">Listing 15-20: A cons list definition that holds a
`RefCell` so that we can modify what a `Cons` variant is referring to</span>

We're using another variation of the `List` definition from Listing 15-6. The
second element in the `Cons` variant is now `RefCell<Rc<List>>`, meaning that
instead of having the ability to modify the `i32` value like we did in Listing
15-19, we want to be able to modify which `List` a `Cons` variant is pointing
to. We've also added a `tail` method to make it convenient for us to access the
second item, if we have a `Cons` variant.

<!-- Can you link this more clearly, what do we have at this point? This change
to a new listing feels unexpected. What are we going to do with this cons list?
Why are we making this next listing, what is it's overall purpose? -->
<!-- I'm not sure if the new listing you're talking about being unexpected is
referring to the listing above or the listing below? The listing above is just
definitions we're going to use, the listing below is the `main` function that
uses the definitions. We just broke these apart to avoid having a lot of code
and then a lot of explanation, I'd be fine having this be one big listing if
you think that would be better /Carol -->

In listing 15-21, we're adding a `main` function that uses the definitions from
Listing 15-20. This code creates a list in `a`, a list in `b` that points to
the list in `a`, and then modifies the list in `a` to point to `b`, which
creates a reference cycle. There are `println!` statements along the way to
show what the reference counts are at various points in this process.

<!-- so are we adding this to the end of the previous listing? It's in the same
file -->
<!-- yes /Carol -->

<span class="filename">Filename: src/main.rs</span>

```rust
# use List::{Cons, Nil};
# use std::rc::Rc;
# use std::cell::RefCell;
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
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle; it will
    // overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

<span class="caption">Listing 15-21: Creating a reference cycle of two `List`
values pointing to each other</span>

We create an `Rc` instance holding a `List` value in the variable `a` with an
initial list of `5, Nil`. We then create an `Rc` instance holding another
`List` value in the variable `b` that contains the value 10, then points to the
list in `a`.

Finally, we modify `a` so that it points to `b` instead of `Nil`, which creates
a cycle. We do that by using the `tail` method to get a reference to the
`RefCell` in `a`, which we put in the variable `link`. Then we use the
`borrow_mut` method on the `RefCell` to change the value inside from an `Rc`
that holds a `Nil` value to the `Rc` in `b`.

If we run this code, keeping the last `println!` commented out for the moment,
we'll get this output:

```text
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
```

We can see that the reference count of the `Rc` instances in both `a` and `b`
are 2 after we change the list in `a` to point to `b`. At the end of `main`,
Rust will try and drop `b` first, which will decrease the count in each of the
`Rc` instances in `a` and `b` by one.

<!-- Above -- previously `a` and `b` said `Rc`, I wanted to clarify that by Rc
we mean a and b, is that right? -->
<!-- There's lots of stuff in `a` and `b`; we specifically mean the `Rc` values
here which is why we said `Rc`. I've tried to say both `a` & `b` and `Rc` here
instead, to be most precise. What do you think? /Carol -->

<!-- Below--"that Rc" - what are we referring to, a is still referencing b? Can
you clarify that? -->
<!-- Yes, the `Rc` in `b`. /Carol -->

However, because `a` is still referencing the `Rc` that was in `b`, that `Rc`
has a count of 1 rather than 0, so the memory the `Rc` has on the heap won't be
dropped. The memory will just sit there with a count of one, forever.

To visualize this, we've created a reference cycle that looks like Figure 15-22:

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />

<span class="caption">Figure 15-22: A reference cycle of lists `a` and `b`
pointing to each other</span>

If you uncomment the last `println!` and run the program, Rust will try and
print this cycle out with `a` pointing to `b` pointing to `a` and so forth
until it overflows the stack.

<!-- Can you show us the output? Also, why are we commenting out the print
statement in the first place?-->
<!-- We have the last println commented out to begin with because otherwise you
get a LOT of output until the stack overflows. We thought that would be
confusing and make it harder to see the reference counts we're printing out
before that point. Did you try the code with and without that line commented
out? Which one would make a better first experience when running this code?
/Carol -->

In this specific case, right after we create the reference cycle, the program
ends. The consequences of this cycle aren't so dire. If a more complex program
allocates lots of memory in a cycle and holds onto it for a long time, the
program would be using more memory than it needs, and might overwhelm the
system and cause it to run out of available memory.

Creating reference cycles is not easily done, but it's not impossible either.
If you have `RefCell<T>` values that contain `Rc<T>` values or similar nested
combinations of types with interior mutability and reference counting, be aware
that you have to ensure you don't create cycles yourself; you can't rely on
Rust to catch them. Creating a reference cycle would be a logic bug in your
program that you should use automated tests, code reviews, and other software
development practices to minimize.

<!-- Above-- this seems like a vague solution, just not writing the code that
creates cycles, can you be more specific about which part they should
exclude/change? -->
<!-- Not really, this example was deliberately creating a reference cycle, so
if you don't want reference cycles, you shouldn't write this code. It's similar
to a logic bug-- if you want your program to add 2 to a number instead of 50,
then you have to type 2 rather than typing 50. I'm not sure how to be more
specific or helpful here; I've referenced writing tests and other things that
can help mitigate logic bugs. /Carol -->

Another solution is reorganizing your data structures so that some references
express ownership and some references don't. In this way, we can have cycles
made up of some ownership relationships and some non-ownership relationships,
and only the ownership relationships affect whether a value may be dropped or
not. In Listing 15-20, we always want `Cons` variants to own their list, so
reorganizing the data structure isn't possible. Let's look at an example using
graphs made up of parent nodes and child nodes to see when non-ownership
relationships are an appropriate way to prevent reference cycles.

### Preventing Reference Cycles: Turn an `Rc<T>` into a `Weak<T>`

So far, we've shown how calling `Rc::clone` increases the `strong_count` of an
`Rc` instance, and that an `Rc` instance is only cleaned up if its
`strong_count` is 0. We can also create a *weak reference* to the value within
an `Rc` instance by calling `Rc::downgrade` and passing a reference to the
`Rc`. When we call `Rc::downgrade`, we get a smart pointer of type `Weak<T>`.
Instead of increasing the `strong_count` in the `Rc` instance by one, calling
`Rc::downgrade` increases the `weak_count` by one. The `Rc` type uses
`weak_count` to keep track of how many `Weak<T>` references exist, similarly to
`strong_count`. The difference is the `weak_count` does not need to be 0 in
order for the `Rc` instance to be cleaned up.

<!-- What is a weak_count? I don't think we've defined that, or strong_count,
really. Are we just giving another variable to store the count that has no
input on whether memory is dropped? When is a count stored in strong_count and
when is it stored in weak_count? -->
<!-- We're not giving `Rc` another variable, the standard library has defined
`Rc` to have both the `strong_count` and `weak_count` as fields. I've tried to
clarify the paragraph above to address your questions. /Carol -->

Strong references are how we can share ownership of an `Rc` instance. Weak
references don't express an ownership relationship. They won't cause a
reference cycle since any cycle involving some weak references will be broken
once the strong reference count of values involved is 0.

<!-- Below: I'm struggling to follow here, why do we want to get a value from
Weak<T>? This section is losing me somewhat, can you slow this down, make sure
you define anything new up front and give it's purpose, what we intend it to
do? -->
<!-- I've tried to clarify /Carol -->

Because the value that `Weak<T>` references might have been dropped, in order
to do anything with the value that a `Weak<T>` is pointing to, we have to check
to make sure the value is still around. We do this by calling the `upgrade`
method on a `Weak<T>` instance, which will return an `Option<Rc<T>>`. We'll get
a result of `Some` if the `Rc` value has not been dropped yet, and `None` if
the `Rc` value has been dropped. Because `upgrade` returns an `Option`, we can
be sure that Rust will handle both the `Some` case and the `None` case, and
there won't be an invalid pointer.

As an example, rather than using a list whose items know only about the next
item, we'll create a tree whose items know about their children items *and*
their parent items.

#### Creating a Tree Data Structure: a `Node` with Child Nodes

To start building this tree, we'll create a struct named `Node` that holds its
own `i32` value as well as references to its children `Node` values:

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

We want a `Node` to own its children, and we want to be able to share that
ownership with variables so we can access each `Node` in the tree directly. To
do this, we define the `Vec` items to be values of type `Rc<Node>`. We also
want to be able to modify which nodes are children of another node, so we have
a `RefCell` in `children` around the `Vec`.

Next, let's use our struct definition and create one `Node` instance named
`leaf` with the value 3 and no children, and another instance named `branch`
with the value 5 and `leaf` as one of its children, as shown in Listing 15-23:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::rc::Rc;
# use std::cell::RefCell;
#
# #[derive(Debug)]
# struct Node {
#     value: i32,
#    children: RefCell<Vec<Rc<Node>>>,
# }
#
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

<span class="caption">Listing 15-23: Creating a `leaf` node with no children
and a `branch` node with `leaf` as one of its children</span>

We clone the `Rc` in `leaf` and store that in `branch`, meaning the `Node` in
`leaf` now has two owners: `leaf` and `branch`. We can get from `branch` to
`leaf` through `branch.children`, but there's no way to get from `leaf` to
`branch`. `leaf` has no reference to `branch` and doesn't know they are
related. We'd like `leaf` to know that `branch` is its parent.

#### Adding a Reference from a Child to its Parent

To make the child node aware of its parent, we need to add a `parent` field to
our `Node` struct definition. The trouble is in deciding what the type of
`parent` should be. We know it can't contain an `Rc<T>` because that would
create a reference cycle, with `leaf.parent` pointing to `branch` and
`branch.children` pointing to `leaf`, which would cause their `strong_count`
values to never be zero.

Thinking about the relationships another way, a parent node should own its
children: if a parent node is dropped, its child nodes should be dropped as
well. However, a child should not own its parent: if we drop a child node, the
parent should still exist. This is a case for weak references!

So instead of `Rc`, we'll make the type of `parent` use `Weak<T>`, specifically
a `RefCell<Weak<Node>>`. Now our `Node` struct definition looks like this:

<!-- I think because I still don't understand what Weak<T> is, I'm not really
sure what it means for the parent to use Weak<T>, can you make sure that's
clear at this point -->
<!-- I've tried, I'm not sure though /Carol -->

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

<!-- Can you fill out this line, above; talk through the syntax, too? Also,
below, how does this mean a node can refer to a parent without owning it?
What's is actually doing here?-->
<!-- The first line is importing `Weak` from the standard library; the reader
really should be familiar with bringing types into scope by this point, don't
you think? It seems repetitive to explain this every time. /Carol
-->

This way, a node will be able to refer to its parent node, but does not own its
parent. In Listing 15-24, let's update `main` to use this new definition so
that the `leaf` node will have a way to refer to its parent, `branch`:

<!-- Why are we updating it, what are we doing here? Can you make that clear?
-->
<!-- Done /Carol -->

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::rc::{Rc, Weak};
# use std::cell::RefCell;
#
# #[derive(Debug)]
# struct Node {
#     value: i32,
#     parent: RefCell<Weak<Node>>,
#     children: RefCell<Vec<Rc<Node>>>,
# }
#
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
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

<span class="caption">Listing 15-24: A `leaf` node with a `Weak` reference to
its parent node, `branch`</span>

<!-- Below: looks similar to what? What are we doing with this listing, can you
talk it through -->

Creating the `leaf` node looks similar to how creating the `leaf` node looked
in Listing 15-23, with the exception of the `parent` field: `leaf` starts out
without a parent, so we create a new, empty `Weak` reference instance.

At this point, when we try to get a reference to the parent of `leaf` by using
the `upgrade` method, we get a `None` value. We see this in the output from the
first `println!`:

```text
leaf parent = None
```

<!-- Is this the explanation of the previous program? If so, can you change the
tone to an active tone, make it clear that it's connected? I'm struggling to
connect things up -->
<!-- I've tried, this will be better with wingdings /Carol -->

When we create the `branch` node, it will also have a new `Weak` reference,
since `branch` does not have a parent node. We still have `leaf` as one of the
children of `branch`. Once we have the `Node` instance in `branch`, we can
modify `leaf` to give it a `Weak` reference to its parent. We use the
`borrow_mut` method on the `RefCell` in the `parent` field of `leaf`, then we
use the `Rc::downgrade` function to create a `Weak` reference to `branch` from
the `Rc` in `branch.`

<!-- Below: What does this mean for our program, that now leaf recognizes its
parent? -->
<!-- Yes /Carol -->

When we print out the parent of `leaf` again, this time we'll get a `Some`
variant holding `branch`: `leaf` can now access its parent! When we print out
`leaf`, we also avoid the cycle that eventually ended in a stack overflow like
we had in Listing 15-21: the `Weak` references are printed as `(Weak)`:

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

The lack of infinite output indicates that this code didn't create a reference
cycle. We can also tell this by looking at the values we get from calling
`Rc::strong_count` and `Rc::weak_count`.

#### Visualizing Changes to `strong_count` and `weak_count`

Let's look at how the `strong_count` and `weak_count` values of the `Rc`
instances change by creating a new inner scope and moving the creation of
`branch` into that scope. This will let us see what happens when `branch` is
created and then dropped when it goes out of scope. The modifications are shown
in Listing 15-25:

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
            children: RefCell::new(vec![Rc::clone(&leaf)]),
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

<span class="caption">Listing 15-25: Creating `branch` in an inner scope and
examining strong and weak reference counts</span>

Once `leaf` is created, its `Rc` has a strong count of 1 and a weak count of 0.
In the inner scope we create `branch` and associate it with `leaf`, at which
point the `Rc` in `branch` will have a strong count of 1 and a weak count of 1
(for `leaf.parent` pointing to `branch` with a `Weak<T>`). Here `leaf` will
have a strong count of 2, because `branch` now has a clone of the `Rc` of
`leaf` stored in `branch.children`, but will still have a weak count of 0.

When the inner scope ends, `branch` goes out of scope and the strong count of
the `Rc` decreases to 0, so its `Node` gets dropped. The weak count of 1 from
`leaf.parent` has no bearing on whether `Node` is dropped or not, so we don't
get any memory leaks!

If we try to access the parent of `leaf` after the end of the scope, we'll get
`None` again. At the end of the program, the `Rc` in `leaf` has a strong count
of 1 and a weak count of 0, because the variable `leaf` is now the only
reference to the `Rc` again.

<!-- Just to clarify, leaf is pointing to itself? -->
<!-- `leaf` is the variable pointing to the `Rc`, the `Rc` is what has the
strong and weak counts. /Carol -->

All of the logic that manages the counts and value dropping is built in to
`Rc` and `Weak` and their implementations of the `Drop` trait. By specifying
that the relationship from a child to its parent should be a `Weak<T>`
reference in the definition of `Node`, we're able to have parent nodes point to
child nodes and vice versa without creating a reference cycle and memory leaks.

<!-- Ah! This actually cleared up a lot, we specify in the definition that a
reference should be weak and therefore ignored by the Drop trait, is that
right? It would really help to specify that up front, can you add something
like that to the start of the Weak section? -->
<!-- Done /Carol -->

## Summary

This chapter covered how you can use smart pointers to make different
guarantees and tradeoffs than those Rust makes by default with regular
references. `Box<T>` has a known size and points to data allocated on the heap.
`Rc<T>` keeps track of the number of references to data on the heap so that
data can have multiple owners. `RefCell<T>` with its interior mutability gives
us a type that can be used when we need an immutable type but need the ability
to change an inner value of that type, and enforces the borrowing rules at
runtime instead of at compile time.

We also discussed the `Deref` and `Drop` traits that enable a lot of the
functionality of smart pointers. We explored reference cycles that can cause
memory leaks, and how to prevent them using `Weak<T>`.

If this chapter has piqued your interest and you want to implement your own
smart pointers, check out [The Nomicon] for even more useful information.

[The Nomicon]: https://doc.rust-lang.org/stable/nomicon/

Next, let's talk about concurrency in Rust. We'll even learn about a few new
smart pointers.
