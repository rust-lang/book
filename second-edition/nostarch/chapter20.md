
[TOC]

# Final Project: Building a Multithreaded Web Server

It’s been a long journey, but here we are! The end of the book. Parting is such
sweet sorrow. But before we go, let’s build one more project together, to show
off some of the things we learned in these final chapters, as well as re-cap
some lessons from earlier.

For our final projct we’re going to make a really simple web server that just
says "hello"; something like Figure 20-1.

<img src="trpl20-01.png" />

Figure 20-1: Our final shared project together

To build our web server, you'll need to:

1. Learn a little bit about TCP and HTTP
2. Listen for TCP connections on a socket
3. Parse a small number of HTTP requests
4. Create a proper HTTP response
5. Improve the throughput of our server with a thread pool

Before we get started, however, there’s one thing we should mention: the method
we use here will not be the most direct of efficient way to build a web server
with Rust. There are a number of robust crates available on *http://crates.io*
that provide much more complete web server and thread pool implementations than
we are going to build.

However, for this chapter, our intention is to help you learn, not to take the
easy route. Since Rust is a systems programming language, we’re able to choose
what level of abstraction we want to work with, and can go to a lower level
than is possible or practical in other languages. We'll therefore write the
basic HTTP server and thread pool ourselves so you can learn the general ideas
and techniques behind the crates you might use in the future.

## Building a Single Threaded Web Server

First we'll get a single threaded web server working, but before we begin let's
look at a quick overview of the protocols involved in building web servers. The
details of these protocols are beyond the scope of this book, but a short
overview will give you the information you need.

The two main protocols involved in web servers are the *Hypertext Transfer
Protocol* (*HTTP*) and the *Transmission Control Protocol* (*TCP*). Both
protocols are *request-response* protocols, meaning a *client* initiates
requests, and a *server* listens to the requests and provides a response to the
client. The contents of those requests and responses are defined by the
protocols themselves.

TCP is the lower-level protocol that describes the low-level details of how
information gets from one server to another, but doesn’t specify what that
information is. HTTP builds on top of TCP by defining the content of the
requests and responses. It’s technically possible to use HTTP with other
protocols, but in the vast majority of cases, HTTP sends its data over TCP.
We're going to work with the raw bytes of TCP and HTTP requests and responses.

### Listening to the TCP Server

Our web server needs to be able to listen to a TCP server, so that's the first
thing we'll work on. The standard library offers a `std::net` module that lets
us do this. Let’s make a new project in the usual fashion:

```
$ cargo new hello --bin
     Created binary (application) `hello` project
$ cd hello
```

Now enter the code in Listing 20-1 in `src/main.rs` to start. This code will
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

The `TcpListener` method allows us to listen for TCP connections. We’ve chosen
to listen to the address `127.0.0.1:8080`, which is a port on the machine
you're using. Breaking this address down, the section before the colon is an IP
address representing your own computer (this is the same on each computer, and
doesn't represent the authors' computer specifically), and `8080` is the port.
We’ve chosen this port for two reasons: HTTP is normally accepted on this port
and 8080 is easy to remember since it’s the HTTP port 80 repeated. Note that
connecting to port 80 does require administrator privileges; non-administraors
can only listen on ports higher than 1024.

The `bind` function in this scenario works like the `new` function, in that it
will return a new `TcpListener` instance. This functon is called `bind` in the
`net` module because, in networking, connecting to a port to listen to is known
as “binding to a port”.

The `bind` function returns a `Result<T, E>` to dictate what happens if it
fails. Binding may fail if, for example, we tried to connect to port 80 without
being an administrator, or we ran two instances of our program and so had two
programs listening to the same port. Since we’re writing a basic server for
learning purposes here, we’re not going to worry about handling these kinds of
errors, so we just use `unwrap` to stop the program if errors happen.

The `incoming` method on `TcpListener` returns an iterator that gives us a
sequence of streams (more specifically, streams of type `TcpStream`). A single
*stream* represents an open connection between the client and the server. A
*connection* is the name for the full request/response process in which a
client connects to the server, the server generates a response, and the server
closes the connection. As such, `TcpStream` will read from itself to see what
the client sent, and allow us to write our response to the stream. Overall,
this `for` loop will process each connection in turn and produce a series of
streams for us to handle.

<!-- Below -- What if there aren't errors, how is the stream handled? Or is
there no functionality for that yet, only functionality for errors?

Also, highlighted below -- can you specify what errors we're talking
about---errors in *producing* the streams or connecting to the port?-->

For now, our handling of the stream consists of calling `unwrap` to terminate
our program if the stream has any errors, then printing a message. At the
moment errors can happen ==when producing streams== because we’re not actually
iterating over connections, we’re iterating over *connection attempts*. The
connection might not take for a number of reasons, many of them
operating-system specific. For example, many operating systems have a limit to
the number of simultaneous open connections they can support; new connection
attempts beyond that number will produce an error until some of the open
connections are closed.

Let’s try this code out! First invoke `cargo run` in the terminal, then load up
`127.0.0.1:8080` in a web browser. The browser should show an error message
like “Connection reset”, since we’re not currently sending any data back. If
you look at your terminal, though, you should see a bunch of messages that were
printed when the browser connected to the server!

```
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

Sometimes, we'll get multiple messages printed out for one browser request;
that might be becuase the browser is making a request for the page as well as a
request for other things, like the `favicon.ico` icon that appears in the
browser tab.

It could also be that the browser retrying to connect again in each iteration
of the loop. When `stream` goes out of scope and drops at the end of the loop,
the connection is closed as part of the `drop` implementation, and browsers
sometimes deal with closed connections by retrying, since the problem might be
temporary. The important thing is that we’ve successfully gotten a handle on a
TCP connection!

Remember to stop the program with <span class="keystroke">ctrl-C</span> when
you’re done running a particular version of the code, and restart `cargo run`
after you’ve made each set of code changes to make sure you're running the
newest code.

### Reading the Request

Let’s create the functionality to read in the request from our browser! To
separate out our concerns, we'll start a new function for processing
connections. In this new `handle_connection` function, we’ll read data from
`stream` and print it out so we can see the data being sent from the browser.
Change the code to look like Listing 20-2:

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

We import from `std::io::prelude` to bring certain traits into scope that let
us read from and write to the stream. In the main `for` loops, instead of
printing a message that says we made a connection, we now call the new
`handle_connection` function and pass the `stream` to it.

In the `handle_connection` function, we've made the `stream` parameter mutable.
This is because, the `TcpStream` instance keeps track of what data it returns
to us ==internally, and it might read more than we asked for into a buffer. It
therefore needs to be `mut` because its state might change;== usually we think
of “reading” as not needing mutation, but in this case we need the `mut`
keyword.

<!-- Above -- I'm not clear what state will change here, the content of stream
when the program tempers what data it takes? -->

Next, we need to actually read from the stream. We do this in two steps: first,
we declare a `buffer` on the stack to hold the data that's read in. We’ve made
the buffer 512 bytes in size, which is big enough to hold the data of a basic
request and sufficient for our purposes in this chapter. If we wanted to handle
requests of an arbitrary size, the management of the buffer would need to be
more complicated, but we’re keeping it simple for now. We pass the buffer to
`stream.read`, which will read bytes from the `TcpStream` and put them in the
buffer.

We then convert the bytes in the buffer to a string and print out that string.
The `String::from_utf8_lossy` function takes a `&[u8]` and produces a `String`
from it. The ‘lossy’ part of the name indicates the behavior of this function
when it sees an invalid UTF-8 sequences: it will replace the invalid sequences
with �, the `U+FFFD REPLACEMENT CHARACTER`. You might see replacement
characters for characters in the buffer that aren’t filled by request data.

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

You’ll probably get slightly different output depending on your browser. Now
that we’re printing out the request data, we can see why we get multiple
connections from one browser request by looking at the path after `Request:
GET`. If the repeated connections are all requesting `/`, we know the browser
is trying to fetch `/` repeatedly because it’s not getting a response from our
program.

Let’s break down this request data to understand what the browser is asking of
our program.

#### [potential heading, something like A Closer Look at an HTTP Request]

HTTP is a text-based protocol, and a request takes this format:

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

First we have the *request line* that holds information about what the client
is requesting. The first part of the request line tells us the *method* being
used, like `GET` or `POST`, that describes how the client is making this
request. Our client used a `GET` request.

<!-- Below, is that right that the / part is the URI *being requested*, and not
the URI of the requester? -->

The next part of the `Request` line is `/` which tell us the *URI* (Uniform
Resource Identifier) of the client is requesting---a URI is almost, but not
quite, the same as a URL (*Uniform Resource Locators*). The difference between
URIs and URLs isn’t important for our purposes of this chapter, but the HTTP
spec uses the term URI, so we can just mentally substitute URL for URI here.

Finally, we're given the HTTP version used by the client, and then the request
line ends in a CRLF sequence. The CRLF sequence can also be written as `\r\n`:
`\r` is a *carriage return* and `\n` is a *line feed*. (These terms come from
the typewriter days!) The CRLF sequence separates the request line from the
rest of the request data.

<!-- Above, I don't see a CRLF here in the request line in the actual output,
is it just implied because the next line begins on the next line? -->

Taking a look at the request line data we received in full from running our
program so far, we see that`GET` is the method, `/` is the Request URI, and
`HTTP/1.1` is the version.

The remaining lines starting from `Host:` onward are headers; `GET` requests
have no body.

Try making a request from a different browser, or asking for a different
address like `127.0.0.1:8080/test` to see how the request data changes, if
you’d like.

Now that we know what the browser is asking for, let’s send some data back!

### Writing a Response

Now we want to send data in reponse to a client request. Responses have the
following format:

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

The first line is a *status line* that contains the HTTP version used in the
response, a numeric status code that summarizes the result of the request, and
a reason phrase that provides a text description of the status code. After the
CRLF sequence comes any headers, another CRLF sequence, and the body of the
response.

Here’s an example response that uses version 1.1 of HTTP, has a status code of
`200`, a reason phrase of `OK`, no headers, and no body:

```
HTTP/1.1 200 OK\r\n\r\n
```


The status code 200 is the standard success response. The text is a tiny
successful HTTP response. Let’s write this to the stream as our response to a
successful request!

From the `handle_connection` function, we need to remove the `println!` that
was printing the request data, and replace it with the code in Listing 20-3:

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

<!-- Flagging for addition of wingdings later -->

The first new line defines the `response` variable that holds the data of the
success message. Then we call `as_bytes` on our `response` because the `write`
method on `stream` takes a `&[u8]` and sends those bytes directly down the
connection.

<!-- Above--So what does adding as_bytes actually do, *allow* us to send bytes
directly? -->

Since the `write` operation could fail, we set `write` to return a `Result<T,
E>`, with `unwrap` in the error result as before. Again, in a real application
you would add error-handling here. Finally, `flush` will wait until all of the
bytes are written to the connection; `TcpStream` contains an internal buffer to
minimize calls into the underlying operating system.

<!-- Above -- Will flush wait until all bytes are written and then do
something? I'm not sure what task it's performing -->

With these changes, let’s run our code and make a request! We’re no longer
printing any data to the terminal, so we won’t see any output other than the
output from Cargo. Load `127.0.0.1:8080` in a web browser, though, and you
should get a blank page instead of an error. How exciting! You’ve just
hand-coded an HTTP request and response.

### Returning Real HTML

Let’s set our application to return more than a blank page. Create a new file,
*hello.html*, in the root of your project directory---that is, not in the `src`
directory. You can put in any HTML you want; Listing 20-4 shows what we, the
authors, used for ours:

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

This is a minimal HTML 5 document with a heading and some text. To show this on
the page when a request is received, let’s modify `handle_connection` as shown
in Listing 20-5 to read the HTML file, add it to the response as a body, and
send it:

Filename: src/main.rs

```
use std::fs::File;

// --snip--

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
scope. The code for opening files and reading code should look familiar from
Chapter 12, when we read the contents of a file for our I/O project in Listing
12-4.

Next, we’re using `format!` to add the file’s contents as the body of the
success response.

Run this code with `cargo run`, load up `127.0.0.1:8080` in your browser, and
you should see your HTML rendered!

Currently we're ignoring the request data in `buffer` and just sending back the
contents of the HTML file unconditionally. That means if you try requesting
`127.0.0.1:8080/something-else` in your browser you’ll still get back this same
HTML response, even though the URI doesn't precisely match. This makes for a
pretty limited server and is not what most web servers do. We need to customize
our responses depending on the request, and only send back the HTML file for a
well-formed request to `/`.

### Validating the Request and Selectively Responding

Right now, our web server will return the HTML in the file no matter what the
client requested. Let’s add functionality to check that the browser is
requesting `/` before returning the HTML file, and return an error if the
browser requests anything else. For this we need to modify `handle_connection`
as shown in Listing 20-6. This new code checks the content of the request
received against what we know a request for `/` looks like and adds `if` and
`else` blocks to treat requests differently:

Filename: src/main.rs

```
// --snip--

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

Listing 20-6: Matching the request and handling requests to `/` differently to
other requests

First, we hardcode the data corresponding to the correct `/` request into the
`get` variable. Because we’re reading raw bytes into the buffer, we transform
`get` into a byte string by adding the `b""` byte string syntax at the start of
the content data. Then, we check to see if `buffer` starts with the bytes in
`get`. If it does, it means we've received a well-formed request to `/`, which
is the success case we'll handle in the `if` block that returns the contents of
our HTML file.

If `buffer` does *not* start with the bytes in `get`, it means we’ve received
some other request. We’ll add code to the `else` block in a moment to respond
to all other requests.

Run this code now and request `127.0.0.1:8080`, and you should get the HTML in
*hello.html*. If you make any other request, such as
`127.0.0.1:8080/something-else`, you’ll get a connection error like we saw when
running the code in Listing 20-1 and Listing 20-2.

Now let’s add the code in Listing 20-7 to the `else` block to return a response
with the status code `404`, which signals that the content for the request was
not found. We’ll also create some HTML for a page to render in the browser
indicating as such to the end user:

Filename: src/main.rs

```
// --snip--

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
phrase `NOT FOUND`. We're still not returning headers, and the body of the
response will be the HTML in the file *404.html*. You'll need to create a
*404.html* file next to *hello.html* for the error page; again feel free to use
any HTML you’d like or use the example HTML in Listing 20-8:

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

### A Touch of Refactoring

At the moment our `if` and `else` blocks have a lot of repetition: they’re both
reading files and writing the contents of the files to the stream. The only
differences are the status line and the filename. Let’s make our code more
efficient by pulling those differences out into an `if` and `else` of one line
each that will assign the values of the status line and the filename to
variables; we can then use those variables unconditionally in the code to read
the file and write the response. The resulting code after this refactoring is
shown in Listing 20-9:

Filename: src/main.rs

```
// --snip--

fn handle_connection(mut stream: TcpStream) {
    // --snip--

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

Now the `if` and `else` blocks simply return the appropriate values for the
status line and filename in a tuple; we then use destructuring to assign these
two values to `status_line` and `filename` using a pattern in the `let`
statement like we discussed in Chapter 18.

The duplicated code is outside the `if` and `else` blocks, and uses the
`status_line` and `filename` variables. This makes it easier to see exactly
what’s different between the two cases, and means we have only one place to
update the code if we want to change how the file reading and response writing
works. The behavior of the code in Listing 20-9 will be exactly the same as
that in Listing 20-8.

Awesome! We have a simple little web server in about 40 lines of Rust code that
responds to one request with a page of content and responds to all other
requests with a `404` response.

Currently our server runs in a single thread, meaning it can only serve one
request at a time. Let’s see how that can be a problem by simulating some slow
requests, and then fix it so our server can handle multiple requests at once.

## Turning our Single-Threaded Server into a Multi-Threaded Server

<!-- Reading ahead, the original heading didn't seem to fit all of the sub
headings -- this might not be totally right either, so feel free to replace
with something more appropriate -->

Right now, the server will process each request in turn, meaning it won't
process a second connection until the first is finished processing. That works
for services like ours that aren’t expected to get very many requests, but as
applications get more complex, this sort of serial execution isn’t optimal. If
the server recieves a request that takes a long time to process, subsequent
requests will have to wait until the long request is finished, even if the new
requests can be processed quickly. We'll need to fix this, but first, we'll
look at the problem in action.

### Simulating a Slow Request in the Current Server Implementation

Let’s see how a slow-processing request can effect other requests made to our
current server implementation. Listing 20-10 simulates a slow request with a
`/sleep` request that will cause the server to sleep for five seconds before
responding.

Filename: src/main.rs

```
use std::thread;
use std::time::Duration;
// --snip--

fn handle_connection(mut stream: TcpStream) {
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

Listing 20-10: Simulating a slow request by recognizing `/sleep` and sleeping
for 5 seconds

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
`sleep` has slept for its full five seconds before moving on.

There are multiple ways we could change how our web server works in order to
avoid having all requests back up behind a slow request; the one we’re going to
implement is a thread pool.

### Improving Throughput with a Thread Pool

<!--There seems to be some repetition throughout these thread pool sections, is
there any way to condense it? I've edited with this in mind, but am wary of
changing too much -->

A *thread pool* is a group of spawned threads that are waiting ready to handle
some task. When the program receives a new task, it will assign one of the
threads in the pool to the task, and that thread will go off and process the
task. The remaining threads in the pool are available to handle any other tasks
that come in while the first thread is processing. When the first thread is
done processing its task, it's returned to the pool of idle threads ready to
handle a new task. A thread pool will allow us to process connections
concurrently, increasing the throughput of our server.

We’ll limit the number of threads in the pool to a small number to protect us
from DDOS attacks; if we had our program create a new thread for each request
as it come in, someone making ten million requests to our server could create
havoc by using up all of our server’s resources and grinding the processing of
all requests to a halt.

Rather than spawning unlimited threads, then, we’ll have a fixed number of
threads waiting in the pool. As requests come in, they'll be sent to the pool
for processing. The pool will maintain a queue of incoming requests. Each of
the threads in the pool will pop a request off of this queue, handle the
request, and then ask the queue for another request. With this design, we can
process `N` requests concurrently, where `N` is the number of threads. Of
course, if each thread is responding to a long-running requests, subsequent
requests can stil back up in the queue, but we’ve increased the number of
long-running requests we can handle before that point.

This is just one of many ways to improve the throughput of our web server.
Other options you might explore are the fork/join model and the single threaded
async I/O model. If you’re interested in this topic, you may want to read more
about other solutions and try to implement them in Rust; with a low-level
language like Rust, all of these options are possible.

Before we begin let’s talk about what using the pool should look like. When
trying to design code, writing the client interface first can really help guide
your design. Write the API of the code so that it's structured in the way you’d
want to call it, then implement the functionality within that structure, rather
than implementing the functionality then designing the public API.

Similar to how we used Test Driven Development in the project in Chapter 12,
we’re going to use Compiler Driven Development here. We’ll write the code that
calls the functions we wish we had, then we’ll look at errors from on the
compiler to tell us what we should change next to get things working.

#### Code Structure if We Could Spawn a Thread for Each Request

First, let’s explore how our code might look like if it did create a new thread
for every connection. As mentioned, this isn’t our final plan due to the
problems with potentially spawning an unlimited number of threads, but it’s a
starting point. Listing 20-11 shows the changes to make to `main` to spawn a
new thread to handle each stream within the `for` loop:

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
run the code in the closure in the new thread. If you run this code and load
`/sleep` in your browser, then `/` in two more browser tabs, you’ll indeed see
the request to `/` doesn’t have to wait for `/sleep` to finish. But as we
mentioned, this will eventually overwhelm the system since we’re making new
threads without any limit.

#### Creating a Similar Interface for a Finite Number of Threads

We want our thread pool to work in a similar, familiar way so that switching
from threads to a thread pool doesn’t require large changes to the code.
Listing 20-12 shows the hypothetical interface for a `ThreadPool` struct we’d
like to use instead of `thread::spawn`:

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

Listing 20-12: Our ideal `ThreadPool` structure

We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, in this case four. Then, in the `for` loop, `pool.execute` will
work in a similar way to `thread::spawn`, in that it wil XXXX. This code will
not yet work, but we're going to try to run it so the compiler can guide us in
how to fix it.

<!-- Can you be more specific here about how pool.execute will work? -->

#### Building the ThreadPool Struct Using Compiler Driven Development

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

Great, this is telling us we need a `ThreadPool` type or module, so we'll build
one now. Our `threadpool` implementation will be independent of the kind of
work our web server is doing, so let’s switch the `hello` crate from a binary
crate to a library crate to hold our `ThreadPool` implementation. This also
means we could use the separate thread pool library for whatever work we want
to do, not just for serving web requests.

Create a *src/lib.rs* that contains the following, which is simplest definition
of a `ThreadPool` struct that we can have for now:

Filename: src/lib.rs

```
pub struct ThreadPool;
```

Then create a new directory, *src/bin*, and move the binary crate rooted in
*src/main.rs* into *src/bin/main.rs*. This will make the library crate the
primary crate in the *hello* directory; we can still run the binary in
*src/bin/main.rs* using `cargo run` though. After moving the *main.rs* file,
edit it to bring the library crate in and bring `ThreadPool` into scope by
adding the following code to the top of *src/bin/main.rs*:

Filename: src/bin/main.rs

```
extern crate hello;
use hello::ThreadPool;
```

This still won't work, but let's try running it again in order to get the next
error that we need to address:

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

Cool, this tells us that next we need to create an associated function named
`new` for `ThreadPool`. We also know that `new` needs to have one parameter
that can accept `4` as an argument, and should return a `ThreadPool` instance.
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
negative number of threads makes no sense. Once we actually implement `new` for
real, we’ll reconsider whether this is the right choice for what the
implementation needs, but for now, we’re just working through compiler errors.

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

<!--Can you say a few words on why we would need an execute method, what Rust
needs it for? Also why we need a closure/what indicated that we need a closure
here? -->

Now we get a warning and an error. Ignoring the warning for a moment, the error
occurs because we don’t have an `execute` method on `ThreadPool`. We'll define
one that takes a closure. If you remember from Chapter 13, we can take closures
as arguments with three different traits: `Fn`, `FnMut`, and `FnOnce`. We need
to decide which kind of closure to use here. We know we’re going to end up
doing something similar to the standard library `thread::spawn` implementation,
so we can look at what bounds the signature of `thread::spawn` has on its
argument. The documentation tells us:

```
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`F` is the parameter we care about here; `T` is related to the return value and
we’re not concerned with that. We can see that `spawn` uses `FnOnce` as the
trait bound on `F`. This is probably what we want as well, since we’ll
eventually be passing the argument we get in `execute` to `spawn`. We can be
further confident that `FnOnce` is the trait we want to use since the thread
for running a request is only going to execute that request’s closure one time.

<!-- Above -- why does that second reason mean FnOnce is the trait to use, can
you remind us? -->

`F` also has the trait bound `Send` and the lifetime bound `'static`, which are
useful for our situation: we need `Send` to transfer the closure from one
thread to another, and `'static` because we don’t know how long the thread will
take to execute. Let’s create an `execute` method on `ThreadPool` that will
take a generic parameter `F` with these bounds:

Filename: src/lib.rs

```
impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}
```

We still use the `()` after `FnOnce` since this `FnOnce` is representing a
closure that takes no parameters and doesn’t return a value. Just like function
definitions, the return type can be omitted from the signature, but even if we
have no parameters, we still need the parentheses.

Again, we’ll add the simplest implementation of the `execute` method, which
does nothing, just to get our code working. Let’s check it again:

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

We're receiving only warnings now! That means it compiles! Note, though, that
if you try `cargo run` and make a request in the browser, you’ll see the errors
in the browser that we saw in the beginning of the chapter. Our library isn’t
actually calling the closure passed to `execute` yet!

> A saying you might hear about languages with strict compilers like Haskell
> and Rust is “if the code compiles, it works.” This is a good time to remember
> that this is is not actually universally true. Our project compiles, but it
> does absolutely nothing! If we were building a real, complete project, this
> would be a great time to start writing unit tests to check that the code
> compiles *and* has the behavior we want.

#### Validating the Number of Threads in new

We're still getting warnings because we aren’t doing anything with the
parameters to `new` and `execute`. Let’s implement the bodies of these
functions with the behavior we want. To start, let’s think about `new`.

Earlier we chose an unsigned type for the `size` parameter, since a pool with a
negative number of threads makes no sense. However, a pool with zero threads
also makes no sense, yet zero is a perfectly valid `u32`. Let’s add code to
check that `size` is greater than zero before we return a `ThreadPool`
instance, and have the program panic if a zero is received by using the
`assert!` macro as shown in Listing 20-13:

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

    // --snip--
}
```

Listing 20-13: Implementing `ThreadPool::new` to panic if `size` is zero

We’ve taken this opportunity to add some documentation for our `ThreadPool`
with doc comments. Note that we followed good documentation practices by adding
a section that calls out the situations in which our function can panic as we
discussed in Chapter 14. Try running `cargo doc --open` and clicking on the
`ThreadPool` struct to see what the generate docs for `new` look like!

Instead of adding the `assert!` macro as we’ve done here, we could make `new`
return a `Result` like we did with `Config::new` in the I/O project in Listing
12-9, but we’ve decided in this case that trying to create a thread pool
without any threads should be an unrecoverable error. If you’re feeling
ambitious, try to write a version of `new` with this signature to see how you
feel about both versions:

```
fn new(size: u32) -> Result<ThreadPool, PoolCreationError> {
```

#### Storing Threads in the Pool

Now that we have a way to know we have a valid number of threads to store in
the pool, we can actually create those threads and store them in the
`ThreadPool` struct before returning it.

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
    // --snip--
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

    // --snip--
}
```

Listing 20-14: Creating a vector for `ThreadPool` to hold the threads

We’ve brought `std::thread` into scope in the library crate, since we’re using
`thread::JoinHandle` as the type of the items in the vector in `ThreadPool`.

Once a valid size is recevied, our `ThreadPool` creates a new vector that can
hold `size` items. We haven’t used the `with_capacity` function in this book
yet, which does the same thing as `Vec::new`, but with an important difference:
it pre-allocates space in the vector. Since we know that we need to store
`size` elements in the vector, doing this allocation up-front is slightly more
efficient than using `Vec::new`, which resizes itself as elements get inserted.

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
options for fixing this: we can change our function’s signature, or we can cast
the `u32` as a `usize`. When we defined `new` we didn’t think too hard about
what number type made sense, we just chose one to go on with. Let’s give it
some more thought now. Given that `size` is the length of a vector, `usize`
makes a lot of sense. Let’s change the signature of `new`, which will get the
code in Listing 20-14 to compile:

<!-- Can you specify quickly why this means usize make sense? -->

```
fn new(size: usize) -> ThreadPool {
```

If you run `cargo check` again, you’ll get a few more warnings, but it should
succeed.

<!-- I wasn't sure what this next paagraph was relevant to, can you connect it
up more clearly?-->

We left a comment in the `for` loop in Listing 20-14 regarding the creation of
threads. How do we actually create threads? This is a tough question. What
should go in these threads? We don’t know what work they need to do at this
point, since the `execute` method takes the closure and gives it to the pool.

#### Refactoring the XXXX Code

<!-- Can you say how doing this refactoring will improve the code -- why don't
we want the pool to store threads directly? (I got that from the listing
caption because I wasn't sure what the end game was) -->

Let’s refactor slightly: instead of storing a vector of `JoinHandle<()>`
instances, we'll create a new struct to hold a single `JoinHandle<()> instance
so that the `ThreadPool` isn't storing threads directly. This new struct
represents the concept of a *worker*, which will receive a closure in the
`execute` method, and will take care of actually calling the closure.

<!-- Below -- what will let us store the number of worker instances, do we just
mean this method of doing things? -->

The XXX will let us store a fixed `size` number of `Worker` instances that
don’t yet know about the closures they’re going to be executing. We can also
give each worker an `id` so we can tell the different workers in the pool apart
when logging or debugging.

<!-- I was unclear on what a worker actually is here -- is this a
programming/Rust term, or just what we're calling the struct? Can you make it
clearer what the worker is and its responsibilities? -->

In this section we'll make these changes:

1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`
2. Change `ThreadPool` to hold a vector of `Worker` instances
3. Define a `Worker::new` function that takes an `id` number and returns a
   `Worker` instance that holds the allocated `id` and a thread spawned with an
   empty closure
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
    // --snip--
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
    // --snip--
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
holding threads directly

We’ve changed the name of the field on `ThreadPool` from `threads` to `workers`
since it's now holding `Worker` instances instead of `JoinHandle<()>`
instances. We use the counter in the `for` loop as an argument to
`Worker::new`, and we store each new `Worker` in the vector named `workers`.

External code (like our server in *src/bin/main.rs*) doesn’t need to know the
implementation details regarding using a `Worker` struct within `ThreadPool`,
so we make the `Worker` struct and its `new` private. The `Worker::new`
function uses the `id` we give it and stores a `JoinHandle<()>` instance that's
created by spawning a new thread using an empty closure.

This code will compile and and will store the number of `Worker` instances we
specified as an argument to `ThreadPool::new`, but we’re *still* not processing
the closure that we get in `execute`. Let’s talk about how to do that next.

### Sending Requests to Threads Via Channels

The next problem to tackle is that our closures do absolutely nothing.
Currently, we get the closure we want to execute in the `execute` method, but
it seems like we'd need to know the actual closures to use at the time we
create the `ThreadPool`.

Let’s think about what we really want to do: we want the `Worker` structs that
we just created to fetch jobs from the queue held in the `ThreadPool`, and run
those jobs in a thread.

In Chapter 16, we learned about *channels*---a simple way to communicate
between two threads---that would be perfect for this use-case. We'll use a
channel to function as the queue of jobs, and `execute` will send a job from
the `ThreadPool` to the `Worker` instances. Here’s the plan:

1. `ThreadPool` will create a channel and hold on to the sending side of the
   channel.
2. Each `Worker` will hold on to the receiving side of the channel.
3. We'll create a new `Job` struct that will hold the closures we want to send
   down the channel.
4. The `execute` method will send the job it wants to execute down the sending
   side of the channel.
5. In a thread, the `Worker` will loop over its receiving side of the channel
   and execute the closures of any jobs it receives.

Let’s start by creating a channel in `ThreadPool::new` and holding the sending
side in the `ThreadPool` instance, as shown in Listing 20-16. `Job` is a struct
that doesn't hold anything for now, and this will be the type of item we’re
sending down the channel:

Filename: src/lib.rs

```
// --snip--
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    // --snip--
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
    // --snip--
}
```

Listing 20-16: Modifying `ThreadPool` to store the sending end of a channel
that sends `Job` instances

In `ThreadPool::new`, we create our new channel, and set the pool to hold the
sending end. This will successfully compile, still with warnings.

Let’s try passing a receiving end of the channel into each worker as the thread
pool creates them. We know we want to use the receiving end in the thread that
the workers spawn, so we’re going to reference the `receiver` parameter in the
closure. The code shown here in Listing 20-17 won’t quite compile yet:

Filename: src/lib.rs

```
impl ThreadPool {
    // --snip--
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
    // --snip--
}

// --snip--

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

These are small and straightforward changes: we pass the receiving end of the
channel into `Worker::new`, and then we use it inside of the closure.

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

The code is trying to pass `receiver` to multiple `Worker` instances. This
won't work, as we recall from Chapter 16: the channel implementation provided
by Rust is multiple *producer*, single *consumer*. This means we can’t just
clone the consuming end of the channel to fix this. Even if we could, that's
not the method we'd want to choose; we want to distribute the jobs across
threads by sharing the single `receiver` between all of the workers.

<!-- Above - you may be able to tell I struggled to follow this explanation,
can you double check my edits and correct here? -->

Additionally, taking a job off the channel queue involves mutating the
`receiver`, so the threads need a safe way to share and modify `receiver`,
otherwise we might get race conditions (as covered in Chapter 16).

Remembering the threadsafe smart pointers that we discussed in Chapter 16, in
order to share ownership across multiple threads and allow the threads to
mutate the value, we need to use `Arc<Mutex<T>>`. `Arc` will let multiple
workers own the receiver, and `Mutex` will make sure that only one worker is
getting a job from the receiver at a time. Listing 20-18 shows the changes we
need to make:

Filename: src/lib.rs

```
use std::sync::Arc;
use std::sync::Mutex;

// --snip--

impl ThreadPool {
    // --snip--
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

    // --snip--
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--
    }
}
```

Listing 20-18: Sharing the receiving end of the channel between the workers
using `Arc` and `Mutex`

In `ThreadPool::new`, we put the receiving end of the channel in an `Arc` and a
`Mutex`. For each new worker, we clone the `Arc` to bump the reference count so
the workers can share ownership of the receiving end.

With these changes, the code compiles! We’re getting there!

#### Implementing the execute Method

Let’s finally implement the `execute` method on `ThreadPool`. We’re also going
to change `Job` from a struct to a type alias for a trait object that holds the
type of closure that `execute` receives. As we discussed, type aliases allow us
to make long types shorter. Take a look at Listing 20-19:

Filename: src/lib.rs

```
// --snip--

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// --snip--
```

Listing 20-19: Creating a `Job` type alias for a `Box` that holds each closure,
then sending the job down the channel

After creating a new `Job` instance using the closure we get in `execute`, we
send that job down the sending end of the channel. We’re calling `unwrap` on
`send` for the case that sending fails, which might happen if, for example, we
stop all of our threads from executing, meaning the receiving end has stopped
receiving new messages. At the moment, though, we can`t stop our threads
executing; our threads continue executing as long as the pool exists. The
reason we use `unwrap`, then, is that we we know the failure case won’t happen
but the compiler can’t tell that.

But we`re not quite done yet! In the worker, our closure being passed to
`thread::spawn` still only *references* the receiving end of the channel.
Instead, we need the closure to loop forever, asking the receiving end of the
channel for a job, and running the job when it gets one. Let’s make the change
shown in Listing 20-20 to `Worker::new`:

Filename: src/lib.rs

```
// --snip--

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
a *poisoned* state, which can happen if some other thread panicked while
holding the lock, rather than releasing the lock. In this situtation, calling
`unwrap` to have this thread panic is the correct action to take. Feel free to
change this `unwrap` to an `expect` with an error message that is meaningful to
you if you’d like.

If we get the lock on the mutex, then we call `recv` to receive a `Job` from
the channel. A final `unwrap` moves past any errors here as well, which might
occur if the thread holding the sending side of the channel has shut down,
similar to how the `send` method returns `Err` if the receiving side shuts down.

The call to `recv` *blocks*, so if there’s no job yet, the current thread will
sit until a job becomes available. The `Mutex<T>` makes sure that only one
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
our `Job` type alias is), the closure needs to be able to move itself *out* of
the `Box<T>` because the closure takes ownership of `self` when we call it. In
general, Rust doesn't allow use to move value out of a `Box<T>` since Rust
doesn’t know how big the value inside the `Box<T>` is going to be; recall in
Chapter 15 that we used `Box<T>` precisely because we had something of an
unknown size that we wanted to store in a `Box<T>` to get a value of a known
size.

We saw in Chapter 17, Listing 17-15 that we can write methods that use the
syntax `self: Box<Self>`, which allows the the method to take ownership of a
`Self` value stored in a `Box<T>`. That’s exactly what we want to do here, but
unfortunately Rust won't let us: the part of Rust that implements behavior when
a closure is called isn’t implemented using `self: Box<Self>`. So Rust doesn’t
yet understand that it could use `self: Box<Self>` in this situation in order
to take ownership of the closure and move the closure out of the `Box<T>`.

Rust is still a work in progress with places that the compiler could be
improved, but in the future, the code in Listing 20-20 should work just fine.
There are people just like you working to fix this and other issues! Once
you’ve finished the book, we would love for you to join in.

But for now, let’s work around this problem with a handy trick. We can tell
Rust explicitly that in this case we can take ownership of the value inside the
`Box<T>` using `self: Box<Self>`, and once we have ownership of the closure, we
can call it. This involves defining a new trait `FnBox` with the method
`call_box` that will use `self: Box<Self>` in its signature, defining `FnBox`
for any type that implements `FnOnce()`, changing our type alias to use the new
trait, and changing `Worker` to use the `call_box` method. These changes are
shown in Listing 20-21:

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

type Job = Box<FnBox + Send + 'static>; <!-- ' -->

// --snip--

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

First, we create a new trait named `FnBox`. This trait has the one method
`call_box`, which is similar to the `call` methods on the other `Fn*` traits
except that it takes `self: Box<Self>` in order to take ownership of `self` and
move the value out of the `Box<T>`.

Next, we implement the `FnBox` trait for any type `F` that implements the
`FnOnce()` trait. Effectively, this means that any `FnOnce()` closures can use
our `call_box` method. The implementation of `call_box` uses `(*self)()` to
move the closure out of the `Box<T>` and call the closure.

We now need our `Job` type alias to be a `Box` of anything that implements our
new trait `FnBox`. This will allow us to use `call_box` in `Worker` when we get
a `Job` value. Implementing the `FnBox` trait for any `FnOnce()` closure means
we don’t have to change anything about the actual values we’re sending down the
channel.

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

Success! We now have a thread pool executing connections asynchronously. There
are never more than four threads created, so our system won’t get overloaded if
the server receives a lot of requests. If we make a request to `/sleep`, the
server will be able to serve other requests by having another thread run them.

You'll notice we still get warnings, however, telling us the `workers`, `id`,
and `thread` fields are unused. In reality, right now we’re using all three of
these fields to hold onto some data, but nothing is actually done with the data
once the thread pool is set up and running the code that sends jobs down the
channel to the threads. We need to hold onto these values, though, or they go
out of scope: for example, if we didn’t return the `Vec<Worker>` value as part
of the `ThreadPool`, the vector would get cleaned up at the end of
`ThreadPool::new`.

So are these warnings wrong? In one sense yes, the warnings are wrong, since we
are using the fields to store data we need to keep around. In another sense,
the warnings aren’t wrong, because they’re telling us that we’ve forgotten to
do something: we never do anything to clean up our thread pool once it’s done
being used. Let’s implement a graceful shutdown that cleans up everything we’ve
created instead.

## Graceful Shutdown and Cleanup

The code in Listing 20-21 is responding to requests asynchronously through the
use of a thread pool, as we intended. We get some warnings about fields we’re
not using in a direct way that reminds us we’re not cleaning anything up. When
we use the less elegant <span class="keystroke">ctrl-C</span> method to halt
the main thread, all other threads are stopped immediately as well, even if
they’re in the middle of serving a request.

We’re now going to implement the `Drop` trait to call `join` on each of the
threads in the pool so they can finish the requests they’re working on before
closing. Then we’ll implement a way to tell the threads they should stop
accepting new requests and shut down. To see this code in action, we’ll modify
our server to only accept two requests before gracefully shutting down its
thread pool.

### heading?

Let’s start with implementing `Drop` for our thread pool. When the pool is
dropped, our threads should all join on to make sure they finish their work.
Listing 20-22 shows a first attempt at a `Drop` implementation; this code won’t
quite work yet:

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

First we loop through each of the thread pool `workers`. We use `&mut` for this
because `self` is itself a mutable reference and we also need to be able to
mutate `worker`. For each worker, we print a message saying that this
particular worker is shutting down, and then we call `join` on that worker’s
thread. If the call to `join` fails, we use `unwrap` to make Rust panic and go
into an ungraceful shutdown.

Here’s the error we get if we compile this code:

```
error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:65:13
   |
65 |             worker.thread.join().unwrap();
   |             ^^^^^^ cannot move out of borrowed content
```

This tells use we can't call `join` because we only have a mutable borrow of
each `worker`, and `join` takes ownership of its argument. In order to solve
this, we need a way to move the thread out of the `Worker` instance that owns
`thread` so that `join` can consume the thread. We saw a way to do this in
Listing 17-15: if `Worker` holds an `Option<thread::JoinHandle<()>` instead, we
can call the `take` method on the `Option` to move the value out of the `Some`
variant and leave a `None` variant in its place. In other words, a `Worker`
that is running will have a `Some` variant in `thread`, and when we want to
clean up a worker, we’ll replace `Some` with `None` so the worker doesn’t have
a thread to run.

So we know we want to update the definition of `Worker` like this:

Filename: src/lib.rs

```
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

Now let’s lean on the compiler to find the other places that need to change.
Checking this code, we get two errors:

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

Let's address the second error, which points to the code at the end of
`Worker::new`; we need to wrap the `thread` value in `Some` when we create a
new `Worker`. Make the following changes to fix this:

Filename: src/lib.rs

```
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

The first error is in our `Drop` implementation. We mentioned earlier that we
inended to call `take` on the `Option` value to move `thread` out of `worker`.
The following changes will do so:

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
`Some` and get the thread, then we call `join` on the thread. If a worker’s
thread is already `None`, we know that worker has already had its thread
cleaned up, so nothing happens in that case.

### HEADING

With this, our code compiles without any warnings. Bad news though, this code
doesn’t function the way we want it to yet. The key is the logic in the
closures run by the threads of the `Worker` instances: at the moment we call
`join`, but that won’t shut down the threads since they `loop` forever looking
for jobs. If we try to drop our `ThreadPool` with this implementation, the main
thread will block forever waiting for the first thread to finish.

To fix this, we’re going to modify the threads so they listen for either a
`Job` to run or a signal that they should stop listening and exit the infinite
loop. Instead of `Job` instances, then, our channel will send one of these two
enum variants:

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

// --snip--

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // --snip--
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// --snip--

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

To incorporate this we need to change `Job` to `Message` in a few different
places: the definition of `ThreadPool`, `ThreadPool::new` where we create the
channel, and the signature of `Worker::new`. The `execute` method of
`ThreadPool` needs to send jobs wrapped in the `Message::NewJob` variant. Then,
in `Worker::new` where a `Message` is received from the channel, the job will
be processed if the `NewJob` variant is received, and break out of the loop the
`Terminate` variant is received.

With these changes, the code will compile and continue to function in the same
way as it has been. We will get a warning, though, because we aren’t using the
`Terminate` variant in any messages. Let’s fix this by changing our `Drop`
implementation to look like Listing 20-24:

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
tried to send a message and `join` immediately in the same loop, we couldn't
guarantee that the worker in the current iteration would be the one to get the
message from the channel.

To better understand why we need two separate loops, imagine a scenario with
two workers. If we used a single loop to iterate through each worker, on the
first iteration terminate message would be sent down the channel and `join`
called on the first worker’s thread. If that first worker was busy processing a
request at that moment, the second worker would pick up the terminate message
from the channel and shut down. We’d be left waiting on the first worker to
shut down, but it never will since the second thread picked up the terminate
message. Deadlock!

To prevent this, we first put all of our `Terminate` messages on the channel in
one loop, and then we join on all the threads in another loop. Each worker will
stop receiving requests on the channel once it gets a terminate message,
meaning we can be sure that if we send the same number of terminate messages as
there are workers, each worker will receive a terminate message before `join`
is called on its thread.

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

Obviously you wouldn't want a real-world web server to shut down after serving
only two requests, this just demonstrates the graceful shutdown and cleanup in
working order.

We’ve added a `counter` variable that will increment every time an incoming TCP
stream is received. If that counter reaches 2, we’ll stop serving requests and
instead break out of the `for` loop. The `ThreadPool` will go out of scope at
the end of `main`, and we’ll see the `drop` implementation run.

Start the server with `cargo run`, and make three requests. The third request
should error, and in your terminal you should see output that looks like this:

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
third request we stop accepting connections. When the `ThreadPool` goes out of
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

Congrats! We have now completed our project, and we have a basic web server
that uses a thread pool to respond asynchronously. We’re able to perform a
graceful shutdown of the server, which cleans up all the threads in the pool.
Here’s the full code for reference:

<!-- As an option, we could refer to the full code file that the readers will
be able to download from the book's page to save printing it all out again
here, since this is already a really long chapter -- what do you think? -->

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
- Use `ThreadPool` to perform some task other than serving web requests
- Find a thread pool crate on crates.io and implement a similar web server
  using the crate instead and compare its API and robustness to the thread pool
  we implemented

## Summary

Well done! You’ve made it to the end of the book! We’d like to thank you for
joining us on this tour of Rust. You’re now ready to go out and implement your
own Rust projects and help with other people’s. Remember there’s a community of
other Rustaceans who would love to help you with any challenges you encounter
on your Rust journey.
