// State pattern implementation using enums instead of structs + extra tasks from The Book:

//✅ 1. Add a reject method that changes the post’s state from PendingReview back to Draft.

//✅ 2. Require two calls to approve before the state can be changed to Published.

//✅ 3. Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.



pub mod post_checker {

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum States { // represents Post states
        Draft,
        PendingReview,
        Published(bool) // bool is needed to check 1 or 2 times approve method was called
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
    
        // return content only if state is Published(true)
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }
    
        // add text only if state is Draft
        pub fn add_text(&mut self, text: &str) {
            if self.state.as_ref().unwrap().add_text() {
                self.content.push_str(text)
            }
        }
    
        // change state from Draft to PendingReview
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
    
        // change state from PendindReview to Published(false) and then to Published(true)
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        // change state back to Draft
        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }

        // return currernt state
        pub fn get_state(&self) -> States {
            self.state.as_ref().unwrap().get_state()
        }
    }
    
    // base trait of trait object for dynamic dispatch
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn get_state(&self) -> States;
        fn add_text(&self) -> bool;
    }
    
    // implementation of all functionality based on trait object that represents possible states to call then inside Post methods
    impl State for States {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            match *self {
                Draft => Box::new(PendingReview),
                PendingReview => self,
                Published(_) => self,
            }
        }
    
        fn approve(self: Box<Self>) -> Box<dyn State> {
            match *self {
                Draft => self,
                PendingReview => Box::new(Published(false)),
                Published(b) => {
                    if b == false {
                        Box::new(Published(true))
                    } else {
                        self
                    }
                },
            }
        }
    
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            match self {
                Draft => "",
                PendingReview => "",
                Published(b) => {
                    if b == &true {
                        &post.content
                    } else {
                        ""
                    }
                },
            }
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            match *self {
                Draft => self,
                PendingReview => Box::new(Draft),
                Published(_) => Box::new(Draft),
            }
        }

        fn get_state(&self) -> States {
            match self {
                s => *s
            }
        }

        fn add_text(&self) -> bool{
            match *self {
                Draft => true,
                _ => false,
            }
        }
    }    
}