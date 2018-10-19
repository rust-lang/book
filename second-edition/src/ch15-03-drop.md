## Running Code on Cleanup with the `Drop` Trait

The second trait important to the smart pointer pattern is `Drop`, which lets
you customize what happens when a value is about to go out of scope. You can
provide an implementation for the `Drop` trait on any type, and the code you
specify can be used to release resources like files or network connections.
We’re introducing `Drop` in the context of smart pointers because the
functionality of the `Drop` trait is almost always used when implementing a
smart pointer. For example, `Box<T>` customizes `Drop` to deallocate the space
on the heap that the box points to.

In some languages, the programmer must call code to free memory or resources
every time they finish using an instance of a smart pointer. If they forget,
the system might become overloaded and crash. In Rust, you can specify that a
particular bit of code be run whenever a value goes out of scope, and the
compiler will insert this code automatically. As a result, you don’t need to be
careful about placing cleanup code everywhere in a program that an instance of
a particular type is finished with—you still won’t leak resources!

Specify the code to run when a value goes out of scope by implementing the
`Drop` trait. The `Drop` trait requires you to implement one method named
`drop` that takes a mutable reference to `self`. To see when Rust calls `drop`,
let’s implement `drop` with `println!` statements for now.

Listing 15-14 shows a `CustomSmartPointer` struct whose only custom
functionality is that it will print `Dropping CustomSmartPointer!` when the
instance goes out of scope. This example demonstrates when Rust runs the `drop`
function.

<span class="filename">Filename: src/main.rs</span>

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
```

<span class="caption">Listing 15-14: A `CustomSmartPointer` struct that
implements the `Drop` trait where we would put our cleanup code</span>

The `Drop` trait is included in the prelude, so we don’t need to import it. We
implement the `Drop` trait on `CustomSmartPointer` and provide an
implementation for the `drop` method that calls `println!`. The body of the
`drop` function is where you would place any logic that you wanted to run when
an instance of your type goes out of scope. We’re printing some text here to
demonstrate when Rust will call `drop`.

In `main`, we create two instances of `CustomSmartPointer` and then print
`CustomSmartPointers created.`. At the end of `main`, our instances of
`CustomSmartPointer` will go out of scope, and Rust will call the code we put
in the `drop` method, printing our final message. Note that we didn’t need to
call the `drop` method explicitly.

When we run this program, we’ll see the following output:

```text
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

Rust automatically called `drop` for us when our instances went out of scope,
calling the code we specified. Variables are dropped in the reverse order of
their creation, so `d` was dropped before `c`. This example gives you a visual
guide to how the `drop` method works; usually you would specify the cleanup
code that your type needs to run rather than a print message.

### Dropping a Value Early with `std::mem::drop`

Unfortunately, it’s not straightforward to disable the automatic `drop`
functionality. Disabling `drop` isn’t usually necessary; the whole point of the
`Drop` trait is that it’s taken care of automatically. Occasionally, however,
you might want to clean up a value early. One example is when using smart
pointers that manage locks: you might want to force the `drop` method that
releases the lock to run so other code in the same scope can acquire the lock.
Rust doesn’t let you call the `Drop` trait’s `drop` method manually; instead
you have to call the `std::mem::drop` function provided by the standard library
if you want to force a value to be dropped before the end of its scope.

If we try to call the `Drop` trait’s `drop` method manually by modifying the
`main` function from Listing 15-14, as shown in Listing 15-15, we’ll get a
compiler error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
```

<span class="caption">Listing 15-15: Attempting to call the `drop` method from
the `Drop` trait manually to clean up early</span>

When we try to compile this code, we’ll get this error:

```text
error[E0040]: explicit use of destructor method
  --> src/main.rs:14:7
   |
14 |     c.drop();
   |       ^^^^ explicit destructor calls not allowed
```

This error message states that we’re not allowed to explicitly call `drop`. The
error message uses the term *destructor*, which is the general programming term
for a function that cleans up an instance. A *destructor* is analogous to a
*constructor*, which creates an instance. The `drop` function in Rust is one
particular destructor.

Rust doesn’t let us call `drop` explicitly because Rust would still
automatically call `drop` on the value at the end of `main`. This would be a
*double free* error because Rust would be trying to clean up the same value
twice.

We can’t disable the automatic insertion of `drop` when a value goes out of
scope, and we can’t call the `drop` method explicitly. So, if we need to force
a value to be cleaned up early, we can use the `std::mem::drop` function.

The `std::mem::drop` function is different than the `drop` method in the `Drop`
trait. We call it by passing the value we want to force to be dropped early as
an argument. The function is in the prelude, so we can modify `main` in Listing
15-15 to call the `drop` function, as shown in Listing 15-16:

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

<span class="caption">Listing 15-16: Calling `std::mem::drop` to explicitly
drop a value before it goes out of scope</span>

Running this code will print the following:

```text
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
```

The text ```Dropping CustomSmartPointer with data `some data`!``` is printed
between the `CustomSmartPointer created.` and `CustomSmartPointer dropped
before the end of main.` text, showing that the `drop` method code is called to
drop `c` at that point.

You can use code specified in a `Drop` trait implementation in many ways to
make cleanup convenient and safe: for instance, you could use it to create your
own memory allocator! With the `Drop` trait and Rust’s ownership system, you
don’t have to remember to clean up because Rust does it automatically.

You also don’t have to worry about problems resulting from accidentally
cleaning up values still in use: the ownership system that makes sure
references are always valid also ensures that `drop` gets called only once when
the value is no longer being used.

Now that we’ve examined `Box<T>` and some of the characteristics of smart
pointers, let’s look at a few other smart pointers defined in the standard
library.
