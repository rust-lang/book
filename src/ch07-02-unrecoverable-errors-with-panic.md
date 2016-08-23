# Unrecoverable errors with panic!

Sometimes, bad things happen, and there's nothing that you can do about it. For
these cases, Rust has a macro, `panic!`. When this macro executes, your program
will terminate execution, printing a failure message and then quitting. Try
this program:

```rust,should_panic
fn main() {
    panic!("crash and burn");
}
```

If you run it, you'll see something like this:

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///home/steve/tmp/panic)
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
useful. Let's modify our example slightly:

```rust,should_panic
fn main() {
    let v = vec![1, 2, 3];

    v[100];
}
```

We attempt to access the hundredth element of our vector, but it only has three
elements. In this situation, Rust will panic. Let's try it:

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///home/steve/tmp/panic)
    Finished debug [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', ../src/libcollections/vec.rs:1265
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

This points at a file we didn't write, `../src/libcollections/vec.rs`, line 1265.
That's the implementation of `Vec<T>` in the standard library. While it's easy
to see in this short program where the error was, it would be nicer if we could
have Rust tell us what line in our program caused the error.

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
                        at /home/steve/tmp/panic/src/main.rs:4
  12:     0x56095615af46 - __rust_maybe_catch_panic
  13:     0x560956152082 - std::rt::lang_start::h352a66f5026f54bd
  14:     0x56095614c1b3 - main
  15:     0x7f75b88ed72f - __libc_start_main
  16:     0x56095614b3c8 - _start
  17:                0x0 - <unknown>
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

That's a lot of output! Line `11` there has the line in our project:
`src/main.rs` line four. We've been looking at the error message, but Cargo
also told us something important about backtraces early on: `[unoptimized +
debuginfo]`. 'debuginfo' is what enables the file names to be shown here.
If we instead compile with `--release`:

```bash
$ RUST_BACKTRACE=1 cargo run --release
   Compiling panic v0.1.0 (file:///home/steve/tmp/panic)
    Finished release [optimized] target(s) in 0.28 secs
     Running `target/release/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', ../src/libcollections/vec.rs:1265
stack backtrace:
   1:     0x565238fd0e79 -
std::sys::backtrace::tracing::imp::write::h482d45d91246faa2
   2:     0x565238fd37ec -
std::panicking::default_hook::_{{closure}}::h89158f66286b674e
   3:     0x565238fd2cae - std::panicking::default_hook::h9e30d428ee3b0c43
   4:     0x565238fd3318 -
std::panicking::rust_panic_with_hook::h2224f33fb7bf2f4c
   5:     0x565238fd31b2 - std::panicking::begin_panic::hcb11a4dc6d779ae5
   6:     0x565238fd30e0 - std::panicking::begin_panic_fmt::h310416c62f3935b3
   7:     0x565238fd3061 - rust_begin_unwind
   8:     0x565239008dbf - core::panicking::panic_fmt::hc5789f4e80194729
   9:     0x565239008d63 -
core::panicking::panic_bounds_check::hb2d969c3cc11ed08
  10:     0x565238fcc526 - panic::main::h2d7d3751fb8705e2
  11:     0x565238fdb2d6 - __rust_maybe_catch_panic
  12:     0x565238fd2412 - std::rt::lang_start::h352a66f5026f54bd
  13:     0x7f36aad6372f - __libc_start_main
  14:     0x565238fcc408 - _start
  15:                0x0 - <unknown>
error: Process didn't exit successfully: `target/release/panic` (exit code:
101)
```

Now it just says 'optimized', and we don't have the file names any more. These
settings are only the default; you can include debuginfo in a release build,
or exclude it from a debug build, by configuring Cargo. See its documentation
for more details: http://doc.crates.io/manifest.html#the-profile-sections

So why does Rust panic here? In this case, using `[]` is supposed to return
a number. But if you pass it an invalid index, there's no number Rust could
return here, it would be wrong. So the only thing that we can do is terminate
the program.
