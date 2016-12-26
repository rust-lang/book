## Unrecoverable Errors with `panic!`

Sometimes, bad things happen, and there's nothing that you can do about it. For
these cases, Rust has the `panic!` macro. When this macro executes, your
program will print a failure message, unwind and clean up the stack, and then
quit. The most common situation this occurs in is when a bug of some kind has
been detected and it's not clear to Rust how to handle the error.

<!-- PROD: START BOX -->

> #### Unwinding
> By default, when a `panic!` occurs, the program starts
> *unwinding*, which means Rust walks back up the stack and cleans up the data
> from each function it encounters, but this walking and cleanup is a lot of
> work. The alternative is to immediately `abort`, which ends the program
> without cleaning up. Memory that the program was using will then need to be
> cleaned up by the operating system. If in your program you need to make
> the resulting binary as small as possible, you can switch from unwinding to
> aborting on panic by adding `panic = 'abort'` to the appropriate `[profile]`
> sections in your `Cargo.toml`.
<!-- Which is the appropriate profile section, will that be obvious? Maybe we
could give a screenshot? -->

<!-- PROD: END BOX -->

Let's try calling `panic!()` with a simple program:

```rust,should_panic
fn main() {
    panic!("crash and burn");
}
```

If you run it, you'll see something like this:

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished debug [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

There are three lines of errors here. The first line shows our panic
message and the place in our source code where the panic occurred:
`src/main.rs:2` indicates that it's the second like of our *main.rs* file.

<!-- Can you clarify which three lines are the error, the last three? -->

But that only shows us the exact line that called `panic!`. That's not always
useful.
<!-- why isn't that always useful, you mean because it doesn't specify which
part of that line causes the panic? What's the alternative? I'm not entirely
sure what point we're making? -->

### Heading

<!-- I may be wrong, but this reads like a slightly different topic, can you
suggest a heading? Otherwise, perhaps just make it more clear how they connect?
Is this next section all just about tracing the error? It's not quite clear -->

Let's look at another example to see what it's like when a `panic!` call comes
from a bug in our code instead of from calling the macro directly:

```rust,should_panic
fn main() {
    let v = vec![1, 2, 3];

    v[100];
}
```

We're attempting to access the hundredth element of our vector, but it only has
three elements. In this situation, Rust will panic. Using `[]` is supposed to
return an element, but if you pass an invalid index, there's no element that
Rust could return here that would be correct.

Other languages like C will attempt to give you exactly what you asked for in
this situation, even though it isn't what you want: you'll get whatever is at
the location in memory that would correspond to that element in the vector,
even though the memory doesn't belong to the vector. This is called a *buffer
overread*, and can lead to security vulnerabilities if an attacker can
manipulate the index in such a way as to read data they shouldn't be allowed to
that is stored after the array.

In order to protect your program from this sort of vulnerability, if you try to
read an element at an index that doesn't exist, Rust will stop execution and
refuse to continue. Let's try it and see:

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished debug [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', ../src/libcollections/vec.rs:1265
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

This points at a file we didn't write, `../src/libcollections/vec.rs`. That's
the implementation of `Vec<T>` in the standard library.

<!-- I'm not sure what Vec<T> is doing here, can you go over that a bit more?
Also, are there other types of panic errors that will automatically be caught
by Rust other than an index out of bounds type error? Can we give a general
rule on what causes panics? -->

While it's easy to see in this short program where the error was, it would be
helpful if Rust could tell us what line in our program caused the error.

<!-- Hm, earlier we seemed to say that pointing to the line in the program was
not very useful -->

The next `note` line tells us that we can set the `RUST_BACKTRACE` environment
variable to get a backtrace of exactly what happened to cause the error. Let's
try that. Listing 9-1 shows the output:

<figure>

```bash
$ RUST_BACKTRACE=1 cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', ../src/libcollections/vec.rs:1265
stack backtrace:
   1:     0x560956150ae9 -
std::sys::backtrace::tracing::imp::write::h482d45d91246faa2
   2:     0x56095615345c -
std::panicking::default_hook::_{{closure}}::h89158f66286b674e
   3:     0x56095615291e - std::panicking::default_hook::h9e30d428ee3b0c43
   4:     0x560956152f88 -
std::panicking::rust_panic_with_hook::h2224f33fb7bf2f4c
   5:     0x560956152e22 - std::panicking::begin_panic::hcb11a4dc6d779ae5
   6:     0x560956152d50 - std::panicking::begin_panic_fmt::h310416c62f3935b3
   7:     0x560956152cd1 - rust_begin_unwind
   8:     0x560956188a2f - core::panicking::panic_fmt::hc5789f4e80194729
   9:     0x5609561889d3 -
core::panicking::panic_bounds_check::hb2d969c3cc11ed08
  10:     0x56095614c075 - _<collections..vec..Vec<T> as
core..ops..Index<usize>>::index::hb9f10d3dadbe8101
                        at ../src/libcollections/vec.rs:1265
  11:     0x56095614c134 - panic::main::h2d7d3751fb8705e2
                        at /projects/panic/src/main.rs:4
  12:     0x56095615af46 - __rust_maybe_catch_panic
  13:     0x560956152082 - std::rt::lang_start::h352a66f5026f54bd
  14:     0x56095614c1b3 - main
  15:     0x7f75b88ed72f - __libc_start_main
  16:     0x56095614b3c8 - _start
  17:                0x0 - <unknown>
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

<figcaption>

Listing 9-1: The backtrace generated by a call to `panic!` displayed when
the environment variable `RUST_BACKTRACE` is set

</figcaption>
</figure>

That's a lot of output! Line 11 of the backtrace points to the line in our
project causing the problem: `src/main.rs`, line four. The key to reading the
backtrace is to start from the top and read until you see files you wrote:
that's where the problem originated.

<!-- So is the rest of it code embedded in Rust? -->

If we don't want our program to panic, this line is where we would start
investigating in order to figure out how we got to this location with values
that caused the panic.

<!--Are we going to show them how to fix the panic, or is this just something
that's particular to each case?-->

We'll come back to `panic!` and when we should and should not use these methods
later in the chapter. Next, we'll now look at how to recover from an error with
`Result`.
