use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch yesterday");

    let post = post.request_review().approve();

    assert_eq!("I ate a salad for lunch yesterday", post.content());
}
