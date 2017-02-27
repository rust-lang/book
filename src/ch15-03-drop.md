## The `Drop` Trait Runs Code on Cleanup

The other trait that's important to the smart pointer pattern is the `Drop`
trait. `Drop` lets us run some code when a value is about to go out of scope.
This is especially useful for smart pointers that manage a resource as opposed
to those that manage memory: often resources like files or network connections
need to be closed when our code is done with them. In other languages, if we
forget to call code to close these kinds of resources, the system our code is
running on might get overloaded and crash.

In Rust, we can specify that some code should be run when a value goes out of
scope. The compiler will insert this code automatically. That means we don't
need to remember to put this code everywhere we're done with an instance of
these types, but we still won't leak resources!

The way we specify code should be run when a value goes out of scope is by
implementing the `Drop` trait. The `Drop` trait requires us to implement one
method named `drop` that takes a mutable reference to `self`.

Listing 15-6 shows a `WebSocket` struct that doesn't actually connect to
anything, but it prints out `Closing the socket!` when we create the struct and
when it goes out of scope so that we can see when this code gets run:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
struct WebSocket {
    uri: String,
}

impl Drop for WebSocket {
    fn drop(&mut self) {
        println!("Closing the socket!");
    }
}

fn main() {
    let w = WebSocket { uri: String::from("http://example.com/not-real") };
    println!("WebSocket created.");
    println!("Wait for it...");
}
```

<figcaption>

Listing 15-6: A `WebSocket` struct that implements the `Drop` trait, where we
could put code that would close the socket.

</figcaption>
</figure>

The `Drop` trait is in the prelude, so we don't need to import it. The `drop`
method implementation calls the `println!`; this is where you'd put the actual
code needed to close the socket. In `main`, we create a new instance of
`WebSocket` then print out `WebSocket created.` to be able to see that our code
got to that point at runtime. At the end of `main`, our instance of `WebSocket`
will go out of scope. Note that we didn't call the `drop` method explicitly.

When we run this program, we'll see:

```text
WebSocket created.
Wait for it...
Closing the socket!
```

printed to the screen, which shows that Rust automatically called `drop` for us
when our instance went out of scope.

We can use the `std::mem::drop` function to drop a value earlier than when it
goes out of scope. This isn't usually necessary; the whole point of the `Drop`
trait is that it's taken care of automatically for us. We'll see an example of
a case when we'll need to drop a value earlier than when it goes out of scope
in Chapter 16 when we're talking about concurrency. For now, let's just see
that it's possible, and `std::mem::drop` is in the prelude so we can just call
`drop` as shown in Listing 15-7:

<figure>

```rust,ignore
fn main() {
    let w = WebSocket { uri: String::from("http://example.com/not-real") };
    println!("WebSocket created.");
    drop(w);
    println!("Wait for it...");
}
```

<figcaption>

Listing 15-7: Calling `std::mem::drop` to explicitly drop a value before it
goes out of scope

</figcaption>
</figure>

Running this code will print the following, showing that the destructor code is
called since `Closing the socket!` is printed between `WebSocket created.` and
`Wait for it...`:

```text
WebSocket created.
Closing the socket!
Wait for it...
```

Note that we aren't allowed to call the `drop` method that we defined directly:
if we replaced `drop(w)` in Listing 15-7 with `w.drop()`, we'll get a compiler
error that says `explicit destructor calls not allowed`. TODO: why aren't we
allowed to call the drop method directly?

The definition of `std::mem::drop` is:

```rust
pub mod std {
    pub mod mem {
        pub fn drop<T>(_x: T) { }
    }
}
```

This function is generic over any type `T`, so we can pass any value to it. The
function doesn't actually have anything in its body, so it doesn't use its
parameter. The parameter is named `_x` because the `_` is a signal to the
compiler that we're intentionally not using the parameter, so it doesn't need
to warn us that we're not using it. The reason this empty function is useful is
that `drop` takes ownership of its parameter, which means the value gets
dropped at the end of this function when it goes out of scope.

Code specified in a `Drop` trait implementation can be used for many reasons to
make cleanup convenient and safe: we could use it to create our own memory
allocator, for instance! By using the `Drop` trait and Rust's ownership system,
we can't mess up and forget to clean up, or clean up a value that's still in
use. The ownership system makes sure that we call `drop` at the right time.

Now that we've gone over `Box<T>` and some of the characteristics of smart
pointers, let's talk about a few other smart pointers defined in the standard
library that add different kinds of useful functionality.
