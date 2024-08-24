> ## Restructuring
>
> This is just a placeholder for material that needs to be restructured so that
> the earlier sections of the book can avoid getting sidetracked into details of
> things like `Pin` or even just the full gnarliness of the `Future` trait at
> points where it would be better for the text to keep moving.

---

Here is the definition of the trait:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

As we learned earlier, `Future`’s associated type `Output` says what the future
will resolves to. (This is analogous to the `Item` associated type for the
`Iterator` trait.) Beyond that, `Future` also has the `poll` method, which takes
a special `Pin` reference for its `self` parameter and a mutable reference to a
`Context` type, and returns a `Poll<Self::Output>`. We will talk a little more
about `Pin` and `Context` later in the chapter. For now, let’s focus on what the
method returns, the `Poll` type:

```rust
enum Poll<T> {
    Ready(T),
    Pending
}
```

This `Poll` type is a lot like an `Option`: it has one variant which has a value
(`Ready(T)`), and one which does not (`Pending`). It means something quite
different, though! The `Pending` variant indicates that the future still has
work to do, so the caller will need to check again later. The `Ready` variant
indicates that the `Future` has finished its work and the `T` value is
available.

> Note: With most futures, the caller should not call `poll()` again after the
> future has returned `Ready`. Many futures will panic if polled again after
> becoming ready! Futures which are safe to poll again will say so explicitly in
> their documentation.

Under the hood, when you call `.await`, Rust compiles that to code which calls
`poll`, kind of (although not exactly <!-- TODO: describe `IntoFuture`? -->)
like this:

```rust,ignore
match hello("async").poll() {
    Ready(_) => {
        // We’re done!
    }
    Pending => {
        // But what goes here?
    }
}
```

What should we do when the `Future` is still `Pending`? We need some way to try
again… and again, and again, until the future is finally ready. In other words,
a loop:

```rust,ignore
let hello_fut = hello("async");
loop {
    match hello_fut.poll() {
        Ready(_) => {
            break;
        }
        Pending => {
            // continue
        }
    }
}
```

If Rust compiled it to exactly that code, though, every `.await` would be
blocking—exactly the opposite of what we were going for! Instead, Rust needs
makes sure that the loop can hand off control to something which can pause work
on this future and work on other futures and check this one again later. That
“something” is an async runtime, and this scheduling and coordination work is
one of the main jobs for a runtime.

---

Recall our description of how `rx.recv()` waits in the [Counting][counting]
section. The `recv()` call returns a `Future`, and awaiting it polls it. In our
initial discussion, we noted that a runtime will pause the future until it is
ready with either `Some(message)` or `None` when the channel closes. With a
deeper understanding of `Future` in place, and specifically its `poll` method,
we can see how that works. The runtime knows the future is not ready when it
returns `Poll::Pending`. Conversely, the runtime knows the future is ready and
advances it when `poll` returns `Poll::Ready(Some(message))` or
`Poll::Ready(None)`.

[counting]: /ch17-02-concurrency-with-async.md
