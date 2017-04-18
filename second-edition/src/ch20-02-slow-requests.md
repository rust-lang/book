## How Slow Requests Affect Throughput

Right now, the server will process each request in turn. That works for
services like ours that aren't expected to get very many requests, but as
applications get more complex, this sort of serial execution isn't optimal.
Let's make our web server better by adding a *thread pool*. A thread pool is a
group of spawned threads that are ready to handle some task. When the program
receives a new task, one of the threads in the pool will be assigned the task
and will go off and process it. The remaining threads in the pool are available
to handle any other tasks that come in while the first thread is processing.
When the first thread is done processing its task, it gets returned to the pool
of idle threads ready to handle a new task.

Because our current program processes connections sequentially, it won't
process a second connection until it's completed processing the first. If we
get one request that takes a long time to process, requests coming in during
that time will have to wait until the long request is finished, even if the new
requests can be processed quickly. A thread pool allows us to process
connections concurrently: we can start processing a new connection before an
older connection is finished. This increases the throughput of our server.

Here's what we're going to implement: instead of waiting for each request to
process before starting on the next one, we'll send the processing of each
connection to a different thread. The threads will come from a pool of four
threads that we'll spawn when we start our program. The reason we're limiting
the number of threads to a small number is that if we created a new thread for
each request as the requests come in, someone making ten million requests to
our server could create havoc by using up all of our server's resources and
grinding the processing of all requests to a halt.

Instead, we'll create a pool of threads with a fixed size of our choosing. As
requests come in, we'll send them to the pool for processing. The pool will
maintain a queue of incoming requests. Each of the threads in the pool will pop
a request off of this queue, handle the request, and then ask the queue for
another request. With this design, we can process `N` requests concurrently,
where `N` is the number of threads. This still means that `N` long-running
requests can cause requests to back up in the queue, but we've increased the
number of long-running requests we can handle before that point from one to `N`.

### Simulating a Slow Request in the Current Server Implementation

Let's see the effect of a request that takes a long time to process on requests
made to our current server implementation. Listing 20-10 shows the code to
respond to another request, `/sleep`, that will cause the server to sleep for
five seconds before responding. This will simulate a slow request so that we
can see that our server processes requests serially.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// ...snip...

fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 512];
#     stream.read(&mut buffer).unwrap();
    // ...snip...

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let get_start = &buffer[..get.len()];
    let sleep_start = &buffer[..sleep.len()];

    let (status_line, filename) = if get_start == get {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if sleep_start == sleep {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // ...snip...
}
```

<span class="caption">Listing 20-10: Simulating a slow request by recognizing
`/sleep` and sleeping for 5 seconds</span>

This code is a bit messy, but it's good enough for our simulation purposes! We
created a second request whose data we'll recognize, `sleep`, and a second
slice of `buffer` that's the same size as `sleep` to try comparing with
`sleep`. We called the new slice `sleep_start` and changed the `start` slice
variable name to `get_start` to tell the two slices apart. We added an `else
if` after the `if` block to check for the request to `/sleep`, and when we see
that request, we'll sleep for five seconds before rendering the hello page.

You can really see how primitive our server is here; real libraries would
handle the recognition of multiple requests in a less verbose way!

Start the server with `cargo run`, and then open up two browser windows: one
for `http://localhost:8080/` and one for `http://localhost:8080/sleep`. If
you hit `/` a few times, as before, you'll see it respond quickly. But if you
hit `/sleep`, and then load up `/`, you'll see that `/` waits until `sleep`
has slept for its full five seconds before going on. This is the issue we can
improve with our thread pool.

This design is one of many ways to improve the throughput of our web server.
This isn't a book about web servers, though, so it's the one we're going to
cover. Other options are the "fork/join" model, and the "single threaded async
I/O" model. If you're interested in this topic, you may want to read more about
them and try to implement them in Rust; with a low-level language like Rust,
all of these options are possible.
