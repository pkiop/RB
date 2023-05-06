use actix_web::{get, web, Responder};

#[get("/post")]
async fn get_posts() -> impl Responder {
    format!("Hello!")
}

pub fn binding(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/post").service(get_posts));
}
