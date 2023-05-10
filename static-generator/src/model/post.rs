use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub author: String,
    pub updated_at: String,
    pub created_at: String,
    pub tags: Vec<String>,
    pub category: String,
    pub view_count: i32,
    pub like_count: i32,
}
