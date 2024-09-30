use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use tracing::info;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/example").to(index_service))
        .route("/example/echo", web::post().to(echo_service));
}

pub async fn index_service() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn echo_service(req_body: String) -> impl Responder {
    info!("body: {}", req_body);
    HttpResponse::Ok().json(json!({
        "value": req_body
    }))
}
