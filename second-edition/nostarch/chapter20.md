
[TOC]

# Final Project: Building a Multithreaded Web Server

It’s been a long journey, but here we are! It’s the end of the book. Parting is
such sweet sorrow. But before we go, let’s build one more project together, to
show off some of the things we learned in these final chapters, as well as
re-cap some of the earlier ones.

Here’s what we’re going to make: a web server that says hello:

<img src="trpl20-01.png" />

To do this, we will:

1. Learn a little bit about TCP and HTTP
2. Listen for TCP connections on a socket
3. Parse a tiny number of HTTP requests
4. Create a proper HTTP response
5. Improve the throughput of our server with a thread pool

Before we get started, however, there’s one thing we should mention: if you
were writing this code in production, there are a lot of better ways to write
it. Specifically, there are a number of robust crates on crates.io that provide
much more complete web server and thread pool implementations than we are going
to build.

However, for this chapter, our intention is to learn, not to take the easy
route. Since Rust is a systems programming language, we’re able to choose what
level of abstraction we want to work with. We’re able to go to a lower level
than is possible or practical in other languages if we so choose. So we’ll be
writing a basic HTTP server and thread pool ourselves in order to learn the
general ideas and techniques behind the crates we might use in the future.

## A Single Threaded Web Server

First, let’s get a single threaded web server working. We’re going to work with
the raw bytes of TCP and HTTP requests and responses to send HTML from our
server to a web browser. Let’s start with a quick overview of the protocols
involved.

The *Hypertext Transfer Protocol* (*HTTP*) that powers the web is built on top
of the *Transmission Control Protocol* (*TCP*). We won’t get into the details
too much, but here’s a short overview: TCP is a low-level protocol, and HTTP
builds a higher-level protocol on top of TCP. Both protocols are what’s called a
*request-response protocol*, that is, there is a *client* that initiates
requests, and a *server* that listens to requests and provides a response to
the client. The contents of those requests and responses are defined by the
protocols themselves.

TCP describes the low-level details of how information gets from one server to
another, but doesn’t specify what that information is; it’s just a bunch of
ones and zeroes. HTTP builds on top of TCP by defining what the content of the
requests and responses should be. As such, it’s technically possible to use
HTTP with other protocols, but in the vast majority of cases, HTTP sends its
data over TCP.

So the first thing we need to build for our web server is to be able to listen
to a TCP connection. The standard library has a `std::net` module that lets us
do this. Let’s make a new project:

```
$ cargo new hello --bin
     Created binary (application) `hello` project
$ cd hello
```

And put the code in Listing 20-1 in `src/main.rs` to start. This code will
listen at the address `127.0.0.1:8080` for incoming TCP streams. When it gets
an incoming stream, it will print `Connection established!`:

Filename: src/main.rs

```
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
```

Listing 20-1: Listening for incoming streams and printing a message when we
receive a stream

A `TcpListener` allows us to listen for TCP connections. We’ve chosen to listen
to the address `127.0.0.1:8080`. The part before the colon is an IP address
representing our own computer, and `8080` is the port. We’ve chosen this port
because HTTP is normally accepted on port 80, but connecting to port 80 requires
administrator privileges. Regular users can listen on ports higher than 1024;
8080 is easy to remember since it’s the HTTP port 80 repeated.

The `bind` function is sort of like `new` in that it returns a new
`TcpListener` instance, but `bind` is a more descriptive name that fits with
the domain terminology. In networking, people will often talk about “binding to
a port”, so the function that the standard library defined to create a new
`TcpListener` is called `bind`.

The `bind` function returns a `Result<T, E>`. Binding may fail, for example, if
we had tried to connect to port 80 without being an administrator. Another
example of a case when binding would fail is if we tried to have two programs
listening to the same port, which would happen if we ran two instances of our
program. Since we’re writing a basic server here, we’re not going to worry
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
we’re not actually iterating over connections, we’re iterating over *connection
attempts*. The connection might not work for a number of reasons, many of them
operating-system specific. For example, many operating systems have a limit to
the number of simultaneous open connections; new connection attempts will then
produce an error until some of the open connections are closed.

Let’s try this code out! First invoke `cargo run` in the terminal, then load up
`127.0.0.1:8080` in a web browser. The browser will show an error message that
will say something similar to “Connection reset”, since we’re not currently
sending any data back. If we look at our terminal, though, we’ll see a bunch of
messages that were printed when the browser connected to the server!

```
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

We got multiple messages printed out for one browser request; these connections
might be the browser making a request for the page and a request for a
`favicon.ico` icon that appears in the browser tab, or the browser might be
retrying the connection. Our browser is expecting to speak HTTP, but we aren’t
replying with anything, just closing the connection by moving on to the next
loop iteration. When `stream` goes out of scope and dropped at the end of the
loop, its connection gets closed as part of the `drop` implementation for
`TcpStream`. Browsers sometimes deal with closed connections by retrying, since
the problem might be temporary. The important thing is that we’ve successfully
gotten a handle on a TCP connection!

Remember to stop the program with <span class="keystroke">ctrl-C</span> when
you’re done running a particular version of the code, and restart `cargo run`
after you’ve made each set of code changes in order to be running the newest
code.

### Reading the Request

Let’s read in the request from our browser! Since we’re adding more
functionality that has the purpose of handling the connection, let’s start a
new function to have a nice separation of the concerns around setting up the
server and connections versus processing each connection. In this new
`handle_connection` function, we’ll read data from the `stream` and print it
out in order to see the data that the browser is sending us. Change the code to
look like Listing 20-2:

Filename: src/main.rs

```
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

Listing 20-2: Reading from the `TcpStream` and printing out the data

We added `std::io::prelude` to the beginning in order to bring traits into
scope that let us read from and write to the stream. Instead of printing a
message that we got a connection in the `for` loop in `main`, we’re calling the
new `handle_connection` function and passing the `stream` to it.

In `handle_connection`, we made the `stream` parameter mutable with the `mut`
keyword. As we read from a stream, the `TcpStream` instance might read more
than what we ask for into a buffer. Internally, it keeps track of what data it
has returned to us. It needs to be `mut` because of that state changing, so
even though we usually think of “reading” as not needing mutation, in this
case, we do need to use the `mut` keyword.

Next, we need to actually read from the stream. We do this in two steps: first,
we declare a `buffer` on the stack to hold the data that we read in. We’ve made
the buffer 512 bytes in size, which is big enough to hold the data of a basic
request. That’s sufficient for our purposes in this chapter. If we wanted to
handle requests of an arbitrary size, managing the buffer would need to be more
complicated, but we’re keeping it simple for now. We then pass the buffer to
`stream.read`, which will read bytes from the `TcpStream` and put them in the
buffer.

Then we convert the bytes in the buffer to a string and print out that string.
The `String::from_utf8_lossy` function takes a `&[u8]` and produces a `String`.
The ‘lossy’ part of the name comes from the behavior when this function sees
invalid UTF-8 sequences: it replaces the invalid sequences with �, `U+FFFD
REPLACEMENT CHARACTER`. You might see the replacement characters for remaining
characters in the buffer that aren’t filled by request data.

Let’s give this a try! Start up the program and make a request in a web browser
again. Note that we’ll still get an error page in the browser, but the output
of our program in the terminal will now look similar to this:

```
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
     Running `target/debug/hello`
Request: GET / HTTP/1.1
Host: 127.0.0.1:8080
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
```

You’ll probably get slightly different output depending on your browser. You
also might see this request repeated again. Now that we’re printing out the
request data, we can see why we’re getting multiple connections from one
browser request by looking at the path after `Request: GET`. If the repeated
connections are all requesting `/`, we know the browser is trying to fetch `/`
repeatedly since it’s not getting a response from us.

Let’s break down this request data to understand what the browser is asking of
us. HTTP is a text-based protocol, and a request takes this format:

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

The first line is called the *request line*, and it holds information about
what the client is requesting. The first part of the request line is a
*method*, like `GET` or `POST`, that describes how the client is making this
request.

Then comes the request’s *URI*, which stands for *Uniform Resource Identifier*.
URIs are almost, but not quite the same as URLs (*Uniform Resource Locators*),
which is what we typically call the addresses that we enter into a web browser.
The HTTP spec uses the term URI, and the difference between URIs and URLs isn’t
important for our purposes of this chapter, so we can just mentally substitute
URL for URI here.

Next, we have the HTTP version that the client used, and then the request line
ends in a CRLF sequence. The CRLF sequence can also be written as `\r\n`: `\r`
is a *carriage return* and `\n` is a *line feed*. These terms come from the
typewriter days! The CRLF sequence separates the request line from the rest of
the request data.

Taking a look at the request line data we saw printed out by our code:

```
GET / HTTP/1.1
```

`GET` is the method, `/` is the Request URI, and `HTTP/1.1` is the version.

The remaining lines starting from `Host:` onward are headers; `GET` requests
have no body.

Try making a request from a different browser, or asking for a different
address like `127.0.0.1:8080/test` to see how the request data changes, if
you’d like.

Now that we know what the browser is asking for, let’s send some data back!

### Writing a Response

Let’s send data back to our browser in response to its request. Responses have
this format:

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

The first line is called a *status line* and contains the HTTP version used in
the response, a numeric status code that summarizes the result of the request,
and a reason phrase that provides a text description of the status code. After
the CRLF sequence comes any headers, another CRLF sequence, and the body of the
response.

Here’s an example response that uses version 1.1 of HTTP, has a status code of
`200`, a reason phrase of `OK`, no headers, and no body:

```
HTTP/1.1 200 OK\r\n\r\n
```

This text is a tiny successful HTTP response. Let’s write this to the stream!
Remove the `println!` that was printing the request data, and add the code in
Listing 20-3 in its place:

Filename: src/main.rs

```
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Listing 20-3: Writing a tiny successful HTTP response to the stream

The first new line defines the `response` variable that holds the data of the
tiny success response we’re sending back. Then, we call `as_bytes` on our
`response` because the `write` method on `stream` takes a `&[u8]` and sends
those bytes directly down the connection.

The `write` operation could fail, so `write` returns a `Result<T, E>`; we’re
continuing to use `unwrap` to make progress on the core ideas in this chapter
rather than error handling. Finally, `flush` will wait until all of the bytes
are written to the connection; `TcpStream` contains an internal buffer to
minimize calls into the underlying operating system.

With these changes, let’s run our code and make a request! We’re no longer
printing any data to the terminal, so we won’t see any output there other than
the output from Cargo. When we load `127.0.0.1:8080` in a web browser, though,
we get a blank page instead of an error. How exciting! You’ve just hand-coded
an HTTP request and response.

### Returning Real HTML

Let’s return more than a blank page. Create a new file, *hello.html*, in the
root of your project directory, that is, not in the `src` directory. You can
put any HTML you want in it; Listing 20-4 shows what the authors used for
theirs:

Filename: hello.html

```
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
```

Listing 20-4: A sample HTML file to return in a response

This is a minimal HTML 5 document with a heading and a little paragraph. Let’s
modify `handle_connection` as shown in Listing 20-5 to read the HTML file, add
it to the response as a body, and send it:

Filename: src/main.rs

```
use std::fs::File;

// ...snip...

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let mut file = File::open("hello.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Listing 20-5: Sending the contents of *hello.html* as the body of the response

We’ve added a line at the top to bring the standard library’s `File` into
scope, and the file opening and reading code should look familiar since we had
similar code in Chapter 12 when we read the contents of a file for our I/O
project in Listing 12-4.

Next, we’re using `format!` to add the file’s contents as the body of the
success response that we write to the stream.

Run it with `cargo run`, load up `127.0.0.1:8080` in your browser, and you
should see your HTML rendered!

Note that we’re currently ignoring the request data in `buffer` and sending
back the contents of the HTML file unconditionally. Try requesting
`127.0.0.1:8080/something-else` in your browser and you’ll get back your HTML
for that request too. Sending back the same response for all requests is pretty
limited and not what most web servers do; let’s examine the request and only
send back the HTML file for a well-formed request to `/`.

### Validating the Request and Selectively Responding

Right now, our web server will return the HTML in the file no matter what the
client requested. Let’s check that the browser is requesting `/`, and instead
return an error if the browser requests anything else. Let’s modify
`handle_connection` as shown in Listing 20-6, which adds part of the code we’ll
need. This part checks the content of the request we received against what we
know a request for `/` looks like and adds `if` and `else` blocks where we’ll
add code to treat requests differently:

Filename: src/main.rs

```
// ...snip...

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // some other request
    };
}
```

Listing 20-6: Matching the request against the content we expect for a request
to `/` and setting up conditionally handling requests to `/` differently than
other requests

Here, we hardcoded the data corresponding to the request that we’re looking for
in the variable `get`. Because we’re reading raw bytes into the buffer, we use
a byte string, created with `b""`, to make `get` a byte string too. Then, we
check to see if `buffer` starts with the bytes in `get`. If it does, we’ve
gotten a well-formed request to `/`, which is the success case that we want to
handle in the `if` block. The `if` block contains the code we added in Listing
20-5 that returns the contents of our HTML file.

If `buffer` does not start with the bytes in `get`, we’ve gotten some other
request. We’ll respond to all other requests using the code we’re about to add
in the `else` block.

If you run this code and request `127.0.0.1:8080`, you’ll get the HTML that’s
in *hello.html*. If you make any other request, such as
`127.0.0.1:8080/something-else`, you’ll get a connection error like we saw when
running the code in Listing 20-1 and Listing 20-2.

Let’s add code to the `else` block as shown in Listing 20-7 to return a
response with the status code `404`, which signals that the content for the
request was not found. We’ll also return HTML for a page to render in the
browser indicating as such to the end user:

Filename: src/main.rs

```
// ...snip...

} else {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let mut file = File::open("404.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Listing 20-7: Responding with status code `404` and an error page if anything
other than `/` was requested

Here, our response has a status line with status code `404` and the reason
phrase `NOT FOUND`. We still aren’t returning any headers, and the body of the
response will be the HTML in the file *404.html*. Also create a *404.html* file
next to *hello.html* for the error page; again feel free to use any HTML you’d
like or use the example HTML in Listing 20-8:

Filename: 404.html

```
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
```

Listing 20-8: Sample content for the page to send back with any `404` response

With these changes, try running your server again. Requesting `127.0.0.1:8080`
should return the contents of *hello.html*, and any other request, like
`127.0.0.1:8080/foo`, should return the error HTML from *404.html*!

There’s a lot of repetition between the code in the `if` and the `else` blocks:
they’re both reading files and writing the contents of the files to the stream.
The only differences between the two cases are the status line and the
filename. Let’s pull those differences out into an `if` and `else` of one line
each that will assign the values of the status line and the filename to
variables; we can then use those variables unconditionally in the code to read
the file and write the response. The resulting code after this refactoring is
shown in Listing 20-9:

Filename: src/main.rs

```
// ...snip...

fn handle_connection(mut stream: TcpStream) {
    // ...snip...

   let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Listing 20-9: Refactoring so that the `if` and `else` blocks only contain the
code that differs between the two cases

Here, the only thing the `if` and `else` blocks do is return the appropriate
values for the status line and filename in a tuple; we then use destructuring
to assign these two values to `status_line` and `filename` using a pattern in
the `let` statement like we discussed in Chapter 18.

The duplicated code to read the file and write the response is now outside the
`if` and `else` blocks, and uses the `status_line` and `filename` variables.
This makes it easier to see exactly what’s different between the two cases, and
makes it so that we only have one place to update the code if we want to change
how the file reading and response writing works. The behavior of the code in
Listing 20-9 will be exactly the same as that in Listing 20-8.

Awesome! We have a simple little web server in about 40 lines of Rust code that
responds to one request with a page of content and responds to all other
requests with a `404` response.

Since this server runs in a single thread, though, it can only serve one
request at a time. Let’s see how that can be a problem by simulating some
slow requests.

## How Slow Requests Affect Throughput

Right now, the server will process each request in turn. That works for
services like ours that aren’t expected to get very many requests, but as
applications get more complex, this sort of serial execution isn’t optimal.

Because our current program processes connections sequentially, it won’t
process a second connection until it’s completed processing the first. If we
get one request that takes a long time to process, requests coming in during
that time will have to wait until the long request is finished, even if the new
requests can be processed quickly. Let’s see this in action.

### Simulating a Slow Request in the Current Server Implementation

Let’s see the effect of a request that takes a long time to process on requests
made to our current server implementation. Listing 20-10 shows the code to
respond to another request, `/sleep`, that will cause the server to sleep for
five seconds before responding. This will simulate a slow request so that we
can see that our server processes requests serially.

Filename: src/main.rs

```
use std::thread;
use std::time::Duration;
// ...snip...

fn handle_connection(mut stream: TcpStream) {
    // ...snip...

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

    // ...snip...
}
```

Listing 20-10: Simulating a slow request by recognizing `/sleep` and sleeping
for 5 seconds

This code is a bit messy, but it’s good enough for our simulation purposes! We
created a second request `sleep`, whose data we’ll recognize. We added an `else
if` after the `if` block to check for the request to `/sleep`, and when we see
that request, we’ll sleep for five seconds before rendering the hello page.

You can really see how primitive our server is here; real libraries would
handle the recognition of multiple requests in a less verbose way!

Start the server with `cargo run`, and then open up two browser windows: one
for `http://localhost:8080/` and one for `http://localhost:8080/sleep`. If
you hit `/` a few times, as before, you’ll see it respond quickly. But if you
hit `/sleep`, and then load up `/`, you’ll see that `/` waits until `sleep`
has slept for its full five seconds before going on.

There are multiple ways we could change how our web server works in order to
avoid having all requests back up behind a slow request; the one we’re going to
implement is a thread pool.

### Improving Throughput with a Thread Pool

A *thread pool* is a group of spawned threads that are ready to handle some
task. When the program receives a new task, one of the threads in the pool will
be assigned the task and will go off and process it. The remaining threads in
the pool are available to handle any other tasks that come in while the first
thread is processing. When the first thread is done processing its task, it
gets returned to the pool of idle threads ready to handle a new task.

A thread pool will allow us to process connections concurrently: we can start
processing a new connection before an older connection is finished. This
increases the throughput of our server.

Here’s what we’re going to implement: instead of waiting for each request to
process before starting on the next one, we’ll send the processing of each
connection to a different thread. The threads will come from a pool of four
threads that we’ll spawn when we start our program. The reason we’re limiting
the number of threads to a small number is that if we created a new thread for
each request as the requests come in, someone making ten million requests to
our server could create havoc by using up all of our server’s resources and
grinding the processing of all requests to a halt.

Rather than spawning unlimited threads, we’ll have a fixed number of threads
waiting in the pool. As requests come in, we’ll send the requests to the pool
for processing. The pool will maintain a queue of incoming requests. Each of
the threads in the pool will pop a request off of this queue, handle the
request, and then ask the queue for another request. With this design, we can
process `N` requests concurrently, where `N` is the number of threads. This
still means that `N` long-running requests can cause requests to back up in the
queue, but we’ve increased the number of long-running requests we can handle
before that point from one to `N`.

This design is one of many ways to improve the throughput of our web server.
This isn’t a book about web servers, though, so it’s the one we’re going to
cover. Other options are the fork/join model and the single threaded async I/O
model. If you’re interested in this topic, you may want to read more about
other solutions and try to implement them in Rust; with a low-level language
like Rust, all of these options are possible.

## Designing the Thread Pool Interface

Let’s talk about what using the pool should look like. The authors often find
that when trying to design some code, writing the client interface first can
really help guide your design. Write the API of the code to be structured in
the way you’d want to call it, then implement the functionality within that
structure rather than implementing the functionality then designing the public
API.

Similar to how we used Test Driven Development in the project in Chapter 12,
we’re going to use Compiler Driven Development here. We’re going to write the
code that calls the functions we wish we had, then we’ll lean on the compiler
to tell us what we should change next. The compiler error messages will guide
our implementation.

### Code Structure if We Could Use `thread::spawn`

First, let’s explore what the code to create a new thread for every connection
could look like. This isn’t our final plan due to the problems with potentially
spawning an unlimited number of threads that we talked about earlier, but it’s
a start. Listing 20-11 shows the changes to `main` to spawn a new thread to
handle each stream within the `for` loop:

Filename: src/main.rs

```
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
```

Listing 20-11: Spawning a new thread for each stream

As we learned in Chapter 16, `thread::spawn` will create a new thread and then
run the code in the closure in it. If you run this code and load `/sleep` and
then `/` in two browser tabs, you’ll indeed see the request to `/` doesn’t have
to wait for `/sleep` to finish. But as we mentioned, this will eventually
overwhelm the system since we’re making new threads without any limit.

### Creating a Similar Interface for `ThreadPool`

We want our thread pool to work in a similar, familiar way so that switching
from threads to a thread pool doesn’t require large changes to the code we want
to run in the pool. Listing 20-12 shows the hypothetical interface for a
`ThreadPool` struct we’d like to use instead of `thread::spawn`:

Filename: src/main.rs

```
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

Listing 20-12: How we want to be able to use the `ThreadPool` we’re going to
implement

We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, in this case four. Then, in the `for` loop, `pool.execute` will
work in a similar way to `thread::spawn`.

### Compiler Driven Development to Get the API Compiling

Go ahead and make the changes in Listing 20-12 to *src/main.rs*, and let’s use
the compiler errors to drive our development. Here’s the first error we get:

```
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0433]: failed to resolve. Use of undeclared type or module `ThreadPool`
  --> src\main.rs:10:16
   |
10 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ Use of undeclared type or module
   `ThreadPool`

error: aborting due to previous error
```

Great, we need a `ThreadPool`. Let’s switch the `hello` crate from a binary
crate to a library crate to hold our `ThreadPool` implementation, since the
thread pool implementation will be independent of the particular kind of work
that we’re doing in our web server. Once we’ve got the thread pool library
written, we could use that functionality to do whatever work we want to do, not
just serve web requests.

So create *src/lib.rs* that contains the simplest definition of a `ThreadPool`
struct that we can have for now:

Filename: src/lib.rs

```
pub struct ThreadPool;
```

Then create a new directory, *src/bin*, and move the binary crate rooted in
*src/main.rs* into *src/bin/main.rs*. This will make the library crate be the
primary crate in the *hello* directory; we can still run the binary in
*src/bin/main.rs* using `cargo run` though. After moving the *main.rs* file,
edit it to bring the library crate in and bring `ThreadPool` into scope by
adding this at the top of *src/bin/main.rs*:

Filename: src/bin/main.rs

```
extern crate hello;
use hello::ThreadPool;
```

And try again in order to get the next error that we need to address:

```
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error: no associated item named `new` found for type `hello::ThreadPool` in the
current scope
  --> src\main.rs:13:16
   |
13 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^
   |
```

Cool, the next thing is to create an associated function named `new` for
`ThreadPool`. We also know that `new` needs to have one parameter that can
accept `4` as an argument, and `new` should return a `ThreadPool` instance.
Let’s implement the simplest `new` function that will have those
characteristics:

Filename: src/lib.rs

```
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        ThreadPool
    }
}
```

We picked `u32` as the type of the `size` parameter, since we know that a
negative number of threads makes no sense. `u32` is a solid default. Once we
actually implement `new` for real, we’ll reconsider whether this is the right
choice for what the implementation needs, but for now, we’re just working
through compiler errors.

Let’s check the code again:

```
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`, #[warn(unused_variables)] on by default
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: u32) -> ThreadPool {
  |                ^^^^

error: no method named `execute` found for type `hello::ThreadPool` in the
current scope
  --> src/main.rs:18:14
   |
18 |         pool.execute(|| {
   |              ^^^^^^^
```

Okay, a warning and an error. Ignoring the warning for a moment, the error is
because we don’t have an `execute` method on `ThreadPool`. Let’s define one,
and we need it to take a closure. If you remember from Chapter 13, we can take
closures as arguments with three different traits: `Fn`, `FnMut`, and `FnOnce`.
What kind of closure should we use? Well, we know we’re going to end up doing
something similar to `thread::spawn`; what bounds does the signature of
`thread::spawn` have on its argument? Let’s look at the documentation, which
says:

```
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`F` is the parameter we care about here; `T` is related to the return value and
we’re not concerned with that. Given that `spawn` uses `FnOnce` as the trait
bound on `F`, it’s probably what we want as well, since we’ll eventually be
passing the argument we get in `execute` to `spawn`. We can be further
confident that `FnOnce` is the trait that we want to use since the thread for
running a request is only going to execute that request’s closure one time.

`F` also has the trait bound `Send` and the lifetime bound `'static`, which
also make sense for our situation: we need `Send` to transfer the closure from
one thread to another, and `'static` because we don’t know how long the thread
will execute. Let’s create an `execute` method on `ThreadPool` that will take a
generic parameter `F` with these bounds:

Filename: src/lib.rs

```
impl ThreadPool {
    // ...snip...

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}
```

The `FnOnce` trait still needs the `()` after it since this `FnOnce` is
representing a closure that takes no parameters and doesn’t return a value.
Just like function definitions, the return type can be omitted from the
signature, but even if we have no parameters, we still need the parentheses.

Again, since we’re working on getting the interface compiling, we’re adding the
simplest implementation of the `execute` method, which does nothing. Let’s
check again:

```
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`, #[warn(unused_variables)] on by default
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: u32) -> ThreadPool {
  |                ^^^^

warning: unused variable: `f`, #[warn(unused_variables)] on by default
 --> src/lib.rs:8:30
  |
8 |     pub fn execute<F>(&self, f: F)
  |                              ^
```

Only warnings now! It compiles! Note that if you try `cargo run` and making a
request in the browser, though, you’ll see the errors in the browser again that
we saw in the beginning of the chapter. Our library isn’t actually calling the
closure passed to `execute` yet!

> A saying you might hear about languages with strict compilers like Haskell
> and Rust is “if the code compiles, it works.” This is a good time to remember
> that this is just a phrase and a feeling people sometimes have, it’s not
> actually universally true. Our project compiles, but it does absolutely
> nothing! If we were building a real, complete project, this would be a great
> time to start writing unit tests to check that the code compiles *and* has
> the behavior we want.

## Creating the Thread Pool and Storing Threads

The warnings are because we aren’t doing anything with the parameters to `new`
and `execute`. Let’s implement the bodies of both of these with the actual
behavior we want.

### Validating the Number of Threads in the Pool

To start, let’s think about `new`. We mentioned before that we picked an
unsigned type for the `size` parameter since a pool with a negative number of
threads makes no sense. However, a pool with zero threads also makes no sense,
yet zero is a perfectly valid `u32`. Let’s check that `size` is greater than
zero before we return a `ThreadPool` instance and panic if we get zero by using
the `assert!` macro as shown in Listing 20-13:

Filename: src/lib.rs

```
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    // ...snip...
}
```

Listing 20-13: Implementing `ThreadPool::new` to panic if `size` is zero

We’ve taken this opportunity to add some documentation for our `ThreadPool`
with doc comments. Note that we followed good documentation practices and added
a section that calls out the situations in which our function can panic as we
discussed in Chapter 14. Try running `cargo doc --open` and clicking on the
`ThreadPool` struct to see what the generate docs for `new` look like!

Instead of adding the use of the `assert!` macro as we’ve done here, we could
make `new` return a `Result` instead like we did with `Config::new` in the I/O
project in Listing 12-9, but we’ve decided in this case that trying to create a
thread pool without any threads should be an unrecoverable error. If you’re
feeling ambitious, try to write a version of `new` with this signature to see
how you feel about both versions:

```
fn new(size: u32) -> Result<ThreadPool, PoolCreationError> {
```

### Storing Threads in the Pool

Now that we know we have a valid number of threads to store in the pool, we can
actually create that many threads and store them in the `ThreadPool` struct
before returning it.

This raises a question: how do we “store” a thread? Let’s take another look at
the signature of `thread::spawn`:

```
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`spawn` returns a `JoinHandle<T>`, where `T` is the type that’s returned from
the closure. Let’s try using `JoinHandle` too and see what happens. In our
case, the closures we’re passing to the thread pool will handle the connection
and not return anything, so `T` will be the unit type `()`.

This won’t compile yet, but let’s consider the code shown in Listing 20-14.
We’ve changed the definition of `ThreadPool` to hold a vector of
`thread::JoinHandle<()>` instances, initialized the vector with a capacity of
`size`, set up a `for` loop that will run some code to create the threads, and
returned a `ThreadPool` instance containing them:

Filename: src/lib.rs

```
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    // ...snip...
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool {
            threads
        }
    }

    // ...snip...
}
```

Listing 20-14: Creating a vector for `ThreadPool` to hold the threads

We’ve brought `std::thread` into scope in the library crate, since we’re using
`thread::JoinHandle` as the type of the items in the vector in `ThreadPool`.

After we have a valid size, we’re creating a new vector that can hold `size`
items. We haven’t used `with_capacity` in this book yet; it does the same thing
as `Vec::new`, but with an important difference: it pre-allocates space in the
vector. Since we know that we need to store `size` elements in the vector,
doing this allocation up-front is slightly more efficient than only writing
`Vec::new`, since `Vec::new` resizes itself as elements get inserted. Since
we’ve created a vector the exact size that we need up front, no resizing of the
underlying vector will happen while we populate the items.

That is, if this code works, which it doesn’t quite yet! If we check this code,
we get an error:

```
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0308]: mismatched types
  --> src\main.rs:70:46
   |
70 |         let mut threads = Vec::with_capacity(size);
   |                                              ^^^^ expected usize, found u32

error: aborting due to previous error
```

`size` is a `u32`, but `Vec::with_capacity` needs a `usize`. We have two
options here: we can change our function’s signature, or we can cast the `u32`
as a `usize`. If you remember when we defined `new`, we didn’t think too hard
about what number type made sense, we just chose one. Let’s give it some more
thought now. Given that `size` is the length of a vector, `usize` makes a lot
of sense. They even almost share a name! Let’s change the signature of `new`,
which will get the code in Listing 20-14 to compile:

```
fn new(size: usize) -> ThreadPool {
```

If run `cargo check` again, you’ll get a few more warnings, but it should
succeed.

We left a comment in the `for` loop in Listing 20-14 regarding the creation of
threads. How do we actually create threads? This is a tough question. What
should go in these threads? We don’t know what work they need to do at this
point, since the `execute` method takes the closure and gives it to the pool.

Let’s refactor slightly: instead of storing a vector of `JoinHandle<()>`
instances, let’s create a new struct to represent the concept of a *worker*. A
worker will be what receives a closure in the `execute` method, and it will
take care of actually calling the closure. In addition to letting us store a
fixed `size` number of `Worker` instances that don’t yet know about the
closures they’re going to be executing, we can also give each worker an `id` so
we can tell the different workers in the pool apart when logging or debugging.

Let’s make these changes:

1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`
2. Change `ThreadPool` to hold a vector of `Worker` instances
3. Define a `Worker::new` function that takes an `id` number and returns a
   `Worker` instance with that `id` and a thread spawned with an empty closure,
   which we’ll fix soon
4. In `ThreadPool::new`, use the `for` loop counter to generate an `id`, create
   a new `Worker` with that `id`, and store the worker in the vector

If you’re up for a challenge, try implementing these changes on your own before
taking a look at the code in Listing 20-15.

Ready? Here’s Listing 20-15 with one way to make these modifications:

Filename: src/lib.rs

```
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    // ...snip...
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers
        }
    }
    // ...snip...
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}
```

Listing 20-15: Modifying `ThreadPool` to hold `Worker` instances instead of
threads directly

We’ve chosen to change the name of the field on `ThreadPool` from `threads` to
`workers` since we’ve changed what we’re holding, which is now `Worker`
instances instead of `JoinHandle<()>` instances. We use the counter in the
`for` loop as an argument to `Worker::new`, and we store each new `Worker` in
the vector named `workers`.

The `Worker` struct and its `new` function are private since external code
(like our server in *src/bin/main.rs*) doesn’t need to know the implementation
detail that we’re using a `Worker` struct within `ThreadPool`. The
`Worker::new` function uses the given `id` and stores a `JoinHandle<()>`
created by spawning a new thread using an empty closure.

This code compiles and is storing the number of `Worker` instances that we
specified as an argument to `ThreadPool::new`, but we’re *still* not processing
the closure that we get in `execute`. Let’s talk about how to do that next.

## Sending Requests to Threads Via Channels

The next problem to tackle is that our closures do absolutely nothing. We’ve
been working around the problem that we get the actual closure we want to
execute in the `execute` method, but it feels like we need to know the actual
closures when we create the `ThreadPool`.

Let’s think about what we really want to do though: we want the `Worker`
structs that we just created to fetch jobs from a queue that the `ThreadPool`
holds, and run those jobs in a thread.

In Chapter 16, we learned about channels. Channels are a great way to
communicate between two threads, and they’re perfect for this use-case. The
channel will function as the queue of jobs, and `execute` will send a job from
the `ThreadPool` to the `Worker` instances that are checking for jobs in the
thread they’ve spawned. Here’s the plan:

1. `ThreadPool` will create a channel and hold on to the sending side.
2. Each `Worker` will hold on to the receiving side of the channel.
3. A new `Job` struct will hold the closures we want to send down the channel.
4. The `execute` method of `ThreadPool` will send the job it wants
   to execute down the sending side of the channel.
5. In a thread, the `Worker` will loop over its receiving side of the channel
   and execute the closures of any jobs it receives.

Let’s start by creating a channel in `ThreadPool::new` and holding the sending
side in the `ThreadPool` instance, as shown in Listing 20-16. `Job` is the type
of item we’re going to be sending down the channel; it’s a struct that doesn’t
hold anything for now:

Filename: src/lib.rs

```
// ...snip...
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    // ...snip...
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // ...snip...
}
```

Listing 20-16: Modifying `ThreadPool` to store the sending end of a channel
that sends `Job` instances

In `ThreadPool::new`, we create our new channel, and then have the pool hang on
to the sending end. This will successfully compile, still with warnings.

Let’s try passing a receiving end of the channel into each worker when the
thread pool creates them. We know we want to use the receiving end of the
channel in the thread that the workers spawn, so we’re going to reference the
`receiver` parameter in the closure. The code shown here in Listing 20-17
won’t quite compile yet:

Filename: src/lib.rs

```
impl ThreadPool {
    // ...snip...
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // ...snip...
}

// ...snip...

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker {
            id,
            thread,
        }
    }
}
```

Listing 20-17: Passing the receiving end of the channel to the workers

These are small and straightforward changes: we pass in the receiving end of
the channel into `Worker::new`, and then we use it inside of the closure.

If we try to check this, we get this error:

```
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0382]: use of moved value: `receiver`
  --> src/lib.rs:27:42
   |
27 |             workers.push(Worker::new(id, receiver));
   |                                          ^^^^^^^^ value moved here in
   previous iteration of loop
   |
   = note: move occurs because `receiver` has type
   `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
```

The code as written won’t quite work since it’s trying to pass `receiver` to
multiple `Worker` instances. Recall from Chapter 16 that the channel
implementation provided by Rust is multiple *producer*, single *consumer*, so
we can’t just clone the consuming end of the channel to fix this. We also don’t
want to clone the consuming end even if we wanted to; sharing the single
`receiver` between all of the workers is the mechanism by which we’d like to
distribute the jobs across the threads.

Additionally, taking a job off the channel queue involves mutating `receiver`,
so the threads need a safe way to share `receiver` and be allowed to modify it.
If the modifications weren’t threadsafe, we might get race conditions such as
two threads executing the same job if they both take the same job off the queue
at the same time.

So remembering the threadsafe smart pointers that we discussed in Chapter 16,
in order to share ownership across multiple threads and allow the threads to
mutate the value, we need to use `Arc<Mutex<T>>`. `Arc` will let multiple
workers own the receiver, and `Mutex` will make sure that only one worker is
getting a job from the receiver at a time. Listing 20-18 shows the changes we
need to make:

Filename: src/lib.rs

```
use std::sync::Arc;
use std::sync::Mutex;

// ...snip...

impl ThreadPool {
    // ...snip...
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    // ...snip...
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // ...snip...
    }
}
```

Listing 20-18: Sharing the receiving end of the channel between the workers by
using `Arc` and `Mutex`

In `ThreadPool::new`, we put the receiving end of the channel in an `Arc` and a
`Mutex`. For each new worker, we clone the `Arc` to bump the reference count so
the workers can share ownership of the receiving end.

With these changes, the code compiles! We’re getting there!

Let’s finally implement the `execute` method on `ThreadPool`. We’re also going
to change the `Job` struct: instead of being a struct, `Job` is going to be a
type alias for a trait object that holds the type of closure that `execute`
receives. We discussed how type aliases can help make long types shorter, and
this is such a case! Take a look at Listing 20-19:

Filename: src/lib.rs

```
// ...snip...

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {
    // ...snip...

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// ...snip...
```

Listing 20-19: Creating a `Job` type alias for a `Box` that holds each closure,
then sending the job down the channel

After creating a new `Job` instance using the closure we get in
`execute`, we send that job down the sending end of the channel. We’re calling
`unwrap` on `send` since sending may fail if the receiving end has stopped
receiving new messages, which would happen if we stop all of our threads from
executing. This isn’t possible right now, though, since our threads continue
executing as long as the pool exists. We use `unwrap` since we know the failure
case won’t happen even though the compiler can’t tell that, which is an
appropriate use of `unwrap` as we discussed in Chapter 9.

Are we done yet? Not quite! In the worker, we’ve still got a closure being
passed to `thread::spawn` that only *references* the receiving end of the
channel. Instead, we need the closure to loop forever, asking the receiving end
of the channel for a job, and running the job when it gets one. Let’s make the
change shown in Listing 20-20 to `Worker::new`:

Filename: src/lib.rs

```
// ...snip...

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                (*job)();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

Listing 20-20: Receiving and executing the jobs in the worker’s thread

Here, we first call `lock` on the `receiver` to acquire the mutex, then
`unwrap` to panic on any errors. Acquiring a lock might fail if the mutex is in
a state called *poisoned*, which can happen if some other thread panicked while
holding the lock rather than releasing it. If this thread can’t get the lock
for that reason, calling `unwrap` to have this thread panic is the correct
action to take as well. Feel free to change this `unwrap` to an `expect` with
an error message that is meaningful to you if you’d like.

If we get the lock on the mutex, then we call `recv` to receive a `Job` from
the channel. A final `unwrap` moves past those errors as well. `recv` will
return `Err` if the thread holding the sending side of the channel has shut
down, similar to how the `send` method returns `Err` if the receiving side
shuts down.

The call to `recv` blocks; that is, if there’s no job yet, this thread will sit
here until a job becomes available. The `Mutex<T>` makes sure that only one
`Worker` thread at a time is trying to request a job.

Theoretically, this code should compile. Unfortunately, the Rust compiler isn’t
perfect yet, and we get this error:

```
error[E0161]: cannot move a value of type std::ops::FnOnce() +
std::marker::Send: the size of std::ops::FnOnce() + std::marker::Send cannot be
statically determined
  --> src/lib.rs:63:17
   |
63 |                 (*job)();
   |                 ^^^^^^
```

This error is fairly cryptic, and that’s because the problem is fairly cryptic.
In order to call a `FnOnce` closure that is stored in a `Box<T>` (which is what
our `Job` type alias is), the closure needs to be able to move itself out of
the `Box<T>` since when we call the closure, it takes ownership of `self`. In
general, moving a value out of a `Box<T>` isn’t allowed since Rust doesn’t know
how big the value inside the `Box<T>` is going to be; recall in Chapter 15 that
we used `Box<T>` precisely because we had something of an unknown size that we
wanted to store in a `Box<T>` to get a value of a known size.

We saw in Chapter 17, Listing 17-15 that we can write methods that use the
syntax `self: Box<Self>` so that the method takes ownership of a `Self` value
that is stored in a `Box<T>`. That’s what we want to do here, but unfortunately
the part of Rust that implements what happens when we call a closure isn’t
implemented using `self: Box<Self>`. So Rust doesn’t yet understand that it
could use `self: Box<Self>` in this situation in order to take ownership of the
closure and move the closure out of the `Box<T>`.

In the future, the code in Listing 20-20 should work just fine. Rust is still a
work in progress with places that the compiler could be improved. There are
people just like you working to fix this and other issues! Once you’ve finished
the book, we would love for you to join in.

But for now, let’s work around this problem. Luckily, there’s a trick that
involves telling Rust explicitly that we’re in a case where we can take
ownership of the value inside the `Box<T>` using `self: Box<Self>`, and once we
have ownership of the closure, we can call it. This involves defining a new
trait that has a method `call_box` that uses `self: Box<Self>` in its
signature, defining that trait for any type that implements `FnOnce()`,
changing our type alias to use the new trait, and changing `Worker` to use the
`call_box` method. These changes are shown in Listing 20-21:

Filename: src/lib.rs

```
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

// ...snip...

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

Listing 20-21: Adding a new trait `FnBox` to work around the current
limitations of `Box<FnOnce()>`

First, we create a new trait named `FnBox`. This trait has one method,
`call_box`, similar to the `call` methods on the other `Fn*` traits, except
this method takes `self: Box<Self>` in order to take ownership of `self` and
move the value out of the `Box<T>`.

Next, we implement the `FnBox` trait for any type `F` that implements the
`FnOnce()` trait. Effectively, this means that any `FnOnce()` closures can use
our `call_box` method. The implementation of `call_box` uses `(*self)()` to
move the closure out of the `Box<T>` and call the closure.

Instead of `FnOnce()`, we now want our `Job` type alias to be a `Box` of
anything that implements our new trait `FnBox`. This will allow us to use
`call_box` in `Worker` when we get a `Job` value. Because we implemented the
`FnBox` trait for any `FnOnce()` closure, we don’t have to change anything
about the actual values we’re sending down the channel.

Finally, in the closure run in the thread in `Worker::new`, we use `call_box`
instead of invoking the closure directly. Now Rust is able to understand that
what we want to do is fine.

This is a very sneaky, complicated trick. Don’t worry too much if it doesn’t
make perfect sense; someday, it will be completely unnecessary.

With this trick, our thread pool is in a working state! Give it a `cargo run`,
and make some requests:

```
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field is never used: `workers`
 --> src/lib.rs:7:5
  |
7 |     workers: Vec<Worker>,
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: field is never used: `id`
  --> src/lib.rs:61:5
   |
61 |     id: usize,
   |     ^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

warning: field is never used: `thread`
  --> src/lib.rs:62:5
   |
62 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.99 secs
     Running `target/debug/hello`
     Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

Success! We now have a thread pool executing connections asynchronously. We
never create more than four threads, so our system won’t get overloaded if the
server gets a lot of requests. If we make a request to `/sleep`, the server
will be able to serve other requests by having another thread run them.

What about those warnings, though? Don’t we use the `workers`, `id`, and
`thread` fields? Well, right now, we’re using all three of these fields to hold
onto some data, but we don’t actually *do* anything with the data once we’ve
set up the thread pool and started running the code that sends jobs down the
channel to the threads. If we didn’t hold onto these values, though, they’d go
out of scope: for example, if we didn’t return the `Vec<Worker>` value as part
of the `ThreadPool`, the vector would get cleaned up at the end of
`ThreadPool::new`.

So are these warnings wrong? In one sense yes, the warnings are wrong, since we
are using the fields to store data we need to keep around. In another sense,
no, the warnings aren’t wrong, and they’re telling us that we’ve forgotten to
do something: we never do anything to clean up our thread pool once it’s done
being used, we just use <span class="keystroke">ctrl-C</span> to stop the
program and let the operating system clean up after us. Let’s implement a
graceful shutdown that cleans up everything we’ve created instead.

## Graceful Shutdown and Cleanup

The code in Listing 20-21 is responding to requests asynchronously through the
use of a thread pool, as we intended. We get some warnings about fields that
we’re not using in a direct way, which are a reminder that we’re not cleaning
anything up. When we use <span class="keystroke">ctrl-C</span> to halt the main
thread, all the other threads are stopped immediately as well, even if they’re
in the middle of serving a request.

We’re now going to implement the `Drop` trait for `ThreadPool` to call `join`
on each of the threads in the pool so that the threads will finish the requests
they’re working on. Then we’ll implement a way for the `ThreadPool` to tell the
threads they should stop accepting new requests and shut down. To see this code
in action, we’ll modify our server to only accept two requests before
gracefully shutting down its thread pool.

Let’s start with implementing `Drop` for our thread pool. When the pool is
dropped, we should join on all of our threads to make sure they finish their
work. Listing 20-22 shows a first attempt at a `Drop` implementation; this code
won’t quite work yet:

Filename: src/lib.rs

```
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

Listing 20-22: Joining each thread when the thread pool goes out of scope

We loop through each of the thread pool `workers`, using `&mut` because `self`
is itself a mutable reference and we also need to be able to mutate `worker`.
We print out a message saying that this particular worker is shutting down, and
then we call `join` on that worker’s thread. If the call to `join` fails, we
`unwrap` the error to panic and go into an ungraceful shutdown.

Here’s the error we get if we compile this code:

```
error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:65:13
   |
65 |             worker.thread.join().unwrap();
   |             ^^^^^^ cannot move out of borrowed content
```

Because we only have a mutable borrow of each `worker`, we can’t call `join`:
`join` takes ownership of its argument. In order to solve this, we need a way
to move the `thread` out of the `Worker` instance that owns `thread` so that
`join` can consume the thread. We saw a way to do this in Listing 17-15: if the
`Worker` holds an `Option<thread::JoinHandle<()>` instead, we can call the
`take` method on the `Option` to move the value out of the `Some` variant and
leave a `None` variant in its place. In other words, a `Worker` that is running
will have a `Some` variant in `thread`, and when we want to clean up a worker,
we’ll replace `Some` with `None` so the worker doesn’t have a thread to run.

So we know we want to update the definition of `Worker` like this:

Filename: src/lib.rs

```
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

Now let’s lean on the compiler to find the other places that need to change. We
get two errors:

```
error: no method named `join` found for type
`std::option::Option<std::thread::JoinHandle<()>>` in the current scope
  --> src/lib.rs:65:27
   |
65 |             worker.thread.join().unwrap();
   |                           ^^^^

error[E0308]: mismatched types
  --> src/lib.rs:89:21
   |
89 |             thread,
   |             ^^^^^^ expected enum `std::option::Option`, found
   struct `std::thread::JoinHandle`
   |
   = note: expected type `std::option::Option<std::thread::JoinHandle<()>>`
              found type `std::thread::JoinHandle<_>`
```

The second error is pointing to the code at the end of `Worker::new`; we need
to wrap the `thread` value in `Some` when we create a new `Worker`:

Filename: src/lib.rs

```
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // ...snip...

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

The first error is in our `Drop` implementation, and we mentioned that we’ll be
calling `take` on the `Option` value to move `thread` out of `worker`. Here’s
what that looks like:

Filename: src/lib.rs

```
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

As we saw in Chapter 17, the `take` method on `Option` takes the `Some` variant
out and leaves `None` in its place. We’re using `if let` to destructure the
`Some` and get the thread, then call `join` on the thread. If a worker’s thread
is already `None`, then we know this worker has already had its thread cleaned
up so we don’t do anything in that case.

With this, our code compiles without any warnings. Bad news though, this code
doesn’t function the way we want it to yet. The key is the logic in the
closures that the spawned threads of the `Worker` instances run: calling `join`
won’t shut down the threads since they `loop` forever looking for jobs. If we
try to drop our `ThreadPool` with this implementation, the main thread will
block forever waiting for the first thread to finish.

To fix this, we’re going to modify the threads to listen for either a `Job` to
run or a signal that they should stop listening and exit the infinite loop. So
instead of `Job` instances, our channel will send one of these two enum
variants:

Filename: src/lib.rs

```
enum Message {
    NewJob(Job),
    Terminate,
}
```

This `Message` enum will either be a `NewJob` variant that holds the `Job` the
thread should run, or it will be a `Terminate` variant that will cause the
thread to exit its loop and stop.

We need to adjust the channel to use values of type `Message` rather than type
`Job`, as shown in Listing 20-23:

Filename: src/lib.rs

```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// ...snip...

impl ThreadPool {
    // ...snip...
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // ...snip...
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// ...snip...

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

Listing 20-23: Sending and receiving `Message` values and exiting the loop if a
`Worker` receives `Message::Terminate`

We need to change `Job` to `Message` in the definition of `ThreadPool`, in
`ThreadPool::new` where we create the channel, and in the signature of
`Worker::new`. The `execute` method of `ThreadPool` needs to send jobs wrapped
in the `Message::NewJob` variant. Then, in `Worker::new` where we receive a
`Message` from the channel, we’ll process the job if we get the `NewJob`
variant and break out of the loop if we get the `Terminate` variant.

With these changes, the code will compile again and continue to function in the
same way as it has been. We’ll get a warning, though, because we aren’t using
the `Terminate` variant in any messages. Let’s change our `Drop` implementation
to look like Listing 20-24:

Filename: src/lib.rs

```
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

Listing 20-24: Sending `Message::Terminate` to the workers before calling
`join` on each worker thread

We’re now iterating over the workers twice, once to send one `Terminate`
message for each worker, and once to call `join` on each worker’s thread. If we
tried to send a message and join immediately in the same loop, it’s not
guaranteed that the worker in the current iteration will be the one that gets
the message from the channel.

To understand better why we need two separate loops, imagine a scenario with
two workers. If we iterated through each worker in one loop, on the first
iteration where `worker` is the first worker, we’d send a terminate message
down the channel and call `join` on the first worker’s thread. If the first
worker was busy processing a request at that moment, the second worker would
pick up the terminate message from the channel and shut down. We’re waiting on
the first worker to shut down, but it never will since the second thread picked
up the terminate message. We’re now blocking forever waiting for the first
worker to shut down, and we’ll never send the second message to terminate.
Deadlock!

To prevent this, we first put all of our `Terminate` messages on the channel,
and then we join on all the threads. Because each worker will stop receiving
requests on the channel once it gets a terminate message, we can be sure that
if we send the same number of terminate messages as there are workers, each
worker will receive a terminate message before we call `join` on its thread.

In order to see this code in action, let’s modify `main` to only accept two
requests before gracefully shutting the server down as shown in Listing 20-25:

Filename: src/bin/main.rs

```
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    let mut counter = 0;

    for stream in listener.incoming() {
        if counter == 2 {
            println!("Shutting down.");
            break;
        }

        counter += 1;

        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

Listing 20-25: Shut down the server after serving two requests by exiting the
loop

Only serving two requests isn’t behavior you’d like a production web server to
have, but this will let us see the graceful shutdown and cleanup working since
we won’t be stopping the server with <span class="keystroke">ctrl-C</span>.

We’ve added a `counter` variable that we’ll increment every time we receive an
incoming TCP stream. If that counter reaches 2, we’ll stop serving requests and
instead break out of the `for` loop. The `ThreadPool` will go out of scope at
the end of `main`, and we’ll see the `drop` implementation run.

Start the server with `cargo run`, and make three requests. The third request
should error, and in your terminal you should see output that looks like:

```
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0 secs
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 3 got a job; executing.
Shutting down.
Sending terminate message to all workers.
Shutting down all workers.
Shutting down worker 0
Worker 1 was told to terminate.
Worker 2 was told to terminate.
Worker 0 was told to terminate.
Worker 3 was told to terminate.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

You may get a different ordering, of course. We can see how this works from the
messages: workers zero and three got the first two requests, and then on the
third request, we stop accepting connections. When the `ThreadPool` goes out of
scope at the end of `main`, its `Drop` implementation kicks in, and the pool
tells all workers to terminate. The workers each print a message when they see
the terminate message, and then the thread pool calls `join` to shut down each
worker thread.

One interesting aspect of this particular execution: notice that we sent the
terminate messages down the channel, and before any worker received the
messages, we tried to join worker zero. Worker zero had not yet gotten the
terminate message, so the main thread blocked waiting for worker zero to
finish. In the meantime, each of the workers received the termination messages.
Once worker zero finished, the main thread waited for the rest of the workers
to finish, and they had all received the termination message and were able to
shut down at that point.

Congrats! We now have completed our project, and we have a basic web server
that uses a thread pool to respond asynchronously. We’re able to perform a
graceful shutdown of the server, which cleans up all the threads in the pool.
Here’s the full code for reference:

Filename: src/bin/main.rs

```
extern crate hello;
use hello::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    let mut counter = 0;

    for stream in listener.incoming() {
        if counter == 2 {
            println!("Shutting down.");
            break;
        }

        counter += 1;

        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

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

     let mut file = File::open(filename).unwrap();
     let mut contents = String::new();

     file.read_to_string(&mut contents).unwrap();

     let response = format!("{}{}", status_line, contents);

     stream.write(response.as_bytes()).unwrap();
     stream.flush().unwrap();
}
```

Filename: src/lib.rs

```
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

There’s more we could do here! If you’d like to continue enhancing this
project, here are some ideas:

- Add more documentation to `ThreadPool` and its public methods
- Add tests of the library’s functionality
- Change calls to `unwrap` to more robust error handling
- Use `ThreadPool` to perform some other task rather than serving web requests
- Find a thread pool crate on crates.io and implement a similar web server
  using the crate instead and compare its API and robustness to the thread pool
  we implemented

## Summary

Well done! You’ve made it to the end of the book! We’d like to thank you for
joining us on this tour of Rust. You’re now ready to go out and implement your
own Rust projects or help with other people’s. Remember there’s a community of
other Rustaceans who would love to help you with any challenges you encounter
on your Rust journey.
