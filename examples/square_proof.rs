//! Example: Square proof generation and verification
//!
//! Demonstrates the SquareCircuit (x * x = y) using Groth16 on BN254.
//! The prover knows a secret `x` and proves that `y = x²` without revealing `x`.
//!
//! Run with: `cargo run --example square_proof`

use loka_zk_middleware::zk::ZkService;

fn main() {
    println!("=== Square Proof Example ===\n");

    let service = ZkService::new();

    // The prover knows the secret value 7, and wants to prove that 7² = 49
    let secret = 7u64;
    println!("Secret value (witness): {}", secret);
    println!(
        "Public value (instance): {} (= {}²)\n",
        secret * secret,
        secret
    );

    // Generate proof
    println!("Generating Groth16 proof...");
    let result = service
        .generate_square_proof(secret)
        .expect("Proof generation failed");

    let proof_hex = hex::encode(&result.proof_bytes);
    let vk_hex = hex::encode(&result.vk_bytes);

    println!("Proof (hex):             {}...", &proof_hex[..40]);
    println!("Verification key (hex):  {}...", &vk_hex[..40]);
    println!("Public inputs:           {:?}\n", result.public_inputs);

    // Verify proof
    println!("Verifying proof...");
    let valid = service
        .verify_proof(&proof_hex, &vk_hex, &result.public_inputs)
        .expect("Verification call failed");

    println!("Proof valid: {}\n", valid);
    assert!(valid, "Square proof should be valid");

    println!("=== Square Proof Example Complete ===");
}
