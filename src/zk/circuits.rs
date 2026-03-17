use ark_ff::Field;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};

/// A circuit that proves knowledge of a secret value `x` such that
/// `x * x = y` (square relationship). This is a fundamental building block
/// for privacy-preserving payment verification in Agentic flows.
///
/// Use case: An agent can prove it computed a valid payment hash
/// without revealing the underlying secret (e.g., payment preimage).
#[derive(Clone)]
pub struct SquareCircuit<F: Field> {
    /// Secret input (witness) - the value known only to the prover
    pub x: Option<F>,
    /// Public input (instance) - the value verifiable by anyone
    pub y: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for SquareCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let x = cs.new_witness_variable(|| {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let y = cs.new_input_variable(|| {
            self.y.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Enforce: x * x = y
        cs.enforce_constraint(lc!() + x, lc!() + x, lc!() + y)?;

        Ok(())
    }
}

/// A circuit that proves knowledge of two secret values `a` and `b`
/// such that `a + b = sum` (addition relationship).
///
/// Use case: Prove that a payment split between multiple agents
/// sums to the correct total without revealing individual amounts.
#[derive(Clone)]
pub struct SumCircuit<F: Field> {
    /// First secret addend
    pub a: Option<F>,
    /// Second secret addend
    pub b: Option<F>,
    /// Public sum
    pub sum: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for SumCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| {
            self.a.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let b = cs.new_witness_variable(|| {
            self.b.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let sum = cs.new_input_variable(|| {
            self.sum.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Enforce: (a + b) * 1 = sum
        cs.enforce_constraint(
            lc!() + a + b,
            lc!() + Variable::One,
            lc!() + sum,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_relations::r1cs::ConstraintSystem;

    #[test]
    fn test_square_circuit_valid() {
        let cs = ConstraintSystem::<Fr>::new_ref();

        let x = Fr::from(3u64);
        let y = Fr::from(9u64);

        let circuit = SquareCircuit {
            x: Some(x),
            y: Some(y),
        };

        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_square_circuit_invalid() {
        let cs = ConstraintSystem::<Fr>::new_ref();

        let x = Fr::from(3u64);
        let y = Fr::from(10u64);

        let circuit = SquareCircuit {
            x: Some(x),
            y: Some(y),
        };

        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_sum_circuit_valid() {
        let cs = ConstraintSystem::<Fr>::new_ref();

        let a = Fr::from(5u64);
        let b = Fr::from(7u64);
        let sum = Fr::from(12u64);

        let circuit = SumCircuit {
            a: Some(a),
            b: Some(b),
            sum: Some(sum),
        };

        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_sum_circuit_invalid() {
        let cs = ConstraintSystem::<Fr>::new_ref();

        let a = Fr::from(5u64);
        let b = Fr::from(7u64);
        let sum = Fr::from(13u64);

        let circuit = SumCircuit {
            a: Some(a),
            b: Some(b),
            sum: Some(sum),
        };

        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());
    }
}
