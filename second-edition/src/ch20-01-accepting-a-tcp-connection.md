## Accepting a TCP Connection

The *Hypertext Transfer Protocol* (*HTTP*) that powers the web is built on top
of the *Transmission Control Protocol* (*TCP*). We won't get into the details
too much, but here's a short overview: TCP is a low-level protocol, and HTTP
builds a higher-level protocol on top of TCP. Both protocols are what's called a
*request-response protocol*, that is, there is a *client* that initiates
requests, and a *server* that listens to requests and provides a response to
the client. The contents of those requests and responses are defined by the
protocols themselves.

TCP describes the low-level details of how information gets from one server to
another, but doesn't specify what that information is; it's just a bunch of
ones and zeroes. HTTP builds on top of TCP by defining what the content of the
requests and responses should be. As such, it's technically possible to use
HTTP with other protocols, but in the vast majority of cases, HTTP sends its
data over TCP.

So the first thing we need to build for our web server is to be able to listen
to a TCP connection. The standard library has a `std::net` module that lets us
do this. Let's make a new project:

```text
$ cargo new hello --bin
     Created binary (application) `hello` project
$ cd hello
```

And put the code in Listing 20-1 in `src/main.rs` to start. This code will
listen at the address `127.0.0.1:8080` for incoming TCP streams. When it gets
an incoming stream, it will print `Connection established!`:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
```

<span class="caption">Listing 20-1: Listening for incoming streams and printing
a message when we receive a stream</span>

A `TcpListener` allows us to listen for TCP connections. We've chosen to listen
to the address `127.0.0.1:8080`. The part before the colon is an IP address
representing our own computer, and `8080` is the port. We've chosen this port
because HTTP is normally accepted on port 80, but connecting to port 80 requires
administrator privileges. Regular users can listen on ports higher than 1024;
8080 is easy to remember since it's the HTTP port 80 repeated.

The `bind` function is sort of like `new` in that it returns a new
`TcpListener` instance, but `bind` is a more descriptive name that fits with
the domain terminology. In networking, people will often talk about "binding to
a port", so the function that the standard library defined to create a new
`TcpListener` is called `bind`.

The `bind` function returns a `Result<T, E>`. Binding may fail, for example, if
we had tried to connect to port 80 without being an administrator. Another
example of a case when binding would fail is if we tried to have two programs
listening to the same port, which would happen if we ran two instances of our
program. Since we're writing a basic server here, we're not going to worry
about handling these kinds of errors, and `unwrap` lets us just stop the
program if they happen.

The `incoming` method on `TcpListener` returns an iterator that gives us a
sequence of streams (more specifically, streams of type `TcpStream`). A
*stream* represents an open connection between the client and the server. A
*connection* is the name for the full request/response process when a client
connects to the server, the server generates a response, and the server closes
the connection. As such, the `TcpStream` will let us read from itself to see
what the client sent, and we can write our response to it. So this `for` loop
will process each connection in turn and produce a series of streams for us to
handle.

For now, handling a stream means calling `unwrap` to terminate our program if
the stream has any errors, then printing a message. Errors can happen because
we're not actually iterating over connections, we're iterating over *connection
attempts*. The connection might not work for a number of reasons, many of them
operating-system specific. For example, many operating systems have a limit to
the number of simultaneous open connections; new connection attempts will then
produce an error until some of the open connections are closed.

Let's try this code out! First invoke `cargo run` in the terminal, then load up
`127.0.0.1:8080` in a web browser. The browser will show an error message that
will say something similar to "Connection reset", since we're not currently
sending any data back. If we look at our terminal, though, we'll see a bunch of
messages that were printed when the browser connected to the server!

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

We got multiple messages printed out for one browser request; these connections
might be the browser making a request for the page and a request for a
`favicon.ico` icon that appears in the browser tab, or the browser might be
retrying the connection. Our browser is expecting to speak HTTP, but we aren't
replying with anything, just closing the connection by moving on to the next
loop iteration. When `stream` goes out of scope and dropped at the end of the
loop, its connection gets closed as part of the `drop` implementation for
`TcpStream`. Browsers sometimes deal with closed connections by retrying, since
the problem might be temporary. The important thing is that we've successfully
gotten a handle on a TCP connection!

Remember to stop the program with `CTRL-C` when you're done running a
particular version of the code, and restart `cargo run` after you've made each
set of code changes in order to be running the newest code.
