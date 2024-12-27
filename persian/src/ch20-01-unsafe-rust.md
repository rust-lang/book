## Unsafe Rust

All the code we’ve discussed so far has had Rust’s memory safety guarantees
enforced at compile time. However, Rust has a second language hidden inside it
that doesn’t enforce these memory safety guarantees: it’s called _unsafe Rust_
and works just like regular Rust, but gives us extra superpowers.

Unsafe Rust exists because, by nature, static analysis is conservative. When
the compiler tries to determine whether or not code upholds the guarantees,
it’s better for it to reject some valid programs than to accept some invalid
programs. Although the code _might_ be okay, if the Rust compiler doesn’t have
enough information to be confident, it will reject the code. In these cases,
you can use unsafe code to tell the compiler, “Trust me, I know what I’m
doing.” Be warned, however, that you use unsafe Rust at your own risk: if you
use unsafe code incorrectly, problems can occur due to memory unsafety, such as
null pointer dereferencing.

Another reason Rust has an unsafe alter ego is that the underlying computer
hardware is inherently unsafe. If Rust didn’t let you do unsafe operations, you
couldn’t do certain tasks. Rust needs to allow you to do low-level systems
programming, such as directly interacting with the operating system or even
writing your own operating system. Working with low-level systems programming
is one of the goals of the language. Let’s explore what we can do with unsafe
Rust and how to do it.

### Unsafe Superpowers

To switch to unsafe Rust, use the `unsafe` keyword and then start a new block
that holds the unsafe code. You can take five actions in unsafe Rust that you
can’t in safe Rust, which we call _unsafe superpowers_. Those superpowers
include the ability to:

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of a `union`

It’s important to understand that `unsafe` doesn’t turn off the borrow checker
or disable any other of Rust’s safety checks: if you use a reference in unsafe
code, it will still be checked. The `unsafe` keyword only gives you access to
these five features that are then not checked by the compiler for memory
safety. You’ll still get some degree of safety inside of an unsafe block.

In addition, `unsafe` does not mean the code inside the block is necessarily
dangerous or that it will definitely have memory safety problems: the intent is
that as the programmer, you’ll ensure the code inside an `unsafe` block will
access memory in a valid way.

People are fallible, and mistakes will happen, but by requiring these five
unsafe operations to be inside blocks annotated with `unsafe` you’ll know that
any errors related to memory safety must be within an `unsafe` block. Keep
`unsafe` blocks small; you’ll be thankful later when you investigate memory
bugs.

To isolate unsafe code as much as possible, it’s best to enclose unsafe code
within a safe abstraction and provide a safe API, which we’ll discuss later in
the chapter when we examine unsafe functions and methods. Parts of the standard
library are implemented as safe abstractions over unsafe code that has been
audited. Wrapping unsafe code in a safe abstraction prevents uses of `unsafe`
from leaking out into all the places that you or your users might want to use
the functionality implemented with `unsafe` code, because using a safe
abstraction is safe.

Let’s look at each of the five unsafe superpowers in turn. We’ll also look at
some abstractions that provide a safe interface to unsafe code.

### Dereferencing a Raw Pointer

In Chapter 4, in the [“Dangling References”][dangling-references]<!-- ignore
--> section, we mentioned that the compiler ensures references are always
valid. Unsafe Rust has two new types called _raw pointers_ that are similar to
references. As with references, raw pointers can be immutable or mutable and
are written as `*const T` and `*mut T`, respectively. The asterisk isn’t the
dereference operator; it’s part of the type name. In the context of raw
pointers, _immutable_ means that the pointer can’t be directly assigned to
after being dereferenced.

Different from references and smart pointers, raw pointers:

- Are allowed to ignore the borrowing rules by having both immutable and
  mutable pointers or multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup

By opting out of having Rust enforce these guarantees, you can give up
guaranteed safety in exchange for greater performance or the ability to
interface with another language or hardware where Rust’s guarantees don’t apply.

Listing 20-1 shows how to create an immutable and a mutable raw pointer.

<Listing number="20-1" caption="Creating raw pointers with the raw borrow operators">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

</Listing>

Notice that we don’t include the `unsafe` keyword in this code. We can create
raw pointers in safe code; we just can’t dereference raw pointers outside an
unsafe block, as you’ll see in a bit.

We’ve created raw pointers by using the raw borrow operators: `&raw const num`
creates a `*const i32` immutable raw pointer, and `&raw mut num` creates a `*mut
i32` mutable raw pointer. Because we created them directly from a local
variable, we know these particular raw pointers are valid, but we can’t make
that assumption about just any raw pointer.

To demonstrate this, next we’ll create a raw pointer whose validity we can’t be
so certain of, using `as` to cast a value instead of using the raw reference
operators. Listing 20-2 shows how to create a raw pointer to an arbitrary
location in memory. Trying to use arbitrary memory is undefined: there might be
data at that address or there might not, the compiler might optimize the code so
there is no memory access, or the program might error with a segmentation fault.
Usually, there is no good reason to write code like this, especially in cases
where you can use a raw borrow operator instead, but it is possible.

<Listing number="20-2" caption="Creating a raw pointer to an arbitrary memory address">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

</Listing>

Recall that we can create raw pointers in safe code, but we can’t _dereference_
raw pointers and read the data being pointed to. In Listing 20-3, we use the
dereference operator `*` on a raw pointer that requires an `unsafe` block.

<Listing number="20-3" caption="Dereferencing raw pointers within an `unsafe` block">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

</Listing>

Creating a pointer does no harm; it’s only when we try to access the value that
it points at that we might end up dealing with an invalid value.

Note also that in Listing 20-1 and 20-3, we created `*const i32` and `*mut i32`
raw pointers that both pointed to the same memory location, where `num` is
stored. If we instead tried to create an immutable and a mutable reference to
`num`, the code would not have compiled because Rust’s ownership rules don’t
allow a mutable reference at the same time as any immutable references. With
raw pointers, we can create a mutable pointer and an immutable pointer to the
same location and change data through the mutable pointer, potentially creating
a data race. Be careful!

With all of these dangers, why would you ever use raw pointers? One major use
case is when interfacing with C code, as you’ll see in the next section,
[“Calling an Unsafe Function or
Method.”](#calling-an-unsafe-function-or-method)<!-- ignore --> Another case is
when building up safe abstractions that the borrow checker doesn’t understand.
We’ll introduce unsafe functions and then look at an example of a safe
abstraction that uses unsafe code.

### Calling an Unsafe Function or Method

The second type of operation you can perform in an unsafe block is calling
unsafe functions. Unsafe functions and methods look exactly like regular
functions and methods, but they have an extra `unsafe` before the rest of the
definition. The `unsafe` keyword in this context indicates the function has
requirements we need to uphold when we call this function, because Rust can’t
guarantee we’ve met these requirements. By calling an unsafe function within an
`unsafe` block, we’re saying that we’ve read this function’s documentation and
take responsibility for upholding the function’s contracts.

Here is an unsafe function named `dangerous` that doesn’t do anything in its
body:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

We must call the `dangerous` function within a separate `unsafe` block. If we
try to call `dangerous` without the `unsafe` block, we’ll get an error:

```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

With the `unsafe` block, we’re asserting to Rust that we’ve read the function’s
documentation, we understand how to use it properly, and we’ve verified that
we’re fulfilling the contract of the function.

> Note: In earlier versions of Rust, the body of an unsafe function was treated
> as an `unsafe` block, so you could perform any unsafe operation within the
> body of an `unsafe` function. In later versions of Rust, the compiler will
> warn you that you need to use an `unsafe` block to perform unsafe operations
> in the body of an unsafe function. This is because Rust now distinguishes
> between `unsafe fn`, which defines what you need to do to call the function
> safely, and an `unsafe` block, where you actually uphold that “contract” the
> function establishes.

#### Creating a Safe Abstraction over Unsafe Code

Just because a function contains unsafe code doesn’t mean we need to mark the
entire function as unsafe. In fact, wrapping unsafe code in a safe function is
a common abstraction. As an example, let’s study the `split_at_mut` function
from the standard library, which requires some unsafe code. We’ll explore how
we might implement it. This safe method is defined on mutable slices: it takes
one slice and makes it two by splitting the slice at the index given as an
argument. Listing 20-4 shows how to use `split_at_mut`.

<Listing number="20-4" caption="Using the safe `split_at_mut` function">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

</Listing>

We can’t implement this function using only safe Rust. An attempt might look
something like Listing 20-5, which won’t compile. For simplicity, we’ll
implement `split_at_mut` as a function rather than a method and only for slices
of `i32` values rather than for a generic type `T`.

<Listing number="20-5" caption="An attempted implementation of `split_at_mut` using only safe Rust">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

</Listing>

This function first gets the total length of the slice. Then it asserts that
the index given as a parameter is within the slice by checking whether it’s
less than or equal to the length. The assertion means that if we pass an index
that is greater than the length to split the slice at, the function will panic
before it attempts to use that index.

Then we return two mutable slices in a tuple: one from the start of the
original slice to the `mid` index and another from `mid` to the end of the
slice.

When we try to compile the code in Listing 20-5, we’ll get an error.

```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

Rust’s borrow checker can’t understand that we’re borrowing different parts of
the slice; it only knows that we’re borrowing from the same slice twice.
Borrowing different parts of a slice is fundamentally okay because the two
slices aren’t overlapping, but Rust isn’t smart enough to know this. When we
know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.

Listing 20-6 shows how to use an `unsafe` block, a raw pointer, and some calls
to unsafe functions to make the implementation of `split_at_mut` work.

<Listing number="20-6" caption="Using unsafe code in the implementation of the `split_at_mut` function">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

</Listing>

Recall from [“The Slice Type”][the-slice-type]<!-- ignore --> section in
Chapter 4 that slices are a pointer to some data and the length of the slice.
We use the `len` method to get the length of a slice and the `as_mut_ptr`
method to access the raw pointer of a slice. In this case, because we have a
mutable slice to `i32` values, `as_mut_ptr` returns a raw pointer with the type
`*mut i32`, which we’ve stored in the variable `ptr`.

We keep the assertion that the `mid` index is within the slice. Then we get to
the unsafe code: the `slice::from_raw_parts_mut` function takes a raw pointer
and a length, and it creates a slice. We use this function to create a slice
that starts from `ptr` and is `mid` items long. Then we call the `add`
method on `ptr` with `mid` as an argument to get a raw pointer that starts at
`mid`, and we create a slice using that pointer and the remaining number of
items after `mid` as the length.

The function `slice::from_raw_parts_mut` is unsafe because it takes a raw
pointer and must trust that this pointer is valid. The `add` method on raw
pointers is also unsafe, because it must trust that the offset location is also
a valid pointer. Therefore, we had to put an `unsafe` block around our calls to
`slice::from_raw_parts_mut` and `add` so we could call them. By looking at
the code and by adding the assertion that `mid` must be less than or equal to
`len`, we can tell that all the raw pointers used within the `unsafe` block
will be valid pointers to data within the slice. This is an acceptable and
appropriate use of `unsafe`.

Note that we don’t need to mark the resulting `split_at_mut` function as
`unsafe`, and we can call this function from safe Rust. We’ve created a safe
abstraction to the unsafe code with an implementation of the function that uses
`unsafe` code in a safe way, because it creates only valid pointers from the
data this function has access to.

In contrast, the use of `slice::from_raw_parts_mut` in Listing 20-7 would
likely crash when the slice is used. This code takes an arbitrary memory
location and creates a slice 10,000 items long.

<Listing number="20-7" caption="Creating a slice from an arbitrary memory location">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

</Listing>

We don’t own the memory at this arbitrary location, and there is no guarantee
that the slice this code creates contains valid `i32` values. Attempting to use
`values` as though it’s a valid slice results in undefined behavior.

#### Using `extern` Functions to Call External Code

Sometimes, your Rust code might need to interact with code written in another
language. For this, Rust has the keyword `extern` that facilitates the creation
and use of a _Foreign Function Interface (FFI)_. An FFI is a way for a
programming language to define functions and enable a different (foreign)
programming language to call those functions.

Listing 20-8 demonstrates how to set up an integration with the `abs` function
from the C standard library. Functions declared within `extern` blocks are
usually unsafe to call from Rust code, so they must also be marked `unsafe`. The
reason is that other languages don’t enforce Rust’s rules and guarantees, and
Rust can’t check them, so responsibility falls on the programmer to ensure
safety.

<Listing number="20-8" file-name="src/main.rs" caption="Declaring and calling an `extern` function defined in another language">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

</Listing>

Within the `unsafe extern "C"` block, we list the names and signatures of
external functions from another language we want to call. The `"C"` part defines
which _application binary interface (ABI)_ the external function uses: the ABI
defines how to call the function at the assembly level. The `"C"` ABI is the
most common and follows the C programming language’s ABI.

This particular function does not have any memory safety considerations, though.
In fact, we know that any call to `abs` will always be safe for any `i32`, so we
can use the `safe` keyword to say that this specific function is safe to call
even though it is in an `unsafe extern` block. Once we make that change, calling
it no longer requires an `unsafe` block, as shown in Listing 20-9.

<Listing number="20-9" file-name="src/main.rs" caption="Explicitly marking a function as `safe` within an `unsafe extern` block and calling it safely">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

</Listing>

Marking a function as `safe` does not inherently make it safe! Instead, it is
like a promise you are making to Rust that it _is_ safe. It is still your
responsibility to make sure that promise is kept!

> #### Calling Rust Functions from Other Languages
>
> We can also use `extern` to create an interface that allows other languages to
> call Rust functions. Instead of creating a whole `extern` block, we add the
> `extern` keyword and specify the ABI to use just before the `fn` keyword for
> the relevant function. We also need to add a `#[unsafe(no_mangle)]` annotation
> to tell the Rust compiler not to mangle the name of this function. _Mangling_
> is when a compiler changes the name we’ve given a function to a different name
> that contains more information for other parts of the compilation process to
> consume but is less human readable. Every programming language compiler
> mangles names slightly differently, so for a Rust function to be nameable by
> other languages, we must disable the Rust compiler’s name mangling. This is
> unsafe because there might be name collisions across libraries without the
> built-in mangling, so it is our responsibility to make sure the name we have
> exported is safe to export without mangling.
>
> In the following example, we make the `call_from_c` function accessible from
> C code, after it’s compiled to a shared library and linked from C:
>
> ```rust
> #[unsafe(no_mangle)]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```
>
> This usage of `extern` does not require `unsafe`.

### Accessing or Modifying a Mutable Static Variable

In this book, we’ve not yet talked about _global variables_, which Rust does
support but can be problematic with Rust’s ownership rules. If two threads are
accessing the same mutable global variable, it can cause a data race.

In Rust, global variables are called _static_ variables. Listing 20-10 shows an
example declaration and use of a static variable with a string slice as a
value.

<Listing number="20-10" file-name="src/main.rs" caption="Defining and using an immutable static variable">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-10/src/main.rs}}
```

</Listing>

Static variables are similar to constants, which we discussed in the
[“Constants”][differences-between-variables-and-constants]<!-- ignore --> section
in Chapter 3. The names of static variables are in `SCREAMING_SNAKE_CASE` by
convention. Static variables can only store references with the `'static`
lifetime, which means the Rust compiler can figure out the lifetime and we
aren’t required to annotate it explicitly. Accessing an immutable static
variable is safe.

A subtle difference between constants and immutable static variables is that
values in a static variable have a fixed address in memory. Using the value
will always access the same data. Constants, on the other hand, are allowed to
duplicate their data whenever they’re used. Another difference is that static
variables can be mutable. Accessing and modifying mutable static variables is
_unsafe_. Listing 20-11 shows how to declare, access, and modify a mutable
static variable named `COUNTER`.

<Listing number="20-11" file-name="src/main.rs" caption="Reading from or writing to a mutable static variable is unsafe">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-11/src/main.rs}}
```

</Listing>

As with regular variables, we specify mutability using the `mut` keyword. Any
code that reads or writes from `COUNTER` must be within an `unsafe` block. This
code compiles and prints `COUNTER: 3` as we would expect because it’s single
threaded. Having multiple threads access `COUNTER` would likely result in data
races, so it is undefined behavior. Therefore, we need to mark the entire
function as `unsafe`, and document the safety limitation, so anyone calling the
function knows what they are and are not allowed to do safely.

Whenever we write an unsafe function, it is idiomatic to write a comment
starting with `SAFETY` and explaining what the caller needs to do to call the
function safely. Likewise, whenever we perform an unsafe operation, it is
idiomatic to write a comment starting with `SAFETY` to explain how the safety
rules are upheld.

With mutable data that is globally accessible, it’s difficult to ensure there
are no data races, which is why Rust considers mutable static variables to be
unsafe. Where possible, it’s preferable to use the concurrency techniques and
thread-safe smart pointers we discussed in Chapter 16 so the compiler checks
that data accessed from different threads is done safely.

### Implementing an Unsafe Trait

We can use `unsafe` to implement an unsafe trait. A trait is unsafe when at
least one of its methods has some invariant that the compiler can’t verify. We
declare that a trait is `unsafe` by adding the `unsafe` keyword before `trait`
and marking the implementation of the trait as `unsafe` too, as shown in
Listing 20-12.

<Listing number="20-12" caption="Defining and implementing an unsafe trait">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-12/src/main.rs}}
```

</Listing>

By using `unsafe impl`, we’re promising that we’ll uphold the invariants that
the compiler can’t verify.

As an example, recall the `Sync` and `Send` marker traits we discussed in the
[“Extensible Concurrency with the `Sync` and `Send`
Traits”][extensible-concurrency-with-the-sync-and-send-traits]<!-- ignore -->
section in Chapter 16: the compiler implements these traits automatically if
our types are composed entirely of `Send` and `Sync` types. If we implement a
type that contains a type that is not `Send` or `Sync`, such as raw pointers,
and we want to mark that type as `Send` or `Sync`, we must use `unsafe`. Rust
can’t verify that our type upholds the guarantees that it can be safely sent
across threads or accessed from multiple threads; therefore, we need to do
those checks manually and indicate as such with `unsafe`.

### Accessing Fields of a Union

The final action that works only with `unsafe` is accessing fields of a
_union_. A `union` is similar to a `struct`, but only one declared field is
used in a particular instance at one time. Unions are primarily used to
interface with unions in C code. Accessing union fields is unsafe because Rust
can’t guarantee the type of the data currently being stored in the union
instance. You can learn more about unions in [the Rust Reference][reference].

### Using Miri to check unsafe code

When writing unsafe code, you might want to check that what you have written
actually is safe and correct. One of the best ways to do that is to use
[Miri][miri], an official Rust tool for detecting undefined behavior. Whereas
the borrow checker is a _static_ tool which works at compile time, Miri is a
_dynamic_ tool which works at runtime. It checks your code by running your
program, or its test suite, and detecting when you violate the rules it
understands about how Rust should work.

Using Miri requires a nightly build of Rust (which we talk about more in
[Appendix G: How Rust is Made and “Nightly Rust”][nightly]). You can install
both a nightly version of Rust and the Miri tool by typing `rustup +nightly
component add miri`. This does not change what version of Rust your project
uses; it only adds the tool to your system so you can use it when you want to.
You can run Miri on a project by typing `cargo +nightly miri run` or `cargo
+nightly miri test`.

For an example of how helpful this can be, consider what happens when we run it
against Listing 20-11:

```console
{{#include ../listings/ch20-advanced-features/listing-20-11/output.txt}}
```

It helpfully and correctly notices that we have shared references to mutable
data, and warns about it. In this case, it does not tell us how to fix the
problem, but it means that we know there is a possible issue and can think about
how to make sure it is safe. In other cases, it can actually tell us that some
code is _sure_ to be wrong and make recommendations about how to fix it.

Miri doesn’t catch _everything_ you might get wrong when writing unsafe code.
For one thing, since it is a dynamic check, it only catches problems with code
that actually gets run. That means you will need to use it in conjunction with
good testing techniques to increase your confidence about the unsafe code you
have written. For another thing, it does not cover every possible way your code
can be unsound. If Miri _does_ catch a problem, you know there’s a bug, but just
because Miri _doesn’t_ catch a bug doesn’t mean there isn’t a problem. Miri can
catch a lot, though. Try running it on the other examples of unsafe code in this
chapter and see what it says!

### When to Use Unsafe Code

Using `unsafe` to take one of the five actions (superpowers) just discussed
isn’t wrong or even frowned upon. But it is trickier to get `unsafe` code
correct because the compiler can’t help uphold memory safety. When you have a
reason to use `unsafe` code, you can do so, and having the explicit `unsafe`
annotation makes it easier to track down the source of problems when they occur.
Whenever you write unsafe code, you can use Miri to help you be more confident
that the code you have written upholds Rust’s rules.

For a much deeper exploration of how to work effectively with unsafe Rust, read
Rust’s official guide to the subject, the [Rustonomicon][nomicon].

[dangling-references]: ch04-02-references-and-borrowing.html#dangling-references
[differences-between-variables-and-constants]: ch03-01-variables-and-mutability.html#constants
[extensible-concurrency-with-the-sync-and-send-traits]: ch16-04-extensible-concurrency-sync-and-send.html#extensible-concurrency-with-the-sync-and-send-traits
[the-slice-type]: ch04-03-slices.html#the-slice-type
[reference]: ../reference/items/unions.html
[miri]: https://github.com/rust-lang/miri
[nightly]: appendix-07-nightly-rust.html
[nomicon]: https://doc.rust-lang.org/nomicon/
