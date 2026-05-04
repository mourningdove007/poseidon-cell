use halo2_proofs::{
    circuit::{AssignedCell, SimpleFloorPlanner, Value},
    halo2curves::bn256::Fr,
    plonk::{self, Advice, Circuit, Column, Selector},
    poly::Rotation,
};

use crate::config::MerkleConfig;

pub struct MerkleCircuit {
    pub salts: [Fr; 2],
    pub cards: [Fr; 2],
}

impl Circuit<Fr> for MerkleCircuit {
    type Config = MerkleConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        MerkleCircuit {
            salts: [0.into(); 2],
            cards: [0.into(); 2],
        }
    }

    fn configure(meta: &mut plonk::ConstraintSystem<Fr>) -> Self::Config {
        let witness_column = meta.advice_column();
        let salt_column: Column<Advice> = meta.advice_column();
        let leaf_column: Column<Advice> = meta.advice_column();
        let node_column: Column<Advice> = meta.advice_column();

        let q_equality: Selector = meta.selector();

        meta.create_gate("equality", |meta| {
            let q = meta.query_selector(q_equality);
            let witness = meta.query_advice(witness_column, Rotation::cur());
            let salt = meta.query_advice(salt_column, Rotation::cur());
            vec![q * (witness - salt)]
        });

        MerkleConfig {
            witness_column,
            salt_column,
            leaf_column,
            node_column,
            q_equality,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl halo2_proofs::circuit::Layouter<Fr>,
    ) -> Result<(), plonk::Error> {
        let cards: [Fr; 2] = self.cards;
        let salts: [Fr; 2] = self.salts;

        layouter.assign_region(
            || "deck_and_salts",
            |mut region| {
                let mut card_cells: Vec<AssignedCell<Fr, Fr>> = Vec::with_capacity(2);
                let mut salt_cells: Vec<AssignedCell<Fr, Fr>> = Vec::with_capacity(2);
                for i in 0..2 {
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
                    card_cells.push(card_cell);
                    salt_cells.push(salt_cell);
                    config.q_equality.enable(&mut region, i)?;
                }
                Ok(())
            },
        )
    }
}
