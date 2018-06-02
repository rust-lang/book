# Concurrency

Concurrency and parallelism are incredibly important topics in computer
science, and are also a hot topic in industry today. Computers are gaining more
and more cores, yet many programmers aren't prepared to fully utilize them.

Rust's memory safety features also apply to its concurrency story. Even
concurrent Rust programs must be memory safe, having no data races. Rust's type
system is up to the task, and gives you powerful ways to reason about
concurrent code at compile time.

Before we talk about the concurrency features that come with Rust, it's important
to understand something: Rust is low-level enough that the vast majority of
this is provided by the standard library, not by the language. This means that
if you don't like some aspect of the way Rust handles concurrency, you can
implement an alternative way of doing things.
[mio](https://github.com/carllerche/mio) is a real-world example of this
principle in action.

## Background: `Send` and `Sync`

Concurrency is difficult to reason about. In Rust, we have a strong, static
type system to help us reason about our code. As such, Rust gives us two traits
to help us make sense of code that can possibly be concurrent.

### `Send`

The first trait we're going to talk about is
[`Send`](../../std/marker/trait.Send.html). When a type `T` implements `Send`, it
indicates that something of this type is able to have ownership transferred
safely between threads.

This is important to enforce certain restrictions. For example, if we have a
channel connecting two threads, we would want to be able to send some data
down the channel and to the other thread. Therefore, we'd ensure that `Send` was
implemented for that type.

In the opposite way, if we were wrapping a library with [FFI][ffi] that isn't
thread-safe, we wouldn't want to implement `Send`, and so the compiler will help
us enforce that it can't leave the current thread.

[ffi]: ffi.html

### `Sync`

The second of these traits is called [`Sync`](../../std/marker/trait.Sync.html).
When a type `T` implements `Sync`, it indicates that something
of this type has no possibility of introducing memory unsafety when used from
multiple threads concurrently through shared references. This implies that
types which don't have [interior mutability](mutability.html) are inherently
`Sync`, which includes simple primitive types (like `u8`) and aggregate types
containing them.

For sharing references across threads, Rust provides a wrapper type called
`Arc<T>`. `Arc<T>` implements `Send` and `Sync` if and only if `T` implements
both `Send` and `Sync`. For example, an object of type `Arc<RefCell<U>>` cannot
be transferred across threads because
[`RefCell`](choosing-your-guarantees.html#refcellt) does not implement
`Sync`, consequently `Arc<RefCell<U>>` would not implement `Send`.

These two traits allow you to use the type system to make strong guarantees
about the properties of your code under concurrency. Before we demonstrate
why, we need to learn how to create a concurrent Rust program in the first
place!

## Threads

Rust's standard library provides a library for threads, which allow you to
run Rust code in parallel. Here's a basic example of using `std::thread`:

```rust
use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Hello from a thread!");
    });
}
```

The `thread::spawn()` method accepts a [closure](closures.html), which is executed in a
new thread. It returns a handle to the thread, that can be used to
wait for the child thread to finish and extract its result:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        "Hello from a thread!"
    });

    println!("{}", handle.join().unwrap());
}
```

As closures can capture variables from their environment, we can also try to
bring some data into the other thread:

```rust,ignore
use std::thread;

fn main() {
    let x = 1;
    thread::spawn(|| {
        println!("x is {}", x);
    });
}
```

However, this gives us an error:

```text
5:19: 7:6 error: closure may outlive the current function, but it
                 borrows `x`, which is owned by the current function
...
5:19: 7:6 help: to force the closure to take ownership of `x` (and any other referenced variables),
          use the `move` keyword, as shown:
      thread::spawn(move || {
          println!("x is {}", x);
      });
```

This is because by default closures capture variables by reference, and thus the
closure only captures a _reference to `x`_. This is a problem, because the
thread may outlive the scope of `x`, leading to a dangling pointer.

To fix this, we use a `move` closure as mentioned in the error message. `move`
closures are explained in depth [here](closures.html#move-closures); basically
they move variables from their environment into themselves.

```rust
use std::thread;

fn main() {
    let x = 1;
    thread::spawn(move || {
        println!("x is {}", x);
    });
}
```

Many languages have the ability to execute threads, but it's wildly unsafe.
There are entire books about how to prevent errors that occur from shared
mutable state. Rust helps out with its type system here as well, by preventing
data races at compile time. Let's talk about how you actually share things
between threads.

## Safe Shared Mutable State

Due to Rust's type system, we have a concept that sounds like a lie: "safe
shared mutable state." Many programmers agree that shared mutable state is
very, very bad.

Someone once said this:

> Shared mutable state is the root of all evil. Most languages attempt to deal
> with this problem through the 'mutable' part, but Rust deals with it by
> solving the 'shared' part.

The same [ownership system](ownership.html) that helps prevent using pointers
incorrectly also helps rule out data races, one of the worst kinds of
concurrency bugs.

As an example, here is a Rust program that would have a data race in many
languages. It will not compile:

```rust,ignore
use std::thread;
use std::time::Duration;

fn main() {
    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        thread::spawn(move || {
            data[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
```

This gives us an error:

```text
8:17 error: capture of moved value: `data`
        data[0] += i;
        ^~~~
```

Rust knows this wouldn't be safe! If we had a reference to `data` in each
thread, and the thread takes ownership of the reference, we'd have three owners!
`data` gets moved out of `main` in the first call to `spawn()`, so subsequent
calls in the loop cannot use this variable.

So, we need some type that lets us have more than one owning reference to a
value. Usually, we'd use `Rc<T>` for this, which is a reference counted type
that provides shared ownership. It has some runtime bookkeeping that keeps track
of the number of references to it, hence the "reference count" part of its name.

Calling `clone()` on an `Rc<T>` will return a new owned reference and bump the
internal reference count. We create one of these for each thread:


```rust,ignore
use std::thread;
use std::time::Duration;
use std::rc::Rc;

fn main() {
    let mut data = Rc::new(vec![1, 2, 3]);

    for i in 0..3 {
        // Create a new owned reference:
        let data_ref = data.clone();

        // Use it in a thread:
        thread::spawn(move || {
            data_ref[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
```

This won't work, however, and will give us the error:

```text
13:9: 13:22 error: the trait bound `alloc::rc::Rc<collections::vec::Vec<i32>> : core::marker::Send`
            is not satisfied
...
13:9: 13:22 note: `alloc::rc::Rc<collections::vec::Vec<i32>>`
            cannot be sent between threads safely
```

As the error message mentions, `Rc` cannot be sent between threads safely. This
is because the internal reference count is not maintained in a thread-safe
manner and can have a data race.

To solve this, we'll use `Arc<T>`, Rust's standard atomic reference count type.

The Atomic part means `Arc<T>` can safely be accessed from multiple threads.
To do this the compiler guarantees that mutations of the internal count use
indivisible operations which can't have data races.

In essence, `Arc<T>` is a type that lets us share ownership of data _across
threads_.


```rust,ignore
use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let mut data = Arc::new(vec![1, 2, 3]);

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            data[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
```

Similarly to last time, we use `clone()` to create a new owned handle.
This handle is then moved into the new thread.

And... still gives us an error.

```text
<anon>:11:24 error: cannot borrow immutable borrowed content as mutable
<anon>:11                    data[0] += i;
                             ^~~~
```

`Arc<T>` by default has immutable contents. It allows the _sharing_ of data
between threads, but shared mutable data is unsafe—and when threads are
involved—can cause data races!


Usually when we wish to make something in an immutable position mutable, we use
`Cell<T>` or `RefCell<T>` which allow safe mutation via runtime checks or
otherwise (see also: [Choosing Your Guarantees](choosing-your-guarantees.html)).
However, similar to `Rc`, these are not thread-safe. If we try using these, we
will get an error about these types not being `Sync`, and the code will fail to
compile.

It looks like we need some type that allows us to safely mutate a shared value
across threads, for example a type that can ensure only one thread at a time is
able to mutate the value inside it at any one time.

For that, we can use the `Mutex<T>` type!

Here's the working version:

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
```

Note that the value of `i` is bound (copied) to the closure and not shared
among the threads.

We're "locking" the mutex here. A mutex (short for "mutual exclusion"), as
mentioned, only allows one thread at a time to access a value. When we wish to
access the value, we use `lock()` on it. This will "lock" the mutex, and no
other thread will be able to lock it (and hence, do anything with the value)
until we're done with it. If a thread attempts to lock a mutex which is already
locked, it will wait until the other thread releases the lock.

The lock "release" here is implicit; when the result of the lock (in this case,
`data`) goes out of scope, the lock is automatically released.

Note that [`lock`](../../std/sync/struct.Mutex.html#method.lock) method of
[`Mutex`](../../std/sync/struct.Mutex.html) has this signature:

```rust,ignore
fn lock(&self) -> LockResult<MutexGuard<T>>
```

and because `Send` is not implemented for `MutexGuard<T>`, the guard cannot
cross thread boundaries, ensuring thread-locality of lock acquire and release.

Let's examine the body of the thread more closely:

```rust
# use std::sync::{Arc, Mutex};
# use std::thread;
# use std::time::Duration;
# fn main() {
#     let data = Arc::new(Mutex::new(vec![1, 2, 3]));
#     for i in 0..3 {
#         let data = data.clone();
thread::spawn(move || {
    let mut data = data.lock().unwrap();
    data[0] += i;
});
#     }
#     thread::sleep(Duration::from_millis(50));
# }
```

First, we call `lock()`, which acquires the mutex's lock. Because this may fail,
it returns a `Result<T, E>`, and because this is just an example, we `unwrap()`
it to get a reference to the data. Real code would have more robust error handling
here. We're then free to mutate it, since we have the lock.

Lastly, while the threads are running, we wait on a short timer. But
this is not ideal: we may have picked a reasonable amount of time to
wait but it's more likely we'll either be waiting longer than
necessary or not long enough, depending on just how much time the
threads actually take to finish computing when the program runs.

A more precise alternative to the timer would be to use one of the
mechanisms provided by the Rust standard library for synchronizing
threads with each other. Let's talk about one of them: channels.

## Channels

Here's a version of our code that uses channels for synchronization, rather
than waiting for a specific time:

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    let data = Arc::new(Mutex::new(0));

    // `tx` is the "transmitter" or "sender".
    // `rx` is the "receiver".
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }
}
```

We use the `mpsc::channel()` method to construct a new channel. We `send`
a simple `()` down the channel, and then wait for ten of them to come back.

While this channel is sending a generic signal, we can send any data that
is `Send` over the channel!

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let tx = tx.clone();

        thread::spawn(move || {
            let answer = i * i;

            tx.send(answer).unwrap();
        });
    }

    for _ in 0..10 {
        println!("{}", rx.recv().unwrap());
    }
}
```

Here we create 10 threads, asking each to calculate the square of a number (`i`
at the time of `spawn()`), and then `send()` back the answer over the channel.


## Panics

A `panic!` will crash the currently executing thread. You can use Rust's
threads as a simple isolation mechanism:

```rust
use std::thread;

let handle = thread::spawn(move || {
    panic!("oops!");
});

let result = handle.join();

assert!(result.is_err());
```

`Thread.join()` gives us a `Result` back, which allows us to check if the thread
has panicked or not.
