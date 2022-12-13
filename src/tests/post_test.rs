trait State {
    fn request_view(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn context<'a>(self: &Self, post: &'a Post) -> &'a str;
}

struct Draft {}

struct PendingReview {}

struct Published {}

impl State for Draft {
    fn request_view(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn context<'a>(self: &Self, post: &'a Post) -> &'a str {
        ""
    }
}

impl State for PendingReview {
    fn request_view(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn context<'a>(self: &Self, post: &'a Post) -> &'a str {
        ""
    }
}

impl State for Published {
    fn request_view(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn context<'a>(self: &Self, post: &'a Post) -> &'a str {
        &post.content
    }
}

struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().context(&self)
    }

    fn add_text(&mut self, t: &str) {
        self.content.push_str(t);
    }

    fn request_view(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_view())
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

#[test]
fn test_post() {
    let mut post = Post::new();
    println!("content: {}", post.content());
    assert_eq!("", post.content());
    post.add_text("hello");
    println!("content: {}", post.content());
    assert_eq!("", post.content());
    post.request_view();
    println!("content: {}", post.content());
    assert_eq!("", post.content());
    post.approve();
    println!("content: {}", post.content());
    assert_eq!("hello", post.content());
}
