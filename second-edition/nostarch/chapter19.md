
[TOC]

<!-- This is a long chapter! I was trying to consider whether to split it, and
if so where --- the only solution I could come up with was to split it into the
five main subjects: Unsafe, Lifetimes, Traits, Types, and Functions and
Closures. However, I'm not convinced that's ideal, so I thought we might
include a ToC at the top of this chapter in print so the reader can use it as a
reference when they come across something they can't figure out. What do you
think? -->

# Advanced Features

We’ve come a long way! By now, you’ve learned 99% of the things you’ll need to
know when writing Rust. Before we do one more project in Chapter 20, let’s talk
about a few things you may run into that last 1% of the time. Feel free to skip
this chapter and come back to it only when you run into something unknown in
the wild; the features you’ll learn to use here are useful in very specific
situations. We don’t want to leave these features out, but you won’t find
yourself reaching for them often.

In this chapter, we’re going to cover:

* Unsafe Rust: for when you need to opt out of some of Rust’s guarantees and
  make yourself responsible for upholding the guarantees
  instead
* Advanced Lifetimes: syntax for complex lifetime situations
* Advanced Traits: Associated Types, default type parameters, fully qualified
  syntax, supertraits, and the newtype pattern in relation to traits
* Advanced Types: some more about the newtype pattern, type aliases, the
  “never” type, and dynamically sized types
* Advanced Functions and Closures: function pointers and returning closures

It’s a panoply of Rust features with something for everyone! Let’s dive in!

## Unsafe Rust

All the code we’ve discussed so far has had Rust's memory safety guarantees
enforced at compile time. However, Rust has a second language hiding inside of
it that does not enforce these memory safety guarantees: unsafe Rust. This
works just like regular Rust, but gives you extra superpowers.

Unsafe Rust exists because, by nature, static analysis is conservative. When
the compiler is trying to determine if code upholds the guarantees or not, it’s
better for it to reject some programs that are valid than accept some programs
that are invalid. That inevitably means there are some times when your code
might be okay, but Rust thinks it’s not! In these cases, you can use unsafe
code to tell the compiler, “trust me, I know what I’m doing.” The downside is
that you’re on your own; if you get unsafe code wrong, problems due to memory
unsafety, like null pointer dereferencing, can occur.

There’s another reason Rust has an unsafe alter ego: the underlying hardware of
computers is inherently not safe. If Rust didn’t let you do unsafe operations,
there would be some tasks that you simply could not do. Rust needs to allow you
to do low-level systems programming like directly interacting with your
operating system, or even writing your own operating system! That’s one of the
goals of the language. Let's see what you can do with unsafe Rust, and how to
do it.

### Unsafe Superpowers

To switch into unsafe Rust we use the `unsafe` keyword, and then we can start a
new block that holds the unsafe code. There are four actions that you can take
in unsafe Rust that you can’t in safe Rust that we call “unsafe superpowers.”
Those superpowers are the ability to:

1. Dereference a raw pointer
2. Call an unsafe function or method
3. Access or modify a mutable static variable
4. Implement an unsafe trait

It’s important to understand that `unsafe` doesn’t turn off the borrow checker
or disable any other of Rust’s safety checks: if you use a reference in unsafe
code, it will still be checked. The `unsafe` keyword only gives you access to
these four features that are then not checked by the compiler for memory
safety. You still get some degree of safety inside of an unsafe block!

Furthermore, `unsafe` does not mean the code inside the block is necessarily
dangerous or that it will definitely have memory safety problems: the intent is
that you as the programmer will ensure the code inside an `unsafe` block will
have valid memory.

People are fallible, and mistakes will happen, but by requiring these four
unsafe operations to be inside blocks annotated with `unsafe`, you’ll know that
any errors related to memory safety must be within this unsafe block of code.
Keep `unsafe` blocks small and you’ll thank yourself later when you go to
investigate memory bugs.

To isolate unsafe code as much as possible, it’s a good idea to enclose unsafe
code within a safe abstraction and provide a safe API, which we’ll be
discussing once we get into unsafe functions and methods. Parts of the standard
library are implemented as safe abstractions over unsafe code that has been
audited. This technique prevents uses of `unsafe` from leaking out into all the
places that you or your users might want to make use of the functionality
implemented with `unsafe` code, since using a safe abstraction is safe.

Let’s talk about each of the four unsafe superpowers in turn, and along the way
we’ll look at some abstractions that provide a safe interface to unsafe code.

### Dereferencing a Raw Pointer

Way back in Chapter 4, when first learned about references, we also learned
that the compiler ensures references are always valid. Unsafe Rust has two new
types similar to references called *raw pointers*. Just like with references,
raw pointers can be immutable or mutable, written as `*const T` and `*mut T`,
respectively. In the context of raw pointers, “immutable” means that the
pointer can’t be directly assigned to after being dereferenced.

Different to references and smart pointers, raw pointers:

- Are allowed to ignore the borrowing rules and have both immutable and
  mutable pointers, or multiple mutable pointers to the same location
- Aren't guaranteed to point to valid memory
- Are allowed to be null
- Don't implement any automatic clean-up

<!-- Can you say here what benefits these provide, over smart pointers and
references, and using the aspects in these bullets? -->

Listing 19-1 shows how to create both an immutable and a mutable raw pointer
from references.

```
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

Listing 19-1: Creating raw pointers from references

<!--So we create a raw pointer using the dereference operator? Is that the same
operator? Is it worth touching on why? -->

Notice we don't include the `unsafe` keyword here---you can *create* raw
pointers in safe code, you just can’t *dereference* raw pointers with an unsafe
block, as we'll see in a bit.

We’ve created raw pointers by using `as` to cast an immutable and a mutable
reference into their corresponding raw pointer types: The `*const T` type
immutable, and `*mut T` is mutable. Because we created them directly from
references that are guaranteed to be valid, we can know that these particular
raw pointers are valid, but we can't make that assumption about just any raw
pointer.

Next we'll create a raw pointer whose validity we can't be so certain of.
Listing 19-2 shows how to create a raw pointer to an arbitrary location in
memory. Trying to use arbitrary memory is undefined: there may be data at that
address or there may not, the compiler might optimize the code so that there is
no memory access, or your program might segfault. There’s not usually a good
reason to be writing code like this, but it is possible:

```
let address = 0x012345usize;
let r = address as *const i32;
```

Listing 19-2: Creating a raw pointer to an arbitrary memory address

Remeber that we said you can create raw pointers in safe code, but you can’t
*dereference* raw pointers and read the data being pointed to. We'll do so now
using the dereference operator, `*`, on a raw pointer, which does require an
`unsafe` block, as shown in Listing 19-3:

```
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

Listing 19-3: Dereferencing raw pointers within an `unsafe` block

Creating a pointer can’t do any harm; it’s only when accessing the value that
it points at that you might end up dealing with an invalid value.

Note also that in Listing 19-1 and 19-3 we created `*const i32` and `*mut i32`
raw pointers that both pointed to the same memory location, that of `num`. If
instead we'd tried to create an immutable and a mutable reference to `num`,
this would not have compiled because Rust's ownership rules don't allow a
mutable reference at the same time as any immutable references. With raw
pointers, we are able to create a mutable pointer and an immutable pointer to
the same location, and change data through the mutable pointer, potentially
creating a data race. Be careful!

With all of these dangers, why would we ever use raw pointers? One major use
case is when interfacing with C code, as we’ll see in the next section on
unsafe functions. Another case is when building up safe abstractions that the
borrow checker doesn’t understand. Let’s introduce unsafe functions then look
at an example of a safe abstraction that uses unsafe code.

### Calling an Unsafe Function or Method

The second type of operation that requires an unsafe block is calls to unsafe
functions. Unsafe functions and methods look exactly like regular functions and
methods, but they have an extra `unsafe` out front.

<!-- Above -- so what is the difference, when and why would we ever use the
unsafe function? -->

Bodies of unsafe functions are effectively `unsafe` blocks. Here’s an unsafe
function named `dangerous`:

```
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

We call the `dangerous` function within a separate `unsafe` block. If we try to
call `dangerous` without the `unsafe` block, we’ll get an error:

```
error[E0133]: call to unsafe function requires unsafe function or block
 --> <anon>:4:5
  |
4 |     dangerous();
  |     ^^^^^^^^^^^ call to unsafe function
```

By inserting the `unsafe` block around our call to `dangerous`, we’re asserting
to Rust that we’ve read the documentation for this function, we understand how
to use it properly, and we’ve verified that everything is correct.

#### Creating a Safe Abstraction Over Unsafe Code

As an example, let’s check out a function from the standard library,
`split_at_mut`, that requires some unsafe code and explore how we might
implement it. This safe method is defined on mutable slices: it takes one slice
and makes it into two by splitting the slice at the index given as an argument.
This is demonstrated in Listing 19-4:

```
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

Listing 19-4: Using the safe `split_at_mut` function

This function can’t be implemented using only safe Rust. An attempt might look
something like Listing 19-5, and will not compile. For simplicity, we’re
implementing `split_at_mut` as a function rather than a method, and only for
slices of `i32` values rather than for a generic type `T`.

```
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid],
     &mut slice[mid..])
}
```

Listing 19-5: An attempted implementation of `split_at_mut` using only safe Rust

This function first gets the total length of the slice, then asserts that the
index given as a parameter is within the slice by checking that its less than
or equal to the length. The assertion means that if we pass an index that’s
greater than the index to split the slice at, the function will panic before it
attempts to use that index.

Then we return two mutable slices in a tuple: one from the start of the
original slice to the `mid` index, and another from `mid` to the end of the
slice.

If we try to compile this, we’ll get an error:

```
error[E0499]: cannot borrow `*slice` as mutable more than once at a time
 --> <anon>:6:11
  |
5 |     (&mut slice[..mid],
  |           ----- first mutable borrow occurs here
6 |      &mut slice[mid..])
  |           ^^^^^ second mutable borrow occurs here
7 | }
  | - first borrow ends here
```

Rust’s borrow checker can’t understand that we’re borrowing different parts of
the slice; it only knows that we’re borrowing from the same slice twice.
Borrowing different parts of a slice is fundamentally okay because our two
slices aren’t overlapping, but Rust isn’t smart enough to know this. When we
know something is okay, but Rust doesn’t, it’s time to reach for unsafe code.

Listing 19-6 shows how to use an `unsafe` block, a raw pointer, and some calls
to unsafe functions to make the implementation of `split_at_mut` work:

```
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
```

Listing 19-6: Using unsafe code in the implementation of the `split_at_mut`
function

Recall from Chapter 4 that slices are a pointer to some data and the length of
the slice. We use the `len` method to get the length of a slice, and the
`as_mut_ptr` method to access the raw pointer of a slice. In this case, since
we have a mutable slice to `i32` values, `as_mut_ptr` returns a raw pointer
with the type `*mut i32`, which we’ve stored in the variable `ptr`.

We keep the assertion that the `mid` index is within the slice. Then we get to
the unsasfe code: the `slice::from_raw_parts_mut` takes a raw pointer and a
length and creates a slice. We use this function to create a slice that starts
from `ptr` and is `mid` items long. Then we call the `offset` method on `ptr`
with `mid` as an argument to get a raw pointer that starts at `mid`, and we
create a slice using that pointer and the remaining number of items after `mid`
as the length. The function `slice::from_raw_parts_mut` is unsafe because it
takes a raw pointer and must trust that this pointer is valid. The `offset`
method on raw pointers is also unsafe, because it must trust that the offset
location is also a valid pointer. We therefore had to put an `unsafe` block
around our calls to `slice::from_raw_parts_mut` and `offset` to be allowed to
call them. We can tell, by looking at the code and by adding the assertion that
`mid` must be less than or equal to `len`, that all the raw pointers used
within the `unsafe` block will be valid pointers to data within the slice. This
is an acceptable and appropriate use of `unsafe`.

Note that the resulting `split_at_mut` function is safe and didn`t have o be in
an `unsafe` block, and we can call this function from safe Rust. We’ve created
a safe abstraction to the unsafe code with an implementation of the function
that uses `unsafe` code in a safe way because it creates only valid pointers
from the data this function has access to.

In contrast, the use of `slice::from_raw_parts_mut` in Listing 19-7 would
likely crash when the slice is used. This code takes an arbitrary memory
location and creates a slice ten thousand items long:

```
use std::slice;

let address = 0x012345usize;
let r = address as *mut i32;

let slice = unsafe {
    slice::from_raw_parts_mut(r, 10000)
};
```

Listing 19-7: Creating a slice from an arbitrary memory location

We don’t own the memory at this arbitrary location, and there’s no guarantee
that the slice this code creates contains valid `i32` values. Attempting to use
`slice` as if it was a valid slice would result in undefined behavior.

#### Using `extern`  Functions to Call External Code

Sometimes, your Rust code may need to interact with code written in another
language. For this, Rust has a keyword, `extern`, that facilitates the creation
and use of a *Foreign Function Interface* (FFI).

<!-- Can you give a definition for FFI? -->

Listing 19-8 demonstrates how to set up an integration with the `abs` function
from the C standard library. Functions declared within `extern` blocks are
always unsafe to call from Rust code, because other languages don`t enforce
Rust's rules and guarantees and cannot be checked, so responsibility falls on
the programmer to ensure safety:

Filename: src/main.rs

```
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

Listing 19-8: Declaring and calling an `extern` function defined in another
language

Within the `extern "C"` block, we list the names and signatures of external
functions from another language we want to be able to call. The `"C"` keyword
defines which *application binary interface* (ABI) the external function
uses---the ABI defines how to call the function at the assembly level. The
`"C"` ABI is the most common, and follows the C programming language’s ABI.

Calling an external function is always unsafe, because the external language
does not enforce Rust’s safety guarantees. Since Rust can’t check that the
external code is safe, we are responsible for checking its safety and
indicating we have done so by placing the call to external functions in an
`unsafe` block.

<!-- PROD: START BOX -->

##### Calling Rust Functions from Other Languages

You can also use `extern` to create an interface that allows other languages to
call Rust functions. Instead of an `extern` block, we add the `extern` keyword
and specify the ABI to use just before the `fn` keyword. We also need to add a
`#[no_mangle]` annotation to tell the Rust compiler not to mangle the name of
this function.

<!-- have we discussed mangling before this? It doesn't ring a bell with me,
though it may have been in an early chapter that I forgot --- if not could you
give a quick explanation here? -->

In this example we make the `call_from_c` function accessible from C code, once
it's compiled to a shared library and linked from C:

```
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

This usage of `extern` does not require `unsafe`

<!-- PROD: END BOX -->

### Accessing or Modifying a Mutable Static Variable

We’ve managed to go this entire book without talking about *global variables*,
which Rust does support, but which can be problematic with Rust's owmership
rules: if you have two threads accessing the same mutable global variable, it
can incur a data race.

Global variables are called *static* variables in Rust. Listing 19-9 shows an
example declaration and use of a static variable with a string slice as a value:

Filename: src/main.rs

```
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

Listing 19-9: Defining and using an immutable static variable

`static` variables are similar to constants: their names are in
`SCREAMING_SNAKE_CASE` by convention, and we *must* annotate the variable’s
type, which is `&'static str` in this case. Static variables may only store
references with the `'static` lifetime, which means the Rust compiler can
figure out the lifetime by itself and we don’t need to annotate it explicitly.

One difference between constants and static variabels is that accessing
immutable static variables is safe--values in a static variable have a fixed
address in memory, and using the value will always access the same data.
Constants, on the other hand, are allowed to duplicate their data whenever they
are used.

Another difference is that static variables can be mutable. Both accessing and
modifying mutable static variables is *unsafe*. Listing 19-10 shows how to
declare, access, and modify a mutable static variable named `COUNTER`:

Filename: src/main.rs

```
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

Listing 19-10: Reading from or writing to a mutable static variable is unsafe

Just like with regular variables, we specify mutability using the `mut`
keyword. Any code that reads or writes from `COUNTER` must be within an
`unsafe` block. This code compiles and prints `COUNTER: 3` as we would expect
since it’s single threaded. Having multiple threads access `COUNTER` would
likely result in data races.

With mutable data that's globally accessible, it's difficult to ensure there
are no data races, which is why Rust considers mutable static variables to be
unsafe. Where possible, its preferable to use the concurrency techniques and
threadsafe smart pointers we discussed in Chapter 16, so the compiler checks
that data accessed from different threads is done safely.

### Implementing an Unsafe Trait

Finally, the last action that only works with `unsafe` is implementing an
unsafe trait. A trait is unsafe when at least one of its methods has some
invariant that the compiler can't varify. We can declare that a trait is
`unsafe` by adding the `unsafe` keyword before `trait`, and then implementation
of the trait must be marked as `unsafe` too, as shown in Listing 19-11:

```
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```

Listing 19-11: Defining and implementing an unsafe trait

By using `unsafe impl`, we’re promising that we’ll uphold the invariants that
the compiler can't verify.

As an example, recall the `Sync` and `Send` marker traits from Chapter 16, and
that the compiler implements these automatically if our types are composed
entirely of `Send` and `Sync` types. If we implement a type that contains
something that’s not `Send` or `Sync`, such as raw pointers, and we want to
mark that type as `Send` or `Sync`, we must use `unsafe`. Rust can’t verify
that our type upholds the guarantees that it can be safely sent across threads
or accessed from multiple threads, so we need to do those checks ourselves and
indicate as such with `unsafe`.

### When to Use Unsafe Code

Using `unsafe` to take one of these four actions isn’t wrong or even frowned
upon, but it is trickier to get `unsafe` code correct since the compiler isn’t
able to help uphold memory safety. When you have a reason to use `unsafe` code,
it is possible to do so, and having the explicit `unsafe` annotation makes it
easier to track down the source of problems if they occur.

## Advanced Lifetimes

Back in Chapter 10, we learned how to annotate references with lifetime
parameters to tell Rust how lifetimes of different references relate. We saw
how every reference has a lifetime but, most of the time, Rust will let you
elide lifetimes. Here we'll look at three advanced features of lifetimes that
we haven’t covered yet: *lifetime subtyping*---a way to ensure that one
lifetime outlives another lifetime; *lifetime bounds*, and *trait object
lifetimes*.

<!-- maybe add a small summary of each here? That would let us launch straight
into examples in the next section -->

### Lifetime Subtyping

To explore lifetime subtyping, imagine we want to write a parser. We’ll have a
structure called `Context` that holds a reference to the string we’re parsing.
We’ll write a parser that will parse this string and return success or failure.
The parser will need to borrow the context to do the parsing. Implementing this
without lifetime annotations would look like the code in Listing 19-12, but
this won't compile:

```
struct Context(&str);

struct Parser {
    context: &Context,
}

impl Parser {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

Listing 19-12: Defining a parser without lifetime annotations

<!-- What will the compile time error be here? I think it'd be worth showing
that to the reader -->

For simplicity’s sake, our `parse` function returns a `Result<(), &str>`. That
is, it will do nothing on success, and on failure will return the part of the
string slice that didn’t parse correctly. A real implementation would have more
error information than that, and would actually return something when parsing
succeeds, but we’ll leave those off since they aren’t relevant to the lifetimes
part of this example. We’re also defining `parse` to always produce an error
after the first byte. Note that this may panic if the first byte is not on a
valid character boundary; again, we’re simplifying the example in order to
concentrate on the lifetimes involved.

<!-- why do we want to always error afer the first byte? -->

To get this cocde working, we need to fill in the lifetime parameters for the
string slice in `Context` and the reference to the `Context` in `Parser`. The
most straightforward way to do this is to use the same lifetime everywhere, as
shown in Listing 19-13:

```
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

Listing 19-13: Annotating all references in `Context` and `Parser` with the
same lifetime parameter

This compiles fine. Next, in Listing 19-14, let’s write a function that takes
an instance of `Context`, uses a `Parser` to parse that context, and returns
what `parse` returns. This won’t quite work:

<!-- can you let the reader know they should be taking away from this previous
example? I'm not totally clear on why adding lifetimes here saved the code -->

```
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

Listing 19-14: An attempt to add a `parse_context` function that takes a
`Context` and uses a `Parser`

We get two quite verbose errors when we try to compile the code with the
addition of the `parse_context` function:

```
error: borrowed value does not live long enough
  --> <anon>:16:5
   |
16 |     Parser { context: &context }.parse()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ does not live long enough
17 | }
   | - temporary value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the
body at 15:55...
  --> <anon>:15:56
   |
15 |   fn parse_context(context: Context) -> Result<(), &str> {
   |  ________________________________________________________^
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^

error: `context` does not live long enough
  --> <anon>:16:24
   |
16 |     Parser { context: &context }.parse()
   |                        ^^^^^^^ does not live long enough
17 | }
   | - borrowed value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the
body at 15:55...
  --> <anon>:15:56
   |
15 |   fn parse_context(context: Context) -> Result<(), &str> {
   |  ________________________________________________________^
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^
```

These errors are saying that both the `Parser` instance that's created and the
`context` parameter live only from when the `Parser` is created until the end
of the `parse_context` function, but they both need to live for the entire
lifetime of the function.

In other words, `Parser` and `context` need to *outlive* the entire function
and be valid before the function starts as well as after it ends in order for
all the references in this code to always be valid. Both the `Parser` we’re
creating and the `context` parameter go out of scope at the end of the
function, though (since `parse_context` takes ownership of `context`).

<!-- Oh interesting, why do they need to outlive the fucntion, simply to
absolutely ensure they will live for as long as the function? -->

Let’s look at the definitions in Listing 19-13 again, especially the signature
of the `parse` method:

```
    fn parse(&self) -> Result<(), &str> {
```

<!--What exactly is it the reader should be looking at in this signature? -->

Remember the elision rules? If we annotate the lifetimes of the references
rather than eliding, the signature would be:

```
    fn parse<'a>(&'a self) -> Result<(), &'a str> {
```

<!--'-->

That is, the error part of the return value of `parse` has a lifetime that is
tied to the lifetime of the `Parser` instance (that of `&self` in the `parse`
method signature). That makes sense: the returned string slice references the
string slice in the `Context` instance held by the `Parser`, and the definition
of the `Parser` struct specifies that the lifetime of the reference to
`Context` and the lifetime of the string slice that `Context` holds should be
the same.

The problem is that the `parse_context` function returns the value returned
from `parse`, so the lifetime of the return value of `parse_context` is tied to
the lifetime of the `Parser` as well. But the `Parser` instance created in the
`parse_context` function won’t live past the end of the function (it’s
temporary), and `context` will go out of scope at the end of the function
(`parse_context` takes ownership of it).

Rust thinks we're trying to return a reference to a value that goes out of
scope at the end of the function, because we annotated all the lifetimes with
the same lifetime parameter. That told Rust the lifetime of the string slice
that `Context` holds is the same as that of the lifetime of the reference to
`Context` that `Parser` holds.

The `parse_context` function can’t see that within the `parse` function, the
string slice returned will outlive both `Context` and `Parser`, and that the
reference `parse_context` returns refers to the string slice, not to `Context`
or `Parser`.

By knowing what the implementation of `parse` does, we know that the only
reason the return value of `parse` is tied to the `Parser` is because it’s
referencing the `Parser`’s `Context`, which is referencing the string slice, so
it’s really the lifetime of the string slice that `parse_context` needs to care
about. We need a way to tell Rust that the string slice in `Context` and the
reference to the `Context` in `Parser` have different lifetimes and that the
return value of `parse_context` is tied to the lifetime of the string slice in
`Context`.

First we'll giving only `Parser` and `Context` different lifetime parameters as
shown in Listing 19-15. Well use `'s` and `'c` as lifetime parameter names to
be clear about which lifetime goes with the string slice in `Context` and which
goes with the reference to `Context` in `Parser`. Note that this won’t
completely fix the problem, but it’s a start and we’ll look at why this isn’t
sufficient when we try to compile.

```
struct Context<'s>(&'s str);

struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

Listing 19-15: Specifying different lifetime parameters for the references to
the string slice and to `Context`

We’ve annotated the lifetimes of the references in all the same places that we
annotated them in Listing 19-13, but used different parameters depending on
whether the reference goes with the string slice or with `Context`. We’ve also
added an annotation to the string slice part of the return value of `parse` to
indicate that it goes with the lifetime of the string slice in `Context`.

The following is the error we get now when we try to compile:

```
error[E0491]: in type `&'c Context<'s>`, reference has a longer lifetime than the data it references
 --> src/main.rs:4:5
  |
4 |     context: &'c Context<'s>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime 'c as defined on the struct at 3:0
 --> src/main.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
note: but the referenced data is only valid for the lifetime 's as defined on the struct at 3:0
 --> src/main.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
```

<!-- ' -->

Rust doesn’t know of any relationship between `'c` and `'s`. In order to be
valid, the referenced data in `Context` with lifetime `'s` needs to be
constrained, to guarantee that it lives longer than the reference with lifetime
`'c`. If `'s` is not longer than `'c`, the reference to `Context` might not be
valid.

Which gets us to the point of this section: the Rust feature *lifetime
subtyping* is a way to specify that one lifetime parameter lives at least as
long as another one. In the angle brackets where we declare lifetime
parameters, we can declare a lifetime `'a` as usual, and declare a lifetime
`'b` that lives at least as long as `'a` by declaring `'b` with the syntax `'b:
'a`.

In our definition of `Parser`, in order to say that `'s` (the lifetime of the
string slice) is guaranteed to live at least as long as `'c` (the lifetime of
the reference to `Context`), we change the lifetime declarations to look like
this:

```
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}
```

Now, the reference to `Context` in the `Parser` and the reference to the string
slice in the `Context` have different lifetimes, and we’ve ensured that the
lifetime of the string slice is longer than the reference to the `Context`.

That was a very long-winded example, but as we mentioned at the start of this
chapter, these features are pretty niche. You won’t often need this syntax, but
it can come up in situations like this one, where you need to refer to
something you have a reference to.

### Lifetime Bounds

In Chapter 10, we discussed using trait bounds on generic types. We can also
add lifetime parameters as constraints on generic types, and these are called
*lifetime bounds*.

<!-- Can you say up front why/when we use these? -->

For an example, consider a type that is a wrapper over references. Recall the
`RefCell<T>` type from Chapter 15: its `borrow` and `borrow_mut` methods return
the types `Ref` and `RefMut`, respectively. These types are wrappers over
references that keep track of the borrowing rules at runtime. The definition of
the `Ref` struct is shown in Listing 19-16, without lifetime bounds for now:

```
struct Ref<'a, T>(&'a T);
```

Listing 19-16: Defining a struct to wrap a reference to a generic type; without
lifetime bounds to start

Without explicitly constraining the lifetime `'a` in relation to the generic
parameter `T`, Rust wll error because it doesn’t know how long the generic type
`T` will live:

```
error[E0309]: the parameter type `T` may not live long enough
 --> <anon>:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
  |
  = help: consider adding an explicit lifetime bound `T: 'a`...
note: ...so that the reference type `&'a T` does not outlive the data it points at
 --> <anon>:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
```

Since `T` can be any type, `T` could itself be a reference or a type that holds
one or more references, each of which could have their own lifetimes. Rust
can’t be sure `T` will live as long as `'a`. 

<!--'-->

Fortunately, that error gave us helpful advice on how to specify the lifetime
bound in this case:

```
consider adding an explicit lifetime bound `T: 'a` so that the reference type
`&'a T` does not outlive the data it points at.
```

Listing 19-17 shows how to apply this advice by specifying the lifetime bound
when we declare the generic type `T`.

```
struct Ref<'a, T: 'a>(&'a T);
```

Listing 19-17: Adding lifetime bounds on `T` to specify that any references in
`T` live at least as long as `'a`

This code now compiles because the `T: 'a` syntax specifies that `T` can be any
type, but if it contains any references, the references must live at least as
long as `'a`.

We could solve this in a different way, shown in the definition of a
`StaticRef` struct in Listing 19-18, by adding the `'static` lifetime bound on
`T`. This means if `T` contains any references, they must have the `'static`
lifetime:

```
struct StaticRef<T: 'static>(&'static T);
```

Listing 19-18: Adding a `'static` lifetime bound to `T` to constrain `T` to
types that have only `'static` references or no references

This esnures that types without any references count as `T: 'static`. Because
`'static` means the reference must live as long as the entire program, a type
that contains no references meets the criteria of having all references live as
long as the program. For the borrow checker concerned about references living
long enough, there’s no real distinction between a type that has no references
and a type that has references that live forever; both of them are the same for
the purpose of determining whether or not a reference has a shorter lifetime
than what it refers to.

### Trait Object Lifetimes

In Chapter 17, we learned about trait objects, consisting of a trait behind a
reference, that allow us to use dynamic dispatch. We haven't yet discussed what
happens if the type implementing the trait in the trait object has a lifetime
of its own. Consider Listing 19-19, where we have a trait `Foo` and a struct
`Bar` that holds a reference (and thus has a lifetime parameter) that
implements trait `Foo`. We want to use an instance of `Bar` as the trait object
`Box<Foo>`:

```
trait Foo { }

struct Bar<'a> {
    x: &'a i32,
}

impl<'a> Foo for Bar<'a> { }

let num = 5;

let obj = Box::new(Bar { x: &num }) as Box<Foo>;
```

Listing 19-19: Using a type that has a lifetime parameter with a trait object

This code compiles without any errors, even though we haven’t said anything
explicit about the lifetimes involved in `obj`. This works because there are
rules having to do with lifetimes and trait objects:

* The default lifetime of a trait object is `'static`.
* For `&'a X` or `&'a mut X`, the default is lifetime is `'a`.
* For a single `T: 'a` clause, the default lifetime is `'a`.
* For multiple `T: 'a`-like clauses, there is no default; we must
  be explicit.

When we must be explicit, we can add a lifetime bound on a trait object like
`Box<Foo>` with the syntax `Box<Foo + 'a>` or `Box<Foo + 'static>`, depending
on what’s needed. Just as with the other bounds, this means that any
implementer of the `Foo` trait that has references inside must have the
lifetime specified in the trait object bounds as those references.

Next, let’s take a look at some other advanced features dealing with traits!

## Advanced Traits

We covered traits in Chapter 10 but, like lifetimes, we didn’t get to all the
details. Now that you know more Rust, we can get into the nitty-gritty.

### Associated Types

In Rust we can use *associated types* to associate a type placeholder with a
trait, allowing the trait method definitions to use these placeholder types in
their signatures. The implementer of a trait will specify the concrete type to
be used in this type’s place for the particular implementation.

<!-- Can you say what this is useful for -- it seems like a way to not to have
to specify a type prior to use, is that right? -->

We’ve described most of the things in this chapter as being needed very rarely.
Associated types are somewhere in the middle; they’re used more rarely than the
rest of the book, but more commonly than many of the things in this chapter.

One example of a trait with an associated type is the `Iterator` trait provided
by the standard library. This has an associated type named `Item` that stands
in for the type of the values it's iterating over. We mentioned in Chapter 13
that the definition of the `Iterator` trait is as shown in Listing 19-20:

```
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Listing 19-20: The definition of the `Iterator` trait that has an associated
type `Item`

The `Iterator` trait has an associated type named `Item`. This is a placeholder
type. The return value of the `next` method will return values of type
`Option<Self::Item>`. Implementers of this trait will specify the concrete type
for `Item`, and the `next` method will return an `Option` containing a value of
that concrete type.

#### Associated Types Versus Generics

This may seem like a similar concept to generics, in that it allows us to
define a function without specifying what types it can deal with. So why use
associated types?

Let's examine the difference with an example that implements the `Iterator`
trait on the `Counter` struct. In Listing 13-6, we specified that the `Item`
type was `u32`:

```
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
```

This feels similar to generics. So why not just define the `Iterator` trait
with generics as shown in Listing 19-21?

```
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

Listing 19-21: A hypothetical definition of the `Iterator` trait using generics

The difference lies in the fact that when using generics like in Listing 19-21,
we have to annotate the types in each implementation. This is because we can
also implement `Iterator<String> for Counter`, or any other type, which would
give us multiple implementations of `Iterator` for `Counter`. In other words,
when a trait has a generic parameter, it can be implemented for a type multiple
times, changing the concrete types of the generic type parameters each time.
When we use the `next` method on `Counter`, we’d then have to provide type
annotations to indicate which implementation of `Iterator` we wanted to use.

With associated types, we don't need to annotate types because we can’t
implement a trait on a type multiple times. With Listing 19-20, we can only
choose once what the type of `Item` will be, since there can only be one `impl
Iterator for Counter`. We don’t have to specify that we want an iterator of
`u32` values everywhere that we call `next` on `Counter`.

The benefit of not having to specify generic type parameters shows up in
another way as well. Consider the two traits defined in Listing 19-22, both of
which have to do with a graph structure that contains nodes of some type and
edges of some type. `GGraph` is defined using generics, and `AGraph` is defined
using associated types:

```
trait GGraph<Node, Edge> {
    // methods would go here
}

trait AGraph {
    type Node;
    type Edge;

    // methods would go here
}
```

Listing 19-22: Two graph trait definitions: `GGraph` using generics and
`AGraph` using associated types for `Node` and `Edge`

Let’s say we want to make a `distance` function that computes the distance
between two nodes in any types that implement the graph trait. With the
`GGraph` trait defined using generics, our `distance` function signature would
have to look like Listing 19-23:

```
fn distance<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
    // ...snip...
}
```

Listing 19-23: The signature of a `distance` function that uses the trait
`GGraph` and has to specify all the generic parameters

Our function would need to specify the generic type parameters `N`, `E`, and
`G`. Of these parameters, `G` is bound by the trait `GGraph` that has type `N`
as its `Node` type and type `E` as its `Edge` type. The `distance` function
doesn’t need to know the types of the edges, but we need to to use the `GGraph`
trait which requires specifying the type for `Edge`, so we need to declare an
`E` parameter.

Contrast with the definition of `distance` in Listing 19-24 that uses the
`AGraph` trait from Listing 19-22 with associated types:

```
fn distance<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 {
    // ...snip...
}
```

Listing 19-24: The signature of a `distance` function that uses the trait
`AGraph` and the associated type `Node`

This is much cleaner. We only need to have one generic type parameter, `G`,
with the trait bound `AGraph`. Since `distance` doesn’t use the `Edge` type at
all, it doesn’t need to be specified anywhere. To use the `Node` type
associated with `AGraph`, we can specify `G::Node`.

#### Trait Objects with Associated Types

You may wonder why we didn’t just use a trait object in the `distance`
functions. If we do this for Listing 19-3 that uses the `GGraph`, the signature
does become a bit more concise, as shown:

```
fn distance<N, E>(graph: &GGraph<N, E>, start: &N, end: &N) -> u32 {
    // ...snip...
}
```

This might be a more fair comparison to Listing 19-24. We still have to specify
the `Edge` type, though, which means Listing 19-24 is preferable since we don’t
have to specify something we don’t use.

However, it’s not possible to change Listing 19-24 to use a trait object for
the graph, because there would be no way to refer to the `AGraph` trait’s
associated type.

In general, it is possible to use trait objects of traits that have associated
types, though; Listing 19-25 shows a function named `traverse` that doesn’t
need to use the trait’s associated types in other arguments. We do, however,
have to specify the concrete types for the associated types in this case. Here,
we’ve chosen to accept types that implement the `AGraph` trait with the
concrete type of `usize` as their `Node` type and a tuple of two `usize` values
for their `Edge` type:

```
fn traverse(graph: &AGraph<Node=usize, Edge=(usize, usize)>) {
    // ...snip...
}
```

While using trait objects means we don’t need to know the concrete type of the
`graph` parameter at compile time, we do need to constrain the use of the
`AGraph` trait in the `traverse` function by the concrete types of the
associated types. If we didn’t provide this constraint, Rust wouldn’t be able
to figure out which `impl` to match this trait object to.

### Operator Overloading and Default Type Parameters

Another use for the `<PlaceholderType=ConcreteType>` syntax is in specifying
the default type for a generic type. A great example of a situation where this
is useful is with operator overloading.

<!-- Are we safe in assuming the reader is familiar with operator overloading
and why/when to use it, or is it worth giving a quick definition here? -->

Rust does not allow you to create your own operators or overload arbitrary
operators, but you *can* overload the operations and corresponding traits
listed in `std::ops` by implementing the traits associated with the operator.
For example, in Listing 19-25 we overload the `+` operator to add two `Point`
instances together. We do this by simply implementing the `Add` trait on a
`Point` struct:

Filename: src/main.rs

```
use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```

Listing 19-25: Implementing the `Add` trait to overload the `+` operator for
`Point` instances

The `add` method adds the `x` values of two `Point` instances together and the
`y` values of two `Point` instances together to create a new `Point`. The `Add`
trait has an associated type named `Output` that determines the type returned
from the `add` method.

The default generic type here is within the `Add` trait. Here’s its definition:

```
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

This should look generally familiar, as a trait with one method and an
associated type. The new part here is the `RHS=Self` in the angle brackets:
this syntax is called *default type parameters*. The `RHS` section is a generic
type parameter---short for “right hand side”---that’s used to define the type
of the `rhs` parameter in the `add` method. If we don’t specify a concrete type
for `RHS` when we implement the `Add` trait, the type of `RHS` will default to
`Self`, which will be the type we’re implementing `Add` on.

<!-- Can you say what we're looking out for in this next trait -->

Let’s look at another example of implementing the `Add` trait that utilizes the
default value. Imagine we have two structs holding values in different units,
`Millimeters` and `Meters`. We want to be able to add the different units
`Millimeters` to `Meters`, as well as to other `Millimeters` We can implement
`Add` for `Millimeters` in different ways as shown in Listing 19-26:

```
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

Listing 19-26: Implementing the `Add` trait on `Millimeters` to be able to add
`Millimeters` to `Millimeters` and `Millimeters` to `Meters`

If we’re adding `Millimeters` to other `Millimeters`, we can use the default
`Self` type and so don’t need to parameterize the `RHS` type. To be able to add
`Millimeters` and `Meters`, we specify `impl Add<Meters>` to set the value of
the `RHS` type parameter.

Default type parameters are used in two main ways:

1. To extend a type without breaking existing code.
2. To allow customization in specific cases most users won't need.

<!-- Above, in 2., do you mean customization used in corner cases? -->

Our `Add` trait is an example of the second purpose: most of the time, you’re
adding two like types together, but it gives the ability for customizing beyond
that. Using a default type parameter in the `Add` trait definition means you
don’t have to specify the extra parameter most of the time. In other words,
we’ve removed a little bit of implementation boilerplate, making it easier to
use the trait.

The first purpose is similar, but in reverse: if we want to add a type
parameter to an existing trait, we can give it a default to let us extend the
functionality of the trait without breaking the existing implementation code.

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Nothing in Rust prevents a trait from having a method with the same name as
another trait’s method, nor can it prevent us from implementing both of these
traits on one type. It's also possible to have a method implemented directly on
the type with the same name as well!

<!-- Same name as the type, you mean? -->

When calling methods with the same name, then, we need to tell Rust which one
we want to use. Consider the code in Listing 19-27 where traits `Foo` and `Bar`
both have method `f` and we implement both traits on struct `Baz`, which *also*
has a method named `f`:

Filename: src/main.rs

```
trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) { println!("Baz’s impl of Foo"); }
}

impl Bar for Baz {
    fn f(&self) { println!("Baz’s impl of Bar"); }
}

impl Baz {
    fn f(&self) { println!("Baz's impl"); }
}

fn main() {
    let b = Baz;
    b.f();
}
```

Listing 19-27: Implementing two traits that both have a method with the same
name as a method defined on the struct directly

When we implement the `f` method for the `Foo` trait on `Baz`, it prints out
`Baz's impl of Foo`. When we implement the `f` method for the `Bar` trait on
`Baz`, it prints out `Baz's impl of Bar`. The implementation of `f` directly on
`Baz` prints out `Baz's impl`.

So what should happen when we implement `f` for `Baz` with `b.f()`? In this
case, Rust will use the implementation on `Baz` directly and will print out
`Baz's impl`. We want to be able to call the `f` method from `Foo` and the `f`
method from `Baz` rather than the implementation of `f` directly on `Baz`. To
do this, we need to use the *fully qualified syntax* for calling methods. It
works like this: for any method call like:

```
receiver.method(args);
```

We can fully qualify the method call with a signature like this:

```
<Type as Trait>::method(receiver, args);
```

In our case, to disambiguate so we can call all the `f` methods defined in
Listing 19-27, we first need to specify that we want to treat the type `Baz` as
each trait within angle brackets. We then use two colons, call the `f` method,
and pass the instance of `Baz` as the `receiver` argument.

<!-- what are the other arguments you might add to that? -->

Listing 19-28 shows how to call `f` from `Foo` and then `f` from `Bar` on `b`:

Filename: src/main.rs

```
fn main() {
    let b = Baz;
    b.f();
    <Baz as Foo>::f(&b);
    <Baz as Bar>::f(&b);
}
```

Listing 19-28: Using fully qualified syntax to call the `f` methods defined as
part of the `Foo` and `Bar` traits

This will print:

```
Baz's impl
Baz’s impl of Foo
Baz’s impl of Bar
```

We only need to use the `Type as` part if it’s ambiguous, and we only need the
`<>` part if we need the `Type as` part. So if we only had the `f` method
directly on `Baz` and the `Foo` trait implemented on `Baz` in scope, we could
call the `f` method in `Foo` by using `Foo::f(&b)` since we wouldn’t have to
disambiguate from the `Bar` trait.

We could also have called the `f` defined directly on `Baz` by using
`Baz::f(&b)`, but that definition of `f` is the one used by default when we
call `b.f()`, so isn't required.

### Implementing Supertraits to Use One Trait’s Functionality Within Another Trait

Sometimes, we may need one trait to use another trait's functionality. In this
case, we need to be able to rely on the dependent trait also being implemented.
The trait we're relying on is a *supertrait* of the trait we’re implementing.

For example, let’s say we want to make an `OutlinePrint` trait with an
`outline_print` method that will print out a value framed in asterisks. That
is, our `Point` struct will implement `Display` and result in `(x, y)`. When we
calle `outline_print` on a `Point` instance that has 1 for `x` and 3 for `y`,
it would result in the following:

```
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of `outline_print`, we need to use the `Display` trait’s
functionality. We therefore need to specify that the `OutlinePrint` trait will
only work for types that also implement `Display` and provide the functionality
that `OutlinePrint` needs. We can do that in the trait definition by specifying
`OutlinePrint: Display`. This is similar to adding a trait bound to the trait.
Listing 19-29 shows an implementation of the `OutlinePrint` trait:

```
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

Listing 19-29: Implementing the `OutlinePrint` trait that requires the
functionality from `Display`

Because we’ve specified that `OutlinePrint` requires the `Display` trait, we
can use the `to_string` fuction that's automatically implemented for any type
that implements `Display`. If we tried to use `to_string` without adding`:
Display` after the trait name we’d get an error saying that no method named
`to_string` was found for the type `&Self` in the current scope.

Le'ts see what happens if we try to implement `OutlinePrint` on a type that
doesn’t implement `Display`, such as the `Point` struct:

```
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

We’ll get an error saying that `Display` is required but not implemented:

```
error[E0277]: the trait bound `Point: std::fmt::Display` is not satisfied
  --> src/main.rs:20:6
   |
20 | impl OutlinePrint for Point {}
   |      ^^^^^^^^^^^^ the trait `std::fmt::Display` is not implemented for
   `Point`
   |
   = note: `Point` cannot be formatted with the default formatter; try using
   `:?` instead if you are using a format string
   = note: required by `OutlinePrint`
```

Once we implement `Display` on `Point` and satisfy the constraint that
`OutlinePrint` requires, like so:

```
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

then implementing the `OutlinePrint` trait on `Point` will compile successfully
and we can call `outline_print` on a `Point` instance to display it within an
outline of asterisks.

### The Newtype Pattern to Implement External Traits on External Types

In Chapter 10, we mentioned the orphan rule that says we’re allowed to
implement a trait on a type as long as either the trait or the type are local
to our crate. It is possible to get around this restriction using the *newtype
pattern*, which involves creating a new type in a tuple struct, with one field
as a thin wrapper around the type we want to implement a trait for. Then the
wrapper type is local to our crate, and we can implement the trait on the
wrapper. “Newtype” is a term originating from the Haskell programming language.
There’s no runtime performance penalty for using this pattern, and the wrapper
type is elided at compile time.

As our example, we want to implement `Display` on `Vec`. We can make a
`Wrapper` struct that holds an instance of `Vec`, then we can implement
`Display` on `Wrapper` and use the `Vec` value as shown in Listing 19-30:

Filename: src/main.rs

```
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

Listing 19-30: Creating a `Wrapper` type around `Vec<String>` to be able to
implement `Display`

The implementation of `Display` uses `self.0` to access the inner `Vec`, and
then we can use the functionality of the `Display` type on `Wrapper`.

<!-- What is self.0? I think the syntax here might need a bit more talking
through -->

The downside of this method is that, since `Wrapper` is a new type, it doesn’t
have the methods of the value it’s holding; we’d have to implement all the
methods of `Vec` directly on `Wrapper`, so that it can delegate to `self.0`---
this allows us to treat `Wrapper` exactly like a `Vec`. If we wanted the new
type to have every single method that the inner type has, implementing the
`Deref` trait that we discussed in Chapter 15 on the wrapper to return the
inner type can be a solution. If we don’t want the wrapper type to have all the
methods of the inner type, in order to restrict the wrapper type’s behavior for
example, we’d have to implement just the methods we do want ourselves.

That’s how the newtype pattern is used in relation to traits; it’s also a
useful pattern without having traits involved. Let’s switch focus now to talk
about some advanced ways to interact with Rust’s type system.

## Advanced Types

The Rust type system has some features that we’ve mentioned in this book but
haven't yet discussed. We’ll start our discussion on advanced types with a more
general discussion about why newtypes are useful as types. We’ll then move to
type aliases, a feature similar to newtypes but with slightly different
semantics. We’ll also discuss the `!` type and dynamically sized types.

### Using the Newtype Pattern for Type Safety and Abstraction

> This section assumed you've read the newtype pattern section in the “Advanced
> Traits” section.

The newtype pattern is useful for other things beyond what we've discussed do
far, including statically enforcing that values are never confused, and as
indication of the units of a value. We actually had an example of this in
Listing 19-26: the `Millimeters` and `Meters` structs both wrap `u32` values in
a new type. If we write a function with a parameter of type `Millimeters`, we
won’t be able to compile a program that accidentally tries to call that
function with a value of type `Meters` or a plain `u32`.

Another use is in abstracting away some implementation details of a type: the
wrapper type can expose a public API that's different to the API of the private
inner type, if we used it directly to restrict the available functionality, for
example.

New types can also hide internal generic types. For example, we could provide a
`People` type to wrap a `HashMap<i32, String>` that stores a person’s ID
associated with their name. Code using `People` would only interact with the
public API we provide, such as a method to add a name string to the `People`
collection, and that code wouldn’t need to know that we assign an `i32` ID to
names internally. The newtype pattern is a lightweight way to achieve
encapsulation to hide implementation details that we discussed in Chapter 17.

### Type Aliases Create Type Synonyms

Alongside the newtype pattern, Rust provides the ability to declare a *type
alias* to give an existing type another name. For this we use the `type`
keyword. For example, we can create the alias `Kilometers` to `i32` like so:

```
type Kilometers = i32;
```

This means `Kilometers` is a *synonym* for `i32`; unlike the `Millimeters` and
`Meters` types we created in Listing 19-26, `Kilometers` is not a separate, new
type. Values that have the type `Kilometers` will be treated exactly the same
as values of type `i32`:

```
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

Since `Kilometers` and `i32`, are the same type, we can add values of both
types, and we can also pass `Kilometers` values to functions that take `i32`
parameters. With this method, though, we don’t get the type checking benefits
that we get from the newtype pattern discussed in the previous section.

The main use case for type synonyms is to reduce repetition. For example, we
may have a lengthy type like this:

```
Box<Fn() + Send + 'static>
```

Writing this out in function signatures and as type annotations all over the
place can be tiresome and error-prone. Imagine having a project full of code
like that in Listing 19-31:

```
let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));

fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    // ...snip...
}

fn returns_long_type() -> Box<Fn() + Send + 'static> {
    // ...snip...
}
```

Listing 19-31: Using a long type in many places

A type alias makes this code more manageable by reducing the repetition. Here,
we’ve introduced an alias named `Thunk` for the verbose type, and can replace
all uses of the type with the shorter `Thunk` as shown in Listing 19-32:

```
type Thunk = Box<Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // ...snip...
}

fn returns_long_type() -> Thunk {
    // ...snip...
}
```

<!-- '-->

Listing 19-32: Introducing a type alias `Thunk` to reduce repetition

Much easier to read and write! Choosing a good name for a type alias can help
communicate your intent as well (*thunk* is a word for code to be evaluated at
a later time, so it’s an appropriate name for a closure that gets stored).

Type aliases are also commonly used with the `Result<T, E>` type for reducing
repetition and reducing risk of mistakes. Consider the `std::io` module in the
standard library. I/O operations often return a `Result<T, E>` to handle
situations when operations fail to work. This library has a `std::io::Error`
struct that represents all possible I/O errors. Many of the functions in
`std::io` will be returning `Result<T, E>` where the `E` is `std::io::Error`,
such as these functions in the `Write` trait:

```
use std::io::Error;
use std::fmt;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

We have `Result<..., Error>` repeated a lot. As such, `std::io` has this type
alias declaration:

```
type Result<T> = Result<T, std::io::Error>;
```

Because this is in the `std::io` module, we can use the fully qualified alias
`std::io::Result<T>`; that is, a `Result<T, E>` with the `E` filled in as
`std::io::Error`. The `Write` trait function signatures end up looking like
this:

```
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
}
```

The type alias helps in two ways: this is easier to write *and* it gives us a
consistent interface across all of `std::io`. Because it’s an alias, it is just
another `Result<T, E>`, which means we can use any methods that work on
`Result<T, E>` with it, as well as special syntax like `?`.

### The `!` Never Type that Never Returns

Rust has a special type named `!` that's known in type theory lingo as the
*empty type*, because it has no values. We prefer to call it the *never type*,
because it stands in the place of the return type when a function will never
return. For example:

```
fn bar() -> ! {
    // ...snip...
}
```

This is read as “the function `bar` returns never.” Functions that return never
are called *diverging functions*. We can’t create values of the type `!`, so
`bar` can never possibly return.

But what use is a type you can never create values for? If you think all the
way back to Chapter 2, we had some code that looked like Listing 19-33:

```
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

Listing 19-33: A `match` with an arm that ends in `continue`

At the time, we skipped over some details in this code. In Chapter 6, we
learned that `match` arms must all return the same type. This, for example,
doesn’t work:

```
let guess = match guess.trim().parse()  {
    Ok(_) => 5,
    Err(_) => "hello",
}
```

The type of `guess` here would have to be both an integer and a string, and
Rust requires that `guess` can only have one type. So what does `continue`
return? How were we allowed to return a `u32` from one arm and have another arm
that ends with `continue` in Listing 19-33?

As you may have guessed, `continue` has a value of `!`. That is, when Rust goes
to compute the type of `guess`, it looks at both of the match arms, the former
with a value of `u32`, and the latter a value of `!`. Since `!` can never have
a value, Rust decides that the type of `guess` is `u32`.

The formal way of describing this behavior is that the never type unifies with
all other types. We’re allowed to end this `match` arm with `continue` because
`continue` doesn’t actually return a value; it instead moves control back to
the top of the loop, so in the `Err` case, we never actually assign a value to
`guess`.

<!-- I'm not sure I'm following what would then occur in the event of an error,
literally nothing? -->

The never type is also useful with\ `panic!`. Remember the `unwrap` function
that we call on `Option<T>` values to produce a value or panic? Here’s its
definition:

```
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

Here, the same thing happens as in the `match` in Listing 19-33: we know that
`val` has the type `T`, and `panic!` has the type `!`, so the result of the
overall `match` expression is `T`. This works because `panic!` doesn’t produce
a value; it ends the program. In the `None` case, we won’t be returning a value
from `unwrap`, so this code is valid.

One final expression that has the type `!` is a `loop`:

```
print!("forever ");

loop {
    print!("and ever ");
}
```

Here, the loop never ends, so the value of the expression is `!`. This wouldn’t
be true if we included a `break`, however, as the loop would terminate when it
got to the `break`.

### Dynamically Sized Types & `Sized`

Due to Rust's need to know things like memory layout, there’s a particular
corner of its type system that can be confusing: the concept of *dynamically
sized types*. Sometimes referred to as ‘DSTs’ or ‘unsized types’, these types
let us talk about types whose size we can only know at runtime.

Let’s dig into the details of a dynamically sized type that we’ve been using
this whole book: `str`. That’s right, not `&str`, but `str` on its own, is a
DST. We can’t know how long the string is until runtime, meaning we can’t
create a variable of type `str`, nor can we take an argument of type `str`.
Consider this code, which does not work:

```
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

<!-- Why do they need to have the same memory layout? Perhaps I'm not
understanding fully what is meant by the memory layout, is it worth explaining
that a little in this section? -->

These two `str` values would need to have the exact same memory layout, but
they have different lengths: `s1` needs 12 bytes of storage, and `s2` needs 15.
This is why it’s not possible to create a variable holding a dynamically sized
type.

So what to do? You already know the answer in this case: we make the types of
`s1` and `s2` a `&str` rather than `str`. If you think back to Chapter 4, we
said this about `&str`:

> ... it’s a reference to an internal position in the String and the number of
> elements that it refers to.

So while a `&T` is a single value that stores the memory address of where the
`T` is located, a `&str` is *two* values: the address of the `str` and its
lengty. As such, a `&str` has a size we can know at compile time: it’s two
times the size of a `usize` in length. That is, we always know the size of a
`&str`, no matter how long the string it refers to is. This is the general way
in which dynamically sized types are used in Rust; they have an extra bit of
metadata that stores the size of the dynamic information. This leads us to the
golden rule of dynamically sized types: we must always put values of
dynamically sized types behind a pointer of some kind.

<!-- Note for Carol: `Rc<str>` is only in an accepted RFC right now, check on
its progress and pull this out if it's not going to be stable by Oct -->

We can combine `str` with all kinds of pointers: `Box<str>`, for example, or
`Rc<str>`. In fact, you’ve seen this before, but with a different dynamically
sized type: traits. Every trait is a dynamically sized type we can refer to by
using the name of the trait. In Chapter 17, we mentioned that in order to use
traits as trait objects, we have to put them behind a pointer like `&Trait` or
`Box<Trait>` (`Rc<Trait>` would work too). Traits being dynamically sized is
the reason we have to do that!

#### The `Sized` Trait

<!-- If we end up keeping the section on object safety in ch 17, we should add
a back reference here. /Carol -->

<!-- I think we dropped that one, right? -->

To work with DSTs, Rust has a particualr trait to determine if a type’s size is
known at compile time or not: the `Sized` trait. This trait is automatically
implemented for everything whose size is known at compile time. In addition,
Rust implicitly adds a bound on `Sized` to every generic function. That is, a
generic function definition like this:

```
fn generic<T>(t: T) {
    // ...snip...
}
```

is actually treated as if we had written this:

```
fn generic<T: Sized>(t: T) {
    // ...snip...
}
```

By default, generic functions will only work on types that have a known size at
compile time. There is, however, special syntax you can use to relax this
restriction:

```
fn generic<T: ?Sized>(t: &T) {
    // ...snip...
}
```

A trait bound on `?Sized` is the opposite of a trait bound on `Sized`; that is,
we would read this as “`T` may or may not be `Sized`”. This syntax is only
available for `Sized`, no other traits.

Also note we switched the type of the `t` parameter from `T` to `&T`: since the
type might not be `Sized`, we need to use it behind some kind of pointer. In
this case, we’ve chosen a reference.

Next let’s talk about functions and closures!

## Advanced Functions & Closures

Finally, let’s discuss some advanced features related to functions and
closures: function pointers, diverging functions, and returning closures.

### Function Pointers

<!-- Maybe give an example of when we'd want to use this? -->

We’ve talked about how to pass closures to functions; you can also pass regular
functions to functions! We do this using function pointers to allow us to use
functions as arguments to other functions. Functions coerce to the type `fn`,
with a lower case ‘f’ not to be confused with the `Fn` closure trait. The `fn`
type is called a *function pointer*. The syntax for specifying that a parameter
is a function pointer is similar to that of closures, as shown in Listing 19-34:

Filename: src/main.rs

```
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

Listing 19-34: Using the `fn` type to accept a function pointer as an argument

This prints `The answer is: 12`. We specify that the parameter `f` in
`do_twice` is an `fn` that takes one parameter of type `i32` and returns an
`i32`. We can then call `f` in the body of `do_twice`. In `main`, we can pass
the function name `add_one` as the first argument to `do_twice`.

Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the
parameter type directly, rather than declaring a generic type parameter with
one of the `Fn` traits as a trait bound.

Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and
`FnOnce`), so we can always pass a function pointer as an argument for a
function that expects a closure. Prefer to write functions using a generic type
and one of the closure traits, so that your functions can accept either
functions or closures.

An example of a case where you’d want to only accept `fn` and not closures is
when interfacing with external code that doesn’t have closures: C functions can
accept functions as arguments, but C doesn’t have closures.

For example, if we wanted to use the `map` function to turn a vector of numbers
into a vector of strings, we could use a closure:

```
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();
```

Or we could name a function as the argument to `map` instead of the closure:

```
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();
```

Note that we do have to use the fully qualified syntax that we talked about in
the “Advanced Traits” section because there are multiple functions available
named `to_string`; here, we’re using the `to_string` function defined in the
`ToString` trait, which the standard library has implemented for any type that
implements `Display`.

Some people prefer this style, some people prefer to use closures. They end up
with the same code, so use whichever feels more clear to you.

### Returning Closures

Closures are represented by traits, which means we can’t return closures
directly. In most cases where we may want to return a trait, we can instead use
the concrete type that implements the trait as the return value of the
function. We can’t do that with closures, though, because they don’t have a
concrete type that’s returnable; we’re not allowed to use the function pointer
`fn` as a return type, for example.

This code that tries to return a closure directly won’t compile:

```
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
```

The compiler error is:

```
error[E0277]: the trait bound `std::ops::Fn(i32) -> i32 + 'static:
std::marker::Sized` is not satisfied
 --> <anon>:2:25
  |
2 | fn returns_closure() -> Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^ the trait `std::marker::Sized` is
  not implemented for `std::ops::Fn(i32) -> i32 + 'static`
  |
  = note: `std::ops::Fn(i32) -> i32 + 'static` does not have a constant size
  known at compile-time
  = note: the return type of a function must have a statically known size
```

Our error references the `Sized` trait again! Rust doesn’t know how much space
it will need to store the closure. We saw a solution to this in the previous
section: we can use a trait object:

```
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

This code will compile just fineFor more about trait objects, refer back to
Chapter 18.

## Summary

Whew! Now we’ve gone over features of Rust that aren’t used often, but are
available if you need them in very particular circumstances. We’ve introduced a
lot of complex topics so that, when you encounter them in error message
suggestions or in others’ code, you’ll at least have seen these concepts and
syntax once before. You can use this chapter as a reference to guide you to
your solutions.

Now, let’s put everything we’ve learned throughout the book into practice with
one more project!
