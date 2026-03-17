use actix_web::{web, App, HttpServer};
use log::info;
use std::sync::Arc;

mod api;
mod config;
mod errors;
mod zk;

use config::AppConfig;
use zk::ZkService;

pub struct AppState {
    pub zk_service: Arc<ZkService>,
    pub config: AppConfig,
}

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
