# Unsafe Rust

So far, we've been talking about code written in Rust. That's what you'd expect
from a book on Rust! However, Rust has a second language hiding out inside of
it: unsafe Rust. Unsafe Rust works just like regular Rust does, but it gives
you extra superpowers not available in safe Rust code.

You may be wondering why this is. While Rust's safety guarantees are a
wonderful thing, by nature, static analysis is conservative. That is, when
trying to determine if something is okay or not, it's better to reject some
programs that are valid than it is to accept some programs that are invalid.
There are some times when your code might be okay, but Rust thinks it's not! In
these cases, you can use unsafe code to tell the compiler, "trust me, I know
what I'm doing." The downside is that you're on your own; if you get it wrong,
bad things can happen.

There's another reason that Rust needs to have unsafe code: the underyling
hardware of computers is not safe. If Rust didn't let you do unsafe things,
then there would be some things that you simply could not do. But Rust needs to
be able to let you do things like directly interact with your operating system,
or even write your own operating system! That's part of the goals of the
language. So we need some way to do these kinds of things.

## Unsafe Superpowers

More specifically, there are four things that you can do with unsafe Rust that
you cannot do in safe Rust. We call these the "unsafe superpowers." Here they
are:

1. Dereference a raw pointer.
2. Call an unsafe function.
3. Access or modify a static variable.
4. Implement an unsafe trait.

We haven't seen most of these features yet because, well, they're only usable
by unsafe! That is, it's important to understand that unsafe doesn't "turn off
the borrow checker" or disable any of Rust's safety checks: if you use a
reference in unsafe code, it will still be checked. What it does do is give you
access to these new, unchecked features. You still get some degree of safety
inside of an unsafe block!

Rust's strategy here is to make sure everything is safe, but allow you to do
extra unsafe things when you specifically annotate your code to allow unsafe
things. What kind of annotations? It looks like this:

```rust
// only safe stuff here!
let x = 5;

unsafe {
    // here be dragons!
}
```

You can only use these features inside of these blocks. This means that you do
make a mistake and something goes wrong, you'll know that it has to be related
to one of the places that you opted into this unsafety. That makes these bugs
much easier to find. Because of this, it's important to contain your unsafe
code to as small of an area as possible. Once you use unsafe inside of a
module, any of the code in that module is supect. Keep them small and you'll
thank yourself later.

One final note about unsafe blocks: while unsafe blocks let you do almost
anything, there are still rules. That is, `unsafe` does not mean "now I will do
anything," `unsafe` means "I have manually checked that I am following the
rules." If you break the rules, bad things can still happen!

Let's talk about each of these four superpowers in turn.

## Raw Pointers

Way back in chapter four, we learned about references:

```rust
let r = &5;
```

We also learned that references are always valid, and that the compiler makes
sure that this is so. Unsafe Rust has two new types that are similar to
references called "raw pointers."

```rust
let mut num = 5;

let r1 = &5 as *const i32;
let r2 = &mut 5 as *mut i32;
```

The `*const T` and `*mut T` types are raw pointers, in contrast with references
and mutable references, respectively. Unlike references, these pointers may or
may not be valid. We can even create raw pointers to arbitrary locations in
memory:

```rust
// don't try this at home:
let address = 0x012345;
let r = address as *const i32;

// bad things will happen if you try to use r
```

But wait, we said that you need to use `unsafe` with raw pointers, but there's
no `unsafe` block in the above examples. What gives? While you can _create_
raw pointers in safe code, you can't _dereference_ raw pointers in safe code.
To use `*`, you need `unsafe`:

```rust
let mut num = 5;

let r1 = &5 as *const i32;
let r2 = &mut 5 as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

This is because creating a pointer can't do any harm; it's only when accessing
the value that it points at that you might end up dealing with something that's
invalid.

Furthermore, in these examples, you may have noticed something: we created both
a `*const i32` and a `*mut i32` to the same memory location. With references,
this would be impossible, due to the mutability rules. With raw pointers, you
can do this. Be careful!

With all of these dangers, why would we ever use raw pointers? One major
use-case is interfacing with C code; we'll talk about this more in the next
section.  Another case is to build up safe abstractions that the borrow checker
doesn't understand. Before we show an example, let's talk about unsafe
functions; you'll often be using them with raw pointers.

## Unsafe Functions

The second thing that requires an unsafe block is a call to an unsafe function.
Unsafe functions look exactly like regular functions, but with an extra
`unsafe` out front:

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

If you try to call `dangerous` without the `unsafe` block, you'll get an error:

```text
error[E0133]: call to unsafe function requires unsafe function or block
 --> <anon>:4:5
  |
4 |     dangerous();
  |     ^^^^^^^^^^^ call to unsafe function
```

By inserting the `unsafe` block, you're asserting to Rust that you've read the
documentation for this function, you understand how to use it properly, and
you've verified that everything is correct.

Raw pointers and unsafe functions often interact, becuase unsafe functions
often take raw pointers as arguments. Given that raw pointers aren't checked, a
very common constraint on unsafe functions is "make sure the raw pointers
you're passing to it are valid."

As an example, let's check out some functionality from the standard library,
`split_at_mut`. This method is defined on mutable slices, and it takes one
slice and makes it into two, like this:

```rust
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

This function couldn't be written in safe Rust. If we tried, it might look like
this:

```rust,ignore
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // get the total length of the slice
    let len = slice.len();

    // make sure that our midpoint is in bounds
    assert!(mid <= len);

    // return two slices, from the start to mid, and from mid to the end
    (&mut slice[..mid],
     &mut slice[(len - mid)..])
}
```

If you try to compile this, you'll get an error:

```text
error[E0499]: cannot borrow `*slice` as mutable more than once at a time
 --> <anon>:6:11
  |
5 |     (&mut slice[..mid],
  |           ----- first mutable borrow occurs here
6 |      &mut slice[(len - mid)..])
  |           ^^^^^ second mutable borrow occurs here
7 | }
  | - first borrow ends here
```

Rust's borrow checker can't understand that we're borrowing different parts of
the slice; it only knows that we're borrowing from the same slice twice. Doing
this is fundamentally okay; our two `&mut [i32]`s aren't overlapping. But Rust
isn't smart enough to know this. When you know something is okay, but Rust
doesn't, it's time to reach for unsafe code.

Here's how to use `unsafe` to make this work:


```rust,ignore
use std::slice;

// in the standard library, this is generic over any T, but we'll use i32 here.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    unsafe {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
```

Remember how slices are a pointer to some data, and then the length of the
slice? You can get these bits with the `len` and `as_mut_ptr` methods.
`as_mut_ptr` returns a raw pointer, an `*mut i32` in this case. Then,
the `slice::from_raw_pts_mut` method does the reverse: it takes a raw pointer
and a length, and then conjures up a slice. Because slices are checked, they're
safe, but since `from_raw_parts_mut` takes a raw pointer, it just trusts that
this pointer is valid. For example, this code would _not_ work:

```rust
use std::slice;

let address = 0x012345;
let r = address as *mut i32;

let slice = unsafe {
    // noooooooooooo
    slice::from_raw_parts_mut(r, 10000)
};
```

Now you have a ten thousand long slice to a random place in memory. This won't
work. Don't try this at home.

But above, since we got our raw pointer from an existing slice, we know this is
safe! So it's fine. We also have a second `unsafe` function hidden in there:
`offset`. The `offset` method on raw pointers takes a number, and then
increments the pointer in memory. We use this function to create the second
slice.

That's the general idea of unsafe functions, but let's talk about two other
specific cases.

### `transmute`

The `transmute` function is an unsafe function, but it should really be known
as the most unsafe function, so unsafe that you shouldn't ever use it. What
does it do? It says "hey, compiler, you know this type? Treat the data as this
other type. Don't think about it, just trust me." So for example,

```rust
let ptr = &0;

let other_ptr: usize = unsafe { std::mem::transmute(ptr) };
```

Here, we say "hey Rust! You know how you have a reference? Convert it into a
`usize`. Since a `usize` has the same number of bits as a reference, this works
just fine.

However, there's almost always a better alternative to transmute. For example,
in this case, we could use `as` to first cast our reference to a raw pointer,
and then use it again to cast as a `usize`:

```rust
let ptr = &0;

let other_ptr = ptr as *const i32 as usize;
```

This is much safer.

For more details, see the documentation for `transmute` in the standard
library.

Or don't, because you shouldn't use `transmute`. Unless you absolutely,
absolutely, absolutely must.

### `extern fn`

Sometimes, your Rust code may need to interact with code written in another
language. To do this, Rust has a keyword, `extern`, that facilitates this:

```rust,ignore
// This function is defined somewhere externally:
extern "C" {
    fn some_function();
}

// This function can be exposed externally:
pub extern "C" fn call_from_c() {
    // code goes here
}

fn main() {
    unsafe { some_function() };
}
```

As you can see, `extern` can be used in two ways: to refer to a function
defined somewhere else, and to expose a Rust function to be used externally.
The block form is used for the former case, and putting it before the `fn` is
used for the latter case.

If you're calling an external function, you need to use `unsafe`. The reason is
this: if you're calling into some other language, that language is not Rust,
and so does not follow Rust's safety guarantees. Since Rust can't check that
it's safe, you must.

You'll also notice the `"C"` there; this defines which ABI, or "application
binary interface", your external function is. The ABI defines how to call the
function at the assembly level. The `"C"` ABI is the most common, and follows
the C programming language's ABI.

## `static`

We've gone this entire book without talking about "global variables." Many
programming languages support them, and so does Rust. However, global variables
can be problematic: if you have two threads, for example, accessing the same
mutable global variable, bad things can happen.

We call global variables "static" in Rust, and they look like this:

```rust
static HELLO_WORLD: &'static str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

You'll notice two things about `static`s: their names are in
`SCREAMING_SNAKE_CASE` by convention, and you _must_ declare the type, which is
`&'static str` in this case. Any references stored in a static will have the
`'static` lifetime.

You can also have mutable statics, but those require `unsafe`:

```rust
static mut COUNTER: u32 = 0;

fn main() {
    // mutation is unsafe...
    unsafe {
        COUNTER = COUNTER + 1;
    }

    // ... but so is access
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

Global mutable state is tricky!

## Unsafe Traits

Finally, the last feature of `unsafe` is related to traits. We can declare a
trait as `unsafe`:

```rust
unsafe trait Foo {
    // methods go here
}
```

And then they require the `unsafe` keyword to implement:

```rust
# unsafe trait Foo {
#     // methods go here
# }

unsafe impl Foo for i32 {
    // methods go here
}
```

Like general unsafe functions, an unsafe trait says "hey, there is some sort of
invariant here that the compiler cannot verify. By using `unsafe impl`, you are
promising that you uphold these invariants."

As an example, remember the `Sync` and `Send` traits from Chapter 16? These
marker traits have no methods, and there's no way for the compiler to verify
that, if you try to implement these traits, that they actually have the `Sync`
and `Send` properties. As such, they're `unsafe` traits, and so you need
`unsafe` to implement them.

## Summary

That's the gist of unsafe! If you want an even more thorough coverage of unsafe
code, check out the Nomicon.

Let's move on. Time to talk more about lifetimes!
