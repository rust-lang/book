
[TOC]

# Advanced Features

We’ve come a long way! By now, we’ve learned 99% of the things you’ll need to
know when writing Rust. Before we do one more project in Chapter 20, let’s talk
about a few things that you may run into that last 1% of the time. Feel free to
skip this chapter and come back to it once you run into these things in the
wild; the features we’ll learn to use here are useful in very specific
situations. We don’t want to leave these features out, but you won’t find
yourself reaching for them often.

In this chapter, we’re going to cover:

* Unsafe Rust: for when you need to opt out of some of Rust’s guarantees and
  tell the compiler that you will be responsible for upholding the guarantees
  instead
* Advanced Lifetimes: Additional lifetime syntax for complex situations
* Advanced Traits: Associated Types, default type parameters, fully qualified
  syntax, supertraits, and the newtype pattern in relation to traits
* Advanced Types: some more about the newtype pattern, type aliases, the
  “never” type, and dynamically sized types
* Advanced Functions and Closures: function pointers and returning closures

It’s a panoply of Rust features with something for everyone! Let’s dive in!

## Unsafe Rust

In all of the previous chapters in this book, we’ve been discussing code
written in Rust that has memory safety guarantees enforced at compile time.
However, Rust has a second language hiding out inside of it, unsafe Rust, which
does not enforce these memory safety guarantees. Unsafe Rust works just like
regular Rust does, but it gives you extra superpowers not available in safe
Rust code.

Unsafe Rust exists because, by nature, static analysis is conservative. When
trying to determine if code upholds some guarantees or not, it’s better to
reject some programs that are valid than it is to accept some programs that are
invalid. There are some times when your code might be okay, but Rust thinks
it’s not! In these cases, you can use unsafe code to tell the compiler, “trust
me, I know what I’m doing.” The downside is that you’re on your own; if you get
unsafe code wrong, problems due to memory unsafety like null pointer
dereferencing can occur.

There’s another reason that Rust needs to have unsafe code: the underlying
hardware of computers is inherently not safe. If Rust didn’t let you do unsafe
operations, there would be some tasks that you simply could not do. But Rust
needs to be able to let you do low-level systems programming like directly
interacting with your operating system, or even writing your own operating
system! That’s part of the goals of the language. We need some way to do these
kinds of things.

### Unsafe Superpowers

We switch into unsafe Rust by using the `unsafe` keyword and starting a new
block that holds the unsafe code. There are four actions that you can take in
unsafe Rust that you can’t in safe Rust. We call these the “unsafe
superpowers.” We haven’t seen most of these features yet since they’re only
usable with `unsafe`!

1. Dereferencing a raw pointer
2. Calling an unsafe function or method
3. Accessing or modifying a mutable static variable
4. Implementing an unsafe trait

It’s important to understand that `unsafe` doesn’t turn off the borrow checker
or disable any other of Rust’s safety checks: if you use a reference in unsafe
code, it will still be checked. The only thing the `unsafe` keyword does is
give you access to these four features that aren’t checked by the compiler for
memory safety. You still get some degree of safety inside of an unsafe block!
Furthermore, `unsafe` does not mean the code inside the block is dangerous or
definitely will have memory safety problems: the intent is that you as the
programmer will ensure that the code inside an `unsafe` block will have valid
memory, since you’ve turned off the compiler checks.

People are fallible, however, and mistakes will happen. By requiring these four
unsafe operations to be inside blocks annotated with `unsafe`, if you make a
mistake and get an error related to memory safety, you’ll know that it has to
be related to one of the places that you opted into this unsafety. That makes
the cause of memory safety bugs much easier to find, since we know Rust is
checking all of the other code for us. To get this benefit of only having a few
places to investigate memory safety bugs, it’s important to contain your unsafe
code to as small of an area as possible. Any code inside of an `unsafe` block
is suspect when debugging a memory problem: keep `unsafe` blocks small and
you’ll thank yourself later since you’ll have less code to investigate.

In order to isolate unsafe code as much as possible, it’s a good idea to
enclose unsafe code within a safe abstraction and provide a safe API, which
we’ll be discussing once we get into unsafe functions and methods. Parts of the
standard library are implemented as safe abstractions over unsafe code that has
been audited. This prevents uses of `unsafe` from leaking out into all the
places that you or your users might want to make use of the functionality
implemented with `unsafe` code, since using a safe abstraction is safe.

Let’s talk about each of the four unsafe superpowers in turn, and along the way
we’ll look at some abstractions that provide a safe interface to unsafe code.

### Dereferencing a Raw Pointer

Way back in Chapter 4, we first learned about references. We also learned that
the compiler ensures that references are always valid. Unsafe Rust has two new
types similar to references called *raw pointers*. Just like references, we can
have an immutable raw pointer and a mutable raw pointer, written as `*const T`
and `*mut T`, respectively. In the context of raw pointers, “immutable” means
that the pointer can’t be directly assigned to after being dereferenced.

Raw pointers are different than references and smart pointers in a few ways.
Raw pointers:

- Are allowed to ignore the borrowing rules and have both immutable and a
  mutable pointer or multiple mutable pointers to the same location
- Aren't guaranteed to point to valid memory
- Are allowed to be null
- Don't implement any automatic clean-up

Listing 19-1 shows how to create raw pointers from references:

```
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

Listing 19-1: Creating raw pointers from references

The `*const T` type is an immutable raw pointer, and `*mut T` is a mutable raw
pointer. We’ve created raw pointers by using `as` to cast an immutable and a
mutable reference into their corresponding raw pointer types. These particular
raw pointers will be valid since we created them directly from references that
are guaranteed to be valid, but we can't make that assumption about any raw
pointer.

Listing 19-2 shows how to create a raw pointer to an arbitrary location in
memory. Trying to use arbitrary memory is undefined: there may be data at that
address, there may not be any data at that address, the compiler might optimize
the code so that there is no memory access, or your program might segfault.
There’s not usually a good reason to be writing code like this, but it is
possible:

```
let address = 0x012345usize;
let r = address as *const i32;
```

Listing 19-2: Creating a raw pointer to an arbitrary memory address

Note there’s no `unsafe` block in either Listing 19-1 or 19-2. You can *create*
raw pointers in safe code, but you can’t *dereference* raw pointers and read
the data being pointed to. Using the dereference operator, `*`, on a raw
pointer requires an `unsafe` block, as shown in Listing 19-3:

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

Note also that in Listing 19-1 and 19-3 we created a `*const i32` and a `*mut
i32` that both pointed to the same memory location, that of `num`. If we had
tried to create an immutable and a mutable reference to `num` instead of raw
pointers, this would not have compiled due to the rule that says we can’t have
a mutable reference at the same time as any immutable references. With raw
pointers, we are able to create a mutable pointer and an immutable pointer to
the same location, and change data through the mutable pointer, potentially
creating a data race. Be careful!

With all of these dangers, why would we ever use raw pointers? One major use
case is interfacing with C code, as we’ll see in the next section on unsafe
functions. Another case is to build up safe abstractions that the borrow
checker doesn’t understand. Let’s introduce unsafe functions then look at an
example of a safe abstraction that uses unsafe code.

### Calling an Unsafe Function or Method

The second operation that requires an unsafe block is calling an unsafe
function. Unsafe functions and methods look exactly like regular functions and
methods, but they have an extra `unsafe` out front. Bodies of unsafe functions
are effectively `unsafe` blocks. Here’s an unsafe function named `dangerous`:

```
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

If we try to call `dangerous` without the `unsafe` block, we’ll get an error:

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

As an example, let’s check out some functionality from the standard library,
`split_at_mut`, and explore how we might implement it ourselves. This safe
method is defined on mutable slices, and it takes one slice and makes it into
two by splitting the slice at the index given as an argument, as demonstrated
in Listing 19-4:

```
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

Listing 19-4: Using the safe `split_at_mut`
function

This function can’t be implemented using only safe Rust. An attempt might look
like Listing 19-5. For simplicity, we’re implementing `split_at_mut` as a
function rather than a method, and only for slices of `i32` values rather than
for a generic type `T`:

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
index given as a parameter is within the slice by checking that the parameter
is less than or equal to the length. The assertion means that if we pass an
index that’s greater than the length of the slice to split at, the function
will panic before it attempts to use that index.

Then we return two mutable slices in a tuple: one from the start of the initial
slice to the `mid` index, and another from `mid` to the end of the slice.

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
Borrowing different parts of a slice is fundamentally okay; our two `&mut
[i32]` slices aren’t overlapping. However, Rust isn’t smart enough to know
this. When we know something is okay, but Rust doesn’t, it’s time to reach for
unsafe code.

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
the slice. We’ve often used the `len` method to get the length of a slice; we
can use the `as_mut_ptr` method to get access to the raw pointer of a slice. In
this case, since we have a mutable slice to `i32` values, `as_mut_ptr` returns
a raw pointer with the type `*mut i32`, which we’ve stored in the variable
`ptr`.

The assertion that the `mid` index is within the slice stays the same. Then,
the `slice::from_raw_parts_mut` function does the reverse from the `as_mut_ptr`
and `len` methods: it takes a raw pointer and a length and creates a slice. We
call `slice::from_raw_parts_mut` to create a slice that starts from `ptr` and is
`mid` items long. Then we call the `offset` method on `ptr` with `mid` as an
argument to get a raw pointer that starts at `mid`, and we create a slice using
that pointer and the remaining number of items after `mid` as the length.

Because slices are checked, they’re safe to use once we’ve created them. The
function `slice::from_raw_parts_mut` is an unsafe function because it takes a
raw pointer and trusts that this pointer is valid. The `offset` method on raw
pointers is also unsafe, since it trusts that the location some offset after a
raw pointer is also a valid pointer. We’ve put an `unsafe` block around our
calls to `slice::from_raw_parts_mut` and `offset` to be allowed to call them,
and we can tell by looking at the code and by adding the assertion that `mid`
must be less than or equal to `len` that all the raw pointers used within the
`unsafe` block will be valid pointers to data within the slice. This is an
acceptable and appropriate use of `unsafe`.

Note that the resulting `split_at_mut` function is safe: we didn’t have to add
the `unsafe` keyword in front of it, and we can call this function from safe
Rust. We’ve created a safe abstraction to the unsafe code by writing an
implementation of the function that uses `unsafe` code in a safe way by only
creating valid pointers from the data this function has access to.

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
`slice` as if it was a valid slice would be undefined behavior.

#### `extern`  Functions for Calling External Code are Unsafe

Sometimes, your Rust code may need to interact with code written in another
language. To do this, Rust has a keyword, `extern`, that facilitates creating
and using a *Foreign Function Interface* (FFI). Listing 19-8 demonstrates how
to set up an integration with the `abs` function defined in the C standard
library. Functions declared within `extern` blocks are always unsafe to call
from Rust code:

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

Within the `extern "C"` block, we list the names and signatures of functions
defined in a library written in another language that we want to be able to
call.`"C"` defines which *application binary interface* (ABI) the external
function uses. The ABI defines how to call the function at the assembly level.
The `"C"` ABI is the most common, and follows the C programming language’s ABI.

Calling an external function is always unsafe. If we’re calling into some other
language, that language does not enforce Rust’s safety guarantees. Since Rust
can’t check that the external code is safe, we are responsible for checking the
safety of the external code and indicating we have done so by using an `unsafe`
block to call external functions.

<!-- PROD: START BOX -->

##### Calling Rust Functions from Other Languages

The `extern` keyword is also used for creating an interface that allows other
languages to call Rust functions. Instead of an `extern` block, we can add the
`extern` keyword and specifying the ABI to use just before the `fn` keyword. We
also add the `#[no_mangle]` annotation to tell the Rust compiler not to mangle
the name of this function. The `call_from_c` function in this example would be
accessible from C code, once we’ve compiled to a shared library and linked from
C:

```
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

This usage of `extern` does not require `unsafe`

<!-- PROD: END BOX -->

### Accessing or Modifying a Mutable Static Variable

We’ve gone this entire book without talking about *global variables*. Many
programming languages support them, and so does Rust. However, global variables
can be problematic: for example, if you have two threads accessing the same
mutable global variable, a data race can happen.

Global variables are called *static* in Rust. Listing 19-9 shows an example
declaration and use of a static variable with a string slice as a value:

Filename: src/main.rs

```
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

Listing 19-9: Defining and using an immutable static variable

`static` variables are similar to constants: their names are also in
`SCREAMING_SNAKE_CASE` by convention, and we *must* annotate the variable’s
type, which is `&'static str` in this case. Only references with the `'static`
lifetime may be stored in a static variable. Because of this, the Rust compiler
can figure out the lifetime by itself and we don’t need to annotate it
explicitly.

Accessing immutable static variables is safe. Values in a static variable have a
fixed address in memory, and using the value will always access the same data.
Constants, on the other hand, are allowed to duplicate their data whenever they
are used.

Another way in which static variables are different from constants is that
static variables can be mutable. Both accessing and modifying mutable static
variables is unsafe. Listing 19-10 shows how to declare, access, and modify a
mutable static variable named `COUNTER`:

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

Just like with regular variables, we specify that a static variable should be
mutable using the `mut` keyword. Any time that we read or write from `COUNTER`
has to be within an `unsafe` block. This code compiles and prints `COUNTER: 3`
as we would expect since it’s single threaded, but having multiple threads
accessing `COUNTER` would likely result in data races.

Mutable data that is globally accessible is difficult to manage and ensure that
there are no data races, which is why Rust considers mutable static variables
to be unsafe. If possible, prefer using the concurrency techniques and
threadsafe smart pointers we discussed in Chapter 16 to have the compiler check
that data accessed from different threads is done safely.

### Implementing an Unsafe Trait

Finally, the last action we’re only allowed to take when we use the `unsafe`
keyword is implementing an unsafe trait. We can declare that a trait is
`unsafe` by adding the `unsafe` keyword before `trait`, and then implementing
the trait must be marked as `unsafe` too, as shown in Listing 19-11:

```
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```

Listing 19-11: Defining and implementing an unsafe trait

Like unsafe functions, methods in an unsafe trait have some invariant that the
compiler cannot verify. By using `unsafe impl`, we’re promising that we’ll
uphold these invariants.

As an example, recall the `Sync` and `Send` marker traits from Chapter 16, and
that the compiler implements these automatically if our types are composed
entirely of `Send` and `Sync` types. If we implement a type that contains
something that’s not `Send` or `Sync` such as raw pointers, and we want to mark
our type as `Send` or `Sync`, that requires using `unsafe`. Rust can’t verify
that our type upholds the guarantees that a type can be safely sent across
threads or accessed from multiple threads, so we need to do those checks
ourselves and indicate as such with `unsafe`.

Using `unsafe` to take one of these four actions isn’t wrong or frowned upon,
but it is trickier to get `unsafe` code correct since the compiler isn’t able
to help uphold memory safety. When you have a reason to use `unsafe` code,
however, it’s possible to do so, and having the explicit `unsafe` annotation
makes it easier to track down the source of problems if they occur.

## Advanced Lifetimes

Back in Chapter 10, we learned how to annotate references with lifetime
parameters to help Rust understand how the lifetimes of different references
relate. We saw how most of the time, Rust will let you elide lifetimes, but
every reference has a lifetime. There are three advanced features of lifetimes
that we haven’t covered though: *lifetime subtyping*, *lifetime bounds*, and
*trait object lifetimes*.

### Lifetime Subtyping

Imagine that we want to write a parser. To do this, we’ll have a structure that
holds a reference to the string that we’re parsing, and we’ll call that struct
`Context`. We’ll write a parser that will parse this string and return success
or failure. The parser will need to borrow the context to do the parsing.
Implementing this would look like the code in Listing 19-12, which won’t
compile because we’ve left off the lifetime annotations for now:

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

Listing 19-12: Defining a `Context` struct that holds a string slice, a
`Parser` struct that holds a reference to a `Context` instance, and a `parse`
method that always returns an error referencing the string slice

For simplicity’s sake, our `parse` function returns a `Result<(), &str>`. That
is, we don’t do anything on success, and on failure we return the part of the
string slice that didn’t parse correctly. A real implementation would have more
error information than that, and would actually return something created when
parsing succeeds, but we’re leaving those parts of the implementation off since
they aren’t relevant to the lifetimes part of this example. We’re also defining
`parse` to always produce an error after the first byte. Note that this may
panic if the first byte is not on a valid character boundary; again, we’re
simplifying the example in order to concentrate on the lifetimes involved.

So how do we fill in the lifetime parameters for the string slice in `Context`
and the reference to the `Context` in `Parser`? The most straightforward thing
to do is to use the same lifetime everywhere, as shown in Listing 19-13:

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

These errors are saying that both the `Parser` instance we’re creating and the
`context` parameter live from the line that the `Parser` is created until the
end of the `parse_context` function, but they both need to live for the entire
lifetime of the function.

In other words, `Parser` and `context` need to *outlive* the entire function
and be valid before the function starts as well as after it ends in order for
all the references in this code to always be valid. Both the `Parser` we’re
creating and the `context` parameter go out of scope at the end of the
function, though (since `parse_context` takes ownership of `context`).

Let’s look at the definitions in Listing 19-13 again, especially the signature
of the `parse` method:

```
    fn parse(&self) -> Result<(), &str> {
```

Remember the elision rules? If we annotate the lifetimes of the references, the
signature would be:

```
    fn parse<'a>(&'a self) -> Result<(), &'a str> {
```

That is, the error part of the return value of `parse` has a lifetime that is
tied to the `Parser` instance’s lifetime (that of `&self` in the `parse` method
signature). That makes sense, as the returned string slice references the
string slice in the `Context` instance that the `Parser` holds, and we’ve
specified in the definition of the `Parser` struct that the lifetime of the
reference to `Context` that `Parser` holds and the lifetime of the string slice
that `Context` holds should be the same.

The problem is that the `parse_context` function returns the value returned
from `parse`, so the lifetime of the return value of `parse_context` is tied to
the lifetime of the `Parser` as well. But the `Parser` instance created in the
`parse_context` function won’t live past the end of the function (it’s
temporary), and the `context` will go out of scope at the end of the function
(`parse_context` takes ownership of it).

We’re not allowed to return a reference to a value that goes out of scope at
the end of the function. Rust thinks that’s what we’re trying to do because we
annotated all the lifetimes with the same lifetime parameter. That told Rust
the lifetime of the string slice that `Context` holds is the same as that of
the lifetime of the reference to `Context` that `Parser` holds.

The `parse_context` function can’t see that within the `parse` function, the
string slice returned will outlive both `Context` and `Parser`, and that the
reference `parse_context` returns refers to the string slice, not to `Context`
or `Parser`.

By knowing what the implementation of `parse` does, we know that the only
reason that the return value of `parse` is tied to the `Parser` is because it’s
referencing the `Parser`’s `Context`, which is referencing the string slice, so
it’s really the lifetime of the string slice that `parse_context` needs to care
about. We need a way to tell Rust that the string slice in `Context` and the
reference to the `Context` in `Parser` have different lifetimes and that the
return value of `parse_context` is tied to the lifetime of the string slice in
`Context`.

We could try only giving `Parser` and `Context` different lifetime parameters
as shown in Listing 19-15. We’ve chosen the lifetime parameter names `'s` and
`'c` here to be clearer about which lifetime goes with the string slice in
`Context` and which goes with the reference to `Context` in `Parser`. Note that
this won’t completely fix the problem, but it’s a start and we’ll look at why
this isn’t sufficient when we try to compile.

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

Here’s the error we get now:

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

Rust doesn’t know of any relationship between `'c` and `'s`. In order to be
valid, the referenced data in `Context` with lifetime `'s` needs to be
constrained to guarantee that it lives longer than the reference to `Context`
that has lifetime `'c`. If `'s` is not longer than `'c`, then the reference to
`Context` might not be valid.

Which gets us to the point of this section: Rust has a feature called *lifetime
subtyping*, which is a way to specify that one lifetime parameter lives at
least as long as another one. In the angle brackets where we declare lifetime
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

In Chapter 10, we discussed how to use trait bounds on generic types. We can
also add lifetime parameters as constraints on generic types, which are called
*lifetime bounds*. For example, consider a type that is a wrapper over
references. Recall the `RefCell<T>` type from Chapter 15: its `borrow` and
`borrow_mut` methods return the types `Ref` and `RefMut`, respectively. These
types are wrappers over references that keep track of the borrowing rules at
runtime. The definition of the `Ref` struct is shown in Listing 19-16, without
lifetime bounds for now:

```
struct Ref<'a, T>(&'a T);
```

Listing 19-16: Defining a struct to wrap a reference to a generic type; without
lifetime bounds to start

Without constraining the lifetime `'a` in relation to the generic parameter
`T`, we get an error because Rust doesn’t know how long the generic type `T`
will live:

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

Fortunately, Rust gave us helpful advice on how to specify the lifetime bound in
this case:

```
consider adding an explicit lifetime bound `T: 'a` so that the reference type
`&'a T` does not outlive the data it points at.
```

Listing 19-17 shows how to apply this advice by specifying the lifetime bound
when we declare the generic type `T`. This code now compiles because the `T:
'a` syntax specifies that `T` can be any type, but if it contains any
references, the references must live at least as long as `'a`:

```
struct Ref<'a, T: 'a>(&'a T);
```

Listing 19-17: Adding lifetime bounds on `T` to specify that any references in
`T` live at least as long as `'a`

We could choose to solve this in a different way, shown in the definition of a
`StaticRef` struct in Listing 19-18, by adding the `'static` lifetime bound on
`T`. This means if `T` contains any references, they must have the `'static`
lifetime:

```
struct StaticRef<T: 'static>(&'static T);
```

Listing 19-18: Adding a `'static` lifetime bound to `T` to constrain `T` to
types that have only `'static` references or no references

Types without any references count as `T: 'static`. Because `'static` means the
reference must live as long as the entire program, a type that contains no
references meets the criteria of all references living as long as the entire
program (since there are no references). Think of it this way: if the borrow
checker is concerned about references living long enough, then there’s no real
distinction between a type that has no references and a type that has
references that live forever; both of them are the same for the purpose of
determining whether or not a reference has a shorter lifetime than what it
refers to.

### Trait Object Lifetimes

In Chapter 17, we learned about trait objects that consist of putting a trait
behind a reference in order to use dynamic dispatch. However, we didn’t discuss
what happens if the type implementing the trait used in the trait object has a
lifetime. Consider Listing 19-19, where we have a trait `Foo` and a struct
`Bar` that holds a reference (and thus has a lifetime parameter) that
implements trait `Foo`, and we want to use an instance of `Bar` as the trait
object `Box<Foo>`:

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
about the lifetimes involved in `obj`. This works because there are rules
having to do with lifetimes and trait objects:

* The default lifetime of a trait object is `'static`.
* If we have `&'a X` or `&'a mut X`, then the default is `'a`.
* If we have a single `T: 'a` clause, then the default is `'a`.
* If we have multiple `T: 'a`-like clauses, then there is no default; we must
  be explicit.

When we must be explicit, we can add a lifetime bound on a trait object like
`Box<Foo>` with the syntax `Box<Foo + 'a>` or `Box<Foo + 'static>`, depending
on what’s needed. Just as with the other bounds, this means that any
implementer of the `Foo` trait that has any references inside must have the
lifetime specified in the trait object bounds as those references.

Next, let’s take a look at some other advanced features dealing with traits!

## Advanced Traits

We covered traits in Chapter 10, but like lifetimes, we didn’t get to all the
details. Now that we know more Rust, we can get into the nitty-gritty.

### Associated Types

*Associated types* are a way of associating a type placeholder with a trait
such that the trait method definitions can use these placeholder types in their
signatures. The implementer of a trait will specify the concrete type to be
used in this type’s place for the particular implementation.

We’ve described most of the things in this chapter as being very rare.
Associated types are somewhere in the middle; they’re more rare than the rest
of the book, but more common than many of the things in this chapter.

An example of a trait with an associated type is the `Iterator` trait provided
by the standard library. It has an associated type named `Item` that stands in
for the type of the values that we’re iterating over. We mentioned in Chapter
13 that the definition of the `Iterator` trait is as shown in Listing 19-20:

```
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Listing 19-20: The definition of the `Iterator` trait that has an associated
type `Item`

This says that the `Iterator` trait has an associated type named `Item`. `Item`
is a placeholder type, and the return value of the `next` method will return
values of type `Option<Self::Item>`. Implementers of this trait will specify
the concrete type for `Item`, and the `next` method will return an `Option`
containing a value of whatever type the implementer has specified.

#### Associated Types Versus Generics

When we implemented the `Iterator` trait on the `Counter` struct in Listing
13-6, we specified that the `Item` type was `u32`:

```
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
```

This feels similar to generics. So why isn’t the `Iterator` trait defined as
shown in Listing 19-21?

```
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

Listing 19-21: A hypothetical definition of the `Iterator` trait using generics

The difference is that with the definition in Listing 19-21, we could also
implement `Iterator<String> for Counter`, or any other type as well, so that
we’d have multiple implementations of `Iterator` for `Counter`. In other words,
when a trait has a generic parameter, we can implement that trait for a type
multiple times, changing the generic type parameters’ concrete types each time.
Then when we use the `next` method on `Counter`, we’d have to provide type
annotations to indicate which implementation of `Iterator` we wanted to use.

With associated types, we can’t implement a trait on a type multiple times.
Using the actual definition of `Iterator` from Listing 19-20, we can only
choose once what the type of `Item` will be, since there can only be one `impl
Iterator for Counter`. We don’t have to specify that we want an iterator of
`u32` values everywhere that we call `next` on `Counter`.

The benefit of not having to specify generic type parameters when a trait uses
associated types shows up in another way as well. Consider the two traits
defined in Listing 19-22. Both are defining a trait having to do with a graph
structure that contains nodes of some type and edges of some type. `GGraph` is
defined using generics, and `AGraph` is defined using associated types:

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

Listing 19-22: Two graph trait definitions, `GGraph` using generics and
`AGraph` using associated types for `Node` and `Edge`

Let’s say we wanted to implement a function that computes the distance between
two nodes in any types that implement the graph trait. With the `GGraph` trait
defined using generics, our `distance` function signature would have to look
like Listing 19-23:

```
fn distance<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
    // ...snip...
}
```

Listing 19-23: The signature of a `distance` function that uses the trait
`GGraph` and has to specify all the generic parameters

Our function would need to specify the generic type parameters `N`, `E`, and
`G`, where `G` is bound by the trait `GGraph` that has type `N` as its `Node`
type and type `E` as its `Edge` type. Even though `distance` doesn’t need to
know the types of the edges, we’re forced to declare an `E` parameter, because
we need to to use the `GGraph` trait and that requires specifying the type for
`Edge`.

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

You may have been wondering why we didn’t use a trait object in the `distance`
functions in Listing 19-23 and Listing 19-24. The signature for the `distance`
function using the generic `GGraph` trait does get a bit more concise using a
trait object:

```
fn distance<N, E>(graph: &GGraph<N, E>, start: &N, end: &N) -> u32 {
    // ...snip...
}
```

This might be a more fair comparison to Listing 19-24. Specifying the `Edge`
type is still required, though, which means Listing 19-24 is still preferable
since we don’t have to specify something we don’t use.

It’s not possible to change Listing 19-24 to use a trait object for the graph,
since then there would be no way to refer to the `AGraph` trait’s associated
type.

It is possible in general to use trait objects of traits that have associated
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

While trait objects mean that we don’t need to know the concrete type of the
`graph` parameter at compile time, we do need to constrain the use of the
`AGraph` trait in the `traverse` function by the concrete types of the
associated types. If we didn’t provide this constraint, Rust wouldn’t be able
to figure out which `impl` to match this trait object to.

### Operator Overloading and Default Type Parameters

The `<PlaceholderType=ConcreteType>` syntax is used in another way as well: to
specify the default type for a generic type. A great example of a situation
where this is useful is operator overloading.

Rust does not allow you to create your own operators or overload arbitrary
operators, but the operations and corresponding traits listed in `std::ops` can
be overloaded by implementing the traits associated with the operator. For
example, Listing 19-25 shows how to overload the `+` operator by implementing
the `Add` trait on a `Point` struct so that we can add two `Point` instances
together:

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

We’ve implemented the `add` method to add the `x` values of two `Point`
instances together and the `y` values of two `Point` instances together to
create a new `Point`. The `Add` trait has an associated type named `Output`
that’s used to determine the type returned from the `add` method.

Let’s look at the `Add` trait in a bit more detail. Here’s its definition:

```
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

This should look familiar; it’s a trait with one method and an associated type.
The new part is the `RHS=Self` in the angle brackets: this syntax is called
*default type parameters*. `RHS` is a generic type parameter (short for “right
hand side”) that’s used for the type of the `rhs` parameter in the `add`
method. If we don’t specify a concrete type for `RHS` when we implement the
`Add` trait, the type of `RHS` will default to the type of `Self` (the type
that we’re implementing `Add` on).

Let’s look at another example of implementing the `Add` trait. Imagine we have
two structs holding values in different units, `Millimeters` and `Meters`. We
can implement `Add` for `Millimeters` in different ways as shown in Listing
19-26:

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

If we’re adding `Millimeters` to other `Millimeters`, we don’t need to
parameterize the `RHS` type for `Add` since the default `Self` type is what we
want. If we want to implement adding `Millimeters` and `Meters`, then we need
to say `impl Add<Meters>` to set the value of the `RHS` type parameter.

Default type parameters are used in two main ways:

1. To extend a type without breaking existing code.
2. To allow customization in a way most users don’t want.

The `Add` trait is an example of the second purpose: most of the time, you’re
adding two like types together. Using a default type parameter in the `Add`
trait definition makes it easier to implement the trait since you don’t have to
specify the extra parameter most of the time. In other words, we’ve removed a
little bit of implementation boilerplate.

The first purpose is similar, but in reverse: since existing implementations of
a trait won’t have specified a type parameter, if we want to add a type
parameter to an existing trait, giving it a default will let us extend the
functionality of the trait without breaking the existing implementation code.

### Fully Qualified Syntax for Disambiguation

Rust cannot prevent a trait from having a method with the same name as another
trait’s method, nor can it prevent us from implementing both of these traits on
one type. We can also have a method implemented directly on the type with the
same name as well! In order to be able to call each of the methods with the
same name, then, we need to tell Rust which one we want to use. Consider the
code in Listing 19-27 where traits `Foo` and `Bar` both have method `f` and we
implement both traits on struct `Baz`, which also has a method named `f`:

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

For the implementation of the `f` method for the `Foo` trait on `Baz`, we’re
printing out `Baz's impl of Foo`. For the implementation of the `f` method for
the `Bar` trait on `Baz`, we’re printing out `Baz's impl of Bar`. The
implementation of `f` directly on `Baz` prints out `Baz's impl`. What should
happen when we call `b.f()`? In this case, Rust will always use the
implementation on `Baz` directly and will print out `Baz's impl`.

In order to be able to call the `f` method from `Foo` and the `f` method from
`Baz` rather than the implementation of `f` directly on `Baz`, we need to use
the *fully qualified syntax* for calling methods. It works like this: for any
method call like:

```
receiver.method(args);
```

We can fully qualify the method call like this:

```
<Type as Trait>::method(receiver, args);
```

So in order to disambiguate and be able to call all the `f` methods defined in
Listing 19-27, we specify that we want to treat the type `Baz` as each trait
within angle brackets, then use two colons, then call the `f` method and pass
the instance of `Baz` as the first argument. Listing 19-28 shows how to call
`f` from `Foo` and then `f` from `Bar` on `b`:

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

We only need the `Type as` part if it’s ambiguous, and we only need the `<>`
part if we need the `Type as` part. So if we only had the `f` method directly
on `Baz` and the `Foo` trait implemented on `Baz` in scope, we could call the
`f` method in `Foo` by using `Foo::f(&b)` since we wouldn’t have to
disambiguate from the `Bar` trait.

We could also have called the `f` defined directly on `Baz` by using
`Baz::f(&b)`, but since that definition of `f` is the one that gets used by
default when we call `b.f()`, it’s not required to fully specify that
implementation if that’s what we want to call.

### Supertraits to Use One Trait’s Functionality Within Another Trait

Sometimes, we may want a trait to be able to rely on another trait also being
implemented wherever our trait is implemented, so that our trait can use the
other trait’s functionality. The required trait is a *supertrait* of the trait
we’re implementing.

For example, let’s say we want to make an `OutlinePrint` trait with an
`outline_print` method that will print out a value outlined in asterisks. That
is, if our `Point` struct implements `Display` to result in `(x, y)`, calling
`outline_print` on a `Point` instance that has 1 for `x` and 3 for `y` would
look like:

```
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of `outline_print`, since we want to be able to use the
`Display` trait’s functionality, we need to be able to say that the
`OutlinePrint` trait will only work for types that also implement `Display` and
provide the functionality that `OutlinePrint` needs. We can do that in the
trait definition by specifying `OutlinePrint: Display`. It’s like adding a
trait bound to the trait. Listing 19-29 shows an implementation of the
`OutlinePrint` trait:

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
can use `to_string` in `outline_print` (`to_string` is automatically
implemented for any type that implements `Display`). If we hadn’t added the `:
Display` after the trait name and we tried to use `to_string` in
`outline_print`, we’d get an error that no method named `to_string` was found
for the type `&Self` in the current scope.

If we try to implement `OutlinePrint` on a type that doesn’t implement
`Display`, such as the `Point` struct:

```
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

We’ll get an error that `Display` isn’t implemented and that `Display` is
required by `OutlinePrint`:

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

In Chapter 10, we mentioned the orphan rule, which says we’re allowed to
implement a trait on a type as long as either the trait or the type are local
to our crate. One way to get around this restriction is to use the *newtype
pattern*, which involves creating a new type using a tuple struct with one
field as a thin wrapper around the type we want to implement a trait for. Then
the wrapper type is local to our crate, and we can implement the trait on the
wrapper. “Newtype” is a term originating from the Haskell programming language.
There’s no runtime performance penalty for using this pattern. The wrapper type
is elided at compile time.

For example, if we wanted to implement `Display` on `Vec`, we can make a
`Wrapper` struct that holds an instance of `Vec`. Then we can implement
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

The downside is that since `Wrapper` is a new type, it doesn’t have the methods
of the value it’s holding; we’d have to implement all the methods of `Vec` like
`push`, `pop`, and all the rest directly on `Wrapper` to delegate to `self.0`
in order to be able to treat `Wrapper` exactly like a `Vec`. If we wanted the
new type to have every single method that the inner type has, implementing the
`Deref` trait that we discussed in Chapter 15 on the wrapper to return the
inner type can be a solution. If we don’t want the wrapper type to have all the
methods of the inner type, in order to restrict the wrapper type’s behavior for
example, we’d have to implement just the methods we do want ourselves.

That’s how the newtype pattern is used in relation to traits; it’s also a
useful pattern without having traits involved. Let’s switch focus now to talk
about some advanced ways to interact with Rust’s type system.

## Advanced Types

The Rust type system has some features that we’ve mentioned or used without
discussing. We started talking about the newtype pattern in regards to traits;
we’ll start with a more general discussion about why newtypes are useful as
types. We’ll then move to type aliases, a feature that is similar to newtypes
but has slightly different semantics. We’ll also discuss the `!` type and
dynamically sized types.

### Using the Newtype Pattern for Type Safety and Abstraction

The newtype pattern that we started discussing at the end of the “Advanced
Traits” section, where we create a new type as a tuple struct with one field
that wraps a type can also be useful for statically enforcing that values are
never confused, and is often used to indicate the units of a value. We actually
had an example of this in Listing 19-26: the `Millimeters` and `Meters` structs
both wrap `u32` values in a new type. If we write a function with a parameter
of type `Millimeters`, we won’t be able to compile a program that accidentally
tries to call that function with a value of type `Meters` or a plain `u32`.

Another reason to use the newtype pattern is to abstract away some
implementation details of a type: the wrapper type can expose a different
public API than the private inner type would if we used it directly in order to
restrict the functionality that is available, for example. New types can also
hide internal generic types. For example, we could provide a `People` type that
wraps a `HashMap<i32, String>` that stores a person’s ID associated with their
name. Code using `People` would only interact with the public API we provide,
such as a method to add a name string to the `People` collection, and that code
wouldn’t need to know that we assign an `i32` ID to names internally. The
newtype pattern is a lightweight way to achieve encapsulation to hide
implementation details that we discussed in Chapter 17.

### Type Aliases Create Type Synonyms

The newtype pattern involves creating a new struct to be a new, separate type.
Rust also provides the ability to declare a *type alias* with the `type`
keyword to give an existing type another name. For example, we can create the
alias `Kilometers` to `i32` like so:

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

Since `Kilometers` is an alias for `i32`, they’re the same type. We can add
values of type `i32` and `Kilometers` together, and we can pass `Kilometers`
values to functions that take `i32` parameters. We don’t get the type checking
benefits that we get from the newtype pattern that we discussed in the previous
section.

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

A type alias makes this code more manageable by reducing the amount of
repetition this project has. Here, we’ve introduced an alias named `Thunk` for
the verbose type, and we can replace all uses of the type with the shorter
`Thunk` as shown in Listing 19-32:

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

Listing 19-32: Introducing a type alias `Thunk` to reduce repetition

Much easier to read and write! Choosing a good name for a type alias can help
communicate your intent as well (*thunk* is a word for code to be evaluated at
a later time, so it’s an appropriate name for a closure that gets stored).

Another common use of type aliases is with the `Result<T, E>` type. Consider
the `std::io` module in the standard library. I/O operations often return a
`Result<T, E>`, since their operations may fail to work. There’s a
`std::io::Error` struct that represents all of the possible I/O errors. Many of
the functions in `std::io` will be returning `Result<T, E>` where the `E` is
`std::io::Error`, such as these functions in the `Write` trait:

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

We’re writing `Result<..., Error>` a lot. As such, `std::io` has this type
alias declaration:

```
type Result<T> = Result<T, std::io::Error>;
```

Because this is in the `std::io` module, the fully qualified alias that we can
use is `std::io::Result<T>`; that is, a `Result<T, E>` with the `E` filled in
as `std::io::Error`. The `Write` trait function signatures end up looking like
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
`Result<T, E>` with it, and special syntax like `?`.

### The Never Type, `!`, that Never Returns

Rust has a special type named `!`. In type theory lingo, it’s called the *empty
type*, because it has no values. We prefer to call it the *never type*. The name
describes what it does: it stands in the place of the return type when a
function will never return. For example:

```
fn bar() -> ! {
    // ...snip...
}
```

This is read as “the function `bar` returns never,” and functions that return
never are called *diverging functions*. We can’t create values of the type `!`,
so `bar` can never possibly return. What use is a type you can never create
values for? If you think all the way back to Chapter 2, we had some code that
looked like this, reproduced here in Listing 19-33:

```
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

Listing 19-33: A `match` with an arm that ends in `continue`

At the time, we skipped over some details in this code. In Chapter 6, we
learned that `match` arms must return the same type. This doesn’t work:

```
let guess = match guess.trim().parse()  {
    Ok(_) => 5,
    Err(_) => "hello",
}
```

What would the type of `guess` be here? It’d have to be both an integer and a
string, and Rust requires that `guess` can only have one type. So what does
`continue` return? Why are we allowed to return a `u32` from one arm in Listing
19-33 and have another arm that ends with `continue`?

As you may have guessed, `continue` has a value of `!`. That is, when Rust goes
to compute the type of `guess`, it looks at both of the match arms. The former
has a value of `u32`, and the latter has a value of `!`. Since `!` can never
have a value, Rust is okay with this, and decides that the type of `guess` is
`u32`. The formal way of describing this behavior of `!` is that the never type
unifies with all other types. We’re allowed to end this `match` arm with
`continue` because `continue` doesn’t actually return a value; it instead moves
control back to the top of the loop, so in the `Err` case, we never actually
assign a value to `guess`.

Another use of the never type is `panic!`. Remember the `unwrap` function that
we call on `Option<T>` values to produce a value or panic? Here’s its
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
gets to the `break`.

### Dynamically Sized Types & `Sized`

Because Rust needs to know things like memory layout, there’s a particular
corner of its type system that can be confusing, and that’s the concept of
*dynamically sized types*. Sometimes referred to as ‘DSTs’ or ‘unsized types’,
these types let us talk about types whose size we can only know at runtime.

Let’s dig into the details of a dynamically sized type that we’ve been using
this whole book: `str`. That’s right, not `&str`, but `str` on its own. `str`
is a DST; we can’t know how long the string is until runtime. Since we can’t
know that, we can’t create a variable of type `str`, nor can we take an
argument of type `str`. Consider this code, which does not work:

```
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

These two `str` values would need to have the exact same memory layout, but
they have different lengths: `s1` needs 12 bytes of storage, and `s2` needs 15.
This is why it’s not possible to create a variable holding a dynamically sized
type.

So what to do? Well, you already know the answer in this case: the types of
`s1` and `s2` are `&str` rather than `str`. If you think back to Chapter 4, we
said this about `&str`:

> ... it’s a reference to an internal position in the String and the number of
> elements that it refers to.

So while a `&T` is a single value that stores the memory address of where the
`T` is located, a `&str` is *two* values: the address of the `str` and how long
it is. As such, a `&str` has a size we can know at compile time: it’s two times
the size of a `usize` in length. That is, we always know the size of a `&str`,
no matter how long the string it refers to is. This is the general way in which
dynamically sized types are used in Rust; they have an extra bit of metadata
that stores the size of the dynamic information. This leads us to the golden
rule of dynamically sized types: we must always put values of dynamically sized
types behind a pointer of some kind.

<!-- Note for Carol: `Rc<str>` is only in an accepted RFC right now, check on
its progress and pull this out if it's not going to be stable by Oct -->

While we’ve talked a lot about `&str`, we can combine `str` with all kinds of
pointers: `Box<str>`, for example, or `Rc<str>`. In fact, you’ve already seen
this before, but with a different dynamically sized type: traits. Every trait
is a dynamically sized type we can refer to by using the name of the trait. In
Chapter 17, we mentioned that in order to use traits as trait objects, we have
to put them behind a pointer like `&Trait` or `Box<Trait>` (`Rc<Trait>` would
work too). Traits being dynamically sized is the reason we have to do that!

#### The `Sized` Trait

<!-- If we end up keeping the section on object safety in ch 17, we should add
a back reference here. /Carol -->

To work with DSTs, Rust has a trait that determines if a type’s size is known
at compile time or not, which is `Sized`. This trait is automatically
implemented for everything the compiler knows the size of at compile time. In
addition, Rust implicitly adds a bound on `Sized` to every generic function.
That is, a generic function definition like this:

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

Finally, let’s discuss some advanced features having to do with functions and
closures: function pointers, diverging functions, and returning closures.

### Function pointers

We’ve talked about how to pass closures to functions, but you can pass regular
functions to functions too! Functions coerce to the type `fn`, with a lower
case ‘f’ not to be confused with the `Fn` closure trait. `fn` is called a
*function pointer*. The syntax for specifying that a parameter is a function
pointer is similar to that of closures, as shown in Listing 19-34:

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
parameter type directly rather than declaring a generic type parameter with one
of the `Fn` traits as a trait bound.

Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and
`FnOnce`), so we can always pass a function pointer as an argument when calling
a function that expects a closure. Prefer to write functions using a generic
type and one of the closure traits, so that your functions can accept either
functions or closures. An example of a case where you’d only want to accept
`fn` is when interfacing with external code that doesn’t have closures: C
functions can accept functions as arguments, but C doesn’t have closures.

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

Some people prefer this style, some people prefer the closure. They end up
with the same code, so use whichever feels more clear to you.

### Returning Closures

Because closures are represented by traits, returning closures is a little
tricky; we can’t do it directly. In most cases where we may want to return a
trait, we can instead use the concrete type that implements the trait of what
we’re returning as the return value of the function. We can’t do that with
closures, though. They don’t have a concrete type that’s returnable; we’re not
allowed to use the function pointer `fn` as a return type, for example.

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

The `Sized` trait again! Rust doesn’t know how much space it’ll need to store the
closure. We saw a solution to this in the previous section, though: we can use
a trait object:

```
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

For more about trait objects, refer back to Chapter 18.

## Summary

Whew! Now we’ve gone over features of Rust that aren’t used very often, but are
available if you need them. We’ve introduced a lot of complex topics so that
when you encounter them in error message suggestions or when reading others’
code, you’ll at least have seen these concepts and syntax once before.

Now, let’s put everything we’ve learned throughout the book into practice with
one more project!
