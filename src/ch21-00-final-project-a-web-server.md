# Final Project: Building a Multithreaded Web Server

It’s been a long journey, but we’ve reached the end of the book. In this
chapter, we’ll build one more project together to demonstrate some of the
concepts we covered in the final chapters, as well as recap some earlier
lessons.

For our final project, we’ll make a web server that says “hello” and looks like
Figure 21-1 in a web browser.

Here is our plan for building the web server:

1. Learn a bit about TCP and HTTP.
2. Listen for TCP connections on a socket.
3. Parse a small number of HTTP requests.
4. Create a proper HTTP response.
5. Improve the throughput of our server with a thread pool.

![hello from rust](img/trpl21-01.png)

<span class="caption">Figure 21-1: Our final shared project</span>

Before we get started, we should mention two details. First, the method we’ll
use won’t be the best way to build a web server with Rust. Community members
have published a number of production-ready crates available at
[crates.io](https://crates.io/) that provide more complete web server and thread
pool implementations than we’ll build. However, our intention in this chapter is
to help you learn, not to take the easy route. Because Rust is a systems
programming language, we can choose the level of abstraction we want to work
with and can go to a lower level than is possible or practical in other
languages.

Second, we will not be using async and await here. Building a thread pool is a
big enough challenge on its own, without adding in building an async runtime!
However, we will note how async and await might be applicable to some of the
same problems we will see in this chapter. Ultimately, as we noted back in
Chapter 17, many async runtimes use thread pools for managing their work.

We’ll therefore write the basic HTTP server and thread pool manually so you can
learn the general ideas and techniques behind the crates you might use in the
future.
