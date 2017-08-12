## Closures: Anonymous Functions that can Capture their Environment

<!-- Bill's suggested we flesh out some of these subtitles, which I think we
did more with earlier chapters but we (I, included!) have got a bit lax with. I
don't think this is quite right, is there a shorter heading we could use to
capture what a closure is/is for? -->
<!-- I've attempted a more descriptive subtitle, what do you think? /Carol -->

Rust’s *closures* are anonymous functions that you can save in a variable or
pass as arguments to other functions. You can create the closure in one place,
and then call the closure to evaluate it in a different context. Unlike
functions, closures are allowed to capture values from the scope in which they
are called. We’re going to demonstrate how these features of closures allow for
code reuse and customization of behavior.

<!-- Can you say what sets closures apart from functions, explicitly, above? I
can't see it clearly enough to be confident, after one read through this
chapter. I think it would help to have the closure definition up front, to help
to let the reader know what they are looking out for in the examples. When
would you use a closure, for example, rather than using a function? And is it
the closure that's stored in the variable, or is it the result of applying the
closure to a value passed as an argument? -->
<!-- I've tried reworking the above paragraph and restructuring the examples to
be more motivating. I've also tried to make it clear throughout that storing a
closure is storing the *unevaluated* code, and then you call the closure in
order to get the result. /Carol -->

### Creating an Abstraction of Behavior Using a Closure

Let’s work on an example that will show a situation where storing a closure to
be executed at a later time is useful. We’ll talk about the syntax of closures,
type inference, and traits along the way.

The hypothetical situation is this: we’re working at a startup that’s making an
app to generate custom exercise workout plans. The backend is written in Rust,
and the algorithm that generates the workout plan takes into account many
different factors like the app user’s age, their Body Mass Index, their
preferences, their recent workouts, and an intensity number they specify. The
actual algorithm used isn’t important in this example; what’s important is that
this calculation takes a few seconds. We only want to call this algorithm if we
need to, and we only want to call it once, so that we aren’t making the user
wait more than they need to. We’re going to simulate calling this hypothetical
algorithm by calling the `simulated_expensive_calculation` function shown in
Listing 13-1 instead, which will print `calculating slowly...`, wait for two
seconds, and then return whatever number we passed in:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

<span class="caption">Listing 13-1: A function we’ll use to stand in for a
hypothetical calculation that takes about two seconds to run</span>

Next, we have a `main` function that contains the parts of the workout app that
are important for this example. This represents the code that the app would
call when a user asks for a workout plan. Because the interaction with the
app’s frontend isn’t relevant to the use of closures, we’re going to hardcode
values representing inputs to our program and print the outputs.

The inputs to the program are:

- An `intensity` number from the user, specified when they request a workout,
  so they can indicate whether they’d like a low intensity workout or a high
  intensity workout
- A random number that will generate some variety in the workout plans

The output the program prints will be the recommended workout plan.

Listing 13-2 shows the `main` function we’re going to use. We’ve hardcoded the
variable `simulated_user_specified_value` to 10 and the variable
`simulated_random_number` to 7 for simplicity’s sake; in an actual program we’d
get the intensity number from the app frontend and we’d use the `rand` crate to
generate a random number like we did in the Guessing Game example in Chapter 2.
The `main` function calls a `generate_workout` function with the simulated
input values:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
# fn generate_workout(intensity: i32, random_number: i32) {}
```

<span class="caption">Listing 13-2: A `main` function containing hardcoded
values to simulate user input and random number generation inputs to the
`generate_workout` function</span>

That’s the context of what we’re working on. The `generate_workout` function in
Listing 13-3 contains the business logic of the app that we’re most concerned
with in this example. The rest of the code changes in this example will be made
to this function:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# fn simulated_expensive_calculation(num: i32) -> i32 {
#     println!("calculating slowly...");
#     thread::sleep(Duration::from_secs(2));
#     num
# }
#
fn generate_workout(intensity: i32, random_number: i32) {
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
            )
        }
    }
}
```

<span class="caption">Listing 13-3: The business logic of the program that
prints the workout plans based on the inputs and calls to the
`simulated_expensive_calculation` function</span>

The code in Listing 13-3 has multiple calls to the slow calculation function.
The first `if` block calls `simulated_expensive_calculation` twice, the `if`
inside the outer `else` doesn’t call it at all, and the code inside the `else`
case inside the outer `else` calls it once.

<!-- Will add wingdings in libreoffice /Carol -->

The desired behavior of the `generate_workout` function is to first check if
the user wants a low intensity workout (indicated by a number less than 25) or
a high intensity workout (25 or more). Low intensity workout plans will
recommend a number of pushups and situps based on the complex algorithm we’re
simulating with the `simulated_expensive_calculation` function, which needs the
intensity number as an input.

If the user wants a high intensity workout, there’s some additional logic: if
the value of the random number generated by the app happens to be 3, the app
will recommend a break and hydration instead. If not, the user will get a high
intensity workout of a number of minutes of running that comes from the complex
algorithm.

The data science team has let us know that there are going to be some changes
to the way we have to call the algorithm. To simplify the update when those
changes happen, we would like to refactor this code to have only a single call
to the `simulated_expensive_calculation` function. We also want to get rid of
the spot where we’re currently calling the function twice unnecessarily, and
we don’t want to add any other calls to that function in the process. That is,
we don’t want to call it if we’re in the case where the result isn’t needed at
all, and we still want to call it only once in the last case.

There are many ways we could restructure this program. The way we’re going to
try first is extracting the duplicated call to the expensive calculation
function into a variable, as shown in Listing 13-4:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# fn simulated_expensive_calculation(num: i32) -> i32 {
#     println!("calculating slowly...");
#     thread::sleep(Duration::from_secs(2));
#     num
# }
#
fn generate_workout(intensity: i32, random_number: i32) {
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
            )
        }
    }
}
```

<span class="caption">Listing 13-4: Extracting the calls to
`simulated_expensive_calculation` to one place before the `if` blocks and
storing the result in the `expensive_result` variable</span>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

This change unifies all the calls to `simulated_expensive_calculation` and
solves the problem of the first `if` block calling the function twice
unnecessarily. Unfortunately, we’re now calling this function and waiting for
the result in all cases, which includes the inner `if` block that doesn’t use
the result value at all.

We want to be able to specify some code in one place in our program, but then
only execute that code if we actually need the result in some other place in
our program. This is a use case for closures!

### Closures Store Code to be Executed Later

Instead of always calling the `simulated_expensive_calculation` function before
the `if` blocks, we can define a closure and store the closure in a variable
instead of the result as shown in Listing 13-5. We can actually choose to move
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

<span class="caption">Listing 13-5: Defining a closure with the body that was
in the expensive function and store the closure in the `expensive_closure`
variable</span>

<!-- Can you elaborate on *how* to define the closure first? I've had a go here
based on what I can see but not sure it's correct. Are we saying that a closure
is function that assigned its result to a variable you can then use? -->
<!-- I've attempted to elaborate on defining a closure in the next few
paragraphs starting here, is this better? /Carol -->

The closure definition is the part after the `=` that we’re assigning to the
variable `expensive_closure`. To define a closure, we start with a pair of
vertical pipes (`|`). Inside the pipes is where we specify the parameters to
the closure; this syntax was chosen because of its similarity to closure
definitions in Smalltalk and Ruby. This closure has one parameter named `num`;
if we had more than one parameter, we would separate them with commas, like
`|param1, param2|`.

After the parameters, we put curly braces that hold the body of the closure.
The curly braces are optional if the closure body only has one line. After the
curly braces, we need a semicolon to go with the `let` statement. The value
returned from the last line in the closure body (`num`), since that line
doesn’t end in a semicolon, will be the value returned from the closure when
it’s called, just like in function bodies.

Note that this `let` statement means `expensive_closure` contains the
*definition* of an anonymous function, not the *resulting value* of calling the
anonymous function. Recall the reason we’re using a closure is because we want
to define the code to call at one point, store that code, and actually call it
at a later point; the code we want to call is now stored in `expensive_closure`.

Now that we have the closure defined, we can change the code in the `if` blocks
to call the closure in order to execute the code and get the resulting value.
Calling a closure looks very similar to calling a function; we specify the
variable name that holds the closure definition and follow it with parentheses
containing the argument values we want to use for that call as shown in Listing
13-6:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
fn generate_workout(intensity: i32, random_number: i32) {
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
            )
        }
    }
}
```

<span class="caption">Listing 13-6: Calling the `expensive_closure` we’ve
defined</span>

Now we’ve achieved the goal of unifying where the expensive calculation is
called to one place, and we’re only executing that code where we need the
results. However, we’ve reintroduced one of the problems from Listing 13-3:
we’re still calling the closure twice in the first `if` block, which will call
the expensive code twice and make the user wait twice as long as they need to.
We could fix this problem by creating a variable local to that `if` block to
hold the result of calling the closure, but there’s another solution we can use
since we have a closure. We’ll get back to that solution in a bit; let’s first
talk about why there aren’t type annotations in the closure definition and the
traits involved with closures.

### Closure Type Inference and Annotation

Closures differ from functions defined with the `fn` keyword in a few
ways. The first is that closures don’t require you to annotate the types of the
parameters or the return value like `fn` functions do.

<!-- I've suggested moving this next paragraph up from below, I found this
section difficult to follow with this next paragraph -->

Type annotations are required on functions because they are part of an
explicit interface exposed to your users. Defining this interface rigidly is
important for ensuring that everyone agrees on what types of values a function
uses and returns. Closures aren’t used in an exposed interface like this,
though: they’re stored in variables and used without naming them and exposing
them to be invoked by users of our library.

Additionally, closures are usually short and only relevant within a narrow
context rather than in any arbitrary scenario. Within these limited contexts,
the compiler is reliably able to infer the types of the parameters and return
type similarly to how it’s able to infer the types of most variables. Being
forced to annotate the types in these small, anonymous functions would be
annoying and largely redundant with the information the compiler already has
available.

<!--Can you expand above on what you mean by "stored in bindings and called
directly"? Do you mean stored in a variable? I'm struggling to visualize how
closures are used, and what the important difference is between them and
functions. I think a clearer definition of what they are, what they do, and
what they're used for at the start of the closures section would help clear
this up -->
<!-- Yes, sorry, in Rust terminology "binding" is mostly synonymous to
"variable", but when we started working on the book we decided to be consistent
and more like what people are used to by referring to the concept as "variable"
throughout, but we missed this spot. /Carol -->

Like variables, we can choose to add type annotations if we want to increase
explicitness and clarity in exchange for being more verbose than is strictly
necessary; annotating the types for the closure we defined in Listing 13-4
would look like the definition shown here in Listing 13-7:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
let expensive_closure = |num: i32| -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

<span class="caption">Listing 13-7: Adding optional type annotations of the
parameter and return value types in the closure</span>

<!-- Why might you want to, if you don't need to? In a particular situation? -->
<!-- I've added an explanation /Carol -->

<!-- Below -- Am I right in assuming the closures below are doing the same
thing as the functions? -->
<!-- Yes /Carol -->

The syntax of closures and functions looks more similar with type annotations.
Here’s a vertical comparison of the syntax for the definition of a function
that adds one to its parameter, and a closure that has the same behavior. We’ve
added some spaces here to line up the relevant parts). This illustrates how
closure syntax is similar to function syntax except for the use of pipes rather
than parentheses and the amount of syntax that is optional:

<!-- Prod: can you align this as shown in the text? -->
<!-- I'm confused, does this note mean that production *won't* be aligning all
of our other code examples as shown in the text? That's concerning to me, we're
trying to illustrate idiomatic Rust style in all the code examples, so our
alignment is always intentional... /Carol -->

```rust,ignore
fn  add_one_v1   (x: i32) -> i32 { x + 1 }
let add_one_v2 = |x: i32| -> i32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

<!-- Can you point out where we're looking at here, where the important
differences lie? -->
<!-- The importance isn't the difference but the similarity... I've tried to
clarify /Carol -->

<!-- Will add wingdings and ghosting in libreoffice /Carol -->

The first line shows a function definition, and the second line shows a fully
annotated closure definition. The third line removes the type annotations from
the closure definition, and the fourth line removes the braces that are
optional since the closure body only has one line. These are all valid
definitions that will produce the same behavior when they’re called.

<!--Below--I'm not sure I'm following, is the i8 type being inferred? It seems
like we're annotating it. -->
<!-- The types in the function definitions are being inferred, but since Rust's
variable types for numbers defaults to `i32`, in order to illustrate our point
here we're forcing the type of the *variable* to be `i8` by annotating it in
the *variable* declaration. I've changed the example to hopefully be less
confusing and convey our point better. /Carol -->

Closure definitions will have one concrete type inferred for each of their
parameters and for their return value. For instance, Listing 13-8 shows the
definition of a short closure that just returns the value it gets as a
parameter. This closure isn’t very useful except for the purposes of this
example. Note that we haven’t added any type annotations to the definition: if
we then try to call the closure twice, using a `String` as an argument the
first time and an `i32` the second time, we’ll get an error:

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

<!-- Will add wingdings in libreoffice /Carol -->

The first time we call `example_closure` with the `String` value, the compiler
infers the type of `x` and the return type of the closure to be `String`. Those
types are then locked in to the closure in `example_closure`, and we get a type
error if we try to use a different type with the same closure.

### Using Closures with Generic Parameters and the `Fn` Traits

Returning to our workout generation app, in Listing 13-6 we left our code still
calling the expensive calculation closure more times than it needs to. In each
place throughout our code, if we need the results of the expensive closure more
than once, we could save the result in a variable for reuse and use the
variable instead of calling the closure again. This could be a lot of repeated
code saving the results in a variety of places.

However, because we have a closure for the expensive calculation, we have
another solution available to us. We can create a struct that will hold the
closure and the resulting value of calling the closure. The struct will only
execute the closure if we need the resulting value, and it will cache the
resulting value so that the rest of our code doesn’t have to be responsible for
saving and reusing the result. You may know this pattern as *memoization* or
*lazy evaluation*.

In order to make a struct that holds a closure, we need to be able to specify
the type of the closure. Each closure instance has its own unique anonymous
type: that is, even if two closures have the same signature, their types are
still considered to be different. In order to define structs, enums, or
function parameters that use closures, we use generics and trait bounds like we
discussed in Chapter 10.

<!-- So Fn is a trait built into the language, is that right? I wasn't sure if
it was just a placeholder here -->
<!-- Fn is provided by the standard library; I've clarified here. /Carol -->

The `Fn` traits are provided by the standard library. All closures implement
one of the traits `Fn`, `FnMut`, or `FnOnce`. We’ll discuss the difference
between these traits in the next section on capturing the environment; in this
example, we can use the `Fn` trait.

We add types to the `Fn` trait bound to represent the types of the parameters
and return values that the closures must have in order to match this trait
bound. In this case, our closure has a parameter of type `i32` and returns an
`i32`, so the trait bound we specify is `Fn(i32) -> i32`.

Listing 13-9 shows the definition of the `Cacher` struct that holds a closure
and an optional result value:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Cacher<T>
    where T: Fn(i32) -> i32
{
    calculation: T,
    value: Option<i32>,
}
```

<span class="caption">Listing 13-9: Defining a `Cacher` struct that holds a
closure in `calculation` and an optional result in `value`</span>

The `Cacher` struct has a `calculation` field of the generic type `T`. The
trait bounds on `T` specify that `T` is a closure by using the `Fn` trait. Any
closure we want to store in the `calculation` field of a `Cacher` instance must
have one `i32` parameter (specified within the parentheses after `Fn`) and must
return an `i32` (specified after the `->`).

The `value` field is of type `Option<i32>`. Before we execute the closure,
`value` will be `None`. If the code using a `Cacher` asks for the result of the
closure, we’ll execute the closure at that time and store the result within a
`Some` variant in the `value` field. Then if the code asks for the result of
the closure again, instead of executing the closure again, we’ll return the
result that we’re holding in the `Some` variant.

The logic around the `value` field that we’ve just described is defined in
Listing 13-10:

<span class="filename">Filename: src/main.rs</span>

```rust
# struct Cacher<T>
#     where T: Fn(i32) -> i32
# {
#     calculation: T,
#     value: Option<i32>,
# }
#
impl<T> Cacher<T>
    where T: Fn(i32) -> i32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
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

<!-- Liz: the `new` function is using the "struct init shorthand" by just
saying `calculation` instead of `calculation: calculation`, since the parameter
matches the struct field name. This was recently added to stable Rust and we've
added an introduction of it in Chapter 5. Just adding an explanation here for
you in case you read this chapter before the changes we've made to Chapter 5!
/Carol -->

<span class="caption">Listing 13-10: Implementations on `Cacher` of an
associated function named `new` and a method named `value` that manage the
caching logic</span>

The fields on the `Cacher` struct are private since we want `Cacher` to manage
their values rather than letting the calling code potentially change the values
in these fields directly. The `Cacher::new` function takes a generic parameter
`T`, which we’ve defined in the context of the `impl` block to have the same
trait bound as the `Cacher` struct. `Cacher::new` returns a `Cacher` instance
that holds the closure specified in the `calculation` field and a `None` value
in the `value` field, since we haven’t executed the closure yet.

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
#     where T: Fn(i32) -> i32
# {
#     calculation: T,
#     value: Option<i32>,
# }
#
# impl<T> Cacher<T>
#     where T: Fn(i32) -> i32
# {
#     fn new(calculation: T) -> Cacher<T> {
#         Cacher {
#             calculation,
#             value: None,
#         }
#     }
#
#     fn value(&mut self, arg: i32) -> i32 {
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
fn generate_workout(intensity: i32, random_number: i32) {
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
            )
        }
    }
}
```

<span class="caption">Listing 13-11: Using `Cacher` in the `generate_workout`
function to abstract away the caching logic</span>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Instead of saving the closure in a variable directly, we save a new instance of
`Cacher` that holds the closure. Then, in each place we want the result, we
call the `value` method on the `Cacher` instance. We can call the `value`
method as many times as we want, or not call it at all, and the expensive
calculation will be run a maximum of once. Try running this program with the
`main` function from Listing 13-2, and change the values in the
`simulated_user_specified_value` and `simulated_random_number` variables to
verify that in all of the cases in the various `if` and `else` blocks,
`calculating slowly...` printed by the closure only shows up once and only when
needed.

The `Cacher` takes care of the logic necessary to ensure we aren’t calling the
expensive calculation more than we need to, so that `generate_workout` can
focus on the business logic. Caching values is a more generally useful behavior
that we might want to use in other parts of our code with other closures as
well. However, there are a few problems with the current implementation of
`Cacher` that would make reusing it in different contexts difficult.

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
passed into it. We call the `value` method on this `Cacher` instance with
an `arg` value of 1 and then an `arg` value of 2, and we expect that the call
to `value` with the `arg` value of 2 returns 2.

Run this with the `Cacher` implementation from Listing 13-9 and Listing 13-10
and the test will fail on the `assert_eq!` with this message:

```text
thread 'call_with_different_arg_values' panicked at 'assertion failed:
`(left == right)` (left: `1`, right: `2`)', src/main.rs
```

The problem is that the first time we called `c.value` with 1, the `Cacher`
instance saved `Some(1)` in `self.value`. After that, no matter what we pass
in to the `value` method, it will always return 1.

Try modifying `Cacher` to hold a hash map rather than a single value. The keys
of the hash map will be the `arg` values that are passed in, and the values of
the hash map will be the result of calling the closure on that key. Instead of
looking at whether `self.value` directly has a `Some` or a `None` value, the
`value` function will look up the `arg` in the hash map and return the value if
it’s present. If it’s not present, the `Cacher` will call the closure and save
the resulting value in the hash map associated with its `arg` value.

Another problem with the current `Cacher` implementation that restricts its use
is that it only accepts closures that take one parameter of type `i32` and
return an `i32`. We might want to be able to cache the results of closures that
take a string slice as an argument and return `usize` values, for example. Try
introducing more generic parameters to increase the flexibility of the `Cacher`
functionality.

### Closures Can Capture Their Environment

In the workout generator example, we only used closures as inline anonymous
functions. Closures have an additional ability we can use that functions don’t
have, however: they can capture their environment and access variables from the
scope in which they’re defined.

<!-- To clarify, by enclosing scope, do you mean the scope that the closure is
inside? Can you expand on that?-->
<!-- Yes, I've tried here to clarify that it's the scope in which the closure
is defined /Carol -->

Listing 13-12 has an example of a closure stored in the variable `equal_to_x`
that uses the variable `x` from the closure’s surrounding environment:

<!-- To clarify how we talk about a closure, does the closure include the
variable name, or are we referring to the closure as the functionality that is
on the right side of the = and so not including to variable name? I thought it
was the former, but it seems like the latter above. If it's the former, would
"an example of a closure with the variable `equal_to_x`" make more sense? -->
<!-- No, the closure does not include the variable name in which it's stored;
storing a closure in a variable is optional. It should always be the latter,
and I hope the reworking of the example used throughout the closure section
and making the wording consistent has cleared this confusion up by this point.
/Carol -->

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

<!-- So *why* is this allowed with closures and not functions, what about
closures makes this safe? -->
<!-- It's not really about safety; capturing the environment is the defining
functionality a closure has. The reason functions *don't* capture their
environment is mostly around storage overhead; I've added an explanation of
that aspect. Can you elaborate on what led to the conclusion that allowing
captures with functions wouldn't be safe and what you mean by "safe" here?
/Carol -->

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

When a closure captures a value from its environment, the closure uses memory
to store the values for use in the closure body. This use of memory is overhead
that we don’t want to pay for in the more common case where we want to execute
code that doesn’t capture its environment. Because functions are never allowed
to capture their environment, defining and using functions will never incur
this overhead.

<!-- Why didn't this work, is there a reason ingrained in the language? Or is
that not really relevant? -->
<!-- I've added an explanation /Carol -->

Closures can capture values from their environment in three ways, which
directly map to the three ways a function can take a parameter: taking
ownership, borrowing immutably, and borrowing mutably. These ways of capturing
values are encoded in the three `Fn` traits as follows:

* `FnOnce` consumes the variables it captures from its enclosing scope (the
  enclosing scope is called the closure’s *environment*). In order to consume
  the captured variables, the closure must therefore take ownership of these
  variables and moves them into the closure when the closure is defined. The
  `Once` part of the name is because the closure can’t take ownership of the
  same variables more than once, so it can only be called one time.
* `Fn` borrows values from the environment immutably.
* `FnMut` can change the environment since it mutably borrows values.

When we create a closure, Rust infers how we want to reference the environment
based on how the closure uses the values from the environment. In Listing
13-12, the `equal_to_x` closure borrows `x` immutably (so `equal_to_x` has the
`Fn` trait) since the body of the closure only needs to read the value in `x`.

If we want to force the closure to take ownership of the values it uses in the
environment, we can use the `move` keyword before the parameter list. This is
mostly useful when passing a closure to a new thread in order to move the data
to be owned by the new thread. We’ll have more examples of `move` closures in
Chapter 16 when we talk about concurrency, but for now here’s the code from
Listing 13-12 with the `move` keyword added to the closure definition and using
vectors instead of integers, since integers can be copied rather than moved:

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

The `x` value is moved into the closure when the closure is defined because of
the `move` keyword. The closure then has ownership of `x`, and `main` isn’t
allowed to use `x` anymore. Removing the `println!` will fix this example.

Most of the time when specifying one of the `Fn` trait bounds, you can start
with `Fn` and the compiler will tell you if you need `FnMut` or `FnOnce` based
on what happens in the closure body.

To illustrate situations where closures that can capture their environment are
useful as function parameters, let’s move on to our next topic: iterators.
