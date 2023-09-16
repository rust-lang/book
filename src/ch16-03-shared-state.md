## Shared-State Concurrency

Message passing is a fine way of handling concurrency, but it’s not the only
one. Another method would be for multiple threads to access the same shared
data. Consider this part of the slogan from the Go language documentation
again: “do not communicate by sharing memory.”

What would communicating by sharing memory look like? In addition, why would
message-passing enthusiasts caution not to use memory sharing?

In a way, channels in any programming language are similar to single ownership,
because once you transfer a value down a channel, you should no longer use that
value. Shared memory concurrency is like multiple ownership: multiple threads
can access the same memory location at the same time. As you saw in Chapter 15,
where smart pointers made multiple ownership possible, multiple ownership can
add complexity because these different owners need managing. Rust’s type system
and ownership rules greatly assist in getting this management correct. For an
example, let’s look at mutexes, one of the more common concurrency primitives
for shared memory.

### Using Mutexes to Allow Access to Data from One Thread at a Time

*Mutex* is an abbreviation for *mutual exclusion*, as in, a mutex allows only
one thread to access some data at any given time. To access the data in a
mutex, a thread must first signal that it wants access by asking to acquire the
mutex’s *lock*. The lock is a data structure that is part of the mutex that
keeps track of who currently has exclusive access to the data. Therefore, the
mutex is described as *guarding* the data it holds via the locking system.

Mutexes have a reputation for being difficult to use because you have to
remember two rules:

* You must attempt to acquire the lock before using the data.
* When you’re done with the data that the mutex guards, you must unlock the
  data so other threads can acquire the lock.

For a real-world metaphor for a mutex, imagine a panel discussion at a
conference with only one microphone. Before a panelist can speak, they have to
ask or signal that they want to use the microphone. When they get the
microphone, they can talk for as long as they want to and then hand the
microphone to the next panelist who requests to speak. If a panelist forgets to
hand the microphone off when they’re finished with it, no one else is able to
speak. If management of the shared microphone goes wrong, the panel won’t work
as planned!

Management of mutexes can be incredibly tricky to get right, which is why so
many people are enthusiastic about channels. However, thanks to Rust’s type
system and ownership rules, you can’t get locking and unlocking wrong.

#### The API of `Mutex<T>`

As an example of how to use a mutex, let’s start by using a mutex in a
single-threaded context, as shown in Listing 16-12:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

<span class="caption">Listing 16-12: Exploring the API of `Mutex<T>` in a
single-threaded context for simplicity</span>

As with many types, we create a `Mutex<T>` using the associated function `new`.
To access the data inside the mutex, we use the `lock` method to acquire the
lock. This call will block the current thread so it can’t do any work until
it’s our turn to have the lock.

The call to `lock` would fail if another thread holding the lock panicked. In
that case, no one would ever be able to get the lock, so we’ve chosen to
`unwrap` and have this thread panic if we’re in that situation.

After we’ve acquired the lock, we can treat the return value, named `num` in
this case, as a mutable reference to the data inside. The type system ensures
that we acquire a lock before using the value in `m`. The type of `m` is
`Mutex<i32>`, not `i32`, so we *must* call `lock` to be able to use the `i32`
value. We can’t forget; the type system won’t let us access the inner `i32`
otherwise.

As you might suspect, `Mutex<T>` is a smart pointer. More accurately, the call
to `lock` *returns* a smart pointer called `MutexGuard`, wrapped in a
`LockResult` that we handled with the call to `unwrap`. The `MutexGuard` smart
pointer implements `Deref` to point at our inner data; the smart pointer also
has a `Drop` implementation that releases the lock automatically when a
`MutexGuard` goes out of scope, which happens at the end of the inner scope. As
a result, we don’t risk forgetting to release the lock and blocking the mutex
from being used by other threads, because the lock release happens
automatically.

After dropping the lock, we can print the mutex value and see that we were able
to change the inner `i32` to 6.

#### Sharing a `Mutex<T>` Between Multiple Threads

Now, let’s try to share a value between multiple threads using `Mutex<T>`.
We’ll spin up 10 threads and have them each increment a counter value by 1, so
the counter goes from 0 to 10. The next example in Listing 16-13 will have
a compiler error, and we’ll use that error to learn more about using
`Mutex<T>` and how Rust helps us use it correctly.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

<span class="caption">Listing 16-13: Ten threads each increment a counter
guarded by a `Mutex<T>`</span>

We create a `counter` variable to hold an `i32` inside a `Mutex<T>`, as we did
in Listing 16-12. Next, we create 10 threads by iterating over a range of
numbers. We use `thread::spawn` and give all the threads the same closure: one
that moves the counter into the thread, acquires a lock on the `Mutex<T>` by
calling the `lock` method, and then adds 1 to the value in the mutex. When a
thread finishes running its closure, `num` will go out of scope and release the
lock so another thread can acquire it.

In the main thread, we collect all the join handles. Then, as we did in Listing
16-2, we call `join` on each handle to make sure all the threads finish. At
that point, the main thread will acquire the lock and print the result of this
program.

We hinted that this example wouldn’t compile. Now let’s find out why!

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

The error message states that the `counter` value was moved in the previous
iteration of the loop. Rust is telling us that we can’t move the ownership
of lock `counter` into multiple threads. Let’s fix the compiler error with a
multiple-ownership method we discussed in Chapter 15.

#### Multiple Ownership with Multiple Threads

In Chapter 15, we gave a value multiple owners by using the smart pointer
`Rc<T>` to create a reference counted value. Let’s do the same here and see
what happens. We’ll wrap the `Mutex<T>` in `Rc<T>` in Listing 16-14 and clone
the `Rc<T>` before moving ownership to the thread.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

<span class="caption">Listing 16-14: Attempting to use `Rc<T>` to allow
multiple threads to own the `Mutex<T>`</span>

Once again, we compile and get... different errors! The compiler is teaching us
a lot.

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

Wow, that error message is very wordy! Here’s the important part to focus on:
`` `Rc<Mutex<i32>>` cannot be sent between threads safely ``. The compiler is
also telling us the reason why: ``the trait `Send` is not implemented for
`Rc<Mutex<i32>>` ``. We’ll talk about `Send` in the next section: it’s one of
the traits that ensures the types we use with threads are meant for use in
concurrent situations.

Unfortunately, `Rc<T>` is not safe to share across threads. When `Rc<T>`
manages the reference count, it adds to the count for each call to `clone` and
subtracts from the count when each clone is dropped. But it doesn’t use any
concurrency primitives to make sure that changes to the count can’t be
interrupted by another thread. This could lead to wrong counts—subtle bugs that
could in turn lead to memory leaks or a value being dropped before we’re done
with it. What we need is a type exactly like `Rc<T>` but one that makes changes
to the reference count in a thread-safe way.

#### Atomic Reference Counting with `Arc<T>`

Fortunately, `Arc<T>` *is* a type like `Rc<T>` that is safe to use in
concurrent situations. The *a* stands for *atomic*, meaning it’s an *atomically
reference counted* type. Atomics are an additional kind of concurrency
primitive that we won’t cover in detail here: see the standard library
documentation for [`std::sync::atomic`][atomic]<!-- ignore --> for more
details. At this point, you just need to know that atomics work like primitive
types but are safe to share across threads.

You might then wonder why all primitive types aren’t atomic and why standard
library types aren’t implemented to use `Arc<T>` by default. The reason is that
thread safety comes with a performance penalty that you only want to pay when
you really need to. If you’re just performing operations on values within a
single thread, your code can run faster if it doesn’t have to enforce the
guarantees atomics provide.

Let’s return to our example: `Arc<T>` and `Rc<T>` have the same API, so we fix
our program by changing the `use` line, the call to `new`, and the call to
`clone`. The code in Listing 16-15 will finally compile and run:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

<span class="caption">Listing 16-15: Using an `Arc<T>` to wrap the `Mutex<T>`
to be able to share ownership across multiple threads</span>

This code will print the following:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Result: 10
```

We did it! We counted from 0 to 10, which may not seem very impressive, but it
did teach us a lot about `Mutex<T>` and thread safety. You could also use this
program’s structure to do more complicated operations than just incrementing a
counter. Using this strategy, you can divide a calculation into independent
parts, split those parts across threads, and then use a `Mutex<T>` to have each
thread update the final result with its part.

Note that if you are doing simple numerical operations, there are types simpler
than `Mutex<T>` types provided by the [`std::sync::atomic` module of the
standard library][atomic]<!-- ignore -->. These types provide safe, concurrent,
atomic access to primitive types. We chose to use `Mutex<T>` with a primitive
type for this example so we could concentrate on how `Mutex<T>` works.

### Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`

You might have noticed that `counter` is immutable but we could get a mutable
reference to the value inside it; this means `Mutex<T>` provides interior
mutability, as the `Cell` family does. In the same way we used `RefCell<T>` in
Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>`
to mutate contents inside an `Arc<T>`.

Another detail to note is that Rust can’t protect you from all kinds of logic
errors when you use `Mutex<T>`. Recall in Chapter 15 that using `Rc<T>` came
with the risk of creating reference cycles, where two `Rc<T>` values refer to
each other, causing memory leaks. Similarly, `Mutex<T>` comes with the risk of
creating *deadlocks*. These occur when an operation needs to lock two resources
and two threads have each acquired one of the locks, causing them to wait for
each other forever. If you’re interested in deadlocks, try creating a Rust
program that has a deadlock; then research deadlock mitigation strategies for
mutexes in any language and have a go at implementing them in Rust. The
standard library API documentation for `Mutex<T>` and `MutexGuard` offers
useful information.

We’ll round out this chapter by talking about the `Send` and `Sync` traits and
how we can use them with custom types.

[atomic]: ../std/sync/atomic/index.html
