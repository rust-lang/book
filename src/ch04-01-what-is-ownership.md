## What Is Ownership?

Ownership is a discipline for ensuring the **safety** of Rust programs. To understand ownership, we first need to understand what makes a Rust program safe (or unsafe).

### Safety is the Absence of Undefined Behavior

Let's start with an example. This program is safe to execute:

```rust
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x);
}
```

We can make this program unsafe to execute by moving the call to `read` before the definition of `x`:

```rust,ignore,does_not_compile
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    read(x); // oh no! x isn't defined!
    let x = true;
}
```

> *Note*: in this chapter, we will use many code examples that do not compile. Make sure to look for the question mark crab if you are not sure whether a program should compile or not.

This second program is unsafe because `read(x)` expects `x` to have a value of type `bool`, but `x` doesn't have a value yet.

When programs are executed by an interpreter, reading `x` before it's defined would usually raise an exception such as Python's [`NameError`] or Javascript's [`ReferenceError`]. But these safeguards come at a cost. Each time an interpreted program reads a variable, then the interpreter must check whether that variable is defined.

Rust's goal is to compile programs into efficient binaries that require as few runtime checks as possible. Therefore Rust does not check at *runtime* whether a variable is defined before being used. Instead, Rust checks at *compile-time*. If you try to compile the unsafe program, you will get this error:

```text
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:8:10
  |
8 |     read(x); // oh no! x isn't defined!
  |          ^ not found in this scope
```

You probably have the intuition that it's good for Rust to ensure that variables are defined before they are used. But why? To justify the rule, we have to ask: **what would happen if Rust allowed a rejected program to compile?**

Let's first consider how the safe program compiles and executes. On a computer with a processor using an [x86](https://en.wikipedia.org/wiki/X86) architecture, Rust generates the following assembly code for the `main` function in the safe program ([see the full assembly code here](https://rust.godbolt.org/z/xnT1fzsqv)):

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

If the unsafe function was allowed to compile, its assembly might look like this:

```x86asm
main:
    ; ...
    call    read
    mov     edi, 1    ; mov is after call
    ; ...
```

This program is unsafe because `read` will expect `edi` to be a boolean, which is either the number `0` or `1`. But `edi` could be anything: `2`, `100`, `0x1337BEEF`. When `read` wants to use its argument `y` for any purpose, it will immediately cause _**UNDEFINED BEHAVIOR!**_

Rust doesn't specify what happens if you try to run `if y { .. }` when `y` isn't `true` or `false`. That *behavior*, or what happens after executing the instruction, is *undefined*. Something will happen, for example:

- The code executes without crashing, and no one notices a problem.
- The code immediately crashes due to a [segmentation fault](https://en.wikipedia.org/wiki/Segmentation_fault) or another kind of operating system error.
- The code executes without crashing, until a malicious actor creates the right input to delete your production database, overwrite your backups, and steal your lunch money.

**A foundational goal of Rust is to ensure that your programs never have undefined behavior.** That is the meaning of "safety." Undefined behavior is especially dangerous for low-level programs with direct access to memory. About [70% of reported security vulnerabilities](https://msrc.microsoft.com/blog/2019/07/a-proactive-approach-to-more-secure-code/) in low-level systems are caused by memory corruption, which is one form of undefined behavior.

A secondary goal of Rust is to prevent undefined behavior at _compile-time_ instead of _run-time_. This goal has two motivations:

1. Catching bugs at compile-time means avoiding those bugs in production, improving the reliability of your software.
2. Catching bugs at compile-time means fewer runtime checks for those bugs, improving the performance of your software.

Rust cannot prevent all bugs. If an application exposes a public and unauthenticated `/delete-production-database` endpoint, then a malicious actor doesn't need a suspicious if-statement to delete the database. But Rust's protections are still likely to make programs safer versus using a language with fewer protections, e.g. as found by [Google's Android team](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html).

### Ownership as a Discipline for Memory Safety

Since safety is the absence of undefined behavior, and since ownership is about safety, then we need to understand ownership in terms of the undefined behaviors it prevents. The Rust Reference maintains a large list of ["Behavior considered undefined"](https://doc.rust-lang.org/reference/behavior-considered-undefined.html). For now, we will focus on one category: operations on memory.

Memory is the space where data is stored during the execution of a program. There are many ways to think about memory:

- If you are unfamiliar with systems programming, you might think of memory at a high level like "memory is the RAM in my computer" or "memory is the thing that runs out if I load too much data".
- If you are familiar with systems programming, you might think of memory at a low level like "memory is an array of bytes" or "memory is the pointers I get back from `malloc`".

Both of these memory models are _valid_, but they are not _useful_ ways to think about how Rust works. The high-level model is too abstract to explain how Rust works. You will need to understand the concept of a pointer, for instance. The low-level model is too concrete to explain how Rust works. Rust does not allow you to interpret memory as an array of bytes, for instance.

Rust provides a particular way to think about memory. Ownership is a discipline for safely using memory within that way of thinking. The rest of this chapter will explain the Rust model of memory.

### Variables Live in the Stack

Here's a program like the one you saw in Section 3.3 that defines a number `n` and calls a function `plus_one` on `n`. Beneath the program is a new kind of diagram. This diagram visualizes the contents of memory during the program's execution at the three marked points.

```aquascope,interpreter,horizontal
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

Frames are organized into a **stack** of currently-called-functions. For example, at L2 the frame for `main` sits above the frame for the called function `plus_one`. After a function returns, Rust deallocates the function's frame. (Deallocation is also called **freeing** or **dropping**, and we use those terms interchangeably.) This sequence of frames is called a stack because the most recent frame added is always the next frame freed.

> _Note:_ this memory model does not fully describe how Rust actually works! As we saw earlier with the assembly code, the Rust compiler might put `n` or `x` into a register rather than a stack frame. But that distinction is an implementation detail. It shouldn't change your understanding of safety in Rust, so we can focus on the simpler case of frame-only variables.

When an expression reads a variable, the variable's value is copied from its slot in the stack frame. For example, if we run this program:

```aquascope,interpreter,horizontal
#fn main() {
let a = 5;`[]`
let mut b = a;`[]`
b += 1;`[]`
#}
```

The value of `a` is copied into `b`, and `a` is left unchanged, even after changing `b`.

### Boxes Live in the Heap

However, copying data can take up a lot of memory. For example, here's a slightly different program. This program copies an array with 1 million elements:

```aquascope,interpreter
#fn main() {
let a = [0; 1_000_000];`[]`
let b = a;`[]`
#}
```

Observe that copying `a` into `b` causes the `main` frame to contain 2 million elements. 

To transfer access to data without copying it, Rust uses **pointers**. A pointer is a value that describes a location in memory. One common way to make a pointer is to allocate memory in the **heap**. The heap is a separate region of memory where data can live indefinitely. Heap data is not tied to a specific stack frame. Rust provides a construct called [`Box`](https://doc.rust-lang.org/std/boxed/index.html) for putting data on the heap. For example, we can wrap the million-element array in `Box::new` like this:

```aquascope,interpreter
#fn main() {
let a = Box::new([0; 1_000_000]);`[]`
let b = a;`[]`
#}
```

Observe that now, there is only ever a single array at a time. At L1, the value of `a` is a pointer (represented by dot with an arrow) to the array inside the heap. The statement `let b = a` copies the pointer from `a` into `b`, but the pointed-to data is not copied.

{{#quiz ../quizzes/ch04-01-ownership-sec1-stackheap.toml}}

### Rust Does Not Permit Manual Memory Management

Memory management is the process of allocating memory and deallocating memory. In other words, it's the process of finding unused memory and later returning that memory when it is no longer used. Stack frames are automatically managed by Rust. When a function is called, Rust allocates a stack frame for the called function. When the call ends, Rust deallocates the stack frame.

As we saw above, heap data is allocated when calling `Box::new(..)`. But when is heap data deallocated? Imagine that Rust had a `free()` function that frees a heap allocation. Imagine that Rust let a programmer call `free` whenever they wanted. This kind of "manual" memory management easily leads to bugs. For example, we could read a pointer to freed memory:

```aquascope,interpreter,shouldFail
#fn free<T>(_t: T) {}
#fn main() {
let b = Box::new([0; 100]);`[]`
free(b);`[]`
assert!(b[0] == 0);`[]`
#}
```

> *Note:* you may wonder how we are executing this Rust program that doesn't compile. We use [special tools](https://github.com/cognitive-engineering-lab/aquascope) to simulate Rust as if the borrow checker were disabled, for educational purposes. That way we can answer what-if questions, like: what if Rust let this unsafe program compile?

Here, we allocate an array on the heap. Then we call `free(b)`, which deallocates the heap memory of `b`. Therefore the value of `b` is a pointer to invalid memory, which we represent as the "⦻" icon. No undefined behavior has happened yet! The program is still safe at L2. It's not necessarily a problem to have an invalid pointer.

The undefined behavior happens when we try to *use* the pointer by reading `b[0]`. That would attempt to access invalid memory, which could cause the program to crash. Or worse, it could not crash and return arbitrary data. Therefore this program is **unsafe**.

Rust does not allow programs to manually deallocate memory. That policy avoids the kinds of undefined behaviors shown above.

### A Box's Owner Manages Deallocation

Instead, Rust _automatically_ frees a box's heap memory. Here is an _almost_ correct description of Rust's policy for freeing boxes:

> **Box deallocation principle (almost correct):** If a variable is bound to a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

For example, let's trace through a program that allocates and frees a box:

```aquascope,interpreter,horizontal
fn main() {
    let a_num = 4;
    `[]`make_and_drop();`[]`
}

fn make_and_drop() {
    let a_box = Box::new(5);`[]`
}
```

At L1, before calling `make_and_drop`, the state of memory is just the stack frame for `main`. Then at L2, while calling `make_and_drop`, `a_box` points to `5` on the heap. Once `make_and_drop` is finished, Rust deallocates its stack frame. `make_and_drop` contains the variable `a_box`, so Rust also deallocates the heap data in `a_box`. Therefore the heap is empty at L3.

The box's heap memory has been successfully managed. But what if we abused this system? Returning to our earlier example, what happens when we bind two variables to a box?

```rust,ignore
#fn main() {
let a = Box::new([0; 1_000_000]);
let b = a;
#}
```

The boxed array has now been bound to both `a` and `b`. By our "almost correct" principle, Rust would try to free the box's heap memory *twice* on behalf of both variables. That's undefined behavior too!

To avoid this situation, we finally arrive at ownership. When `a` is bound to `Box::new([0; 1_000_000])`, we say that `a` **owns** the box. The statement `let b = a` **moves** ownership of the box from `a` to `b`. Given these concepts, Rust's policy for freeing boxes is more accurately described as:

> **Box deallocation principle (fully correct):** If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

In the example above, `b` owns the boxed array. Therefore when the scope ends, Rust deallocates the box only once on behalf of `b`, not `a`.


### At Runtime, A Move is Just a Copy

A common misconception is that a "move" actually moves data around in memory. But that is not true! A move is just a copy. For example, let's look again at what happens when we move a boxed array from `a` to `b`:

```aquascope,interpreter,horizontal
#fn main() {
let a = Box::new([0; 1_000_000]);`[]`
let b = a;`[]`
#}
```

An exceedingly common question from readers is: if `a` is moved at L2, why is it still in the diagram? Shouldn't `a` disappear, or gray out, or otherwise "move" somewhere?

No! **At runtime, nothing happens to `a` when it is moved.** There is no "ownership bit" that gets flipped in memory. There is no "has-been-moved" flag that gets turned on. Ownership only exists at compile-time. The diagram does not show how the compiler "thinks" about the program. It shows how the program actually executes at runtime. At runtime, a move is just a copy. At compile-time, a move is a transfer of ownership.


### Collections Use Boxes

Boxes are used by Rust data structures[^boxed-data-structures] like [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), [`String`](https://doc.rust-lang.org/std/string/struct.String.html), and [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) to hold a variable number of elements. For example, here's a program that creates, moves, and mutates a string:

```aquascope,interpreter,horizontal
fn main() {
    let first = String::from("Ferris");`[]`
    let full = add_suffix(first);`[]`
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    `[]`name.push_str(" Jr.");`[]`
    name
}
```


This program is more involved, so make sure you follow each step:

1. At L1, the string "Ferris" has been allocated on the heap. It is owned by `first`.
2. At L2, the function `add_suffix(first)` has been called. This moves ownership of the string from `first` to `name`. The string data is not copied, but the pointer to the data is copied.
3. At L3, the function `name.push_str(" Jr.")` resizes the string's heap allocation. This does three things. First, it frees the original heap memory. Second, it creates a new larger allocation. Third, it writes "Ferris Jr." into the new allocation. `first` now points to deallocated memory.
4. At L4, the frame for `add_suffix` is gone. This function returned `name`, transferring ownership of the string to `full`.


### Variables Cannot Be Used After Being Moved

The string program helps illustrate a key safety principle for ownership. Imagine that `first` were used in `main` after calling `add_suffix`. We can simulate such a program and see the undefined behavior that results:

```aquascope,interpreter,shouldFail
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}, originally {first}");`[]` // first is now used here
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
```

`first` points to deallocated memory after calling `add_suffix`. Reading `first` in `println!` would therefore be a violation of memory safety (undefined behavior). Remember: it's not a problem that `first` points to deallocated memory. It's a problem that we tried to *use* `first` after it became invalid.

Thankfully, Rust will refuse to compile this program, giving the following error:

```text
error[E0382]: borrow of moved value: `first`
 --> test.rs:4:35
  |
2 |     let first = String::from("Ferris");
  |         ----- move occurs because `first` has type `String`, which does not implement the `Copy` trait
3 |     let full = add_suffix(first);
  |                           ----- value moved here
4 |     println!("{full}, originally {first}"); // first is now used here
  |                                   ^^^^^ value borrowed here after move
```

Let's walk through the steps of this error. Rust says that `first` is moved when we called `add_suffix(first)` on line 3. The error clarifies that `first` is moved because it has type `String`, which does not implement `Copy`. We will discuss `Copy` soon &mdash; in brief, you would not get this error if you used an `i32` instead of `String`. Finally, the error says that we use `first` after being moved (it's "borrowed", which we discuss next section).

So if you move a variable, Rust will stop you from using that variable later. More generally, the compiler will enforce this principle:

> **Moved heap data principle:** if a variable `x` moves ownership of heap data to another variable `y`, then `x` cannot be used after the move.

Now you should start to see the relationship between ownership, moves, and safety. Moving ownership of heap data avoids undefined behavior from reading deallocated memory.

### Cloning Avoids Moves

One way to avoid moving data is to *clone* it using the `.clone()` method. For example, we can fix the safety issue in the previous program with a clone:

```aquascope,interpreter
fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();`[]`
    let full = add_suffix(first_clone);`[]`
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
```

Observe that at L1, `first_clone` did not "shallow" copy the pointer in `first`, but instead "deep" copied the string data into a new heap allocation. Therefore at L2, while `first_clone` has been moved and invalidated by `add_suffix`, the original `first` variable is unchanged. It is safe to continue using `first`.

{{#quiz ../quizzes/ch04-01-ownership-sec2-moves.toml}}

### Summary

Ownership is primarily a discipline of heap management:[^pointer-management]

- All heap data must be owned by exactly one variable.
- Rust deallocates heap data once its owner goes out of scope.
- Ownership can be transferred by moves, which happen on assignments and function calls.
- Heap data can only be accessed through its current owner, not a previous owner.

We have emphasized not just _how_ Rust's safeguards work, but _why_ they avoid undefined behavior. When you get an error message from the Rust compiler, it's easy to get frustrated if you don't understand why Rust is complaining. These conceptual foundations should help you with interpreting Rust's error messages.  They should also help you design more Rustic APIs.

[^boxed-data-structures]: These data structures don't use the literal `Box` type. For example, `String` is implemented with `Vec`, and `Vec` is implemented with [`RawVec`](https://doc.rust-lang.org/nomicon/vec/vec-raw.html) rather than `Box`. But types like `RawVec` are still box-like: they own memory in the heap.

[^pointer-management]: In another sense, ownership is a discipline of *pointer* management. But we haven't described yet about how to create pointers to anywhere other than the heap. We'll get there in the next section.

[`NameError`]: https://docs.python.org/3/library/exceptions.html#NameError
[`ReferenceError`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ReferenceError