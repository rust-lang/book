# Functional Language features in Rust - Iterators and Closures

As a language, Rust takes influence from a lot of places. One of those places
is functional programming. We won't use this chapter to debate what exactly
'functional programming' is, but instead, show off some features of Rust that
are similar to many languages that are referred to as functional.

More specifically, we're going to cover:

* Closures, a function-like construct you can store in a binding.
* Iterators, a way of processing series of elements.
* Using these features to improve upon the project from the last chapter.
* A bit about these features' performance. Spoiler alert: they're faster than
  you might think!

This is not a complete list of Rust's influence from the functional style;
pattern matching, enums, and many other features are too. But mastering
closures and iterators will help you write idiomatic, fast Rust code.

## Closures

Rust gives you the ability to define *closures*, which are sort of like
functions. Instead of giving you a technical definintion, let's dive into
what clousures look like, syntactically:

```rust
let add_one = |x| x + 1;

let five = add_one(4);

assert_eq!(5, five);
```

The first line defines a closure, and binds it to the `add_one` variable. The
arguments to the closure go in between the pipes (`|`).

This is a simple closure with only one expression as its body, let's see one
that's got a bit more going on:

```rust
let calculate = |a, b| {
    let mut result = a * 2;

    result += b;

    result 
};

assert_eq!(7, calculate(2, 3)); // 2 * 2 + 3 == 7
assert_eq!(13, calculate(4, 5)); // 4 * 2 + 5 == 13
```

We can use `{}`s to give a closure a body with more than one expression.

You'll notice a few things about closures that are a bit different from
functions defined with `fn`. The first is that we did not need to annotate the
types of arguments the closure takes or the values it returns. We can:

```rust
let plus_one = |x: i32| -> i32 { x + 1 };

assert_eq!(2, plus_one(1));
```

But we dont have to. Why is this? Functions are part of an explicit interface
that's exposed to your users, so defining this interface rigidly is helpful.
But closures aren't used like this: they're stored in bindings and called
directly. Being forced to annotate the types would be a significant ergonomic
loss for little advantage.

The syntax is similar, but a bit different. Let's compare more directly. We've
added some spaces here to line up the relevant parts:

```rust
fn  plus_one_v1   (x: i32) -> i32 { x + 1 }  // a function
let plus_one_v2 = |x: i32| -> i32 { x + 1 }; // the full syntax for a closure
let plus_one_v3 = |x: i32|        { x + 1 }; // a closure eliding types
let plus_one_v4 = |x: i32|          x + 1  ; // without braces
```

Small differences, but they're similar. Why come up with a different syntax
for closures? There's one additional way in which they're different from
functions: they posses an 'environment'.

## Closures and their environment

We've learned that functions can only use variables that are in scope, either
by being static or being declared as parameters. But closures can do more.
They can access variables from their enclosing scope. Like this:


```rust
fn main() {
    let x = 4;
  
    let equal_to_x = |z| z == x;
  
    let y = 4;
  
    assert!(equal_to_x(y));
}
```

Here, even though `x` is not an argument to `equal_to_x`, it's able to
refer to it, as it's a variable defined in the same scope. We couldn't
do this with a `fn`. If we tried...


```rust,ignore
fn main() {
    let x = 4;
  
    fn equal_to_x(z) { z == x }
  
    let y = 4;
  
    assert!(equal_to_x(y));
}
```

We'd get an error:

```text
error: can't capture dynamic environment in a fn item; use the || { ... }
closure form instead [E0434]
z == x
     ^
```

This only works with closures! This property is also subject to all of the
usual rules around ownership and borrowing. Because closures attempt to infer
the types of their arguments, they also have to infer how they're borrowed.
They'll do that from how they are used. Consider this example:

```rust
struct Foo;

fn borrow(f: &Foo) {
    println!("Took foo by reference.");
}

fn borrow_mut(f: &mut Foo) {
    println!("Took foo by mutable reference.");
}

fn moves(f: Foo) {
    println!("Took foo by ownership.");
}

let f = Foo;
let borrows = |f| borrow(f);
borrows(&f);

let mut f = Foo;
let borrows_mut = |f| borrow_mut(f);
borrows_mut(&mut f);

let f = Foo;
let moves = |f| moves(f);
moves(f);
```

Here, Rust is able to look at how we use `f` inside of each closure. If we pass
it to a function that takes `&Foo`, then the type of `f` must be `&Foo`. If we
pass it to a function that takes `&mut Foo`, then the type of `f` must be `Foo`.
And so on.

### The `move` keyword

Rust will allow you to override this inference with the `move` keyword. This
will cause all variables to be taken by ownership, instead of whatever they
were inferred as. Consider this:

```rust
let mut num = 5;

{
    let mut add_num = |x| num += x;

    add_num(5);
}

assert_eq!(10, num);
```

So in this case, our closure took a mutable reference to `num`, and then when
we called `add_num`, it mutated the underlying value, as we'd expect. We also
needed to declare `add_num` as `mut` too, because were mutating its
environment.

If we change to a `move` closure, its different:

```rust
let mut num = 5;

{
    let mut add_num = move |x| num += x;

    add_num(5);
}

assert_eq!(5, num);
```

We only get `5`. Rather than taking a mutable borrow out on our `num`, we took
ownership of a copy.

One of the most common places you'll see the `move` keyword used is with threads.
We'll talk more about that in Chapter XX.

### Closures, ownership, and borrowing

Remember Listing 10.8 from Chapter 10.3? It looked like this:

```rust,ignore
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

This example doesn't compile, becuase `x` doesn't have a long enough lifetime.
Becuase closures may borrow variables from their enclosing scope, we can
construct a similar example with closures. It also won't compile:

```rust,ignore
{
    let closure;
    
    {
        let x = 4;
    
        closure = || x ; // A closure that takes no arguments and returns x.
    }
}
```

We get an error because `x` does not live long enough:

```text
error: `x` does not live long enough
  --> <anon>:8:22
   |
8  |         closure = || x ; // A closure that takes no arguments and returns x.
   |                   -- ^ does not live long enough
   |                   |
   |                   capture occurs here
9  |     }
   |     - borrowed value only lives until here
10 | }
   | - borrowed value needs to live until here
```

In this instance, we can use the `move` keyword from the last section
to have the closure take `x` by value, and since `x` is a number, it
is `Copy` and therefore will be copied:

```rust
{
    let closure;
    
    {
        let mut x = 4;
    
        closure = move || x ; // A closure that takes no arguments and returns x.

        x = 5;

        assert_eq!(closure(), 4); 
    }
}
```

Even though we modified `x`, since `closure` now has its own version, our
changes to `x` won't change the version of `x` that's in the closure.

Rust doesn't provide a way to say "these variables must be captured like this";
it's either all by inference, or all by value. However, you can accomplish
this sort of goal by combining `move` with some extra bindings:

```rust
let s1 = String::from("hello");
let s2 = String::from("hello");

// We want to capture s1 by reference, but s2 by value. What to do? First, make
// an extra binding for s1:

let r = &s1;

// Then make it a `move` closure:

let calculation = move || {
    // ... and use them inside. That's the trick: r is captured, but it's a
    // reference; so we've effectively taken s1 by reference and s2 by move.
    r;
    s2;
};
```

### Accepting closures as arguments with the `Fn` traits

While we can bind closures to variables, that's not the most useful thing we
can do with them. We can also take closures as arguments to functions. We can
do that with the `Fn` traits. Like this:

```rust
fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32 {

    some_closure(1)
}

let answer = call_with_one(|x| x + 2);

assert_eq!(3, answer);
```

We pass our closure, `|x| x + 2`, to `call_with_one`. It does what it suggests:
it calls the closure, giving it `1` as an argument.

Let's examine the signature of `call_with_one` in more depth:

```rust
fn call_with_one<F>(some_closure: F) -> i32
#    where F: Fn(i32) -> i32 {
#    some_closure(1) }
```

We take one parameter, and it has the type `F`. We also return an `i32`. This
part isnt interesting. The next part is:

```rust
# fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32 {
#   some_closure(1) }
```

The `Fn` trait represents a closure. We can use it as a bound for our generic
type. In this case, our closure takes an `i32` as an argument and returns an
`i32`, and so the generic bound we use is `Fn(i32) -> i32`.

Why a trait? Well, each closure has a unique type. Becuase of this, we can't
write the type of a closure directly, we have to use generics.

`Fn` isn't the only trait, however, there are three. `Fn`, `FnMut`, and
`FnOnce`. This continues the patterns of threes we've seen elsewhere in Rust:
by owner, by reference, and by mutable reference. By using `Fn`, you may only
refer to things in its environment by reference. If you mutate the environment,
you must use `FnMut`, and if you take ownership of the environment, `FnOnce`.
Most of the time, you can write `Fn`, and then the compiler will tell you if
you need `FnMut` or `FnOnce`.

## Iterators

Let's move on to another topic: iterators. Iterators are a pattern in Rust
that allows you to do some processing on a sequence of items. Like this:

```rust
let v1 = vec![1, 2, 3];

let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, [2, 3, 4]);
```

That second line is full of new concepts. Let's check each bit out, in turn:

```rust,ignore
v1.iter()
```

The `iter` method on vectors allows us to produce an *iterator* from the
vector. This iterator has a number of its own methods. The next section
is one of those:

```rust,ignore
.map(|x| x + 1)
```

The `map` method on an iterator allows us to process each element: for every
element `x`, we add one to it. `map` is one of the most basic ways of
interacting with an iterator, as processing each element in turn is very
useful!

`map` itself is sometimes called an *iterator adapter*, that is, it's a method
on an iterator that produces a new iterator. That is, `map` builds on top of
our previous iterator and produces another one, by calling the closure it's
passed to produce the new sequence of values. There are many useful iterator
adapters, but before we talk about them, let's get to the last bit:

```rust,ignore
.collect()
```

The `collect` method consumes an iterator, and puts them into a new data
structure. In this case, since we've said that `v2` has the type `Vec<i32>`, it
will create a new vector out of them.

So, to recap:

1. Create an iterator from the vector.
2. Use the `map` adaptor to add one to each element.
3. Use the `collect` adaptor to consume the iterator and make a new vector.

That's how we end up with `[2, 3, 4]`. As you can see, closures are a very
important part of using iterators; they provide the way of customizing the
behavior of an iterator adapter like `map`.

### Iterators are Lazy

In the previous section, you may have noticed a subtle bit of wording: we
said that `map` adapts an iterator, but that `collect` 'consumes' one. That
was intentional. By itself, iterators won't do anything; that is, they're
lazy. That is, if we write code like this:

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1); // without collect
```

It will compile, but it will give us a warning:

```text
error: unused result which must be used: iterator adaptors are lazy and do
       nothing unless consumed
 --> <anon>:5:1
  |
5 | v1.iter().map(|x| x + 1);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
```

That is, iterator adaptors won't start actually doing the processing on their
own. They need some sort of adaptor that causes the iterator chain to evaluate.
We call those 'consuming adaptors', and `collect` is one of them.

So, with these different kinds of iterator adaptors... how do we tell which
ones consume or not? And what adaptors are available? For that, let's look at
the `Iterator` trait.

### The Iterator trait

Iterators all implement a trait, `Iterator`, that is defined in the standard
library. It looks like this:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

There's some new syntax that we haven't covered here yet, and that's the `type
Item` and `Self::Item` bits. This is called an "associated type", and we'll
talk about them in Chapter XX. For now, all you need to know is that this
says that the `Iterator` trait requires that you also define an `Item` type,
and that its type is used in the return type of `next`. The `Item` type will
be the type of element that's returned from the iterator. You'll learn more
about why this is needed in that chapter.

Let's make an iterator named `Counter` which counts from `1` to `5`, using
this trait. First, we need to create a struct that holds the state for
our iterator:

```rust
struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

The `new` method here isn't strictly neccesary, but we want our `Counter`
to go from one to five, so we're going to initialize it with a zero. Let's
see why by implementing `Iterator` for it:

```rust,ignore
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

There's a lot going on here! First of all, we assign `Item` to be `u32`. Remember
that we don't really understand this syntax yet, you'll just have to trust me.

Next, we implement the `next` method. This method is the main interface into an
iterator: it returns an option. If the option is `Some(value)`, we have gotten
another value from the iterator. If it's `None`, we know that iteration is
finished. Inside of it, we do whatever kind of calculation our iterator needs.
In this case, we add one, and then check to see if we're still below six. If we
are, we can return `Some(self.count)`, to give the next value, but if we're at
six or more, iteration is over, and we return `None`.

Once we've implemented this, we have an iterator! We can use it by calling
`next` repeatedly:

```rust,ignore
let mut counter = Counter::new();

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);
```

This will print `1` through `5`, each on their own line.

Calling `next()` this way gets repetitive. Rust has a construct which can call
`next()` on your iterator, until it reaches `None`. Let's go over that next.

### IntoIterator and for loops

When we said that iterators are important to Rust, we weren't joking: iterators
are how `for` loops work under the hood! Remember this `for` loop from Chapter 3?

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

At the time, we didn't explain the `.iter()` bit, but now you know that it
makes an iterator. Rust's `for` loop is actually 'synatx sugar', that is, it's
special syntax, but we could write it ourselves. It's just a bit nicer to write
by using `for`. If we took the code above and expanded it, it would look like
this:


```rust,ignore
let a = [10, 20, 30, 40, 50];

{
    let result = match IntoIterator::into_iter(a) {
        mut iter => loop {
            match iter.next() {
                Some(element) => { println!("the value is: {}", element); },
                None => break,
            }
        },
    };
    result
}
```

Whew! This code is very compact, and uses a lot of concepts. We've talked
about them all already though, so let's break it down:

```rust,ignore
let result = match IntoIterator::into_iter(a) {
```

`IntoIterator` is another trait that we haven't discussed yet. As the name
suggests, it has an `into_iter` method that takes one argument, and turns
that argument into an `Iterator`. This means that we can pass anything
that's can be converted into an iterator to a `for` loop, and it will
just work. That's nice! However, arrays do not implement `IntoIterator`,
and so we had to call the `iter` method ourself. But since that returns
an iterator, calling `into_iter` on it does nothing, so we're still good!

We're also `match`ing on the iterator that's returned. Let's look at how
that works:

```rust,ignore
mut iter => loop {
```

The `match` only has one arm: we match the result, binding it to a mutable
binding, `iter`, and then immediately call `loop` to loop forever. What's
in the body of that loop? Another `match!`

```rust,ignore
match iter.next() {
    Some(element) => { println!("the value is: {}", element); },
    None => break,
}
```

Here, we call `next()`, and see if it's `Some`. If it is, then we call
the body of the `for` loop, which in this case is a single `println!`.
If we got `None` back from the iterator, we `break` out of the loop.

### IntoIterator and vectors

Let's talk a bit more about `IntoIterator`. As we said above, it's job is to
convert something into an iterator. You'll find it implemented on all kinds of
handy things. Consider this example:

```rust
let v = vec![1, 2, 3];

for e in v {
    println!("iterating by owner");
}

let v = vec![1, 2, 3];

for e in &v {
    println!("iterating by reference");
}

let mut v = vec![1, 2, 3];

for e in &mut v {
    println!("iterating by mutable reference");
}
```

Whoah! The standard library implements `IntoIterator` on vectors directly,
allowing you to take ownership of each element of the vector. But it also
implements it on `&Vec<T>` and `&mut Vec<T>`, which allow you to iterate over
references and mutable references, respectively. Since the `for` loop
implicitly calls `IntoIterator::into_iter` for us, we don't need to do
anything. It just works.

`IntoIterator` is a very useful trait, but we should move on. You can find more
about it in its documentation.

### All sorts of adaptors

So we implemented `Iterator` for our `Counter`, but we only wrote `next`.
When we used iterators at the start of this section, we had other methods,
like `map` and `collect`. What about those?

Well, when we told you about the definition of `Iterator`, we committed a
small lie of omission. The `Iterator` trait has a number of other useful
methods. Like `map`:

```rust,ignore
fn map<B, F>(self, f: F) -> Map<Self, F> where
    Self: Sized, F: FnMut(Self::Item) -> B;
```

And `collect`:

```rust,ignore
fn collect<B: FromIterator<Self::Item>>(self) -> B where Self: Sized;
```

Both of these type signatures make advanced usages of generics, so don't
worry about fully understanding them right now. The point is this: all
of these other methods on `Iterators` are implemented for you, in terms
of `next`. That is, you only need to implement `next`, and you get
all of the other adaptors for free. There are a lot of them!

```rust
let sum: u64 = (1..).zip(2..)
    .map(|(a, b)| a + b)
    .filter(|&x| x < 100)
    .take(5)
    .sum();
    
assert_eq!(35, sum);
```

The `1..` and `2..` are ranges, but they work as infinite iterators: they'll
count from one and two until infinity. The `zip` adaptor zips two iterators
together, producing a new iterator which gives you tuples. The first element is
from the first iterator, the second is from the second iterator. We then take
that iterator, and add the two numbers together. We then filter out only the
sums that are less than 100, and then finally, take the first five of those
numbers. Finally, `sum` will add up all of the numbers into one last number.
This is kind of a silly calculation, but it shows off a few different iterator
adaptors, you can do all kinds of things! Check the documentation of `Iterator`
to see them all. Some crates you may use in the ecosystem might add even more
adaptors as well.

## Improving our I/O project

Let's use what we learned to improve our project from the last chapter. Back
in listing 12-5, we had this code:

```rust,ignore
fn parse_config(args: &[String]) -> Config {
    let search = args[1].clone();
    let filename = args[2].clone();

    Config {
        search: search,
        filename: filename,
    }
}
```

At the time, we told you to not worry about the `clone` calls here, and that
we could remove them in the future. Well, that time is now! Later in that
section, we moved this code into `Config::new`, and it looked like this:

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

Let's fix that version, as it's the final form of the code we had. So, why do
we need `clone` here? The issue is, we have a slice of `String`s in `args`,
that is, we don't own them. So the only thing we can do is copy them. But now
that we know more about iterators, we can make this work. Let's modify
`new` to take a different kind of argument:

```rust,ignore
fn new(args: std::env::Args) -> Result<Config, &'static str> {
```

`std::env::Args` is the return type of `the `std::env::args` function.
It's an iterator! Let's modify `main` to pass it in directly, rather than
using `collect` to make a vector:

```rust,ignore
fn main() {
    let config = Config::new(env::args());
```

Much simpler. Now we need to fix the body. We know that `next` will give
us the next value of the iterator. Let's use that:

```rust,ignore
impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // The first argument is the program name, let's ignore that
	args.next();

        let search = match args.next() {
            Some(arg) => arg,
            None => return "Didn't get a search string",
	};

        let filename = match args.next() {
            Some(arg) => arg,
            None => return "Didn't get a file name",
	};

        Ok(Config {
            search: search,
            filename: filename,
        })
    }
}
```

Here, we use `next` to get the arguments out of `Args`. This code is way
better: we now will not `panic!` when we get too few arguments, but instead
return a `Result<T, E>` with a meaningful error message. One slightly
unfortunate bit is the repetition with the `match`. `?` only works on
`Result<T, E>`s, and not `Option<T>`s at the moment. It also won't copy
the `String`s, as we can get them directly from the iterator, rather than
the slice we had before. Given that `Args` produces its arguments by value,
we move them instead. Another win!

The other bit was in our version of `grep`:

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

We can write this code like this instead:

```rust
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(search))
        .collect() 
}
```

Wow, much shorter! Here, we use the `filter` adapter to only select
the lines that `contains(search)` returns true for. We then collect
them up into another vector with `collect`. Much simpler!

We can do the same for `grep_case_insensitive`:

```rust
fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| {
            line.to_lowercase().contains(&search)
        }).collect()
}
```

Not too bad! So which style should you chose? Most Rust programmers prefer to
use the iterator style. It's a bit tougher to understand at first, but once you
gain an intuition for what the various iterator adaptors do, this is much
easier to understand. Instead of fiddling with the various bits of looping
and building a new vector, it focuses on the high-level objective of the loop.

But are they truly equivalent? Surely the more low-level loop will be faster?
Let's talk about performance.

## Summary: Performance

Which version of our `grep` is faster, the one with an explicit `for` loop,
or iterators? We ran a quick benchmark by loading the entire contents of
"The Adventures of Sherlock Holmes" by Sir Arthur Conan Doyle into a `String`
and looking for the word `the` in it. Here were the times:

```text
test bench_grep_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_grep_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

That's right, the iterator version ended up slightly faster! We're not going
to share the bencharmark code exactly here, as the point is not to prove that
they're exactly equivalent. For a _real_ benchmark, you'd want to check various
texts of various sizes, different words, words of different lengths, and all
kinds of other things. The point here is this: iterators, while a high-level
abstraction, get compiled down to roughly the same code as if you'd written
the lower-level code yourself.

As another example, here's an iterator chain that does some math:

```rust,ignore
// We have these three variables in scope:
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

This code sample is taken from an audio decoder. It "restores sample values
from residues," if that means anything to you. The point is, doing math is
something that often needs to be done very quickly, so you care about speed.
But here, we're creating an iterator, using two adaptors, and then finally
consuming the value. What would this code compile to? Well, as of this writing,
this, it compiles down to the same assembly you'd write by hand: there's no
loop at all, as it knows that there are twelve iterations, and so it "unrolls"
the loop. All of the coefficients get stored in registers (read: they're very
fast). There are no bounds checks on the array access. It's extremely
efficient.

Now that you know this, go use iterators and closures without fear! They can
really make code feel more high-level, but don't have a performance penalty for
doing so.
