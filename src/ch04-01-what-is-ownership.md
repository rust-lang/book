## What Is Ownership?

Ownership is a discipline for ensuring the **safety** of Rust programs. To understand ownership, we first need to understand what makes a Rust program safe (or unsafe).

### Safety is the Absence of Undefined Behavior

To start with an example, here is a program that safe to execute:

```rust
fn read(y: bool) {}

fn main() {
    let x = true;
    read(x);
}
```

We can make this program unsafe to execute by moving the `read` before the definition of `x`:

```rust,ignore,does_not_compile
fn read(y: bool) {}

fn main() {
    read(x); // oh no! x isn't defined!
    let x = true;
}
```

This second program is unsafe because `read(x)` expects `x` to have a value of type `bool`, but `x` doesn't have a value yet.

When programs are executed by an interpreter, reading `x` before it's defined would usually raise an exception such as Python's [`NameError`](https://docs.python.org/3/library/exceptions.html#NameError). But these safeguards come at a cost: each read of a variable must check whether that variable is defined. Rust's goal is to compile programs into efficient binaries that require as few runtime checks as possible. Therefore Rust does not check *at runtime* whether a variable is defined before it is used. 

So what would happen if Rust allowed the unsafe program to compile? Let's first consider how the safe program compiles and executes. On a computer with a processor using an [x86](https://en.wikipedia.org/wiki/X86) architecture, Rust generates the following assembly code for the `main` function in the safe program ([see the full assembly code here](https://rust.godbolt.org/z/TbqnTaK3j)):

```x86asm
main:
    ; ...
    mov     edi, 1
    call    read
    ; ...
```

> _Note_: if you aren't familiar with assembly code, that's ok! This section contains a few examples of assembly just to show you how Rust actually works under the hood. You don't generally need to know assembly to understand Rust.

This assembly code will:

- Move the number 1, representing `true`, into a "register" (a kind of assembly variable) called `edi`.
- Call the `read` function, which expects its first argument `y` to be in the `edi` register.

If the second function was allowed to compile, its assembly might look like this:

```x86asm
main:
    ; ...
    call    read
    mov     edi, 1    ; mov is after call
    ; ...
```

This program is unsafe because `read` will expect `edi` to be a boolean, which is either the number `0` or `1`. But `edi` could be anything: `2`, `100`, `0x1337BEEF`. When `read` wants to use its argument `y` for any purpose, it will immediately cause _**UNDEFINED BEHAVIOR!**_

Rust doesn't specify what happens if you try to do `if y { .. }` when `y` isn't `true` or `false`. That *behavior*, i.e. what happens after executing the instruction, is *undefined*. Something will happen, for example:

- The code executes without crashing, and no one notices a problem.
- The code immediately crashes due to a [segmentation fault](https://en.wikipedia.org/wiki/Segmentation_fault) or another kind of operating system error.
- The code executes without crashing, until a malicious actor creates the right input to delete your production database, overwrite your backups, and steal your lunch money.

To avoid these kinds of outcomes, Rust uses compile-time checks to ensure that variables are defined before they are used. If you actually try to compile the unsafe program, you will get this error from the compiler:

```
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:4:10
  |
4 |     read(x); // oh no! x isn't defined!
  |          ^ not found in this scope
```

**A foundational goal of Rust is to ensure your programs never have undefined behavior.** That is meaning of "safety." A secondary goal is to prevent undefined behavior at _compile-time_ instead of _run-time_. This goal has two motivations:

1. Catching bugs at compile-time means not dealing with those bugs in production, improving the reliability of your software.
2. Catching bugs at compile-time means fewer checks to catch bugs at runtime, improving the performance of your software.

Rust cannot prevent all bugs. If your application exposes a public and unauthenticated `/delete-production-database` endpoint, then a malicious actor doesn't need to exploit your suspicious if-statement. But Rust's protections are still likely to make your programs safer versus using a language with fewer protections, e.g. as found by [Google's Android team](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html).

### Ownership as a Discipline for Memory Safety

Since safety is the absence of undefined behavior, and since ownership is about safety, then we need to understand ownership in terms of the undefined behaviors it prevents. The Rust Reference maintains a large list of ["Behavior considered undefined"](https://doc.rust-lang.org/reference/behavior-considered-undefined.html), but for now we will focus on one category: operations on memory.

Memory is the space where data is stored during the execution of a program. There are many ways to think about memory:

- If you are unfamiliar with systems programming, you might think of memory at a high level like "memory is the RAM in my computer" or "memory is the thing that runs out if I load too much data".
- If you are familiar with systems programming, you might think of memory at a low level like "memory is an array of bytes" or "memory is the pointers I get back from `malloc`".

Both of these memory models are _valid_, but they are not _useful_ ways to think about how Rust works. The high-level model is too abstract to explain how Rust works: you will need to understand the concept of a pointer, for instance. The low-level model is too concrete to explain how Rust works: Rust does not allow you to (safely) interpret memory as an array of bytes, for instance.

Rust provides a particular way to think about memory, and ownership is a discipline for safely using memory within that way of thinking. The remainder of this chapter will explain the Rust model of memory.

### Variables Live in the Stack

Here's a program like the one you saw in Section 3.3 that defines a number `n` and calls a function `plus_one` on `n`. Beneath the program is a diagram that visualizes the state of the program at the three marked points.

```aquascope,interpreter,horizontal=true
fn main() {
    let n = 5;`[]`
    let y = plus_one(n);`[]`
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    `[]`x + 1
}
```

Variables live in **frames**. A frame is a mapping from variables to values within a single scope, such as a function. For example:

- The frame for `main` at location L1 holds `n = 5`.
- The frame for `plus_one` at L2 holds `x = 5`.
- The frame for `main` at location L3 holds `n = 5; y = 6`.

Frames are organized into a **stack** of currently-called-functions. For example, at L2 the top frame `main` points to the called function `plus_one`. After a function returns, Rust deallocates (or "frees") the function's frame. This sequence of frames is called a stack because the most recent frame added is always the next frame to be freed.

> _Note:_ this memory model does not fully describe how Rust actually works! As we saw earlier with the assembly code, the Rust compiler might put `n` or `x` into a register rather than a stack frame. But that distinction is an implementation detail that shouldn't change your understanding of safety in Rust, so we can focus on the simpler case of frame-only variables.

When an expression reads a variable, the variable's value is copied out of its frame. For example, if we run this program:

```aquascope,interpreter,horizontal=true
#fn main() {
let a = 5;`[]`
let mut b = a;`[]`
b += 1;`[]`
#}
```

The value of `a` is copied into `b`, and `a` is left unchanged, even after changing `b`.

### Boxes Live in the Heap

However, copying data can take up a lot of memory. For example, here's a slightly different program using an array with 1 million elements:

```aquascope,interpreter
#fn main() {
let a = [0; 1_000_000];`[]`
let b = a;`[]`
#}
```

Observe that copying `a` into `b` causes the `main` frame to contain 2 million elements. 

To transfer access to data without copying it, Rust uses the **heap**. The heap is a separate region of memory where data can live indefinitely, not tied to a specific frame. Rust provides a construct called [`Box`](https://doc.rust-lang.org/std/boxed/index.html) for putting data on the heap. For example, we can wrap the million-element array in `Box::new` like this:

```aquascope,interpreter
#fn main() {
let a = Box::new([0; 1_000_000]);`[]`
let b = a;`[]`
#}
```

Observe that now, there is only ever a single array. At L1, `a` contains a **pointer** (represented by dot with an arrow) to the array on the heap. The statement `let b = a` copies the pointer from `a` into `b`, but the pointed-to data is not copied.

<!-- 
You'll also notice that in the diagram, `a` has been crossed out. That's because of ownership. To understand why, we first need to discuss _memory management_. -->

### Rust Does Not Permit Manual Memory Management

Memory management is the process of allocating and deallocating memory. Stack frames are automatically managed by Rust: when a function is called, Rust allocates a stack frame for the called-function. When the call ends, Rust deallocates the stack frame.

As we saw above, heap data is allocated when calling `Box::new(..)`. But when is heap data deallocated? Imagine that Rust had a `free()` function that frees a heap allocation. This kind of "manual" memory management easily leads to bugs. For example, we could read a pointer after its data has been freed:

```aquascope,interpreter,shouldFail
#fn free<T>(_t: T) {}
#fn main() {
let b = Box::new([0; 100]);`[]`
free(b);`[]`
assert!(b[0] == 0);`[]`
#}
```

> *Note:* you may wonder how we are executing this Rust program that doesn't compile. We use special tools to simulate Rust as if the borrow checker were disabled, only for educational purposes.

Rust does not allow programs to manually deallocate memory, so as to avoid these kinds of errors.

### A Box's Owner Manages Deallocation

Instead, Rust _automatically_ frees a box's heap memory. Here is an _almost_ correct description of Rust's policy for freeing boxes:

> **Box deallocation principle (almost correct):** If a variable is bound to a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

For example, let's trace through a program that allocates and frees a box:

```aquascope,interpreter
fn main() {
    make_and_drop();`[]`
}

fn make_and_drop() {
    let a_box = Box::new(5);`[]`
}
```

At L1, `a_box` points to `5` on the heap. Once `make_and_drop` is finished, Rust deallocates the frame containing `a_box`, so Rust also deallocates the heap data in `a_box`, and the heap is empty at L2.

The box's heap memory has been successfully managed. But what if we abused this system? Returning to our earlier example, what happens when we bind two variables to a box?

```rust
#fn main() {
let a = Box::new([0; 1_000_000]);
let b = a;
#}
```

The boxed array has now been bound to both `a` and `b`. By our "almost correct" principle, Rust would try to free the box's heap memory *twice* on behalf of both variables &mdash; that's undefined behavior!

To avoid this kind of situation, we finally arrive at ownership. When `a` is bound to `Box::new([0; 1_000_000])`, we say that `a` **owns** the box. The statement `let b = a` **moves** ownership of the box from `a` to `b`. Given these concepts, Rust's policy for freeing boxes is more accurately described as:

> **Box deallocation principle (fully correct):** If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

<!-- SK: "In the example above, b is the owner of the box at L2." — the problem is we've just seen two examples, both with a `b`, and the `b` inside make-and-drop drops at L1, etc. Use different names. Also, explain both programs, not just the most recent one. -->

In the example above, `b` owns the box at L2. Therefore when the scope ends, Rust deallocates the box only once on behalf of `b`, not `a`.

### Collections Use Boxes

Boxes are used by Rust data structures like [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), [`String`](https://doc.rust-lang.org/std/string/struct.String.html), and [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) to hold a variable number of elements. For example, here's a program that creates, moves, and mutates a string:

```aquascope,interpreter
fn main() {
    let s1 = String::from("Hello");`[]`
    let s3 = add_suffix(s1);`[]`
    println!("{s3}");
}

fn add_suffix(mut s2: String) -> String {
    `[]`s2.push_str(" world");`[]`
    s2
}
```

This program is more involved, so make sure you follow each step:

1. At L1, the string "Hello" has been allocated on the heap. It is owned by `s1`.
2. At L2, the function `add_suffix(s1)` has been called. This moves ownership of the string from `s1` to `s2`. The string data is not copied, but the pointer to the data is copied.
3. At L3, the function `s2.push_str(" world")` resizes the string's heap allocation. This frees the original heap memory, creates a new allocation, and writes "Hello world" into the new location. `s1` now points to deallocated memory (represented as an `X`).
4. At L4, the frame for `add_suffix` is gone. This function returned `s2`, transferring ownership of the string to `s3`.

This program also illustrates a key safety principle for ownership. Imagine that `s1` were used in `main` after calling `add_suffix`, like this:

```aquascope,interpreter,shouldFail
fn main() {
    let s1 = String::from("Hello");
    let s3 = add_suffix(s1);
    println!("{s1} {s3}");`[]` // s1 is now used here
}

fn add_suffix(mut s2: String) -> String {
    s2.push_str(" world");
    s2
}

```

`s1` points to deallocated memory after calling `add_suffix`. Reading `s1` would therefore be a violation of memory safety, i.e. undefined behavior. 

Thankfully, Rust will refuse to compile this program, giving the following error:

```
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:4:16
  |
2 |     let s1 = String::from("Hello ");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s3 = add_suffix(s1);
  |                         -- value moved here
4 |     println!("{s1} {s3}"); // s1 is now used here
  |                ^^ value borrowed here after move
  |
```

This error identifies that `s1` is moved by `add_suffix`, and therefore it cannot be used afterward. More generally, the compiler will enforce this principle:

> **Moved heap data principle:** if a variable `x` moves ownership of heap data to another variable `y`, then `x` cannot be used after the move.

### Summary

Ownership is primarily a discipline of heap management:

- All heap data must be owned by exactly one variable.
- Rust deallocates heap data once its owner goes out of scope.
- Ownership can be transferred by moves, which happen on assignments and function calls.
- Heap data can only be accessed through its current owner, not a previous owner.

In this explanation, we have repeatedly emphasized not just _how_ Rust's safeguards work, but _why_ they avoid undefined behavior. When you get an error message from the Rust compiler, it's easy to get frustrated if you don't understand why Rust is complaining. Hopefully the models of memory and ownership we describe here will help you in the future with interpreting Rust's error messages and designing more Rustic APIs.