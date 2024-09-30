use actix_web::{cookie::Key, dev, http::{header, StatusCode}, web, App, HttpServer, Result};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_cors::Cors;
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};

mod middleware;
mod example;
mod metrics;
mod ws;

pub async fn run_forever(host:&str, port:u16) -> Result<(),actix_web::Error> {
    HttpServer::new(move || {
        let cors = Cors::default()
        // 指定源
        // .allowed_origin("http://example.com")
        // .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        // .allowed_header(header::CONTENT_TYPE)
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600);

        let user_session = SessionMiddleware::builder(
            CookieSessionStore::default(),
            Key::from(&[0; 64])
        )
        .cookie_secure(false)
        .build();

        // 默认headers
        let default_headers = actix_web::middleware::DefaultHeaders::new()
            .add(("X-Version", "0.2"));

        App::new()
        // 错误处理
        .wrap(ErrorHandlers::new()
            .handler(
                StatusCode::INTERNAL_SERVER_ERROR, 
                add_error_header
            ),
        )
        // 默认headers
        .wrap(default_headers)
        // 用户cookie session
        .wrap(user_session)
        // 自定义中间件
        .wrap(middleware::SayHi)
        // 跨域
        .wrap(cors)
        // 路由配置
        .configure(example::routes)
        .configure(metrics::routes)
        .configure(ws::routes)
        

    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}


// 错误处理函数
fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}