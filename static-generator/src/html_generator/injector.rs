use crate::model;

pub fn inject(posts: Vec<model::post::Post>) {
    println!("injecting posts: {:?}", posts);
}
