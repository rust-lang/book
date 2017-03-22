## Object-Oriented Design Pattern Implementation

Let's look at an example of the state design pattern and how to use it in Rust.
The *state pattern* is when a value has some internal state, and the value's
behavior changes based on the internal state. The internal state changes
according to rules encoded in a *state machine*, which manages states and the
valid transitions between states.

We're going to implement a blog post workflow. The workflow that we want our
blog posts to follow is:

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
demonstration purposes; asserting that a draft blog post returns an empty
string from the `content` method would also make an excellent unit test in our
library.

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

Let's get started on the implementation of the library! We know we want to have
a public `Post` struct that holds some content, so let's start with the
definition of the struct and an associated public `new` function to create an
instance of `Post` as shown in Listing 17-12. We're also going to have a
private enum `State` to represent the states a post can be in, and a post will
hold a `State` value in a private field:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Post {
    state: State,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: State::new(),
            content: String::new(),
        }
    }
}

#[derive(Clone, Copy)]
enum State {
    Draft,
    PendingReview,
    Published,
}

impl State {
    fn new() -> State {
        State::Draft
    }
}
```

<span class="caption">Listing 17-12: Definition of a `Post` struct, a `State`
enum, and `new` functions for each</span>


We're going to use Rust's type system and encapsulation to change the behavior of a value based on some internal

State pattern is for:
- can only be in 1 state at a time
- transitions from one state to the next, some are valid and some aren't
- would like to verify valid transitions at compile time
- change behavior when the state changes


blog post
  - status: draft, review pending, review approved, published
  - spellcheck: unchecked, errors, no errors



## Summary

TODO: Trait objects provide dynamic dispatch, which enables OOP-like patterns,
whether that means you want to call it object oriented or not, make up your own
mind. Reiterate tradeoffs of trait objects
