## Method Syntax

<!-- This seems like a bit of a segue, is there a reason we include this method
section here? If it's arbitrary, we might want to look at the arrangement of
the chapters, there ought to be a logical arrangement so the reader can
navigate easily. Otherwise, can you include some kind of introduction that says
why we're going into methods here? -->

In Chapter 4 when we discussed ownership, we made several references to
*methods*. Here's an example of the `clone` method in use:

```rust
let s1 = "hello";

// call a method on s1
let s2 = s1.clone();

println!("{}", s1);
```

The call to `clone()` is attached to `s1` with a dot. This is the *method
syntax*, and it’s a way to call certain functions with a different style.

<!-- Can you expand on what we mean by a different style? I'm not sure this
sections sells methods fully, is there no other reason you'd use a method that
for easier nesting? It doesn't seem like we go into deeper, ownership reasons,
like we say we will below -->

Why have two ways to call functions? We’ll talk about some deeper reasons
related to ownership in a moment, but one big reason is that methods are much
more readable when chained together than functions. Here's the same chaining
example using both methods and functions:

```rust,ignore
// with functions
h(g(f(x)));

// with methods
x.f().g().h();
```

The nested-functions version is read by Rust in reverse: the program executes
`f()`, then `g()`, then `h()`, but we read it left-to-right as `h()`, then
`g()`, then `f()`. This could be confusing if you need your functions exectuted
in a specific order.

The method syntax on the other hand is executed in the same order as we would
read it, and is listed rather than nested to make it much easier to read.

Before we get into the details, let’s talk about how to define your own
methods.

### Defining Methods

We can define methods with the `impl` keyword, short for *implementation*.
Doing so looks like this:

```rust
#[derive(Debug,Copy,Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);

        f64::sqrt(x_squared + y_squared)
    }
}

let p1 = Point { x: 0.0, y: 0.0 };
let p2 = Point { x: 5.0, y: 6.5 };

assert_eq!(8.200609733428363, p1.distance(&p2));
```

Let’s break this down. First, we define our `Point` struct from earlier in the
chapter. Next comes our first use of the `impl` keyword, followed by a call to
`Point`:

```rust,ignore
impl Point {
    // ...
}
```

<!-- We might want to use the wingding numbers here too, to save repetition of
code in the text and put the explanations in context. -->

Everything we put inside the curly braces here is a method to be implemented on
`Point`. Next we give our definition of the method, here doing the same as our
`distance` fuction from earlier.

<!-- ```rust,ignore
fn distance(&self, other: &Point) -> f64 {
    // ...
}
``` -->

Other than this, the rest of the example is familiar: an implementation of
`distance()` and use of the method to find an answer.

Our definition of `distance()` here as a method looks very similar to our
previous definition of `distance()` as a function, but with two differences.
Here's the `distance()` function again followed by our new `distance()` method:

```rust,ignore
fn distance(p1: Point, p2: Point) -> f64 { // function
    // ...

    fn distance(&self, other: &Point) -> f64 {  // method
    // ...
}
}
```

<!-- What's the second difference? I can't see that we go over it, unless you
mean the assert line? In which case, we might need to rephrase, this reads like
there are two differences in this line alone-->

The first difference is in the first argument. In the method version we replace
the name and type with `&self`. This is the main distinction from a function;
we use `self` inside of an `impl` block in a method because we already know
that we are implementing this method on `Point`, due to the surrounding `impl
Point` block, so we don’t need to write the type of `self` out again.

<!-- Have we mentioned `self` before here in the book? If not, I'd suggest
going into a little more detail, I'm not sure this makes it clear what self is
and how you'd use it in other situations. Especially since in these two
examples we seem to be giving the type, f64, in both anyway? Is this a
siginificant enough concept to warrant its own section? If it isn't that
significant, I wonder if we want to make this self section a box, it seems
quite abrupt here? -->


Note that we have written `&self`, with the reference syntax, rather than
`self`, because we want to take a reference to our argument's value rather than
taking ownership of it.

<!-- Do you want to point out here why we're taking a reference and not
ownership in this example? -->

In other words, these two forms are the same:

```rust,ignore
fn foo(self: &Point)
fn foo(&self)
```

In both we are taking a reference of `Point`.

Self is a parameter like any other, and as such you can take `self` in three
forms:

```rust,ignore
fn foo(&self) // take self by reference
fn foo(&mut self) // take self by mutable reference
fn foo(self) // take self by ownership
```

<!-- It might help to explain why it's so much more common to take self by
immutable reference that any other way? -->

Taking `self` by reference is the most common, followed by mutable reference,
and the least common is taking `self` by ownsership. In this case, we only need
a reference of `Point`, and we don’t need to mutate either `Point` to get the
distance between them, so we won't take a mutable reference to . Methods that
take ownership of `self` are rarely used. One of the few times we might take
ownership of `self` would be if we needed a method that would transform `self`
into something else and prevent other code from using the value of `self` after
the transformation happens.

<!-- We haven't mentioned the `other` terminology here, but I think that's new
too, right? Could you add an explanation? I think we ought to, since it's new,
but if you are wanting to save it for another chapter maybe just mention that,
so the reader's not left wondering. -->

#### Methods and Automatic Referencing

There's another new bit of information in this last script, this last line of
the example:

```rust,ignore
assert_eq!(8.200609733428363, p1.distance(&p2));
```

<!-- what is this final line, what does the assert section mean/do? What's the
long floating point for? -->

When we defined `distance()`, we took both `self` and the `other` argument by
reference. Yet, in this final line, we needed a `&` for `p2` but not `p1`. What
gives?

This feature is called *automatic referencing*, and calling methods is one of
the few places in Rust that has behavior like this. When you call a method with
`self.(`, Rust will automatically add in `&`s or `&mut`s to match the
signature. In other words, these are the same:

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

The first one looks much, much cleaner.
<!-- I think I'm following, but why do we add the reference to the second entry, p2, and not p1? -->

Here’s another example:

```rust
let mut s = String::from("Hello,");

s.push_str(" world!");

// The above is the same as:
// (&mut s).push_str(" world!");

assert_eq!("Hello, world!", s);
```

<!-- is this an unfinished sentence below? Got a bit lost! -->

Because `push_str()` has the following signature:

```rust,ignore
fn push_str(&mut self, string: &str) {
```

This automatic referencing behavior works because methods have a clear receiver
— the type of `self` — and in most cases it’s clear given the receiver and name
of a method whether the method is just reading (so needs `&self`), mutating (so
`&mut self`), or consuming (so `self`). The fact that Rust makes borrowing
implicit for method receivers is a big part of making ownership ergonomic in
practice.

<!-- I dd find this automatic referencing section quite hard to read, I think
it could use a little fleshing out to make it clearer, talk through the
examples a bit more, that kind of thing. -->


<!-- Could you give a chapter summary? -->
