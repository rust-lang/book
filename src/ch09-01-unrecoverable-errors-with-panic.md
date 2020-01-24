## Unrecoverable Errors with `panic!`

Sometimes, bad things happen in your code, and there’s nothing you can do about
it. In these cases, Rust has the `panic!` macro. When the `panic!` macro
executes, your program will print a failure message, unwind and clean up the
stack, and then quit. This most commonly occurs when a bug of some kind has
been detected and it’s not clear to the programmer how to handle the error.

> ### Unwinding the Stack or Aborting in Response to a Panic
>
> By default, when a panic occurs, the program starts *unwinding*, which
> means Rust walks back up the stack and cleans up the data from each function
> it encounters. But this walking back and cleanup is a lot of work. The
> alternative is to immediately *abort*, which ends the program without
> cleaning up. Memory that the program was using will then need to be cleaned
> up by the operating system. If in your project you need to make the resulting
> binary as small as possible, you can switch from unwinding to aborting upon a
> panic by adding `panic = 'abort'` to the appropriate `[profile]` sections in
> your *Cargo.toml* file. For example, if you want to abort on panic in release
> mode, add this:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Let’s try calling `panic!` in a simple program:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

When you run the program, you’ll see something like this:

```text
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

The call to `panic!` causes the error message contained in the last two lines.
The first line shows our panic message and the place in our source code where
the panic occurred: *src/main.rs:2:5* indicates that it’s the second line,
fifth character of our *src/main.rs* file.

In this case, the line indicated is part of our code, and if we go to that
line, we see the `panic!` macro call. In other cases, the `panic!` call might
be in code that our code calls, and the filename and line number reported by
the error message will be someone else’s code where the `panic!` macro is
called, not the line of our code that eventually led to the `panic!` call. We
can use the backtrace of the functions the `panic!` call came from to figure
out the part of our code that is causing the problem. We’ll discuss what a
backtrace is in more detail next.

### Using a `panic!` Backtrace

Let’s look at another example to see what it’s like when a `panic!` call comes
from a library because of a bug in our code instead of from our code calling
the macro directly. Listing 9-1 has some code that attempts to access an
element by index in a vector.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

<span class="caption">Listing 9-1: Attempting to access an element beyond the
end of a vector, which will cause a call to `panic!`</span>

Here, we’re attempting to access the 100th element of our vector (which is at
index 99 because indexing starts at zero), but it has only 3 elements. In this
situation, Rust will panic. Using `[]` is supposed to return an element, but if
you pass an invalid index, there’s no element that Rust could return here that
would be correct.

Other languages, like C, will attempt to give you exactly what you asked for in
this situation, even though it isn’t what you want: you’ll get whatever is at
the location in memory that would correspond to that element in the vector,
even though the memory doesn’t belong to the vector. This is called a *buffer
overread* and can lead to security vulnerabilities if an attacker is able to
manipulate the index in such a way as to read data they shouldn’t be allowed to
that is stored after the array.

To protect your program from this sort of vulnerability, if you try to read an
element at an index that doesn’t exist, Rust will stop execution and refuse to
continue. Let’s try it and see:

```text
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

This error points at a file we didn’t write, *libcore/slice/mod.rs*. That’s the
implementation of `slice` in the Rust source code. The code that gets run when
we use `[]` on our vector `v` is in *libcore/slice/mod.rs*, and that is where
the `panic!` is actually happening.

The next note line tells us that we can set the `RUST_BACKTRACE` environment
variable to get a backtrace of exactly what happened to cause the error. A
*backtrace* is a list of all the functions that have been called to get to this
point. Backtraces in Rust work as they do in other languages: the key to
reading the backtrace is to start from the top and read until you see files you
wrote. That’s the spot where the problem originated. The lines above the lines
mentioning your files are code that your code called; the lines below are code
that called your code. These lines might include core Rust code, standard
library code, or crates that you’re using. Let’s try getting a backtrace by
setting the `RUST_BACKTRACE` environment variable to any value except 0.
Listing 9-2 shows output similar to what you’ll see.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

```text
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libcore/slice/mod.rs:2715:10
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /Users/vsts/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /Users/vsts/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
   7: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:384
   8: rust_begin_unwind
             at src/libstd/panicking.rs:311
   9: std::panicking::begin_panic
  10: std::panicking::begin_panic
  11: <usize as core::slice::SliceIndex<[T]>>::index
             at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libcore/slice/mod.rs:2715
  12: core::slice::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libcore/slice/mod.rs:2566
  13: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/liballoc/vec.rs:1791
  14: panic::main
             at src/main.rs:4
  15: std::rt::lang_start::{{closure}}
             at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/rt.rs:64
  16: std::rt::lang_start_internal::{{closure}}
             at src/libstd/rt.rs:49
  17: std::panicking::try::do_call
             at src/libstd/panicking.rs:296
  18: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  19: std::panicking::try
             at src/libstd/panicking.rs:275
  20: std::panic::catch_unwind
             at src/libstd/panic.rs:394
  21: std::rt::lang_start_internal
             at src/libstd/rt.rs:48
  22: std::rt::lang_start
             at /rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/libstd/rt.rs:64
  23: panic::main
```

<span class="caption">Listing 9-2: The backtrace generated by a call to
`panic!` displayed when the environment variable `RUST_BACKTRACE` is set</span>

That’s a lot of output! The exact output you see might be different depending
on your operating system and Rust version. In order to get backtraces with this
information, debug symbols must be enabled. Debug symbols are enabled by
default when using `cargo build` or `cargo run` without the `--release` flag,
as we have here.

In the output in Listing 9-2, line 14 of the backtrace points to the line in
our project that’s causing the problem: line 4 of *src/main.rs*. If we don’t
want our program to panic, the location pointed to by the first line mentioning
a file we wrote is where we should start investigating. In Listing 9-1, where
we deliberately wrote code that would panic in order to demonstrate how to use
backtraces, the way to fix the panic is to not request an element at index 99
from a vector that only contains 3 items. When your code panics in the future,
you’ll need to figure out what action the code is taking with what values to
cause the panic and what the code should do instead.

We’ll come back to `panic!` and when we should and should not use `panic!` to
handle error conditions in the [“To `panic!` or Not to
`panic!`”][to-panic-or-not-to-panic]<!-- ignore --> section later in this
chapter. Next, we’ll look at how to recover from an error using `Result`.

[to-panic-or-not-to-panic]:
ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic
