## References and Borrowing

Ownership, boxes, and moves provide a foundation for safely programming with the heap. However, move-only APIs can be inconvenient to use. For example, say you want to read some strings twice:

```rust,ignore,does_not_compile
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    let n = m1.len() + m2.len(); // Error: m1 and m2 are moved
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}
```

In this example, calling `greet` moves the data from `m1` and `m2` into the parameters of `greet`. Both strings are dropped at the end of `greet`, and therefore cannot be used within `main`.

This move behavior is extremely inconvenient. Programs often need to use a string more than once. Hypothetically, an alternative `greet` can return ownership of the strings:

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_again, m2_again) = greet(m1, m2);
    let n = m1_again.len() + m2_again.len();
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}
```

However, this style of program is quite verbose. Rust provides a more concise style of reading and writing movable objects with references.

### References Are Non-Owning Pointers

A **reference** is a kind of pointer. Here's an example of a reference that rewrites our `greet` program in a more convenient manner:

```aquascope,interpreter
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");`[]`
    greet(&m1, &m2);`[]` // note the ampersands
    let n = m1.len() + m2.len();
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    `[]`println!("{} {}!", g1, g2);
}
```

The expression `&m1` uses the ampersand operator to create a reference to (or "borrow") `m1`. The type of the `greet` parameter `g1` is changed to `&String`, meaning "a reference to a `String`". 

<!-- At runtime, the references look like this:

<img src="img/experiment/ch04-02-stack1.jpg" class="center" width="350" /> -->

Observe at L2 that there are two steps from `g1` to the string "Hello". `g1` is a reference that points to `m1` on the stack, and `m1` is a String containing a box that points to "Hello" on the heap.

While `m1` owns the heap data "Hello", `g1` does _not_ own either `m1` or "Hello". Therefore after `greet` ends and the program reaches L3, no heap data has been deallocated. Only the stack frame for `greet` disappears. This fact is consistent with our Moved Heap Data Principle: because `g1` did not own "Hello", Rust did not deallocate "Hello" on behalf of `g1`.

References are **non-owning pointers**, because they do not own the data they point to.

### Dereferencing a Pointer Accesses Its Data

The previous examples using boxes and strings have not shown how Rust "follows" a pointer to its data. For example, the `println!` macro has mysteriously worked for inputs that were both plain strings of type `String`, and references to strings of type `&String`. The underlying mechanism is the **dereference** operator, written with an asterisk (`*`). For example, here's a program that uses dereferences in a few different ways:

```aquascope,interpreter
fn main() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value, 
                             //   x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;`[]`    // so only one dereference is needed to read it
}
```

Observe the difference between `r1` pointing to `x` on the stack, and `r2` pointing to the heap value `2`.

<!--
Let's walk through each line:
1. First, we create a box `x` with `Box::new(1)`.
2. The expression `*x` dereferences `x`, which copies the value `1` off the heap. Then `let a = *x` puts `1` into the stack slot for `a`.
3. By putting the dereference on the left-hand side of `*x += 1`, we are modifying the heap data in-place. The heap value for `x` now contains `2`.
4. The statement `let r1 = &x` creates a reference to `x`, which points to `x` on the stack. `r1` has type `&Box<i32>`, a reference to a box of a 32-bit integer.
5. The statement `let b = **r1` follows the two-step pointer: `*r` goes to `x`, and `**r1` goes to the heap value `2`, therefore `b` is bound to `2`.
6. The statement `let r2 = &*x` dereferences `x` to its heap data, and gets a pointer to the heap. `r2` then points *directly* to the heap, bypassing `x`.
7. The statement `let c = *r2` dereferences `r2` once, getting the value `2` from the heap. -->

You probably won't see the dereference operator very often when you read Rust code. This is because Rust implicitly inserts both dereferences and references in certain cases, such as calling a method with the dot operator. For example, this program shows two equivalent ways of calling the [`i32::abs`](https://doc.rust-lang.org/std/primitive.i32.html#method.abs) (absolute value) and [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len) (string length) functions:

```rust
# fn main()  {
let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); // explicit dereference
let x_abs2 = x.abs();      // implicit dereference
assert_eq!(x_abs1, x_abs2);

let r: &Box<i32> = &x;
let r_abs1 = i32::abs(**r); // explicit dereference (twice)
let r_abs2 = r.abs();       // implicit dereference (twice)
assert_eq!(r_abs1, r_abs2);

let s = String::from("Hello");
let s_len1 = str::len(&s); // explicit reference
let s_len2 = s.len();      // implicit reference
assert_eq!(s_len1, s_len2);
# }
```

This example shows implicit conversions in three ways:
1. The `i32::abs` function expects an input of type `i32`. To call `abs` with a `Box<i32>`, you can explicitly dereference the box like `i32::abs(*x)`, or you can implicitly dereference the box usig method-call syntax like `x.abs()`. The dot syntax is syntactic sugar for the function-call syntax.

2. This implicit conversion works for multiple layers of pointers. For example, calling `abs` on a reference to a box `r: &Box<i32>` will insert two dereferences. 

3. This conversion also works the opposite direction: if a function like `str::len` expects a reference `&str`, then by providing an owned `String`, Rust will insert a single borrowing operator. (In fact, there is a further conversion from `String` to `str`!)

We will say more about method calls and implicit conversions in later chapters. For now, the important takeaway is to recognize that these conversions are happening, especially with method calls and some macros like `println`. We want to unravel all the "magic" of Rust so you can have a clear mental model of how Rust works.

### Rust Avoids Simultaneous Aliasing and Mutation

Pointers are a powerful and dangerous feature because they enable **aliasing**: accessing the same data through different variables. On its own, aliasing is harmless. But combined with **mutation**, we have a recipe for disaster. One variable can "pull the rug out" from another variable in many ways, for example:

- By deallocating the aliased data, leaving the other variable to point to deallocated memory.
- By mutating the aliased data, invalidating runtime properties expected by the other variable.
- By _concurrently_ mutating the aliased data, causing a data race with nondeterministic behavior for the other variable.

Therefore Rust follows a basic principle to prevent undefined behavior:

> **Pointer Safety Principle**: data should never be aliased and mutated at the same time.

Data is allowed to be aliased. Data is allowed to be mutated. But data is _not_ allowed to be _both_ aliased _and_ mutated. For example, Rust enforces this principle for boxes (owned pointers) by disallowing aliasing. Assigning a box from one variable to another will move ownership, invalidating the previous variable. Owned data can only be accessed through the owner &mdash; no aliases.

However, references need different rules to enforce the Pointer Safety Principle because they are non-owning pointers. By design, references are meant to temporarily create aliases. In the remainder of this section, we will explain the basics of how Rust ensures the safety of programs with references.

### References Change Permissions on Paths

The core idea is that variables have three kinds of **permissions** on their data:

- **Read** (`R`): data can be copied to another location.
- **Write** (`W`): data can be mutated in-place.
- **Own** (`O`): data can be moved or dropped.

These permissions don't exist at runtime, only within the compiler. They describe how the compiler "thinks" about your program before the program is ever executed.

By default, a variable has read/own permissions (`RO`) on its data. If a variable is annotated with `let mut`, then it also has write permissions (`W`). The key idea is 
that **references can temporarily remove these permissions.** 

To illustrate this idea, we will use a new kind of diagram. This diagram shows the changes in permissions on each line of the program. For example:

<!-- TODO: can we show unchanged permissions for variables appearing in a table? -->
<!-- horizontal line should be vertical-aligned to bottom of the row, not middle
     to indicate that it describes after-permissions -->

```aquascope,permission-diffs
fn main() {
    let mut x = String::from("Hello");
    let y = &x;
    println!("{} = {}", x, y);
    x.push_str(" world"); 
}
```

Let's walk through each line:

1. After `let mut x = (...)`, the variable `x` has been initialized (indicated by <i class="fa fa-level-up"></i>). It gains read/write/own permissions (green is for gain).
2. After `let y = &x`, the data in `x` has been **borrowed** by `y` (indicated by <i class="fa fa-arrow-right"></i>). Three things happen:
   - The borrow removes write/own permissions from `x` (red is for loss): it cannot write or own its data, but it can still read its data (neutral-colored is for unchanged).
   - The variable `y` has gained read/own permissions. `y` is not writable (the missing permission is shown as a dash `-`) because it was not marked `let mut`.
   - The **path** `*y` has gained read permissions. Note that `y` is different from `*y`! `y` is a reference to a string, while `*y` is the string itself (accessed through a reference).
3. After `println!(...)`, `y` is no longer in use, so `x` is no longer borrowed. Therefore:
   - `x` regains its write/own permissions (indicated by <i class="fa fa-rotate-left"></i>).
   - `y` and `*y` have lost all of their permissions (indicated by <i class="fa fa-level-down"></i>).
4. After `x += 1`, `x` is no longer in use, and it loses all of its permissions.

Permissions are not just defined on variables (like `x` or `y`), but also on **paths** (like `*y`). A path is anything you can put on the left-hand side of an assignment. Paths include:

- Variables, like `a`.
- Dereferences of paths, like `*a`.
- Array accesses of paths, like `a[0]`.
- Fields of paths, like `a.0` for tuples or `a.field` for structs (discussed next section).
- Any combination of the above, like `*((*a)[0].1)`.

For example, here's a program that shows how you can borrow one field of a tuple, and write to a different field of the same tuple:

<!-- option to always show all paths vs only show changed paths -->

```aquascope,permission-diffs
fn main() {
    let mut tup = (
      String::from("A"), 
      String::from("B")
    );
    let s = &tup.0;
    tup.1.push_str("C");
    println!("{} {}", s, tup.1);
}
```

The statement `let s = &tup.0` borrows `tup.0`. This borrow removes write/own permissions from `x.0`. It also removes write/own permissions from `x`, e.g. one could not pass `tup` to a function that takes as input a value of type `(String, String)`. However, `x.1` still retains write permissions, so doing `x.1 += 1` is a valid operation.

Returning to the Pointer Safety Principle, the goal of these permissions is to ensure that data cannot be mutated if it is aliased. Creating a reference to data ("borrowing" it) causes that data to be temporarily read-only until the reference is no longer used.

<!-- The key thing to observe is that a variable's *type* is not the same as its *permissions*. A variable always has the same type, but its permissions will change depending on context. Specifically, a variable loses permissions when its contents are borrowed by a reference. It regains those permissions when the reference is no longer used.

Permissions are a compile-time concept, not a run-time concept. The Rust compiler has a program analyzer called the "borrow checker" which statically checks your program for permissions violations. -->

### The Borrow Checker Finds Permission Violations

Rust uses these permissions in its **borrow checker**. The borrow checker determines whether a program is doing potentially unsafe operations involving references. For example, suppose we placed the `x += 1` statement in-between the definition of `y` and the use of `*y`, like this:

```aquascope,receiver-types
fn main() {
    let mut x = 1;
    let y = &x;
    x += 1;
    println!("{} = {}", x, *y);
}
```

This example shows another kind of visualization. Any time a path is *used*, Rust expects that path to have certain permissions. For example, the borrow `&x` requires that `x` is readable, therefore the permission `R` is shown. The letter is filled-in because `x` has the read permission at that line. 

By contrast, the increment operation `x += 1` requires that `x` is readable and writable, so both `R` and `W` are shown. However, `x` does not have write permissions (it is borrowed by `y`), so the letter is hollow, indicating that `W` is *expected* but `x` does not have it.

If you try to compile this program, then the Rust compiler will reject this program with the following error:

```text
error[E0506]: cannot assign to `x` because it is borrowed
 --> src/main.rs:4:1
  |
3 | let y = &x;
  |         -- borrow of `x` occurs here
4 | x += 1;
  | ^^^^^^ assignment to borrowed `x` occurs here
5 | println!("{} = {}", x, *y);
  |                        -- borrow later used here
```

Now, you may think that Rust is being overzealous here &mdash; what's the harm in incrementing `x`? That won't affect the reference `y`. But here is a similar example with an actual safety issue:


```aquascope,receiver-types
fn main() {
  let mut v = vec![1, 2, 3];
  let n = &v[1];
  v.push(4);
  println!("{:?}", v, n);
}
```

The function `v.push(4)` could resize `v`, deallocating its original contents. This operation would invalidate the reference `n`, leading it to point to deallocated memory. The subsequent read of `*n` would therefore be undefined behavior.

Thankfully, Rust rejects this program for the same reason as before. The statement `&v[0]` borrows the vector `v`, which removes write permissions from `v`, so `v.push(4)` is flagged as a permissions violation.

In sum, Rust's borrow checker prevents _all_ unsafe uses of references (like `v.push(4)`). However, it also prevents _some_ safe uses of references (like `x += 1`). This is an important point: just because Rust rejects your program does not mean your program is _definitely_ unsafe!

### Mutable References Provide Unique and Non-Owning Access to Data

The references we have seen so far are read-only: **immutable references** (also called **shared references**). These references permit aliasing but disallow mutation. However, it is also convenient to be able to temporarily provide mutable access to data without moving it. For example, [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push) should add an element to a vector without consuming ownership of the vector.

The mechanism for this is **mutable references** (also called **unique references**). Here's a simple example of a mutable reference with the accompanying permissions changes:

```aquascope,permission-diffs
fn main() {
    let mut x = 1;
    let y: &mut i32 = &mut x;
    *y += 1;
    println!("{x}");
}
```

A mutable reference is created with the syntax `&mut x`. The type of `y` is written as `&mut i32`. You can see two important differences in the transfer of permissions compared to the previous example:

1. When `y` was an immutable reference, `x` still had read permissions. Now that `y` is a mutable reference, `x` has lost _all_ permissions while `y` is in use.
2. When `y` was an immutable reference, the path `*y` only had read permissions. Now that `y` is a mutable reference, `*y` has also gained write permissions.

The first observation is what makes mutable references safe. Mutable references allow mutation but prevent aliasing. The borrowed path becomes temporarily unusable (i.e. effectively not an alias).

The second observation is what makes mutable references useful. `x` can be mutated through `*y`. For example, `*y += 1` adds 1 to `x`. Note that while `*y` has write permissions, `y` does not. This is because `y` refers to the mutable reference itself, e.g. `y` cannot be reassigned to a *different* mutable reference.

Mutable references can also be temporarily "downgraded" to read-only references. For example:

```aquascope,permission-diffs
fn main() {
    let mut x = 1;
    let y = &mut x;
    let z = &*y;
    println!("{y} {z}");
}
```

In this program, the operation `&*y` removes the write permission from `*y` but _not_ the read permission, so `println!(..)` can read both `*y` and `*z`.

### Data Cannot Be Mutably and Immutably Borrowed

To borrow from a path, that path must have the appropriate permissions: `R` for an immutable reference, and `RW` for a mutable reference. A common source of borrow checker errors is trying to borrow data without enough permissions, for example:

```aquascope,receiver-types
fn main() {
    let mut x = 1;
    let y = &x;
    let z = &mut x;
    *z += 1;
    println!("{}", y);
}
```

Observe that `x` does not have write permissions in the expression `&mut x` because it was previously borrowed by `y`. Therefore, if you try to compile this program, Rust will return the following error:

```text
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:13
  |
3 |     let y = &x;
  |             -- immutable borrow occurs here
4 |     let z = &mut x;
  |             ^^^^^^ mutable borrow occurs here
5 |     *z += 1;
6 |     println!("{}", y);
  |                    - immutable borrow later used here
```

### Permissions Are Returned At The End of a Reference's Lifetime

Previously, we explained that that a reference changes permissions while the reference is "in use". The phrase "in use" is describing a reference's **lifetime**, which is the range of code spanning from its birth (where the reference is created) to its death (the last time(s) the reference is used).

For example, in this program, the lifetime of `y` starts with `let y = &x`, and ends with `let z = *y`:

```aquascope,permission-diffs
fn main() {
    let mut x = 1;
    let y = &x;
    let z = *y;
    x += z;
}
```

The read permissions on `x` are returned to `x` after the lifetime of `y` has ended, like we have seen before.

In the previous examples, a lifetime has been a contiguous region of code. However, once we introduce control flow, this is not necessarily the case. For example, here is a simple function that capitalizes the first character in a vector of ASCII characters:

```aquascope,permission-diffs
fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let c_up = c.to_ascii_uppercase();
        v[0] = c_up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}
```

Note that the lifetime of the character `c` starts with `let c = &v[0]`. `c` is then used in two places: the `if` condition, and the statement `let c_up = (...)`. But `c` is not used inside the `else` block.

Therefore `c` has a different status in each branch. Going into the `if`-block, `c` is still in-use. It would be a permissions violation to mutate `v` before `let c_up = (...)`. But going into the `else`-block, `c` is out-of-use. `v` has full permissions.

In general, Rust will infer the lifetimes of references at the smallest granularity possible.

### Data Must Outlives All Of Its References

One final safety property for references is that **data must outlives its references.** For example, consider this function that adds a reference to a vector of references:

```rust,ignore,does_not_compile
fn add_ref(v: &mut Vec<&i32>, n: i32) {
    let r = &n;
    v.push(r);
}
```

Rust will reject this function with the following error:

```text
error[E0597]: `n` does not live long enough
 --> src/lib.rs:2:13
  |
1 | fn add_ref(v: &mut Vec<&i32>, n: i32) {
  |                        - let's call the lifetime of this reference `'1`
2 |     let r = &n;
  |             ^^ borrowed value does not live long enough
3 |     v.push(r);
  |     --------- argument requires that `n` is borrowed for `'1`
4 | }
  |  - `n` dropped here while still borrowed
```

The argument `n` only lives for the duration of `add_ref`. However, the reference `r` is being pushed into `v`, and `v` lives longer than `add_ref`. Therefore Rust complains that the data (`n`) does not outlive all of its references (`r`).

If this function were allowed, we could call `add_ref` like this:

```rust,ignore
let mut v = Vec::new();
add_ref(&mut v, 0);
println!("{}", v[0]);
```

Then `v` would contain a reference that points to deallocated memory, and printing `v[0]` would violate memory safety.


### Rust's Standard Library Can Help Convince the Borrow Checker That an Operation is Actually Safe

Understanding the distinction between safe and unsafe uses of references will help you deal with Rust compiler errors. Safe operations can often be rewritten using data structures designed to work around Rust's limitations. For example, you could rewrite the `x += 1` program without reordering the code by using a [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html): 

```rust
# use std::cell::Cell;
# fn main() {
let x: Cell<i32> = Cell::new(1);
let y = &x;
x.set(x.get() + 1);
println!("{} {}", x.get(), y.get());
# }
```

The `Cell` structure is carefully designed such that `Cell::set` "pretends" to the borrow checker that it is only reading `x` (it expects `&Cell` and not `&mut Cell`), while in fact it actually mutates `x`. A `Cell` is purely a compiler trick: a `Cell<i32>` takes the same space as `i32`, and the operation `x += 1` generates the same code as `x.set(x.get() + 1)`.

Notably, this pretension is only safe because `Cell` is also carefully designed to not be shared across threads. For example, if you needed to share references across a thread boundary:

```rust,ignore,does_not_compile
# use std::cell::Cell;
# use std::thread;
# fn main() {
let x: Cell<i32> = Cell::new(1);
let y = &x;
thread::scope(|s| { s.spawn(|| println!("{}", y.get())); })
x.set(x.get() + 1);
# }
```

Then Rust will reject your program, saying "`Cell<i32>` cannot be shared between threads safely". In this case, you would need to use a data structure specifically designed for sharing across threads, such as [`AtomicI32`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI32.html):

```rust
# use std::sync::atomic::{Ordering, AtomicI32};
# use std::thread;
# fn main() {
let x: AtomicI32 = AtomicI32::new(1);
let y = &x;
thread::scope(|s| { s.spawn(|| println!("{}", y.load(Ordering::SeqCst))); });
x.fetch_add(1, Ordering::SeqCst);
# }
```

The `Atomici32` structure is similar to `Cell<i32>`, except it uses special ["atomic"](https://doc.rust-lang.org/std/sync/atomic/index.html) operations that avoid data races. Therefore an atomic structure can be shared across threads. We will cover both of these constructs in detail in later chapters. For now, the takeaway is that there are constructs specialized for safely working around limitations of the borrow checker.


### ...But Actually Unsafe Operations Require Deeper Changes

Unsafe operations cannot generally be fixed with a straightforward rewrite. For example, in the case of `v.push(4)`, if a vector owns some data, you can never hold a reference to that data while also resizing the vector. There is no space-and-time-equivalent trick like `Cell` to fix such a program.

One possibility is to **shorten the lifetime of references**. For example, by moving the reference `&v[0]` after `v.push(4)`:

```rust
# fn main() {
let mut v = vec![1, 2, 3];
v.push(4);
let n = &v[0];
println!("{:?}, {}", v, *n);
# }
```

Another possibility is to **copy data instead of pointing to it.** For numbers, this is as simple as removing the ampersand:

```rust
# fn main() {
let mut v = vec![1, 2, 3];
let n = v[0];
v.push(4);
println!("{:?}, {}", v, n);
# }
```

If `v` held strings, then you would have to use the "clone" method like `v[0].clone()` instead. The larger your data, the more expensive it gets to clone it.

Finally, if you simply must hold a reference to an owned value while also mutating its container, **you can fall back on garbage collection**, such as with the reference-counting pointer [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html):

```rust
# use std::rc::Rc;
# fn main() {
let mut v: Vec<Rc<i32>> = vec![Rc::new(1), Rc::new(2), Rc::new(3)];
let n = Rc::clone(&v[0]);
v.push(Rc::new(4));
println!("{:?}, {}", v, *n);
# }
```



The right solution will naturally depend on the specific circumstances of your codebase. But at the very least, whenever you get an error from Rust's borrow checker, you should ask: is my code actually unsafe? The answer will guide how you can change your code to satisfy Rust.

### Summary

References provide the ability to read and write data without consuming ownership of it. References are created with borrows (`&` and `&mut`) and used with dereferences (`*`), sometimes implicitly.

However, references can be easily misused, so Rust provides a system of permissions to verify that programs use references safely:

- All variables can read, own, and (optionally) write their data.
- Creating a reference will transfer permissions from the borrowed path to the reference.
- Permissions are returned once the reference's lifetime has ended.
- Data must outlive all references that point to it.

In this section, it probably feels like we've described more of what Rust _cannot_ do than what Rust _can_ do. That is intentional! One of Rust's core features is enabling you to use references safely but without garbage collection. Understanding these safety rules now will help you avoid frustration with the compiler down the road.
