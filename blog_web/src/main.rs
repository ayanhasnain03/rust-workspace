use blog_shared::Post;
fn main() {
let post = Post::new(
    "Post on the server".to_owned(),
    "Lets get rusty".to_owned(),
);
println!("{post:?}");
}
