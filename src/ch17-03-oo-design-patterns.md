## Implementing an Object-Oriented Design Pattern

The *state pattern* is an object-oriented design pattern. The crux of the
pattern is that a value has some internal state, which is represented by a set
of *state objects*, and the value’s behavior changes based on the internal
state. The state objects share functionality: in Rust, of course, we use
structs and traits rather than objects and inheritance. Each state object is
responsible for its own behavior and for governing when it should change into
another state. The value that holds a state object knows nothing about the
different behavior of the states or when to transition between states.

Using the state pattern means when the business requirements of the program
change, we won’t need to change the code of the value holding the state or the
code that uses the value. We’ll only need to update the code inside one of the
state objects to change its rules or perhaps add more state objects. Let’s look
at an example of the state design pattern and how to use it in Rust.

We’ll implement a blog post workflow in an incremental way. The blog’s final
functionality will look like this:

1. A blog post starts as an empty draft.
2. When the draft is done, a review of the post is requested.
3. When the post is approved, it gets published.
4. Only published blog posts return content to print, so unapproved posts can’t
   accidentally be published.

Any other changes attempted on a post should have no effect. For example, if we
try to approve a draft blog post before we’ve requested a review, the post
should remain an unpublished draft.

Listing 17-11 shows this workflow in code form: this is an example usage of the
API we’ll implement in a library crate named `blog`. This won’t compile yet
because we haven’t implemented the `blog` crate yet.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:all}}
```

<span class="caption">Listing 17-11: Code that demonstrates the desired
behavior we want our `blog` crate to have</span>

We want to allow the user to create a new draft blog post with `Post::new`.
Then we want to allow text to be added to the blog post while it’s in the draft
state. If we try to get the post’s content immediately, before approval,
nothing should happen because the post is still a draft. We’ve added
`assert_eq!` in the code for demonstration purposes. An excellent unit test for
this would be to assert that a draft blog post returns an empty string from the
`content` method, but we’re not going to write tests for this example.

Next, we want to enable a request for a review of the post, and we want
`content` to return an empty string while waiting for the review. When the post
receives approval, it should get published, meaning the text of the post will
be returned when `content` is called.

Notice that the only type we’re interacting with from the crate is the `Post`
type. This type will use the state pattern and will hold a value that will be
one of three state objects representing the various states a post can be
in—draft, waiting for review, or published. Changing from one state to another
will be managed internally within the `Post` type. The states change in
response to the methods called by our library’s users on the `Post` instance,
but they don’t have to manage the state changes directly. Also, users can’t
make a mistake with the states, like publishing a post before it’s reviewed.

### Defining `Post` and Creating a New Instance in the Draft State

Let’s get started on the implementation of the library! We know we need a
public `Post` struct that holds some content, so we’ll start with the
definition of the struct and an associated public `new` function to create an
instance of `Post`, as shown in Listing 17-12. We’ll also make a private
`State` trait. Then `Post` will hold a trait object of `Box<dyn State>`
inside an `Option<T>` in a private field named `state`. You’ll see why the
`Option<T>` is necessary in a bit.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-12/src/lib.rs}}
```

<span class="caption">Listing 17-12: Definition of a `Post` struct and a `new`
function that creates a new `Post` instance, a `State` trait, and a `Draft`
struct</span>

The `State` trait defines the behavior shared by different post states, and the
`Draft`, `PendingReview`, and `Published` states will all implement the `State`
trait. For now, the trait doesn’t have any methods, and we’ll start by defining
just the `Draft` state because that is the state we want a post to start in.

When we create a new `Post`, we set its `state` field to a `Some` value that
holds a `Box`. This `Box` points to a new instance of the `Draft` struct. This
ensures whenever we create a new instance of `Post`, it will start out as a
draft. Because the `state` field of `Post` is private, there is no way to
create a `Post` in any other state! In the `Post::new` function, we set the
`content` field to a new, empty `String`.

### Storing the Text of the Post Content

Listing 17-11 showed that we want to be able to call a method named
`add_text` and pass it a `&str` that is then added to the text content of the
blog post. We implement this as a method rather than exposing the `content`
field as `pub`. This means we can implement a method later that will control
how the `content` field’s data is read. The `add_text` method is pretty
straightforward, so let’s add the implementation in Listing 17-13 to the `impl
Post` block:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-13/src/lib.rs:here}}
```

<span class="caption">Listing 17-13: Implementing the `add_text` method to add
text to a post’s `content`</span>

The `add_text` method takes a mutable reference to `self`, because we’re
changing the `Post` instance that we’re calling `add_text` on. We then call
`push_str` on the `String` in `content` and pass the `text` argument to add to
the saved `content`. This behavior doesn’t depend on the state the post is in,
so it’s not part of the state pattern. The `add_text` method doesn’t interact
with the `state` field at all, but it is part of the behavior we want to
support.

### Ensuring the Content of a Draft Post Is Empty

Even after we’ve called `add_text` and added some content to our post, we still
want the `content` method to return an empty string slice because the post is
still in the draft state, as shown on line 7 of Listing 17-11. For now, let’s
implement the `content` method with the simplest thing that will fulfill this
requirement: always returning an empty string slice. We’ll change this later
once we implement the ability to change a post’s state so it can be published.
So far, posts can only be in the draft state, so the post content should always
be empty. Listing 17-14 shows this placeholder implementation:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-14/src/lib.rs:here}}
```

<span class="caption">Listing 17-14: Adding a placeholder implementation for
the `content` method on `Post` that always returns an empty string slice</span>

With this added `content` method, everything in Listing 17-11 up to line 7
works as intended.

### Requesting a Review of the Post Changes Its State

Next, we need to add functionality to request a review of a post, which should
change its state from `Draft` to `PendingReview`. Listing 17-15 shows this code:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-15/src/lib.rs:here}}
```

<span class="caption">Listing 17-15: Implementing `request_review` methods on
`Post` and the `State` trait</span>

We give `Post` a public method named `request_review` that will take a mutable
reference to `self`. Then we call an internal `request_review` method on the
current state of `Post`, and this second `request_review` method consumes the
current state and returns a new state.

We’ve added the `request_review` method to the `State` trait; all types that
implement the trait will now need to implement the `request_review` method.
Note that rather than having `self`, `&self`, or `&mut self` as the first
parameter of the method, we have `self: Box<Self>`. This syntax means the
method is only valid when called on a `Box` holding the type. This syntax takes
ownership of `Box<Self>`, invalidating the old state so the state value of the
`Post` can transform into a new state.

To consume the old state, the `request_review` method needs to take ownership
of the state value. This is where the `Option` in the `state` field of `Post`
comes in: we call the `take` method to take the `Some` value out of the `state`
field and leave a `None` in its place, because Rust doesn’t let us have
unpopulated fields in structs. This lets us move the `state` value out of
`Post` rather than borrowing it. Then we’ll set the post’s `state` value to the
result of this operation.

We need to set `state` to `None` temporarily rather than setting it directly
with code like `self.state = self.state.request_review();` to get ownership of
the `state` value. This ensures `Post` can’t use the old `state` value after
we’ve transformed it into a new state.

The `request_review` method on `Draft` needs to return a new, boxed instance of
a new `PendingReview` struct, which represents the state when a post is waiting
for a review. The `PendingReview` struct also implements the `request_review`
method but doesn’t do any transformations. Rather, it returns itself, because
when we request a review on a post already in the `PendingReview` state, it
should stay in the `PendingReview` state.

Now we can start seeing the advantages of the state pattern: the
`request_review` method on `Post` is the same no matter its `state` value. Each
state is responsible for its own rules.

We’ll leave the `content` method on `Post` as is, returning an empty string
slice. We can now have a `Post` in the `PendingReview` state as well as in the
`Draft` state, but we want the same behavior in the `PendingReview` state.
Listing 17-11 now works up to line 10!

### Adding the `approve` Method that Changes the Behavior of `content`

The `approve` method will be similar to the `request_review` method: it will
set `state` to the value that the current state says it should have when that
state is approved, as shown in Listing 17-16:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-16/src/lib.rs:here}}
```

<span class="caption">Listing 17-16: Implementing the `approve` method on
`Post` and the `State` trait</span>

We add the `approve` method to the `State` trait and add a new struct that
implements `State`, the `Published` state.

Similar to `request_review`, if we call the `approve` method on a `Draft`, it
will have no effect because it will return `self`. When we call `approve` on
`PendingReview`, it returns a new, boxed instance of the `Published` struct.
The `Published` struct implements the `State` trait, and for both the
`request_review` method and the `approve` method, it returns itself, because
the post should stay in the `Published` state in those cases.

Now we need to update the `content` method on `Post`: if the state is
`Published`, we want to return the value in the post’s `content` field;
otherwise, we want to return an empty string slice, as shown in Listing 17-17:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-17/src/lib.rs:here}}
```

<span class="caption">Listing 17-17: Updating the `content` method on `Post` to
delegate to a `content` method on `State`</span>

Because the goal is to keep all these rules inside the structs that implement
`State`, we call a `content` method on the value in `state` and pass the post
instance (that is, `self`) as an argument. Then we return the value that is
returned from using the `content` method on the `state` value.

We call the `as_ref` method on the `Option` because we want a reference to the
value inside the `Option` rather than ownership of the value. Because `state`
is an `Option<Box<dyn State>>`, when we call `as_ref`, an `Option<&Box<dyn State>>` is
returned. If we didn’t call `as_ref`, we would get an error because we can’t
move `state` out of the borrowed `&self` of the function parameter.

We then call the `unwrap` method, which we know will never panic, because we
know the methods on `Post` ensure that `state` will always contain a `Some`
value when those methods are done. This is one of the cases we talked about in
the [“Cases In Which You Have More Information Than the
Compiler”][more-info-than-rustc]<!-- ignore --> section of Chapter 9 when we
know that a `None` value is never possible, even though the compiler isn’t able
to understand that.

At this point, when we call `content` on the `&Box<dyn State>`, deref coercion will
take effect on the `&` and the `Box` so the `content` method will ultimately be
called on the type that implements the `State` trait. That means we need to add
`content` to the `State` trait definition, and that is where we’ll put the
logic for what content to return depending on which state we have, as shown in
Listing 17-18:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-18/src/lib.rs:here}}
```

<span class="caption">Listing 17-18: Adding the `content` method to the `State`
trait</span>

We add a default implementation for the `content` method that returns an empty
string slice. That means we don’t need to implement `content` on the `Draft`
and `PendingReview` structs. The `Published` struct will override the `content`
method and return the value in `post.content`.

Note that we need lifetime annotations on this method, as we discussed in
Chapter 10. We’re taking a reference to a `post` as an argument and returning a
reference to part of that `post`, so the lifetime of the returned reference is
related to the lifetime of the `post` argument.

And we’re done—all of Listing 17-11 now works! We’ve implemented the state
pattern with the rules of the blog post workflow. The logic related to the
rules lives in the state objects rather than being scattered throughout `Post`.

### Trade-offs of the State Pattern

We’ve shown that Rust is capable of implementing the object-oriented state
pattern to encapsulate the different kinds of behavior a post should have in
each state. The methods on `Post` know nothing about the various behaviors. The
way we organized the code, we have to look in only one place to know the
different ways a published post can behave: the implementation of the `State`
trait on the `Published` struct.

If we were to create an alternative implementation that didn’t use the state
pattern, we might instead use `match` expressions in the methods on `Post` or
even in the `main` code that checks the state of the post and changes behavior
in those places. That would mean we would have to look in several places to
understand all the implications of a post being in the published state! This
would only increase the more states we added: each of those `match` expressions
would need another arm.

With the state pattern, the `Post` methods and the places we use `Post` don’t
need `match` expressions, and to add a new state, we would only need to add a
new struct and implement the trait methods on that one struct.

The implementation using the state pattern is easy to extend to add more
functionality. To see the simplicity of maintaining code that uses the state
pattern, try a few of these suggestions:

* Add a `reject` method that changes the post’s state from `PendingReview` back
  to `Draft`.
* Require two calls to `approve` before the state can be changed to `Published`.
* Allow users to add text content only when a post is in the `Draft` state.
  Hint: have the state object responsible for what might change about the
  content but not responsible for modifying the `Post`.

One downside of the state pattern is that, because the states implement the
transitions between states, some of the states are coupled to each other. If we
add another state between `PendingReview` and `Published`, such as `Scheduled`,
we would have to change the code in `PendingReview` to transition to
`Scheduled` instead. It would be less work if `PendingReview` didn’t need to
change with the addition of a new state, but that would mean switching to
another design pattern.

Another downside is that we’ve duplicated some logic. To eliminate some of the
duplication, we might try to make default implementations for the
`request_review` and `approve` methods on the `State` trait that return `self`;
however, this would violate object safety, because the trait doesn’t know what
the concrete `self` will be exactly. We want to be able to use `State` as a
trait object, so we need its methods to be object safe.

Other duplication includes the similar implementations of the `request_review`
and `approve` methods on `Post`. Both methods delegate to the implementation of
the same method on the value in the `state` field of `Option` and set the new
value of the `state` field to the result. If we had a lot of methods on `Post`
that followed this pattern, we might consider defining a macro to eliminate the
repetition (see the [“Macros”][macros]<!-- ignore --> section in Chapter 19).

By implementing the state pattern exactly as it’s defined for object-oriented
languages, we’re not taking as full advantage of Rust’s strengths as we could.
Let’s look at some changes we can make to the `blog` crate that can make
invalid states and transitions into compile time errors.

#### Encoding States and Behavior as Types

We’ll show you how to rethink the state pattern to get a different set of
trade-offs. Rather than encapsulating the states and transitions completely so
outside code has no knowledge of them, we’ll encode the states into different
types. Consequently, Rust’s type checking system will prevent attempts to use
draft posts where only published posts are allowed by issuing a compiler error.

Let’s consider the first part of `main` in Listing 17-11:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:here}}
```

We still enable the creation of new posts in the draft state using `Post::new`
and the ability to add text to the post’s content. But instead of having a
`content` method on a draft post that returns an empty string, we’ll make it so
draft posts don’t have the `content` method at all. That way, if we try to get
a draft post’s content, we’ll get a compiler error telling us the method
doesn’t exist. As a result, it will be impossible for us to accidentally
display draft post content in production, because that code won’t even compile.
Listing 17-19 shows the definition of a `Post` struct and a `DraftPost` struct,
as well as methods on each:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-19/src/lib.rs}}
```

<span class="caption">Listing 17-19: A `Post` with a `content` method and a
`DraftPost` without a `content` method</span>

Both the `Post` and `DraftPost` structs have a private `content` field that
stores the blog post text. The structs no longer have the `state` field because
we’re moving the encoding of the state to the types of the structs. The `Post`
struct will represent a published post, and it has a `content` method that
returns the `content`.

We still have a `Post::new` function, but instead of returning an instance of
`Post`, it returns an instance of `DraftPost`. Because `content` is private
and there aren’t any functions that return `Post`, it’s not possible to create
an instance of `Post` right now.

The `DraftPost` struct has an `add_text` method, so we can add text to
`content` as before, but note that `DraftPost` does not have a `content` method
defined! So now the program ensures all posts start as draft posts, and draft
posts don’t have their content available for display. Any attempt to get around
these constraints will result in a compiler error.

#### Implementing Transitions as Transformations into Different Types

So how do we get a published post? We want to enforce the rule that a draft
post has to be reviewed and approved before it can be published. A post in the
pending review state should still not display any content. Let’s implement
these constraints by adding another struct, `PendingReviewPost`, defining the
`request_review` method on `DraftPost` to return a `PendingReviewPost`, and
defining an `approve` method on `PendingReviewPost` to return a `Post`, as
shown in Listing 17-20:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-20/src/lib.rs:here}}
```

<span class="caption">Listing 17-20: A `PendingReviewPost` that gets created by
calling `request_review` on `DraftPost` and an `approve` method that turns a
`PendingReviewPost` into a published `Post`</span>

The `request_review` and `approve` methods take ownership of `self`, thus
consuming the `DraftPost` and `PendingReviewPost` instances and transforming
them into a `PendingReviewPost` and a published `Post`, respectively. This way,
we won’t have any lingering `DraftPost` instances after we’ve called
`request_review` on them, and so forth. The `PendingReviewPost` struct doesn’t
have a `content` method defined on it, so attempting to read its content
results in a compiler error, as with `DraftPost`. Because the only way to get a
published `Post` instance that does have a `content` method defined is to call
the `approve` method on a `PendingReviewPost`, and the only way to get a
`PendingReviewPost` is to call the `request_review` method on a `DraftPost`,
we’ve now encoded the blog post workflow into the type system.

But we also have to make some small changes to `main`. The `request_review` and
`approve` methods return new instances rather than modifying the struct they’re
called on, so we need to add more `let post =` shadowing assignments to save
the returned instances. We also can’t have the assertions about the draft and
pending review posts' contents be empty strings, nor do we need them: we can’t
compile code that tries to use the content of posts in those states any longer.
The updated code in `main` is shown in Listing 17-21:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-21/src/main.rs}}
```

<span class="caption">Listing 17-21: Modifications to `main` to use the new
implementation of the blog post workflow</span>

The changes we needed to make to `main` to reassign `post` mean that this
implementation doesn’t quite follow the object-oriented state pattern anymore:
the transformations between the states are no longer encapsulated entirely
within the `Post` implementation. However, our gain is that invalid states are
now impossible because of the type system and the type checking that happens at
compile time! This ensures that certain bugs, such as display of the content of
an unpublished post, will be discovered before they make it to production.

Try the tasks suggested for additional requirements that we mentioned at the
start of this section on the `blog` crate as it is after Listing 17-20 to see
what you think about the design of this version of the code. Note that some of
the tasks might be completed already in this design.

We’ve seen that even though Rust is capable of implementing object-oriented
design patterns, other patterns, such as encoding state into the type system,
are also available in Rust. These patterns have different trade-offs. Although
you might be very familiar with object-oriented patterns, rethinking the
problem to take advantage of Rust’s features can provide benefits, such as
preventing some bugs at compile time. Object-oriented patterns won’t always be
the best solution in Rust due to certain features, like ownership, that
object-oriented languages don’t have.

## Summary

No matter whether or not you think Rust is an object-oriented language after
reading this chapter, you now know that you can use trait objects to get some
object-oriented features in Rust. Dynamic dispatch can give your code some
flexibility in exchange for a bit of runtime performance. You can use this
flexibility to implement object-oriented patterns that can help your code’s
maintainability. Rust also has other features, like ownership, that
object-oriented languages don’t have. An object-oriented pattern won’t always
be the best way to take advantage of Rust’s strengths, but is an available
option.

Next, we’ll look at patterns, which are another of Rust’s features that enable
lots of flexibility. We’ve looked at them briefly throughout the book but
haven’t seen their full capability yet. Let’s go!

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler
[macros]: ch19-06-macros.html#macros
