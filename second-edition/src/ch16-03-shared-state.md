## Shared State Concurrency

While message passing is a fine way of dealing with concurrency, it's not the
only one. Consider the slogan again:

> Do not communicate by sharing memory; instead, share memory by
> communicating.

What would "communicate by sharing memory" look like? And moreover, why would
message passing enthusiasts dislike it, and dislike it enough to invert it
entirely?

Remember how channels are sort of like single ownership? Shared memory
concurrency is sort of like multiple ownership: multiple threads can access the
same memory location at the same time. As we saw with ownership, this can add
additional complexity, as we need to manage these different owners somehow.

Ownership can help a lot here, though. For example, let's look at one of the
more common concurrency primitives for shared memory: mutexes.

### Mutexes Allow Access to One Thread at a Time

A *mutex* is a concurrency primitive for sharing memory. It's short for "mutual
exclusion", that is, it only allows one thread to access some data at any given
time. Mutexes have a reputation as being hard to use, because there's a lot you
have to remember:

1. You have to remember to actually attempt to acquire the lock.
2. One you're done with the memory that's being guarded by the mutex, you have
   to remember to unlock the memory.

This can be incredibly tricky to get right, and that's why so many are
enthusiastic about channels. However, in Rust, you cannot get these two points
wrong, thanks to our secret power again, ownership. Let's see an example:

```rust
use std::sync::Mutex;

let m = Mutex::new(5);

let mut num = m.lock().unwrap();

*num = 6;
```

Like many types, we create a `Mutex<T>` through a `new` method. To access
the data inside the mutex, we use the `lock` method to acquire the lock. This
call will block until we're able to do so. This call can fail, so similar to
the last section, we use `unwrap()` for now, rather than better error handling.
See Chapter 9 for better tools.

Once we have acquired the lock, we can treat its return value, named `num` in
this case, as a mutable reference to the data inside. This is how the first
problem with mutexes is solved; `Mutex<i32>` is not an `i32`, and so we _must_
acquire the lock in order to use this memory. We can't forget; the type system
won't let us do otherwise.

As you may have suspected, `Mutex<T>` is a smart pointer. Well, more
accurately, the call to `lock` returns a smart pointer, called `MutexGuard`.
This smart pointer implements `Deref` to point at our inner data, similar to
the other smart pointers we saw in the last chapter. In addition, it has a
`Drop` implementation that releases the lock. This is the secret to solving
problem number two with mutexes: we can't forget to release the lock. It
happens for us automatically when the `MutexGuard` goes out of scope.

Let's try to use `Mutex<T>` in an example. We'll spin up ten threads, and have
them each increment a counter. This example won't _quite_ work yet:

```rust,ignore
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);

    let guards: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut num = counter.lock().unwrap();

            *num += 1;
        })
    }).collect();

    for guard in guards {
        guard.join().unwrap();
    }

    println!("Answer: {}", *counter.lock().unwrap());
}
```

Here's the error:

```text
error[E0373]: closure may outlive the current function, but it borrows `**counter`, which is owned by the current function
 --> <anon>:8:23
  |
8 |         thread::spawn(|| {
  |                       ^^ may outlive borrowed value `**counter`
9 |             let mut num = counter.lock().unwrap();
  |                           ------- `**counter` is borrowed here
  |
help: to force the closure to take ownership of `**counter` (and any other referenced variables), use the `move` keyword, as shown:
  |         thread::spawn(move || {
```

Given that we spin up multiple threads, Rust can't know how long the threads
will run. As such, we can't borrow the mutex. We need something else. The help
message has a clue: we could use `move` to give ownership to each thread. Let's
try it:

```text
error[E0382]: use of moved value: `counter`
  --> <anon>:19:29
   |
7  |     let guards: Vec<_> = (0..10).map(|_| {
   |                                      --- value moved (into closure) here
...
19 |     println!("Answer: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`, which does not implement the `Copy` trait

error[E0507]: cannot move out of captured outer variable in an `FnMut` closure
 --> <anon>:8:23
  |
8 |         thread::spawn(move || {
  |                       ^^^^^^^ cannot move out of captured outer variable in an `FnMut` closure
```

Ouch! That doesn't work either. We can't give away ownership to multiple
threads. We need a different solution. Well, in the last section, we heard
about multiple ownership with `Rc<T>` Let's try that? Here's the code:


```rust,ignore
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));

    let guards: Vec<_> = (0..10).map(|_| {
	let counter = counter.clone();

        thread::spawn(|| {
            let mut num = counter.lock().unwrap();

            *num += 1;
        })
    }).collect();

    for guard in guards {
        guard.join().unwrap();
    }

    println!("Answer: {}", *counter.lock().unwrap());
}
```

This doesn't quite work either. Here's the error:

```text
error[E0277]: the trait bound `std::rc::Rc<std::sync::Mutex<i32>>: std::marker::Sync` is not satisfied
  --> <anon>:11:9
   |
11 |         thread::spawn(|| {
   |         ^^^^^^^^^^^^^ the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
   |
   = note: `std::rc::Rc<std::sync::Mutex<i32>>` cannot be shared between threads safely
   = note: required because of the requirements on the impl of `std::marker::Send` for `&std::rc::Rc<std::sync::Mutex<i32>>`
   = note: required because it appears within the type `[closure@<anon>:11:23: 15:10 counter:&std::rc::Rc<std::sync::Mutex<i32>>]`
   = note: required by `std::thread::spawn`
```

... buried in the middle of all of those types is the answer: `Rc<Mutex<i32>>`
cannot be shared between threads safely. Ugh! So close.

Why isn't `Rc<T>` safe to share across threads? Well, if you recall, when we
deal with the reference count, we have to add and subtract for each call to
`clone` and when they go out of scope. This isn't threadsafe, since `Rc<T>`
doesn't use any concurrency primitives to make sure that the count is safe.
This could lead to subtle bugs where the counts are wrong, and we accidentally
leak memory or free it too early. So, what if we had a type that was exactly
like `Rc<T>`, but made these counts thread-safe?

If you thought that question sounded like a leading one, you'd be right. There
is: `Arc<T>`. The 'a' stands for 'atomic', so it's an 'atomically reference
counted' type. Atomics are an additional kind of concurrency primitive that we
won't cover here, see the standard library documentation for `std::atomic` for
more. The gist of it is this: they work like primitive types, but are safe to
share across threads. Why isn't this safety the default? Because it comes with
a performance penalty.

Anyway, `Arc<T>` and `Rc<T>` are identical except for the atomic internals. The
API is the same, so we can change our import line, and change our call to
`new`, and now it works:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let guards: Vec<_> = (0..10).map(|_| {
	let counter = counter.clone();

        thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        })
    }).collect();

    for guard in guards {
        guard.join().unwrap();
    }

    println!("Answer: {}", *counter.lock().unwrap());
}
```

We did it!

You may notice that this means that `Mutex<T>` provides interior mutability,
like the `Cell` family does. In the same way that we use `RefCell<T>` to be
able to mutate contents inside an `Rc<T>`, we use `Mutex<T>` to be able to
mutate contents inside of an `Arc<T>`.
