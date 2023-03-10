## Fixing Ownership Errors

Learning how to fix an ownership error is a core Rust skill. When the borrow checker rejects your code, how should you respond? In this section, we will discuss several case studies of common ownership errors. Each case study will present a function rejected by the compiler. Then we will explain why Rust rejects the function, and show several ways to fix it.

A common theme will be understanding whether a function is *actually* safe or unsafe. Rust will always reject an unsafe program[^safe-subset]. But sometimes, Rust will also reject a safe program. These case studies will show how to respond to errors in both situations.

<!-- The last two sections have shown how a Rust program can be **unsafe** if it triggers undefined behavior. The ownership guarantee is that Rust will reject all unsafe programs. However, Rust will also reject *some* safe programs. Fixing an ownership error will depend on whether your program is *actually* safe or unsafe. -->

### Fixing an Unsafe Program: Returning a Reference to the Stack

Our first case study is about returning a reference to the stack, just like we discussed last section in ["Data Must Outlive All Of Its References"](ch04-02-references-and-borrowing.html#data-must-outlive-all-of-its-references). Here's the function we looked at:

```rust,ignore,does_not_compile
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
```

When thinking about how to fix this function, we need to ask: **why is this program unsafe?** Here, the issue is with the lifetime of the referred data. If you want to pass around a reference to a string, you have to make sure that the underlying string lives long enough. 

Depending on the situation, here are four ways you can extend the lifetime of the string. One is to move ownership of the string out of the function, changing `&String` to `String`

```rust
fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}
```

Another possibility is to return a string literal, which lives forever (indicated by `'static`). This solution applies if we never intend to change the string, and the string can be written directly in the program source code:

```rust
fn return_a_string() -> &'static str {
    "Hello world"    
}
```

Another possibility is to defer lifetime-checking to runtime by using garbage collection. For example, you can use a [reference-counted pointer][rc]:

```rust
use std::rc::Rc;
fn return_a_string() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}
```

We will discuss reference-counting more in Chapter 15.4 ["`Rc<T>`, the Reference Counted Smart Pointer"](ch15-04-rc.html). In short, `Rc::clone` only clones a pointer to `s` and not the data itself. At runtime, the `Rc` checks when the last `Rc` pointing to data has been dropped, and then deallocates the data.

Yet another possibility is to have the caller provide a "slot" to put the string using a mutable reference:

```rust
fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}
```

With this strategy, the caller is responsible for creating space for the string. This style can be verbose, but it can also be more memory-efficient if the caller needs to carefully control when allocations occur.

Which strategy is most appropriate will depend on your application. But the key idea is to recognize the root issue underlying the surface-level ownership error. How long should my string live? Who should be in charge of deallocating it? Once you have a clear answer to those questions, then it's a matter of changing your API to match.


### Fixing an Unsafe Program: Not Enough Permissions

Another common issue is trying to mutate read-only data, or trying to drop data behind a reference. For example, let's say we tried to write a function `stringify_name_with_title`. This function is supposed to create a person's full name from a vector of name parts, including an extra title.

```aquascope,permissions,stepper,boundaries,shouldFail
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));`{}`
    let full = name.join(" ");
    full
}

// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
```

This program is rejected by the borrow checker because `name` is an immutable reference, but `name.push(..)` requires the @Perm{write} permission. This program is unsafe because `push` could invalidate other references to `name` outside of `stringify_name_with_title`, like this:

```aquascope,interpreter,shouldFail,horizontal
#fn stringify_name_with_title(name: &Vec<String>) -> String {
#    name.push(String::from("Esq."));`{}`
#    let full = name.join(" ");
#    full
#}
fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];`[]`
    stringify_name_with_title(&name);`[]`
    println!("{}", first);`[]`
}
```

So how do we fix this program? One straightforward solution is to change the type of name from `&Vec<String>` to `&mut Vec<String>`:

```rust,ignore
fn stringify_name_with_title(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
```

But this is not a good solution! **Functions should not mutate their inputs if the caller would not expect it.** A person calling `stringify_name_with_title` probably does not expect their vector to be modified by this function. Another function like `add_title_to_name` might be expected to mutate a name, but not this one.

Another option is to take ownership of the name, by changing `&Vec<String>` to `Vec<String>`:

```rust,ignore
fn stringify_name_with_title(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
```

But this is also not a good solution! **It is very rare for Rust functions to take ownership of heap-owning data structures like `Vec` and `String`.**  This version of `stringify_name_with_title` would make the input `name` unusable, which is very annoying to a caller as we discussed at the beginning of ["References and Borrowing"](ch04-02-references-and-borrowing.html).

So the choice of `&Vec` is actually a good one, which we do *not* want to change. Instead, we can change the body of the function. There are many possible fixes which vary in how much memory they use. One possibility is to clone the input `name`:

```rust,ignore
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}
```

By cloning `name`, we are allowed to mutate the local copy of the vector. However, the clone copies every string in the input. We can avoid unnecessary copies by adding the suffix later:

```rust,ignore
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}
```

In general, writing Rust functions is a careful balance of asking for the *right* level of permissions. For this example, it's most idiomatic to only expect read permission.

{{#quiz ../quizzes/ch04-03-fixing-ownership-errors-sec1-idioms.toml}}

### Fixing an Unsafe Program: Aliasing and Mutating a Data Structure

Another unsafe operation is using a reference to heap data that gets deallocated by another alias. For example, here's a function that gets a reference to the largest string in a vector, and then uses it while mutating the vector:

```aquascope,permissions,stepper,boundaries,shouldFail
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = 
      dst.iter().max_by_key(|s| s.len()).unwrap();`(focus,paths:*dst)`
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());`{}`
        }
    }
}
```

> *Note:* this example uses [iterators] and [closures] to succinctly find a reference to the largest string. We will discuss those features in later chapters, and for now we will provide an intuitive sense of how the features work here.

This program is rejected by the borrow checker because `let largest = ..` removes the @Perm{write} permissions on `dst`. However, `dst.push(..)` requires the @Perm{write} permission. Again, we should ask: **why is this program unsafe?** Because `dst.push(..)` could deallocate the contents of `dst`, invalidating the reference `largest`.

To fix the program, the key insight is that we need to shorten the lifetime of `largest` to not overlap with `dst.push(..)`. One possibility is to clone `largest`:

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

These solutions all share in common the key idea: shortening the lifetime of borrows on `dst` to not overlap with a mutation to `dst`.

### Fixing an Unsafe Program: Copying vs. Moving Out of a Collection

A common confusion for Rust learners happens when copying data out of a collection, like a vector. For example, here's a safe program that copies a number out of a vector:

```aquascope,permissions,stepper,boundaries
#fn main() {
let v: Vec<i32> = vec![0, 1, 2];
let n_ref: &i32 = &v[0];`(focus,paths:*n_ref)`
let n: i32 = *n_ref;`{}`
#}
```

The dereference operation `*n_ref` expects the @Perm{own} permission, which the path `*n_ref` has. But what happens if we change the type of elements in the vector from `i32` to `String`? Then it turns out we no longer have the necessary permissions:

```aquascope,permissions,stepper,boundaries,shouldFail
#fn main() {
let v: Vec<String> = 
  vec![String::from("Hello world")];
let s_ref: &String = &v[0];`(focus,paths:*s_ref)`
let s: String = *s_ref;`[]``{}`
#}
```

The first program will compile, but the second program will not compile. Rust gives the following error message:

```text
error[E0507]: cannot move out of `*s_ref` which is behind a shared reference
 --> test.rs:4:9
  |
4 | let s = *s_ref;
  |         ^^^^^^
  |         |
  |         move occurs because `*s_ref` has type `String`, which does not implement the `Copy` trait
```

The issue is that the vector `v` owns the string "Hello world". When we dereference `s_ref`, that tries to take ownership of the string from the vector. But references are non-owning pointers &mdash; we can't take ownership *through* a reference. Therefore Rust complains that we "cannot move out of \[...\] a shared reference".

But why is this unsafe? We can illustrate the problem by simulating the rejected program:

```aquascope,interpreter,shouldFail,horizontal
#fn main() {
let v: Vec<String> = 
  vec![String::from("Hello world")];
let s_ref: &String = &v[0];`(focus,paths:*s_ref)`
let s: String = *s_ref;`[]``{}`

// These drops are normally implicit, but we've added them for clarity.
drop(s);`[]`
drop(v);`[]`
#}
```

What happens here is a **double-free.** After executing `let s = *s_ref`, both `v` and `s` think they own "Hello world". After `s` is dropped, "Hello world" is deallocated. Then `v` is dropped, and undefined behavior happens when the string is freed a second time.

> *Note:* we don't even have to *use* `v` or `s` to cause undefined behavior through the double-free. After we move the string out from `s_ref`, undefined behavior will happen once the elements are dropped.

However, this undefined behavior does not happen when the vector contains `i32` elements. The difference is that copying a `String` copies a pointer to heap data. Copying an `i32` does not.
In technical terms, Rust says that the type `i32` implements the `Copy` trait, while `String` does not implement `Copy` (we will discuss traits in a later chapter).

In sum, **if a value does not own heap data, then it can be copied without a move.** For example:

* An `i32` **does not** own heap data, so it **can** be copied without a move. 
* A `String` **does** own heap data, so it **can not** be copied without a move.
* An `&String` **does not** own heap data, so it **can** be copied without a move.

> *Note:* One exception to this rule is mutable references. For example, `&mut i32` is not a copyable type. So if you do something like:
> ```rust,ignore
> let mut n = 0;
> let a = &mut n;
> let b = a;
> ```
> Then `a` cannot be used after being assigned to `b`. That prevents two mutable references to the same data from being used at the same time.

So if we have a vector of non-`Copy` types like `String`, then how do we safely get access to an element of the vector? Here's a few different ways to safely do so. First, you can avoid taking ownership of the string and just use an immutable reference:

```rust,ignore
#fn main() {
let v: Vec<String> = vec![String::from("Hello world")];
let s_ref: &String = &v[0];
println!("{s_ref}!");
#}
```

Second, you can clone the data if you want to get ownership of the string while leaving the vector alone:

```rust,ignore
#fn main() {
let v: Vec<String> = vec![String::from("Hello world")];
let mut s: String = v[0].clone();
s.push('!');
println!("{s}");
#}
```

Finally, you can use a method like [`Vec::remove`] to move the string out of the vector:

```rust,ignore
#fn main() {
let mut v: Vec<String> = vec![String::from("Hello world")];
let mut s: String = v.remove(0);
s.push('!');
println!("{s}");
assert!(v.len() == 0);
#}
```


### Fixing a Safe Program: Mutating Different Tuple Fields

The above examples are cases where a program is unsafe. Rust may also reject safe programs. One common issue is that Rust tries to track permissions at a fine-grained level. However, Rust may conflate two different paths as the same path. 
 
Let's first look at an example of fine-grained permission tracking that passes the borrow checker. This program shows how you can borrow one field of a tuple, and write to a different field of the same tuple:

```aquascope,permissions,stepper,boundaries
#fn main() {
let mut name = (
    String::from("Ferris"), 
    String::from("Rustacean")
);`(focus,paths:name)`
let first = &name.0;`(focus,paths:name)`
name.1.push_str(", Esq.");`{}`
println!("{first} {}", name.1);
#}
```

The statement `let first = &name.0` borrows `name.0`. This borrow removes @Perm{write}@Perm{own} permissions from `name.0`. It also removes @Perm{write}@Perm{own} permissions from `name`. (For example, one could not pass `name` to a function that takes as input a value of type `(String, String)`.) But `name.1` still retains the @Perm{write} permission, so doing `name.1.push_str(...)` is a valid operation.

However, Rust can lose track of exactly which paths are borrowed. For example, let's say we refactor the expression `&name.0` into a function `get_first`. Notice how after calling `get_first(&name)`, Rust now removes the @Perm{write} permission on `name.1`:

```aquascope,permissions,stepper,boundaries,shouldFail
fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn main() {
    let mut name = (
        String::from("Ferris"), 
        String::from("Rustacean")
    );
    let first = get_first(&name);`(focus,paths:name)`
    name.1.push_str(", Esq.");`{}`
    println!("{first} {}", name.1);
}
```

Now we can't do `name.1.push_str(..)`! Rust will return this error:

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

That's strange, since the program was safe before we edited it. The edit we made doesn't meaningfully change the runtime behavior. So why does it matter that we put `&name.0` into a function?

The problem is that Rust doesn't look at the implementation of `get_first` when deciding what `get_first(&name)` should borrow. Rust only looks at the type signature, which just says "some `String` in the input gets borrowed". Rust conservatively decides then that both `name.0` and `name.1` get borrowed, and eliminates write and own permissions on both. 

Remember, the key idea is that **the program above is safe.** It has no undefined behavior! A future version of Rust may be smart enough to let it compile, but for today, it gets rejected. So how should we work around the borrow checker today? One possibility is to inline the expression `&name.0`, like in the original program. Another possibility is to defer borrow checking to runtime with [cells], which we will discuss in future chapters.

### Fixing a Safe Program: Mutating Different Array Elements

A similar kind of problem arises when we borrow elements of an array. For example, observe what paths are borrowed when we take a mutable reference to an array:

```aquascope,permissions,stepper,boundaries
#fn main() {
let mut a = [0, 1, 2, 3];
let x = &mut a[0];`(focus,paths:a[_])`
*x += 1;`(focus,paths:a[_])`
println!("{a:?}");
#}
```

Rust's borrow checker does not contain different paths for `a[0]`, `a[1]`, and so on. It uses a single path `a[_]` that represents *all* indexes of `a`. Rust does this because it cannot always determine the value of an index. For example, imagine a more complex scenario like this:

```rust,ignore
let idx = a_complex_function();
let x = &mut a[idx];
```

What is the value of `idx`? Rust isn't going to guess, so it assumes `idx` could be anything. For example, let's say we try to read from one array index while writing to a different one:

```aquascope,permissions,boundaries,stepper,shouldFail
#fn main() {
let mut a = [0, 1, 2, 3];
let x = &mut a[0];`(focus,paths:a[_])`
let y = &a[1];`{}`
*x += *y;
#}
```

However, Rust will reject this program because `a` gave its read permission to `x`. The compiler's error message says the same thing:

```text
error[E0502]: cannot borrow `a[_]` as immutable because it is also borrowed as mutable
 --> test.rs:4:9
  |
3 | let x = &mut a[0];
  |         --------- mutable borrow occurs here
4 | let y = &a[1];
  |         ^^^^^ immutable borrow occurs here
5 | *x += *y;
  | -------- mutable borrow later used here
```

<!-- However, Rust will reject this program because `a` gave its read permission to `x`. -->


Again, **this program is safe.** For cases like these, Rust often provides a function in the standard library that can work around the borrow checker. For example, we could use [`slice::split_first_mut`][split_first_mut]:

```rust,ignore
#fn main() {
let mut a = [0, 1, 2, 3];
let (x, rest) = a.split_first_mut().unwrap();
let y = &rest[0];
*x += *y;
#}
```

You might wonder, but how is `split_first_mut` implemented? In some Rust libraries, especially core types like `Vec` or `slice`, you will often find **`unsafe` blocks**. `unsafe` blocks allow the use of "raw" pointers, which are not checked for safety by the borrow checker. For example, we could use an unsafe block to accomplish our task:

```rust,ignore
#fn main() {
let mut a = [0, 1, 2, 3];
let x = &mut a[0] as *mut i32;
let y = &a[1] as *const i32;
unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing!
#}
```

Unsafe code is sometimes necessary to work around the limitations of borrow checker. As a general strategy, let's say the borrow checker rejects a program you think is actually safe. Then you should look for standard library functions (like `split_first_mut`) that contain `unsafe` blocks which solve your problem. We will discuss unsafe code further in [Chapter 20][unsafe]. For now, just be aware that unsafe code is how Rust implements certain otherwise-impossible patterns.

{{#quiz ../quizzes/ch04-03-fixing-ownership-errors-sec2-safety.toml}}

### Summary

When fixing an ownership error, you should ask yourself: is my program actually unsafe? If yes, then you need to understand the root cause of the unsafety. If no, then you need to understand the limitations of the borrow checker to work around them.

[rc]: https://doc.rust-lang.org/std/rc/index.html
[cells]: https://doc.rust-lang.org/std/cell/index.html
[split_first_mut]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_first_mut
[unsafe]: ch19-01-unsafe-rust.html
[`Vec::remove`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove
[iterators]: ch13-02-iterators.html
[closures]: ch13-01-closures.html

[^safe-subset]: This guarantee applies for programs written in the "safe subset" of Rust. If you use `unsafe` code or invoke unsafe components (like calling a C library), then you must take extra care to avoid undefined behavior.
