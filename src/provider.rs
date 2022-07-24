use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::Deref;

use async_trait::async_trait;

use crate::circuit::{ConcreteCircuit, QuantumCircuit, TranspiledCircuit};
use crate::instruction::{Instr, InstrVec};
use crate::symbol::{Qubit, Ancillas};

pub struct Histogram {
    // TODO
}

#[async_trait]
pub trait Provider: Sized {
    type TranspileConfig;

    type TranspileError;

    type RuntimeError;

    fn transpile<'id>(instructions: InstrVec<'id>, ancillas: Option<Ancillas<'id>>, config: &Self::TranspileConfig) -> Result<InstrVec<'id>, Self::TranspileError>;

    fn execute(circ: &TranspiledCircuit<Self>) -> Result<Histogram, Self::RuntimeError>;
}