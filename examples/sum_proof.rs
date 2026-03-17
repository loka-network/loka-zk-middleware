//! Example: Sum proof generation and verification
//!
//! Demonstrates the SumCircuit (a + b = sum) using Groth16 on BN254.
//! The prover knows two secret values `a` and `b`, and proves their sum
//! equals a public value without revealing the individual addends.
//!
//! Run with: `cargo run --example sum_proof`

use loka_zk_middleware::zk::ZkService;

fn main() {
    println!("=== Sum Proof Example ===\n");

    let service = ZkService::new();

    // The prover knows two secret values and wants to prove their sum
    let a = 42u64;
    let b = 58u64;
    println!("Secret a (witness):      {}", a);
    println!("Secret b (witness):      {}", b);
    println!("Public sum (instance):   {} (= {} + {})\n", a + b, a, b);

    // Generate proof
    println!("Generating Groth16 proof...");
    let result = service
        .generate_sum_proof(a, b)
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
    assert!(valid, "Sum proof should be valid");

    println!("=== Sum Proof Example Complete ===");
}
