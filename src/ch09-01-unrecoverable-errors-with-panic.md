# Unrecoverable errors with panic!

Sometimes, bad things happen, and there's nothing that you can do about it. For
these cases, Rust has a macro, `panic!`. When this macro executes, your program
will print a failure message and then quit. The most common reason for this is
when a bug of some kind has been detected, and it's not clear how to handle the
error.

Let's try it out with a simple program:

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

There are three lines of error message here. The first line shows our panic
message and the place in our source code where the panic occurred:
`src/main.rs`, line two.

But that only shows us the exact line that called `panic!`. That's not always
useful. Let's modify our example slightly to see what it's like when a `panic!`
call comes from code we call instead of from our code directly:

```rust,should_panic
fn main() {
    let v = vec![1, 2, 3];

    v[100];
}
```

We attempt to access the hundredth element of our vector, but it only has three
elements. In this situation, Rust will panic. Using `[]` is supposed to return
an element. If you pass `[]` an invalid index, though, there's no element that
Rust could return here that would be correct. In this case, we've typed in a
literal index of `100`, so in theory, Rust could examine our code at compile
time and raise an error at that point. But if our program was different, say,
maybe reading the index from user input, it would not be possible to determine
at compile time if the index was in bounds.

Other languages like C will attempt to give you exactly what you asked for in
this situation, even though it isn't what you want: you'll get whatever is at
the location in memory that would correspond to that element in the vector,
even though the memory doesn't belong to the vector. This is called a *buffer
overread*, and can lead to security vulnerabilities if an attacker can
manipulate the index in such a way as to read data they shouldn't be allowed to
that is stored after the array.

In order to protect your program from this sort of vulnerability, if you try to
read an element at an index that doesn't exist, Rust will terminate the program.
Let's try it and see:

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

This points at a file we didn't write, `../src/libcollections/vec.rs`, line
1265. That's the implementation of `Vec<T>` in the standard library. While it's
easy to see in this short program where the error was, it would be nicer if we
could have Rust tell us what line in our program caused the error.

That's what the next line, the `note` is about. If we set the `RUST_BACKTRACE`
environment variable, we'll get a backtrace of exactly how the error happend.
Let's try it:

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

That's a lot of output! Line `11` there has the line in our project:
`src/main.rs` line four.
