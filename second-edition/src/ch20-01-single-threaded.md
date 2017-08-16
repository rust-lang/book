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

```text
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

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
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

<span class="caption">Listing 20-2: Reading from the `TcpStream` and printing
out the data</span>

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

```text
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

```text
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

```text
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

```text
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

```text
HTTP/1.1 200 OK\r\n\r\n
```

This text is a tiny successful HTTP response. Let’s write this to the stream!
Remove the `println!` that was printing the request data, and add the code in
Listing 20-3 in its place:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<span class="caption">Listing 20-3: Writing a tiny successful HTTP response to
the stream</span>

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

<span class="filename">Filename: hello.html</span>

```html
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

<span class="caption">Listing 20-4: A sample HTML file to return in a
response</span>

This is a minimal HTML 5 document with a heading and a little paragraph. Let’s
modify `handle_connection` as shown in Listing 20-5 to read the HTML file, add
it to the response as a body, and send it:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
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

<span class="caption">Listing 20-5: Sending the contents of *hello.html* as the
body of the response</span>

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

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
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

<span class="caption">Listing 20-6: Matching the request against the content we
expect for a request to `/` and setting up conditionally handling requests to
`/` differently than other requests</span>

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

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
# fn handle_connection(mut stream: TcpStream) {
# if true {
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
# }
```

<span class="caption">Listing 20-7: Responding with status code `404` and an
error page if anything other than `/` was requested</span>

Here, our response has a status line with status code `404` and the reason phrase
`NOT FOUND`. We still aren’t returning any headers, and the body of the
response will be the HTML in the file *404.html*. Also create a *404.html* file
next to *hello.html* for the error page; again feel free to use any HTML you’d
like or use the example HTML in Listing 20-8:

<span class="filename">Filename: 404.html</span>

```html
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

<span class="caption">Listing 20-8: Sample content for the page to send back
with any `404` response</span>

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

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// ...snip...

fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 512];
#     stream.read(&mut buffer).unwrap();
#
#     let get = b"GET / HTTP/1.1\r\n";
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

<span class="caption">Listing 20-9: Refactoring so that the `if` and `else`
blocks only contain the code that differs between the two cases</span>

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
