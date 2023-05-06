use actix_web::web;

use crate::routes::post;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    println!("app_config::config_app");
    post::binding(cfg);
}
