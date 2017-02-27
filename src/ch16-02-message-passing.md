# Message Passing

One approach to concurrency that's seen a rise in popularity as of late is
*message passing*, where threads or actors communicate by sending each other
messages. Here's the idea in slogan form:

> Do not communicate by sharing memory; instead, share memory by
> communicating.
>
> --[Effective Go](http://golang.org/doc/effective_go.html)

A major tool to accomplish this goal is the *channel*. Channels look like
this:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

tx.send(String::from("Hello!")).unwrap();

println!("{:?}", rx.recv().unwrap());
```

The `mpsc::channel` function crates a new channel. What's up with 'mpsc'?
It stands for "multiple producer, single consumer." In short, you can have
multiple *sending* ends of a channel, but only one *receiving* end.

`mpsc::channel` returns a tuple: the first element is the sending end, the
second element is the receiving end. For historical reasons, many people use
`rx` and `tx` to abbreviate 'transmitter' and 'receiver', and so those are the
names we're using for the variables bound to each end.

The `tx` end has one useful method: `send`. In our example, we send a `String`
down the channel. Doing this returns a `Result<T, E>` type, because if the
receiving end has already been deallocated, there's nowhere to send it to, and
so we'd get an error. In this example, we're simply calling `unwrap` to ignore
this error, but for a real application, we'd want to handle it properly.
Chapter 9 is where you'd go to review strategies for proper error handling.

The `rx` end has two useful methods: `recv` and `try_recv`. 'recv' is short for
'receive'. Here, we're using the former. This method will block execution until
a value is sent down the channel. Once one is, it will return it. The `try_`
variant of `recv` will not block; it instead returns a `Result<T, E>`
immediately.

## Channels and ownership

These basic aspects of channels are the same in every language: send something
down one end, receive it on the other end. Where Rust is special, though, is
ownership. Consider this example, which is slightly modified from the above
one:

```rust,ignore
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
let message = String::from("Hello!");

tx.send(message).unwrap();

println!("Sent {}", message);

println!("{:?}", rx.recv().unwrap());
```

Here, we try to print out `message` after we've sent it down the channel via
`tx.send`. If you try to compile this code, Rust will error:

```text
error[E0382]: use of moved value: `message`
 --> <anon>:9:21
  |
7 | tx.send(message).unwrap();
  |         ------- value moved here
8 |
9 | println!("Sent {}", message);
  |                     ^^^^^^^ value used here after move
  |
  = note: move occurs because `message` has type `std::string::String`, which does not implement the `Copy` trait
```

Our concurrency mistake has caused a compile-time error! `send` takes its
argument by value, and so moves whatever it's passed. This means we can't
accidentally use it again; the ownership system checks that everything is okay.

In this regard, message passing is very similar to single ownership in Rust.
Message passing enthusiasts enjoy it for similar reasons as Rustaceans enjoy
Rust; single ownership means certain classes of problems go away. If only one
thread at a time can own some memory, there's no chance of getting a data race.

## An example

Let's show off a slightly more realistic example. Let's make a thread that
generates numbers. It'll then send the numbers down a channel. The main thread
will receive the numbers and print them out.

Let's start without any threads:

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        tx.send(i).unwrap();
    }

    drop(tx);

    for num in rx {
        println!("got number: {}", num);
    }
}
```

We've already covered `use` statements in previous chapters, and we covered
creating `tx` and `rx` earlier in this chapter. Let's look at the first new
part:

```rust,ignore
for i in 0..10 {
    tx1.send(i).unwrap();
}
```

Instead of sending one number, we're sending ten numbers. We've seen loops
before, we've seen the `send` call before, this is putting the two together.

```rust,ignore
drop(tx);
```

Here's the first truly new thing in this example. In the last chapter, we
talked about the `Drop` trait and how it cleans up resources. Well, when we're
done with a channel, the sender's `Drop` implementation notifies any receivers
that we're done sending data. As such, we can use the `drop` function to do
this before the variable goes out of scope. Why can't we simply wait until `tx`
goes out of scope? The key to answering that question is in the last line:

```rust,ignore
for num in rx {
    println!("got number: {}", num);
}
```

The receiving end of channels can be used as an iterator. This `for` loop
will keep going until the sending end finishes. If we didn't call `drop`,
we'd be waiting in this loop forever, as `tx` hasn't left scope yet!

Since we've explicitly hung up the sending end, though, this works. Give
it a try:

```text
$ cargo run
got number: 0
got number: 1
got number: 2
got number: 3
got number: 4
got number: 5
got number: 6
got number: 7
got number: 8
got number: 9
```

Great! Things are working. But this isn't using any concurrency. Let's add
in a thread, and run our number generation inside of it:

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });

    for num in rx {
        println!("got number: {}", num);
    }
}
```

There are two changes here. The first is using `thread::spawn` to create a new
thread. We use a `move` closure to make `tx` move into the closure, which
explains our second difference: we no longer need an explicit `drop` of `tx`.

If we run this example, we should see the same results as before. We've
introduced a thread, but we don't see any evidence that the thread is running
in paralell to the main thread.

We can show that this is happening at the same time with a call
`thread::sleep`:

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();

	    thread::sleep(Duration::new(1, 0));
        }
    });

    for num in rx {
        println!("got number: {}", num);
    }
}
```

The call to `thread::sleep` will make the thread stop executing for the
`Duration` it's been passed. In this case, we've created a duration of one
second. When running this version of the example, you'll see the same output,
but with a one-second pause in between. There's no pausing in our recieving
loop, so we can tell that the main thread is waiting on the secondary thread.

Near the start of this section, we mentioned that 'mpsc' stood for "multiple
producer, single consumer." We can expand our example to show this behavior
off. Check it out:

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 0..10 {
            tx1.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        for i in 10..20 {
            tx.send(i).unwrap();
        }
    });

    for num in rx {
        println!("got number: {}", num);
    }
}
```

Here, we call `clone` on the sending end of the channel. This will give us a
new sending handle, which we can pass to a new thread. So we create two
threads, pass the two sending ends of the channel to them, and generate
different numbers for each.

If you run this, you'll *probably* see output like this:

```text
got number: 0
got number: 1
got number: 2
got number: 3
got number: 4
got number: 5
got number: 6
got number: 7
got number: 8
got number: 9
got number: 10
got number: 11
got number: 12
got number: 13
got number: 14
got number: 15
got number: 16
got number: 17
got number: 18
got number: 19
```

You also may see the two-digit numbers first, or you may see them interleaved.
It depends on your system! This is what makes concurrency interesting, but also
difficult. If you play around with `thread::sleep`, giving it different values,
you can make the runs more non-deterministic, and get different output each
time. Here's an example that worked on my computer:

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 0..10 {
            tx1.send(i).unwrap();

            if i % 2 == 0 {
                thread::sleep(Duration::new(0, 10));
            }
        }
    });

    thread::spawn(move || {
        for i in 10..20 {
            tx.send(i).unwrap();

            if i % 2 == 1 {
                thread::sleep(Duration::new(0, 20));
            }
        }
    });

    for num in rx {
        println!("got number: {}", num);
    }
}
```

## Summary

Now that we've seen how channels work, let's look at shared-memory concurrency.
