use serde::{Deserialize, Serialize};

/// Request to generate a square proof (x * x = y)
#[derive(Debug, Deserialize)]
pub struct SquareProofRequest {
    /// The secret value (witness). Only the prover knows this.
    pub secret: u64,
}

/// Request to generate a sum proof (a + b = sum)
#[derive(Debug, Deserialize)]
pub struct SumProofRequest {
    /// First secret addend
    pub a: u64,
    /// Second secret addend
    pub b: u64,
}

/// Response containing a generated ZK proof
#[derive(Debug, Serialize)]
pub struct ProofResponse {
    /// Unique identifier for this proof
    pub proof_id: String,
    /// Hex-encoded proof data
    pub proof: String,
    /// Hex-encoded verification key
    pub verification_key: String,
    /// Hex-encoded public inputs
    pub public_inputs: Vec<String>,
    /// Proof system used
    pub scheme: String,
    /// Curve used
    pub curve: String,
}

/// Request to verify a ZK proof
#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    /// Hex-encoded proof data
    pub proof: String,
    /// Hex-encoded verification key
    pub verification_key: String,
    /// Hex-encoded public inputs
    pub public_inputs: Vec<String>,
}

/// Response from proof verification
#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    /// Whether the proof is valid
    pub valid: bool,
    /// Proof system used for verification
    pub scheme: String,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Service name
    pub service: String,
    /// Service version
    pub version: String,
    /// Supported proof schemes
    pub supported_schemes: Vec<String>,
    /// Supported curves
    pub supported_curves: Vec<String>,
}
