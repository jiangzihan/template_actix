use actix_web::{test, HttpRequest, HttpResponse, HttpMessage};
use actix_web::http::{header, StatusCode};

async fn handler(req: HttpRequest) -> HttpResponse {
    if let Some(hdr) = req.headers().get(header::CONTENT_TYPE) {
        HttpResponse::Ok().into()
    } else {
        HttpResponse::BadRequest().into()
    }
}

#[actix_web::test]
async fn test_index() {
    let req = test::TestRequest::default()
        .insert_header(header::ContentType::plaintext())
        .to_http_request();

    let resp = handler(req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::default().to_http_request();
    let resp = handler(req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}