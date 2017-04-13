# Un-named project

It's been a long journey, but here we are! It's the end of the book. Parting is
such sweet sorrow. But before we go, let's build one more project together, to
show off some of the things we learned in these final chapters, as well as
re-cap some of the earlier ones.

Here's what we're going to make: a web server that says hello:

![hello from rust](hello.png)

To do this, we will:

1. Learn a little bit about TCP and HTTP.
2. Listen for TCP connections on a socket
3. Parse a tiny bit of HTTP requests
4. Create a proper HTTP response
5. Improve the throughput of our server with a thread pool

Before we get started, however, there's one thing we should mention: if you were
writing this code in production, there are a lot of better ways to write it.
Specifically, there are a number of robust crates on crates.io that would make
writing this easier. However, for this chapter, our intention is to learn, not
to take the easy route. So we'll be writing a basic implementation ourselves.

# Accepting a TCP connection

The HTTP protocol is built on top of the TCP protocol. We won't get too much
into the details, but here's a small overview: TCP is a low-level protocol, and
HTTP builds a higher-level one on top of it. Both protocols are what's called a
"request-response protocol", that is, there is a *client* that initiates
requests, and a *server* that listens to requests and provides a response to the
client. The contents of those requests and responses are defined by the protocols
themselves. TCP describes the low-level details of "how does this information get
from one server to another", but doesn't specify what that information is; it's
just a bunch of ones and zeroes. HTTP builds on top of this by defining what those
contents should be. As such, it's technically possible to use HTTP with other
protocols, but in the vast, vast majority of cases, it's over TCP.

So the first thing we need to build our web server is to be able to listen to a
TCP connection. The standard library has a `std::net` module that lets us do
this. Let's make a new project:

```text
$ cargo new hello --bin
     Created binary (application) `hello` project
$ cd hello
```

We'll put this in `src/main.rs`:

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

Let's talk about each part in turn:

```rust,ignore
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

```

A `TcpListener` allows us to listen for TCP connections. We've chosen to listen
to the address `127.0.0.1:8008`. The part before the colon is an IP address
representing our own computer, and `8080` is the port. We've chosen this port
because HTTP is normally accepted on port 80, but connecting to port 80 requires
administrator privileges. Regular users can listen on ports higher than 1024;
8080 is easy to remember since it's port 80, but twice.

The `bind` method is sort of like `new`, but with a more descriptive name. In
networking, people will often talk about "binding to a port", and so the
function is called `bind`. Finally, it returns a `Result<T, E>`; binding may
fail. For example, if we had tried to connect to port 80 without being an
administrator. Another example is if we tried to have two programs listening
to the same port; for example, if we ran two instances of our program. Since
we're writing a basic server here, we're not going to worry about handling these
kinds of errors, and so `unwrap` lets us ignore them.

```rust,ignore
for stream in listener.incoming() {
```

The `incoming` method on `TcpListener` gives us an iterator that gives us a
sequence of streams, more specifically, `TcpStream`s. A 'stream' represents
represents an open connection between the client and the server. A 'connection'
is a name for the full request/response process, that is, a client connects to
the server, the server generates a response, and then closes the connection.
As such, the `TcpStream` will let us read from itself to see what the client
sent, and write our response to it. So this `for` loop will process each
connection in turn, and produce a series of streams. We can then handle each one
in turn.

```rust,ignore
let stream = stream.unwrap();

println!("Connection established!");
```

Right now, "handling" a stream means `unwrap`ping it to terminate our program if
it encounters any errors, and then printing a message. What kind of errors can
happen here? Well, we're not actually iterating over connections, we're
iterating over *connection attempts*. The connection might not work for a number
of reasons, many of them operating-system specific. For example, many operating
systems have a limit to the number of simultaneous open connections you're
allowed to have; new connections will then produce an error until some of them
are closed.

Let's try this code out! First invoke `cargo run`:

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
warning: unused variable: `stream`, #[warn(unused_variables)] on by default
 --> src\main.rs:8:13
  |
8 |         let stream = stream.unwrap();
  |             ^^^^^^

     Running `target\debug\hello.exe`
```

And then load up `127.0.0.1:8080` in your web browser. Your browser will
show an error message, something like "Connection reset", but if you look
at your terminal...

```text
     Running `target\debug\hello.exe`
Connection established!
Connection established!
Connection established!
```

A bunch of messages! Why did we get multiple ones? Well, our browser is
expecting to speak HTTP, but we aren't replying with anything, just closing the
connection. We're closing it by moving on to the next loop iteration; when
`stream` gets dropped at the end of the loop, it closes it for us. These
connections might be the browser making a request for the page and a request for
a `favicon.ico`, it might be retrying on its own... the important thing is that
we've successfully gotten a handle on a TCP connection!

In order to keep things clean, let's move our processing of the connection out
to a function. Modify your code to look like this:

```rust,no_run
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    println!("Connection established!");
}
```

Now we can worry about handling the `TcpStream` in `handle_connection` only, and
not worry about all of the connection processing stuff.

# Reading the request

Let's read in the request from our browser! Modify our code like this:

```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // no changes in here!
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

We've added one new `use` declaration, importing the `std::io` module's
`prelude`. This will bring important traits into scope that let us read from and
write to the stream.

In `handle_connection`, we had to make `stream` mutable with the `mut` keyword.
We're going to be reading data from the stream, so it's going to get modified.
Next, we need to actually read from the stream; we do this in two steps:

```rust,ignore
let mut buffer = [0; 512];

stream.read(&mut buffer).unwrap();
```

First, we declare a `buffer` on the stack; we've made it 512 bytes. Why 512?
It's big enough to get a basic request, but not super huge. If we wanted to
handle requests of an arbitrary size, this would need to be more
complicated, but we're keeping it simple for now! We then pass that buffer
to `stream.read`. This will read bytes from the `TcpStream` and put them in
the buffer.

Next, we print that stream out:

```rust,ignore
println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
```

The `String::from_utf8_lossy` function will take a `&[u8]` and produce a `String`. The
'lossy' part of its name comes from its behavior when it sees invalid UTF-8 sequences;
it replaces them with �, `U+FFFD REPLACEMENT CHARACTER`.

Let's give this a try!

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
     Running `target\debug\hello.exe`
Request: GET / HTTP/1.1
Host: 127.0.0.1:8080
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101 Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
```

You'll probably get slightly different output depending on your browser! You
also might see this request repeated; now we can tell that the reason we
have multiple connections is because the browser is trying to fetch `/`
repeatedly. Let's break this request down. HTTP is a text-based protocol, a
request looks like this:

```text
Request-Line headers CRLF message-body
```

First there's a 'request line'. Then, any headers. Next, a CRLF sequence, and then, the body
of the message. A request line looks like this:

```text
Request-Line = Method Request-URI HTTP-Version CRLF
```

First, we have a method, like `GET` or `POST`. Then, the request's URI, which is
a term the HTTP spec uses. You have probably heard of a 'URL'. All URLs are URIs,
but not all URIs are URLs. Since this isn't a book about the HTTP specification,
given this fact, we can just think "URL" when we see "URI" and move on. Next, we
have the HTTP version, and then a CRLF sequence. That's `\r\n` is the CRLF
sequence; `\r` is a "carriage return" and `\n` is a "line feed"; these terms
come from the typewriter days!

If we apply this to our request:

```text
GET / HTTP/1.1
Host: 127.0.0.1:8080
<more headers>
```

`GET` is our method, `/` is our Request URI, and `HTTP/1.1` is our version.
All the stuff from `Host` and after are headers. `GET` requests have no body.
Neat!

# Writing a response

 Let's respond to our browser with a response. Responses look like this:

```text
Status-Line headers CRLF message-body
```

First, we need a status line. Then, any headers. Next, a CRLF sequence, and then, the body
of the message. What's a status line? Here's an example of one:

```text
HTTP/1.1 200 OK\r\n\r\n
```

Status lines look like this:

```text
Status-Line = HTTP-Version Status-Code Reason-Phrase CRLF
```

We're using version 1.1 of the protocol, and `200` is the status code. `OK` is
the "reason phrase", it's like a text description of the status code. Finally,
`\r\n` is the CRLF sequence; `\r` is a "carriage return" and `\n` is a "line
feed"; these terms come from the typewriter days!

We don't have any headers, so there's nothing to put there. Next, another CRLF
to separate the headers from the body, which is empty. Whew! With this text,
we've got a successful, tiny, HTTP response. We have to write it to the stream
though! Let's modify our code to write out a response. Remove the `println!`
line, and add these below:

```rust,ignore
let response = "HTTP/1.1 200 OK\r\n\r\n";

stream.write(response.as_bytes()).unwrap();
stream.flush().unwrap();
```

The first line defines our response. Then, we call `as_bytes` on our `response`,
as the `write` method on `stream` takes a `&[u8]`, and writes those bytes
directly down the connection. This could fail, so `write` returns a `Result<T,
E>`; we continue to use `unwrap` to make progress here. Finally, `flush()` will
wait until all of the underlying bytes are written to the connection;
`TcpStream` contains an internal buffer to minimize calls into the underlying
operating system.

With these changes, let's run our code!

```text
> cargo run
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39 secs
     Running `target\debug\hello.exe`
```

Once we've loaded `127.0.0.1:8080` in our web browser... we get a blank page!
How exciting! You've just hand-coded an HTTP request and response. From here on
out, it's all just details.

## Returning real HTML

Let's return more than a blank page. Create a new file, `hello.html`, in the
root of the project; that is, not in the `src` directory. You can put any
HTML you want in it, here's what the authors used for theirs:

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

This is a minimal HTML 5 document, with a heading and a little paragraph. Let's
modify `handle_connection` to read that file, append it to our header, and send
it as the response:

```rust,ignore
// add this import at the top
use std::fs::File;

// our new handle_connection
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let mut file = File::open("hello.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let header = "HTTP/1.1 200 OK\r\n\r\n";
    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Here's opening and reading the file:

```rust,ignore
let mut file = File::open("hello.html").unwrap();
let mut contents = String::new();

file.read_to_string(&mut contents).unwrap();
```

We talked about this in the I/O project chapter, so this should
look fairly familiar. We open the file with `File::open`, and
the read it into a `String` with `file.read_to_string`.

Next, we write our response out:

```rust,ignore
let header = "HTTP/1.1 200 OK\r\n\r\n";
let response = format!("{}{}", header, contents);

stream.write(response.as_bytes()).unwrap();
stream.flush().unwrap();
```

We use `format!` to concatenate our header onto the body,
and then change `write` to write `response`. Easy! Run it with
`cargo run`, load up `127.0.0.1:8080` in your browser, and you
should see your HTML rendered!

# Validating the request

Right now, our web server will return this HTML no matter what the request.
Let's check that the browser is requesting `/`, and then return an error if
it's not. First, modify `handle_connection` to look like this:

```rust,ignore
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let start = &buffer[..get.len()];

    if start == get {
        // success!
    } else {
        // error :(
    };
```

Here, we defined the HTTP request we're looking for with `get`. Because we are
reading raw bytes into the buffer, we use a byte string, with `b"`, to make this
a byte string too. Then, we take a slice of the `buffer` that's the same length
as `get`, and compare them. If they're identical, we've gotten a good request.
If not, we've gotten a bad request.

Let's add in the code to handle each side:

```rust,ignore
if start == get {
    let header = "HTTP/1.1 200 OK\r\n\r\n";
    let mut file = File::open("hello.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
} else {
    let header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let mut file = File::open("404.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
};
```

The interesting bit is in the else case:

```rust,ignore
let header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
let mut file = File::open("404.html").unwrap();
```

`404 NOT FOUND` is the proper error code here. And we need to make a new file,
`404.html`, to go along with `hello.html`. Here's its contents:

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

With these changes, try running your server again. Requesting `127.0.0.1:8080` should
return our `hello.html`, and any other request, like `127.0.0.1:8080/foo`, should return
our error!

There's a lot of repetition in this function; let's pull it out:

```rust,ignore
   let (header, filename) = if start == get {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Here, the only thing in our `if` is the header and the filename; we then use
destructuring to assign these two bits to `filename` and `header`. We have to
change the call to `File::open` to use this new variable.

Awesome! We have a simple little web server in ~40 lines of Rust code. So far,
this project has been relatively straightforward as far as Rust code goes; we
haven't done much of the more advanced things yet. Let's kick it up a notch
and add a feature to our web server: a threadpool.

# Thread pools

Right now, we process each request in turn. That works for small services like
ours, but as applications get more complex, this sort of serial execution isn't
optimal. Let's make our web server better by adding a *thread pool*. How does a
thread pool make things better? Well, right now, we process connections
sequentially. We need to fully process each connection before moving on to the
next one. A thread pool allows us to process connections concurrently, that is,
we can start processing a new connection before an older connection is finished.
This increases the throughput of our server. This can matter even more in some
situations, as we'll see below.

Here's the basics: instead of waiting for each request to process before
starting on the next one, we create a new thread for every connection, and do
the processing inside of the thread. There's a problem with that, however: if we
get a thousand requests, then we create a thousand threads. Someone making ten
million requests to our server could create havoc by using up all of our
server's resources and grinding things to a halt. So instead, we create a 'pool'
of threads, with a size of our choosing. As requests come in, we send them to
the pool for processing. The pool maintains a queue of requests. Each of the
threads in the pool pops a request off of this queue, handles the request, and
then asks the queue for another request. With this design, we can process N
requests concurrently, where N is the number of threads. This still means that
`N` long-running tasks can cause problems, but we've increased that number from
one to `N`.

This design is one of many ways to improve the throughput of our web server.
This isn't a book about web servers, though, so it's the one we're going to
cover. Other options are the "fork/join" model, and the "single threaded async
I/O" model. If you're interested in this topic, you may want to read more about
them and try to implement them in Rust; with a low-level language like Rust, all
of these options are possible.

# The Pool interface

Let's talk about what using the pool should look like. The authors often find that
when trying to design some code, writing the client interface first can really
help guide your design. Write the code you'd want to use, then implement it, rather
than the other way around.

To do this, first, let's examine what the "create a new thread for every
connection" would look like. It's not our final plan due to the problems we
talked about earlier, but it's a start. Here's the changes to `main.rs`:

```rust,ignore
// add this import at the top:
use std::thread;

// and then this change to main:
for stream in listener.incoming() {
    let stream = stream.unwrap();

    thread::spawn(|| {
        handle_connection(stream);
    });
}
```

As we learned in Chapter 16, `thread::spawn` will create a new thread and then run
the code in the closure in it. We'd want our thread pool to work in a similar way.
Something like this:

```rust,ignore
// create a pool with four threads
let pool = ThreadPool::new(4);

for stream in listener.incoming() {
    let stream = stream.unwrap();

    // run this closure in the pool
    pool.execute(|| {
        handle_connection(stream);
    });
}
```

We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, and then `pool.execute` in a similar way to `thread::spawn`. Go ahead
and make those changes to `main.rs`, and then let's use the compiler errors to
drive our development. Here's the first error we get:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0433]: failed to resolve. Use of undeclared type or module `ThreadPool`
  --> src\main.rs:10:16
   |
10 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ Use of undeclared type or module `ThreadPool`

error: aborting due to previous error
```

Great, we need a `ThreadPool`. Let's define one:

```rust
struct ThreadPool;
```

And try again:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error: no associated item named `new` found for type `ThreadPool` in the current scope
  --> src\main.rs:10:16
   |
10 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope; the following traits define an item `new`, perhaps you need to implement one of them:
   = help: candidate #1: `std::sys_common::thread_info::NewThread`
   = help: candidate #2: `std::iter::ZipImpl`

error: aborting due to previous error
```

The helpful messages aren't super helpful here; we need to define our own
`new` function, not implement a trait. Here it is:

```rust,ignore
impl ThreadPool {
    fn new() {
    }
}
```

Let's check it again:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0061]: this function takes 0 parameters but 1 parameter was supplied
  --> src\main.rs:10:32
   |
10 |       let pool = ThreadPool::new(4);
   |                                  ^ expected 0 parameters
...
49 |       fn new() {
   |  _____- starting here...
50 | |
51 | |     }
   | |_____- ...ending here: defined here

error: no method named `execute` found for type `()` in the current scope
  --> src\main.rs:15:14
   |
15 |         pool.execute(|| {
   |              ^^^^^^^

error: aborting due to 2 previous errors
```

Two errors: we need a parameter for `new`, and a type error. Let's focus
on the first error for now:

```rust,ignore
impl ThreadPool {
    fn new(size: u32) {

    }
}
```

Why a `u32`? We know that a negative number of threads makes no sense, so an unsigned
value makes sense. `u32` is a solid default. Once we actually implement `new` for real,
we'll reconsider it, but for now, we're just working through compiler errors.

And check again:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error: no method named `execute` found for type `()` in the current scope
  --> src\main.rs:15:14
   |
15 |         pool.execute(|| {
   |              ^^^^^^^

error: aborting due to previous error
```

Okay, now we only have the second error. It's slightly obtuse: because
`new` doesn't return anything, `pool` has the type unit. And unit doesn't
have an `execute` method. What we actually intended was for `new` to return
a `ThreadPool`, so let's fix that, and then also add the `execute` method:

```rust,ignore
impl ThreadPool {
    fn new(size: u32) -> ThreadPool {
        ThreadPool
    }

    fn execute(&self) {

    }
}
```

Let's check again:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0061]: this function takes 0 parameters but 1 parameter was supplied
  --> src\main.rs:15:22
   |
15 |           pool.execute(|| {
   |  ______________________^ starting here...
16 | |             handle_connection(stream);
17 | |         });
   | |_________^ ...ending here: expected 0 parameters
...
53 |       fn execute(&self) {
   |  _____- starting here...
54 | |
55 | |     }
   | |_____- ...ending here: defined here

error: aborting due to previous error
```

We need `execute` to take a closure parameter. If you remember from Chapter 13, we can
take closures as arguments with three different traits: `Fn`, `FnMut`, and `FnOnce`.
What kind of closure should we use? Well, we know we're going to end up doing
something similar to `thread::spawn`; what bounds does it have?

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`F` is the parameter we care about here; not `T`. Given that `spawn` uses `FnOnce`, it's
probably what we want as well, given that we're eventually passing something to `spawn`.
In addition, we have a `Send` and `'static` bound, which also makes sense: we need `Send`
to transfer something from one thread to another, and `'static` because we don't know
how long the thread will execute. Let's modify `execute` to have these bounds:

```rust,ignore
fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
{

}
```

Let's check again:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
warning: unused import: `std::thread`, #[warn(unused_imports)] on by default
 --> src\main.rs:5:5
  |
5 | use std::thread;
  |     ^^^^^^^^^^^

warning: unused variable: `size`, #[warn(unused_variables)] on by default
  --> src\main.rs:49:12
   |
49 |     fn new(size: usize) -> ThreadPool {
   |            ^^^^

warning: unused variable: `f`, #[warn(unused_variables)] on by default
  --> src\main.rs:53:26
   |
53 |     fn execute<F>(&self, f: F)
   |                          ^
```

It compiles!

> This is a good time to remember that while "if it compiles, it works" is often
> true of Rust code, it's not universal. Our project compiles, but does absolutely
> nothing! If we were building something real, this would be a great time to start
> writing unit tests.

We do have some warnings; we're no longer using `std::thread`, and we aren't
doing anything with our arguments. Let's implement both of these methods on our
`ThreadPool`.

To start, let's think about `new`. The first thing that matters is something we said
above: a pool with a negative number of threads makes no sense. However, a pool with
zero threads also makes no sense, yet zero is a perfectly valid `u32`. Let's check
that our number is greater than zero:

```rust,ignore
/// Create a new ThreadPool.
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The `new` function will panic if the size is zero.
fn new(size: u32) -> ThreadPool {
    assert!(size > 0);

    ThreadPool
}
```

We've added some documentation for our `ThreadPool` with doc comments. Careful
observers will note we called out the situations in which our function can panic
as well; see Chapter 14 for more details on writing good documentation.

We've also added in an `assert!` to check the validity of `Size`. We could
also make `new` return a `Result` instead, but it involves a bunch
of more code, and arguably, passing in a zero is incoherent, and therefore
deserves to be an unrecoverable error rather than a recoverable one.
If you're feeling ambitious, try to write a version of `new` with this
signature:

```rust,ignore
fn new(size: u32) -> Result<ThreadPool, PoolCreationError> {
```

See how you feel about both versions.

Let's write some more code. Here's the sketch for the changes we need to
make:

* create `size` new threads
* store these new threads inside the `ThreadPool` and return it.

This raises a question: how do we "store" a thread? Let's turn again
to the signature of `spawn`:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`spawn` returns a `JoinHandle<T>`, where `T` is the type that's returned
from the closure. In our case, we're handling our connection and not returning
anything, so `T` will be `()`, unit, here.

This won't compile yet, but let's start here:

```rust,ignore
struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: u32) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in threads
        }

        ThreadPool {
            threads: threads,
        }
    }
```

We get an error:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0308]: mismatched types
  --> src\main.rs:70:46
   |
70 |         let mut threads = Vec::with_capacity(size);
   |                                              ^^^^ expected usize, found u32

error: aborting due to previous error
```

`size` is a `u32`, but `Vec::with_capacity` needs a `usize`. We haven't used
`with_capacity` in this book yet; it does the same thing as `Vec::new`, but
with an important difference: it pre-allocates space in the vector. Since
we know that we need to store `size` elements in the vector, doing this
allocation up-front is slightly more efficent than only writing `Vec::new`,
and it's not harder to write. Well, until we get an error like this!

We have two options here: we can change our function's signature, or we can
cast. If you remember when we defined `new`, we didn't think too hard about what
number made sense, we just chose one. Let's give it some more thought now. Given
that `size` is the length of a vector, `usize` makes a lot of sense. They even
almost share a name! Let's change the signature, and this will now compile:

```rust,ignore
fn new(size: usize) -> ThreadPool {
```

If you check this out with `cargo check`, you'll get a few more warnings, but
it should succeed. We left a little comment above regarding the creation of
threads. This is a tough question though... what should go in these threads we've
created? We don't know what work they need to do, because the `execute` method
takes the closure and gives it to the pool.

Let's refactor slightly: instead of storing a vector of `JoinHandle<()>`s, let's
create a new `struct` to represent each of these 'workers'. We can also then
give each worker an `id` so we can tell them apart when logging or debugging.

This won't work yet, but let's start here:

```rust,ignore
struct ThreadPool {
    threads: Vec<Worker>,
}

struct Worker {
    id: u32,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32) -> Worker {
        let thread = thread::spawn(||{ });

        Worker {
            id: id,
            thread: thread,
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32));
        }

        ThreadPool {
            threads: threads,
        }
    }

```

Here we've made a few changes:

1. `ThreadPool` now has a vector of `Worker`s.
2. `Worker`s have a `new` method that takes an `id` number
3. Currently the closure a `Worker` has does nothing; we'll fix this soon.
4. In `ThreadPool::new`, we use the loop counter to generate an `id`, and
   then create a new worker for each iteration. We use `as` to convert between
   `u32` and `usize` here.

This compiles, though we still get a number of warnings. Let's keep going!

The next problem to tackle is that our closures do absolutely nothing. This
raises the question: what should they do? We get the actual closure we want to
execute in the `execute` method, but we need to know it here.

Or do we? This closure is the behavior of the *worker*, not of the work it does.
And as we said above, our workers are going to attempt to fetch jobs off of a
queue that the `ThreadPool` holds. We have none of that infrastructure yet.

In Chapter 16, we learned about channels. Channels are a great way to
communicate between two threads, and they're perfect with our use-case. Here's
the plan of attack:

1. `ThreadPool` will hold on to a sending side of a channel.
2. Each `Worker` will hold on to a receiving side.
3. The `execute` method of `ThreadPool` will then send the closure it wants
   to execute down the sending side of the channel.
4. The `Worker` will loop over its receiving side, and when it gets a job,
   execute it.

Once we get all of this working, we should be in a good place!

Let's start by adding the sending side to `ThreadPool`:

```rust,ignore
// add this import at the top:
use std::sync::mpsc;

// and then modify this code below:
struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Job>();

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32));
        }

        ThreadPool {
            threads: threads,
            sender: job_sender,
        }
    }
```

We've introduced a new structure, `Job`, to represent each job we want to
execute. We have our `ThreadPool` hold onto an `mpsc::Sender`, which if you
recall is the type of a sending end of a channel. In `ThreadPool::new`, we
create our new channel, and then have the pool hang on to the sending end.

 If you compile this, it will successfully compile, but still have warnings. The
 it won't do the right thing yet, but it gets past the compiler. Let's try
 passing the receiving end into our workers. This won't compile yet:

```rust,ignore
impl Worker {
    fn new(id: u32, job_receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(||{
            // we want to use receiver in the closure, let's just
            // reference it for now
            job_receiver;
        });

        Worker {
            id: id,
            thread: thread,
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Job>();

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32, job_receiver));
        }
```

These are small and straightforward changes: we pass in the receiving end
of the channel into `Worker::new`, and then we use it inside of the closure.

If we try to compile this, we get this error:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0382]: use of moved value: `job_receiver`
  --> src\main.rs:82:48
   |
82 |             threads.push(Worker::new(i as u32, job_receiver));
   |                                                ^^^^^^^^^^^^ value moved here in previous iteration of loop
   |
   = note: move occurs because `job_receiver` has type `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait

error: aborting due to previous error
```

This won't quite work: we are trying to pass `job_receiver` to multiple
`Worker`s, but that won't work. We instead need to share the single receiver between
all of our workers. If you remember Chapter 16, you'll know the answer: `Arc<Mutex<T>>`
to the rescue! Here's the changes:


```rust,ignore
// add these imports to the top
use std::sync::Arc;
use std::sync::Mutex;

// and then change this code
impl Worker {
    fn new(id: u32, job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(||{
            // we want to use the receiver in the closure
            job_receiver;
        });

        Worker {
            id: id,
            thread: thread,
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Job>();

        let job_receiver = Arc::new(Mutex::new(job_receiver));

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32, job_receiver.clone()));
        }
```

We now accept an `Arc<Mutex<Receiver>>` in `Worker::new`, and we
create one in `ThreadPool::new`. Finally, when we call `Worker::new`,
we use the `clone` method of the `Arc<T>` to bump the reference count
for each new `Worker`.

With these changes, things compile! We're getting there!

Let's finally implement the `execute` method. It looks like this:

```rust,ignore
struct Job {
    job: Box<FnOnce() + Send + 'static>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        // no changes here
    }

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Job {
            job: Box::new(f),
        };

        self.sender.send(job).unwrap();
    }
}
```

Here, `Job` is now holding a trait object; specifically, a boxed closure. We
then send that `job` down the sending end of the channel. Sending may fail;
we use `unwrap` to ignore the error. It fails if the receiving end has stopped
receiving new messages; this would happen once we stop all of our threads from
executing, but that doesn't happen as long as the pool exists.

Now that we've got the sending side working, let's write the logic of the worker.
Here's a first attempt, but it won't quite work:

```rust,ignore
let thread = thread::spawn(move ||{
    loop {
        let job = job_receiver.lock().unwrap().recv().unwrap();

        println!("Worker {} got a job; executing.", id);

        job.job();
    }
});
```

Here, we first call `lock` on the `job_receiver` to acquire the mutex, then
`unwrap` to panic on any errors, then `recv` to receive a `Job` from the
channel. A final `unwrap` moves past those errors as well. What kinds of
errors are we ignorning here? Well, a mutex can be "poisoned", that is, if
a thread is holding the mutex and panics, it enters a "poisoned" state.
Almost all of the time, propogating this panic with `unwrap` is correct.
As for `recv`, it will return `Err` if the sending side has shut down,
similar to how the `send` method returns `Err` if the receiving side shuts
down.

The call to `recv` blocks; that is, if there's no job yet, it will sit here
until one becomes available. The `Mutex<T>` makes sure that only one Worker
at a time tries to request a job.

Here's the error we'll get if we try to compile the above code:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error: no method named `job` found for type `Job` in the current scope
  --> src\main.rs:69:21
   |
69 |                 job.job();
   |                     ^^^
   |
note: use `(job.job)(...)` if you meant to call the function stored in the `job` field
  --> src\main.rs:69:21
   |
69 |                 job.job();
   |                     ^^^

error: aborting due to previous error
```

Rust helpfully informs us that this is ambiguous: We're trying to invoke the closure
that `job.job` holds, not call a method `job`. In order to fix this, we have to change
that line:

```rust,ignore
(job.job)();
```

It looks a little funky, but it works. Well, almost. Now we get a different error:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0161]: cannot move a value of type std::ops::FnOnce() + std::marker::Send + 'static: the size of std::ops::FnOnce() + std::marker::Send + 'static cannot be statically determined
  --> src\main.rs:69:17
   |
69 |                 (job.job)();
   |                 ^^^^^^^^^

error: aborting due to previous error
```

This error is fairly cryptic. And that's because the problem is fairly cryptic. Basically,
in order to call a boxed `FnOnce`, it needs to be able to move itself out of the box. But
the compiler doesn't understand that this is okay to do.

In the future, this code should work just fine. But that doesn't help us in the present.

Luckily, there's a trick. It looks like this:

```rust,ignore
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

struct Job {
    job: Box<FnBox + Send + 'static>,
}

// we use this instead of (job.job)();
job.job.call_box();
```

Here's how the trick works: Rust _does_ understand that when `self` is a `Box<T>`,
it can be moved out of. As such, we do four things:

First, we create a new trait, `FnBox`. This trait has one method, `call_box`, similar
to the `call` methods on the other `Fn*` traits. This method takes `Box<Self>`.

Next, we implement `FnBox` for all things that implement `FnOnce()`:

```rust,ignore
impl<F: FnOnce()> FnBox for F {
```

That's what this line says: for any type `F` that implements `FnOnce()`, we are going
to implement `FnBox` for that type. Effectively, this means that any `FnOnce()` closures
can use our `call_box` method. Tricky!


Here's the implementation of `call_box`:

```rust,ignore
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
```

We do the same thing with `()()`s as we did above, only now instead of `job.job`, it's
`self`. And the dereference of self is what moves the contents out of the box.

Finally, we use `call_box` instead of invoking the closure directly.

This is a very sneaky, complicated trick. Don't worry too much if it doesn't make perfect
sense; someday, it will be completely un-needed.

With this trick, our thread pool is in a working state! Give it a `cargo run`, and make
some requests:

```text
> cargo run
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
warning: field is never used: `threads`, #[warn(dead_code)] on by default
  --> src\main.rs:50:5
   |
50 |     threads: Vec<Worker>,
   |     ^^^^^^^^^^^^^^^^^^^^

warning: field is never used: `id`, #[warn(dead_code)] on by default
  --> src\main.rs:69:5
   |
69 |     id: u32,
   |     ^^^^^^^

warning: field is never used: `thread`, #[warn(dead_code)] on by default
  --> src\main.rs:70:5
   |
70 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    Finished dev [unoptimized + debuginfo] target(s) in 0.99 secs
     Running `target\debug\hello.exe`
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

Success! We now have a threadpool executing connections asynchronously.

What about those warnings, though? Don't we use all those things? Well, here's
the thing: right now, we are using all three of these things to hold onto some
data, but we don't actually _do_ anything with them. That is, we set up a ton
of interesting stuff, but then it just sits there.

So are these warnings wrong? In one sense yes, but in another sense, no. We
never do anything to clean up our threadpool once it's done being used. Let's
implement that now.

# Implementing Drop

The first thing we want to do is to implement `Drop` for our threadpool. When
the pool is dropped, we should join on all of our threads, to make sure they
finish their work. Here's a first attempt at it; it won't quite work yet:

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

Here's the idea: we loop through each of our `threads`, using `&mut` because `self` is
itself a mutable reference. If we tried to iterate over the threads directly, we'd
get an error about moving. Anyway, we print out a message saying that that particular
worker is shutting down, and then we call `join` on that worker's thread. An `unwrap`
disregards the errors.

Here's the error we get:

```text
> cargo run
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0507]: cannot move out of borrowed content
   --> src\main.rs:129:13
    |
129 |             worker.thread.join();
    |             ^^^^^^ cannot move out of borrowed content

error: aborting due to previous error
```

Because we only have a `&mut` in `drop`, we cannot actually call `join`, as `join`
takes its argument by value. What to do? Well, we already have a way to represent
"something or nothing", and that's `Option<T>`. Let's update the definition of
`Worker`:

```rust,ignore
struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
}
```

And then let the compiler tell us about anything we need to fix:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0308]: mismatched types
  --> src\main.rs:87:21
   |
87 |             thread: thread,
   |                     ^^^^^^ expected enum `std::option::Option`, found struct `std::thread::JoinHandle`
   |
   = note: expected type `std::option::Option<std::thread::JoinHandle<()>>`
              found type `std::thread::JoinHandle<_>`

error: no method named `join` found for type `std::option::Option<std::thread::JoinHandle<()>>` in the current scope
   --> src\main.rs:129:27
    |
129 |             worker.thread.join();
    |                           ^^^^

```

The first error is easy to fix; we need to add a `Some` at the end of `ThreadPool::new`:

```rust,ignore
Worker {
        id: id,
        thread: Some(thread),
    }
```

The second one is in our `Drop` implementation. Here's one that works:

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

The `take` method on `Option`, well, takes something out of it. That is, if
the `Option` is `Some(T)`, it will set the original option to `None`, and then
return that `Some(T)`. If the option is `None`, it will return `None`.

We use `if let` to check if the return value of `take` is `Some`, and if it is,
we call `join` on that thread.

With this, our code compiles without any warnings, and still works!

... or does it? There's one last issue we haven't handled yet: this `Drop`
implementation doesn't actually work. The key is the logic of our `Worker`s.
There's no way to shut them down; they only loop forever looking for jobs. If
we try to drop our `ThreadPool` with this implementation, it will block forever
on the first thread.

So what do we do? We need to modify our channel to take a `Message` instead
of a `Job`. Like this:

```rust,ignore
enum Message {
    NewJob(Job),
    Terminate,
}
```

First, we have a new `Message` enum. We have two kinds of messages: "here's a new
`Job`" and "please terminate execution."

```rust,ignore
struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
```

We need to adjust the `ThreadPool` to send `Message`s rather than `Job`s.

```rust,ignore
impl Worker {
    fn new(id: u32, job_receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move ||{
            loop {
                let message = job_receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
```

Inside of our `Worker, instead of receiving a `Job`, we get a `Message`. We then
execute the job if it's a `NewJob`, and break out of our `loop` if it's `Terminate`.

```rust,ignore
impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Message>();

        // no other changes here
    }

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Job {
            job: Box::new(f),
        };

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
```

`ThreadPool` has two changes: first, we need our channel to be of `Message`s instead
of `Job`s. Then, in `execute`, we need to send a `NewJob` rather than just a `Job`.

With these changes, things compile again. But we haven't sent any `Terminate` messages.
Let's change our `Drop` implementation:

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.threads {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

We need two loops here. Why? Well, if we send a message and then try to join,
it's not guaranteed that that worker will be the one that gets that message.
We'd then deadlock. Imagine this scenario: we have two worker threads. We
send a terminate message down the channel, and then join thread one. But thread
one is busy processing a request; thread two is idle. This means thread two would
get the terminate message and shut down; but we're waiting for thread one to shut
down. Since `join` blocks until shut down, we're now blocking forever, and will
never send the second message to terminate. Deadlock!

To prevent this, we first put all of our `Terminate` messages on the
channel, and then we join on all the threads.

Let's give it a try: modify `main` to only accept a small number of requests
before shutting the server down:

```rust,ignore
    let mut counter = 0;

    for stream in listener.incoming() {
        if counter == 2 {
            println!("Shutting down.");
            break;
        }

        counter += 1;
```

And then run it with `cargo run`. Load up the pages a few times, and then check
your terminal. You'll see something like this:

```text
> cargo run
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0 secs
     Running `target\debug\hello.exe`
Worker 0 got a job; executing.
Worker 1 got a job; executing.
Shutting down.
Sending terminate message to all workers.
Shutting down worker 0
Worker 2 was told to terminate.
Worker 3 was told to terminate.
Worker 0 was told to terminate.
Worker 1 was told to terminate.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

You may get a different ordering of course. We can see how this works from the messages
though; workers zero and one get the two page loads, and then, we stop accepting
connections. When the `Pool` goes out of scope at the end of `main`, its `Drop`
implementation kicks in, and tells all workers to terminate. They then each print
the message that they have seen the terminate message, and then they all get shut
down. One interesting thing about this particular execution: you'll notice that we
told every worker to terminate, and then immediately tried to join worker zero.
Since it had not yet gotten the terminate message, it waited, and the threads
each acknowledged their termination.

Congrats! We now have completed our project. Here's the full code, for reference:

```rust,no_run
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

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

    let start = &buffer[..get.len()];

    let (header, filename) = if start == get {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

struct ThreadPool {
    threads: Vec<Worker>,
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

struct Job {
    job: Box<FnBox + Send + 'static>,
}

struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    fn new(id: u32, job_receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move ||{
            loop {
                let message = job_receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Message>();

        let job_receiver = Arc::new(Mutex::new(job_receiver));

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32, job_receiver.clone()));
        }

        ThreadPool {
            threads: threads,
            sender: job_sender,
        }
    }

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Job {
            job: Box::new(f),
        };

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.threads {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

There is still more we could do here; for example, our `ThreadPool` is not
inherently tied to HTTP handling, so we could extract it into its own submodule,
or maybe even its own crate!