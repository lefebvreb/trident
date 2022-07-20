pub mod instruction;
pub mod operation;
pub mod parameter;
pub mod symbol;

use std::default;
use std::ops::{Deref, DerefMut};

use thiserror::Error;

use crate::genericity::Id;

use parameter::Parameter;
use symbol::{SymbolTuple, Symbol, Qubit, Bit, List};

#[derive(Default)]
pub struct QuantumCircuit {
    qubit_count: u32,
    bit_count: u32,
    parameter_count: u32,
}

pub struct CircuitBuilder<'id> {
    _id: Id<'id>,
    circ: QuantumCircuit,
}

impl Deref for CircuitBuilder<'_> {
    type Target = QuantumCircuit;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.circ
    }
}

impl DerefMut for CircuitBuilder<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.circ
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum QuantumCircuitError {
    #[error("quantum allocator overflow")]
    AllocOverflow,
}

impl QuantumCircuit {
    #[inline]
    pub fn edit<F>(self, edit: F) -> Result<Self, QuantumCircuitError>
    where
        F: for<'any> FnOnce(&mut CircuitBuilder<'any>) -> Result<(), QuantumCircuitError>
    {
        let mut builder = CircuitBuilder { _id: Id::default(), circ: self };
        edit(&mut builder).map(|_| builder.circ)
    }

    #[inline]
    pub fn new<F>(init: F) -> Result<Self, QuantumCircuitError>
    where
        F: for<'any> FnOnce(&mut CircuitBuilder<'any>) -> Result<(), QuantumCircuitError>
    {
        Self::default().edit(init)
    }

    #[inline]
    pub fn qubit_count(&self) -> usize {
        self.qubit_count as usize
    }

    #[inline]
    pub fn parameter_count(&self) -> usize {
        self.parameter_count as usize
    }

    #[inline]
    pub fn bit_count(&self) -> usize {
        self.bit_count as usize
    }

    // Returns a mutable reference to the qubit count.
    #[inline]
    pub(crate) fn qubit_count_mut(&mut self) -> &mut u32 {
        &mut self.qubit_count
    }

    // Returns a mutable reference to the parameter count.
    #[inline]
    pub(crate) fn parameter_count_mut(&mut self) -> &mut u32 {
        &mut self.parameter_count
    }

    // Returns a mutable reference to the bit count.
    #[inline]
    pub(crate) fn bit_count_mut(&mut self) -> &mut u32 {
        &mut self.bit_count
    }
}

impl<'id> CircuitBuilder<'id> {
    #[inline]
    pub fn alloc<T: Symbol<'id>>(&mut self) -> Result<T, QuantumCircuitError> {
        T::alloc(self)
    }

    #[inline]
    pub fn alloc_n<T: Symbol<'id>, const N: usize>(&mut self) -> Result<[T; N], QuantumCircuitError> {
        T::alloc_n(self)
    }

    #[inline]
    pub fn alloc_list<T: Symbol<'id>>(&mut self, len: usize) -> Result<List<T>, QuantumCircuitError> {
        T::alloc_list(self, len)
    }

    #[inline]
    pub fn alloc_tuple<T: SymbolTuple<'id>>(&mut self) -> Result<T, QuantumCircuitError> {
        T::alloc(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn bell() {
    //     let circ = QuantumCircuit::new(|circ| {
    //         let [q1, q2] = circ.alloc_n()?;

    //         circ.h(q1)
    //             .cx(q1, q2);

    //         Ok(())
    //     });

    //     assert!(circ.is_ok());
    // }
}