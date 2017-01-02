## Traits

*Traits* are similar to a feature often called 'interfaces' in other languages,
but are also different. Traits let us do another kind of abstraction: they let
us abstract over *behavior* that types can have in common.

When we use a generic type parameter, we are telling Rust that any type is
valid in that location. When other code *uses* a value that could be of any
type, we need to also tell Rust that the type has the functionality that we
need. Traits let us specify that, for example, we need any type `T` that has
methods defined on it that allow us to print a value of that type. This is
powerful because we can still leave our definitions generic to allow use of
many different types, but we can constrain the type at compile-time to types
that have the behavior we need to be able to use.

Listing 10-5 has an example definition of a trait named `Printable` with a
method named `print`:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
trait Printable {
    fn print(&self);
}
```

<figcaption>

Listing 10-5: A `Printable` trait definition with one method, `print`

</figcaption>
</figure>

We declare a trait with the `trait` keyword, then the trait's name. In this
case, our trait will describe types which can be printed. Inside of curly
braces, we declare a method signature, but instead of providing an
implementation inside curly braces, we put a semicolon after the signature. A
trait can have multiple methods in its body, with the method signatures listed
one per line and each line ending in a semicolon.

Implementing a trait for a particular type looks similar to implementing
methods on a type since it's also done with the `impl` keyword, but we specify
the trait name as well. Inside the `impl` block, we specify definitions for the
trait's methods in the context of the specific type. Listing 10-6 has an
example of implementing the `Printable` trait from Listing 10-5 (that only has
the `print` method) for a `Temperature` enum:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
# trait Printable {
#     fn print(&self);
# }
#
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

impl Printable for Temperature {
    fn print(&self) {
        match *self {
            Temperature::Celsius(val) => println!("{}°C", val),
            Temperature::Fahrenheit(val) => println!("{}°F", val),
        }
    }
}
```

<figcaption>

Listing 10-6: Implementing the `Printable` trait on a `Temperature` enum

</figcaption>
</figure>

In the same way `impl` lets us define methods, we've used it to define methods
that pertain to our trait. We can call methods that our trait has defined just
like we can call other methods:

<span class="filename">Filename: src/main.rs</span>

```rust
# trait Printable {
#     fn print(&self);
# }
#
# enum Temperature {
#     Celsius(i32),
#     Fahrenheit(i32),
# }
#
# impl Printable for Temperature {
#    fn print(&self) {
#        match *self {
#             Temperature::Celsius(val) => println!("{}°C", val),
#             Temperature::Fahrenheit(val) => println!("{}°F", val),
#         }
#     }
# }
#
fn main() {
    let t = Temperature::Celsius(37);

    t.print();
}
```

Note that in order to use a trait's methods, the trait itself must be in scope.
If the definition of `Printable` was in a module, the definition would need to
be defined as `pub` and we would need to `use` the trait in the scope where we
wanted to call the `print` method. This is because it's possible to have two
traits that both define a method named `print`, and our `Temperature` enum might
implement both. Rust wouldn't know which `print` method we wanted unless we
brought the trait we wanted into our current scope with `use`.

### Trait Bounds

Defining traits with methods and implementing the trait methods on a particular
type gives Rust more information than just defining methods on a type directly.
The information Rust gets is that the type that implements the trait can be
used in places where the code specifies that it needs some type that implements
a trait. To illustrate this, Listing 10-7 has a `print_anything` function
definition. This is similar to the `show_anything` function from Listing 10-4,
but this function has a *trait bound* on the generic type `T` and uses the
`print` function from the trait. A trait bound constrains the generic type to
be any type that implements the trait specified, instead of any type at all.
With the trait bound, we're then allowed to use the trait method `print` in the
function body:

<figure>
<span class="filename">Filename: src/lib.rs</figure>

```rust
# trait Printable {
#     fn print(&self);
# }
#
fn print_anything<T: Printable>(value: T) {
    println!("I have something to print for you!");
    value.print();
}
```

<figcaption>

Listing 10-7: A `print_anything` function that uses the trait bound `Printable`
on type `T`

</figcaption>
</figure>

Trait bounds are specified in the type name declarations within the angle
brackets. After the name of the type that you want to apply the bound to, add a
colon (`:`) and then specify the name of the trait. This function now specifies
that it takes a `value` parameter that can be of any type, as long as that type
implements the trait `Printable`. We need to specify the `Printable` trait in
the type name declarations because we want to be able to call the `print`
method that is part of the `Printable` trait.

Now we are able to call the `print_anything` function from Listing 10-7 and
pass it a `Temperature` instance as the `value` parameter, since we implemented
the trait `Printable` on `Temperature` in Listing 10-6:

<span class="filename">Filename: src/main.rs</span>

```rust
# trait Printable {
#     fn print(&self);
# }
#
# enum Temperature {
#     Celsius(i32),
#     Fahrenheit(i32),
# }
#
# impl Printable for Temperature {
#    fn print(&self) {
#        match *self {
#             Temperature::Celsius(val) => println!("{}°C", val),
#             Temperature::Fahrenheit(val) => println!("{}°F", val),
#         }
#     }
# }
#
# fn print_anything<T: Printable>(value: T) {
#     println!("I have something to print for you!");
#     value.print();
# }
#
fn main() {
    let temperature = Temperature::Fahrenheit(98);
    print_anything(temperature);
}
```

If we implement the `Printable` trait on other types, we can use them with the
`print_anything` method too. If we try to call `print_anything` with an `i32`,
which does *not* implement the `Printable` trait, we get a compile-time error
that looks like this:

```text
error[E0277]: the trait bound `{integer}: Printable` is not satisfied
   |
29 | print_anything(3);
   | ^^^^^^^^^^^^^^ trait `{integer}: Printable` not satisfied
   |
   = help: the following implementations were found:
   = help:   <Point as Printable>
   = note: required by `print_anything`
```

Traits are an extremely useful feature of Rust. You'll almost never see generic
functions without an accompanying trait bound. There are many traits in the
standard library, and they're used for many, many different things. For
example, our `Printable` trait is similar to one of those traits, `Display`.
And in fact, that's how `println!` decides how to format things with `{}`. The
`Display` trait has a `fmt` method that determines how to format something.

Listing 10-8 shows our original example from Listing 10-3, but this time using
the standard library's `Display` trait in the trait bound on the generic type
in the `show_anything` function:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
use std::fmt::Display;

fn show_anything<T: Display>(value: T) {
    println!("I have something to show you!");
    println!("It's: {}", value);
}
```

<figcaption>

Listing 10-8: The `show_anything` function with trait bounds

</figcaption>
</figure>

Now that this function specifies that `T` can be any type as long as that type
implements the `Display` trait, this code will compile.

### Multiple Trait Bounds and `where` Syntax

Each generic type can have its own trait bounds. The signature for a function
that takes a type `T` that implements `Display` and a type `U` that implements
`Printable` looks like:

```rust,ignore
fn some_function<T: Display, U: Printable>(value: T, other_value: U) {
```

To specify multiple trait bounds on one type, list the trait bounds in a list
with a `+` between each trait. For example, here's the signature of a function
that takes a type `T` that implements `Display` and `Clone` (which is another
standard library trait we have mentioned):

```rust,ignore
fn some_function<T: Display + Clone>(value: T) {
```

When trait bounds start getting complicated, there is another syntax that's a
bit cleaner: `where`. And in fact, the error we got when we ran the code from
Listing 10-3 referred to it:

```text
help: consider adding a `where T: std::fmt::Display` bound
```

The `where` syntax moves the trait bounds after the function parameters list.
This definition of `show_anything` means the exact same thing as the definition
in Listing 10-8, just said a different way:

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::fmt::Display;

fn show_anything<T>(value: T) where T: Display {
    println!("I have something to show you!");
    println!("It's: {}", value);
}
```

Instead of `T: Display` going inside the angle brackets, they go after the
`where` keyword at the end of the function signature. This can make complex
signatures easier to read. The `where` clause and its parts can also go on new
lines. Here's the signature of a function that takes three generic type
parameters that each have multiple trait bounds:

```rust,ignore
fn some_function<T, U, V>(t: T, u: U, v: V)
    where T: Display + Clone,
          U: Printable + Debug,
          V: Clone + Printable
{
```

Generic type parameters and trait bounds are part of Rust's rich type system.
Another important kind of generic in Rust interacts with Rust's ownership and
references features, and they're called *lifetimes*.
