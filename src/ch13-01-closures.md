## Closures: Anonymous Functions that Can Capture Their Environment

Rust’s closures are anonymous functions you can save in a variable or pass as
arguments to other functions. You can create the closure in one place and then
call the closure to evaluate it in a different context. Unlike functions,
closures can capture values from the scope in which they’re defined. We’ll
demonstrate how these closure features allow for code reuse and behavior
customization.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>

### Capturing the Environment with Closures

The first aspect of closures we’re going to examine is that closures can
capture values from the environment they’re defined in for later use. Here’s
the scenario: A t-shirt company gives away a free shirt to someone on their
mailing list every so often. People on the mailing list can optionally add
their favorite color to their profile. If the person chosen to get the free
shirt has their favorite color in their profile, they get that color shirt. If
the person hasn’t specified a favorite color, they get the color that the
company currently has the most of.

There are many ways to implement this. For this example, we’re going to use an
enum called `ShirtColor` that has the variants `Red` and `Blue`. The
company’s inventory is represented by an `Inventory` struct that has a field
named `shirts` that contains a `Vec<ShirtColor>` representing the shirts
currently in stock. The method `shirt_giveaway` defined on `Inventory` gets the
optional shirt color preference of the person getting the free shirt, and
returns the shirt color the person will get. This is shown in Listing 13-1:

<span class="filename">Filename: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

<span class="caption">Listing 13-1: Shirt company giveaway</span>

The `store` defined in `main` has two blue shirts and one red shirt in stock.
Then it calls the `giveaway` method for a user with a preference for a red
shirt and a user without any preference. Running this code prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Again, this code could be implemented in many ways, but this way uses concepts
you’ve already learned, except for the body of the `giveaway` method that uses
a closure. The `giveaway` method takes the user preference `Option<ShirtColor>`
and calls `unwrap_or_else` on it. The [`unwrap_or_else` method on
`Option<T>`][unwrap-or-else]<!-- ignore --> is defined by the standard library.
It takes one argument: a closure without any arguments that returns a value `T`
(the same type stored in the `Some` variant of the `Option<T>`, in this case, a
`ShirtColor`). If the `Option<T>` is the `Some` variant, `unwrap_or_else`
returns the value from within the `Some`. If the `Option<T>` is the `None`
variant, `unwrap_or_else` calls the closure and returns the value returned by
the closure.

This is interesting because we’ve passed a closure that calls
`self.most_stocked()` on the current `Inventory` instance. The standard library
didn’t need to know anything about the `Inventory` or `ShirtColor` types we
defined, or the logic we want to use in this scenario. The closure captured an
immutable reference to the `self` `Inventory` instance and passed it with the
code we specified to the `unwrap_or_else` method. Functions are not able to
capture their environment in this way.

### Closure Type Inference and Annotation

There are more differences between functions and closures. Closures don’t
usually require you to annotate the types of the parameters or the return value
like `fn` functions do. Type annotations are required on functions because
they’re part of an explicit interface exposed to your users. Defining this
interface rigidly is important for ensuring that everyone agrees on what types
of values a function uses and returns. But closures aren’t used in an exposed
interface like this: they’re stored in variables and used without naming them
and exposing them to users of our library.

Closures are typically short and relevant only within a narrow context rather
than in any arbitrary scenario. Within these limited contexts, the compiler can
infer the types of the parameters and the return type, similar to how it’s able
to infer the types of most variables (there are rare cases where the compiler
needs closure type annotations too).

As with variables, we can add type annotations if we want to increase
explicitness and clarity at the cost of being more verbose than is strictly
necessary. Annotating the types for a closure would look like the definition
shown in Listing 13-2.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

<span class="caption">Listing 13-2: Adding optional type annotations of the
parameter and return value types in the closure</span>

With type annotations added, the syntax of closures looks more similar to the
syntax of functions. The following is a vertical comparison of the syntax for
the definition of a function that adds 1 to its parameter and a closure that
has the same behavior. We’ve added some spaces to line up the relevant parts.
This illustrates how closure syntax is similar to function syntax except for
the use of pipes and the amount of syntax that is optional:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

The first line shows a function definition, and the second line shows a fully
annotated closure definition. The third line removes the type annotations from
the closure definition, and the fourth line removes the brackets, which are
optional because the closure body has only one expression. These are all valid
definitions that will produce the same behavior when they’re called. Calling
the closures is required for `add_one_v3` and `add_one_v4` to be able to
compile because the types will be inferred from their usage.

Closure definitions will have one concrete type inferred for each of their
parameters and for their return value. For instance, Listing 13-3 shows the
definition of a short closure that just returns the value it receives as a
parameter. This closure isn’t very useful except for the purposes of this
example. Note that we haven’t added any type annotations to the definition: if
we then try to call the closure twice, using a `String` as an argument the
first time and a `u32` the second time, we’ll get an error.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<span class="caption">Listing 13-3: Attempting to call a closure whose types
are inferred with two different types</span>

The compiler gives us this error:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

The first time we call `example_closure` with the `String` value, the compiler
infers the type of `x` and the return type of the closure to be `String`. Those
types are then locked into the closure in `example_closure`, and we get a type
error if we try to use a different type with the same closure.

### Capturing References or Moving Ownership

Closures can capture values from their environment in three ways, which
directly map to the three ways a function can take a parameter: borrowing
immutably, borrowing mutably, and taking ownership. The closure will decide
which of these to use based on what the body of the function does with the
captured values.

Listing 13-4 defines a closure that captures an immutable borrow to the vector
named `list` because it only needs an immutable borrow to print the value. This
example also illustrates that a variable can bind to a closure definition, and
the closure can later be called by using the variable name and parentheses as
if the variable name were a function name:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

<span class="caption">Listing 13-4: Defining and calling a closure that
captures an immutable borrow</span>

The `list` is still accessible by the code before the closure definition, after
the closure definition but before the closure is called, and after the closure
is called because we can have multiple immutable borrows of `list` at the same
time. This code compiles, runs, and prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Next, Listing 13-5 changes the closure definition to need a mutable borrow
because the closure body adds an element to the `list` vector:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

<span class="caption">Listing 13-5: Defining and calling a closure that
captures a mutable borrow</span>

This code compiles, runs, and prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

Note that there’s no longer a `println!` between the definition and the call of
the `borrows_mutably` closure: when `borrows_mutably` is defined, it captures a
mutable reference to `list`. After the closure is called, because we don’t use
the closure again after that point, the mutable borrow ends. Between the
closure definition and the closure call, an immutable borrow to print isn’t
allowed because no other borrows are allowed when there’s a mutable borrow. Try
adding a `println!` there to see what error message you get!

If you want to force the closure to take ownership of the values it uses in the
environment even though the body of the closure doesn’t strictly need
ownership, you can use the `move` keyword before the parameter list. This
technique is mostly useful when passing a closure to a new thread to move the
data so it’s owned by the new thread. We’ll have more examples of `move`
closures in Chapter 16 when we talk about concurrency.

<!-- Old headings. Do not remove or links may break. -->
<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>

### Moving Captured Values Out of the Closure and the `Fn` Traits

Once a closure has captured a reference or moved a value into the closure, the
code in the body of the function also affects what happens to the references or
values as a result of calling the function. A closure body can move a captured
value out of the closure, can mutate the captured value, can neither move nor
mutate the captured value, or can capture nothing from the environment. The way
a closure captures and handles values from the environment affects which traits
the closure implements. The traits are how functions and structs can specify
what kinds of closures they can use.

Closures will automatically implement one, two, or all three of these `Fn`
traits, in an additive fashion:

1. `FnOnce` applies to closures that can be called at least once. All closures
   implement this trait, because all closures can be called. If a closure moves
   captured values out of its body, then that closure only implements `FnOnce`
   and not any of the other `Fn` traits, because it can only be called once.
2. `FnMut` applies to closures that don’t move captured values out of their
   body, but that might mutate the captured values. These closures can be
   called more than once.
3. `Fn` applies to closures that don’t move captured values out of their body
   and that don’t mutate captured values. These closures can be called more
   than once without mutating their environment, which is important in cases
   such as calling a closure multiple times concurrently. Closures that don’t
   capture anything from their environment implement `Fn`.

Let’s look at the definition of the `unwrap_or_else` method on `Option<T>` that
we used in Listing 13-6:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Recall that `T` is the generic type representing the type of the value in the
`Some` variant of an `Option`. That type `T` is also the return type of the
`unwrap_or_else` function: code that calls `unwrap_or_else` on an
`Option<String>`, for example, will get a `String`.

Next, notice that the `unwrap_or_else` function has an additional generic type
parameter, `F`. The `F` type is the type of the parameter named `f`, which is
the closure we provide when calling `unwrap_or_else`.

The trait bound specified on the generic type `F` is `FnOnce() -> T`, which
means `F` must be able to be called at least once, take no arguments, and
return a `T`. Using `FnOnce` in the trait bound expresses the constraint that
`unwrap_or_else` is only going to call `f` at most one time. In the body of
`unwrap_or_else`, we can see that if the `Option` is `Some`, `f` won’t be
called. If the `Option` is `None`, `f` will be called once. Because all
closures implement `FnOnce`, `unwrap_or_else` accepts the most different kinds
of closures and is as flexible as it can be.

> Note: Functions can implement all three of the `Fn` traits too. If what we
> want to do doesn’t require capturing a value from the environment, we can use
> the name of a function rather than a closure where we need something that
> implements one of the `Fn` traits. For example, on an `Option<Vec<T>>` value,
> we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if the
> value is `None`.

Now let’s look at the standard library method `sort_by_key` defined on slices,
to see how that differs. It takes a closure that implements `FnMut`. The
closure gets one argument, a reference to the current item in the slice being
considered, and returns a value of type `K` that can be ordered. This function
is useful when you want to sort a slice by a particular attribute of each item.
In Listing 13-7, we have a list of `Rectangle` instances and we use
`sort_by_key` to order them by their `width` attribute from low to high:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

<span class="caption">Listing 13-7: Using `sort_by_key` and a closure to sort a
list of `Rectangle` instances by their `width` value</span>

This code prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

The reason `sort_by_key` is defined to take an `FnMut` closure is that it calls
the closure multiple times: once for each item in the slice. The closure `|r|
r.width` doesn’t capture, mutate, or move out anything from its environment, so
it meets the trait bound requirements.

In contrast, Listing 13-8 shows an example of a closure that only implements
`FnOnce` because it moves a value out of the environment. The compiler won’t
let us use this closure with `sort_by_key`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

<span class="caption">Listing 13-8: Attempting to use an `FnOnce` closure with
`sort_by_key`</span>

This is a contrived, convoluted way (that doesn’t work) to try and count the
number of times `sort_by_key` gets called when sorting `list`. This code
attempts to do this counting by pushing `value`, a `String` from the closure’s
environment, into the `sort_operations` vector. The closure captures `value`
then moves `value` out of the closure by transferring ownership of `value` to
the `sort_operations` vector. This closure can be called once; trying to call
it a second time wouldn’t work because `value` would no longer be in the
environment to be pushed into `sort_operations` again! Therefore, this closure
only implements `FnOnce`. When we try to compile this code, we get this error
that `value` can’t be moved out of the closure because the closure must
implement `FnMut`:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

The error points to the line in the closure body that moves `value` out of the
environment. To fix this, we need to change the closure body so that it doesn’t
move values out of the environment. If we’re interested in the number of times
`sort_by_key` is called, keeping a counter in the environment and incrementing
its value in the closure body is a more straightforward way to calculate that.
The closure in Listing 13-9 works with `sort_by_key` because it is only
capturing a mutable reference to the `num_sort_operations` counter and can
therefore be called more than once:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

<span class="caption">Listing 13-9: Using an `FnMut` closure with `sort_by_key`
is allowed</span>

The `Fn` traits are important when defining or using functions or types that
make use of closures. The next section discusses iterators, and many iterator
methods take closure arguments. Keep these details of closures in mind as we
explore iterators!

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
