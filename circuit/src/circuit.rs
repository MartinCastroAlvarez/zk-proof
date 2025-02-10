use ark_ff::Field;
use ark_relations::lc;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable};

/// A simple circuit that enforces the constraint a + b = c.
///
/// # Example
///
/// ```
/// use ark_bn254::Fr;
/// use ark_relations::r1cs::{ConstraintSystem, ConstraintSynthesizer};
/// use my_crate::circuit::PrimerCircuit; // Adjust the path as necessary
///
/// let cs = ConstraintSystem::<Fr>::new_ref();
///
/// let circuit = PrimerCircuit {
///     a: Some(Fr::from(3u32)),
///     b: Some(Fr::from(4u32)),
///     c: Some(Fr::from(7u32)),
/// };
///
/// circuit.generate_constraints(cs.clone()).unwrap();
/// assert!(cs.is_satisfied().unwrap());
/// ```
#[derive(Clone)]
pub struct PrimerCircuit<F: Field> {
    // Public input
    pub c: Option<F>,
    // Private witness
    pub a: Option<F>,
    pub b: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for PrimerCircuit<F> {
    /// Generates the constraints for the circuit.
    ///
    /// # Example
    ///
    /// ```
    /// use ark_bn254::Fr;
    /// use ark_relations::r1cs::{ConstraintSystem, ConstraintSynthesizer};
    /// use my_crate::circuit::PrimerCircuit; // Adjust the path as necessary
    ///
    /// let cs = ConstraintSystem::<Fr>::new_ref();
    ///
    /// let circuit = PrimerCircuit {
    ///     a: Some(Fr::from(3u32)),
    ///     b: Some(Fr::from(4u32)),
    ///     c: Some(Fr::from(7u32)),
    /// };
    ///
    /// circuit.generate_constraints(cs.clone()).unwrap();
    /// assert!(cs.is_satisfied().unwrap());
    /// ```
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate the public input c
        let c_var = cs.new_input_variable(|| self.c.ok_or(SynthesisError::AssignmentMissing))?;

        // Allocate the private inputs a and b
        let a_var = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b_var = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;

        // Enforce a + b = c
        // This translates to: a + b - c = 0
        cs.enforce_constraint(lc!() + a_var + b_var, lc!() + Variable::One, lc!() + c_var)?;

        Ok(())
    }
}
