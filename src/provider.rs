use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::Deref;

use async_trait::async_trait;

use crate::circuit::QuantumCircuit;
use crate::instruction::Instr;

pub trait InstrSet: Sized {
    type Error;

    fn transpile(circ: &QuantumCircuit) -> Result<QuantumCircuit, Self::Error>;
}

#[derive(Clone, Default, Debug)]
pub struct Transpiled<T: InstrSet> {
    _phantom: PhantomData<T>,
    circ: QuantumCircuit,
}

impl<T: InstrSet> Deref for Transpiled<T> {
    type Target = QuantumCircuit;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.circ
    }
}

impl<T: InstrSet> Transpiled<T> {
    #[inline]
    pub fn new_unchecked(circ: QuantumCircuit) -> Self {
        Self { _phantom: PhantomData, circ }
    }

    #[inline]
    pub fn take(self) -> QuantumCircuit {
        self.circ
    }
}

pub struct DefaultSet;

impl InstrSet for DefaultSet {
    type Error = Infallible;

    #[inline]
    fn transpile(circ: &QuantumCircuit) -> Result<QuantumCircuit, Self::Error> {
        Ok(circ.clone())
    }
}

// #[async_trait]
// pub trait Provider {
//     type InstrSet: InstrSet;

//     type RuntimeError;

//     fn execute(circ: &Transpiled<Self::InstrSet>) -> Result<(), Self::RuntimeError>;
// }