
//🔥🔥🔥 import 2 different implementations: based on trait object + enums and pure enums
// use state_pattern_with_enums::post_checker_dyn::{Post, States};
use state_pattern_with_enums::post_checker_pure::{Post, States};

fn main() {
	let mut post = Post::new();

	post.add_text("I ate a salad for lunch today");
	assert_eq!("", post.content());

	post.request_review();
	assert_eq!("", post.content());

	post.approve(); // 1st call to approve
	assert_eq!("", post.content());

    post.approve(); // 2nd call
	assert_eq!("I ate a salad for lunch today", post.content());

    post.add_text(" and still remained hungry"); // doesn't work because add_text is allowed only on Draft state
    assert_eq!("I ate a salad for lunch today", post.content());

	post.reject(); // return state to Draft
	assert_eq!(States::Draft, post.get_state());

    post.add_text(" and still remained hungry"); // adding text in Draft state is allowed
    post.request_review();
    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today and still remained hungry", post.content()); // works!
}

