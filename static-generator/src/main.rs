mod html_generator;
mod http_client;
mod model;
use std::env;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv::dotenv().ok();
    let uri = env::var("API_URI").expect("API_URI must be set");
    let post_uri = uri + "/api/post";
    let posts = http_client::api::get_posts(post_uri.parse().unwrap())
        .await
        .unwrap();
    html_generator::injector::inject(posts);
}
