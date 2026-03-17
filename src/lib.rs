pub mod api;
pub mod config;
pub mod errors;
pub mod zk;

use std::sync::Arc;

use config::AppConfig;
use zk::ZkService;

pub struct AppState {
    pub zk_service: Arc<ZkService>,
    pub config: AppConfig,
}
