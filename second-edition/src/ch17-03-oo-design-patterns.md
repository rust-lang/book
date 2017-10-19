## Implementing an Object-Oriented Design Pattern

The *state pattern* is an object-oriented design pattern. The crux of the
pattern is that a value has some internal state, represented by a set of *state
objects*, and the value’s behavior changes based on the internal state. The
state objects share functionality--in Rust, of course, we use structs and
traits rather than objects and inheritance. Each state object representing the
state is responsible for its own behavior and for governing when it should
change into another state. The value that holds a state object knows nothing
about the different behavior of the states or when to transition between states.

<!-- Below -- requirements for what, for what we need the value for? -->
<!-- I've clarified /Carol -->

Using the state pattern means when the business requirements of the program
change, we won’t need to change the code of the value holding the state or the
code that uses the value. We’ll only need to update the code inside one of the
state objects to change its rules, or perhaps add more state objects. Let’s
look at an example of the state design pattern and how to use it in Rust.

To explore this idea, we’ll implement a blog post workflow in an incremental
way. The blog’s final functionality will look like this:

1. A blog post starts as an empty draft.
2. Once the draft is done, a review of the post is requested.
3. Once the post is approved, it gets published.
4. Only published blog posts return content to print, so unapproved posts can’t
   accidentally get published.

Any other changes attempted on a post should have no effect. For example, if we
try to approve a draft blog post before we’ve requested a review, the post
should stay an unpublished draft.

Listing 17-11 shows this workflow in code form. This is an example usage of the
API we’re going to implement in a library crate named `blog`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

<span class="caption">Listing 17-11: Code that demonstrates the desired
behavior we want our `blog` crate to have</span>

We want to allow the user to create a new draft blog post with `Post::new`.
Then, we want to allow text to be added to the blog post while it’s in the
draft state. If we try to get the post’s content immediately, before
approval, nothing should happen because the post is still a draft. We’ve added
an `assert_eq!` here for demonstration purposes. An excellent unit test for
this would be to assert that a draft blog post returns an empty string from the
`content` method, but we’re not going to write tests for this example.

Next, we want to enable a request for a review of the post, and we want
`content` to return an empty string while waiting for the review. Lastly, when
the post receives approval, it should get published, meaning the text of the
post will be returned when `content` is called.

<!-- Below -- so this is where we'll implement the state pattern? If so, can
you make that explicit, just to be clear! I've added some text to the second
line, not sure if that's accurate though -->
<!-- Yes, the state pattern will be implemented within the `Post` type. I've
tweaked the wording a bit but you've pretty much got it! /Carol-->

Notice that the only type we’re interacting with from the crate is the `Post`
type. This type will use the state pattern and will hold a value that will be
one of three state objects representing the various states a post can be
in---draft, waiting for review, or published. Changing from one state to
another will be managed internally within the `Post` type. The states change in
response to the methods users of our library call on the `Post` instance, but
they don’t have to manage the state changes directly. This also means users
can’t make a mistake with the states, like publishing a post before it is
reviewed.

### Defining `Post` and Creating a New Instance in the Draft State

Let’s get started on the implementation of the library! We know we need a
public `Post` struct that holds some content, so let’s start with the
definition of the struct and an associated public `new` function to create an
instance of `Post`, as shown in Listing 17-12. We’ll also make a private
`State` trait. Then `Post` will hold a trait object of `Box<State>` inside an
`Option` in a private field named `state`. We’ll see why the `Option` is
necessary in a bit.

The `State` trait defines the behavior shared by different post states, and the
`Draft`, `PendingReview`, and `Published` states will all implement the `State`
trait. For now, the trait does not have any methods, and we’re going to start
by defining just the `Draft` state since that’s the state we want a post to
start in:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

<span class="caption">Listing 17-12: Definition of a `Post` struct and a `new`
function that creates a new `Post` instance, a `State` trait, and a `Draft`
struct</span>

When we create a new `Post`, we set its `state` field to a `Some` value that
holds a `Box`. This `Box` points to a new instance of the `Draft` struct. This
ensures whenever we create a new instance of `Post`, it’ll start out as a
draft. Because the `state` field of `Post` is private, there’s no way to create
a `Post` in any other state!

### Storing the Text of the Post Content

In the `Post::new` function, we set the `content` field to a new, empty
`String`. Listing 17-11 showed that we want to be able to call a method named
`add_text` and pass it a `&str` that’s then added to the text content of the
blog post. We implement this as a method rather than exposing the `content`
field as `pub`. This means we can implement a method later that will control
how the `content` field’s data is read. The `add_text` method is pretty
straightforward, so let’s add the implementation in Listing 17-13 to the `impl
Post` block:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
impl Post {
    // ...snip...
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

<span class="caption">Listing 17-13: Implementing the `add_text` method to add
text to a post’s `content`</span>

`add_text` takes a mutable reference to `self`, since we’re changing the `Post`
instance that we’re calling `add_text` on. We then call `push_str` on the
`String` in `content` and pass the `text` argument to add to the saved
`content`. This behavior doesn’t depend on the state the post is in so it’s not
part of the state pattern. The `add_text` method doesn’t interact with the
`state` field at all, but it is part of the behavior we want to support.

### Ensuring the Content of a Draft Post is Empty

Even after we’ve called `add_text` and added some content to our post, we still
want the `content` method to return an empty string slice since the post is
still in the draft state, as shown on line 8 of Listing 17-11. For now, let’s
implement the `content` method with the simplest thing that will fulfill this
requirement: always returning an empty string slice. We’re going to change this
later once we implement the ability to change a post’s state so it can be
published. So far, though, posts can only be in the draft state, so the post
content should always be empty. Listing 17-14 shows this placeholder
implementation:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
impl Post {
    // ...snip...
    pub fn content(&self) -> &str {
        ""
    }
}
```

<span class="caption">Listing 17-14: Adding a placeholder implementation for
the `content` method on `Post` that always returns an empty string slice</span>

With this added `content` method, everything in Listing 17-11 up to line 8
works as we intend.

### Requesting a Review of the Post Changes its State

Next up we need to add functionality to request a review of a post, which
should change its state from `Draft` to `PendingReview`. We want to give `Post`
a public method named `request_review` that will take a mutable reference to
`self`. Then we’re going to call an internal `request_review` method on the
current state of `Post`, and this second `request_review` method will consume
the current state and return a new state. Listing 17-15 shows this code:

<!-- NOTE TO DE/AU: We might want to move this explanation to after the code if
you want to add wingdings, we can see once we transfer it to Word -->
<!-- I decided to move some of this explanation after the code for this reason
and because we got some questions about this example that I wanted to expand
upon /Carol -->

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // ...snip...
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
}
```

<span class="caption">Listing 17-15: Implementing `request_review` methods on
`Post` and the `State` trait</span>

We’ve added the `request_review` method to the `State` trait; all types that
implement the trait will now need to implement the `request_review` method.
Note that rather than having `self`, `&self`, or `&mut self` as the first
parameter of the method, we have `self: Box<Self>`. This syntax means the
method is only valid when called on a `Box` holding the type. This syntax takes
ownership of `Box<Self>`, invalidating the old state so that the state value of
the `Post` can transform itself into a new state.

<!-- Above -- so Post can transform, or so Draft can transform? -->
<!-- Technically it's so the Draft value can transform into another value,
which changes the state of Post-- I've tried to clarify. /Carol -->

To consume the old state, the `request_review` method needs to take ownership
of the state value. This is where the `Option` in the `state` field of `Post`
comes in: we call the `take` method to take the `Some` value out of the `state`
field and leave a `None` in its place, since Rust doesn’t let us have
unpopulated fields in structs. This lets us move the `state` value out of
`Post` rather than borrowing it. Then we’ll set the post’s `state` value to the
result of this operation.

We need to set `state` to `None` temporarily, rather than code like `self.state
= self.state.request_review();` that would set the `state` field directly, to
get ownership of the `state` value. This ensures `Post` can’t use the old
`state` value after we’ve transformed it into a new state.

The `request_review` method on `Draft` needs to return a new, boxed instance of
a new `PendingReview` struct, which represents the state when a post is waiting
for a review. The `PendingReview` struct also implements the `request_review`
method, but doesn’t do any transformations. Rather, it returns itself, since
when we request a review on a post already in the `PendingReview` state, it
should stay in the `PendingReview` state.

Now we can start seeing the advantages of the state pattern: the
`request_review` method on `Post` is the same no matter its `state` value. Each
state is responsible for its own rules.

We’re going to leave the `content` method on `Post` as it is, returning an
empty string slice. We can now have a `Post` in the `PendingReview` state as
well as the `Draft` state, but we want the same behavior in the `PendingReview`
state. Listing 17-11 now works up until line 11!

### Adding the `approve` Method that Changes the Behavior of `content`

The `approve` method will be similar to the `request_review` method: it will
set `state` to the value that the current state says it should have when that
state is approved, shown in Listing 17-16.

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // ...snip...
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
#     fn request_review(self: Box<Self>) -> Box<State> {
#         Box::new(PendingReview {})
#     }
#
    // ...snip...
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
#     fn request_review(self: Box<Self>) -> Box<State> {
#         self
#     }
#
    // ...snip...
    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}
```

<span class="caption">Listing 17-16: Implementing the `approve` method on
`Post` and the `State` trait</span>

We add the `approve` method to the `State` trait, and add a new struct that
implements `State`, the `Published` state.

Similar to `request_review`, if we call the `approve` method on a `Draft`, it
will have no effect since it will return `self`. When we call `approve` on
`PendingReview`, it returns a new, boxed instance of the `Published` struct.
The `Published` struct implements the `State` trait, and for both the
`request_review` method and the `approve` method, it returns itself, since the
post should stay in the `Published` state in those cases.

Now to update the `content` method on `Post`: if the state is `Published` we
want to return the value in the post’s `content` field; otherwise we want to
return an empty string slice, as shown in Listing 17-17:

<span class="filename">Filename: src/lib.rs</span>

```rust
# trait State {
#     fn content<'a>(&self, post: &'a Post) -> &'a str;
# }
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // ...snip...
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    // ...snip...
}
```

<span class="caption">Listing 17-17: Updating the `content` method on `Post` to
delegate to a `content` method on `State`</span>

Because the goal is to keep all these rules inside the structs that implement
`State`, we call a `content` method on the value in `state` and pass the post
instance (that is, `self`) as an argument. Then we return the value that’s
returned from using the `content` method on the `state` value.

We call the `as_ref` method on the `Option` because we want a reference to the
value inside the `Option` rather than ownership of it. Because `state` is an
`Option<Box<State>>`, calling `as_ref` returns an `Option<&Box<State>>`. If we
didn’t call `as_ref`, we’d get an error because we can’t move `state` out of
the borrowed `&self` of the function parameter.

We’re then calling the `unwrap` method, which we know will never panic, because
we know the methods on `Post` ensure that `state` will always contain a `Some`
value when those methods are done. This is one of the cases we talked about in
Chapter 12 when we know that a `None` value is never possible, even though the
compiler isn’t able to understand that.

So then we have a `&Box<State>`, and when we call the `content` on it, deref
coercion will take effect on the `&` and the `Box` so that the `content` method
will ultimately be called on the type that implements the `State` trait.

That means we need to add `content` to the `State` trait definition, and that’s
where We’ll put the logic for what content to return depending on which state
we have, as shown in Listing 17-18:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String
# }
trait State {
    // ...snip...
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// ...snip...
struct Published {}

impl State for Published {
    // ...snip...
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

<span class="caption">Listing 17-18: Adding the `content` method to the `State`
trait</span>

We add a default implementation for the `content` method that returns an empty
string slice. That means we don’t need to implement `content` on the `Draft`
and `PendingReview` structs. The `Published` struct will override the `content`
method and return the value in `post.content`.

Note that we need lifetime annotations on this method, like we discussed in
Chapter 10. We’re taking a reference to a `post` as an argument, and returning
a reference to part of that `post`, so the lifetime of the returned reference
is related to the lifetime of the `post` argument.

<!-- Is this it finished, without the touch up we make to get rid of the empty
string? That's pretty awesome coding, maybe give it some ceremony here. Does
all of 17-11 now work? -->
<!-- Yep! Good point, so added! /Carol -->

And we’re done-- all of Listing 17-11 now works! We’ve implemented the state
pattern with the rules of the blog post workflow. The logic around the rules
lives in the state objects rather than scattered throughout `Post`.

### Tradeoffs of the State Pattern

We’ve shown that Rust is capable of implementing the object-oriented state
pattern to encapsulate the different kinds of behavior a post should have in
each state. The methods on `Post` know nothing about the different kinds of
behavior. The way this code is organized, we only have to look in one place to
know the different ways a published post can behave: the implementation of the
`State` trait on the `Published` struct.

If we were to create an alternative implementation that didn’t use the state
pattern we might use `match` statements in the methods on `Post`, or even in
the `main` code that checks the state of the post and changes behavior in those
places instead. That would mean we’d have to look in a lot of places to
understand all the implications of a post being in the published state! This
would only increase the more states we added: each of those `match` statements
would need another arm.

With the state pattern, the `Post` methods and the places we use `Post` don’t
need `match` statements, and to add a new state we would only need to add a new
`struct` and implement the trait methods on that one struct.

This implementation is easy to extend to add more functionality. To see the
simplicity of maintaining code that uses this patterns, try out a few of these
suggestions:

- Allow users to add text content only when a post is in the `Draft` state
- Add a `reject` method that changes the post’s state from `PendingReview` back
  to `Draft`
- Require two calls to `approve` before the state can be changed to `Published`

One downside of the state pattern is that, because the states implement the
transitions between states, some of the states are coupled to each other. If we
add another state between `PendingReview` and `Published`, such as `Scheduled`,
we would have to change the code in `PendingReview` to transition to
`Scheduled` instead. It would be less work if `PendingReview` wouldn’t need to
change with the addition of a new state, but that would mean switching to
another design pattern.

Another downside is that we find ourselves with a few bits of duplicated logic.
To eliminate this, we might try to make default implementations for the
`request_review` and `approve` methods on the `State` trait that return `self`,
but this would violate object safety, since the trait doesn’t know what the
concrete `self` will be exactly. We want to be able to use `State` as a trait
object, so we need its methods to be object safe.

The other duplication is the similar implementations of the `request_review`
and `approve` methods on `Post`. Both methods delegate to the implementation of
the same method on the value in the `state` field of `Option`, and set the new
value of the `state` field to the result. If we had a lot of methods on `Post`
that followed this pattern, we might consider defining a macro to eliminate the
repetition (see Appendix E on macros).

By implementing this pattern exactly as it’s defined for object-oriented
languages, we’re not taking full advantage of Rust’s strengths as much as we
could. Let’s take a look at some changes we can make to this code that can make
invalid states and transitions into compile time errors.

#### Encoding States and Behavior as Types

We’re going to show how to rethink the state pattern to get a different set of
tradeoffs. Rather than encapsulating the states and transitions completely so
that outside code has no knowledge of them, we’re going to encode the states
into different types. Like this, Rust’s type checking system will make attempts
to use draft posts where only published posts are allowed into a compiler error.

Let’s consider the first part of `main` from Listing 17-11:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

We still enable the creation of new posts in the draft state using `Post::new`,
and the ability to add text to the post’s content. But instead of having a
`content` method on a draft post that returns an empty string, we’ll make it so
that draft posts don’t have the `content` method at all. That way, if we try to
get a draft post’s content, we’ll get a compiler error telling us the method
doesn’t exist. This will make it impossible for us to accidentally display
draft post content in production, since that code won’t even compile. Listing
17-19 shows the definition of a `Post` struct, a `DraftPost` struct, and
methods on each:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
       &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

<span class="caption">Listing 17-19: A `Post` with a `content` method and a
`DraftPost` without a `content` method</span>

Both the `Post` and `DraftPost` structs have a private `content` field that
stores the blog post text. The structs no longer have the `state` field since
we’re moving the encoding of the state to the types of the structs. `Post` will
represent a published post, and it has a `content` method that returns the
`content`.

We still have a `Post::new` function, but instead of returning an instance of
`Post`, it returns an instance of `DraftPost`. Because `content` is private,
and there aren’t any functions that return `Post`, it’s not possible to create
an instance of `Post` right now.

`DraftPost` has an `add_text` method so we can add text to `content` as before,
but note that `DraftPost` does not have a `content` method defined! So now the
program ensures all posts start as draft posts, and draft posts don’t have
their content available for display. Any attempt to get around these
constraints will result in a compiler error.

#### Implementing Transitions as Transformations into Different Types

So how do we get a published post then? We want to enforce the rule that a
draft post has to be reviewed and approved before it can be published. A post
in the pending review state should still not display any content. Let’s
implement these constraints by adding another struct, `PendingReviewPost`,
defining the `request_review` method on `DraftPost` to return a
`PendingReviewPost`, and defining an `approve` method on `PendingReviewPost` to
return a `Post` as shown in Listing 17-20:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
# pub struct DraftPost {
#     content: String,
# }
#
impl DraftPost {
    // ...snip...

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

<span class="caption">Listing 17-20: A `PendingReviewPost` that gets created by
calling `request_review` on `DraftPost`, and an `approve` method that turns a
`PendingReviewPost` into a published `Post`</span>

The `request_review` and `approve` methods take ownership of `self`, thus
consuming the `DraftPost` and `PendingReviewPost` instances and transforming
them into a `PendingReviewPost` and a published `Post`, respectively. This way,
we won’t have any `DraftPost` instances lingering around after we’ve called
`request_review` on them, and so forth. `PendingReviewPost` doesn’t have a
`content` method defined on it, so attempting to read its content results in a
compiler error, as with `DraftPost`. Because the only way to get a published
`Post` instance that does have a `content` method defined is to call the
`approve` method on a `PendingReviewPost`, and the only way to get a
`PendingReviewPost` is to call the `request_review` method on a `DraftPost`,
we’ve now encoded the blog post workflow into the type system.

This does mean we have to make some small changes to `main`. The
`request_review` and `approve` methods return new instances rather than
modifying the struct they’re called on, so we need to add more `let post = `
shadowing assignments to save the returned instances. We also can’t have the
assertions about the draft and pending review post’s contents being empty
strings, nor do we need them: we can’t compile code that tries to use the
content of posts in those states any longer. The updated code in `main` is
shown in Listing 17-21:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

<span class="caption">Listing 17-21: Modifications to `main` to use the new
implementation of the blog post workflow</span>

These changes we need to make to `main` to reassign `post` means this
implementation doesn’t quite follow the object-oriented state pattern anymore:
the transformations between the states are no longer encapsulated entirely
within the `Post` implementation. However, our gain is that invalid states are
now impossible because of the type system and the type checking that happens at
compile time! This ensures that certain bugs, such as the content of an
unpublished post being displayed, will be discovered before they make it to
production.

Try the tasks suggested for additional requirements that we mentioned at the
start of this section on this code, to see how working with this version of the
code feels.

We’ve seen that even though Rust is capable of implementing object-oriented
design patterns, other patterns like encoding state into the type system are
also available in Rust. These patterns have different tradeoffs. While you may
be very familiar with object-oriented patterns, rethinking the problem in order
to take advantage of Rust’s features can provide benefits like preventing some
bugs at compile-time. Object-oriented patterns won’t always be the best
solution in Rust, because of the features like ownership that object-oriented
languages don’t have.

## Summary

No matter whether you think Rust is an object-oriented language or not after
reading this chapter, you’ve now seen that trait objects are a way to get some
object-oriented features in Rust. Dynamic dispatch can give your code some
flexibility in exchange for a bit of runtime performance. This flexibility can
be used to implement object-oriented patterns that can help with the
maintainability of your code. Rust also has other different features, like
ownership, that object-oriented languages don’t have. An object-oriented
pattern won’t always be the best way to take advantage of Rust’s strengths, but
is an available option.

Next, let’s look at another feature of Rust that enables lots of flexibility:
patterns. We’ve looked at them briefly throughout the book, but haven’t seen
everything they’re capable of yet. Let’s go!
