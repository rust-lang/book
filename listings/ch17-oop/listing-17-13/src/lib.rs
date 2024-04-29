pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// ANCHOR: here
impl Post {
    // --snip--
    // ANCHOR_END: here
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // ANCHOR: here
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
// ANCHOR_END: here

trait State {}

struct Draft {}

impl State for Draft {}
