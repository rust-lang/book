// ANCHOR: all
use blog::Post;

// ANCHOR: here
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    // ANCHOR_END: here

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    // ANCHOR: here
}
// ANCHOR_END: here
// ANCHOR_END: all
