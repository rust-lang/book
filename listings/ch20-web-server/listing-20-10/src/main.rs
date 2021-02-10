use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
// ANCHOR: here
use std::thread;
use std::time::Duration;
// --snip--
// ANCHOR_END: here

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
// ANCHOR: here

fn handle_connection(mut stream: TcpStream) {
    // --snip--

    // ANCHOR_END: here
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // ANCHOR: here
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // --snip--
    // ANCHOR_END: here

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // ANCHOR: here
}
// ANCHOR_END: here
