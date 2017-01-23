# Fearless Concurrency

So, with Rust, it's more subtle than that. That is, while threading proper
isn't part of the language itself, Rust's type system is structured in such a
way as to make it possible to build those kinds of libraries. In other words,
Rust's focus on aliasability ends up solving these problems.

This is a library abstraction.

Shared mutable state is a problem. Both useful. Functional languages get rid of
mutability.

Ownership rules (that tame the "shared" aspect) enable fearless concurrency: the
compiler is making sure you don't shoot yourself in your foot.

## What are threads



## Rust's concurrency tradeoffs

Lots of different languages tackle this problem in different ways. We are not
going to talk about that: exercise for the reader is investigate other languages
and compare and contrast with Rust's approach.

This is how Rust does it, what rust means by threads

OS threads are exposed in the standard library bc a systems programming language
should integrate with your system.

If you have a different threaded mechanism, you need a runtime, rust is trying
to not have a heavy runtime.

These are the reasons Rust's concurrency model is this way as opposed to other
language's ways, which are optimizing for different things.


## Let's get a thread: `thread::spawn`

Code examples - just print stuff, no data sharing

## Communicating between threads


### Closures, Ownership, and Borrowing

The property of being allowed to use variables from the surrounding scope is
also subject to all of the usual rules around ownership and borrowing. Since
closures attempt to infer the types of their parameters, they also infer how
those parameters are borrowed. Closures make that inference by looking at how
they are used. Consider the example in Listing 13-5 that has functions that
borrow immutably, borrow mutably, and move their parameters, then closures that
reference values from their environment and call each of the functions. We'll
see how this affects inference of when a value is borrowed:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
struct Foo;

fn borrows(f: &Foo) {
    println!("Took {:?} by reference.", f);
}

fn borrows_mut(f: &mut Foo) {
    println!("Took {:?} by mutable reference.", f);
}

fn moves(f: Foo) {
    println!("Took ownership of {:?}.", f);
}

fn main() {
    let f1 = Foo;
    let closure_that_borrows = |x| borrows(x);
    closure_that_borrows(&f1);

    let mut f2 = Foo;
    let closure_that_borrows_mut = |y| borrows_mut(y);
    closure_that_borrows_mut(&mut f2);

    let f3 = Foo;
    let closure_that_moves = |z| moves(z);
    closure_that_moves(f3);
}
```

<figcaption>

Listing 16-something: Closures that borrow, borrow mutably, and take ownership
of their parameters, which is inferred from how the closure body uses the
parameters

</figcaption>
</figure>

Here, Rust is able to look at how we use the parameters of each closure inside
their bodies. If the closure passes its parameter it to a function that takes
`&Foo`, then the type of the parameter must be `&Foo`. If it passes the
parameter to a function that takes `&mut Foo`, then the type of parameter must
be `&mut Foo`, and so on. If we try to use `f3` after the call to
`closure_that_moves` in the last line of `main`, we'll get a compiler error
since ownership of `f3` was transferred to `closure_that_moves`, which
transferred ownership to the function `moves`.

### Overriding Inferred Borrowing with the `move` Keyword

Rust will allow you to override the borrowing inference by using the `move`
keyword. This will cause all of the closure's parameters to be taken by
ownership, instead of whatever they were inferred as. Consider this example:

```rust
let mut num = 4;

{
    let mut add_num = |x| num += x;

    add_num(6);
}

assert_eq!(10, num);
```

In this case, the `add_num` closure took a mutable reference to `num`, then
when we called `add_num`, it mutated the underlying value. In the last line,
`num` contains 10, as we'd expect. We also needed to declare `add_num` itself
as `mut` too, because we're mutating its environment.

If we change the definition of `add_num` to a `move` closure, the behavior is
different:

```rust
let mut num = 4;

{
    let mut add_num = move |x| num += x;

    add_num(6);
}

assert_eq!(4, num);
```

In the last line, `num` now contains 4: `add_num` took ownership of a copy of
`num`, rather than mutably borrowing `num`.

One of the most common places you'll see the `move` keyword used is with
threads, since it's important that one thread is no longer allowed to use a
value once the value has been transferred to another thread through a closure
in order to prevent data races. We'll talk more about that in Chapter XX.

### Closures and Lifetimes

Remember Listing 10-8 from the Lifetime Syntax section of Chapter 10? It looked
like this:

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

This example doesn't compile since `x` doesn't have a long enough lifetime.
Because closures may borrow variables from their enclosing scope, we can
construct a similar example with a closure that borrows `x` and tries to return
that borrowed value. The code in Listing 13-6 also won't compile:

<figure>

```rust,ignore
{
    let closure;

    {
        let x = 4;

        closure = || x ; // A closure that takes no arguments and returns x.
    }
}
```

<figcaption>

Listing 16-something: A closure that tries to return a borrowed value that does
not live long enough

</figcaption>
</figure>

We get an error because `x` does not live long enough:

```text
error: `x` does not live long enough
  -->
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

To fix the error in the code in Listing 13-6, we can use the `move` keyword
from the last section to make the closure take ownership of `x`. Because `x` is
a number, it is a `Copy` type and therefore will be copied into the closure.
The code in Listing 13-7 will compile:

<figure>

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

<figcaption>

Listing 16-something: Moving a value into the closure to fix the lifetime error

</figcaption>
</figure>

Even though we modified `x` between the closure definition and `assert_eq!`,
since `closure` now has its own version, the changes to `x` won't change the
version of `x` that's in the closure.

Rust doesn't provide a way to say that some values a closure uses should be
borrowed and some should be moved; it's either all by inference or all moved by
adding the `move` keyword. However, we can accomplish the goal of borrowing
some values and taking ownership of others by combining `move` with some extra
bindings. Consider this example where we want to borrow `s1` but take ownership
of `s2`:

```rust
let s1 = String::from("hello");
let s2 = String::from("goodbye");

let r = &s1;

let calculation = move || {
    r;
    s2;
};

println!("Can still use s1 here but not s2: {}", s1);
```

We've declared `calculation` to `move` all the values it references. Before
defining `calculation`, we declare a new variable `r` that borrows `s1`. Then
in the body of the `calculation` closure, we use `r` instead of using `s1`
directly. The closure takes ownership of `r`, but `r` is a reference, so the
closure hasn't taken ownership of `s1` even though `calculation` uses `move`.

### `Channels`

Look up examples of cases where channels are useful

Can match modeling of certain problems

#### `Send`

Send is a trait that means i'm allowed to transfer ownership to another thread
down a channel

What things can be send and what can't?

## Sharing data between threads

Try to share data and get an error about which trait it doesn't implement

### `Sync`

It's ok to access a thing from multiple threads at once

Immutable things can be sync easily.

### `Arc<T>`

Atomic Reference Counting. Inner data still has to be immutable.

Steve knows the motivating code that goes here.

### `Mutex<T>`

For mutable data.

`lock` method, you get a Mutex guard. Change, then unlock, which usually happens
automatically when the Mutex guard goes out of scope. If you do this wrong, your
code will hang.

Deadlocks are safe, you have to manage that yourself. Deadlock bugs usually
happen bc you forget to unlock, but drop unlocks automatically.


## Maybe make the I/O project concurrent?

Might be a lot of boilerplate without scoped threads, maybe just allude.



This is a really rough sketch of some ideas that this chapter might cover.

From a comment of steveklabnik's on [the definitely not orange website]. "that paper" refers to [Boehm 2004].

[the definitely not orange website]: https://news.ycombinator.com/item?id=13078384
[Boehm 2004]: http://www.hpl.hp.com/techreports/2004/HPL-2004-209.pdf


So for example, in that paper, 4.1 is about the problem of concurrent
modifiability. And indeed, it says

> Indeed, under the implementation strategy we outlined above, in which the
> compiler is unaware of threads, it is allowed to transform code subject only
> to sequential correctness constraints and hence could generate the code
> containing a race.

However, in Rust, this re-ordering can't happen: Rust won't let you alias x and
y between two threads without some sort of synchronization primitive. But this
isn't because Rust knows about concurrency, it's because Rust knows about
aliasing. In a sense, Rust-the-language makes this program _impossible to
write_, but a library re-enables you to write this program. You need unsafe to
do this, but it's all wrapped up inside of the implementation of, for example,
Mutex<T>.

From the last part of this section:

> Resolving it essential requires a programming-language-defined and
> compiler-respected memory model, simply to ensure that the user and compiler
> can agree on when there is a data race.

We're in agreement here, but the model is built around aliasing, not
concurrency.

4.2 is about speculatively executing store instructions. I know less about
this, but again, it's built on the idea of two threads accessing data at the
same time, unsynchronized. This can't happen in Rust due to the aliasing rules.

4.3 is about register promotion. This cannot happen in Rust, because you don't
call a function to acquire the lock, then do whatever you want. Mutex<T> hides
the value it's locking inside of itself, unable to be accessed from the
outside, and the call to acquire the lock returns a mutable reference to the
inner data. The call to acquire the lock is the only way to get said reference,
and Rust's aliasing rules will forbid any other kind of access through the
returned reference. So this kind of transformation can't happen in Rust either.

Section 5 is about performance. It's true that synchronization primitives are
expensive. Rust can again use unsafe code in a disciplined way to provide safe
concurrent modification, while ruling out data races entirely. For example,
consider a simple map operation. We take an array of integers, and for each
element, add one to it. This is an embarrassingly parallel operation, yet, as
the paper mentions, with a pthreads-style approach to making it safe, one would
need either a single lock around the whole array, which destroys the
concurrency entirely, or some set of more fine-grained locks, which introduce
cost, as well as limiting the amount of concurrency to some degree.

But with a [small utility function][fn], which performs a small (ie, non-atomic)
check at runtime, we can safety split up our array into as many disjoint chunks
as we'd like, and then pass each one off to its own thread, which is free to do
the modification with no more synchronization needed. In fact, libraries like
Rayon can even determine roughly the correct amount for you, if you don't want
to think about it, and it will near-transparently just handle this for you (you
change a call from iter() to par_iter() and you're done).

[fn]: https://github.com/rust-lang/rust/blob/f8614c397313db00e4b4626d1ba77ae00dbf7549/src/libcore/slice.rs#L344-L355

So yeah. I'm in agreement with the paper that the language needs to do _some_
kind of reasoning, but since aliasing and concurrency are so tightly related, I
would argue that the language could understand only aliasing, not concurrency,
and then library abstractions are sufficient.

## Arc

Check out [this awesome explanation of `Arc`](http://stackoverflow.com/a/40985661/51683).
