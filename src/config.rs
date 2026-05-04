use halo2_proofs::plonk::{Advice, Column, Selector};

#[derive(Clone)]
pub struct MerkleConfig {
    pub witness_column: Column<Advice>,
    pub salt_column: Column<Advice>,
    pub leaf_column: Column<Advice>,
    pub node_column: Column<Advice>,
    pub q_equality: Selector,
    pub q_sum: Selector,
}
