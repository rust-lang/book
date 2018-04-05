## Building a Single Threaded Web Server

First we’ll get a single threaded web server working, but before we begin,
let’s look at a quick overview of the protocols involved in building web
servers. The details of these protocols are beyond the scope of this book, but
a short overview will give you the information you need.

The two main protocols involved in web servers are the *Hypertext Transfer
Protocol* (*HTTP*) and the *Transmission Control Protocol* (*TCP*). Both
protocols are *request-response* protocols, meaning a *client* initiates
requests, and a *server* listens to the requests and provides a response to the
client. The contents of those requests and responses are defined by the
protocols themselves.

TCP is the lower-level protocol that describes the details of how information
gets from one server to another, but doesn’t specify what that information is.
HTTP builds on top of TCP by defining the content of the requests and
responses. It’s technically possible to use HTTP with other protocols, but in
the vast majority of cases, HTTP sends its data over TCP. We’re going to work
with the raw bytes of TCP and HTTP requests and responses.

### Listening to the TCP Connection

Our web server needs to be able to listen to a TCP connection, so that’s the
first part we’ll work on. The standard library offers a `std::net` module that
lets us do this. Let’s make a new project in the usual fashion:

```text
$ cargo new hello --bin
     Created binary (application) `hello` project
$ cd hello
```

Now enter the code in Listing 20-1 in `src/main.rs` to start. This code will
listen at the address `127.0.0.1:7878` for incoming TCP streams. When it gets
an incoming stream, it will print `Connection established!`:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
```

<span class="caption">Listing 20-1: Listening for incoming streams and printing
a message when we receive a stream</span>

The `TcpListener` allows us to listen for TCP connections. We’ve chosen to
listen to the address `127.0.0.1:7878`. Breaking this address down, the section
before the colon is an IP address representing your own computer (this is the
same on each computer, and doesn’t represent the authors’ computer
specifically), and `7878` is the port. We’ve chosen this port for two reasons:
HTTP is normally accepted on this port and 7878 is “rust” typed on a telephone.
Note that connecting to port 80 requires administrator privileges;
non-administrators can only listen on ports higher than 1024.

The `bind` function in this scenario works like the `new` function, in that it
will return a new `TcpListener` instance. This function is called `bind`
because, in networking, connecting to a port to listen to is known as “binding
to a port”.

The `bind` function returns a `Result<T, E>`, which indicates that binding
might fail. For example, if we tried to connect to port 80 without being an
administrator, or if we ran two instances of our program and so had two
programs listening to the same port, binding wouldn’t work. Because we’re
writing a basic server for learning purposes here, we’re not going to worry
about handling these kinds of errors, so we use `unwrap` to stop the program if
errors happen.

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
<!--
There is no functionality for a stream without errors yet; I've clarified.
The errors happen when a client attempts to connect to the server; I've
clarified.
/Carol -->

For now, our handling of the stream consists of calling `unwrap` to terminate
our program if the stream has any errors, and if there aren’t any errors, then
print a message. We’ll add more functionality for the success case in the next
Listing. Receiving errors from the `incoming` method when a client connects to
the server is possible because we’re not actually iterating over connections,
we’re iterating over *connection attempts*. The connection might not be
successful for a number of reasons, many of them operating-system specific. For
example, many operating systems have a limit to the number of simultaneous open
connections they can support; new connection attempts beyond that number will
produce an error until some of the open connections are closed.

Let’s try this code out! First invoke `cargo run` in the terminal, then load up
`127.0.0.1:7878` in a web browser. The browser should show an error message
like “Connection reset”, because the server isn’t currently sending any data
back. If you look at your terminal, though, you should see a bunch of messages
that were printed when the browser connected to the server!

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

Sometimes, you’ll see multiple messages printed out for one browser request;
that might be because the browser is making a request for the page as well as a
request for other resources, like the `favicon.ico` icon that appears in the
browser tab.

It could also be that the browser is trying to connect to the server multiple
times because the server isn’t responding with any data. When `stream` goes out
of scope and is dropped at the end of the loop, the connection is closed as
part of the `drop` implementation. Browsers sometimes deal with closed
connections by retrying, because the problem might be temporary. The important
thing is that we’ve successfully gotten a handle to a TCP connection!

Remember to stop the program with <span class="keystroke">ctrl-C</span> when
you’re done running a particular version of the code, and restart `cargo run`
after you’ve made each set of code changes to make sure you’re running the
newest code.

### Reading the Request

Let’s implement the functionality to read in the request from the browser! To
separate out the concerns of getting a connection and then taking some action
with the connection, we’ll start a new function for processing connections. In
this new `handle_connection` function, we’ll read data from the TCP stream and
print it out so we can see the data being sent from the browser. Change the
code to look like Listing 20-2:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

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

We bring `std::io::prelude` into scope to get access to certain traits that let
us read from and write to the stream. In the `for` loop in the `main` function,
instead of printing a message that says we made a connection, we now call the
new `handle_connection` function and pass the `stream` to it.

In the `handle_connection` function, we’ve made the `stream` parameter mutable.
This is because the `TcpStream` instance keeps track of what data it returns to
us internally. It might read more data than we asked for and save that data for
the next time we ask for data. It therefore needs to be `mut` because its
internal state might change; usually we think of “reading” as not needing
mutation, but in this case we need the `mut` keyword.

<!-- Above -- I'm not clear what state will change here, the content of stream
when the program tempers what data it takes? -->
<!-- Yes, which is what we mean by "internally". I've tried to reword a bit,
not sure if it's clearer. /Carol -->

Next, we need to actually read from the stream. We do this in two steps: first,
we declare a `buffer` on the stack to hold the data that’s read in. We’ve made
the buffer 512 bytes in size, which is big enough to hold the data of a basic
request and sufficient for our purposes in this chapter. If we wanted to handle
requests of an arbitrary size, the management of the buffer would need to be
more complicated, but we’re keeping it simple for now. We pass the buffer to
`stream.read`, which will read bytes from the `TcpStream` and put them in the
buffer.

We then convert the bytes in the buffer to a string and print out that string.
The `String::from_utf8_lossy` function takes a `&[u8]` and produces a `String`
from it. The ‘lossy’ part of the name indicates the behavior of this function
when it sees an invalid UTF-8 sequence: it will replace the invalid sequence
with �, the `U+FFFD REPLACEMENT CHARACTER`. You might see replacement
characters for characters in the buffer that aren’t filled by request data.

Let’s give this a try! Start up the program and make a request in a web browser
again. Note that we’ll still get an error page in the browser, but the output
of our program in the terminal will now look similar to this:

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
     Running `target/debug/hello`
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
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

#### A Closer Look at an HTTP Request

HTTP is a text-based protocol, and a request takes this format:

```text
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
<!-- Yes /Carol -->

The next part of the `Request` line is `/` which tells us the *URI* (Uniform
Resource Identifier) that the client is requesting---a URI is almost, but not
quite, the same as a URL (*Uniform Resource Locator*). The difference between
URIs and URLs isn’t important for our purposes of this chapter, but the HTTP
spec uses the term URI, so we can just mentally substitute URL for URI here.

Finally, we’re given the HTTP version used by the client, and then the request
line ends in a CRLF sequence. The CRLF sequence can also be written as `\r\n`:
`\r` is a *carriage return* and `\n` is a *line feed*. (These terms come from
the typewriter days!) The CRLF sequence separates the request line from the
rest of the request data. Note that when CRLF is printed out, we see a new line
started rather than `\r\n`.

<!-- Above, I don't see a CRLF here in the request line in the actual output,
is it just implied because the next line begins on the next line? -->
<!-- Yes, I've clarified. /Carol -->

Taking a look at the request line data we received rom running our program so
far, we see that `GET` is the method, `/` is the Request URI, and `HTTP/1.1` is
the version.

The remaining lines starting from `Host:` onward are headers; `GET` requests
have no body.

Try making a request from a different browser, or asking for a different
address like `127.0.0.1:7878/test` to see how the request data changes, if
you’d like.

Now that we know what the browser is asking for, let’s send some data back!

### Writing a Response

We’re going to implement the sending of data in response to a client request.
Responses have the following format:

```text
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

```text
HTTP/1.1 200 OK\r\n\r\n
```

The status code 200 is the standard success response. The text is a tiny
successful HTTP response. Let’s write this to the stream as our response to a
successful request!

From the `handle_connection` function, we need to remove the `println!` that
was printing the request data, and replace it with the code in Listing 20-3:

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

<!-- Flagging for addition of wingdings later -->

The first new line defines the `response` variable that holds the data of the
success message. Then we call `as_bytes` on our `response` to convert the
string data to bytes. The `write` method on `stream` takes a `&[u8]` and sends
those bytes directly down the connection.

<!-- Above--So what does adding as_bytes actually do, *allow* us to send bytes
directly? -->
<!-- It converts the string data to bytes, I've clarified /Carol -->

Because the `write` operation could fail, we use `unwrap` on any error result
as before. Again, in a real application you would add error-handling here.
Finally, `flush` will wait and prevent the program from continuing until all of
the bytes are written to the connection; `TcpStream` contains an internal
buffer to minimize calls into the underlying operating system.

<!-- Above -- Will flush wait until all bytes are written and then do
something? I'm not sure what task it's performing -->
<!-- `flush` just makes sure all the bytes we sent to `write` are actually
written to the stream before the function ends. Because writing to a stream
takes time, the `handle_connection` function could potentially finish and
`stream` could go out of scope before all the bytes given to `write` are sent,
unless we call `flush`. This is how streams work in many languages and is a
small detail I don't think is worth going into in depth. /Carol -->

With these changes, let’s run our code and make a request! We’re no longer
printing any data to the terminal, so we won’t see any output other than the
output from Cargo. Load `127.0.0.1:7878` in a web browser, though, and you
should get a blank page instead of an error. How exciting! You’ve just
hand-coded an HTTP request and response.

### Returning Real HTML

Let’s implement returning more than a blank page. Create a new file,
*hello.html*, in the root of your project directory---that is, not in the `src`
directory. You can put in any HTML you want; Listing 20-4 shows one possibility:

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

This is a minimal HTML 5 document with a heading and some text. To return this
from the server when a request is received, let’s modify `handle_connection` as
shown in Listing 20-5 to read the HTML file, add it to the response as a body,
and send it:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
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

<span class="caption">Listing 20-5: Sending the contents of *hello.html* as the
body of the response</span>

We’ve added a line at the top to bring the standard library’s `File` into
scope. The code for opening files and reading code should look familiar from
Chapter 12, when we read the contents of a file for our I/O project in Listing
12-4.

Next, we’re using `format!` to add the file’s contents as the body of the
success response.

Run this code with `cargo run`, load up `127.0.0.1:7878` in your browser, and
you should see your HTML rendered!

Currently we’re ignoring the request data in `buffer` and just sending back the
contents of the HTML file unconditionally. That means if you try requesting
`127.0.0.1:7878/something-else` in your browser you’ll still get back this same
HTML response. This makes for a pretty limited server and is not what most web
servers do. We’d like to customize our responses depending on the request, and
only send back the HTML file for a well-formed request to `/`.

### Validating the Request and Selectively Responding

Right now, our web server will return the HTML in the file no matter what the
client requested. Let’s add functionality to check that the browser is
requesting `/` before returning the HTML file, and return an error if the
browser requests anything else. For this we need to modify `handle_connection`
as shown in Listing 20-6. This new code checks the content of the request
received against what we know a request for `/` looks like and adds `if` and
`else` blocks to treat requests differently:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
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
    }
}
```

<span class="caption">Listing 20-6: Matching the request and handling requests
to `/` differently than other requests</span>

First, we hardcode the data corresponding to the `/` request into the `get`
variable. Because we’re reading raw bytes into the buffer, we transform `get`
into a byte string by adding the `b""` byte string syntax at the start of the
content data. Then, we check to see if `buffer` starts with the bytes in `get`.
If it does, it means we’ve received a well-formed request to `/`, which is the
success case we’ll handle in the `if` block that returns the contents of our
HTML file.

If `buffer` does *not* start with the bytes in `get`, it means we’ve received
some other request. We’ll add code to the `else` block in a moment to respond
to all other requests.

Run this code now and request `127.0.0.1:7878`, and you should get the HTML in
*hello.html*. If you make any other request, such as
`127.0.0.1:7878/something-else`, you’ll get a connection error like we saw when
running the code in Listing 20-1 and Listing 20-2.

Now let’s add the code in Listing 20-7 to the `else` block to return a response
with the status code `404`, which signals that the content for the request was
not found. We’ll also return some HTML for a page to render in the browser
indicating as such to the end user:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
# fn handle_connection(mut stream: TcpStream) {
# if true {
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
# }
```

<span class="caption">Listing 20-7: Responding with status code `404` and an
error page if anything other than `/` was requested</span>

Here, our response has a status line with status code `404` and the reason
phrase `NOT FOUND`. We’re still not returning headers, and the body of the
response will be the HTML in the file *404.html*. You’ll need to create a
*404.html* file next to *hello.html* for the error page; again feel free to use
any HTML you’d like or use the example HTML in Listing 20-8:

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

With these changes, try running your server again. Requesting `127.0.0.1:7878`
should return the contents of *hello.html*, and any other request, like
`127.0.0.1:7878/foo`, should return the error HTML from *404.html*!

### A Touch of Refactoring

At the moment our `if` and `else` blocks have a lot of repetition: they’re both
reading files and writing the contents of the files to the stream. The only
differences are the status line and the filename. Let’s make our code more
concise by pulling those differences out into an `if` and `else` of one line
each that will assign the values of the status line and the filename to
variables; we can then use those variables unconditionally in the code to read
the file and write the response. The resulting code after replacing the large
`if` and `else` blocks is shown in Listing 20-9:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// --snip--

fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 512];
#     stream.read(&mut buffer).unwrap();
#
#     let get = b"GET / HTTP/1.1\r\n";
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

<span class="caption">Listing 20-9: Refactoring so that the `if` and `else`
blocks only contain the code that differs between the two cases</span>

Now the `if` and `else` blocks only return the appropriate values for the
status line and filename in a tuple; we then use destructuring to assign these
two values to `status_line` and `filename` using a pattern in the `let`
statement like we discussed in Chapter 18.

The previously duplicated code is now outside the `if` and `else` blocks and
uses the `status_line` and `filename` variables. This makes it easier to see
exactly what’s different between the two cases, and means we have only one
place to update the code if we want to change how the file reading and response
writing works. The behavior of the code in Listing 20-9 will be exactly the
same as that in Listing 20-8.

Awesome! We have a simple little web server in about 40 lines of Rust code that
responds to one request with a page of content and responds to all other
requests with a `404` response.

Currently our server runs in a single thread, meaning it can only serve one
request at a time. Let’s see how that can be a problem by simulating some slow
requests, and then fix it so our server can handle multiple requests at once.
