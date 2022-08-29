pub mod post_checker {
    enum States {
        Draft,
        PendingReview,
        Published
    }
    
    use self::States::{Draft, PendingReview, Published};
    
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }
    
    impl Post {
        pub fn new() -> Self {
            Self {
                state: Some(Box::new(Draft)),
                content: String::new(),
            }
        }
    
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }
    
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
    
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
    
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve());
            }
        }
    }
    
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str;
    }
    
    impl State for States {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            match *self {
                Draft => Box::new(PendingReview),
                PendingReview => self,
                Published => self,
            }
        }
    
        fn approve(self: Box<Self>) -> Box<dyn State> {
            match *self {
                Draft => self,
                PendingReview => Box::new(Published),
                Published => self,
            }
        }
    
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            match self {
                Draft => "",
                PendingReview => "",
                Published => &post.content,
            }
        }
    }    
}