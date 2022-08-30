//The implementation using the state pattern is easy to extend to add more functionality. To see the simplicity of maintaining code that uses the state pattern, try a few of these suggestions:

// 1. Add a reject method that changes the post’s state from PendingReview back to Draft.

// 2. Require two calls to approve before the state can be changed to Published.

// 3. Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.



pub mod post_checker {

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum States {
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
            self.content.push_str(text)
        }
    
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
    
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }

        pub fn display_state(&self) -> States {
            self.state.as_ref().unwrap().display_state()
        }

        


    }
    
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn display_state(&self) -> States;
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

        fn reject(self: Box<Self>) -> Box<dyn State> {
            match *self {
                Draft => self,
                PendingReview => Box::new(Draft),
                Published => Box::new(Draft),
            }
        }

        fn display_state(&self) -> States {
            match self {
                s => *s
            }
        }
    }    
}