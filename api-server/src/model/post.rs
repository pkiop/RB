use super::core::get_mongodb_client;
use futures::stream::TryStreamExt;
use mongodb::{bson::Document, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    title: String,
    content: String,
    author: String,
    updated_at: String,
    created_at: String,
    tags: Vec<String>,
    category: String,
    view_count: i32,
    like_count: i32,
}

pub async fn get_post_collection() -> Collection<Document> {
    let client = get_mongodb_client().await.unwrap();
    let collection = client.collection("posts");
    collection
}

pub async fn add_post(post: Post) {
    let post_collection = get_post_collection().await;
    println!("inserting document: {:?}", post);
    let post_document = bson::to_document(&post).unwrap();
    post_collection
        .insert_one(post_document, None)
        .await
        .expect("insert failed");
    println!("inserting success");
    return;
}

pub async fn get_posts() -> Vec<Post> {
    let post_collection = get_post_collection().await;
    let mut cursor = post_collection.find(None, None).await.unwrap();
    let mut posts: Vec<Post> = Vec::<Post>::new();
    while let Some(post_doc) = cursor.try_next().await.unwrap() {
        let post: Post = bson::from_document(post_doc).unwrap();
        println!("title: {}", post.title);
        posts.push(post);
    }
    return posts;
}
