use log::warn;
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub service_name: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let port = match env::var("PORT") {
            Ok(val) => val.parse().unwrap_or_else(|e| {
                warn!("Invalid PORT value '{}': {}, falling back to 8080", val, e);
                8080
            }),
            Err(_) => 8080,
        };

        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port,
            service_name: env::var("SERVICE_NAME")
                .unwrap_or_else(|_| "loka-zk-middleware".to_string()),
        }
    }
}
