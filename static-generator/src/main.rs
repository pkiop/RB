mod model;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv::dotenv().ok();
    // let document = mongodb::bson::doc! {
    //     "title": "Hello, world!",
    //     "content": "This is my first post!",
    // };
    model::posts::get_posts().await;
}
