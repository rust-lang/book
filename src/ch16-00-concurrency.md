# Fearless Concurrency

So, with Rust, it’s more subtle than that. That is, while threading proper
isn’t part of the language itself, Rust’s type system is structured in such a
way as to make it possible to build those kinds of libraries. In other words,
Rust’s focus on aliasability ends up solving these problems.

This is a library abstraction.

Shared mutable state is a problem. Both useful. Functional languages get rid of
mutability.

Ownership rules (that tame the “shared” aspect) enable fearless concurrency: the
compiler is making sure you don’t shoot yourself in your foot.

## What are threads



## Rust’s concurrency tradeoffs

Lots of different languages tackle this problem in different ways. We are not
going to talk about that: exercise for the reader is investigate other languages
and compare and contrast with Rust’s approach.

This is how Rust does it, what rust means by threads

OS threads are exposed in the standard library bc a systems programming language
should integrate with your system.

If you have a different threaded mechanism, you need a runtime, rust is trying
to not have a heavy runtime.

These are the reasons Rust’s concurrency model is this way as opposed to other
language’s ways, which are optimizing for different things.


## Let’s get a thread: `thread::spawn`

Code examples - just print stuff, no data sharing

## Communicating between threads

### `Channels`

Look up examples of cases where channels are useful

Can match modeling of certain problems

#### `Send`

Send is a trait that means i’m allowed to transfer ownership to another thread
down a channel

What things can be send and what can’t?

## Sharing data between threads

Try to share data and get an error about which trait it doesn’t implement

### `Sync`

It’s ok to access a thing from multiple threads at once

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

From a comment of steveklabnik’s on [the definitely not orange website]. “that paper” refers to [Boehm 2004].

[the definitely not orange website]: https://news.ycombinator.com/item?id=13078384
[Boehm 2004]: http://www.hpl.hp.com/techreports/2004/HPL-2004-209.pdf


So for example, in that paper, 4.1 is about the problem of concurrent
modifiability. And indeed, it says

> Indeed, under the implementation strategy we outlined above, in which the
> compiler is unaware of threads, it is allowed to transform code subject only
> to sequential correctness constraints and hence could generate the code
> containing a race.

However, in Rust, this re-ordering can’t happen: Rust won’t let you alias x and
y between two threads without some sort of synchronization primitive. But this
isn’t because Rust knows about concurrency, it’s because Rust knows about
aliasing. In a sense, Rust-the-language makes this program _impossible to
write_, but a library re-enables you to write this program. You need unsafe to
do this, but it’s all wrapped up inside of the implementation of, for example,
Mutex<T>.

From the last part of this section:

> Resolving it essential requires a programming-language-defined and
> compiler-respected memory model, simply to ensure that the user and compiler
> can agree on when there is a data race.

We’re in agreement here, but the model is built around aliasing, not
concurrency.

4.2 is about speculatively executing store instructions. I know less about
this, but again, it’s built on the idea of two threads accessing data at the
same time, unsynchronized. This can’t happen in Rust due to the aliasing rules.

4.3 is about register promotion. This cannot happen in Rust, because you don’t
call a function to acquire the lock, then do whatever you want. Mutex<T> hides
the value it’s locking inside of itself, unable to be accessed from the
outside, and the call to acquire the lock returns a mutable reference to the
inner data. The call to acquire the lock is the only way to get said reference,
and Rust’s aliasing rules will forbid any other kind of access through the
returned reference. So this kind of transformation can’t happen in Rust either.

Section 5 is about performance. It’s true that synchronization primitives are
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
as we’d like, and then pass each one off to its own thread, which is free to do
the modification with no more synchronization needed. In fact, libraries like
Rayon can even determine roughly the correct amount for you, if you don’t want
to think about it, and it will near-transparently just handle this for you (you
change a call from iter() to par_iter() and you’re done).

[fn]: https://github.com/rust-lang/rust/blob/f8614c397313db00e4b4626d1ba77ae00dbf7549/src/libcore/slice.rs#L344-L355

So yeah. I’m in agreement with the paper that the language needs to do _some_
kind of reasoning, but since aliasing and concurrency are so tightly related, I
would argue that the language could understand only aliasing, not concurrency,
and then library abstractions are sufficient.

## Arc

Check out [this awesome explanation of `Arc`](http://stackoverflow.com/a/40985661/51683).
