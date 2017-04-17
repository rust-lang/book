## Validating the Request and Selectively Responding

Right now, our web server will return the HTML in the file no matter what the
client requested. Let's check that the browser is requesting `/`, and instead
return an error if the browser requests anything else. Let's modify
`handle_connection` as shown in Listing 20-6, which adds part of the code we'll
need. This part checks the content of the request we received against what we
know a request for `/` looks like and adds `if` and `else` blocks where we'll
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

    let start = &buffer[..get.len()];

    if start == get {
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

Here, we hardcoded the data corresponding to the request that we're looking for
in the variable `get`. Because we're reading raw bytes into the buffer, we use
a byte string, created with `b""`, to make `get` a byte string too. Then, we
take a slice of the `buffer` that's the same length as `get` and compare the
slice to `get`. If they're identical, we've gotten a well-formed request to
`/`, which is the success case that we want to handle in the `if` block. The
`if` block contains the code we added in Listing 20-5 that returns the HTML
file.

If `get` and the slice of `buffer` don't match, we've gotten some other
request. We'll respond to all other requests using the code we're about to add
in the `else` block.

If you run this code and request `127.0.0.1:8080`, you'll get the HTML from the
file. If you make any other request, such as `127.0.0.1:8080/something-else`,
you'll get a connection error like we saw when running the code in Listing 20-1
and Listing 20-2.

Let's add code to the `else` block as shown in Listing 20-7 to return a
response with the status code `404`, which signals that the content for the
request was not found. We'll also return HTML for a page to render in the
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
    let header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let mut file = File::open("404.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
# }
```

<span class="caption">Listing 20-7: Responding with status code `404` and an
error page if anything other than `/` was requested</span>

Here, our response has a header with status code `404` and the reason phrase
`NOT FOUND`. We still aren't returning any headers, and the body of the
response will be the HTML in the file *404.html*. Also create a *404.html* file
next to *hello.html* for the error page; again feel free to use any HTML you'd
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
`127.0.0.1:8080/foo`, should return the error HTML!

There's a lot of repetition between the code in the `if` and the `else` blocks:
they're both reading files and writing the contents of the files to the stream.
The only differences between the two cases are the status line and the
filename. Let's pull those differences out into an `if` and `else` of one line
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
#
#     let start = &buffer[..get.len()];
#
    // ...snip...

   let (status_line, filename) = if start == get {
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
to assign these two bits to `filename` and `header` using a pattern in the
`let` statement like we discussed in Chapter 18.

The duplicated code to read the file and write the response is now outside the
`if` and `else` blocks, and uses the `status_line` and `filename` variables.
This makes it easier to see exactly what's different between the two cases, and
makes it so that we only have one place to update the code if we want to change
how the file reading and response writing works. The behavior of the code in
Listing 20-9 will be exactly the same as that in Listing 20-8.

Awesome! We have a simple little web server in about 40 lines of Rust code that
responds to one request with a page of content and responds to all other
requests with a `404` response.

So far, this project has been relatively straightforward as far as Rust code
goes; we haven't done much of the more advanced things yet. Let's kick it up a
notch and add a feature to our web server: a thread pool.
