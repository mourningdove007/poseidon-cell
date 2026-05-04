use halo2_proofs::{dev::MockProver, halo2curves::bn256::Fr};
use poseidon_cell::circuit::MerkleCircuit;

fn verify(cards: [Fr; 2], salts: [Fr; 2]) -> Result<(), Vec<halo2_proofs::dev::VerifyFailure>> {
    let circuit = MerkleCircuit { cards, salts };
    MockProver::run(10, &circuit, vec![])
        .expect("MockProver::run failed")
        .verify()
}

#[test]
fn test_identity_permutation() {
    let cards: [Fr; 2] = [1.into(), 2.into()];
    let salts: [Fr; 2] = [1.into(), 2.into()];
    assert!(verify(cards, salts).is_ok());
}


#[test]
fn test_equality_failure() {
    let cards: [Fr; 2] = [3.into(), 2.into()];
    let salts: [Fr; 2] = [1.into(), 2.into()];
    assert!(verify(cards, salts).is_err());
}
