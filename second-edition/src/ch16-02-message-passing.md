## Message Passing to Transfer Data Between Threads

One approach to concurrency that’s seen a rise in popularity as of late is
*message passing*, where threads or actors communicate by sending each other
messages containing data. Here’s the idea in slogan form:

> Do not communicate by sharing memory; instead, share memory by
> communicating.
>
> --[Effective Go](http://golang.org/doc/effective_go.html)

A major tool to accomplish this goal is the *channel*. A channel has two
halves, a transmitter and a receiver. One part of our code can call methods on
the transmitter with the data we want to send, and another part can check the
receiving end for arriving messages.

We’re going to work up to an example where we have one thread that will
generate values and send them down a channel. The main thread will receive the
values and print them out.

First, though, let’s start by creating a channel but not doing anything with it
in Listing 16-6:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
#     tx.send(()).unwrap();
}
```

<span class="caption">Listing 16-6: Creating a channel and assigning the two
halves to `tx` and `rx`</span>

The `mpsc::channel` function creates a new channel. `mpsc` stands for *multiple
producer, single consumer*. In short, we can have multiple *sending* ends of a
channel that produce values, but only one *receiving* end that consumes those
values. We’re going to start with a single producer for now, but we’ll add
multiple producers once we get this example working.

`mpsc::channel` returns a tuple: the first element is the sending end, and the
second element is the receiving end. For historical reasons, many people use
`tx` and `rx` to abbreviate *transmitter* and *receiver*, so those are the
names we’re using for the variables bound to each end. We’re using a `let`
statement with a pattern that destructures the tuples; we’ll be discussing the
use of patterns in `let` statements and destructuring in Chapter 18.

Let’s move the transmitting end into a spawned thread and have it send one
string, shown in Listing 16-7:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
```

<span class="caption">Listing 16-7: Moving `tx` to a spawned thread and sending
“hi”</span>

We’re using `thread::spawn` to create a new thread, just as we did in the
previous section. We use a `move` closure to make `tx` move into the closure so
that the thread owns it.

The transmitting end of a channel has the `send` method that takes the value we
want to send down the channel. The `send` method returns a `Result<T, E>` type,
because if the receiving end has already been dropped, there’s nowhere to send
a value to, so the send operation would error. In this example, we’re simply
calling `unwrap` to ignore this error, but for a real application, we’d want to
handle it properly. Chapter 9 is where you’d go to review strategies for proper
error handling.

In Listing 16-8, let’s get the value from the receiving end of the channel in
the main thread:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

<span class="caption">Listing 16-8: Receiving the value “hi” in the main thread
and printing it out</span>

The receiving end of a channel has two useful methods: `recv` and `try_recv`.
Here, we’re using `recv`, which is short for *receive*. This method will block
execution until a value is sent down the channel. Once a value is sent, `recv`
will return it in a `Result<T, E>`. When the sending end of the channel closes,
`recv` will return an error. The `try_recv` method will not block; it instead
returns a `Result<T, E>` immediately.

If we run the code in Listing 16-8, we’ll see the value printed out from the
main thread:

```text
Got: hi
```

### How Channels Interact with Ownership

Let’s do an experiment at this point to see how channels and ownership work
together: we’ll try to use `val` in the spawned thread after we’ve sent it down
the channel. Try compiling the code in Listing 16-9:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

<span class="caption">Listing 16-9: Attempting to use `val` after we have sent
it down the channel</span>

Here, we try to print out `val` after we’ve sent it down the channel via
`tx.send`. This is a bad idea: once we’ve sent the value to another thread,
that thread could modify it or drop it before we try to use the value again.
This could cause errors or unexpected results due to inconsistent or
nonexistent data.

If we try to compile this code, Rust will error:

```text
error[E0382]: use of moved value: `val`
  --> src/main.rs:10:31
   |
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val is {}", val);
   |                               ^^^ value used here after move
   |
   = note: move occurs because `val` has type `std::string::String`, which does
   not implement the `Copy` trait
```

Our concurrency mistake has caused a compile-time error! `send` takes ownership
of its parameter and moves the value so that the value is owned by the
receiver. This means we can’t accidentally use the value again after sending
it; the ownership system checks that everything is okay.

In this regard, message passing is very similar to single ownership in Rust.
Message passing enthusiasts enjoy message passing for similar reasons that
Rustaceans enjoy Rust’s ownership: single ownership means certain classes of
problems go away. If only one thread at a time can use some memory, there’s no
chance of a data race.

### Sending Multiple Values and Seeing the Receiver Waiting

The code in Listing 16-8 compiled and ran, but it wasn’t very interesting: it’s
hard to see that we have two separate threads talking to each other over a
channel. Listing 16-10 has some modifications that will prove to us that this
code is running concurrently: the spawned thread will now send multiple
messages and pause for a second between each message.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

<span class="caption">Listing 16-10: Sending multiple messages and pausing
between each one</span>

This time, we have a vector of strings in the spawned thread that we want to
send to the main thread. We iterate over them, sending each individually and
then pausing by calling the `thread::sleep` function with a `Duration` value of
one second.

In the main thread, we’re not calling the `recv` function explicitly anymore:
instead we’re treating `rx` as an iterator. For each value received, we’re
printing it out. When the channel is closed, iteration will end.

When running the code in Listing 16-10, we’ll see this output, with a one second
pause in between each line:

```text
Got: hi
Got: from
Got: the
Got: thread
```

We don’t have any pausing or code that would take a while in the `for` loop in
the main thread, so we can tell that the main thread is waiting to receive
values from the spawned thread.

### Create Multiple Producers by Cloning the Transmitter

Near the start of this section, we mentioned that `mpsc` stood for *multiple
producer, single consumer*. We can expand the code from Listing 16-10 to create
multiple threads that all send values to the same receiver. We do that by
cloning the transmitting half of the channel, as shown in Listing 16-11:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::sync::mpsc;
# use std::time::Duration;
#
# fn main() {
// ...snip...
let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});
// ...snip...
#
#     for received in rx {
#         println!("Got: {}", received);
#     }
# }
```

<span class="caption">Listing 16-11: Sending multiple messages and pausing
between each one</span>

This time, before we create the first spawned thread, we call `clone` on the
sending end of the channel. This will give us a new sending handle that we can
pass to the first spawned thread. We’ll pass the original sending end of the
channel to a second spawned thread, and each thread is sending different
messages to the receiving end of the channel.

If you run this, you’ll *probably* see output like this:

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

You might see the values in a different order, though. It depends on your
system! This is what makes concurrency interesting as well as difficult. If you
play around with `thread::sleep`, giving it different values in the different
threads, you can make the runs more non-deterministic and create different
output each time.

Now that we’ve seen how channels work, let’s look at shared-memory concurrency.
