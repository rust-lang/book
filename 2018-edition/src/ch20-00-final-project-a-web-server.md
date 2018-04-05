# Final Project: Building a Multithreaded Web Server

It’s been a long journey, but here we are! The end of the book. Parting is such
sweet sorrow. But before we go, let’s build one more project together, to show
off some of the concepts we covered in these final chapters, as well as recap
some lessons from earlier.

For our final project we’re going to make a web server that only says “hello”;
which will look like Figure 20-1 in a web browser:

![hello from rust](img/trpl20-01.png)

<span class="caption">Figure 20-1: Our final shared project together</span>

Here’s the plan of how we’ll build the web server:

1. Learn a little bit about TCP and HTTP
2. Listen for TCP connections on a socket
3. Parse a small number of HTTP requests
4. Create a proper HTTP response
5. Improve the throughput of our server with a thread pool

Before we get started, however, there’s one thing we should mention: the method
we use here will not be the best way to build a web server with Rust. There are
a number of production-ready crates available on *https://crates.io* that
provide much more complete web server and thread pool implementations than we
are going to build.

However, for this chapter, our intention is to help you learn, not to take the
easy route. Because Rust is a systems programming language, we’re able to
choose what level of abstraction we want to work with, and can go to a lower
level than is possible or practical in other languages. We’ll therefore write
the basic HTTP server and thread pool ourselves so you can learn the general
ideas and techniques behind the crates you might use in the future.
