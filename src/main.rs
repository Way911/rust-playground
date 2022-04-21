use rust_playground::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // println!("{}", post.content());

    let post = post.request_review();
    // println!("{}", post.content());

    let post = post.approve();
    // println!("{}", post.content());
    assert_eq!("I ate a salad for lunch today", post.content());
}
