## `RefCell<T>` and the Interior Mutability Pattern

*Interior mutability* is a design pattern in Rust for allowing you to mutate
data that's immutable. The interior mutability pattern involves using `unsafe`
code inside a data structure to bend Rust's usual rules around mutation and
borrowing. We haven't yet covered unsafe code, we will in Chapter 20. The
`unsafe` code is then wrapped in a safe API, and the outer type is still
immutable.

Let's explore this by looking at the `RefCell<T>` type that follows the
interior mutability pattern.

### `RefCell<T>` has Interior Mutability

Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data
that it holds. So, what makes `RefCell<T>` different than a type like `Box<T>`?
Let's recall the borrowing rules we learned in Chapter 4:

1. At any given time, you can have *either* but not both of:
  * One mutable reference.
  * Any number of immutable references.
2. References must always be valid.

With references and `Box<T>`, the borrowing rules' invariants are enforced at
compile time. With `RefCell<T>`, these invariants are enforced *at runtime*.
With references, if you break these rules, you'll get a compiler error. With
`RefCell<T>`, if you break these rules, you'll get a `panic!`.

Static analysis, like the Rust compiler performs, is inherently conservative.
That is, if Rust accepts an incorrect program, people would not be able to
trust in the guarantees Rust makes. If Rust rejects a correct program, the
programmer will be inconvenienced, but nothing catastrophic can occur.
`RefCell<T>` is useful in two situations:

1. When you know that the borrowing rules are respected, but when the compiler
  can't understand that that's true.
2. When you need interior mutability.

Similarly to `Rc<T>`, `RefCell<T>` is only for use in single-threaded
scenarios. We'll talk about how to get the functionality of `RefCell<T>` in a
multithreaded program in the next chapter on concurrency. For now, all you
need to know is that if you try to use `RefCell<T>` in a multithreaded
context, you'll get a compile time error.

With references, we use the `&` and `&mut` syntax to create references and
mutable references, respectively. But with `RefCell<T>`, we use the `borrow`
and `borrow_mut` methods, which are part of the safe API that `RefCell<T>` has.

Listing 15-11 shows what it looks like to use `RefCell<T>` with functions that
borrow their parameters immutably and mutably. Note that the `data` variable is
declared as immutable with `let data` rather than `let mut data`, yet
`a_fn_that_mutably_borrows` is allowed to borrow the data mutably and make
changes to the data!

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
use std::cell::RefCell;

fn a_fn_that_immutably_borrows(a: &i32) {
    println!("a is {}", a);
}

fn a_fn_that_mutably_borrows(b: &mut i32) {
    *b += 1;
}

fn main() {
    let data = RefCell::new(5);
    a_fn_that_immutably_borrows(&data.borrow());
    a_fn_that_mutably_borrows(&mut data.borrow_mut());
    a_fn_that_immutably_borrows(&data.borrow());
}
```

<figcaption>

Listing 15-11: Using `RefCell<T>`, `borrow`, and `borrow_mut`

</figcaption>
</figure>

This example prints:

```text
a is 5
a is 6
```

Here, we've created a new `RefCell<T>` containing the value 5. We can get an
immutable reference to the value inside the `RefCell<T>` by calling the `borrow`
method. More interestingly, we can get a mutable reference to the value inside
the `RefCell<T>` with the `borrow_mut` method, and the function
`a_fn_that_mutably_borrows` is allowed to change the value. We can see that the
next time we print out the value, it's 6 instead of 5.

Recall that because of the borrowing rules, this code trying to create two
mutable borrows in the same scope won't compile:

```rust,ignore
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

We'll get this compiler error:

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 -->
  |
5 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
6 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
7 | }
  | - first borrow ends here
```

In contrast, using `RefCell<T>` and calling `borrow_mut` twice in the same
scope *will* compile, but it will panic at runtime instead. This code:

```rust,ignore
use std::cell::RefCell;

fn main() {
    let mut s = RefCell::new(String::from("hello"));

    let r1 = s.borrow_mut();
    let r2 = s.borrow_mut();
}
```

compiles but panics with the following error when we `cargo run`:

```text
    Finished dev [unoptimized + debuginfo] target(s) in 0.83 secs
     Running `target/debug/refcell`
thread 'main' panicked at 'already borrowed: BorrowMutError',
/stable-dist-rustc/build/src/libcore/result.rs:868
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This runtime `BorrowMutError` is similar to the compiler error: it says we've
already borrowed `s` mutably once, so we're not allowed to borrow it again. We
aren't getting around the borrowing rules, we're just choosing to have Rust
enforce them at runtime instead of compile time. You could choose to use
`RefCell<T>` everywhere all the time, but in addition to having to type
`RefCell` a lot, you'd find out about possible problems later (possibly in
production rather than during development). Also, checking the borrowing rules
while your program is running has a performance penalty.

### Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

So why would we choose to make the tradeoffs that using `RefCell<T>` involves?
Well, remember when we said that `Rc<T>` has to store immutable data? Given
that `RefCell<T>` is immutable, but has interior mutability, we can combine
`Rc<T>` and `RefCell<T>` to get a type that's both reference counted and
mutable. Listing 15-12 shows an example of how to do that, again going back to
our cons list from Listing 15-9. In this example, the type we're filling in for
`T` is `Rc<RefCell<i32>>`:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Cons(value.clone(), Rc::new(Nil));
    let shared_list = Rc::new(a);

    let b = Cons(Rc::new(RefCell::new(6)), shared_list.clone());
    let c = Cons(Rc::new(RefCell::new(10)), shared_list.clone());

    *value.borrow_mut() += 10;

    println!("shared_list after = {:?}", shared_list);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

<figcaption>

Listing 15-12: Using `Rc<RefCell<T>>` to create a `List<T>` that we can mutate

</figcaption>
</figure>

We're creating a value, which is an instance of `T`. We're storing it in a
variable named `value` because we want to be able to access it directly later.
Then we create a `List<T>` in `a` that has a `Cons` variant that holds `value`,
and `value` needs to be cloned since we want `value` to also have ownership in
addition to `a`. Then we wrap `a` in an `Rc<T>` so that we can create lists `b`
and `c` that start differently but both refer to `a`, similarly to what we did
in Listing 15-9.

Once we have the lists in `shared_list`, `b`, and `c` created, then we add 10
to the 5 in `value` by dereferencing the `Rc<T>` and calling `borrow_mut` on
the `RefCell`.

When we print out `shared_list`, `b`, and `c`, we can see that they all have
the modified value of 15:

```text
shared_list after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
```

This is pretty neat! We didn't have to modify our definition of `List<T>` at
all, since the generic parameter lets us substitute any immutable type. By
using `RefCell<T>`, we satisfy the immutable type requirement since
`RefCell<T>` is outwardly immutable, but we can use the methods on `RefCell<T>`
that provide access to its interior mutability to be able to modify our data
when we need to. The runtime checks of the borrowing rules that `RefCell<T>`
does protect us from data races, and we've decided that we want to trade a bit
of speed for the flexibility in our data structures.

`RefCell<T>` is not the only standard library type that provides interior
mutability. `Cell<T>` is similar but instead of giving references to the inner
value like `RefCell<T>` does, the value is copied in and out of the `Cell<T>`.
`Mutex<T>` offers interior mutability that is safe to use across threads, and
we'll be discussing its use in the next chapter on concurrency. Check out the
standard library docs for more details on the differences between these types.
