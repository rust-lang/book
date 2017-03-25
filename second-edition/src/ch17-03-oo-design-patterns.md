## Object-Oriented Design Pattern Implementation

Let's look at an example of the state design pattern and how to use it in Rust.
The *state pattern* is when a value has some internal state, and the value's
behavior changes based on the internal state. The internal state is represented
by a set of objects that inherit shared functionality (we'll use structs and
traits since Rust doesn't have objects and inheritance). Each state object is
responsible for its own behavior and the rules for when it should change into
another state. The value that holds one of these state objects doesn't know
anything about the different behavior of the states or when to transition
between states. In the future when requirements change, we won't need to change
the code of the value holding the state or the code that uses the value. We'll
only need to update the code inside one of the state objects to change its
rules, or perhaps add more state objects.

In order to explore this idea, we're going to implement a blog post workflow in
an incremental way. The workflow that we want our blog posts to follow, once
we're done with the implementation, is:

1. A blog post starts as an empty draft.
2. Once the draft is done, we request a review of the post.
3. Once the post is approved, it gets published.
4. Only published blog posts return content to print so that we can't
   accidentally print the text of a post that hasn't been approved.

Any other changes attempted on a post should have no effect. For example, if we
try to approve a draft blog post before we've requested a review, the post
should stay an unpublished draft.

Listing 17-11 shows this workflow in code form. This is an example usage of the
API we're going to implement in a library crate named `blog`:

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

We want to be able to create a new draft blog post with `Post::new`. Then, we
want to add some text to the blog post while we're in the draft state. If we
try to print out the post's content immediately, though, we shouldn't get any
text, since the post is still a draft. We've added an `assert_eq!` here for
demonstration purposes. Asserting that a draft blog post returns an empty
string from the `content` method would make an excellent unit test in our
library, but we're not going to write tests for this example.

Next, we want to be able to request a review of our post, and `content` should
still return an empty string while waiting for a review. Lastly, when we
approve the blog post, it should get published, which means the text we added
will be returned when we call `content`.

Notice that the only type we're interacting with from the crate is the `Post`
type. The various states a post can be in (draft, waiting for review,
published) are managed internally to the `Post` type. The states change due to
the methods we call on the `Post` instance, but we don't have to manage the
state changes directly. This also means we won't make a mistake with the
states, like forgetting to request a review before publishing.

### Defining `Post` and Creating a New Instance in the Draft State

Let's get started on the implementation of the library! We know we want to have
a public `Post` struct that holds some content, so let's start with the
definition of the struct and an associated public `new` function to create an
instance of `Post` as shown in Listing 17-12. We're also going to have a
private trait `State`. `Post` will hold a trait object of `Box<State>` inside
an `Option` in a private field named `state`. We'll see why the `Option` is
necessary in a bit. The `State` trait defines all the behavior different post
states share, and the `Draft`, `PendingReview`, and `Published` states will all
implement the `State` trait. For now, the trait does not have any methods, and
we're going to start by defining just the `Draft` state since that's the state
we want to start in:

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
struct that implements `State`</span>

When we create a new `Post`, we set its `state` field to a `Some` value holding
a `Box` pointing to a new instance of the `Draft` struct. This ensures whenever
we create a new instance of `Post`, it'll start out as a draft. Because the
`state` field of `Post` is private, there's no way to create a `Post` in any
other state!

### Storing the Text of the Post Content

In the `Post::new` function, we set the `content` field to a new, empty
`String`. In Listing 17-11, we showed that we want to be able to call a method
named `add_text` and pass a `&str` to it to add that text to the content of the
blog post. We're choosing to implement this as a method rather than exposing
the `content` field as `pub` because we want to be able to control how the
`content` field's data is read by implementing a method later. The `add_text`
method is pretty straightforward though, let's add the implementation in
Listing 17-13 to the `impl Post` block:

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

<span class="caption">Listing 17-13: Implemeting the `add_text` method to add
text to a post's `content`</span>

`add_text` takes a mutable reference to `self`, since we're changing the `Post`
instance that we're calling `add_text` on. We then call `push_str` on the
`String` in `content` and pass the `text` argument to add to the saved
`content`. This isn't part of the state pattern since its behavior doesn't
depend on the state that the post is in. The `add_text` method doesn't interact
with the `state` field at all, but it is part of the behavior we want to
support.

### Content of a Draft Post is Empty

After we've called `add_text` and added some content to our post, we still want
the `content` method to return an empty string slice since the post is still in
the draft state, as shown on line 8 of Listing 17-11. For now, let's implement
the `content` method with the simplest thing that will fulfil this requirement:
always returning an empty string slice. We're going to change this later once
we implement the ability to change a post's state to be published. With what we
have so far, though, posts can only be in the draft state, which means the post
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

Next up is requesting a review of a post, which should change its state from
`Draft` to `PendingReview`. We want `post` to have a public method named
`request_review` that will take a mutable reference to `self`. Then we're going
to call a `request_review` method on the state that we're holding, and that
`request_review` method will consume the current state and return a new state.
In order to be able to consume the old state, the state `request_review` method
needs to take ownership of the state value. This is where the `Option` comes
in: we're going to take the `Some` value out of the `state` field and leave a
`None` in its place since Rust doesn't let us have unpopulated fields in
structs. Then we'll set the post's `state` value to the result of this
operation. Listing 17-15 shows this code:

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

We've added the `request_review` method to the `State` trait; all types that
implement the trait will now need to implement the `request_review` method. The
implementation for the `request_review` method on `Draft` is to return a new,
boxed instance of the `PendingReview` struct, which is a new type we've
introduced that represents the state when a post is waiting for a review. The
`PendingReview` struct also implements the `request_review` method, but it
doesn't do any transformations. It returns itself since requesting a review on
a post that's already in the `PendingReview` state should stay in the
`PendingReview` state.

Now we can start seeing the advantages of the state pattern: the
`request_review` method on `Post` is the same no matter what its `state` value
is. Each state is responsible for its own rules.

We're going to leave the `content` method on `Post` as it is, returning an
empty string slice. We can now have a `Post` in the `PendingReview` state, not
just the `Draft` state, but we want the same behavior in the `PendingReview`
state. Listing 17-11 now works up until line 11!

### Approving a Post Changes the Behavior of `content`

The `approve` method on `Post` will be similar to that of the `request_review`
method: it will set the `state` to the value that the current state says it
should have when that state is approved. We'll need to add the `approve` method
to the `State` trait, and we'll add a new struct that implements `State`, the
`Published` state. Listing 17-16 shows the new code:

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
#         Box::new(PendingReview {})
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

Similarly to `request_review`, if we call the `approve` method on a `Draft`, it
will have no effect since it will return `self`. When we call `approve` on
`PendingReview`, it returns a new, boxed instance of the `Published` struct.
The `Published` struct implements the `State` trait, and for both the
`request_review` method and the `approve` method, it returns itself since the
post should stay in the `Published` state in those cases.

Now for updating the `content` method on `Post`: we want to return the value in
the post's `content` field if its state is `Published`, otherwise we want to
return an empty string slice. Because the goal is to keep all the rules like
this in the structs that implement `State`, we're going to call a `content`
method on the value in `state` and pass the post instance (that is, `self`) as
an argument. Then we'll return the value returned from the `content` method on
the `state` value as shown in Listing 17-17:

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

We're calling the `as_ref` method on the `Option` because we want a reference
to the value inside the `Option`. We're then calling the `unwrap` method, which
we know will never panic because all the methods on `Post` ensure that the
`state` value will have a `Some` value in it when those methods are done. This
is one of the cases we talked about in Chapter 12 where we know that a `None`
value is never possible even though the compiler isn't able to understand that.

The `content` method on the `State` trait is where the logic for what content
to return will be. We're going to add a default implementation for the
`content` method that returns an empty string slice. That lets us not need to
implement `content` on the `Draft` and `PendingReview` structs. The `Published`
struct will override the `content` method and will return the value in
`post.content`, as shown in Listing 17-18:

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

Note that we need lifetime annotations on this method, like we discussed in
Chapter 10. We're taking a reference to a `post` as an argument, and we're
returning a reference to a part of that `post`, so the lifetime of the returned
reference is related to the lifetime of the `post` argument.

### Tradeoffs of the State Pattern

We've shown that Rust is capable of implementing the object-oriented state
pattern in order to encapsulate the different kinds of behavior that a post
should have that depends on the state that the post is in. The methods on
`Post` don't know anything about the different kinds of behavior. The way this
code is organized, we have one place to look in order to find out all the
different ways that a published post behaves: the implementation of the `State`
trait on the `Published` struct.

An alternative implementation that didn't use the state pattern might have
`match` statements in the methods on `Post` or even in the code that uses
`Post` (`main` in our case) that checks what the state of the post is and
changes behavior in those places instead. That would mean we'd have a lot of
places to look in order to understand all the implications of a post being in
the published state! This would get worse the more states we added: each of
those `match` statements would need another arm. With the state pattern, the
`Post` methods and the places we use `Post` don't need `match` statements and
adding a new state only involves adding a new `struct` and implementing the
trait methods on that one struct.

This implementation is easy to extend to add more functionality. Here are some
changes you can try making to the code in this section to see for yourself what
it's like to maintain code using this pattern over time:

- Only allow adding text content when a post is in the `Draft` state
- Add a `reject` method that changes the post's state from `PendingReview` back
  to `Draft`
- Require two calls to `approve` before changing the state to `Published`

A downside of the state pattern is that since the states implement the
transitions between the states, some of the states are coupled to each other.
If we add another state between `PendingReview` and `Published`, such as
`Scheduled`, we would have to change the code in `PendingReview` to transition
to `Scheduled` instead. It would be nicer if `PendingReview` wouldn't need to
change because of the addition of a new state, but that would mean switching to
another design pattern.

There are a few bits of duplicated logic that are a downside of this
implementation in Rust. It would be nice if we could make default
implementations for the `request_review` and `approve` methods on the `State`
trait that return `self`, but this would violate object safety since the trait
doesn't know what the concrete `self` will be exactly. We want to be able to
use `State` as a trait object, so we need its methods to be object safe.

The other duplication that would be nice to get rid of is the similar
implementations of the `request_review` and `approve` methods on `Post`. They
both delegate to the implementation of the same method on the value in the
`Option` in the `state` field, and set the new value of the `state` field to
the result. If we had a lot of methods on `Post` that followed this pattern, we
might consider defining a macro to eliminate the repetition (see Appendix E on
macros).

## Summary

No matter whether you think Rust is an object-oriented language or not after
reading this chapter, you've now seen that trait objects are a way to get some
object-oriented features in Rust. Dynamic dispatch can give your code some
flexibility in exchange for a bit of runtime performance. This flexibility can
be used to implement object-oriented patterns that can help with the
maintainability of your code.

Next, let's look at another feature of Rust that enables lots of flexibility:
patterns. We've looked at them briefly throughout the book, but haven't seen
everything they're capable of yet. Let's go!
