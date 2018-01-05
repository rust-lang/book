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
		
		pub fn approve(&mut self) {
			if let Some(s) = self.state.take() {
				self.state = Some(s.approve())
			}
		}
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
	
	pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
	
	pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}


trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
	fn approve(self: Box<Self>) -> Box<State>;
	
	fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }
	
	fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
	
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
	
	fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}