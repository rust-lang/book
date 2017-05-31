
[TOC]

# Functional Language features in Rust: Iterators and Closures

<!-- Are closures unique to Rust? -->
<!-- No, they're from functional languages, which is why they're discussed in
this chapter. /Carol -->

Rust's design has taken inspiration from a lot of existing languages and
techniques, and one significant influence is *functional programming*, where
functions are values that can be used as arguments or return values to other
functions, assigned to variables, and so forth. We won't debate here the issue
of what, exactly, functional programming is or is not, but will instead show
off some features of Rust that are similar to features in many languages often
referred to as functional.

More specifically, we're going to cover:

* *Closures*: a function-like construct you can store in a variable.
* *Iterators*: a way of processing a series of elements.
* How to use these features to improve on the XXX project from the last chapter.
* The performance of these features. Spoiler alert: they're faster than you
  might think!

There are other Rust feature influenced by the functional style, like pattern
matching, enums, and many others feature, that we've already covered and will
cover. Mastering closures and iterators is an important part of writing
idiomatic, fast Rust code, so we'll focus on that here.

## Saving Functions as Closures for Later Execution

<!-- Bill's suggested we flesh out some of these subtitles, which I think we
did more with earlier chapters but we (I, included!) have got a bit lax with. I
don't think this is quite right, is there a shorter heading we could use to
capture what a closure is/is for? -->

Rust's *closures* are functions that you can save in a variable or pass as
arguments to other functions. Then you can execute the closure at a later time
in a different context. This is another way to allow for code reuse and
customization of behavior.

<!-- Can you say what sets closures apart from functions, explicitly, above? I
can't see it clearly enough to be confident, after one read through this
chapter. I think it would help to have the closure definition up front, to help
to let the reader know what they are looking out for in the examples. When
would you use a closure, for example, rather than using a function? And is it
the closure that's stored in the variable, or is it the result of applying the
closure to a value passed as an argument? -->

### Defining a Single-Expression Closure

Let's have a look at the syntax for defining closures before returning to
defining what they are. Listing 13-1 shows a small closure whose definition is
assigned to the variable `add_one`. We can then use this variable with an
argument to call the closure on the value we pass as a parameter:

<!-- I'm trying to lay out more clearly how this works, can you help out with
the above paragraph? -->

Filename: src/main.rs

```rust
fn main() {
    let add_one = |x| x + 1;

    let five = add_one(4);

    assert_eq!(5, five);
}
```

<!-- What does this return, a TRUE value? -->

Listing 13-1: A closure that takes a number as a parameter and adds one to it,
assigned to the variable `add_one`

<!--assigned to the variable, or "then assigns the outcome to the variable"? I
feel like this is a distinction we need to make early on -->

To define the closure we first create a variable called `add_one` using the
`let` statement, then we add the functionality on the right side of the `=`=
operator.

<!-- Can you elaborate on *how* to define the closure first? I've had a go here
based on what I can see but not sure it's correct. Are we saying that a closure
is function that assigned its result to a variable you can then use? -->

Here the closure takes one parameter named `x` and adds the value one to it,
storing the result in the variable we name, `five`. Parameters to closures must
go between two vertical pipes (`|`). We use the `assert_eq!` command to check,
here, that passing a `4` to the closure will result in `5`, as we would expect.

<!--Why do they go between pipes? I've added a little more explanation here,
since this is the first instance we are seeing, can you check and change/add to
it? (also, I've assumed it its the *result* we store in the variable `five`,
can you confirm?-->

### Defining a Multi-Expression Closure

Listing 13-1 showed a minimal closure with only one expression as its body.
Listing 13-2 defines a closure with two parameters and multiple expressions to
demonstrate a bit more complexity:

Filename: src/main.rs

```rust
fn main() {
    let calculate = |a, b| {
        let mut result = a * 2;

        result += b;

        result
    };

    assert_eq!(7, calculate(2, 3)); // 2 * 2 + 3 == 7
    assert_eq!(13, calculate(4, 5)); // 4 * 2 + 5 == 13
}
```

Listing 13-2: A more comples closure with two parameters and multiple expressions

This closure multiplies the first parameter, `a`, by two, and then .... We use
curly brackets to define a closure body with more than one expression.

<!-- A few words on what the program aims to do? I'm still not sure at this
point what makes a closure different to a function, is it in the way it's
called? Can you talk this through a little more-->

### Closure Type Inference and Annotation

Closure are different to functions we would define with the `fn` keyword in a
few ways. The first is that, unlike `fn` functions, closures done required you
to annotate the types of the parameters or the values returned.

<!-- I've suggested moving this next paragraph up from below, I found this
section difficult to follow with this next paragraph -->

This is because functions are part of an explicit interface exposed to your
users, so defining this interface rigidly is important for ensuring that
everyone agrees on what types of values a function uses and returns. Closures
aren't used in an exposed interface like this, though: they're stored in
bindings and called directly. Being forced to annotate the types would be a
significant ergonomic loss for little advantage.

<!--Can you expand above on what you mean by "stored in bindings and called
directly"? Do you mean stored in a variable? I'm struggling to visualize how
closures are used, and what the important difference is between them and
functions. I think a clearer definition of what they are, what they do, and
what they're used for at the start of the closures section would help clear
this up -->

We can choose to add type annotations if we want;

<!-- Why might you want to, if you don't need to? In a particular situation? -->

Listing 13-3 shows the closure from Listing 13-1 with annotations for the
parameter and return value types:

Filename: src/main.rs

```rust
fn main() {
    let add_one = |x: i32| -> i32 { x + 1 };

    assert_eq!(2, add_one(1));
}
```

Listing 13-3: A closure definition with optional parameter and return value
type annotations

The syntax of closures and functions looks more similar with type annotations.

<!-- Below -- Am I right in assuming the closures below are doing the same
thing as the functions? -->

Here's a side-by-side comparison of the syntax for function definitions and the
syntax for closure definitions that all perform the same task of adding one to
the parameter (we've added some spaces here to line up the relevant parts):

<!-- Prod: can you align this as shown in the text? -->

```rust,ignore
fn  add_one_v1   (x: i32) -> i32 { x + 1 }  // a function
let add_one_v2 = |x: i32| -> i32 { x + 1 }; // the full syntax for a closure
let add_one_v3 = |x|             { x + 1 }; // a closure eliding types
let add_one_v4 = |x|               x + 1  ; // without braces
```

<!-- Can you point out where we're looking at here, where the important
differences lie? -->

<!--Below--I'm not sure I'm following, is the i8 type being inferred? It seems
like we're annotating it. -->

Closure definitions do have one type inferred for each of their parameters and
for their return value. For instance, if we call the closure from Listing 13-1
using an `i8`, we'll get an error if we then try to call the same closure with
an `i32`:

Filename: src/main.rs

```rust,ignore
let add_one = |x| x + 1;

let five = add_one(4i8);
assert_eq!(5i8, five);

let three = add_one(2i32);
```

The compiler gives us this error:

```text
error[E0308]: mismatched types
 -->
  |
7 | let three = add_one(2i32);
  |                     ^^^^ expected i8, found i32
```

<!-- why does the fact that they are called directly mean they can be reliably
inferred? I think I'm unusre on what you mean by called directly -- that the
closure is immediately given the parameter? -->

Since they're called directly, closure types can be inferred reliably, and it
would be tedious if we were required to annotate their types.

### Closures Can Reference Their Environment

The second difference between closures and functions is that closures possess
an *environment*. You've learned that functions can only use variables that are
in scope, either by being `const` or being declared as parameters. Closures can
do more: they're allowed access to variables from their enclosing scope.

<!-- To clairfy, by enclosing scope, do you mean the scope that the closure is
inside? Can you expand on that?-->

Listing 13-4 has an example of a closure in the variable `equal_to_x` that uses
the variable `x` from the closure's surrounding environment:

<!-- To clarify how we talk about a closure, does the closure include the
variable name, or are we referring to the closure as the functionality that is
on the right side of the = and so not including to variable name? I thought it
was the former, but it seems like the latter above. If it's the former, would
"an example of a closure with the variable `equal_to_x`" make more sense? -->

Filename: src/main.rs

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

Listing 13-4: Example of a closure that refers to a variable in its enclosing
scope

Here, even though `x` is not one of the parameters of `equal_to_x`, the
`equal_to_x` closure is allowed to use the `x` variable that's defined in the
same scope that `equal_to_x` is defined in.

<!-- So *why* is this allowed with closures and not functions, what about
closures makes this safe? -->

We can't do the same with functions; let's see what happens if we try:

Filename: src/main.rs

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

<!-- Why didn't this work, is there a reason ingrained in the language? Or is
that not really relevant? -->

Creating closures that capture values from their environment is mostly used in
the context of starting new threads. We'll show some more examples and explain
more detail about this feature of closures in Chapter 16 when we talk about
concurrency.

### Closures as Function Parameters Using the `Fn` Traits

Closures can do more than bind to variables: We can also define functions that
have closures as parameters by using the `Fn` traits.

<!-- Why bind use a closure as a parameter, what is this useful for? -->

Here's an example of a function named `call_with_one` whose signature has a
closure as a parameter:

```rust
fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32 {

    some_closure(1)
}

let answer = call_with_one(|x| x + 2);

assert_eq!(3, answer);
```

<!-- Ah, so here we refer to just the expression on the right of the = as the
closure. Flagging this to make sure it's consistent -->

We pass the closure `|x| x + 2` to the variable `call_with_one`, and
`call_with_one` calls that closure with `1` as an argument. The return value of
the call to `some_closure` is then returned from `call_with_one`.

<!-- I had to read this a few times, maybe we could use a different variable
name? Too many "call"s! -->

The signature of `call_with_one` is using the `where` syntax (discussed in the
Traits section of Chapter 10) to define the `some_closure` parameter that has
the generic type `F` as having the trait bounds `Fn(i32) -> i32`.

<!-- So Fn is a trait built into the language, is that right? I wasn't sure if
it was just a placeholder here -->

`Fn` is a trait that represents a closure, and we can add types to the `Fn`
trait to represent a specific type of closure. In this case, our closure has a
parameter of type `i32` and returns an `i32`, so the generic bound we specify
is `Fn(i32) -> i32`.

#### Closures with Generics and Trait Bounds

<!-- another placeholder heading --- I'm not convinced that we need one here,
but it seems like its own point and information, what do you think? -->

When specifying a function signature that contains a closure you always need to
use generics and trait bounds--- this is because each closure has a unique
type, meaning we can't give the type of a closure directly.

There are three trait bounds for specifying closures: `Fn`, `FnMut`, and
`FnOnce`. These continue the patterns of threes we've seen elsewhere in Rust:
borrowing, borrowing mutably, and ownership, respectively. Using `Fn` specifies
that the closure used may only borrow values in its environment. To specify a
closure that mutates the environment, use `FnMut`, and if the closure needs to
take ownership of the environment, `FnOnce`. Most of the time, you can start
with `Fn`, and the compiler will tell you if you need `FnMut` or `FnOnce` based
on what happens when the function calls the closure.

To illustrate a situation where it's useful for a function to have a closure as
a parameter, let's move on to our next topic: iterators.

## Processing a Seires with Iterators

<!-- From reading on, it seems like an iterator is useless without the methods
we use it with --- I think this is an important point to make early, I did find
it difficult to know what an iterator actually was throughout, and how it
depends on these other methods. Can you add something to this effect? -->

An iterator is a pattern that allows you to perform some task on a sequence of
items in turn, as the name suggests. For example, the code in Listing 13-5 uses
an iterator on a vector to add one to each number in that vector:

```rust
let v1 = vec![1, 2, 3];

let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, [2, 3, 4]);
```

Listing 13-5: Using an iterator with the `map` and `collect` methods to add one to each number
in a vector

<!-- Will add wingdings in libreoffice /Carol -->

We use the `iter` method on a vector to produce an *iterator* from the vector.
Next, we call the `map` method on the iterator, which processes each element:
in this case, we've passed a closure to `map` that adds one to every element
`x` in the vector. Using `map` is one of the most basic ways to interact with
an iterator, as processing each element in turn is very useful!

<!-- I'm not clear from this last sentence which part is iterating through each
element, iter or map? What is map actually doing?-->

Finally, the `collect` method consumes the iterator and puts the elements into
a new data structure that we specify; here, that's the vector `v2`. Here we
specify `v2` as the type `Vec<i32>`, so `collect` will create a new vector out
of the `i32` values.

Methods like `map` that we use on iterators are sometimes called *iterator
adaptors* because they take one iterator and produce a new iterator. That is,
`map` builds on top of our previous iterator and produces another iterator by
calling the closure it's passed to create the new sequence of values.

<!--Ah, I'm afraid I completely failed to follow this. What is the second
iterator for? I'm still not clear on what map does, can you expand on this? It
seems crucial to using iterators. Map applies the iterator to each element,
which applies the closure?

Also, to generalize this discussion a bit, would you ever use iter without map?
-->

As a recap, this line of code does the following:

<!-- Will add wingdings in libreoffice /Carol -->

1. Creates an iterator from the vector.
2. Uses the `map` adaptor with a closure argument cycle through each element and
   add one to each.
3. Uses the `collect` adaptor to consume the iterator and make a new vector.

That's how we end up with `[2, 3, 4]`, which we check with `assert_eq!`. As you
can see, closures are a very important part of using iterators: they provide a
way of customizing the behavior of an iterator adaptor like `map`.

### Iterators are Lazy

You may have noticed some unusual wording in the previous section: we said that
`map` *adapts* an iterator, but `collect` *consumes* one. That was intentional.

<!-- Can you explain what it is you mean by "consumes" an iterator here? It
doesn't look like we do in this section, I think that's important to lay that
out clearly -->

By themselves, iterators won't do anything; they're lazy, and need methods to
do any work. If we write the code for Listing 13-5 without calling `collect`,
Rust will warn us that nothing is happening with the outcome:

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1); // without collect
```

It will compile, but it will give us a warning:

```text
warning: unused result which must be used: iterator adaptors are lazy and do
nothing unless consumed, #[warn(unused_must_use)] on by default
 --> src/main.rs:4:1
  |
4 | v1.iter().map(|x| x + 1); // without collect
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
```

<!-- Reiterating the need for a clear definition of an iterator here--it seems
like an item that's used in iteration rather than something that performs the
process of iteration itself, is that right? Like a counter passed from element
to element? Can we define this at the begin of the iterator section? -->

We get this warning because iterator adaptors won't start actually doing the
processing on their own, they need some other method that causes the iterator
chain to evaluate. We call those *consuming adaptors*, and `collect` is one of
them.

<!-- This next paragraph doesn't give much away to me I'm afraid, not being
clear what we mean by *consume* at this point. Is a consuming adaptor like a
catalyst? -->

So how do we tell which iterator methods consume the iterator? And what
adaptors are available? For that, let's look at the `Iterator` trait.

### The `Iterator` trait

Iterators all implement a trait named `Iterator` that is defined in the
standard library. The definition of the trait looks like this:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

You'll notice some new syntax that we haven't covered here yet: `type Item` and
`Self::Item`, which are defining an *associated type* with this trait. We'll
talk about associated types in depth in Chapter XX, but for now, all you need
to know is that this code says the `Iterator` trait requires that you also
define an `Item` type, and this `Item` type is used in the return type of the
`next` method. In other words, the `Item` type will be the type of element
that's returned from the iterator.

<!-- So it seems like we are creating a program with an iterator inside, is
that right? I assumed the whole thing we were making was an iterator at first,
which lead to a few confusions, can you lay it out up front? -->

#### Heading -- An Iterator Trait Example: Counter

As an example of how this works, let's make an iterator named `Counter` that
will count from `1` to `5` using the `Iterator` trait. First, we create a
struct to hold the current state of the iterator:

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

Listing 13-x:

<!-- Could you add a caption here? I think that'll help the reader keep track
of what they're working on. Can you also just sum up in a line what this code
has accomplished so far? I moved this down from above the code, if this will
do? -->

We create a struct called `Counter` that is one field named `count` and holds a
`u32`. We also define a `new` method, which isn't strictly necessary.

<!-- Why define the new method, if it isn't necessary? Or is that what this
next line is telling us? -->

We want our `Counter` to go from one to five, so we want it to always start out
holding a zero.

<!-- So does this code just initialize it with 0? Is that jat { count: 0 }
does?-->

Next, we're going to implement the `Iterator` trait for our `Counter` type by
defining the body of the `next` method. We want our iterator to add one to the
current state. This is why we initialized `count` to 0: we want our iterator to
return one first. If the value of `count` is less than six, it will return the
current value, but if `count` is six or higher, our iterator will return
`None`, as shown in Listing 13-6:

```rust
impl Iterator for Counter {
    // Our iterator will produce u32s
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // increment our count. This is why we started at zero.
        self.count += 1;

        // check to see if we've finished counting or not.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

Listing 13-6: Implementing the `Iterator` trait on our `Counter` struct

<!-- I will add wingdings in libreoffice /Carol -->

<!--`type Item = u32` -->

We set the associated `Item` type for our iterator as `u32` <!-- wingding-->,
meaning the iterator will return `u32`s. Again, don't worry about associated
types yet, we'll be covering them in Chapter XX.

<!-- Hm, I'm not clear on what we mean by the main interface into an iterator,
though that may be beause I'm not a programmer, are we safe assuming the reader
will know? -->

The `next` method is the main interface into an iterator and returns an
`Option`. If the option is `Some(value)`, the iterator has returned another
value. If the option is `None`, iteration has finished and we have all our
values. We place the calculation our iterator needs to do inside this `next`
method. In this case, it adds one to the current value, then check to see if
the current value is still below six. If it is, we return `Some(self.count)` to
produce the next value. If it's at six or more, iteration is over, so it
returns `None`.

#### Heading?

The iterator trait specifies that when an iterator returns `None`, iteration is
finished, but it doesn't dictate the iterator's behavior when the `next` method
is called again after a `None` value is returned.

<!-- why would the next method be called after the none value, doesn't the none
value stop the iteration? This next bit seems like an error, we dont intend for
this next to be calles after none, is that right? Can you make that clear too?
-->

In this case every time we call `next` after getting the first `None` value, a
`None` value will still be returned, but the internal `count` field will
continue to be incremented by one each time. If we call `next` as many times as
the maximum value a `u32` value can hold, `count` will overflow, which will
cause a `panic!` in debug mode and a wrap in release mode. Other iterator
implementations choose to start iterating again if this happens.

<!-- Wouldn't you always need to be sure of this? If not, when do you need to
do this, use the fuse method? You may be able to tell, I'm a little confused
about what we are doing/intend to do here! -->

If you need your iterator to always return `None` on subsequent calls to the
`next` method after the first `None` value is returned, you can use the `fuse`
method to create an iterator with that characteristic out of any other iterator.

<!-- what is the fuse method, can you give that a direct explanaton/definition?
How do you use it to do this? So we're creating a new iterator with this
characteristic, rather than changing the original iterator to take on this
characteristic? -->

<!-- Separately, I wonder if this is a box, as it doesn't seem directly
relevant to this disucssion and kind of interrupts the flow here -->

Once we've implemented the `Iterator` trait, we have an iterator! We can use
the iterator functionality that our `Counter` struct now has by calling the
`next` method on it repeatedly:

```rust,ignore
let mut counter = Counter::new();

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);

let x = counter.next();
println!("{:?}", x);
```

This will print `Some(1)` through `Some(5)` and then `None`, each on their own
line.

<!-- So if I have this right, the first line creates a new Counter called
counter, and the rest of them merely call counter with next, store it in x, and
then print x? And we have to do that 5 times to get the 1-5 count? Phew, could
you wrap that up if indeed it is correct!) and sum up here? -->

#### All Sorts of `Iterator` Adaptors

In Listing 13-5, we called methods like `map` and `collect` on our iterators.
In Listing 13-6, however, we only implemented the `next` method on our
`Counter`. How do we get methods like `map` and `collect` on our `Counter`?

<!-- So we can't just use these methods anyway? It seems like we did earlier,
but here we have to use next first, before we cam access these methods? -->

A number of the useful methods defined on the `Iterator` trait come with
default implementations that call the `next` method. The `next` method,
however, is the only method of the `Iterator` trait that does not have a
default implementation.

<!-- below: once you've done what, defined a default implementation? Only then
can you use other adapters, is that what we're saying? And I'm still not clear
on what an adapter does/means, as opposed to a method, or consumer, at this
point. -->

once you've done that, you get all of the other `Iterator` adaptors for free.
There are a lot of them!

For example, say for some reason we wanted to apply the following to our
program: take the first five values produced by an instance of `Counter`, pair
those values with values produced by another `Counter` instance but skip the
first value this latter instance produces, multiply each pair together, keep
only those results that are divisible by three, and add all the resulting
values together. The code for this is surprisingly simple:

```rust,ignore
let sum: u32 = Counter::new().take(5)
                             .zip(Counter::new().skip(1))
                             .map(|(a, b)| a * b)
                             .filter(|x| x % 3 == 0)
                             .sum();
assert_eq!(48, sum);
```

All of these method calls are possible because we implemented the `Iterator`
trait by specifying how the `next` method works. Use the standard library
documentation to find more useful methods that will come in handy when you're
working with iterators.

<!-- Is there somewhere they can learn about all the methods and what they do,
how to use them? This seems like a good sample example, and if we can broaden
it out that would be really helpful -->

## Improving our I/O Project

We can improve our `grep` implementation I/O project from Chapter 12 by using
iterators to make places in the code clearer and more concise. Let's take a
look at how iterators can improve our implementation of both the `Config::new`
function and the `grep` function.

### Removing a `clone` Using an Iterator

Back in the `grep` implementation in Listing 12-8, we had code in Listing 13-X
that took a slice of `String` values and created an instance of the `Config`
struct by checking for the right number of arguments, indexing into the slice,
and cloning the values so that the `Config` struct could own those values:

```rust,ignore
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let search = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            search: search,
            filename: filename,
        })
    }
}
```

<!--Is this why we didnt want to use clone calls, they were inefficient, or was
it that stacking clone calls can become confusing/is bad practice? -->

At the time, we said not to worry about the inefficient `clone` calls here
because we would remove them in the future. Well, that time is now!

The reason we needed `clone` here in the first place is that we have a slice
with `String` elements in the parameter `args`, but the `new` function does not
own `args`. In order to be able to return ownership of a `Config` instance, we
need to clone the values that we put in the `search` and `filename` fields of
`Config`, so that the `Config` instance can own its values.

With out new knowledge about iterators, we can change the `new` function to
instead take ownership of an iterator as its argument. Rather than having to
check the length of the slice and index into specific locations, we'll use the
interator functionality to XXX.

<!-- use the iterator functionality to what? How will iterating allow us to do
the same thing, can you briefly lay that out? -->

Now that we've taken ownership of the iterator and won't be using indexing
operations that borrow, we can move the `String` values from the iterator into
`Config` rather than calling `clone` and making a new allocation.

<!-- below: which file are we in, can you specify here? -->

#### Heading

Let's update our I/O project! Open your XXXX.rs file. First, let's take `main`
as it was in Listing 12-6, and change it to pass the return value of
`env::args` to `Config::new`, instead of calling `collect` and passing a slice:

```rust,ignore
fn main() {
    let config = Config::new(env::args());
    // ...snip...
```

Listing 13-XX

<!-- I think, if we're going to be building this up bit by bit, it might be
worth adding listing numbers and captions to each, can you add those? Don't
worry about being accurate with the numbers, we can update them more easily
later -->

<!-- Will add ghosting in libreoffice /Carol -->

If you look in the standard library documentation for the `env::args` function,
you'll see that its return type is `std::env::Args`. So next we'll update the
signature of the `Config::new` function so that the parameter `args` has the
type `std::env::Args` instead of `&[String]`:

```rust,ignore
impl Config {
    fn new(args: std::env::Args) -> Result<Config, &'static str> {
        // ...snip...
```

<!-- can you give the filename here too? -->

<!-- Will add ghosting in libreoffice /Carol -->

#### Heading

Next, we'll fix the body of `Config::new`. As you can also see in the standard
library documentation, `std::env::Args` implements the `Iterator` trait, so we
know we can call the `next` method on it! Here's the new code:

```rust
impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    	args.next();

        let search = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a search string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config {
            search: search,
            filename: filename,
        })
    }
}
```

Listing 13-x:

<!-- is this the *full* new main.rs code? Worth noting for ghosting purposes -->

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Remember that the first value in the return value of `env::args` is the name of
the program. We want to ignore that and get to the next value, so first we call
`next` and do nothing with the return value. We secondly call `next` on the
value we want to put in the `search` field of `Config`. If `next` returns a
`Some`, we use a `match` to extract the value . If it returns `None`, it means
not enough arguments were given and we return early with an `Err` value.

We do the same thing for the `filename` value. It's slightly unfortunate that
the `match` expressions for `search` and `filename` are so similar. It would be
nice if we could use `?` on the `Option` returned from `next`, but currently in
Rust `?` only works with `Result` values. However, even if we could use `?` on
`Option` we would get a borrowed value, and we need it to be owned to we can
move the `String` from the iterator into `Config`.

<!-- Hm, if ? would not work anyway, I'm not clear on why we mention, why it's
a shame we cant use it on Option? -->

### Making Code Clearer with Iterator Adaptors

The other place in our I/O project we could take advantage of iterators in in
the `grep` function itself, as implemented in Listing 12-15:

<!-- We hadn't had a listing number for this code sample when we submitted
chapter 12; we'll fix the listing numbers in that chapter after you've reviewed
it. /Carol -->

```rust
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(search) {
            results.push(line);
        }
    }

    results
}
```

We can write this code in a much shorter way, and avoiding having to use a
mutable intermediate `results` vector, by using iterator adaptor methods like
this instead:

<!-- Remind us why we want to avoid the mutable results vector? -->

```rust
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(search))
        .collect()
}
```

Here, we use the `filter` adaptor to keep only the lines that
`line.contains(search)` returns true for.

<!-- what is that, here, only lines that contain a matching string? A bit more
context would help out, we probably can't rely on readers remembering all the
details I'm afraid -->

We then collect the mathcing lines up into another vector with `collect`. Much
simpler!

We can use the same technique in the `grep_case_insensitive` function that we
defined in Listing 12-16, as follows:

<!-- Similarly, the code snippet that will be 12-16 didn't have a listing
number when we sent you chapter 12, we will fix it. /Carol -->

```rust
fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| {
            line.to_lowercase().contains(&search)
        }).collect()
}
```

Not too bad! The next logical question is which style you should choose in your
own code. Most Rust programmers prefer to use the iterator style. It's a bit
tougher to get the hang of at first, but once you get a feel for the various
iterator adaptors and what they do, iterators are much easier to understand.
Instead of fiddling with the various bits of looping and building new vectors,
the code focuses on the high-level objective of the loop: abstracting some of
the commonplace code so that it's easier to see the concepts that are unique to
this usage of the code, like the filtering condition each element in the
iterator must pass.

But are they truly equivalent? The intuitive assumption migh tbe that the more
low-level loop will be faster. Let's talk about performance.

## Comparing Performance: Loops versus Iterators

To determine which to use, we need to know which version of our `grep`
functions is faster: the version with an explicit `for` loop or the version
with iterators.

We ran a benchmark by loading the entire contents of "The Adventures of
Sherlock Holmes" by Sir Arthur Conan Doyle into a `String` and looking for the
word "the" in the contents. Here were the results of the benchmark on the
version of grep using the `for` loop and the version using iterators:

```text
test bench_grep_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_grep_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

The iterator version ended up slightly faster! We're not going to go through
the benchmark code here, as the point is not to prove that they're exactly
equivalent, but to get a general sense of how these two implementations compare
perfromance-wise. For a more accurate and useful benchmark, you'd want to check
various texts of various sizes, different words, words of different lengths,
and all kinds of other variations. The point is this: iterators, while a
high-level abstraction, get compiled down to roughly the same code as if you'd
written the lower-level code yourself. Iterators are one of Rust's *zero-cost
abstractions*, by which we mean using the abstraction imposes no additional
runtime overhead in the same way that Bjarne Stroustrup, the original designer
and implementer of C++, defines *zero-overhead*:

> In general, C++ implementations obey the zero-overhead principle: What you
> don’t use, you don’t pay for. And further: What you do use, you couldn’t hand
> code any better.
>
> - Bjarne Stroustrup "Foundations of C++"

<!-- should this be "handle code any better", above? -->

As another example, here is some code taken from an audio decoder.

<!-- Can you briefly explain what the intention of the code it --- that will
help us understand, for example, why we have a `prediction` value -->

This code uses an iterator chain to do some math on three variables in scope: a
`buffer` slice of data, an array of 12 `coefficients`, and an amount by which
to shift data in `qlp_shift`. We've declared the variables within this example
but not given them any values; while this code doesn't have much meaning
outside of its context, it's still a concise, real-world example of how Rust
translates high-level ideas to low-level code:

```rust,ignore
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

In order to calculate the value of `prediction`, this code iterates through
each of the 12 values in `coefficients`, and uses the `zip` method to pair the
coefficient values with the previous 12 values in `buffer`. Then, for each
pair, we multiply the values together, sum all the results, and shift the bits
in the sum `qlp_shift` bits to the right

Calculations in applications like audio decoders often prioritize performance
most highly. Here, we're creating an iterator, using two adaptors, then
consuming the value. What assembly code would this Rust code compile to? Well,
as of this writing, it compiles down to the same assembly you'd write by hand.
There's no loop at all corresponding to the iteration over the values in
`coefficients`: Rust knows that there are twelve iterations, so it "unrolls"
the loop.

<!-- Maybe some expansion on what you mean by unrolls? -->

All of the coefficients get stored in registers, which means it's very fast to
access the values. There are no bounds checks on the array access. It's
extremely efficient.

Now that you know this, go use iterators and closures without fear! They make
code feel higher-level, but don't impose a runtime performance penalty for
doing so.

## Summary

Closures and iterators are Rust features inspired by functional programming
language ideas. They contribute to Rust's ability to clearly express high-level
ideas, at low level performance. The implementations of closures and iterators,
as well as other zero-cost abstractions in Rust, are such that runtime
performance is not affected.

<!-- Are we going to cover which other elements of rust are zero-cost
abstractions, somewhere? Might be good to cross ref or, if we've already
covered, give a brief list or a way to identify them -->

Now that we've improved the expressiveness of our I/O project, let's look at
some more features of `cargo` that would help us get ready to share the project
with the world.
