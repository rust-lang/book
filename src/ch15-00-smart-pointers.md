# Smart Pointers

Now that we've learned quite a bit of Rust, we can start digging into some more
complicated things. In this chapter, we'll learn about a design pattern in Rust
called a "smart pointer." This pattern allows you to leverage Rust's ownership
and borrowing features to manage all kinds of resources in a safe way.

We'll be covering:

* What smart pointers are
* The `Deref` and `Drop` traits
* `Box<T>`, the simplest smart pointer
* Other common smart pointers in the standard library, like `Rc<T>`.
* A family of smart pointers called "cells", which give you "interior
  mutability."

Let's dive in!

## What are smart pointers

So what are smart pointers, anyway? Well, we've learned about references in
Rust. "Pointer" is a generic term for something like a reference, that is,
pointers "point at" something else. References are a kind of pointer that only
borrows data; in many cases, smart pointers *own* the data that they point to.
That is, they have extra capabilities that references don't, hence the "smart"
nickname.

You've already encounted a few smart pointers in the book, we didn't call them
that by name, though. For example, in a certian sense, `String` and `Vec<T>`
are both smart pointers. They own some memory, and allow you to manipualate it.
Another good example is `File`, which we used for our I/O project in chapter
12: it manages a file handle that the operating system gives us.

Given that this is a design pattern in Rust, this chapter won't cover every
smart pointer that exists. Many libraries will build their own, as well, and
you may write some for your own code. The ones we cover here will be the most
common ones from the standard library.

## `Box<T>`

The most straightforward smart pointer is a *box*, more properly written as
`Box<T>`. Boxes allow you to put a single value on the heap. (We talked about
the stack vs. the heap in chapter 4.) Using a box looks like this:

```rust
{
    let b = Box::new(5);

    // other code

} // b goes out of scope here, and is deallocated
```

Just like any value that has ownership, when a box goes out of scope, it will
be dealloacated.

It turns out that putting a single value on the heap isn't very useful, so you
won't use boxes very often. When do you need a box? When you want to ensure
that your type has a known size. What does that mean? Consider this struct:

```rust,ignore
enum List<T> {
    Cons(T, List<T>),
    Nil,
}
```

This is a 'cons list', that is, a list where each item in the list contains the
next item, until the end, which is called `Nil`. Using one would look like this:

```rust,ignore
use List::{Cons, Nil};

let list = Cons(1, Cons(2, Cons(3, Nil)));
```

But if we try to compile the above code, we get an error:

```text
error[E0072]: recursive type `main::List` has infinite size
 --> <anon>:3:1
  |
3 | enum List<T> {
  | ^ recursive type has infinite size
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
          make `main::List` representable
```

The type 'has infinite size'. Why is that? Well, `List<T>` is generic. We
can create a `List<T>` of integers, of strings, of, well, anything. But
different types can take up different amounts of memory. So when the Rust
compiler looks at a type like this:

```rust
enum SomeEnum<T> {
    A(T),
    B,
}
```

It says "How much memory do I need to allocate for a value of `SomeEnum`?
Let's look at `A`. Well, it has a value of type `T`, so we can use however much
memory `T` needs. Then, let's look at `B`. It doesn't save a value, so we don't
need any extra memory. Done." But consider a *recursive* type, like this:

```rust,ignore
enum SomeEnum<T> {
    A(SomeEnum<T>),
    B,
}
```

Now the compiler will say, "How much memory do I need to allocate for a value
of `SomeEnum`?  Let's look at `A`. Well, it has a value of type `SomeEnum<T>`,
so we need to figure out how much memory a `SomeEnum<T>` needs. Let's look at
`A`. Well, it has a value of type `SomeEnum<T>`, so we need to figure out how
much memory a `SomeEnum<T>` needs. Let's look at `A`. Well, it has a value of
type `SomeEnum<T>`, so we need to figure out how much memory a `SomeEnum<T>`
needs. Let's look at..." into infinity. In order to figure out how much memory
`SomeEnum::A` needs, we need to figure out how much memory `SomeEnum::A` needs.
It's impossible to know!

The compiler has a helpful suggestion in the error output:

```text
= help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
        make `main::List` representable
```

Because a `Box<T>` is a pointer, we always know what size it is: a `usize`.
So if we change our definition to look like this:

```rust,ignore
enum SomeEnum<T> {
    A(Box<SomeEnum<T>>),
    B,
}
```

The compiler will look at this type, and say "How much memory do I need to
allocate for a value of `SomeEnum`?  Let's look at `A`. Well, it has a value of
type `Box<SomeEnum<T>>`, and we know that a box always has the size of a
`usize`. Then, let's look at `B`. It doesn't save a value, so we don't need any
extra memory. Done." We've broken the infinite, recursive chain by adding in a
box. This is the main area where boxes are useful: breaking up an infinite data
structure so that the compiler can know what size it is. We'll look at a second
variant of this problem in chapter 17, with 'trait objects'.

Even though you won't be using boxes very often, they are a good way to
understand the smart pointer pattern. What makes boxes smart pointers? Largely,
it's through implementing two important traits: `Deref` and `Drop`. Let's
investigate how boxes use these in more detail.

## The `Deref` trait

The first important trait is `Deref`. It allows us to override the `*`
operator. Remember it from when we talked about references? It looks like this:

```rust
let mut x = 5;
{
    let y = &mut x;
    
    *y += 1
}

assert_eq!(6, x);
```

We use `*y` to access the thing that `y` refers to, rather than `y` itself.
Here's an example of overloading `*` using `Deref`:

```rust
use std::ops::Deref;

struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

fn main() {
    let example = DerefExample { value: 'a' };

    assert_eq!('a', *example);
}
```

Most of this should look familliar: a struct, a trait, a main function. There
is one tricky bit: like we said in chapter 13 on Iterators, the `type Target =
T;` syntax is "associated types", which is covered in Chapter 20. Don't worry
about it too much, it is a slightly different way of delcaring a generic
parameter.

if you look at the `assert_eq!`, we're comparing `'a'` to `*example`: the `*`
is what calls `Deref::deref`. And in the implementation, we can see that we
return a reference to our `value` field, so that's the result we get. Easy!

### `Deref` coercions.

There's one other trick with `Deref`: it's one place where Rust will do
automatic coercions. Consider this code:

```rust
fn takes_a_string_slice(s: &str) {
    println!("got: {}", s);
}

let hello = String::from("hello");

takes_a_string_slice(&hello);
```

This code _shouldn't_ work, but it does. `takes_a_string_slice`, well, takes a
string slice, a `&str`, as an argument. We pass it `&hello`, which, given that
`hello` is a `String`, should be a `&String`. What gives?

It turns out that `String`s implement `Deref`, and when they do, they return
string slices! This means that `&String` will automatically coerce to a slice
of the full string. 

There's also a `DerefMut` trait for overriding `*` on `&mut T`s in the same
fashion that we use `Deref` to override `*` on `&T`s.

Rust does this kind of coercion in three cases:

* From `&T` to `&U` when `T: Deref<U>`.
* From `&mut T` to `&mut U` when `T: DerefMut<U>`.
* From `&mut T` to `&U` when `T: Deref<U>`.

The first two are the same, except for mutability: if you have a `&T`, and
`T` implements `Deref` to some type `U`, you can get a `&U` transparently. Same
for mutable references. The last one is more tricky: if you have a mutable
reference, it will also coerece to an immutable one. The other case is _not_
possible though: immutable references will never coerece to mutable ones.

## The `Drop` trait

Next up: `Drop`. `Drop` lets us run some code when something is about to go out
of scope. It looks like this:

```rust
struct Fireworks {
    number: i32,
}

// Drop is in the prelude, so we don't need to import it
impl Drop for Fireworks {
    fn drop(&mut self) {
        println!("BOOM! {} fireworks explode!", self.number);
    }
}

fn main() {
    let f = Fireworks { number: 10 };
    println!("Fireworks created.");
    println!("Wait for it...");
}
```

When we run this program, we'll see

```text
Fireworks created.
Wait for it...
BOOM! 10 fireworks explode!
```

printed to the screen. Our message is printed, and then when `f` goes out of
scope at the end of `main`, `drop` is called, and we see the message about
exploding printed. We can also use the `drop` function to call `Drop` a bit
earlier:

```rust,ignore
fn main() {
    let f = Fireworks { number: 10 };
    println!("Fireworks created.");
    drop(f);
    println!("Wait for it...");
}
```

This will print:

```text
Fireworks created.
BOOM! 10 fireworks explode!
Wait for it...
```

Normally, you wouldn't want to wait for fireworks _after_ they explode, but you
get the idea.

The ability to run some code when something goes out of scope is very powerful.
For example, here's something that looks similar to a box. This code won't
_work_, but it illustrates the concept:

```rust,ignore
struct MyBox<T> {
    data: &T,
}

impl<T> MyBox<T> {
    fn new(data: T) {
        // allocate some memory
        let mut memory = allocate_memory(); 

        // copy the data into that memory
        *memory = T;

        // return a box containing the reference to the memory
        MyBox {
            data: memory,
        } 
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        free_memory(self.data);
    }
}
```

So first, we declare `MyBox<T>` to have a reference to some data. We can't use
actual `&T` references for this, we'd use `*const T`, but we won't talk about
*raw pointer*s until Chapter 20. So for now, pretend that this works; it's the
same idea.

In `new`, we allocate some memory by asking the operating system for it. This
function doesn't exist in Rust today, so we made up a function name,
`allocate_memory`, for it. This function would return a reference to the new
memory we've been granted. We then copy our data into it by using the `*`
operator. So after the assignment, `memory` will be pointing to a copy of our
`data`. Finally, we return our new `MyBox`, created from this memory.

That takes care of allocation, but what about deallocation? For that, we use
`Drop`, along with another fake function: `free_memory`. This function will
return the memory back to the operating system.

So we're automatically allocating memory in `new`, and then automatically
freeing it in `drop`. We can't mess it up. The ownership system makes sure that
we call `drop` at the right time, and things work out.

Now that you understand the basics of smart pointers, let's talk about a few
other useful ones in the standard library.

## `Rc<T>`

In the majority of cases, ownership is very clear: you know exactly which
binding owns a given bit of data. However, this isn't always the case;
sometimes, you may actually need multiple owners. For this, Rust has a type
called `Rc<T>`. Its name is an abbreviation for 'reference counting'.

Let's look at an example:

```rust
use std::rc::Rc;

let r1 = Rc::new(5);
let r2 = r1.clone();
```

You've seen the `clone` method previously, it's usually used for making a
complete copy of some data. With `Rc<T>`, though, it doesn't make a full copy.
`Rc<T>` holds a 'reference count', that is, a count of how many clones exist.
Let's look at this example in more detail:

```rust
use std::rc::Rc;

let r1 = Rc::new(5); // here, we have an Rc<i32> with a reference count of one.

let r2 = r1.clone(); // here, we increment the reference count; now both r1 and
                     // r2 both refer to the same 5; and we have a total count
                     // of two.

// Here, the scope is ending. r2 goes out of scope first. When it does, it
// doesn't free the 5, like a Box<i32> would; it decrements the count. So after
// r2 goes out of scope, the count is one.

// Now, r1 goes out of scope. When it does, it decrements the count. The count
// is now zero. Since the count is zero, the value is deallocated.
```

This is the basic strategy: you make an `Rc<T>` with `new`, and then call
`clone` for all of the other owners you need. When each `Rc<T>` goes out of
scope, they decrease the count, and when the count is zero, the value is
deallocated. This strategy lets us have multiple owners, as the count will
ensure that the value remains valid as long as any of the owners still exists.

This idea is simple enough, but there are a few twists! Here's the first: In
order for this to be okay, the data inside of an `Rc<T>` must be immutable.  If
it were mutable, we'd run into a similar problem with borrowing: two mutable
borrows to the same place can cause a lot of problems. There's another twist,
too: an "`Rc<T>` cycle." While single ownership is easy to reason about,
multiple owenership is a lot trickier. Before we cover these situations in
detail, we need to talk about another type: `RefCell<T>`.

## `RefCell<T>`

Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the
data that it holds. So, what makes `RefCell<T>` different than a type like
`Box<T>`? For that, we'll have to recall the borrowing rules we learned in
Chapter 4:

1. At any given time, you can have either but not both of:
    * One mutable reference.
    * Any number of immutable references.
2. References must always be valid.

With references, these invariants are enforced at compile time. But with
`RefCell<T>`, these invariants are enforced *at runtime*. With references,
if you break these rules, you'll get a compiler error. With `RefCell<T>`,
if you break these rules, you'll get a `panic!`.

Before we talk about how to use `RefCell<T>`, we should mention one more thing:
`RefCell<T>` is only useful in single-threaded scenarios; it's not threadsafe.
We'll talk about concurrency and paralellism in the next chapter; for now, all
you need to know is that if you try to use `RefCell<T>` in a single-threaded
context, you'll get a compile time error.

### The `borrow` and `borrow_mut` methods

With references, we use the `&` and `&mut` syntax to create references and
mutable references, respectively. But with `RefCell<T>`, we use the `borrow`
and `borrow_mut` methods:

```rust
use std::cell::RefCell;

let five = RefCell::new(5);

// we need these scopes so we don't break the rules!
{
    let r1 = five.borrow();
    let r2 = five.borrow();
    let r3 = five.borrow();

    // r1, r2, and r3 are all immutable references.
}

{
    let r = five.borrow_mut();

    // r1 is a mutable reference
}
```

If we call both `borrow` and `borrow_mut` in the same scope, we'll get a panic.

So why do we need `RefCell<T>`? What good is it to enforce the rules at runtime,
instead of compile time?

Static analysis, like Rust performs, is inherently conservative. That is, if we
accept an incorrect program, very bad things happen, but if we reject a correct
program, the programmer will be inconvenienced, but nothing catastrophic can
occur. `RefCell<T>` is useful in two situations:

1. When you know that the borrowing rules are respected, but when the compiler
can't understand that that's true.
2. When you need "interior mutability."

We'll cover "interior mutability" later in this chapter, but first, let's talk
about a different kind of cell: `Cell<T>`.

## `Cell<T>`

`Cell<T>` is the same as `RefCell<T>`, but only for types that are `Copy`. Instead
of `borrow` and `borrow_mut` methods, it has `get` and `set` methods:

```rust
use std::cell::Cell;

let mut five = Cell::new(5);

five.set(6); // not five any more O_O

println!("five is: {}", five.get());
```

The difference between `Cell<T>` and `RefCell<T>` mirrors the difference
between types that are `Copy` and types that are not; if we don't need the
ownership rules, then we can do things in a much simpler way. If we do, then we
need to deal with borrowing, rather than just copying data around.

So when should you use `Cell<T>`? Since we don't have the borrowing element,
the advice is the same as for `RefCell<T>`, but only the second rule: use
`Cell<T>` when you need interior mutability.

But what is 'interior mutability' anyway?

## The Interior Mutability Pattern

The Interior Mutability Pattern is super unsafe internally but safe to use
from the outside and is totally safe, totally, trust us, seriously, it's safe.

Allude to `UnsafeCell<T>` maybe. Affects optimizations since &mut T is unique.
UnsafeCell turns off those optimizations so that everything doesn't break.

This is how you can opt-out of the default of Rust's ownership rules and opt
in to different guarantees.

### Cycles

We've shown that with `Rc<T>`s, when the last one goes out of scope, the value
is deallocated. But what about this program?

```rust
use std::rc::Rc;

struct Cycle {
    really_bad: Option<Rc<Cycle>>,
    leaked_data: i32,
}

fn main() {
    let mut oh_no = Rc::new(Cycle {
        really_bad: None,
        leaked_data: 5,
    });

    let clone = oh_no.clone();        

    oh_no.really_bad = Some(clone);
}
```

#### Solution: turn an Rc into a `Weak<T>`

Same as Rc, but doesn't count towards the strong ref count. When you do this, the
strong ref count goes down and the weak count goes up.

Data gets cleaned up when the strong count is 0, no matter what the weak count is.

Why is the weak count needed then????


## Summary

If you want to implement your own smart pointer, go read the Nomicon.

Now let's talk about concurrency, and some smart pointers that can be used
with multiple threads.
