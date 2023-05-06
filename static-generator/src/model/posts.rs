use super::core::get_mongodb_client;
use futures::stream::TryStreamExt;
use mongodb::{bson::Document, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    name: String,
    content: String,
}

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

pub async fn get_posts() {
    let post_collection = get_post_collection().await;
    let mut cursor = post_collection.find(None, None).await.unwrap();
    while let Some(post_doc) = cursor.try_next().await.unwrap() {
        let post: Post = bson::from_document(post_doc).unwrap();
        println!("title: {}", post.name);
    }
    return;
}
