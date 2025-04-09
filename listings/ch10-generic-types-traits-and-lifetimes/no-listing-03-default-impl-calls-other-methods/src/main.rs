use aggregator::{self, SocialPost, Summary};

fn main() {
    // ANCHOR: here
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new social post: {}", post.summarize());
    // ANCHOR_END: here
}
