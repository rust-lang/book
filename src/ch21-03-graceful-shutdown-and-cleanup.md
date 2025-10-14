## Graceful Shutdown and Cleanup

The code in Listing 21-20 is responding to requests asynchronously through the
use of a thread pool, as we intended. We get some warnings about the `workers`,
`id`, and `thread` fields that we’re not using in a direct way that reminds us
we’re not cleaning up anything. When we use the less elegant
<kbd>ctrl</kbd>-<kbd>C</kbd> method to halt the main thread, all other threads
are stopped immediately as well, even if they’re in the middle of serving a
request.

Next, then, we’ll implement the `Drop` trait to call `join` on each of the
threads in the pool so they can finish the requests they’re working on before
closing. Then we’ll implement a way to tell the threads they should stop
accepting new requests and shut down. To see this code in action, we’ll modify
our server to accept only two requests before gracefully shutting down its
thread pool.

One thing to notice as we go: none of this affects the parts of the code that
handle executing the closures, so everything here would be just the same if we
were using a thread pool for an async runtime.

### Implementing the `Drop` Trait on `ThreadPool`

Let’s start with implementing `Drop` on our thread pool. When the pool is
dropped, our threads should all join to make sure they finish their work.
Listing 21-22 shows a first attempt at a `Drop` implementation; this code won’t
quite work yet.

<Listing number="21-22" file-name="src/lib.rs" caption="Joining each thread when the thread pool goes out of scope">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

First we loop through each of the thread pool `workers`. We use `&mut` for this
because `self` is a mutable reference, and we also need to be able to mutate
`worker`. For each `worker`, we print a message saying that this particular
`Worker` instance is shutting down, and then we call `join` on that `Worker`
instance’s thread. If the call to `join` fails, we use `unwrap` to make Rust
panic and go into an ungraceful shutdown.

Here is the error we get when we compile this code:

```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

The error tells us we can’t call `join` because we only have a mutable borrow of
each `worker` and `join` takes ownership of its argument. To solve this issue,
we need to move the thread out of the `Worker` instance that owns `thread` so
`join` can consume the thread. One way to do this is by taking the same approach
we did in Listing 18-15. If `Worker` held an `Option<thread::JoinHandle<()>>`,
we could call the `take` method on the `Option` to move the value out of the
`Some` variant and leave a `None` variant in its place. In other words, a
`Worker` that is running would have a `Some` variant in `thread`, and when we
wanted to clean up a `Worker`, we’d replace `Some` with `None` so the `Worker`
wouldn’t have a thread to run.

However, the _only_ time this would come up would be when dropping the `Worker`.
In exchange, we’d have to deal with an `Option<thread::JoinHandle<()>>` anywhere
we accessed `worker.thread`. Idiomatic Rust uses `Option` quite a bit, but when
you find yourself wrapping something you know will always be present in an
`Option` as a workaround like this, it’s a good idea to look for alternative
approaches to make your code cleaner and less error-prone.

In this case, a better alternative exists: the `Vec::drain` method. It accepts
a range parameter to specify which items to remove from the vector and returns
an iterator of those items. Passing the `..` range syntax will remove *every*
value from the vector.

So we need to update the `ThreadPool` `drop` implementation like this:

<Listing file-name="src/lib.rs">

```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

</Listing>

This resolves the compiler error and does not require any other changes to our
code. Note that, because drop can be called when panicking, the unwrap
could also panic and cause a double panic, which immediately crashes the
program and ends any cleanup in progress. This is fine for an example program,
but isn’t recommended for production code.

### Signaling to the Threads to Stop Listening for Jobs

With all the changes we’ve made, our code compiles without any warnings.
However, the bad news is that this code doesn’t function the way we want it to
yet. The key is the logic in the closures run by the threads of the `Worker`
instances: at the moment, we call `join`, but that won’t shut down the threads,
because they `loop` forever looking for jobs. If we try to drop our
`ThreadPool` with our current implementation of `drop`, the main thread will
block forever, waiting for the first thread to finish.

To fix this problem, we’ll need a change in the `ThreadPool` `drop`
implementation and then a change in the `Worker` loop.

First we’ll change the `ThreadPool` `drop` implementation to explicitly drop
the `sender` before waiting for the threads to finish. Listing 21-23 shows the
changes to `ThreadPool` to explicitly drop `sender`. Unlike with the thread,
here we _do_ need to use an `Option` to be able to move `sender` out of
`ThreadPool` with `Option::take`.

<Listing number="21-23" file-name="src/lib.rs" caption="Explicitly dropping `sender` before joining the `Worker` threads">

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

Dropping `sender` closes the channel, which indicates no more messages will be
sent. When that happens, all the calls to `recv` that the `Worker` instances do
in the infinite loop will return an error. In Listing 21-24, we change the
`Worker` loop to gracefully exit the loop in that case, which means the threads
will finish when the `ThreadPool` `drop` implementation calls `join` on them.

<Listing number="21-24" file-name="src/lib.rs" caption="Explicitly breaking out of the loop when `recv` returns an error">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

To see this code in action, let’s modify `main` to accept only two requests
before gracefully shutting down the server, as shown in Listing 21-25.

<Listing number="21-25" file-name="src/main.rs" caption="Shutting down the server after serving two requests by exiting the loop">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

You wouldn’t want a real-world web server to shut down after serving only two
requests. This code just demonstrates that the graceful shutdown and cleanup is
in working order.

The `take` method is defined in the `Iterator` trait and limits the iteration
to the first two items at most. The `ThreadPool` will go out of scope at the
end of `main`, and the `drop` implementation will run.

Start the server with `cargo run`, and make three requests. The third request
should error, and in your terminal you should see output similar to this:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

You might see a different ordering of `Worker` IDs and messages printed. We can
see how this code works from the messages: `Worker` instances 0 and 3 got the
first two requests. The server stopped accepting connections after the second
connection, and the `Drop` implementation on `ThreadPool` starts executing
before `Worker` 3 even starts its job. Dropping the `sender` disconnects all the
`Worker` instances and tells them to shut down. The `Worker` instances each
print a message when they disconnect, and then the thread pool calls `join` to
wait for each `Worker` thread to finish.

Notice one interesting aspect of this particular execution: the `ThreadPool`
dropped the `sender`, and before any `Worker` received an error, we tried to
join `Worker` 0. `Worker` 0 had not yet gotten an error from `recv`, so the main
thread blocked, waiting for `Worker` 0 to finish. In the meantime, `Worker` 3
received a job and then all threads received an error. When `Worker` 0 finished,
the main thread waited for the rest of the `Worker` instances to finish. At that
point, they had all exited their loops and stopped.

Congrats! We’ve now completed our project; we have a basic web server that uses
a thread pool to respond asynchronously. We’re able to perform a graceful
shutdown of the server, which cleans up all the threads in the pool.

Here’s the full code for reference:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

</Listing>

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

</Listing>

We could do more here! If you want to continue enhancing this project, here are
some ideas:

- Add more documentation to `ThreadPool` and its public methods.
- Add tests of the library’s functionality.
- Change calls to `unwrap` to more robust error handling.
- Use `ThreadPool` to perform some task other than serving web requests.
- Find a thread pool crate on [crates.io](https://crates.io/) and implement a
  similar web server using the crate instead. Then compare its API and
  robustness to the thread pool we implemented.

## Summary

Well done! You’ve made it to the end of the book! We want to thank you for
joining us on this tour of Rust. You’re now ready to implement your own Rust
projects and help with other people’s projects. Keep in mind that there is a
welcoming community of other Rustaceans who would love to help you with any
challenges you encounter on your Rust journey.
