pub mod kzg;
pub mod param;
use ark_ec::PairingEngine;

pub struct Commitment<E: PairingEngine, const N: usize> {
    commitment: E::G1Projective,
}

pub trait CommitmentScheme {
    type ProverParam;
    type VerifierParam;
    type MessageUnit;
    type Commitment;
    type Witness;

    /// Commit to a list of inputs with prover parameters
    fn commit(pp: &Self::ProverParam, inputs: &[Self::MessageUnit]) -> Self;

    /// Open an input at a given position
    fn open(pp: &Self::ProverParam, inputs: &[Self::MessageUnit], pos: usize) -> Self::Witness;

    /// Verify the input/witness pair is correct
    fn verify(
        &self,
        vp: &Self::VerifierParam,
        input: &Self::MessageUnit,
        pos: usize,
        witness: &Self::Witness,
    ) -> bool;

    fn to_string(&self) -> String;
}
