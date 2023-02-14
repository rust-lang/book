## Fixing Ownership Errors

A core Rust skill is learning how to fix an ownership error. When the borrow checker rejects your code, how should you respond?

The last two sections have shown how a Rust program can be **unsafe** if it triggers undefined behavior. The ownership guarantee is that Rust will reject all unsafe programs. However, Rust will also reject *some* safe programs. Fixing an ownership error will depend on whether your program is *actually* safe or unsafe.

### Fixing an Unsafe Program: Returning a Reference to the Stack

For example, returning a reference to a stack-allocated variable is always unsafe, like this:

```rust,ignore,does_not_compile
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
```

In these cases, you should ask: **why is my program unsafe?** Here, the issue is with the lifetime of the referred data. If you want to pass around a string, you have to make sure the string lives long enough. 

Depending on your situation, here are four ways you can extend the lifetime of the string. One is to move ownership of the string out of the function:

```rust
fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}
```

Another possibility is to return a string literal, which lives forever (indicated by `'static`). This applies if we never intend to change the string.

```rust
fn return_a_string() -> &'static str {
    "Hello world"    
}
```

Another possibility is to defer lifetime-checking to runtime by the use of garbage collection. For example, you can use a [reference-counted pointer][rc]:

```rust
use std::rc::Rc;
fn return_a_string() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}
```

And another possibility is to have the caller provide a place to put the string, using a mutable reference:

```rust
fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}
```

Which strategy is most appropriate will depend on your application. But the key idea is to recognize the root issue underlying the surface-level ownership error: how long should my string live? Who should be in charge of deallocating it? Once you have a clear answer to those questions, then it's a matter of changing your API to match.

### Fixing an Unsafe Program: Aliasing and Mutating a Data Structure

Another always-unsafe operation is holding a reference to heap data that could be deallocated by another alias. For example, here's a function that gets a reference to the largest string in a vector, and uses it while mutating the vector:

```aquascope,permissions,stepper,boundaries,shouldFail
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = 
      dst.iter().max_by_key(|s| s.len()).unwrap();`(focus,paths:dst)`
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
```

Again, we ask: **why is this program unsafe?** Because `dst.push(..)` could deallocate the contents of `dst`, invalidating the reference `largest`. This unsafety is reflected in the fact that `let largest = ..` removes write permissions on `dst`, so `dst` cannot be pushed.

The key insight is that we need to shorten the lifetime of `largest` to not overlap with `dst.push(..)`. One possibility is to clone `largest`:

```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
```

However, this may cause a performance hit for allocating and copying the string data.

Another possibility is to perform all the length comparisons first, and then mutate `dst` afterwards:

```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = 
        src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
    dst.extend(to_add);
}
```

However, this also causes a performance hit for allocating the vector `to_add`.

A final possibility is to copy out the length, since we don't need the contents of `largest`, just its length. 
This solution is arguably the most idiomatic and the most performant.

```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
```

All these solutions share in common the key idea: shortening the lifetime of string borrowed from `dst` so as to not overlap with a mutation to `dst`.

### Fixing an Unsafe Program: Copying vs. Moving Out of a Data Structure

A common confusion for Rust learners arises around the interaction of ownership and collections, like vectors. For example, here's a safe program that copies a number out of a vector:

```aquascope,permissions,stepper,boundaries
#fn main() {
let v: Vec<i32> = vec![0, 1, 2];
let n_ref: &i32 = v.get(0).unwrap();`(focus,paths:\*n_ref)`
let n: i32 = *n_ref;

// Normally you would just write:
//   let n = v[0];
// But we've expanded the syntactic
// sugar for clarity.
#}
```

But if we change the type of elements from numbers to strings, then the program becomes unsafe!

```aquascope,interpreter+permissions,stepper,boundaries,shouldFail,horizontal
#fn main() {
let v: Vec<String> = vec![String::from("Hello world")];
let s_ref: &String = v.get(0).unwrap();`(focus,paths:\*s_ref)`
let s: String = *s_ref;`[]`

// These are normally implicit,
// but we've added them for clarity.
drop(s);`[]`
drop(v);`[]`
#}
```

What happened here is *a double-free.* **Rust does not allow moves through a dereference.** So the expression `*s_ref` copies the string pointer to "Hello world", but then **both** `v` and `s` think they own "Hello world". Therefore after `s` is dropped, "Hello world" is deallocated. Then `v` is dropped, and undefined behavior happens when the string is freed a second time.

Because this second program is unsafe, Rust's borrow checker will reject it with the following error:

```text
error[E0507]: cannot move out of `*s_ref` which is behind a shared reference
 --> test.rs:4:9
  |
4 | let s = *s_ref;
  |         ^^^^^^
  |         |
  |         move occurs because `*s_ref` has type `String`, which does not implement the `Copy` trait
```

But why did the first program, the vector of numbers, pass the compiler? The error message gives a hint: "move occurs because `*s_ref` has type `String`, *which does not implement the `Copy` trait.*"

In short, **if a value does not own heap data, then it can be copied without a move.** (More formally, the type of the value implements the `Copy` trait, but we'll discuss traits in a later chapter.) For example:

* An `i32` **does not** own heap data, so it **can** be copied without a move. 
* A `String` **does** own heap data, so it **can not** be copied without a move.
* An `&String` **does not** own heap data, so it **can** be copied without a move.

In terms of permissions, the idea is that the dereference operator `*` requires own-permissions. In the example below, you can see how a dereference of an `i32` has own-permissions, while a dereference of a `String` does not:

```aquascope,permissions,stepper
#fn main() {
let v1: Vec<i32> = vec![0];
let n: &i32 = &v1[0];`(focus,paths:\*n)`

let v2: Vec<String> = vec![String::new()];
let s: &String = &v2[0];`(focus,paths:\*s)`
#println!("{n} {s}");
#}
```

But let's say we have a vector of non-`Copy` types like `String`, and we want to get access to an element of the vector. Here's a few different ways to safely do so. First, you can avoid taking ownership of the string and just use an immutable reference:

```rust
#fn main() {
let v: Vec<String> = vec![String::from("Hello world")];
let s_ref: &String = &v[0];
println!("{s_ref}!");
#}
```

Second, you can clone the data if you want to get ownership of the string while leaving the vector alone:

```rust
#fn main() {
let v: Vec<String> = vec![String::from("Hello world")];
let mut s: String = v[0].clone();
s.push('!');
println!("{s}");
#}
```

Finally, you can use [`Vec::remove`] to move the string out of the vector:

```rust
#fn main() {
let mut v: Vec<String> = vec![String::from("Hello world")];
let mut s: String = v.remove(0);
s.push('!');
println!("{s}");
assert!(v.len() == 0);
#}
```


### Fixing a Safe Program: Mutating Disjoint Tuple Fields

The above examples are cases where a program is unsafe. However, Rust may also reject safe programs. One common reason is that Rust tracks permissions at a fine-grained level, but it may end up conflating two distinct paths as the same path. 
 
As an example of fine-grained permission tracking, here's a program that shows how you can borrow one field of a tuple, and write to a different field of the same tuple:

```aquascope,permissions,stepper
#fn main() {
let mut name = (
    String::from("Ferris"), 
    String::from("Rustacean")
);`(focus,paths:name)`
let first = &name.0;`(focus,paths:name)`
name.1.push_str(", Esq.");
println!("{first} {}", name.1);
#}
```

The statement `let first = &name.0` borrows `name.0`. This borrow removes write/own permissions from `name.0`. It also removes write/own permissions from `name`, e.g. one could not pass `name` to a function that takes as input a value of type `(String, String)`. However, `name.1` still retains write permissions, so doing `name.1.push_str(...)` is a valid operation.

However, sometimes Rust might lose track of exactly which paths are borrowed. For example, let's say we factor out `&name.0` into a function `get_first`. Notice how `get_first(&name)` now removes permissions on `name.1`!

```aquascope,permissions,stepper,shouldFail
fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn main() {
    let mut name = (
        String::from("Ferris"), 
        String::from("Rustacean")
    );
    let first = get_first(&name);`(focus,paths:name)`
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}
```

The problem is that Rust doesn't look at the implementation of `get_first` when deciding what `get_first(&name)` should borrow. Rust only looks at the type signature, which just says "some `String` in the input gets borrowed". Rust conservatively decides then that both `name.0` and `name.1` get borrowed, and eliminates write and own permissions on both. The borrow checker therefore rejects the program with the following error:

```text
error[E0502]: cannot borrow `name.1` as mutable because it is also borrowed as immutable
  --> test.rs:11:5
   |
10 |     let first = get_first(&name);
   |                           ----- immutable borrow occurs here
11 |     name.1.push_str(", Esq.");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
12 |     println!("{first} {}", name.1);
   |                ----- immutable borrow later used here
```

Remember, the key idea is that **the program above is safe.** It has no undefined behavior! A future version of Rust may be smart enough to let it compile, but for today, it gets rejected. To work around the borrow checker, one possibility is to inline the expression `&name.0` as with the original program. Another possibility is to defer borrow checking to runtime through the use of [cells], which we will discuss in future chapters.

### Fixing a Safe Program: Mutating Disjoint Array Indices

A similar kind of problem arises when we borrow elements of an array. For example, observe what paths are borrowed when we take a mutable reference to an array:

```aquascope,permissions,stepper
#fn main() {
let mut a = [0, 1, 2, 3];
let x = &mut a[0];`(focus,paths:a)`
*x += 1;`(focus,paths:a)`
println!("{a:?}");
#}
```

The borrow checker does not contain different paths for `a[0]`, `a[1]`, and so on. There is a single path `a[]` that represents *all* elements of `a`. Rust does this because it cannot always determine the value of an index in more complex situations like this: 

```rust,ignore
let idx = a_complex_function();
let x = &mut a[idx];
```

As a result, a perfectly safe program that uses two disjoint indices will be rejected:

```aquascope,permissions,boundaries,stepper,shouldFail
#fn main() {
let mut a = [0, 1, 2, 3];
let x = &mut a[0];`(focus,paths:a\[\])`
let y = &a[1];
*x += *y;
#}
```

Again, **this program is safe.** For cases like these, Rust often provides a function in the standard library that can assist in working around the borrow checker. For example, we could use [`slice::split_first_mut`][split_first_mut]:

```rust
#fn main() {
let mut a = [0, 1, 2, 3];
let (x, rest) = a.split_first_mut().unwrap();
let y = &rest[0];
*x += *y;
#}
```

You might wonder, but how is `split_first_mut` implemented? In some Rust libraries, especially core types like `Vec` or `slice`, you will often find **unsafe code**, or code within an `unsafe { .. }` block. For example, we could use an unsafe block to accomplish our task:

```rust
#fn main() {
let mut a = [0, 1, 2, 3];
let x = &mut a[0] as *mut i32;
let y = &a[1] as *const i32;
unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing!
#}
```

However, you should rarely find yourself directly invoking `unsafe`, especially in application code. More commonly, data structures are designed to carefully encapsulate unsafe code behind a safe API. We will discuss unsafe code further in [Chapter 20][unsafe]. For now, it's simply useful to be aware that unsafe code is how Rust implements certain otherwise-impossible patterns.

{{#quiz ../quizzes/ch04-03-fixing-ownership-errors.toml}}

### Summary

When fixing an ownership error, you should ask yourself: is my program actually unsafe? If yes, then you need to understand the root cause of the unsafety. If no, then you need to understand the limitations of the borrow checker to work around them.

[rc]: https://doc.rust-lang.org/std/rc/index.html
[cells]: https://doc.rust-lang.org/std/cell/index.html
[split_first_mut]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_first_mut
[unsafe]: ch19-01-unsafe-rust.html
[`Vec::remove`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove