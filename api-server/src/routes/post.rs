use crate::model::post::{self, Post};
use actix_web::{get, post, web, Responder, Result};

#[get("")]
async fn get_posts() -> Result<impl Responder> {
    let posts = post::get_posts().await;
    Ok(web::Json(posts))
}

#[post("")]
async fn add_post(new_post: web::Json<Post>) -> Result<impl Responder> {
    let document = new_post.into_inner();
    let posts = post::add_post(document).await;
    Ok(web::Json(posts))
}

pub fn binding(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts).service(add_post);
}
