use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, thiserror::Error)]
pub enum ZkError {
    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),

    #[error("Proof verification failed: {0}")]
    VerificationFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl ResponseError for ZkError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ZkError::InvalidInput(_) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": self.to_string(),
                "code": "INVALID_INPUT"
            })),
            ZkError::ProofGenerationFailed(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": self.to_string(),
                    "code": "PROOF_GENERATION_FAILED"
                }))
            }
            ZkError::VerificationFailed(_) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": self.to_string(),
                "code": "VERIFICATION_FAILED"
            })),
            _ => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": self.to_string(),
                "code": "INTERNAL_ERROR"
            })),
        }
    }
}
