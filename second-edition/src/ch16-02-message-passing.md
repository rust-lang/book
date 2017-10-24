## Message Passing to Transfer Data Between Threads

One increasingly popular approach to ensuring safe concurrency is *message
passing*, where threads or actors communicate by sending each other messages
containing data. Here’s the idea in slogan form from the Go language
documentation:

> Do not communicate by sharing memory; instead, share memory by
> communicating.
>
> --[Effective Go](http://golang.org/doc/effective_go.html)

<!-- below -- what is the channel, precisely? A crate? a technique?-->
<!-- I've elaborated /Carol -->

One major tool Rust has for accomplishing message sending concurrency is the
*channel*, a programming concept that Rust’s standard library provides an
implementation of. You can imagine a channel in programming like a channel of
water, such as a stream or a river. If you put something like a rubber duck or
a boat into a stream, it will travel downstream to the end of the river.

A channel in programming has two halves: a transmitter and a receiver. The
transmitter half is like the upstream location where we put rubber ducks into
the river, and the receiver half is the downstream place where the rubber duck
ends up. One part of our code calls methods on the transmitter with the data we
want to send, and another part checks the receiving end for arriving messages.

Here we’ll work up to a program that has one thread to generate values and send
them down a channel, and another thread that will receive the values and print
them out. We’re going to be sending simple values between threads using a
channel for the purposes of illustration. Once you’re familiar with the
technique, you could use channels to implement a chat system, or a system where
many threads perform parts of a calculation and send the parts to one thread
that aggregates the results.

First, we’ll create a channel but not do anything with it in Listing 16-6:

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

We create a new channel using the `mpsc::channel` function; `mpsc` stands for
*multiple producer, single consumer*. In short, the way Rust’s standard library
has implemented channels is such that a channel can have multiple *sending*
ends that produce values, but only one *receiving* end that consumes those
values. Imagine multiple rivers and streams flowing together into one big
river: everything sent down any of the streams will end up in one river at the
end. We’re going to start with a single producer for now, but we’ll add
multiple producers once we get this example working.

The `mpsc::channel` function returns a tuple, the first element of which is the
sending end and the second element the receiving end. The abbreviations `tx`
and `rx` are traditionally used in many fields for *transmitter* and *receiver*
respectively, so we give our variables those names to indicate each end. We’re
using a `let` statement with a pattern that destructures the tuples; we’ll be
discussing the use of patterns in `let` statements and destructuring in Chapter
18. Using a `let` statement in this way is a convenient way to extract the
pieces of the tuple returned by `mpsc::channel`.

<!-- above -- can you give us a general idea of what that means for us in this
program? -->
<!-- A general idea of what *what* means? I'm not sure what you're asking for;
I've added a bit of explanation of the destructuring but I'm not sure that's
what you meant /Carol -->

Let’s move the transmitting end into a spawned thread and have it send one
string so that the spawned thread is communicating with the main thread, shown
in Listing 16-7. This is like putting a rubber duck in the river upstream or
sending a chat message from one thread to another:

<!-- Can you tell us why we want to do this, what does this do for us and our
program? -->
<!-- Elaborated /Carol -->

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

We’re again using `thread::spawn` to create a new thread, and then use `move`
to move `tx` into the closure so the spawned thread owns `tx`. The spawned
thread needs to own the transmitting end of the channel in order to be able to
send messages through the channel.

The transmitting end has a `send` method that takes the value we want to send.
The `send` method returns a `Result<T, E>` type, so that if the receiving end
has already been dropped and there’s nowhere to send a value, the send
operation will error. In this example, we’re simply calling `unwrap` to panic
in case of error, but for a real application, we’d handle it properly--return
to Chapter 9 to review strategies for proper error handling.

In Listing 16-8, we’ll get the value from the receiving end of the channel in
the main thread. This is like retrieving the rubber duck from the water at the
end of the river, or like getting a chat message:

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
We’re using `recv`, short for *receive*, which will block the main thread’s
execution and wait until a value is sent down the channel. Once a value is
sent, `recv` will return it in a `Result<T, E>`. When the sending end of the
channel closes, `recv` will return an error to signal that no more values will
be coming.

<!-- Why do we want it to error when the sending end closes? And what's the
advantage of blocking here? -->
<!-- We don't necessarily *want* it to error, that's just how the standard
library has implemented it. I've tried to clarify that and blocking. /Carol -->

The `try_recv` method doesn’t block, but will instead return a `Result<T, E>`
immediately: an `Ok` value holding a message if one is available, and an `Err`
value if there aren’t any messages this time. Using `try_recv` is useful if
this thread has other work to do while waiting for messages: we could write a
loop that calls `try_recv` every so often, handles a message if one is
available, and otherwise does other work for a little while until checking
again.

We’ve chosen to use `recv` in this example for simplicity; we don’t have any
other work for the main thread to do other than wait for messages, so blocking
the main thread is appropriate.

<!-- So what is the difference here, what are the different situations you
would want to return the value immediately? -->
<!-- Elaborated above /Carol -->

If we run the code in Listing 16-8, we’ll see the value printed out from the
main thread:

```text
Got: hi
```

Perfect!

### Channels and Ownership Transference

<!-- Hmm i'm not sure we need as it's own section, it seems like it could be
condensed now the reader is pretty familiar with ownership rules. We might not
even need the example, but I'll defer to you on this -->
<!-- We think the examples in this section are important-- there are likely
readers who are skeptical about Rust's ownership system and whether it's worth
putting up with, and this section is the payoff. We're trying to convince those
people that one big benefit Rust's ownership gives you is that it enables you
to write safe, concurrent code. I'm glad we've convinced *you*, but I'm not so
sure that we'll have convinced all our readers at this point! I've tried to
state this a bit more clearly without calling out these readers too
explicitly... /Carol -->

The ownership rules play a vital role in message sending as far as helping us
write safe, concurrent code. Preventing errors in concurrent programming is the
advantage we get by making the tradeoff of having to think about ownership
throughout our Rust programs. Let’s do an experiment to show how channels and
ownership work together to prevent problems: we’ll try to use a `val` value in
the spawned thread *after* we’ve sent it down the channel. Try compiling the
code in Listing 16-9:

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
`tx.send`. Allowing this would be a bad idea: once the value has been sent to
another thread, that thread could modify or drop it before we try to use the
value again, which would potentially cause errors or unexpected results due to
inconsistent or nonexistent data.

However, Rust gives us an error if we try to compile this code:

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

Our concurrency mistake has caused a compile-time error! The `send` function
takes ownership of its parameter, and when the value is moved the receiver
takes ownership of it. This stops us from accidentally use the value again
after sending it; the ownership system checks that everything is okay.

### Sending Multiple Values and Seeing the Receiver Waiting

The code in Listing 16-8 compiled and ran, but doesn’t show us very clearly
that two separate threads are talking to each other over the channel. In
Listing 16-10 we’ve made some modifications that will prove this code is
running concurrently: the spawned thread will now send multiple messages and
pause for a second between each message.

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

This time, the spawned thread has a vector of strings that we want to send to
the main thread. We iterate over them, sending each individually, and pause
between each by calling the `thread::sleep` function with a `Duration` value of
one second.

In the main thread, we’re not calling the `recv` function explicitly anymore:
instead we’re treating `rx` as an iterator. For each value received, we’re
printing it out. When the channel is closed, iteration will end.

When running the code in Listing 16-10, you should see the following output,
with a one second pause in between each line:

```text
Got: hi
Got: from
Got: the
Got: thread
```

Because we don’t have any code that pauses or delays in the `for` loop in the
main thread, we can tell that the main thread is waiting to receive values from
the spawned thread.

<!-- Above -- just to be clear, this is because the main thread is receiving
the pauses from the spawned thread, is that right? -->
<!-- It's not that we're sending the *pauses*, it's that the spawned thread is
pausing between sending each message, so then the main thread doesn't receive
all the messages all at the same time, it receives one message per second
because that's how they were sent. I'm not sure what in the code looks like
"sending/receiving the pauses" or what isn't clear here, so I'm not sure how to
fix it /Carol -->

### Creating Multiple Producers by Cloning the Transmitter

Near the start of this section, we mentioned that `mpsc` stood for *multiple
producer, single consumer*. Let’s put that ability to use and expand the code
from Listing 16-10 to create multiple threads that all send values to the same
receiver. We can do that by cloning the transmitting half of the channel, as
shown in Listing 16-11:

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
sending end of the channel. This will give us a new sending handle we can pass
to the first spawned thread. We pass the original sending end of the channel to
a second spawned thread. This gives us two threads, each sending different
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

You might see the values in a different order, it depends on your system! This
is what makes concurrency interesting as well as difficult. If you play around
with `thread::sleep`, giving it different values in the different threads, each
run will be more non-deterministic and create different output each time.

Now that we’ve seen how channels work, let’s look at a different method of
concurrency.
