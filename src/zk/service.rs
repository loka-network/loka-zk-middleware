use ark_bn254::{Bn254, Fr};
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_snark::SNARK;
use ark_std::rand::thread_rng;
use log::info;

use super::circuits::{SquareCircuit, SumCircuit};
use crate::errors::ZkError;

/// ZkService provides zero-knowledge proof generation and verification
/// capabilities for the middleware API.
pub struct ZkService;

/// Result of proof generation, containing serialized proof and verification key
#[derive(Debug)]
pub struct ProofResult {
    pub proof_bytes: Vec<u8>,
    pub vk_bytes: Vec<u8>,
    pub public_inputs: Vec<String>,
}

impl ZkService {
    pub fn new() -> Self {
        info!("ZkService initialized");
        Self
    }

    /// Generate a Groth16 proof for the square circuit (x * x = y).
    pub fn generate_square_proof(&self, secret: u64) -> Result<ProofResult, ZkError> {
        let x = Fr::from(secret);
        let y = x * x;

        let circuit = SquareCircuit {
            x: Some(x),
            y: Some(y),
        };

        let setup_circuit = SquareCircuit::<Fr> { x: None, y: None };

        self.generate_groth16_proof(setup_circuit, circuit, vec![y])
    }

    /// Generate a Groth16 proof for the sum circuit (a + b = sum).
    pub fn generate_sum_proof(&self, a: u64, b: u64) -> Result<ProofResult, ZkError> {
        let fa = Fr::from(a);
        let fb = Fr::from(b);
        let sum = fa + fb;

        let circuit = SumCircuit {
            a: Some(fa),
            b: Some(fb),
            sum: Some(sum),
        };

        let setup_circuit = SumCircuit::<Fr> {
            a: None,
            b: None,
            sum: None,
        };

        self.generate_groth16_proof(setup_circuit, circuit, vec![sum])
    }

    fn generate_groth16_proof<C: ConstraintSynthesizer<Fr> + Clone>(
        &self,
        setup_circuit: C,
        proving_circuit: C,
        public_inputs: Vec<Fr>,
    ) -> Result<ProofResult, ZkError> {
        let mut rng = thread_rng();

        // WARNING: Per-request trusted setup is for development/demo only.
        // In production, use ceremony-generated parameters (e.g., Perpetual Powers of Tau)
        // and load pre-computed proving/verification keys.
        let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(setup_circuit, &mut rng)
            .map_err(|e| ZkError::ProofGenerationFailed(e.to_string()))?;

        let proof = Groth16::<Bn254>::prove(&pk, proving_circuit, &mut rng)
            .map_err(|e| ZkError::ProofGenerationFailed(e.to_string()))?;

        let mut proof_bytes = Vec::new();
        proof
            .serialize_compressed(&mut proof_bytes)
            .map_err(|e| ZkError::SerializationError(e.to_string()))?;

        let mut vk_bytes = Vec::new();
        vk.serialize_compressed(&mut vk_bytes)
            .map_err(|e| ZkError::SerializationError(e.to_string()))?;

        let public_input_strings: Result<Vec<String>, ZkError> = public_inputs
            .iter()
            .map(|f| {
                let mut bytes = Vec::new();
                f.serialize_compressed(&mut bytes)
                    .map_err(|e| ZkError::SerializationError(e.to_string()))?;
                Ok(hex::encode(&bytes))
            })
            .collect();
        let public_input_strings = public_input_strings?;

        Ok(ProofResult {
            proof_bytes,
            vk_bytes,
            public_inputs: public_input_strings,
        })
    }

    /// Verify a Groth16 proof given serialized proof, verification key, and public inputs
    pub fn verify_proof(
        &self,
        proof_hex: &str,
        vk_hex: &str,
        public_inputs_hex: &[String],
    ) -> Result<bool, ZkError> {
        let proof_bytes = hex::decode(proof_hex)
            .map_err(|e| ZkError::InvalidInput(format!("Invalid proof hex: {}", e)))?;
        let proof = ark_groth16::Proof::<Bn254>::deserialize_compressed(&proof_bytes[..])
            .map_err(|e| ZkError::InvalidInput(format!("Invalid proof format: {}", e)))?;

        let vk_bytes = hex::decode(vk_hex)
            .map_err(|e| ZkError::InvalidInput(format!("Invalid vk hex: {}", e)))?;
        let vk = <Groth16<Bn254> as SNARK<Fr>>::VerifyingKey::deserialize_compressed(&vk_bytes[..])
            .map_err(|e| ZkError::InvalidInput(format!("Invalid vk format: {}", e)))?;

        let mut public_inputs = Vec::new();
        for input_hex in public_inputs_hex {
            let input_bytes = hex::decode(input_hex)
                .map_err(|e| ZkError::InvalidInput(format!("Invalid input hex: {}", e)))?;
            let input = Fr::deserialize_compressed(&input_bytes[..])
                .map_err(|e| ZkError::InvalidInput(format!("Invalid input format: {}", e)))?;
            public_inputs.push(input);
        }

        let pvk = Groth16::<Bn254>::process_vk(&vk)
            .map_err(|e| ZkError::VerificationFailed(e.to_string()))?;

        let valid = Groth16::<Bn254>::verify_with_processed_vk(&pvk, &public_inputs, &proof)
            .map_err(|e| ZkError::VerificationFailed(e.to_string()))?;

        Ok(valid)
    }
}

impl Default for ZkService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_proof_generation_and_verification() {
        let service = ZkService::new();

        let result = service.generate_square_proof(3).unwrap();

        let proof_hex = hex::encode(&result.proof_bytes);
        let vk_hex = hex::encode(&result.vk_bytes);

        let valid = service
            .verify_proof(&proof_hex, &vk_hex, &result.public_inputs)
            .unwrap();

        assert!(valid, "Valid square proof should verify");
    }

    #[test]
    fn test_sum_proof_generation_and_verification() {
        let service = ZkService::new();

        let result = service.generate_sum_proof(5, 7).unwrap();

        let proof_hex = hex::encode(&result.proof_bytes);
        let vk_hex = hex::encode(&result.vk_bytes);

        let valid = service
            .verify_proof(&proof_hex, &vk_hex, &result.public_inputs)
            .unwrap();

        assert!(valid, "Valid sum proof should verify");
    }
}
