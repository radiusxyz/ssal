pub mod kzg;
pub mod param;
pub mod vc;

use ark_bn254::Bn254;
use ark_ec::PairingEngine;
use ark_ff::FromBytes;
use ark_std::test_rng;
use param::{ProverParam, StructuredReferenceString};
use sha2::{Digest, Sha256};
use ssal_core::types::RawTransaction;

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

pub fn get_block_commitment(block: Vec<RawTransaction>) -> String {
    let mut rng = test_rng();
    let srs = StructuredReferenceString::<Bn254, 128>::new_srs_for_testing(&mut rng);
    let prover_param: ProverParam<Bn254, 128> = (&srs).into();
    let message: Vec<<Bn254 as PairingEngine>::Fr> = block
        .into_iter()
        .map(|raw_tx| {
            let mut hasher = Sha256::new();
            hasher.update(raw_tx.as_ref());
            let hashed_raw_tx = hasher.finalize();
            <Bn254 as PairingEngine>::Fr::read(hashed_raw_tx.as_slice()).unwrap()
        })
        .collect();

    let commitment = Commitment::<Bn254, 128>::commit(&prover_param, &message);
    commitment.to_string()
}
