## Turning our Single-Threaded Server into a Multi-Threaded Server

<!-- Reading ahead, the original heading didn't seem to fit all of the sub
headings -- this might not be totally right either, so feel free to replace
with something more appropriate -->
<!-- This is fine! /Carol -->

Right now, the server will process each request in turn, meaning it won't
process a second connection until the first is finished processing. If this
server were to receive more and more requests, this sort of serial execution
would prove to be less and less optimal. If the server recieves a request that
takes a long time to process, subsequent requests will have to wait until the
long request is finished, even if the new requests can be processed quickly.
We'll need to fix this, but first, we'll look at the problem in action.

### Simulating a Slow Request in the Current Server Implementation

Let’s see how a slow-processing request can affect other requests made to our
current server implementation. Listing 20-10 implements handling a request to
`/sleep` with a simulated slow response that will cause the server to sleep for
five seconds before responding.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// --snip--

fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 512];
#     stream.read(&mut buffer).unwrap();
    // --snip--

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // --snip--
}
```

<span class="caption">Listing 20-10: Simulating a slow request by recognizing
`/sleep` and sleeping for 5 seconds</span>

This code is a bit messy, but it’s good enough for our simulation purposes! We
created a second request `sleep`, whose data our server recognizes. We added an
`else if` after the `if` block to check for the request to `/sleep`, and when
that request is received, our server will sleep for five seconds before
rendering the successful HTML page.

You can really see how primitive our server is here; real libraries would
handle the recognition of multiple requests in a much less verbose way!

Start the server with `cargo run`, and then open up two browser windows: one
for `http://localhost:8080/` and one for `http://localhost:8080/sleep`. If you
enter the `/` URI a few times, as before, you’ll see it respond quickly. But if
you enter `/sleep`, and then load up `/`, you’ll see that `/` waits until
`sleep` has slept for its full five seconds before loading.

There are multiple ways we could change how our web server works in order to
avoid having all requests back up behind a slow request; the one we’re going to
implement is a thread pool.

### Improving Throughput with a Thread Pool

<!--There seems to be some repetition throughout these thread pool sections, is
there any way to condense it? I've edited with this in mind, but am wary of
changing too much -->
<!-- Your edits that removed repetition are fine! /Carol -->

A *thread pool* is a group of spawned threads that are waiting and ready to
handle some task. When the program receives a new task, it will assign one of
the threads in the pool to the task, and that thread will go off and process
the task. The remaining threads in the pool are available to handle any other
tasks that come in while the first thread is processing. When the first thread
is done processing its task, it's returned to the pool of idle threads ready to
handle a new task. A thread pool will allow us to process connections
concurrently, increasing the throughput of our server.

We’ll limit the number of threads in the pool to a small number to protect us
from Denial of Service (DoS) attacks; if we had our program create a new thread
for each request as it comes in, someone making ten million requests to our
server could create havoc by using up all of our server’s resources and
grinding the processing of all requests to a halt.

Rather than spawning unlimited threads, then, we’ll have a fixed number of
threads waiting in the pool. As requests come in, they'll be sent to the pool
for processing. The pool will maintain a queue of incoming requests. Each of
the threads in the pool will pop a request off of this queue, handle the
request, and then ask the queue for another request. With this design, we can
process `N` requests concurrently, where `N` is the number of threads. If each
thread is responding to a long-running request, subsequent requests can still
back up in the queue, but we’ve increased the number of long-running requests
we can handle before that point.

This is just one of many ways to improve the throughput of our web server.
Other options you might explore are the fork/join model and the single threaded
async I/O model. If you’re interested in this topic, you may want to read more
about other solutions and try to implement them in Rust; with a low-level
language like Rust, all of these options are possible.

Before we begin, let’s talk about what using the pool should look like. When
trying to design code, writing the client interface first can really help guide
your design. Write the API of the code so that it's structured in the way you’d
want to call it, then implement the functionality within that structure, rather
than implementing the functionality then designing the public API.

Similar to how we used Test Driven Development in the project in Chapter 12,
we’re going to use Compiler Driven Development here. We’ll write the code that
calls the functions we wish we had, then we’ll look at errors from the compiler
to tell us what we should change next to get things working.
