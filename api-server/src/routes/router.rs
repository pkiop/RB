use actix_web::web;

use crate::routes::post;

pub fn binding(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/post").configure(post::binding));
}
