use actix_web::{web, App, HttpServer};
use log::info;
use std::sync::Arc;

use loka_zk_middleware::config::AppConfig;
use loka_zk_middleware::zk::ZkService;
use loka_zk_middleware::{api, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = AppConfig::from_env();
    let bind_addr = format!("{}:{}", config.host, config.port);

    info!("Initializing ZK middleware service...");
    let zk_service = Arc::new(ZkService::new());

    let app_state = web::Data::new(AppState { zk_service, config });

    info!("Starting server on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(api::configure_routes)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
