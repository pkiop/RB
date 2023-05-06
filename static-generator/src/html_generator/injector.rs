use crate::model::post::Post;

pub fn inject(posts: Vec<Post>) {
    println!("injecting posts: {:?}", posts);
}
