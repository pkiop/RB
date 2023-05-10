use crate::model::post;
use hyper::body::HttpBody as _;
use hyper::Client;
use serde_json;
use std::str::from_utf8;

pub async fn get_posts(
    uri: hyper::Uri,
) -> Result<Vec<post::Post>, Box<dyn std::error::Error + Send + Sync>> {
    println!("uri: {:?}", uri);
    let client = Client::new();
    let mut resp = client.get(uri).await?;
    let mut body = String::new();
    while let Some(chunk) = resp.body_mut().data().await {
        let res = &chunk?;
        body += from_utf8(res).expect("fail string");
    }
    let result = serde_json::from_str::<Vec<post::Post>>(&body)?;
    Ok(result)
}
