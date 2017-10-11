## Shared State Concurrency

Message passing is a fine way of dealing with concurrency, but it’s not the
only one. Consider this slogan again:

> Do not communicate by sharing memory; instead, share memory by
> communicating.

What would “communicate by sharing memory” look like? And moreover, why would
message passing enthusiasts choose not to use it and do the opposite instead?

<!-- Can you expand here? I wasn't sure where we were getting the idea that
message passers hated and inverted memory sharing -->
<!-- I'm not sure where you got "hate" from :) I've tried to reword. We're
getting this idea from the slogan that the Go programming language espouses
that we discussed earlier /Carol -->

In a way, channels in any programming language are sort of like single
ownership, because once you transfer a value down a channel, you shouldn’t use
that value any longer. Shared memory concurrency is sort of like multiple
ownership: multiple threads can access the same memory location at the same
time. As we saw in Chapter 15 where multiple ownership was made possible by
smart pointers, multiple ownership can add additional complexity because these
different owners need managing.

Rust’s type system and ownership rules assist a lot in getting this management
correct, though. For an example, let’s look at one of the more common
concurrency primitives for shared memory: mutexes.

### Mutexes Allow Access to Data from One Thread at a Time

A *mutex* is a concurrency primitive for sharing memory. It’s short for “mutual
exclusion”, as in, it only allows one thread to access some data at any given
time. In order to access the data in a mutex, a thread must first signal that
it wants access by asking to acquire the mutex’s *lock*. The lock is a data
structure that is part of the mutex that keeps track of who currently has
exclusive access to the data. We therefore describe the mutex as *guarding* the
data it holds via the locking system.

Mutexes have a reputation for being hard to use because there are some
rules you have to remember:

<!-- below -- what is the lock, here? Can you define that outright? And make it
clear that the mutex is the guard? -->
<!-- I've added definitions/explanations above /Carol -->

1. You must attempt to acquire the lock before using the data.
2. Once you’re done with the data that’s guarded by the mutex, you must unlock
   the data so other threads can acquire the lock.

For a real-world metaphor of a mutex, imagine a panel discussion at a
conference with only one microphone. Before a panelist may speak, they have to
ask or signal that they would like to use the microphone. Once they get the
microphone, they may talk for as long as they would like, then hand the
microphone to the next panelist who requests to speak. If a panelist forgets to
hand the microphone off when they’re finished with it, no one else is able to
speak. If management of the shared microphone goes wrong, the panel would not
work as planned!

Management of mutexes can be incredibly tricky to get right, and that’s why so
many people are enthusiastic about channels. However, thanks to Rust’s type
system and ownership rules, we can’t get locking and unlocking wrong.

#### The API of `Mutex<T>`

Let’s start simply with an example of using a mutex in a single-threaded
context, shown in Listing 16-12:

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

As with many types, we create a `Mutex<T>` using the associated function `new`.
To access the data inside the mutex, we use the `lock` method to acquire the
lock. This call will block the current thread so that it can’t do any work
until it’s our turn to have the lock.

<!-- will block what, other requests for the lock? Or block access to the data?
-->
<!-- This is where I hope our earlier definition of "block" that I added will
help; I've also reworded to reinforce that /Carol -->

The call to `lock` would fail if another thread holding the lock panicked. In
that case, no one would ever be able to get the lock, so we’ve chosen to
`unwrap` and have this thread panic if we’re in that situation.

<!-- As in, the lock would be released? What would failure look like? -->
<!-- As in we wouldn't ever be able to get the lock, I've clarified /Carol -->

Once we’ve acquired the lock, we can treat the return value, named `num` in
this case, as a mutable reference to the data inside. The type system ensures
that we acquire a lock before using this value: `Mutex<i32>` is not an `i32`,
so we *must* acquire the lock in order to be able to use the `i32` value. We
can’t forget; the type system won’t let us do it otherwise.

As you may suspect, `Mutex<T>` is a smart pointer. More accurately, the call to
`lock` *returns* a smart pointer called `MutexGuard`. This smart pointer
implements `Deref` to point at our inner data, and also has a `Drop`
implementation that releases the lock automatically when `MutexGuard` goes out
of scope, which happens at the end of the inner scope in Listing 16-12. This
way, we don’t risk forgetting to release the lock and blocking it from use by
other threads, because it happens automatically.

After dropping the lock, we can print out the mutex value and see that we were
able to change the inner `i32` to 6.

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

<span class="caption">Listing 16-13: Ten threads each increment a counter
guarded by a `Mutex<T>`</span>

We’re creating a `counter` variable to hold an `i32` inside a `Mutex<T>`, like
we did in Listing 16-12. Next, we’re creating 10 threads by mapping over a
range of numbers. We use `thread::spawn` and give all the threads the same
closure, one that moves the counter into the thread, acquires a lock on the
`Mutex<T>` by calling the `lock` method, and then adds 1 to the value in the
mutex. When a thread finishes running its closure, `num` will go out of scope
and release the lock so another thread can acquire it.

In the main thread, we collect all the join handles like we did in Listing
16-2, and then call `join` on each to make sure all the threads finish. At that
point, the main thread will acquire the lock and print out the result of this
program.

We hinted that this example won’t compile, now let’s find out why!

<!-- Hm, since we already saw this error, where we need to include move, maybe
we could skip it here and just include move in the initial program, to focus
more on the new error and new concepts -- what do you think? -->
<!-- Ok, cut! /Carol -->

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

The error message is saying that the `counter` value is moved into the closure,
then is captured when we call `lock`. That sounds like what we wanted, but it’s
not allowed!

Let’s reason this out by simplifying the program. Instead of making 10 threads
in a `for` loop, let’s just make two threads without a loop and see what
happens then. Replace the first `for` loop in Listing 16-13 with this code
instead:

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

We make two threads and change the variable names used with the second thread
to `handle2` and `num2`. When we run this time, compiling gives us:

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

Aha! The first error message tells us that `counter` is moved into the closure
for the thread associated with `handle`. That move is preventing us from
capturing `counter` when we try to call `lock` on it and store the result in
`num2` in the second thread! So Rust is telling us that we can’t move ownership
of `counter` into multiple threads. This was hard to see before because our
threads were in a loop, and Rust can’t point to different threads in different
iterations of the loop. Let’s try to fix this with a multiple-ownership method
we saw in Chapter 15.

#### Multiple Ownership with Multiple Threads

In Chapter 15, we were able to give a value multiple owners by using the smart
pointer `Rc<T>` to create a reference-counted value. Let’s try to do the same
here and see what happens. We’ll wrap the `Mutex<T>` in `Rc<T>` in Listing
16-14, and clone the `Rc<T>` before moving ownership to the thread. Now we’ve
seen the errors, we’ll also switch back to using the `for` loop, and we’ll keep
the `move` keyword with the closure:

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

Wow, that’s quite wordy! Here are some important parts to pick out: the first
note says `Rc<Mutex<i32>> cannot be sent between threads safely`. The reason
for this is in the error message, which, once distilled, says `the trait bound
Send is not satisfied`. We’re going to talk about `Send` in the next section;
it’s one of the traits that ensures the types we use with threads are meant for
use in concurrent situations.

<!-- Maybe we need to save this discussion until after talking about Send?
Otherwise, you might expand on this, what is the reader taking away here? -->
<!-- The reader should take away that we can't use `Rc<T>` with threads, and
we're not sure how to point that out without mentioning `Send`. /Carol -->

Unfortunately, `Rc<T>` is not safe to share across threads. When `Rc<T>`
manages the reference count, it adds to the count for each call to `clone` and
subtracts from the count when each clone is dropped, but it doesn’t use any
concurrency primitives to make sure that changes to the count can’t be
interrupted by another thread. This could lead to wrong counts: subtle bugs
that could in turn lead to memory leaks or a value being dropped before we’re
done with it. What we need is a type exactly like `Rc<T>`, but that makes
changes to the reference count in a thread-safe way.

#### Atomic Reference Counting with `Arc<T>`

Luckily for us, there *is* a type like `Rc<T>` that’s safe to use in concurrent
situations: `Arc<T>`. The ‘a’ stands for *atomic*, meaning it’s an *atomically
reference counted* type. Atomics are an additional kind of concurrency
primitive that we won’t cover in detail here; see the standard library
documentation for `std::sync::atomic` for more details. What you need to know
here is that atomics work like primitive types, but are safe to share across
threads.

You might then wonder why all primitive types aren’t atomic, and why standard
library types aren’t implemented to use `Arc<T>` by default. The reason is that
thread safety comes with a performance penalty that you only want to pay when
you really need to. If you’re only doing operations on values within a single
thread, your code can run faster if it doesn’t have to enforce the guarantees
atomics provide.

Back to our example: `Arc<T>` and `Rc<T>` have the same API, so we fix our
program by changing the `use` line and the call to `new`. The code in Listing
16-15 will finally compile and run:

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

We did it! We counted from 0 to 10, which may not seem very impressive, but it
did teach us a lot about `Mutex<T>` and thread safety! This structure could
also be used to do more complicated operations than just incrementing a
counter: these methods allow us to divide calculations up into independent
parts, which we could split across threads, and then we can use a `Mutex<T>` to
have each thread update the final result with its part.

### Similarities between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`

You may have noticed that `counter` is immutable but we could get a mutable
reference to the value inside it; this means `Mutex<T>` provides interior
mutability, like the `Cell` family does. In the same way we used `RefCell<T>`
in Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use
`Mutex<T>` to mutate contents inside of an `Arc<T>`.

Another thing to note is that Rust can’t prevent us from all kinds of logic
errors when using `Mutex<T>`. Recall from Chapter 15 that using `Rc<T>` came
with the risk of creating reference cycles, where two `Rc<T>` values refer to
each other, causing memory leaks. Similarly, `Mutex<T>` comes the risk of
*deadlocks*. These occur when an operation needs to lock two resources and two
threads have each acquired one of the locks, causing them to wait for each
other forever. If you’re interested in this topic, try creating a Rust program
that has a deadlock, then research deadlock mitigation strategies for mutexes
in any language, and have a go at implementing them in Rust. The standard
library API documentation for `Mutex<T>` and `MutexGuard` will have useful
information.

<!--Rust's type system and ownership has made sure that our threads have
exclusive access to the shared value when they're updating it, so the threads
won't overwrite each other's answers in unpredictable ways. It took us a while
to work with the compiler to get everything right, but we've saved future time
that might be spent trying to reproduce subtly incorrect scenarios that only
happen when the threads run in a particular order.-->
<!-- Feel free to contradict me, but I think this has come across in the
chapters, I'm suggesting cutting just to keep focus, keep it moving -->
<!-- We're tentatively okay with cutting this, but again we want to convince
people who are skeptical that dealing with ownership is worth it /Carol -->

Let’s round out this chapter by talking about the `Send` and `Sync` traits and
how we could use them with custom types.
