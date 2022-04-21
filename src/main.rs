use rust_playground::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("{}", post.get_content());

    post.request_review();
    println!("{}", post.get_content());

    post.approve();
    println!("{}", post.get_content());
}
