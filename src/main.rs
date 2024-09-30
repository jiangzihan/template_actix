use template_actix::{common::log_config::init_tracing, endpoint::run_forever};

#[tokio::main]
async fn main() {
    init_tracing();
    let host = std::env::var("LISTEN_HOST").unwrap_or("0.0.0.0".to_string());
    let port:u16 = std::env::var("LISTEN_PORT").unwrap_or("9000".to_string()).parse().unwrap();
    run_forever(&host, port).await.unwrap()
}