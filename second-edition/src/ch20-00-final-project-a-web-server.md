# Final Project: Building a Multithreaded Web Server

It’s been a long journey, but here we are! It’s the end of the book. Parting is
such sweet sorrow. But before we go, let’s build one more project together, to
show off some of the things we learned in these final chapters, as well as
re-cap some of the earlier ones.

Here’s what we’re going to make: a web server that says hello:

![hello from rust](img/trpl20-01.png)

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
