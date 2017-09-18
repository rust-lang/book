## Shared State Concurrency

While message passing is a fine way of dealing with concurrency, it’s not the
only one. Consider this slogan again:

> Do not communicate by sharing memory; instead, share memory by
> communicating.

What would “communicate by sharing memory” look like? And moreover, why would
message passing enthusiasts dislike it, and dislike it enough to invert it
entirely?

Remember how channels are sort of like single ownership? Shared memory
concurrency is sort of like multiple ownership: multiple threads can access the
same memory location at the same time. As we saw with multiple ownership made
possible by smart pointers in Chapter 15, multiple ownership can add additional
complexity, since we need to manage these different owners somehow.

Rust’s type system and ownership can help a lot here in getting this management
correct, though. For an example, let’s look at one of the more common
concurrency primitives for shared memory: mutexes.

### Mutexes Allow Access to Data from One Thread at a Time

A *mutex* is a concurrency primitive for sharing memory. It’s short for “mutual
exclusion”, that is, it only allows one thread to access some data at any given
time. Mutexes have a reputation for being hard to use, since there’s a lot you
have to remember:

1. You have to remember to attempt to acquire the lock before using the data.
2. Once you’re done with the data that’s being guarded by the mutex, you have
   to remember to unlock the data so that other threads can acquire the lock.

For a real-world example of a mutex, imagine a panel discussion at a conference
where there is only one microphone. Before a panelist may speak, they have to
ask or signal that they would like to use the microphone. Once they get the
microphone, they may talk for as long as they would like, then hand the
microphone to the next panelist who would like to speak. It would be rude for a
panelist to start shouting without having the microphone or to steal the
microphone before another panelist was finished. No one else would be able to
speak if a panelist forgot to hand the microphone to the next person when they
finished using it. If the management of the shared microphone went wrong in any
of these ways, the panel would not work as planned!

Management of mutexes can be incredibly tricky to get right, and that’s why so
many people are enthusiastic about channels. However, in Rust, we can’t get
locking and unlocking wrong, thanks to the type system and ownership.

#### The API of `Mutex<T>`

Let’s look at an example of using a mutex in Listing 16-12, without involving
multiple threads for the moment:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

<span class="caption">Listing 16-12: Exploring the API of `Mutex<T>` in a
single threaded context for simplicity</span>

Like many types, we create a `Mutex<T>` through an associated function named
`new`. To access the data inside the mutex, we use the `lock` method to acquire
the lock. This call will block until it’s our turn to have the lock. This call
can fail if another thread was holding the lock and then that thread panicked.
In a similar way as we did in Listing 16-6 in the last section, we’re using
`unwrap()` for now, rather than better error handling. See Chapter 9 for better
tools.

Once we have acquired the lock, we can treat the return value, named `num` in
this case, as a mutable reference to the data inside. The type system is how
Rust ensures that we acquire a lock before using this value: `Mutex<i32>` is
not an `i32`, so we *must* acquire the lock in order to be able to use the
`i32` value. We can’t forget; the type system won’t let us do otherwise.

As you may have suspected, `Mutex<T>` is a smart pointer. Well, more
accurately, the call to `lock` returns a smart pointer called `MutexGuard`.
This smart pointer implements `Deref` to point at our inner data, similar to
the other smart pointers we saw in Chapter 15. In addition, `MutexGuard` has a
`Drop` implementation that releases the lock. This way, we can’t forget to
release the lock. It happens for us automatically when the `MutexGuard` goes
out of scope, which it does at the end of the inner scope in Listing 16-12. We
can print out the mutex value and see that we were able to change the inner
`i32` to 6.

#### Sharing a `Mutex<T>` Between Multiple Threads

Let’s now try to share a value between multiple threads using `Mutex<T>`. We’ll
spin up ten threads, and have them each increment a counter value by 1 so that
the counter goes from 0 to 10. Note that the next few examples will have
compiler errors, and we’re going to use those errors to learn more about using
`Mutex<T>` and how Rust helps us use it correctly. Listing 16-13 has our
starting example:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(|| {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">Listing 16-13: The start of a program having 10 threads
each increment a counter guarded by a `Mutex<T>`</span>

We’re creating a `counter` variable to hold an `i32` inside a `Mutex<T>`, like
we did in Listing 16-12. Next, we’re creating 10 threads by mapping over a
range of numbers. We use `thread::spawn` and give all the threads the same
closure: they’re each going to acquire a lock on the `Mutex<T>` by calling the
`lock` method and then add 1 to the value in the mutex. When a thread finishes
running its closure, `num` will go out of scope and release the lock so that
another thread can acquire it.

In the main thread, we’re collecting all the join handles like we did in
Listing 16-2, and then calling `join` on each of them to make sure all the
threads finish. At that point, the main thread will acquire the lock and print
out the result of this program.

We hinted that this example won’t compile, let’s find out why!

```text
error[E0373]: closure may outlive the current function, but it borrows
`counter`, which is owned by the current function
  -->
   |
9  |         let handle = thread::spawn(|| {
   |                                    ^^ may outlive borrowed value `counter`
10 |             let mut num = counter.lock().unwrap();
   |                           ------- `counter` is borrowed here
   |
help: to force the closure to take ownership of `counter` (and any other
referenced variables), use the `move` keyword, as shown:
   |         let handle = thread::spawn(move || {
```

This is similar to the problem we solved in Listing 16-5. Given that we spin up
multiple threads, Rust can’t know how long the threads will run and whether
`counter` will still be valid when each thread tries to borrow it. The help
message has a reminder for how to solve this: we can use `move` to give
ownership to each thread. Let’s try it by making this change to the closure:

```rust,ignore
thread::spawn(move || {
```

And trying to compile again. We’ll get different errors this time!

```text
error[E0382]: capture of moved value: `counter`
  -->
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
10 |             let mut num = counter.lock().unwrap();
   |                           ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  -->
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

`move` didn’t fix this program like it fixed Listing 16-5. Why not? This error
message is a little confusing to read, because it’s saying that the `counter`
value is moved into the closure, then is captured when we call `lock`. That
sounds like what we wanted, but it’s not allowed.

Let’s reason this out. Instead of making 10 threads in a `for` loop, let’s just
make two threads without a loop and see what happens then. Replace the first
`for` loop in Listing 16-13 with this code instead:

```rust,ignore
let handle = thread::spawn(move || {
    let mut num = counter.lock().unwrap();

    *num += 1;
});
handles.push(handle);

let handle2 = thread::spawn(move || {
    let mut num2 = counter.lock().unwrap();

    *num2 += 1;
});
handles.push(handle2);
```

Here we’re making 2 threads, and we changed the variable names used with the
second thread to `handle2` and `num2`. We’re simplifying our example for the
moment to see if we can understand the error message we’re getting. This time,
compiling gives us:

```text
error[E0382]: capture of moved value: `counter`
  -->
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
16 |         let mut num2 = counter.lock().unwrap();
   |                        ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  -->
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
26 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

Aha! In the first error message, Rust is showing us that `counter` is moved
into the closure for the thread that goes with `handle`. That move is
preventing us from capturing `counter` when we try to call `lock` on it and
store the result in `num2`, which is in the second thread! So Rust is telling
us that we can’t move ownership of `counter` into multiple threads. This was
hard to see before since we were creating multiple threads in a loop, and Rust
can’t point to different threads in different iterations of the loop.

#### Multiple Ownership with Multiple Threads

In Chapter 15, we were able to have multiple ownership of a value by using the
smart pointer `Rc<T>` to create a reference-counted value. We mentioned in
Chapter 15 that `Rc<T>` was only for single-threaded contexts, but let’s try
using `Rc<T>` in this case anyway and see what happens. We’ll wrap the
`Mutex<T>` in `Rc<T>` in Listing 16-14, and clone the `Rc<T>` before moving
ownership to the thread. We’ll switch back to the `for` loop for creating the
threads, and keep the `move` keyword with the closure:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
    	let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">Listing 16-14: Attempting to use `Rc<T>` to allow
multiple threads to own the `Mutex<T>`</span>

Once again, we compile and get... different errors! The compiler is teaching us
a lot!

```text
error[E0277]: the trait bound `std::rc::Rc<std::sync::Mutex<i32>>:
std::marker::Send` is not satisfied
  -->
   |
11 |         let handle = thread::spawn(move || {
   |                      ^^^^^^^^^^^^^ the trait `std::marker::Send` is not
   implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
   |
   = note: `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads
   safely
   = note: required because it appears within the type
   `[closure@src/main.rs:11:36: 15:10
   counter:std::rc::Rc<std::sync::Mutex<i32>>]`
   = note: required by `std::thread::spawn`
```

Wow, that’s quite wordy! Some important parts to pick out: the first note says
`Rc<Mutex<i32>> cannot be sent between threads safely`. The reason for this is
in the error message, which, once distilled, says `the trait bound Send is not
satisfied`. We’re going to talk about `Send` in the next section; it’s one of
the traits that ensures the types we use with threads are meant for use in
concurrent situations.

Unfortunately, `Rc<T>` is not safe to share across threads. When `Rc<T>`
manages the reference count, it has to add to the count for each call to
`clone` and subtract from the count when each clone is dropped. `Rc<T>` doesn’t
use any concurrency primitives to make sure that changes to the count happen in
an operation that couldn’t be interrupted by another thread. This could lead to
subtle bugs where the counts are wrong, which could lead to memory leaks or
dropping a value before we’re done with it. So what if we had a type that was
exactly like `Rc<T>`, but made changes to the reference count in a thread-safe
way?

#### Atomic Reference Counting with `Arc<T>`

If you thought that question sounded like a leading one, you’d be right. There
is a type like `Rc<T>` that’s safe to use in concurrent situations: `Arc<T>`.
The ‘a’ stands for *atomic*, so it’s an *atomically reference counted* type.
Atomics are an additional kind of concurrency primitive that we won’t cover
here; see the standard library documentation for `std::sync::atomic` for more
details. The gist of it is this: atomics work like primitive types, but are
safe to share across threads.

Why aren’t all primitive types atomic, and why aren’t all standard library
types implemented to use `Arc<T>` by default? Thread safety comes with a
performance penalty that we only want to pay when we need it. If we’re only
doing operations on values within a single thread, our code can run faster
since it doesn’t need the guarantees that atomics give us.

Back to our example: `Arc<T>` and `Rc<T>` are identical except for the atomic
internals of `Arc<T>`. Their API is the same, so we can change the `use` line
and the call to `new`. The code in Listing 16-15 will finally compile and run:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
    	let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">Listing 16-15: Using an `Arc<T>` to wrap the `Mutex<T>`
to be able to share ownership across multiple threads</span>

This will print:

```text
Result: 10
```

We did it! We counted from 0 to 10, which may not seem very impressive, but we
learned a lot about `Mutex<T>` and thread safety along the way! The structure
that we’ve built in this example could be used to do more complicated
operations than just incrementing a counter. Calculations that can be divided
up into independent parts could be split across threads in this way, and we can
use a `Mutex<T>` to allow each thread to update the final result with its part.

You may have noticed that, since `counter` is immutable but we could get a
mutable reference to the value inside it, this means `Mutex<T>` provides
interior mutability, like the `Cell` family does. In the same way that we used
`RefCell<T>` in Chapter 15 to be able to mutate contents inside an `Rc<T>`, we
use `Mutex<T>` to be able to mutate contents inside of an `Arc<T>`.

Recall that `Rc<T>` did not prevent every possible problem: we also talked
about the possibility of creating reference cycles where two `Rc<T>` values
refer to each other, which would cause a memory leak. We have a similar problem
with `Mutex<T>` that Rust also doesn’t prevent: deadlocks. A *deadlock* is a
situation in which an operation needs to lock two resources, and two threads
have each acquired one of the locks and will now wait for each other forever.
If you’re interested in this topic, try creating a Rust program that has a
deadlock, then research deadlock mitigation strategies that apply to the use of
mutexes in any language and try implementing them in Rust. The standard library
API documentation for `Mutex<T>` and `MutexGuard` will have useful information.

Rust’s type system and ownership has made sure that our threads have exclusive
access to the shared value when they’re updating it, so the threads won’t
overwrite each other’s answers in unpredictable ways. It took us a while to
work with the compiler to get everything right, but we’ve saved future time
that might be spent trying to reproduce subtly incorrect scenarios that only
happen when the threads run in a particular order.

Let’s round out this chapter by talking about the `Send` and `Sync` traits and
how we could use them with custom types.
