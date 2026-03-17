use actix_web::{web, HttpResponse};
use log::info;
use uuid::Uuid;

use super::models::*;
use crate::errors::ZkError;
use crate::AppState;

/// Health check endpoint - returns service status and capabilities
pub async fn health_check(data: web::Data<AppState>) -> HttpResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: data.config.service_name.clone(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        supported_schemes: vec!["groth16".to_string()],
        supported_curves: vec!["bn254".to_string()],
    };
    HttpResponse::Ok().json(response)
}

/// Generate a ZK proof for the square circuit (x * x = y)
pub async fn generate_square_proof(
    data: web::Data<AppState>,
    req: web::Json<SquareProofRequest>,
) -> Result<HttpResponse, ZkError> {
    info!("Generating square proof");

    let result = data.zk_service.generate_square_proof(req.secret)?;

    let response = ProofResponse {
        proof_id: Uuid::new_v4().to_string(),
        proof: hex::encode(&result.proof_bytes),
        verification_key: hex::encode(&result.vk_bytes),
        public_inputs: result.public_inputs,
        scheme: "groth16".to_string(),
        curve: "bn254".to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Generate a ZK proof for the sum circuit (a + b = sum)
pub async fn generate_sum_proof(
    data: web::Data<AppState>,
    req: web::Json<SumProofRequest>,
) -> Result<HttpResponse, ZkError> {
    info!("Generating sum proof");

    let result = data.zk_service.generate_sum_proof(req.a, req.b)?;

    let response = ProofResponse {
        proof_id: Uuid::new_v4().to_string(),
        proof: hex::encode(&result.proof_bytes),
        verification_key: hex::encode(&result.vk_bytes),
        public_inputs: result.public_inputs,
        scheme: "groth16".to_string(),
        curve: "bn254".to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Verify a ZK proof
pub async fn verify_proof(
    data: web::Data<AppState>,
    req: web::Json<VerifyRequest>,
) -> Result<HttpResponse, ZkError> {
    info!("Verifying proof");

    let valid = data
        .zk_service
        .verify_proof(&req.proof, &req.verification_key, &req.public_inputs)?;

    let response = VerifyResponse {
        valid,
        scheme: "groth16".to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}
