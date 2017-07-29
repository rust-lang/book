## The `Drop` Trait Runs Code on Cleanup

The other trait that’s important to the smart pointer pattern is the `Drop`
trait. `Drop` lets us run some code when a value is about to go out of scope.
Smart pointers perform important cleanup when being dropped, like deallocating
memory or decrementing a reference count. More generally, data types can manage
resources beyond memory, like files or network connections, and use `Drop` to
release those resources when our code is done with them. We’re discussing
`Drop` in the context of smart pointers, though, because the functionality of
the `Drop` trait is almost always used when implementing smart pointers.

In some other languages, we have to remember to call code to free the memory or
resource every time we finish using an instance of a smart pointer. If we
forget, the system our code is running on might get overloaded and crash. In
Rust, we can specify that some code should be run when a value goes out of
scope, and the compiler will insert this code automatically. That means we don’t
need to remember to put this code everywhere we’re done with an instance of
these types, but we still won’t leak resources!

The way we specify code should be run when a value goes out of scope is by
implementing the `Drop` trait. The `Drop` trait requires us to implement one
method named `drop` that takes a mutable reference to `self`.

Listing 15-8 shows a `CustomSmartPointer` struct that doesn’t actually do
anything, but we’re printing out `CustomSmartPointer created.` right after we
create an instance of the struct and `Dropping CustomSmartPointer!` when the
instance goes out of scope so that we can see when each piece of code gets run.
Instead of a `println!` statement, you’d fill in `drop` with whatever cleanup
code your smart pointer needs to run:

<span class="filename">Filename: src/main.rs</span>

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer!");
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    println!("Wait for it...");
}
```

<span class="caption">Listing 15-8: A `CustomSmartPointer` struct that
implements the `Drop` trait, where we could put code that would clean up after
the `CustomSmartPointer`.</span>

The `Drop` trait is in the prelude, so we don’t need to import it. The `drop`
method implementation calls the `println!`; this is where you’d put the actual
code needed to close the socket. In `main`, we create a new instance of
`CustomSmartPointer` then print out `CustomSmartPointer created.` to be able to
see that our code got to that point at runtime. At the end of `main`, our
instance of `CustomSmartPointer` will go out of scope. Note that we didn’t call
the `drop` method explicitly.

When we run this program, we’ll see:

```text
CustomSmartPointer created.
Wait for it...
Dropping CustomSmartPointer!
```

printed to the screen, which shows that Rust automatically called `drop` for us
when our instance went out of scope.

We can use the `std::mem::drop` function to drop a value earlier than when it
goes out of scope. This isn’t usually necessary; the whole point of the `Drop`
trait is that it’s taken care of automatically for us. We’ll see an example of
a case when we’ll need to drop a value earlier than when it goes out of scope
in Chapter 16 when we’re talking about concurrency. For now, let’s just see
that it’s possible, and `std::mem::drop` is in the prelude so we can just call
`drop` as shown in Listing 15-9:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("Wait for it...");
}
```

<span class="caption">Listing 15-9: Calling `std::mem::drop` to explicitly drop
a value before it goes out of scope</span>

Running this code will print the following, showing that the destructor code is
called since `Dropping CustomSmartPointer!` is printed between
`CustomSmartPointer created.` and `Wait for it...`:

```text
CustomSmartPointer created.
Dropping CustomSmartPointer!
Wait for it...
```

Note that we aren’t allowed to call the `drop` method that we defined directly:
if we replaced `drop(c)` in Listing 15-9 with `c.drop()`, we’ll get a compiler
error that says `explicit destructor calls not allowed`. We’re not allowed to
call `Drop::drop` directly because when Rust inserts its call to `Drop::drop`
automatically when the value goes out of scope, then the value would get
dropped twice. Dropping a value twice could cause an error or corrupt memory,
so Rust doesn’t let us. Instead, we can use `std::mem::drop`, whose definition
is:

```rust
pub mod std {
    pub mod mem {
        pub fn drop<T>(x: T) { }
    }
}
```

This function is generic over any type `T`, so we can pass any value to it. The
function doesn’t actually have anything in its body, so it doesn’t use its
parameter. The reason this empty function is useful is that `drop` takes
ownership of its parameter, which means the value in `x` gets dropped at the
end of this function when `x` goes out of scope.

Code specified in a `Drop` trait implementation can be used for many reasons to
make cleanup convenient and safe: we could use it to create our own memory
allocator, for instance! By using the `Drop` trait and Rust’s ownership system,
we don’t have to remember to clean up after ourselves since Rust takes care of
it automatically. We’ll get compiler errors if we write code that would clean
up a value that’s still in use, since the ownership system that makes sure
references are always valid will also make sure that `drop` only gets called
one time when the value is no longer being used.

Now that we’ve gone over `Box<T>` and some of the characteristics of smart
pointers, let’s talk about a few other smart pointers defined in the standard
library that add different kinds of useful functionality.
