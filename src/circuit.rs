use halo2_proofs::{
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr,
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Selector},
    poly::Rotation,
};

use crate::config::MerkleConfig;

const N:usize = 2;
pub struct MerkleCircuit {
    pub salts: [Fr; N],
    pub cards: [Fr; N],
}

impl Circuit<Fr> for MerkleCircuit {
    type Config = MerkleConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        MerkleCircuit {
            salts: [0.into(); N],
            cards: [0.into(); N],
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        let witness_column = meta.advice_column();
        let salt_column: Column<Advice> = meta.advice_column();
        let leaf_column: Column<Advice> = meta.advice_column();
        let node_column: Column<Advice> = meta.advice_column();

        let q_equality: Selector = meta.selector();
        let q_sum: Selector = meta.selector();

        meta.create_gate("equality", |meta| {
            let q = meta.query_selector(q_equality);
            let witness = meta.query_advice(witness_column, Rotation::cur());
            let salt = meta.query_advice(salt_column, Rotation::cur());
            vec![q * (witness - salt)]
        });
        meta.create_gate("sum", |meta|{
            let q = meta.query_selector(q_sum);
            let witness = meta.query_advice(witness_column, Rotation::cur());
            let salt = meta.query_advice(salt_column, Rotation::cur());
            let node = meta.query_advice(node_column, Rotation::cur());
            vec![q*(witness + salt - node)]

        });

        MerkleConfig {
            witness_column,
            salt_column,
            leaf_column,
            node_column,
            q_equality,
            q_sum
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), plonk::Error> {
        let cards: [Fr; N] = self.cards;
        let salts: [Fr; N] = self.salts;

        layouter.assign_region(
            || "deck_and_salts",
            |mut region| {
                let mut card_cells: Vec<AssignedCell<Fr, Fr>> = Vec::with_capacity(2);
                let mut salt_cells: Vec<AssignedCell<Fr, Fr>> = Vec::with_capacity(2);
                let mut sum_cells: Vec<AssignedCell<Fr, Fr>> = Vec::with_capacity(2);
                for i in 0..N {
                    let card_cell = region.assign_advice(
                        || "",
                        config.witness_column,
                        i,
                        || Value::known(cards[i]),
                    )?;
                    let salt_cell = region.assign_advice(
                        || "",
                        config.salt_column,
                        i,
                        || Value::known(salts[i]),
                    )?;
                    let sum_cell = region.assign_advice(
                        || "",
                        config.node_column,
                        i,
                        || Value::known(cards[i] + salts[i]),
                    )?;
                    card_cells.push(card_cell);
                    salt_cells.push(salt_cell);
                    sum_cells.push(sum_cell);
                    config.q_equality.enable(&mut region, i)?;
                    config.q_sum.enable(&mut region, i)?;
                }
                Ok(())
            },
        )
    }
}
