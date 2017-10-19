## Closures: Anonymous Functions that can Capture their Environment

Rust’s *closures* are anonymous functions you can save in a variable or pass as
arguments to other functions. You can create the closure in one place, and then
call the closure to evaluate it in a different context. Unlike functions,
closures are able to capture values from the scope in which they are called.
We’re going to demonstrate how these features of closures allow for code reuse
and customization of behavior.

### Creating an Abstraction of Behavior Using a Closure

Let’s work on an example of a situation in which it’s useful to store a closure
to be executed at a later time. We’ll talk about the syntax of closures, type
inference, and traits along the way.

The hypothetical situation is this: we work at a startup that’s making an app
to generate custom exercise workout plans. The backend is written in Rust, and
the algorithm that generates the workout plan takes into account many different
factors, like the app user’s age, Body Mass Index, preferences, recent
workouts, and an intensity number they specify. The actual algorithm used isn’t
important in this example; what’s important is that this calculation takes a
few seconds. We only want to call this algorithm when we need to, and only call
it once, so we aren’t making the user wait more than necessary.

We’ll simulate calling this hypothetical algorithm with the
`simulated_expensive_calculation` function shown in Listing 13-1, which will
print `calculating slowly...`, wait for two seconds, and then return whatever
number we passed in:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

<span class="caption">Listing 13-1: A function to stand in for a hypothetical
calculation that takes about two seconds to run</span>

Next, we have a `main` function that contains the parts of the workout app
important for this example. This represents the code that the app would call
when a user asks for a workout plan. Because the interaction with the app’s
frontend isn’t relevant to the use of closures, we’re going to hardcode values
representing inputs to our program and print the outputs.

The required inputs are:

* **An intensity number from the user**, specified when they request a
  workout to indicate whether they’d like a low intensity workout or a high
  intensity workout
* **A random number** that will generate some variety in the workout plans

The output will be the recommended workout plan.

Listing 13-2 shows the `main` function we’re going to use.

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
# fn generate_workout(intensity: u32, random_number: u32) {}
```

<span class="caption">Listing 13-2: A `main` function with hardcoded values to
simulate user input and random number generation</span>

We’ve hardcoded the variable `simulated_user_specified_value` to 10 and the
variable `simulated_random_number` to 7 for simplicity’s sake; in an actual
program we’d get the intensity number from the app frontend and we’d use the
`rand` crate to generate a random number like we did in the Guessing Game
example in Chapter 2. The `main` function calls a `generate_workout` function
with the simulated input values.

There’s the context, so let’s get to the algorithm. The `generate_workout`
function in Listing 13-3 contains the business logic of the app that we’re most
concerned with in this example. The rest of the code changes in this example
will be made to this function:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# fn simulated_expensive_calculation(num: u32) -> u32 {
#     println!("calculating slowly...");
#     thread::sleep(Duration::from_secs(2));
#     num
# }
#
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

<span class="caption">Listing 13-3: The business logic that prints the workout
plans based on the inputs and calls to the `simulated_expensive_calculation`
function</span>

The code in Listing 13-3 has multiple calls to the slow calculation function.
The first `if` block calls `simulated_expensive_calculation` twice, the `if`
inside the outer `else` doesn’t call it at all, and the code inside the
second `else` case calls it once.

The desired behavior of the `generate_workout` function is to first check if
the user wants a low intensity workout (indicated by a number less than 25) or
a high intensity workout (25 or more).

Low intensity workout plans will recommend a number of pushups and situps based
on the complex algorithm we’re simulating.

If the user wants a high intensity workout, there’s some additional logic: if
the value of the random number generated by the app happens to be 3, the app
will recommend a break and hydration. If not, the user will get a number of
minutes of running based on the complex algorithm.

The data science team has let us know that we’ll have to make some changes to
the way we call the algorithm in the future. To simplify the update when those
changes happen, we want to refactor this code so it only calls the
`simulated_expensive_calculation` function once. We also want to cut the place
where we’re currently calling the function twice unnecessarily without adding
any other calls to that function in the process. That is, we don’t want to call
it if the result isn’t needed, and we still want to call it only once.

#### Refactoring Using Functions

There are many ways we could restructure this program. First we’ll try
extracting the duplicated call to the expensive calculation function into a
variable, as shown in Listing 13-4:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# fn simulated_expensive_calculation(num: u32) -> u32 {
#     println!("calculating slowly...");
#     thread::sleep(Duration::from_secs(2));
#     num
# }
#
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
```

<span class="caption">Listing 13-4: Extracting the calls to
`simulated_expensive_calculation` to one place and storing the result in the
`expensive_result` variable</span>

This change unifies all the calls to `simulated_expensive_calculation` and
solves the problem of the first `if` block calling the function twice
unnecessarily. Unfortunately, we’re now calling this function and waiting for
the result in all cases, which includes the inner `if` block that doesn’t use
the result value at all.

We want to define code in one place in our program, but only *execute* that
code where we actually need the result. This is a use case for closures!

#### Refactoring with Closures to Store Code for Later Execution

Instead of always calling the `simulated_expensive_calculation` function before
the `if` blocks, we can define a closure and store the *closure* in a variable
rather than storing the result, as shown in Listing 13-5. We can actually move
the whole body of `simulated_expensive_calculation` within the closure we’re
introducing here:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
# expensive_closure(5);
```

<span class="caption">Listing 13-5: Defining a closure and storing it in the
`expensive_closure` variable</span>

The closure definition comes after the `=` to assign it to the variable
`expensive_closure`. To define a closure, we start with a pair of vertical
pipes (`|`), inside which we specify the parameters to the closure; this syntax
was chosen because of its similarity to closure definitions in Smalltalk and
Ruby. This closure has one parameter named `num`; if we had more than one
parameter, we would separate them with commas, like `|param1, param2|`.

After the parameters, we place curly brackets that hold the body of the
closure—these are optional if the closure body is a single expression. The end
of the closure, after the curly brackets, needs a semicolon to complete the
`let` statement. The value returned from the last line in the closure body
(`num`) will be the value returned from the closure when it’s called, since
that line doesn’t end in a semicolon; just like in function bodies.

Note that this `let` statement means `expensive_closure` contains the
*definition* of an anonymous function, not the *resulting value* of calling the
anonymous function. Recall that we’re using a closure because we want to define
the code to call at one point, store that code, and actually call it at a later
point; the code we want to call is now stored in `expensive_closure`.

Now that we have the closure defined, we can change the code in the `if` blocks
to call the closure, in order to execute the code and get the resulting value.
We call a closure like we do a function: we specify the variable name that
holds the closure definition and follow it with parentheses containing the
argument values we want to use, as shown in Listing 13-6:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
```

<span class="caption">Listing 13-6: Calling the `expensive_closure` we’ve
defined</span>

Now the expensive calculation is called in only one place, and we’re only
executing that code where we need the results.

We have, however, reintroduced one of the problems from Listing 13-3: we’re
still calling the closure twice in the first `if` block, which will call the
expensive code twice and make the user wait twice as long as they need to. We
could fix this problem by creating a variable local to that `if` block to hold
the result of calling the closure, but closures provide us with another
solution. We’ll get back to that solution in a bit; let’s first talk about why
there aren’t type annotations in the closure definition and the traits involved
with closures.

### Closure Type Inference and Annotation

Closures differ from functions defined with the `fn` keyword in a few ways. The
first is that closures don’t require you to annotate the types of the
parameters or the return value like `fn` functions do.

Type annotations are required on functions because they are part of an explicit
interface exposed to your users. Defining this interface rigidly is important
for ensuring that everyone agrees on what types of values a function uses and
returns. Closures aren’t used in an exposed interface like this, though:
they’re stored in variables and used without naming them and exposing them to
users of our library.

Additionally, closures are usually short and only relevant within a narrow
context rather than in any arbitrary scenario. Within these limited contexts,
the compiler is reliably able to infer the types of the parameters and return
type, similar to how it’s able to infer the types of most variables.

Making programmers annotate the types in these small, anonymous functions would
be annoying and largely redundant with the information the compiler already has
available.

Like variables, we can choose to add type annotations if we want to increase
explicitness and clarity at the cost of being more verbose than is strictly
necessary; annotating the types for the closure we defined in Listing 13-4
would look like the definition shown in Listing 13-7:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

<span class="caption">Listing 13-7: Adding optional type annotations of the
parameter and return value types in the closure</span>

The syntax of closures and functions looks more similar with type annotations.
Here’s a vertical comparison of the syntax for the definition of a function
that adds one to its parameter, and a closure that has the same behavior. We’ve
added some spaces here to line up the relevant parts). This illustrates how
closure syntax is similar to function syntax, except for the use of pipes and
the amount of syntax that is optional:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

The first line shows a function definition, and the second line shows a fully
annotated closure definition. The third line removes the type annotations from
the closure definition, and the fourth line removes the brackets that are
optional, since the closure body only has one expression. These are all valid
definitions that will produce the same behavior when they’re called.

Closure definitions will have one concrete type inferred for each of their
parameters and for their return value. For instance, Listing 13-8 shows the
definition of a short closure that just returns the value it receives as a
parameter.

This closure isn’t very useful except for the purposes of this example. Note
that we haven’t added any type annotations to the definition: if we then try to
call the closure twice, using a `String` as an argument the first time and an
`u32` the second time, we’ll get an error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

<span class="caption">Listing 13-8: Attempting to call a closure whose types
are inferred with two different types</span>

The compiler gives us this error:

```text
error[E0308]: mismatched types
 --> src/main.rs
  |
  | let n = example_closure(5);
  |                         ^ expected struct `std::string::String`, found
  integral variable
  |
  = note: expected type `std::string::String`
             found type `{integer}`
```

The first time we call `example_closure` with the `String` value, the compiler
infers the type of `x` and the return type of the closure to be `String`. Those
types are then locked in to the closure in `example_closure`, and we get a type
error if we try to use a different type with the same closure.

### Storing Closures Using Generic Parameters and the `Fn` Traits

Returning to our workout generation app, in Listing 13-6 we left our code still
calling the expensive calculation closure more times than it needs to. One
option to solve this issue is to save the result of the expensive closure in a
variable for reuse and use the variable instead in each place we need the
result instead of calling the closure again. This method, though, could result
in a lot of repeated code.

Fortunately, we have another solution available to us. We can create a struct
that will hold the closure and the resulting value of calling the closure. The
struct will only execute the closure if we need the resulting value, and it
will cache the resulting value so that the rest of our code doesn’t have to be
responsible for saving and reusing the result. You may know this pattern as
*memoization* or *lazy evaluation*.

In order to make a struct that holds a closure, we need to be able to specify
the type of the closure, because a struct definition needs to know the types of
each of its fields. Each closure instance has its own unique anonymous type:
that is, even if two closures have the same signature, their types are still
considered different. In order to define structs, enums, or function parameters
that use closures, we use generics and trait bounds like we discussed in
Chapter 10.

The `Fn` traits are provided by the standard library. All closures implement
one of the traits `Fn`, `FnMut`, or `FnOnce`. We’ll discuss the difference
between these traits in the next section on capturing the environment; in this
example, we can use the `Fn` trait.

We add types to the `Fn` trait bound to represent the types of the parameters
and return values the closures must have in order to match this trait bound. In
this case, our closure has a parameter of type `u32` and returns an `u32`, so
the trait bound we specify is `Fn(u32) -> u32`.

Listing 13-9 shows the definition of the `Cacher` struct that holds a closure
and an optional result value:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
```

<span class="caption">Listing 13-9: Defining a `Cacher` struct that holds a
closure in `calculation` and an optional result in `value`</span>

The `Cacher` struct has a `calculation` field of the generic type `T`. The
trait bounds on `T` specify that it’s a closure by using the `Fn` trait. Any
closure we want to store in the `calculation` field must have one `u32`
parameter (specified within the parentheses after `Fn`) and must return an
`u32` (specified after the `->`).

> Note: Functions implement all three of the `Fn` traits too. If what we want to
> do doesn’t require capturing a value from the environment, we can use a
> function rather than a closure where we need something that implements an `Fn`
> trait.

The `value` field is of type `Option<u32>`. Before we execute the closure,
`value` will be `None`. When code using a `Cacher` asks for the *result* of the
closure, the `Cacher` will execute the closure at that time and store the
result within a `Some` variant in the `value` field. Then if the code asks for
the result of the closure again, instead of executing the closure again, the
`Cacher` will return the result held in the `Some` variant.

The logic around the `value` field we’ve just described is defined in Listing
13-10:

<span class="filename">Filename: src/main.rs</span>

```rust
# struct Cacher<T>
#     where T: Fn(u32) -> u32
# {
#     calculation: T,
#     value: Option<u32>,
# }
#
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
```

<span class="caption">Listing 13-10: The caching logic of `Cacher`</span>

We want `Cacher` to manage the struct fields’ values, rather than letting the
calling code potentially change the values in these fields directly, so these
fields are private.

The `Cacher::new` function takes a generic parameter `T`, which we’ve defined
as having the same trait bound as the `Cacher` struct. Then `Cacher::new`
returns a `Cacher` instance that holds the closure specified in the
`calculation` field and a `None` value in the `value` field, since we haven’t
executed the closure yet.

When the calling code wants the result of evaluating the closure, instead of
calling the closure directly, it will call the `value` method. This method
checks to see if we already have a resulting value in `self.value` in a `Some`;
if we do, it returns the value within the `Some` without executing the closure
again.

If `self.value` is `None`, we call the closure stored in `self.calculation`,
save the result in `self.value` for future use, and return the value as well.

Listing 13-11 shows how we can use this `Cacher` struct in the
`generate_workout` function from Listing 13-6:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# struct Cacher<T>
#     where T: Fn(u32) -> u32
# {
#     calculation: T,
#     value: Option<u32>,
# }
#
# impl<T> Cacher<T>
#     where T: Fn(u32) -> u32
# {
#     fn new(calculation: T) -> Cacher<T> {
#         Cacher {
#             calculation,
#             value: None,
#         }
#     }
#
#     fn value(&mut self, arg: u32) -> u32 {
#         match self.value {
#             Some(v) => v,
#             None => {
#                 let v = (self.calculation)(arg);
#                 self.value = Some(v);
#                 v
#             },
#         }
#     }
# }
#
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
```

<span class="caption">Listing 13-11: Using `Cacher` in the `generate_workout`
function to abstract away the caching logic</span>

Instead of saving the closure in a variable directly, we save a new instance of
`Cacher` that holds the closure. Then, in each place we want the result, we
call the `value` method on the `Cacher` instance. We can call the `value`
method as many times as we want, or not call it at all, and the expensive
calculation will be run a maximum of once.

Try running this program with the `main` function from Listing 13-2. Change the
values in the `simulated_user_specified_value` and `simulated_random_number`
variables to verify that in all of the cases in the various `if` and `else`
blocks, `calculating slowly...` only shows up once and only when needed. The
`Cacher` takes care of the logic necessary to ensure we aren’t calling the
expensive calculation more than we need to, so that `generate_workout` can
focus on the business logic.

### Limitations of the `Cacher` Implementation

Caching values is a generally useful behavior that we might want to use in
other parts of our code with different closures. However, there are a few
problems with the current implementation of `Cacher` that would make reusing it
in different contexts difficult.

The first problem is a `Cacher` instance assumes it will always get the same
value for the parameter `arg` to the `value` method. That is, this test of
`Cacher` will fail:

```rust,ignore
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
```

This test creates a new `Cacher` instance with a closure that returns the value
passed into it. We call the `value` method on this `Cacher` instance with an
`arg` value of 1 and then an `arg` value of 2, and we expect that the call to
`value` with the `arg` value of 2 should return 2.

Run this with the `Cacher` implementation from Listing 13-9 and Listing 13-10
and the test will fail on the `assert_eq!` with this message:

```text
thread 'call_with_different_arg_values' panicked at 'assertion failed:
`(left == right)` (left: `1`, right: `2`)', src/main.rs
```

The problem is that the first time we called `c.value` with 1, the `Cacher`
instance saved `Some(1)` in `self.value`. After that, no matter what we pass in
to the `value` method, it will always return 1.

Try modifying `Cacher` to hold a hash map rather than a single value. The keys
of the hash map will be the `arg` values that are passed in, and the values of
the hash map will be the result of calling the closure on that key. Instead of
looking at whether `self.value` directly has a `Some` or a `None` value, the
`value` function will look up the `arg` in the hash map and return the value,
if it’s present. If it’s not present, the `Cacher` will call the closure and
save the resulting value in the hash map associated with its `arg` value.

Another problem with the current `Cacher` implementation is that it only
accepts closures that take one parameter of type `u32` and return an `u32`. We
might want to cache the results of closures that take a string slice and return
`usize` values, for example. To fix this issue, try introducing more generic
parameters to increase the flexibility of the `Cacher` functionality.

### Closures Can Capture Their Environment

In the workout generator example, we only used closures as inline anonymous
functions. Closures have an additional ability that functions don’t have,
however: they can capture their environment and access variables from the scope
in which they’re defined.

Listing 13-12 has an example of a closure stored in the variable `equal_to_x`
that uses the variable `x` from the closure’s surrounding environment:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

<span class="caption">Listing 13-12: Example of a closure that refers to a
variable in its enclosing scope</span>

Here, even though `x` is not one of the parameters of `equal_to_x`, the
`equal_to_x` closure is allowed to use the `x` variable that’s defined in the
same scope that `equal_to_x` is defined in.

We can’t do the same with functions; let’s see what happens if we try:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}
```

We get an error:

```text
error[E0434]: can't capture dynamic environment in a fn item; use the || { ... }
closure form instead
 -->
  |
4 |     fn equal_to_x(z: i32) -> bool { z == x }
  |                                          ^
```

The compiler even reminds us that this only works with closures!

When a closure captures a value from its environment, it uses memory to store
the values for use in the closure body. This use of memory is overhead that we
don’t want to pay in more common cases, where we want to execute code that
doesn’t capture its environment. Because functions are never allowed to capture
their environment, defining and using functions will never incur this overhead.

Closures can capture values from their environment in three ways, which
directly map to the three ways a function can take a parameter: taking
ownership, borrowing immutably, and borrowing mutably. These are encoded in the
three `Fn` traits as follows:

* `FnOnce` consumes the variables it captures from its enclosing scope, known
  as the closure’s *environment*. In order to consume the captured variables,
  the closure must take ownership of these variables and move them into the
  closure when it is defined. The `Once` part of the name is because the
  closure can’t take ownership of the same variables more than once, so it can
  only be called one time.
* `Fn` borrows values from the environment immutably.
* `FnMut` can change the environment since it mutably borrows values.

When we create a closure, Rust infers which to use based on how the closure
uses the values from the environment. In Listing 13-12, the `equal_to_x`
closure borrows `x` immutably (so `equal_to_x` has the `Fn` trait) since the
body of the closure only needs to read the value in `x`.

If we want to force the closure to take ownership of the values it uses in the
environment, we can use the `move` keyword before the parameter list. This is
mostly useful when passing a closure to a new thread in order to move the data
so that it’s owned by the new thread.

We’ll have more examples of `move` closures in Chapter 16 when we talk about
concurrency, but for now here’s the code from Listing 13-12 with the `move`
keyword added to the closure definition and using vectors instead of integers,
since integers can be copied rather than moved:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

This example doesn’t compile:

```text
error[E0382]: use of moved value: `x`
 --> src/main.rs:6:40
  |
4 |     let equal_to_x = move |z| z == x;
  |                      -------- value moved (into closure) here
5 |
6 |     println!("can't use x here: {:?}", x);
  |                                        ^ value used here after move
  |
  = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not
    implement the `Copy` trait
```

The `x` value is moved into the closure when the closure is defined, because we
added the `move` keyword. The closure then has ownership of `x`, and `main`
isn’t allowed to use `x` anymore in the `println!` statement. Removing
`println!` will fix this example.

Most of the time when specifying one of the `Fn` trait bounds, you can start
with `Fn` and the compiler will tell you if you need `FnMut` or `FnOnce` based
on what happens in the closure body.

To illustrate situations where closures that can capture their environment are
useful as function parameters, let’s move on to our next topic: iterators.
