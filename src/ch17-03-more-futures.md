## Working With More Futures

When we switched from using two futures to three in the previous section, we
also had to switch from using `join` to using `join3`. It would be annoying to
do this every time we changed our code. Happily, we have a macro form of `join`
to which we can pass an arbitrary number of arguments. It also handles awaiting
the futures itself. Thus, we could rewrite the code from Listing 17-TODO to use
`join!` instead of `join3`, as in Listing 17-TODO:

<Listing number="17-TODO" caption="Using `join!` to wait for multiple futures" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09b/src/main.rs:here}}
```

</Listing>

This is definitely a nice improvement over needing to swap between `join` and
`join3` and `join4` and so on! However, both the function nor macro forms of
`join` only work for cases where we know the number of futures ahead of time. If
instead we have a dynamic number of futures, we need a function which works with
a collection type which can grow and shrink dynamically at runtime, such as a
vector. In real-world Rust, pushing futures into a collection and then waiting
on some or all the futures in that collection to complete is a very common
pattern.

The `trpl::join_all` function accepts any type which implements the `Iterator`
trait, which we learned about back in Chapter 13, so it seems like just the
ticket. Let’s try putting our futures in a vector, and replace `join3` with
`join_all`.

<Listing  number="17-TODO" caption="Storing anonymous futures in a vector and calling `join_all`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:here}}
```

</Listing>

Unfortunately, this does not compile. Instead, we get this error:

<!-- TODO: extract to output.txt -->

```text
error[E0308]: mismatched types
  --> src/main.rs:43:37
   |
8  |           let tx1_fut = async move {
   |  _______________________-
9  | |             let vals = vec![
10 | |                 String::from("hi"),
11 | |                 String::from("from"),
...  |
19 | |             }
20 | |         };
   | |_________- the expected `async` block
21 |
22 |           let rx_fut = async {
   |  ______________________-
23 | |             while let Some(value) = rx.recv().await {
24 | |                 println!("received '{value}'");
25 | |             }
26 | |         };
   | |_________- the found `async` block
...
43 |           let futures = vec![tx1_fut, rx_fut, tx_fut];
   |                                       ^^^^^^ expected `async` block, found a different `async` block
   |
   = note: expected `async` block `{async block@src/main.rs:8:23: 20:10}`
              found `async` block `{async block@src/main.rs:22:22: 26:10}`
```

This error message is admittedly not the most helpful! It only tells us that it
expected one async block and found another—but why is it looking for the async
blocks that it names here, and why does it only reference them by where they
appear in the code?

One clue is the format of this message. Notice that it is exactly the same as if
we had tried to create a `Vec` with a a number and a string in it:

<!-- TODO: should this be a listing? -->

```rust
let a = 1;
let b = "Hello";
let vals = vec![a, b];
```

The output there would be:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:24
  |
4 |     let vals = vec![a, b];
  |                        ^ expected integer, found `&str`
```

Saying “expected *something*, found *something else*” is Rust’s standard format
for telling us about a type mismatch. As we saw with vectors in [Using an Enum
to Store Multiple Types][collections] back in Chapter 8, we need the type of
each item in a collection to be the same—and `tx1_fut`, `rx_fut`, and `tx_fut`
do not have the same type.

The underlying issue here is what we learned in the previous section: async
blocks compile to anonymous futures. Under the hood, there is a data structure
corresponding to each of these blocks, and it has its own unique type. This
might be surprising. After all, none of them returns anything, so the `Future`
type in each case is `Future<Output = ()>`. However, `Future` is a trait, not a
concrete type. The actual types here are invisible from our point of view as the
person writing the code.

In Chapter 8, we discussed one way to include multiple types in a single vector:
using an enum to represent each of the different types which can appear in the
vector. We cannot do that here, though. For one thing, we do not even have a way
to name the different types, because they are anonymous. For another, the reason
we reached for a vector and `join_all` in the first place was to be able to work
with a dynamic collection of futures where we do not know what they will all be
until runtime.

To make this work, we need to use *trait objects*, just as we did for returning
different kinds of errors from the same function in [Returning Errors from the
run function][dyn] back in Chapter 12. Again, we will cover trait objects in
detail in Chapter 17. Here, it lets us treat each of the anonymous futures
produced by these types as interchangeable, since all of them by definition
implement the `Future` trait.

We can start by wrapping each of the futures in the `vec!` in a `Box::new()`.
Unfortunately, the initial way we might try this, as shown in Listing 17-TODO,
still does not compile.

<Listing number="17-TODO" caption="" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:here}}
```

</Listing>

In fact, we have the same basic error we did before, but we get one for both the
second and third `Box::new` calls, and we also get new errors referring to the
`Unpin` trait.

We can start by fixing the type error around the `Box::new` calls, by telling
the compiler explicitly that we want to use these types as trait objects. The
clearest way to do that here is by adding a type annotation to the declaration
of `futures`, as we see in Listing 17-TODO. The type we have to write here is a
little involved, so let’s walk through each part of it.

- The innermost type is the future itself. We note explicitly that it the output
  of the future is the unit type `()` by writing `Future<Output = ()>`.
- Then we annotate the trait with `dyn` to mark it as dynamic.
- The entire trait is wrapped in a `Box`.
- Finally, we state explicitly that `futures` is a `Vec` containing these items.

<Listing number="17-TODO" caption="" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:here}}
```

</Listing>

That already made a big difference. Now when we run the compiler, we only have
the errors mentioning `Unpin`, each of which is a variation on this same output:

<!-- TODO: compiler output listing for the listing -->

```text
error[E0277]: `{async block@src/main.rs:8:23: 20:10}` cannot be unpinned
  --> src/main.rs:46:33
   |
46 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:8:23: 20:10}`, which is required by `Box<{async block@src/main.rs:8:23: 20:10}>: Future`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<{async block@src/main.rs:8:23: 20:10}>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> /Users/chris/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`
```

That is a *lot* to digest, so let’s pull it apart. The first part of the message
tell us that the first async block (`src/main.rs:8:23: 20:10`) does not
implement the `Unpin` trait, and suggests using `pin!` or `Box::pin` to resolve
it. The rest of the message tells us *why* that is required: the `JoinAll`
struct, which is itself a `Future`, is also generic over a `Future`, and
`Future` itself requires the `Unpin` trait. Understanding this error means we
need to dive into a little more of how the `Future` type actually works, in
particular the idea of *pinning*.

### Pinning and the Pin and Unpin Traits

When we introduced the `Future` trait in the previous chapter, we saw that the
definition of its `poll` method has an unusual way of specifying the `self`
parameter. To review, here is the full definition of `Future`:

```rust
pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

We have not seen a method definition like this before, where `self` has a type
annotation rather than simply being named like `self`, `mut self`, `&self`, or
`&mut self`. This syntax means that the method can only be called when the
instance of the type which implements `Future` is behind a `Pin` pointer type.
This syntax is not specific to `Pin`; it also works with `Box` and other smart
pointer types, and we will see it again in Chapter 18.

Here, the signature tells us that if we want to poll a future to check whether
it is `Pending` or `Ready(Output)`, the type which implements `Future` has to be
behind a `Pin` smart pointer type. Recalling that `.await` is implemented in
terms of calls to `poll()`, this starts to explain the error message we saw
above—but that was in terms of `Unpin`, not `Pin`. So what exactly are `Pin` and
`Unpin`, how do they relate, and why does `Future` need `self` to be in a `Pin`
type to call `poll`?

<!-- TODO: keep going here: define `Pin`. -->

<!--

  - Pin pins the thing behind a pointer (just using the Rust type system! No
    compiler magic required)
  - The reason is what async compiles to. Refer to the discussion of state

 -->

In [Futures and Syntax: What Are Futures][what-are-futures], we described how
a series of await points in a future get compiled into a state machine—and noted
how the compiler helps make sure that state machine follows all of Rust’s normal
rules around safety, including borrowing and ownership. Consider code like this:

<!-- TODO: extract to listing -->

```rust
async {
    let mut strings = vec![];

    let a = trpl::read_to_string("test-data/hello.txt").await.unwrap();
    strings.push(a.trim());

    let b = trpl::read_to_string("test-data/world.txt").await.unwrap();
    strings.push(b.trim());

    let combined = strings.join(" ");
    println!("{combined}");
}
```

If we think about the state machine that would get compiled to, it might be
something kind of like this:

```rust,ignore
enum AsyncStateMachine<'a> {
    FirstAwait(&'a mut Vec<String>),
    SecondAwait(&'a mut Vec<String>),
}
```

This could actually be fine, on its own—Rust would keep track of those mutable
references, and if we got something wrong, the borrow checker would tell us. It
gets a bit tricky, though, if we want to move around the future that corresponds
to that block. Remember, we could always do something like this:

```rust,ignore
let file_reads_future = async {
    // snip...
};

let some_other_future = async {
    // snip...
};

trpl::join(file_reads_future, some_other_future).await;
```

If we pass those futures into `join`, or return them from a function, or put
them in a data structure to keep track of for some reason, we move the state
machine as well, and that means the `Vec<String>` for the values we read in with
`trpl::read_to_string` moves. But the state machine Rust generated for us has
references to it. Since references point to the actual memory address of the
`Vec`, Rust needs some way to either update them so they are still valid after
the `Vec` moves, or it needs some way to keep `Vec` from getting moved around so
that the references do not need to be updated. Updating all the references to an
object every time it moves could be quite a lot of work for the compiler to add,
especially since there can be a whole web of references that need updating. On
the other hand, making sure the underlying item *does not move in memory* can be
“free” at runtime in exchange for keeping some promises at compile time. That is
where `Pin` and `Unpin` come in.

<!-- TODO: continue with `Pin` discussion! -->

> Note: This allows a whole class of complex types to be safe in Rust which are
> otherwise difficult to implement.

Remember that any time you write a future, a runtime is ultimately responsible
for executing it. That means that an async block might outlive the function
where you write it, the same way a closure can. <!-- TODO: connect this to the
need for pinning. -->

`Unpin` is a marker trait, like `Send` and `Sync`, which we saw in Chapter 16.
Recall that marker traits have no functionality of their own. They exist only to
tell the compiler that it is safe to use the type which implements a given trait
in certain context. Just like `Send` and `Sync`, the compiler implements `Unpin`
automatically for most types.

`Unpin`’s job is to tell the compiler that a given type does *not* need to
uphold any particular guarantees about whether the value in question can be
moved. For example, if a future

<!-- TODO: discussion of `Pin` -->

<!--
    The reason it gets weird to talk about is:

    - Nearly everything gets `Unpin` automatically because it is an auto trait.
    - Things which do not have to `impl !Unpin for TheType`.
    - But `!Unpin` actually means “must be pinned to be able to be used”.
    - So the actual situation is that the `Future` produced by an `async` block
      implements `!Unpin`, “not unpin”,

    My head hurts.
 -->

Now we know enough to understand the error message from above. The problem is
that the futures produced by an async block are *not* pinned by default.
Strictly: they implement `!Unpin` to opt out of being copyable by default the
way most types are. We need to pin them explicitly.

Now that we have an idea what that error message was telling us, we can finally
get our `join_all` call to compile! First, we need to explicitly annotate
`futures` as referring to a pinned `Box` of futures. Second, we actually need to
pin the futures, which we can do using the handy `Box::pin` API, which exists
for exactly this. Putting that together, we end up with the code in Listing
17-TODO.

<Listing number="17-TODO" caption="Using `Pin` and `Box::pin` to make the `Vec` type check" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

If we compile and run this, we finally get the output we hoped for:

<!-- TODO: listing for output.txt -->

```text
received 'hi'
received 'more'
received 'from'
received 'messages'
received 'the'
received 'for'
received 'future'
received 'you'
```

Phew!

There is a bit more we can explore here. For one thing, using `Pin<Box<T>>`
comes with a small amount of extra overhead from putting these futures on the
heap with `Box`—and we are only doing that to get the types to line up. We don’t
actually *need* the heap allocation, after all: these futures are local to this
particular function. As noted above, `Pin` is itself a smart pointer, so we can
get the benefit of having a single type in the `Vec`—the original reason we
reached for `Box`—without doing a heap allocation. We can use `Pin` directly
instead.

The `std::pin::pin` macro exists to do just that for values. However, we must
still be explicit about the type of the pinned reference; otherwise Rust will
still not know to interpret these as dynamic trait objects, which is what we
need them to be in the `Vec`. We therefore `pin!` each future when we define it,
and define `futures` as a `Vec` containing pinned mutable references to the
dynamic `Future` type.

<Listing number="17-TODO" caption="Using `Pin` directly with the `pin!` macro to avoid unnecessary heap allocations" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:here}}
```

</Listing>

This keeps everything on the stack, and that is a nice little performance win,
but it is still a lot of explicit types, which is quite unusual for Rust! There
is another problem, too. We got this far by ignoring the fact that we might have
different `Output` types. For example, in Listing 17-TODO, the anonymous future
type for `a` implements `Future<Output = u32>` and the anonymous future type for
`b` implements `Future<Output = &str>`.

<Listing number="17-TODO" caption="" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:here}}
```

</Listing>

We can use `trpl::join!` to await them together, since it accepts two different
future types, but we cannot use `trpl::join_all` with these futures, because we
will never be able to make them have the same type. (This is the same as working
with any other type in Rust, though: futures are not special, even though we
have some nice syntax for working with them, and that is a good thing!) We have
a basic tradeoff here: we can either deal with a dynamic number of futures with
`join_all`, as long as they all have the same type, or we can deal with a
static number of futures with `join!`, and so on,

<!--
    TODO: validate that this is, you know, true. It matches my own experience,
    but it is a fairly strong claim.
-->
In practice, you will usually work directly with `async` and `.await`, and only
as a secondary tool reach for the functions like `join` or `join_all`, or their
corresponding macro equivalents. These kinds of tools are really handy for
building frameworks, or especially when you are building a runtime itself.

[what-are-futures]: /ch17-01-futures-and-syntax.html#what-are-futures
