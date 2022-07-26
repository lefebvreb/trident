use async_trait::async_trait;

use crate::circuit::TranspiledCircuit;
use crate::instruction::{Instr, InstrVec};
use crate::linalg::UnitaryMatrix;
use crate::symbol::Ancillas;

pub struct Histogram {
    // TODO
}

pub trait Architecture {
    type TranspileError;

    fn num_qubits(&self) -> usize;

    fn connected(&self, qubit1: usize, qubit2: usize) -> bool;

    fn decompose_su2(&self, unitary: UnitaryMatrix<2>) -> (); // TODO

    fn non_local(&self) -> (); // TODO

    fn supports<'id>(&self, instr: &Instr<'id>) -> Result<(), Self::TranspileError>;

    fn transpile<'id>(&self, instructions: InstrVec<'id>, ancillas: Option<Ancillas<'id>>) -> Result<InstrVec<'id>, Self::TranspileError>;
}

#[async_trait]
pub trait Backend {
    type Architecture: Architecture;

    type RuntimeError;

    fn execute(&self, circ: &TranspiledCircuit<Self::Architecture>) -> Result<Histogram, Self::RuntimeError>;

    async fn execute_async(&self, circ: &TranspiledCircuit<Self::Architecture>) -> Result<Histogram, Self::RuntimeError>;
}