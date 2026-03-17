pub mod handlers;
pub mod models;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/health", web::get().to(handlers::health_check))
            .route(
                "/prove/square",
                web::post().to(handlers::generate_square_proof),
            )
            .route("/prove/sum", web::post().to(handlers::generate_sum_proof))
            .route("/verify", web::post().to(handlers::verify_proof)),
    );
}
