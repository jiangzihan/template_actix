use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use serde_json::json;
use actix_web::http::{header, StatusCode};

use crate::errcode::{ControllerError, ErrorCodes};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(index))
        .service(web::resource("/metric").to(metrics_service))
        .route("/error_code", web::post().to(error_code));
}


pub async fn index(request: HttpRequest) -> impl Responder {
    println!("entry");
    HttpResponse::Ok().json(json!({
        "code": 10000,
        "hello": "ok"
    }))
}

pub async fn metrics_service(request: HttpRequest) -> Result<impl Responder> {
    println!("请求头: {:?}", request.headers());
    Ok(HttpResponse::Ok().body("abc"))
}

pub async fn error_code() -> impl Responder {
    HttpResponse::NotFound().json(json!({
        "code": 20000,
        "err_msg": ControllerError::new(ErrorCodes::ControllerErr)
    }))
}

async fn handler(req: HttpRequest) -> HttpResponse {
    if let Some(hdr) = req.headers().get(header::CONTENT_TYPE) {
        HttpResponse::Ok().into()
    } else {
        HttpResponse::BadRequest().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        App,
        http::{self, header::ContentType, StatusCode},
        test,
    };
    use serde_json::Value;
    use tracing::info;

    #[actix_web::test]
    async fn test_index_ok() {
        dotenvy::dotenv().unwrap();

        let app = test::init_service(
            App::new().service(
                web::resource("/").to(handler)
            )
        ).await;

        let req = test::TestRequest::get()
            .uri("/")
            .insert_header(ContentType::json())
            .to_request();

        info!("测试开始");
        let res = test::call_service(&app, req).await;
        // let result:Value = test::read_body_json(res).await;
        let result = test::read_body(res).await;
        let result = String::from_utf8(result.to_vec()).unwrap();
        println!("{:?}", result);
        // assert_eq!(result, Bytes::from_static(b"welcome!"));
    }

}