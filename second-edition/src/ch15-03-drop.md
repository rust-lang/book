## The `Drop` Trait Runs Code on Cleanup

The second trait important to the smart pointer pattern is `Drop`, which lets
us customize what happens when a value is about to go out of scope. We can
provide an implementation for the `Drop` trait on any type, and the code we
specify can be used to release resources like files or network connections.
We're introducing `Drop` in the context of smart pointers because the
functionality of the `Drop` trait is almost always used when implementing a
smart pointer. For example, `Box<T>` customizes `Drop` in order to deallocate
the space on the heap that the box points to.

In some languages, the programmer must call code to free memory or resources
every time they finish using an instance of a smart pointer. If they forget,
the system might become overloaded and crash. In Rust, we can specify that a
particular bit of code should be run whenever a value goes out of scope, and
the compiler will insert this code automatically.

<!-- Are we saying that any code can be run, and that we can use that to clean
up, or that this code that can be run is specifically always for clean up? -->
<!-- I don't understand what the difference between those two choices are?
/Carol -->

This means we don't need to be careful about placing clean up code everywhere
in a program that an instance of a particular type is finished with, but we
still won't leak resources!

We specify the code to run when a value goes out of scope by implementing the
`Drop` trait. The `Drop` trait requires us to implement one method named `drop`
that takes a mutable reference to `self`. In order to be able to see when Rust
calls `drop`, let's implement `drop` with `println!` statements for now.

<!-- Why are we showing this as an example and not an example of it being used
for clean up? -->
<!-- To demonstrate the mechanics of implementing the trait and showing when
this code gets run. It's hard to experience the cleaning up unless we print
something. /Carol -->

Listing 15-8 shows a `CustomSmartPointer` struct whose only custom
functionality is that it will print out `Dropping CustomSmartPointer!` when the
instance goes out of scope. This will demonstrate when Rust runs the `drop`
function:

<!-- Is this below just telling us how to adapt it for cleaning up instead?
Maybe save it for when we have context for it? Instead of a `println!`
statement, you'd fill in `drop` with whatever cleanup code your smart pointer
needs to run: -->
<!-- This is demonstrating what we need to do to use `Drop`, without getting
into the complexities of what "cleaning up" might mean yet, just to give the
reader an idea of when this code gets called and that it gets called
automatically. We're building up to cleaning up. /Carol -->

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
}
```

<span class="caption">Listing 15-8: A `CustomSmartPointer` struct that
implements the `Drop` trait, where we would put our clean up code.</span>

The `Drop` trait is included in the prelude, so we don't need to import it. We
implement the `Drop` trait on `CustomSmartPointer`, and provide an
implementation for the `drop` method that calls `println!`. The body of the
`drop` function is where you'd put any logic that you wanted to run when an
instance of your type goes out of scope. We're choosing to print out some text
here in order to demonstrate when Rust will call `drop`.

<!-- Where you'd put this code, or where this code would be called? It seems
laborious to write this clean up code wherever there's a print call? -->
<!-- I'm not sure how you concluded that from what we had here, could you
elaborate? /Carol -->

In `main`, we create a new instance of `CustomSmartPointer` and then print out
`CustomSmartPointer created.`. At the end of `main`, our instance of
`CustomSmartPointer` will go out of scope, and Rust will call the code we put
in the `drop` method, printing our final message. Note that we didn't need to
call the `drop` method explicitly.

When we run this program, we'll see the following output:

```text
CustomSmartPointer created.
Dropping CustomSmartPointer!
```

Rust automatically called `drop` for us when our instance went out of scope,
calling the code we specified. This is just to give you a visual guide to how
the drop method works, but usually you would specify the cleanup code that your
type needs to run rather than a print message.

<!-- Can you wrap this example up by saying what you would actually put in a
drop method and why?-->
<!-- Done /Carol -->

#### Dropping a Value Early with `std::mem::drop`

<!-- is this a new method from Drop or the same method? -->
<!-- This is a new function. /Carol -->

Rust inserts the call to `drop` automatically when a value goes out of scope,
and it's not straightforward to disable this functionality. Disabling `drop`
isn't usually necessary; the whole point of the `Drop` trait is that it's taken
care of automatically for us. Occasionally you may find that you want to clean
up a value early. One example is when using smart pointers that manage locks;
you may want to force the `drop` method that releases the lock to run so that
other code in the same scope can acquire the lock. First, let's see what
happens if we try to call the `Drop` trait's `drop` method ourselves by
modifying the `main` function from Listing 15-8 as shown in Listing 15-9:

<!-- Above: I'm not following why we are doing this, if it's not necessary and
we aren't going to cover it now anyway -- can you lay out why we're discussing
this here? -->
<!-- Done. /Carol -->

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
```

<span class="caption">Listing 15-9: Attempting to call the `drop` method from
the `Drop` trait manually to clean up early</span>

If we try to compile this, we'll get this error:

```text
error[E0040]: explicit use of destructor method
  --> src/main.rs:15:7
   |
15 |     c.drop();
   |       ^^^^ explicit destructor calls not allowed
```

This error message says we're not allowed to explicitly call `drop`. The error
message uses the term *destructor*, which is the general programming term for a
function that cleans up an instance. A *destructor* is analogous to a
*constructor* that creates an instance. The `drop` function in Rust is one
particular destructor.

Rust doesn't let us call `drop` explicitly because Rust would still
automatically call `drop` on the value at the end of `main`, and this would be
a *double free* error since Rust would be trying to clean up the same value
twice.

Because we can't disable the automatic insertion of `drop` when a value goes
out of scope, and we can't call the `drop` method explicitly, if we need to
force a value to be cleaned up early, we can use the `std::mem::drop` function.

The `std::mem::drop` function is different than the `drop` method in the `Drop`
trait. We call it by passing the value we want to force to be dropped early as
an argument. `std::mem::drop` is in the prelude, so we can modify `main` from
Listing 15-8 to call the `drop` function as shown in Listing 15-10:

<span class="filename">Filename: src/main.rs</span>

```rust
# struct CustomSmartPointer {
#     data: String,
# }
#
# impl Drop for CustomSmartPointer {
#     fn drop(&mut self) {
#         println!("Dropping CustomSmartPointer!");
#     }
# }
#
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

<span class="caption">Listing 15-10: Calling `std::mem::drop` to explicitly
drop a value before it goes out of scope</span>

Running this code will print the following:

```text
CustomSmartPointer created.
Dropping CustomSmartPointer!
CustomSmartPointer dropped before the end of main.
```

<!-- What's the destructor code, here? We haven't mentioned that before, not in
this chapter in any case -->
<!-- I added a definition for destructor a few paragraphs above, the first time
we see it in an error message. /Carol -->

The `Dropping CustomSmartPointer!` is printed between `CustomSmartPointer
created.` and `CustomSmartPointer dropped before the end of main.`, showing
that the `drop` method code is called to drop `c` at that point.

<!-- How does this show that the destructor code (is that drop?) is called? Is
this correct, above?-->
<!-- The order of what gets printed shows that the drop code is called.
/Carol-->

Code specified in a `Drop` trait implementation can be used in many ways to
make cleanup convenient and safe: we could use it to create our own memory
allocator, for instance! With the `Drop` trait and Rust's ownership system, you
don't have to remember to clean up after yourself, Rust takes care of it
automatically.

We also don't have to worry about accidentally cleaning up values still in use
because that would cause a compiler error: the ownership system that makes sure
references are always valid will also make sure that `drop` only gets called
once when the value is no longer being used.

Now that we've gone over `Box<T>` and some of the characteristics of smart
pointers, let's talk about a few other smart pointers defined in the standard
library.
