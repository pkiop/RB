use super::core::get_mongodb_client;
use mongodb::{bson::Document, Collection};

pub async fn get_post_collection() -> Collection<Document> {
    let client = get_mongodb_client().await.unwrap();
    let collection = client.collection("posts");
    collection
}

pub async fn add_post(document: Document) {
    let post_collection = get_post_collection().await;
    println!("inserting document: {:?}", document);
    post_collection
        .insert_one(document, None)
        .await
        .expect("insert failed");
    println!("inserting success");
    return;
}
